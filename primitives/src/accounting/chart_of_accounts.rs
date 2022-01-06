use codec::{Decode, Encode, EncodeLike, Error, Input};
use scale_info::{build::Fields, Path, Type, TypeInfo};
use strum::FromRepr;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromRepr)]
#[repr(u16)]
pub enum Ledger {
    /// Bank Current Account.
    B1010001_0000000D,
    /// Bank Savings Account.
    B1010002_0000000D,
    /// Petty Cash.
    B1010003_0000000D,
    /// Internal Address Balance (Network Currency).
    B1010004_0000000D,
    /// Totem Escrow Deposit.
    B1010005_0000000D,
    /// Fixed deposits.
    B1010006_0000000D,
    /// Prepaid Expenses on account (Operating Expense).
    B1010007_0000000D,
    /// Director's loan account (asset until repaid).
    B1010008_0000000D,
    /// Trade receivables - non-related parties.
    B1010009_0000000D,
    /// Trade receivables - related parties.
    B1010010_0000000D,
    /// Loan to non-related parties - Current Assets.
    B1010011_0000000D,
    /// Loan to related parties - Current Assets.
    B1010011_0000001D,
    /// Other receivables - Current Assets.
    B1010011_0000002D,
    /// Finance lease receivables - Current Assets.
    B1010011_0000003D,
    /// Staff loans - Current Assets.
    B1010011_0000004D,
    /// Government grant receivables.
    B1010011_0000005D,
    /// Allowance for doubtful debts .
    B1010012_0000000C,
    /// Other .
    B1010012_0000001C,
    /// Impairment loss on contract assets .
    B1010012_0000002C,
    /// Raw materials.
    B1010013_0000000D,
    /// Work in progress.
    B1010013_0000001D,
    /// Finished goods.
    B1010013_0000002D,
    /// Advances to suppliers (Trade Supplies).
    B1010014_0000000D,
    /// Contract assets.
    B1010015_0000000D,
    /// Equity securities - Fair value through P&L.
    B1010016_0000000D,
    /// Assets of disposal group classified as held-for-sale.
    B1010017_0000000D,
    /// Loan notes - floating or fixed.
    B1010018_0000000D,
    /// Interest rate swaps - Current Assets.
    B1010019_0000000D,
    /// Currency forwards - Current Assets.
    B1010020_0000000D,
    /// Commodity forwards - Current Assets.
    B1010021_0000000D,
    /// Cryptocurrency (UTXO).
    B1114001_0000000D,
    /// Cryptocurrency (Nonce).
    B1114001_0000001D,
    /// Cryptocurrency (Parachain).
    B1114001_0000002D,
    /// Cryptocurrency (Third-party custodian).
    B1114001_0000003D,
    /// Cryptocurrency (Other).
    B1114001_0000004D,
    /// Fungible tokens (smart contract).
    B1114002_0000000D,
    /// Fungible tokens (Third-party custodian).
    B1114002_0000001D,
    /// Non-Fungible tokens (smart contract).
    B1114003_0000002D,
    /// Non-Fungible tokens (Third-party custodian).
    B1114003_0000003D,
    /// Liquidity Pool Pair (smart contract).
    B1114004_0000000D,
    /// Cryprto Loan Collateral.
    B1114005_0000000D,
    /// Impairment Loss Cryptocurrency (UTXO).
    B1114006_0000002C,
    /// Impairment Loss Cryptocurrency (Nonce).
    B1114006_0000003C,
    /// Impairment Loss Cryptocurrency (Parachain).
    B1114006_0000004C,
    /// Impairment Loss Cryptocurrency (Third-party custodian).
    B1114006_0000000C,
    /// Impairment Loss Cryptocurrency (Other).
    B1114006_0000001C,
    /// Impairment Loss Fungible tokens (smart contract).
    B1114007_0000002C,
    /// Impairment Loss Fungible tokens (Third-party custodian).
    B1114007_0000003C,
    /// Impairment Loss Non-Fungible tokens (smart contract).
    B1114008_0000000C,
    /// Impairment Loss Non-Fungible tokens (Third-party custodian).
    B1114008_0000001C,
    /// Impairment Loss Liquidity Pool Pair (smart contract).
    B1114009_0000000C,
    /// Impairment Loss Crypto Loan Collateral.
    B1114010_0000000C,
    /// Land.
    B1211001_0000000D,
    /// Buildings.
    B1211002_0000000D,
    /// Furniture fixtures and fittings.
    B1211003_0000000D,
    /// Plant and equipment.
    B1211004_0000000D,
    /// Motor vehicles.
    B1211005_0000000D,
    /// Supplies.
    B1211006_0000000D,
    /// Computer and IT equipment.
    B1211007_0000000D,
    /// Right-of-use assets.
    B1211008_0000000D,
    /// Assets under construction.
    B1211009_0000000D,
    /// Leasehold improvements.
    B1211010_0000000D,
    /// Land - Accumulated depreciation.
    B1211011_0000000C,
    /// Buildings- Accumulated depreciation.
    B1211011_0000001C,
    /// Furniture fixtures and fittings - Accumulated depreciation.
    B1211011_0000002C,
    /// Plant and equipment - Accumulated depreciation.
    B1211011_0000003C,
    /// Motor vehicles - Accumulated depreciation.
    B1211011_0000004C,
    /// Supplies - Accumulated depreciation.
    B1211011_0000005C,
    /// Computer and IT equipment - Accumulated depreciation.
    B1211011_0000006C,
    /// Right-of-use assets - Accumulated depreciation.
    B1211011_0000007C,
    /// Assets under construction - Accumulated depreciation.
    B1211011_0000008C,
    /// Leasehold improvements - Accumulated depreciation.
    B1211011_0000009C,
    /// e.g. Films, copyrights or other intellectual property rights - Accumulated depreciation.
    B1312001_0000000D,
    /// Trademark and patents (BS).
    B1312002_0000000D,
    /// Computer software licences.
    B1312003_0000000D,
    /// External Network Cryptocurrency.
    B1312004_0000000D,
    /// Goodwill.
    B1312005_0000000D,
    /// Trademark and patents - Accumulated amortisation.
    B1312006_0000000C,
    /// Computer software licences - Accumulated amortisation.
    B1312006_0000001C,
    /// Others.
    B1312006_0000002C,
    /// Impairment loss on intangible assets.
    B1312007_0000000C,
    /// Investment in subsidiaries.
    B1413001_0000000D,
    /// Investment in joint venture.
    B1413002_0000000D,
    /// Investment in associates.
    B1413003_0000000D,
    /// Investment properties.
    B1413004_0000000D,
    /// Equity securities - Fair value through other comprehensive income.
    B1413005_0000000D,
    /// Debt securities.
    B1413006_0000001D,
    /// Deferred income tax - Non-Current Assets.
    B1413007_0000000D,
    /// Impairment of fixed assets.
    B1413008_0000000C,
    /// Impairment investment in subsidiaries.
    B1413008_0000001C,
    /// Impairment investment in joint venture.
    B1413008_0000002C,
    /// Impairment investment in associates.
    B1413008_0000003C,
    /// Non-listed debt instruments.
    B1413009_0000000D,
    /// Mandatorily measured at FVTPL (e.g. redeemable preference shares).
    B1413009_0000001D,
    /// Convertible bonds - Non-Current Assets (BS).
    B1413009_0000002D,
    /// Loan to non-related parties - Non-Current Assets.
    B1413010_0000000D,
    /// Loan to related parties - Non-Current Assets.
    B1413010_0000001D,
    /// Other receivables - Non-Current Assets.
    B1413010_0000002D,
    /// Finance lease receivables - Non-Current Assets.
    B1413010_0000003D,
    /// Staff loans - Non-Current Assets.
    B1413010_0000004D,
    /// Indemnification assets.
    B1413010_0000005D,
    /// Interest rate swaps - Non-Current Assets.
    B1413011_0000000D,
    /// Currency forwards - Non-Current Assets.
    B1413011_0000001D,
    /// Commodity forwards - Non-Current Assets.
    B1413011_0000002D,
    /// Sales Tax by Jurisdiction (Can be a negative balance for reclaims [technically an asset]).
    B2020001_0000000C,
    /// Federal or State Tax By Jurisdiction on Sales.
    B2020002_0000000C,
    /// Accounts payable (Trade creditors).
    B2020003_0000000C,
    /// Current portion of long-term Debt (non-trade).
    B2020003_0000001C,
    /// Short-term Loans (Payable).
    B2020003_0000002C,
    /// Salaries Payable.
    B2020004_0000000C,
    /// Wages Payable.
    B2020005_0000000C,
    /// Commission Payable.
    B2020006_0000000C,
    /// Freight Payable.
    B2020007_0000000C,
    /// Other Accrued Expenses Payable.
    B2020008_0000000C,
    /// Payroll Tax By Jurisdiction Liabilities.
    B2020009_0000000C,
    /// Interest Payable.
    B2020010_0000000C,
    /// Advances from customers.
    B2020011_0000000C,
    /// Lawsuits and Legal Costs Payable.
    B2020012_0000000C,
    /// Accounts payable (Trade creditors).
    B2020013_0000000C,
    /// Short-term Loans (Payable).
    B2020013_0000001C,
    /// Current portion of long-term Debt (non-trade).
    B2020013_0000002C,
    /// Financial guarantees.
    B2020014_0000000C,
    /// Contract liabilities.
    B2020015_0000000C,
    /// Bank overdrafts.
    B2020016_0000000C,
    /// Current portion of lease liabilities.
    B2020017_0000000C,
    /// Warranty / Legal claims / Others - Current Liabilities (BS).
    B2020018_0000000C,
    /// Unused leave - Current Liabilities (BS).
    B2020018_0000001C,
    /// Interest rate swaps - Current Liabilities.
    B2020019_0000000C,
    /// Currency forwards - Current Liabilities.
    B2020019_0000001C,
    /// Commodity forwards - Current Liabilities.
    B2020019_0000002C,
    /// Dividend payable.
    B2020020_0000000C,
    /// Current portion of bank borrowings (Payable).
    B2020021_0000000C,
    /// Income Tax By Jurisdiction Payable (Witholding Tax).
    B2121001_0000000C,
    /// Deferred Revenues (Unearned Revenues).
    B2121002_0000000C,
    /// Bonds Payable.
    B2121003_0000000C,
    /// Notes (Loans) Payable.
    B2121004_0000000C,
    /// Waranty Liabilities.
    B2121005_0000000C,
    /// Corporation tax by jurisdiction payable (posted at end of period or end of year).
    B2121006_0000000C,
    /// Installment Loans (Payable).
    B2121007_0000000C,
    /// Portion of bank borrowings (Payable) - Non-Current Liabilities.
    B2121008_0000000C,
    /// Mortgage or Property Loans (Payable).
    B2121009_0000000C,
    /// Redeemable preference shares.
    B2121010_0000000C,
    /// Portion of lease liabilities - Non-Current Liabilities.
    B2121011_0000000C,
    /// Contingent consideration payable.
    B2121012_0000000C,
    /// Deferred income tax - Non-Current Liabilities.
    B2121013_0000000C,
    /// Non-current portion of long-term Debt (non-trade).
    B2121014_0000000C,
    /// Non-current portion of long-term Debt (non-trade).
    B2121014_0000001C,
    /// Warranty / Legal claims / Others - Non-Current Liabilities (BS).
    B2121015_0000000C,
    /// Interest rate swaps - Non-Current Liabilities.
    B2121016_0000000C,
    /// Currency forwards - Non-Current Liabilities.
    B2121016_0000001C,
    /// Commodity forwards - Non-Current Liabilities.
    B2121016_0000002C,
    /// Personal Net Worth.
    B3030001_0000000C,
    /// Owner's Equity.
    B3030002_0000000C,
    /// Corporation tax by jurisdiction (calculated after P&L).
    B3131001_0000000C,
    /// Ordinary shares.
    B3232001_0000000C,
    /// Preference shares.
    B3232002_0000000C,
    /// Treasury shares.
    B3232003_0000000D,
    /// Share application monies.
    B3333001_0000000C,
    /// Share option reserve.
    B3333002_0000000C,
    /// Fair value gains / (losses) on financial assets at FVOCI (debt instruments).
    B3333003_0000000C,
    /// Fair value gains / (losses) on financial assets at FVOCI (equity instruments).
    B3333004_0000001C,
    /// Cash flow hedges.
    B3333005_0000000C,
    /// Equity component of convertible bonds.
    B3333006_0000000C,
    /// Revaluation of property, plant and equipment.
    B3333007_0000000C,
    /// Dividend paid.
    B3434001_0000000D,
    /// Retained Earnings.
    B3434002_0000000C,
    /// Sales of services.
    P4040001_0000000C,
    /// Sales of goods.
    P4040002_0000000C,
    /// Sales returns and allowances.
    P4040003_0000000D,
    /// Sales discounts.
    P4040004_0000000C,
    /// Freight Billable.
    P4040005_0000000C,
    /// Commission Billable.
    P4040006_0000000C,
    /// Miscellaneous Income.
    P4040007_0000000C,
    /// Royalty income.
    P4141001_0000000C,
    /// FInancial assets measured at amortised cost.
    P4141002_0000000C,
    /// Investments.
    P4141002_0000001C,
    /// Trade receivables.
    P4141002_0000002C,
    /// Bank deposits.
    P4141002_0000003C,
    /// Loans to an associate / subsidiary.
    P4141002_0000004C,
    /// Debt investments measured at FVOCI.
    P4141002_0000005C,
    /// Dividend income.
    P4141003_0000000C,
    /// Rental income.
    P4141004_0000000C,
    /// Grant income.
    P4141005_0000000C,
    /// Fair value gains / (losses) on financial assets and liabilities at FVTPL.
    P4141006_0000000C,
    /// Fair value gains / (losses) on derivative financial instruments.
    P4141007_0000000C,
    /// Ineffectiveness on fair value / cash flow hedges.
    P4141008_0000000C,
    /// Financial assets at FVOCI, reclassified from OCI on disposal.
    P4141009_0000000C,
    /// Fair value gains / (losses) on investment properties.
    P4141010_0000000C,
    /// Fair value gains / (loss) on contingent consideration .
    P4141011_0000000C,
    /// Share of profit / (loss) of associates and joint ventures.
    P4241001_0000000C,
    /// Gains / (Losses) on sale / disposal of property, plant and equipment.
    P4241002_0000000C,
    /// Profit / (loss) from discontinued operations.
    P4241003_0000000C,
    /// Debt instruments .
    P4342001_0000000C,
    /// Equity instruments.
    P4342002_0000001C,
    /// Fair value gains / (losses) on cash flow hedges.
    P4342003_0000000C,
    /// Share of other comprehensive income of associates.
    P4342004_0000000C,
    /// Reclassification adjustments (for items that may be reclassified subsequently to profit or loss).
    P4342005_0000000C,
    /// Revaluation gains on property, plant and equipment.
    P4342006_0000000C,
    /// Fair Value gains / (loses) holding Cryptocurrency (UTXO).
    P4342007_0000000C,
    /// Fair Value gains / (loses) holding Cryptocurrency (Nonce).
    P4342007_0000001C,
    /// Fair Value gains / (loses) holding Cryptocurrency (Parachain).
    P4342007_0000002C,
    /// Fair Value gains / (loses) holding Cryptocurrency (Third-party custodian).
    P4342007_0000003C,
    /// Fair Value gains / (loses) holding Cryptocurrency (Other).
    P4342007_0000004C,
    /// Fair Value gains / (loses) holding Fungible tokens (smart contract).
    P4342008_0000000C,
    /// Fair Value gains / (loses) holding Fungible tokens (Third-party custodian).
    P4342008_0000001C,
    /// Fair Value gains / (loses) holding Non-Fungible tokens (smart contract).
    P4342009_0000000C,
    /// Fair Value gains / (loses) holding Non-Fungible tokens (Third-party custodian).
    P4342009_0000001C,
    /// Fair Value gains / (loses) holding Liquidity Pool Pair (smart contract).
    P4342010_0000000C,
    /// Fair Value gains / (loses) holding Crypto Loan Collateral.
    P4342011_0000000C,
    /// Gains / (Losses) on sale / disposal of Cryptocurrency Assets.
    P4342012_0000000C,
    /// Raw materials - changes in inventories.
    P5050001_0000000D,
    /// Work in progress - changes in inventories.
    P5050001_0000001D,
    /// Finished goods - changes in inventories.
    P5050001_0000002D,
    /// Purchases - direct material costs.
    P5050001_0000003D,
    /// Direct labour costs.
    P5050001_0000004D,
    /// Manufacturing / Factory overhead.
    P5050001_0000005D,
    /// Purchases returns and allowances.
    P5050001_0000006D,
    /// Purchase discount.
    P5050001_0000007D,
    /// Inventory write-down / Reversal of inventory write-down.
    P5050001_0000008D,
    /// Inter-Entity Charge In.
    P5050002_0000000D,
    /// Inter-Entity Charge Out.
    P5050002_0000001D,
    /// IT Distribution.
    P5050002_0000002D,
    /// Distribution To Capital.
    P5050002_0000003D,
    /// Allocations.
    P5050002_0000004D,
    /// Allocation By Country.
    P5050002_0000005D,
    /// Facilities Allocation.
    P5050002_0000006D,
    /// Assess Research And Development Recharges.
    P5050002_0000007D,
    /// General Project Costs.
    P5050003_0000000D,
    /// Depreciation Land.
    P5050003_0000001D,
    /// Depreciation Buildings.
    P5050003_0000002D,
    /// Depreciation Furniture fixtures and fittings.
    P5050003_0000003D,
    /// Depreciation Plant and equipment.
    P5050003_0000004D,
    /// Depreciation Motor vehicles.
    P5050003_0000005D,
    /// Depreciation Supplies.
    P5050003_0000006D,
    /// Depreciation Computer and IT equipment.
    P5050003_0000007D,
    /// Depreciation Right-of-use assets.
    P5050003_0000008D,
    /// Depreciation Leasehold improvements.
    P5050003_0000009D,
    /// Amortisation of Others intangible assets.
    P5050003_0000010D,
    /// Amortisation of Trademark and patents.
    P5050003_0000011D,
    /// Amortisation of Computer software licences.
    P5050003_0000012D,
    /// Amortization Government Grants.
    P5050003_0000013D,
    /// Gain Loss On Miscellaneous Sales.
    P5050003_0000014D,
    /// Non Capital Project Expense.
    P5050003_0000015D,
    /// Consumption Other.
    P5050004_0000000D,
    /// Trials.
    P5050004_0000001D,
    /// Samples.
    P5050004_0000002D,
    /// Warranties.
    P5050005_0000000D,
    /// Tax (Corporation Tax).
    P5050006_0000000D,
    /// Employee Tax.
    P5050007_0000000D,
    /// Tax Property.
    P5050007_0000001D,
    /// Tax Trade Licences.
    P5050007_0000002D,
    /// Tax Enviromental.
    P5050007_0000003D,
    /// Road Tax.
    P5050007_0000004D,
    /// Taxes Other.
    P5050007_0000005D,
    /// Taxes Other Alternate Acct.
    P5050007_0000006D,
    /// Non-Deductible Fines Penalties.
    P5050007_0000007D,
    /// Penalties.
    P5050007_0000008D,
    /// Licenses Fees Certificates Other Taxes.
    P5050007_0000009D,
    /// Car Tax.
    P5050007_0000010D,
    /// Capital Gains Tax on Crypto Assets.
    P5050007_0000011D,
    /// Claims .
    P5050008_0000000D,
    /// Commissions Manual.
    P5050009_0000000D,
    /// Commissions Intercompany Expense.
    P5050009_0000001D,
    /// Commissions Intercompany Income.
    P5050009_0000002D,
    /// Samples Consumption Marketing.
    P5050010_0000000D,
    /// Promotions.
    P5050010_0000001D,
    /// Advertising Expenses.
    P5050010_0000002D,
    /// Advertising Promotions Plant Visit.
    P5050010_0000003D,
    /// Design And Artwork.
    P5050010_0000004D,
    /// Advertising Promotions Gifts.
    P5050010_0000005D,
    /// Advertising Promotions Meetings Seminars.
    P5050010_0000006D,
    /// Advertising Promotions Printed Material.
    P5050010_0000007D,
    /// Advertising Promotions Outdoor Advertising.
    P5050010_0000008D,
    /// Advertising Promotions Presentation Material.
    P5050010_0000009D,
    /// Distributor Events.
    P5050010_0000010D,
    /// Market Research.
    P5050010_0000011D,
    /// Fairs And Exhibitions.
    P5050010_0000012D,
    /// Media Print.
    P5050010_0000013D,
    /// Media Radio.
    P5050010_0000014D,
    /// Media Television.
    P5050010_0000015D,
    /// Media Other.
    P5050010_0000016D,
    /// Samples.
    P5050010_0000017D,
    /// Custom Gifts Taxable.
    P5050010_0000018D,
    /// Custom Gifts Non Taxable.
    P5050010_0000019D,
    /// Technical Service Fees.
    P5050010_0000020D,
    /// Literature.
    P5050010_0000021D,
    /// Brand Merchandise.
    P5050010_0000022D,
    /// Trade Shows Exhibit Fees.
    P5050010_0000023D,
    /// Agency Fees.
    P5050010_0000024D,
    /// Audio Visuals.
    P5050010_0000025D,
    /// Marketing Events Days In The Field.
    P5050010_0000026D,
    /// Marketing Miscellaneous Expense.
    P5050010_0000027D,
    /// Marketing Cost Sharing.
    P5050010_0000028D,
    /// Consulting Fees .
    P5050011_0000000D,
    /// Miscellaneous Service.
    P5050012_0000000D,
    /// Analytical Testing.
    P5050012_0000001D,
    /// Communications Tarriffs.
    P5050012_0000002D,
    /// Experimental Testing.
    P5050012_0000003D,
    /// Financial Services.
    P5050012_0000004D,
    /// Freight Expenses.
    P5050012_0000005D,
    /// Housekeeping & Laundry.
    P5050012_0000006D,
    /// Language Translation Services.
    P5050012_0000007D,
    /// Mail Delivery Services.
    P5050012_0000008D,
    /// Miscellaneous Fees.
    P5050012_0000009D,
    /// Contractors.
    P5050012_0000010D,
    /// Technical Assistance.
    P5050012_0000011D,
    /// Laboratory Expense.
    P5050012_0000012D,
    /// Labour.
    P5050012_0000013D,
    /// Printing.
    P5050012_0000014D,
    /// Warehouse.
    P5050012_0000015D,
    /// Photographs Graphics.
    P5050012_0000016D,
    /// Project Engineering.
    P5050012_0000017D,
    /// Research Assistance.
    P5050012_0000018D,
    /// Research Shopping Expense.
    P5050012_0000019D,
    /// Other service for Production.
    P5050012_0000020D,
    /// Software Maintenance Fees.
    P5050012_0000021D,
    /// Tax Preparation Fees.
    P5050012_0000022D,
    /// Testing Services.
    P5050012_0000023D,
    /// Travel Expenses Per Diem.
    P5050013_0000000D,
    /// Travel General.
    P5050013_0000001D,
    /// Travel Expenses Air Tickets.
    P5050013_0000002D,
    /// Travel Expenses Air Tickets Outside Country Of Residence.
    P5050013_0000003D,
    /// Travel Expenses Miles.
    P5050013_0000004D,
    /// Taxi & Other Personal Transport.
    P5050013_0000005D,
    /// Lodging Hotel Costs.
    P5050014_0000000D,
    /// Meetings Conferences.
    P5050015_0000000D,
    /// Other Grower Meetings.
    P5050015_0000001D,
    /// Key Distributor Meetings.
    P5050015_0000002D,
    /// Travel Foreign.
    P5050015_0000003D,
    /// Travel Non Employee.
    P5050015_0000004D,
    /// Travel Tech Meetings Conferences.
    P5050015_0000005D,
    /// Business Meals Entertainment.
    P5050016_0000000D,
    /// Business Meals Within Country Of Residence.
    P5050016_0000001D,
    /// Business Meals Outside Country Of Residence.
    P5050016_0000002D,
    /// Entertainment Non-Deductable.
    P5050016_0000003D,
    /// Meals Entertainment Field Sales.
    P5050016_0000004D,
    /// Meals Entertainment Training Education.
    P5050016_0000005D,
    /// Meals Entertainment Business Development (No Meeting).
    P5050016_0000006D,
    /// Meals Entertainment Business Development.
    P5050016_0000007D,
    /// Other Travel Expenses .
    P5050017_0000000D,
    /// Public Relations.
    P5050018_0000000D,
    /// Automobile Expense.
    P5050019_0000000D,
    /// Automobile Expense Parking.
    P5050019_0000001D,
    /// Automobile Expense Company Car.
    P5050019_0000002D,
    /// Automobile Expense Telephone.
    P5050019_0000003D,
    /// Automobile Expense Rental.
    P5050019_0000004D,
    /// Automobile Expense Repairs.
    P5050019_0000005D,
    /// Automobile Expense Car Financing.
    P5050019_0000006D,
    /// Automobile Expense Fuel.
    P5050019_0000007D,
    /// Leased Autos.
    P5050019_0000008D,
    /// Equipment.
    P5050020_0000000D,
    /// Plant Maintenance .
    P5050021_0000000D,
    /// Cellular Usage.
    P5050022_0000000D,
    /// Communication Remote Access.
    P5050022_0000001D,
    /// Miscellaneous Data Communications.
    P5050022_0000002D,
    /// Telephone Facsimiles.
    P5050022_0000003D,
    /// Rent Warehouse.
    P5050023_0000000D,
    /// Non Lease Rental.
    P5050023_0000001D,
    /// Rent Building Office.
    P5050023_0000002D,
    /// Rent Land.
    P5050023_0000003D,
    /// Rent Equipment.
    P5050023_0000004D,
    /// Sub Lease Income.
    P5050023_0000005D,
    /// Rent Other Facilities.
    P5050023_0000006D,
    /// Office Lab Furniture.
    P5050024_0000000D,
    /// Service Contracts.
    P5050024_0000001D,
    /// Repairs.
    P5050024_0000002D,
    /// Repairs Equipment.
    P5050024_0000003D,
    /// IT Maintenance.
    P5050024_0000004D,
    /// Maintenance.
    P5050024_0000005D,
    /// Maintenance Equip.
    P5050024_0000006D,
    /// Maint Other.
    P5050024_0000007D,
    /// Maintenance Other Miscellaneous Hardware.
    P5050024_0000008D,
    /// Maintenance Other Office Cleaning.
    P5050024_0000009D,
    /// Maintenance Building.
    P5050024_0000010D,
    /// Tools.
    P5050024_0000011D,
    /// Services.
    P5050024_0000012D,
    /// Purchased Supplies.
    P5050024_0000013D,
    /// Pm Purchased Equipment Non Capital.
    P5050024_0000014D,
    /// Miscellaneous Maintenance.
    P5050024_0000015D,
    /// Supplies Other.
    P5050025_0000000D,
    /// Supplies Office.
    P5050025_0000001D,
    /// Supplies Office Photcopy Document Handling.
    P5050025_0000002D,
    /// Supplies Lab.
    P5050025_0000003D,
    /// Supplies Safety.
    P5050025_0000004D,
    /// Supplies Production.
    P5050025_0000005D,
    /// Supplies Maintenance.
    P5050025_0000006D,
    /// Supplies Industrial (Chemicals etc...).
    P5050025_0000007D,
    /// Supplies Cleaning.
    P5050025_0000008D,
    /// Equipment Electrical Equipment.
    P5050026_0000000D,
    /// Computer Related Equipment.
    P5050026_0000001D,
    /// Software Licenses.
    P5050026_0000002D,
    /// Electricity Purchased.
    P5050026_0000003D,
    /// Water Purchased.
    P5050026_0000004D,
    /// Gas Purchased.
    P5050026_0000005D,
    /// Fuel.
    P5050026_0000006D,
    /// Utilities Combined.
    P5050026_0000007D,
    /// Cloud Storage.
    P5050026_0000008D,
    /// Datacentres.
    P5050026_0000009D,
    /// Insurance Other.
    P5050027_0000000D,
    /// Insurance Other Export Credit.
    P5050027_0000001D,
    /// Self Insurance.
    P5050027_0000002D,
    /// Motor vehicle insurance.
    P5050027_0000003D,
    /// Medical insurance.
    P5050027_0000004D,
    /// Life insurance.
    P5050027_0000005D,
    /// Travel insurance.
    P5050027_0000006D,
    /// Non-exec Directors Fees, & Other Governance Costs.
    P5050028_0000000D,
    /// External Corporate Secretarial fees.
    P5050029_0000000D,
    /// Litigation.
    P5050029_0000001D,
    /// Outside Counsel Fees.
    P5050029_0000002D,
    /// Licenses Permits.
    P5050029_0000003D,
    /// Legal Fees International.
    P5050029_0000004D,
    /// Legal Fees Other.
    P5050029_0000005D,
    /// FCPA Gifts.
    P5050029_0000006D,
    /// FCPA Travel.
    P5050029_0000007D,
    /// FCPA Meals Entertainment.
    P5050029_0000008D,
    /// Business Contributions.
    P5050029_0000009D,
    /// Charitable Contributions.
    P5050029_0000010D,
    /// Donations Disallowed.
    P5050029_0000011D,
    /// Local Tax.
    P5050029_0000012D,
    /// Audit Fees External.
    P5050029_0000013D,
    /// Auditor Firm Tax Fees.
    P5050029_0000014D,
    /// Auditor Firm Other Fees.
    P5050029_0000015D,
    /// Audit Fees.
    P5050029_0000016D,
    /// Patents.
    P5050029_0000017D,
    /// Product Registrations.
    P5050029_0000018D,
    /// Trademark and patents (P&L).
    P5050029_0000019D,
    /// Design Rights.
    P5050029_0000020D,
    /// Network Transaction Fees.
    P5050030_0000000D,
    /// Bank Charges.
    P5050030_0000001D,
    /// Administration Business Tax.
    P5050030_0000002D,
    /// Corporate Membership.
    P5050030_0000003D,
    /// Memberships Subscriptions.
    P5050030_0000004D,
    /// Other Non Deductible.
    P5050030_0000005D,
    /// Postage Expenses.
    P5050030_0000006D,
    /// Courrier Expenses.
    P5050030_0000007D,
    /// Management Adjustment.
    P5050030_0000008D,
    /// Collection Expense.
    P5050030_0000009D,
    /// Credit Reports.
    P5050030_0000010D,
    /// Other Miscellaneous Expense.
    P5050030_0000011D,
    /// Miscellaneous Month End Accruals.
    P5050030_0000012D,
    /// Sponsorship.
    P5050030_0000013D,
    /// Books Library.
    P5050030_0000014D,
    /// Equipment Purchased Non-Capitalised.
    P5050030_0000015D,
    /// Software Purchased Non-Capitalised.
    P5050030_0000016D,
    /// Miscellaneous Reimbursement Expenses.
    P5050030_0000017D,
    /// Recovered Expenses Canteen Receipts.
    P5050030_0000018D,
    /// Recovered Expense Miscellaneous Charge Out.
    P5050030_0000019D,
    /// Recovered Expenses Insurance.
    P5050030_0000020D,
    /// Recovered Expenses Car Contract.
    P5050030_0000021D,
    /// Recovered Expenses Other.
    P5050030_0000022D,
    /// Recovered Expenses Salaries.
    P5050030_0000023D,
    /// Recovered Expenses Wages.
    P5050030_0000024D,
    /// Procurement Card Purchases.
    P5050030_0000025D,
    /// Administration Service Fees.
    P5050030_0000026D,
    /// Other Miscellaneous Income Expense.
    P5050030_0000027D,
    /// Bad Debt Expense / Reversal.
    P5050031_0000000D,
    /// Shipping Cost.
    P5050033_0000000D,
    /// Other Costs.
    P5050033_0000001D,
    /// Standard Cost Intercompany.
    P5050033_0000002D,
    /// Finished Goods Consumption Each.
    P5050033_0000003D,
    /// Samples Consumption.
    P5050033_0000004D,
    /// Semifinished Consumption.
    P5050033_0000005D,
    /// Freight Non-Customer.
    P5050033_0000006D,
    /// Freight From or To Warehouse.
    P5050033_0000007D,
    /// Duties On Purchases.
    P5050033_0000008D,
    /// Handling Warehouse.
    P5050033_0000009D,
    /// Transportation.
    P5050033_0000010D,
    /// Waste Disposal.
    P5050033_0000011D,
    /// Customs Fees.
    P5050033_0000013D,
    /// Sponsorship / Donations.
    P5050033_0000014D,
    /// Financial guarantee fees.
    P5050034_0000001D,
    /// Royalty expenses.
    P5050035_0000002D,
    /// Extraordinary expenses.
    P5050037_0000004D,
    /// Impairment loss on financial assets.
    P5050038_0000005D,
    /// Impairment loss on contract assets.
    P5050039_0000006D,
    /// Warranty / Legal claims / Others - Expenses (P&L).
    P5050040_0000000D,
    /// Unused leave - Expenses (P&L).
    P5050040_0000001D,
    /// Property, plant and equipment written off.
    P5050041_0000000D,
    /// Information Technology Miscellaneous.
    P5151001_0000000D,
    /// Secondary Costs.
    P5151001_0000001D,
    /// Reporting Adjustment.
    P5151001_0000002D,
    /// Salaries.
    P5252001_0000000D,
    /// Salaries 13th Month.
    P5252001_0000001D,
    /// Salaries Achievement Awards.
    P5252001_0000002D,
    /// Salaries Representation Allowance.
    P5252001_0000003D,
    /// Salaries Fees.
    P5252001_0000004D,
    /// Salaried Operations.
    P5252001_0000005D,
    /// Wages 13th Month.
    P5252001_0000006D,
    /// Payroll Fees.
    P5252001_0000007D,
    /// Payroll Luncheon Vouchers.
    P5252001_0000008D,
    /// Social Security Employer Contribution.
    P5252001_0000009D,
    /// Fringes Variable.
    P5252001_0000010D,
    /// Social Security Other.
    P5252001_0000011D,
    /// Social Security Provision.
    P5252001_0000012D,
    /// Payroll Social Security.
    P5252001_0000013D,
    /// Accruals Social Security Charges 13th Month.
    P5252001_0000014D,
    /// Accruals Social Security Charges Holiday Pay.
    P5252001_0000015D,
    /// Housing Tax.
    P5252001_0000016D,
    /// Apprenticeship Tax.
    P5252001_0000017D,
    /// Work Council.
    P5252001_0000018D,
    /// Severence Pay.
    P5252001_0000019D,
    /// Transportation Subsidy.
    P5252001_0000020D,
    /// Wages Hourly.
    P5252001_0000021D,
    /// Salaries Wages Per Diem.
    P5252001_0000022D,
    /// Overtime Payments.
    P5252001_0000023D,
    /// Sick Pay.
    P5252001_0000024D,
    /// Holiday Pay.
    P5252001_0000025D,
    /// Vacation Pay.
    P5252001_0000026D,
    /// Other Employee Compensation.
    P5252001_0000027D,
    /// Savings Plan.
    P5252001_0000028D,
    /// Field Incentive Plan.
    P5252001_0000029D,
    /// Pension.
    P5252001_0000030D,
    /// Insurance Plan Live Medical.
    P5252001_0000031D,
    /// Insurance Work Accident.
    P5252001_0000032D,
    /// Insurance Private Collective.
    P5252001_0000033D,
    /// Pension Costs Company Portion.
    P5252001_0000034D,
    /// Private Patient Plan.
    P5252001_0000035D,
    /// Early Retirement Plans.
    P5252001_0000036D,
    /// Employee share option expense.
    P5252001_0000037D,
    /// School Expenses.
    P5252002_0000000D,
    /// Home Leave -Expat expenses.
    P5252002_0000001D,
    /// Expat Cost.
    P5252002_0000002D,
    /// Assignee Allowances Other.
    P5252002_0000003D,
    /// Employe Language Lessons.
    P5252002_0000004D,
    /// Employee Moving Expense.
    P5252002_0000005D,
    /// Employee Assistance Program.
    P5252002_0000006D,
    /// Employee Housing Expense.
    P5252002_0000007D,
    /// Expat Other Benefits.
    P5252002_0000008D,
    /// Assignee Tax.
    P5252002_0000009D,
    /// Incentive Plan.
    P5252004_0000000D,
    /// Incentive Plan Overhead.
    P5252005_0000000D,
    /// Employee Service Awards.
    P5252006_0000000D,
    /// Security Services.
    P5252006_0000001D,
    /// Health Safety.
    P5252006_0000002D,
    /// Cleaning And Security Services Alternate Account.
    P5252006_0000003D,
    /// Cafeteria Service.
    P5252006_0000004D,
    /// Medical Benefits.
    P5252006_0000005D,
    /// Medical Services and Supplies.
    P5252006_0000006D,
    /// Recreation Sports Social.
    P5252006_0000007D,
    /// Recruiting.
    P5252006_0000008D,
    /// Employee Agency Fees.
    P5252006_0000009D,
    /// Transport Of Personal.
    P5252006_0000010D,
    /// Education Training.
    P5252006_0000011D,
    /// Education Training Associated Travel Costs.
    P5252006_0000012D,
    /// Training Tax.
    P5252006_0000013D,
    /// Tuition.
    P5252006_0000014D,
    /// Tuition Taxable.
    P5252006_0000015D,
    /// Laundry Clothing.
    P5252006_0000016D,
    /// Meal Allowance.
    P5252006_0000017D,
    /// Celebrations.
    P5252006_0000018D,
    /// Social Security .
    P5252007_0000000D,
    /// Bank borrowings.
    P5353001_0000000D,
    /// Convertible bonds - Interest Expense (P&L).
    P5353001_0000001D,
    /// Dividends on redeemable preference shares.
    P5353001_0000002D,
    /// Lease liabilities.
    P5353001_0000003D,
    /// Unwinding of discount on provisions.
    P5353001_0000004D,
    /// Purchase Control.
    C6060001_0000000D,
    /// Sales Control.
    C6060002_0000000D,
    /// Tax Control.
    C6060003_0000000D,
    /// Escrowed Funds Control.
    C6060004_0000000D,
    /// Borrowings Control.
    C6060005_0000000D,
    /// Defi Borrowings Control.
    C6060006_0000000D,
    /// Liquidity Pool Control.
    C6060007_0000000D,
}

impl Encode for Ledger {
    fn size_hint(&self) -> usize {
        2
    }

    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(&(*self as u16).to_le_bytes())
    }
}

impl Decode for Ledger {
    fn decode<I: Input>(value: &mut I) -> Result<Self, Error> {
        let bytes = {
            let mut buffer = [0, 0];
            value.read(&mut buffer)?;
            buffer
        };
        let variant = u16::from_le_bytes(bytes);

        Self::from_repr(variant).ok_or_else(|| "Invalid variant value for `Ledger`".into())
    }

    fn encoded_fixed_size() -> Option<usize> {
        Some(2)
    }
}

impl TypeInfo for Ledger {
    type Identity = Self;

    fn type_info() -> Type {
        Type::builder()
            .path(Path::new(stringify!(Ledger), module_path!()))
            .composite(Fields::unnamed().field(|f| f.compact::<u16>().type_name("u16")))
    }
}

impl EncodeLike for Ledger {}
impl EncodeLike<u16> for Ledger {}
