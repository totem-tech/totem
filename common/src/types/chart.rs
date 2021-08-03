/* Level 0: 2 bits */

pub enum Statement {
    BalanceSheet(BalanceSheet),
    ProfitAndLoss(ProfitAndLoss),
    ControllAccount(ControllAccount),
}

/* Level 1: 3 bits */

pub trait StatementType {
    fn encode(&self) -> u32;
}

pub enum BalanceSheet {
    Asset(Asset),
    Liabilities(Liabilities),
    Equity(Equity),
}

pub enum ProfitAndLoss {
    Income(Income),
    Expenses(Expenses),
}

pub enum ControllAccount {
    PurchaseControl(PurchaseControl),
    SalesControl(SalesControl),
    TaxControl(TaxControl),
    EscrowedFundsControl(EscrowedFundsControl),
    BorrowingsControl(BorrowingsControl),
}

/* Level 2: 3 bits */

pub trait AccountCategory {
    fn encode(&self) -> u32;
}

pub enum Asset {
    CurrentAssets(CurrentAssets),
    FixedAssets(FixedAssets),
    IntagibleAssets(IntagibleAssets),
    NonCurrentAssets(NonCurrentAssets),
}

pub enum Liabilities {
    CurrentLiabilities(CurrentLiabilities),
    NonCurrentLiabilities(NonCurrentLiabilities),
}

pub enum Equity {
    ShareholdersEquity(ShareholdersEquity),
    OtherEquity(OtherEquity),
    CapitalStock(CapitalStock),
    OtherReserves(OtherReserves),
    RetainedEarnings(RetainedEarnings),
}

pub enum Income {
    Sales(Sales),
    OtherIncome(OtherIncome),
    OtherOperatingIncome(OtherOperatingIncome),
    OtherComprehensiveIncome(OtherComprehensiveIncome),
}

pub enum Expenses {
    OperatingExpenses(OperatingExpenses),
    OtherOperatingExpenses(OtherOperatingExpenses),
    PersonnelCosts(PersonnelCosts),
    FinanceCosts(FinanceCosts),
}

pub struct PurchaseControl;

pub struct SalesControl;

pub struct TaxControl;

pub struct EscrowedFundsControl;

pub struct BorrowingsControl;

/* Level 3: ? bits */

pub enum CurrentAssets {
    BankCurrentAccount,
    BankSavingsAccount,
    PettyCash,
    XtxBalance,
    /// Escrow.
    TotemRuntimeDeposit,
    FixedDeposits,
    /// Operating Expense.
    PrepaidExpensesOnAccount,
    /// Asset until repaid.
    DirectorsLoanAccount,
    /// Sales Control Account or Trade Debtor's Account.
    Accountsreceivable,
    AccruedRevenuesReceivables,
    ImpairmentlossOnFinancialAssets,
    ImpairmentlossOnContractAssets,
    Inventory,
    /// Trade Supplies.
    AdvancesToSuppliers,
    ContractAssets,
    /// At fair value through profit or loss (FVTPL).
    FinancialAssets,
    AssetsOfDisposalGroupClassifiedAsHeldForSale,
    OtherInvestmentsAtAmortisedCost,
    DerivativeFinancialInstruments,
}

pub enum FixedAssets {}

/* --------------- */

impl Statement {
    fn encode(&self) -> u32 {
        match self {
            Statement::BalanceSheet(s) => 0b01 << 30 & s.encode(),
            Statement::ProfitAndLoss(s) => 0b10 << 30 & s.encode(),
            Statement::ControllAccount(s) => 0b11 << 30 & s.encode(),
        }
    }
}
