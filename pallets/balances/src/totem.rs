// Custom totem stuff.

use super::*;
use core::convert::TryFrom;
use frame_support::traits::WithdrawReasons;
use sp_std::vec::Vec;
use totem_common::traits::accounting::Posting;

const ESCROW: WithdrawReasons = unsafe { WithdrawReasons::from_bits_unchecked(0b1000_0000) };

/// A currency whose accounts can have liquidity restrictions.
pub trait TotemLockableCurrency<AccountId>: Currency<AccountId> {
    /// The quantity used to denote time; usually just a `BlockNumber`.
    type Moment;

    /// The maximum number of locks a user should have on their account.
    type MaxLocks: Get<u32>;

    /// Create a new balance lock on account `who`.
    ///
    /// If the new lock is valid (i.e. not already expired), it will push the struct to
    /// the `Locks` vec in storage. Note that you can lock more funds than a user has.
    ///
    /// If the lock `id` already exists, this will update it.
    fn totem_set_lock(
        id: LockIdentifier,
        who: &AccountId,
        amount: Self::Balance,
        until: Self::Moment,
        reasons: WithdrawReasons,
    ) -> Result<(), TotemLocksError>;

    /// Changes a balance lock (selected by `id`) so that it becomes less liquid in all
    /// parameters or creates a new one if it does not exist.
    ///
    /// Calling `extend_lock` on an existing lock `id` differs from `set_lock` in that it
    /// applies the most severe constraints of the two, while `set_lock` replaces the lock
    /// with the new parameters. As in, `extend_lock` will set:
    /// - maximum `amount`
    /// - bitwise mask of all `reasons`
    fn totem_extend_lock(
        id: LockIdentifier,
        who: &AccountId,
        amount: Self::Balance,
        until: Self::Moment,
        reasons: WithdrawReasons,
    ) -> Result<(), TotemLocksError>;

    /// Remove an existing lock.
    fn totem_remove_lock(id: LockIdentifier, who: &AccountId) -> Result<(), TotemLocksError>;
}

impl<T: Config<I>, I: 'static> TotemLockableCurrency<T::AccountId> for Pallet<T, I>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type Moment = T::BlockNumber;

    type MaxLocks = MaxTotemLocks;

    // Set a lock on the balance of `who`.
    // Is a no-op if lock amount is zero or `reasons` `is_none()`.
    fn totem_set_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        until: T::BlockNumber,
        reasons: WithdrawReasons,
    ) -> Result<(), TotemLocksError> {
        if amount.is_zero() || reasons.is_empty() {
            return Ok(());
        }

        let now = frame_system::Pallet::<T>::block_number();

        // Add or update the new lock
        let mut new_lock = Some(TotemBalanceLock {
            id,
            amount,
            reasons: reasons.into(),
            until,
        });
        let mut locks = Self::totem_locks(who)
            .into_iter()
            .filter_map(|l| {
                if l.id == id {
                    // Update lock
                    new_lock.take()
                } else if l.until < now {
                    // Deadline has passed
                    None
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<_>>();
        if let Some(lock) = new_lock {
            locks.push(lock)
        }

        // Now apply the lock by transfering to the escrow
        Self::transfer_to_the_escrow(who, amount)?;
        Self::totem_update_locks(who, locks)
    }

    // Extend a lock on the balance of `who`.
    // Is a no-op if lock amount is zero or `reasons` `is_none()`.
    fn totem_extend_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        until: T::BlockNumber,
        reasons: WithdrawReasons,
    ) -> Result<(), TotemLocksError> {
        if amount.is_zero() || reasons.is_empty() {
            return Ok(());
        }

        let now = frame_system::Pallet::<T>::block_number();

        let mut new_lock = Some(TotemBalanceLock {
            id,
            amount,
            reasons: reasons.into(),
            until,
        });
        let mut locks = Self::totem_locks(who)
            .into_iter()
            .filter_map(|l| {
                if l.id == id {
                    new_lock.take().map(|nl| TotemBalanceLock {
                        id: l.id,
                        amount: l.amount.max(nl.amount),
                        reasons: l.reasons | nl.reasons,
                        until: nl.until,
                    })
                } else if l.until < now {
                    // Deadline has passed
                    None
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<_>>();
        if let Some(lock) = new_lock {
            locks.push(lock)
        }

        // Now apply the lock by transfering to the escrow
        Self::transfer_to_the_escrow(who, amount)?;
        Self::totem_update_locks(who, locks)
    }

    fn totem_remove_lock(id: LockIdentifier, who: &T::AccountId) -> Result<(), TotemLocksError> {
        let mut locks = Self::totem_locks(who);

        let mut i = 0;
        while i != locks.len() {
            if locks[i].id == id {
                let l = locks.remove(i);
                Self::make_free_balance_be(who, l.amount);
            } else {
                i += 1;
            }
        }

        Self::totem_update_locks(who, locks.into_inner())
    }
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    /// Update the account entry for `who`, given the locks.
    fn totem_update_locks(
        who: &T::AccountId,
        locks: Vec<TotemBalanceLock<T::Balance, T::BlockNumber>>,
    ) -> Result<(), TotemLocksError> {
        let existed = TotemLocks::<T, I>::contains_key(who);
        if locks.is_empty() {
            Locks::<T, I>::remove(who);
            if existed {
                // TODO: use Locks::<T, I>::hashed_key
                // https://github.com/paritytech/substrate/issues/4969
                system::Pallet::<T>::dec_consumers(who);
            }
        } else {
            TotemLocks::<T, I>::insert(
                who,
                WeakBoundedVec::try_from(locks).or(Err(TotemLocksError::NoPlaceRemaining))?,
            );
            if !existed {
                if system::Pallet::<T>::inc_consumers(who).is_err() {
                    // No providers for the locks. This is impossible under normal circumstances
                    // since the funds that are under the lock will themselves be stored in the
                    // account and therefore will need a reference.
                    log::warn!(
                        target: "runtime::balances",
                        "Warning: Attempt to introduce lock consumer reference, yet no providers. \
                        This is unexpected but should be safe."
                    );
                }
            }
        }

        Ok(())
    }

    fn transfer_to_the_escrow(
        who: &T::AccountId,
        amount: T::Balance,
    ) -> result::Result<(), TotemLocksError> {
        let imba = Self::withdraw(who, amount, ESCROW, ExistenceRequirement::KeepAlive)?;

        let escrow_account: T::AccountId = T::Accounting::get_escrow_account();

        Self::make_free_balance_be(&escrow_account, amount);
        let _imba_resolved = imba;

        Ok(())
    }
}

pub struct MaxTotemLocks;

impl Get<u32> for MaxTotemLocks {
    fn get() -> u32 {
        u32::MAX
    }
}

pub enum TotemLocksError {
    /// There is no enough place remaining to add a lock for this account.
    NoPlaceRemaining,
    /// The fund cannot be tranfered to the escrow.
    CannotTransferToTheEscrow(DispatchError),
}

impl From<DispatchError> for TotemLocksError {
    fn from(e: DispatchError) -> TotemLocksError {
        TotemLocksError::CannotTransferToTheEscrow(e)
    }
}
