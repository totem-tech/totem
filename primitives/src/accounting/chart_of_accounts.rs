use codec::{Decode, Encode, EncodeLike, Error, Input};
use scale_info::TypeInfo;
use strum::FromRepr;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromRepr, TypeInfo)]
#[repr(u16)]
pub enum Ledger {
    /// Bank Current Account.
    B1010001_0000000,
    /// Bank Savings Account.
    B1010002_0000000,
    /// Petty Cash.
    B1010003_0000000,
    /// Internal Address Balance (Network Currency).
    B1010004_0000000,
    /// Totem Runtime Deposit (Escrow).
    B1010005_0000000,
    /// Fixed deposits.
    B1010006_0000000,
    /// Prepaid Expenses on account (Operating Expense).
    B1010007_0000000,
    /// Director's loan account (asset until repaid).
    B1010008_0000000,
    /// Trade receivables - non-related parties.
    B1010009_0000000,
    /// Trade receivables - related parties.
    B1010010_0000000,
    /// Loan to non-related parties - Current Assets.
    B1010011_0000000,
    /// Loan to related parties - Current Assets.
    B1010011_0000001,
    /// Other receivables - Current Assets.
    B1010011_0000002,
    /// Finance lease receivables - Current Assets.
    B1010011_0000003,
    /// Staff loans - Current Assets.
    B1010011_0000004,
    /// Government grant receivables.
    B1010011_0000005,
    /// Allowance for doubtful debts .
    B1010012_0000000,
    /// Other .
    B1010012_0000001,
    /// Impairment loss on contract assets .
    B1010012_0000002,
    /// Raw materials.
    B1010013_0000000,
    /// Work in progress.
    B1010013_0000001,
    /// Finished goods.
    B1010013_0000002,
    /// Advances to suppliers (Trade Supplies).
    B1010014_0000000,
    /// Contract assets.
    B1010015_0000000,
    /// Equity securities - Fair value through P&L.
    B1010016_0000000,
    /// Assets of disposal group classified as held-for-sale.
    B1010017_0000000,
    /// Loan notes - floating or fixed.
    B1010018_0000000,
    /// Interest rate swaps - Current Assets.
    B1010019_0000000,
    /// Currency forwards - Current Assets.
    B1010020_0000000,
    /// Commodity forwards - Current Assets.
    B1010021_0000000,
    /// Cryptocurrency (UTXO).
    B1114001_0000000,
    /// Cryptocurrency (Nonce).
    B1114001_0000001,
    /// Cryptocurrency (Parachain).
    B1114001_0000002,
    /// Cryptocurrency (Third-party custodian).
    B1114001_0000003,
    /// Cryptocurrency (Other).
    B1114001_0000004,
    /// Fungible tokens (smart contract).
    B1114002_0000000,
    /// Fungible tokens (Third-party custodian).
    B1114002_0000001,
    /// Non-Fungible tokens (smart contract).
    B1114003_0000002,
    /// Non-Fungible tokens (Third-party custodian).
    B1114003_0000003,
    /// Liquidity Pool Pair (smart contract).
    B1114004_0000000,
    /// Cryprto Loan Collateral.
    B1114005_0000001,
    /// Impairment Loss Cryptocurrency (UTXO).
    B1114006_0000002,
    /// Impairment Loss Cryptocurrency (Nonce).
    B1114006_0000003,
    /// Impairment Loss Cryptocurrency (Parachain).
    B1114006_0000004,
    /// Impairment Loss Cryptocurrency (Third-party custodian).
    B1114006_0000000,
    /// Impairment Loss Cryptocurrency (Other).
    B1114006_0000001,
    /// Impairment Loss Fungible tokens (smart contract).
    B1114007_0000002,
    /// Impairment Loss Fungible tokens (Third-party custodian).
    B1114007_0000003,
    /// Impairment Loss Non-Fungible tokens (smart contract).
    B1114008_0000000,
    /// Impairment Loss Non-Fungible tokens (Third-party custodian).
    B1114008_0000001,
    /// Impairment Loss Liquidity Pool Pair (smart contract).
    B1114009_0000000,
    /// Impairment Loss Crypto Loan Collateral.
    B1114010_0000000,
    /// Land.
    B1211001_0000000,
    /// Buildings.
    B1211002_0000000,
    /// Furniture fixtures and fittings.
    B1211003_0000000,
    /// Plant and equipment.
    B1211004_0000000,
    /// Motor vehicles.
    B1211005_0000000,
    /// Supplies.
    B1211006_0000000,
    /// Computer and IT equipment.
    B1211007_0000000,
    /// Right-of-use assets.
    B1211008_0000000,
    /// Assets under construction.
    B1211009_0000000,
    /// Leasehold improvements.
    B1211010_0000000,
    /// Land - Accumulated depreciation.
    B1211011_0000000,
    /// Buildings- Accumulated depreciation.
    B1211011_0000001,
    /// Furniture fixtures and fittings - Accumulated depreciation.
    B1211011_0000002,
    /// Plant and equipment - Accumulated depreciation.
    B1211011_0000003,
    /// Motor vehicles - Accumulated depreciation.
    B1211011_0000004,
    /// Supplies - Accumulated depreciation.
    B1211011_0000005,
    /// Computer and IT equipment - Accumulated depreciation.
    B1211011_0000006,
    /// Right-of-use assets - Accumulated depreciation.
    B1211011_0000007,
    /// Assets under construction - Accumulated depreciation.
    B1211011_0000008,
    /// Leasehold improvements - Accumulated depreciation.
    B1211011_0000009,
    /// e.g. Films, copyrights or other intellectual property rights - Accumulated depreciation.
    B1312001_0000000,
    /// Trademark and patents (BS).
    B1312002_0000000,
    /// Computer software licences.
    B1312003_0000000,
    /// External Network Cryptocurrency.
    B1312004_0000000,
    /// Goodwill.
    B1312005_0000000,
    /// Trademark and patents - Accumulated amortisation.
    B1312006_0000000,
    /// Computer software licences - Accumulated amortisation.
    B1312006_0000001,
    /// Others.
    B1312006_0000002,
    /// Impairment loss on intangible assets.
    B1312007_0000000,
    /// Investment in subsidiaries.
    B1413001_0000000,
    /// Investment in joint venture.
    B1413002_0000000,
    /// Investment in associates.
    B1413003_0000000,
    /// Investment properties.
    B1413004_0000000,
    /// Equity securities  - Fair value through other comprehensive income.
    B1413005_0000000,
    /// Debt securities.
    B1413006_0000001,
    /// Deferred income tax - Non-Current Assets.
    B1413007_0000000,
    /// Impairment of fixed assets.
    B1413008_0000000,
    /// Impairment investment in subsidiaries.
    B1413008_0000001,
    /// Impairment investment in joint venture.
    B1413008_0000002,
    /// Impairment investment in associates.
    B1413008_0000003,
    /// Non-listed debt instruments.
    B1413009_0000000,
    /// Mandatorily measured at FVTPL (e.g. redeemable preference shares).
    B1413009_0000001,
    /// Convertible bonds - Non-Current Assets (BS).
    B1413009_0000002,
    /// Loan to non-related parties - Non-Current Assets.
    B1413010_0000000,
    /// Loan to related parties - Non-Current Assets.
    B1413010_0000001,
    /// Other receivables - Non-Current Assets.
    B1413010_0000002,
    /// Finance lease receivables - Non-Current Assets.
    B1413010_0000003,
    /// Staff loans - Non-Current Assets.
    B1413010_0000004,
    /// Indemnification assets.
    B1413010_0000005,
    /// Interest rate swaps - Non-Current Assets.
    B1413011_0000000,
    /// Currency forwards - Non-Current Assets.
    B1413011_0000001,
    /// Commodity forwards - Non-Current Assets.
    B1413011_0000002,
    /// Sales Tax by Jurisdiction  (Can be a negative balance for reclaims [technically an asset]).
    B2020001_0000000,
    /// Federal or State Tax By Jurisdiction on Sales.
    B2020002_0000000,
    /// Accounts payable (Trade creditors).
    B2020003_0000000,
    /// Current portion of long-term Debt (non-trade).
    B2020003_0000001,
    /// Short-term Loans (Payable).
    B2020003_0000002,
    /// Salaries Payable.
    B2020004_0000000,
    /// Wages Payable.
    B2020005_0000000,
    /// Commission Payable.
    B2020006_0000000,
    /// Freight Payable.
    B2020007_0000000,
    /// Other Accrued Expenses Payable.
    B2020008_0000000,
    /// Payroll Tax By Jurisdiction Liabilities.
    B2020009_0000000,
    /// Interest Payable.
    B2020010_0000000,
    /// Advances from customers.
    B2020011_0000000,
    /// Lawsuits and Legal Costs Payable.
    B2020012_0000000,
    /// Accounts payable (Trade creditors).
    B2020013_0000000,
    /// Short-term Loans (Payable).
    B2020013_0000001,
    /// Current portion of long-term Debt (non-trade).
    B2020013_0000002,
    /// Financial guarantees.
    B2020014_0000000,
    /// Contract liabilities.
    B2020015_0000000,
    /// Bank overdrafts.
    B2020016_0000000,
    /// Current portion of lease liabilities.
    B2020017_0000000,
    /// Warranty / Legal claims / Others - Current Liabilities (BS).
    B2020018_0000000,
    /// Unused leave - Current Liabilities (BS).
    B2020018_0000001,
    /// Interest rate swaps - Current Liabilities.
    B2020019_0000000,
    /// Currency forwards - Current Liabilities.
    B2020019_0000001,
    /// Commodity forwards - Current Liabilities.
    B2020019_0000002,
    /// Dividend payable.
    B2020020_0000000,
    /// Current portion of bank borrowings (Payable).
    B2020021_0000000,
    /// Income Tax By Jurisdiction Payable (Witholding Tax).
    B2121001_0000000,
    /// Deferred Revenues (Unearned Revenues).
    B2121002_0000000,
    /// Bonds Payable.
    B2121003_0000000,
    /// Notes (Loans) Payable.
    B2121004_0000000,
    /// Waranty Liabilities.
    B2121005_0000000,
    /// Corporation tax by jurisdiction payable (posted at end of period or end of year).
    B2121006_0000000,
    /// Installment Loans (Payable).
    B2121007_0000000,
    /// Portion of bank borrowings (Payable) - Non-Current Liabilities.
    B2121008_0000000,
    /// Mortgage or Property Loans (Payable).
    B2121009_0000000,
    /// Redeemable preference shares.
    B2121010_0000000,
    /// Portion of lease liabilities - Non-Current Liabilities.
    B2121011_0000000,
    /// Contingent consideration payable.
    B2121012_0000000,
    /// Deferred income tax - Non-Current Liabilities.
    B2121013_0000000,
    /// Non-current portion of long-term Debt (non-trade).
    B2121014_0000000,
    /// Non-current portion of long-term Debt (non-trade).
    B2121014_0000001,
    /// Warranty / Legal claims / Others - Non-Current Liabilities (BS).
    B2121015_0000000,
    /// Interest rate swaps - Non-Current Liabilities.
    B2121016_0000000,
    /// Currency forwards - Non-Current Liabilities.
    B2121016_0000001,
    /// Commodity forwards - Non-Current Liabilities.
    B2121016_0000002,
    /// Personal Net Worth.
    B3030001_0000000,
    /// Owner's Equity.
    B3030002_0000000,
    /// Corporation tax by jurisdiction (calculated after P&L).
    B3131001_0000000,
    /// Ordinary shares.
    B3232001_0000000,
    /// Preference shares.
    B3232002_0000000,
    /// Treasury shares.
    B3232003_0000000,
    /// Share application monies.
    B3333001_0000000,
    /// Share option reserve.
    B3333002_0000000,
    /// Fair value gains / (losses) on financial assets at FVOCI (debt instruments).
    B3333003_0000000,
    /// Fair value gains / (losses) on financial assets at FVOCI (equity instruments).
    B3333004_0000001,
    /// Cash flow hedges.
    B3333005_0000000,
    /// Equity component of convertible bonds.
    B3333006_0000000,
    /// Revaluation of property, plant and equipment.
    B3333007_0000000,
    /// Dividend paid.
    B3434001_0000000,
    /// Retained Earnings.
    B3434002_0000000,
    /// Sales of services.
    P4040001_0000000,
    /// Sales of goods.
    P4040002_0000000,
    /// Sales returns and allowances.
    P4040003_0000000,
    /// Sales discounts.
    P4040004_0000000,
    /// Freight Billable.
    P4040005_0000000,
    /// Commission Billable.
    P4040006_0000000,
    /// Miscellaneous Income.
    P4040007_0000000,
    /// Royalty income.
    P4141001_0000000,
    /// FInancial assets measured at amortised cost.
    P4141002_0000000,
    /// Investments.
    P4141002_0000001,
    /// Trade receivables.
    P4141002_0000002,
    /// Bank deposits.
    P4141002_0000003,
    /// Loans to an associate / subsidiary.
    P4141002_0000004,
    /// Debt investments measured at FVOCI.
    P4141002_0000005,
    /// Dividend income.
    P4141003_0000000,
    /// Rental income.
    P4141004_0000000,
    /// Grant income.
    P4141005_0000000,
    /// Fair value gains / (losses) on financial assets and liabilities at FVTPL.
    P4141006_0000000,
    /// Fair value gains / (losses) on derivative financial instruments.
    P4141007_0000000,
    /// Ineffectiveness on fair value / cash flow hedges.
    P4141008_0000000,
    /// Financial assets at FVOCI, reclassified from OCI on disposal.
    P4141009_0000000,
    /// Fair value gains / (losses) on investment properties.
    P4141010_0000000,
    /// Fair value gains / (loss) on contingent consideration .
    P4141011_0000000,
    /// Share of profit / (loss) of associates and joint ventures.
    P4241001_0000000,
    /// Gains / (Losses) on sale / disposal of property, plant and equipment.
    P4241002_0000000,
    /// Profit / (loss) from discontinued operations.
    P4241003_0000000,
    /// Debt instruments .
    P4342001_0000000,
    /// Equity instruments.
    P4342002_0000001,
    /// Fair value gains / (losses) on cash flow hedges.
    P4342003_0000000,
    /// Share of other comprehensive income of associates.
    P4342004_0000000,
    /// Reclassification adjustments (for items that may be reclassified subsequently to profit or loss).
    P4342005_0000000,
    /// Revaluation gains on property, plant and equipment.
    P4342006_0000000,
    /// Fair Value gains / (loses) holding Cryptocurrency (UTXO).
    P4342007_0000000,
    /// Fair Value gains / (loses) holding Cryptocurrency (Nonce).
    P4342007_0000001,
    /// Fair Value gains / (loses) holding Cryptocurrency (Parachain).
    P4342007_0000002,
    /// Fair Value gains / (loses) holding Cryptocurrency (Third-party custodian).
    P4342007_0000003,
    /// Fair Value gains / (loses) holding Cryptocurrency (Other).
    P4342007_0000004,
    /// Fair Value gains / (loses) holding Fungible tokens (smart contract).
    P4342008_0000000,
    /// Fair Value gains / (loses) holding Fungible tokens (Third-party custodian).
    P4342008_0000001,
    /// Fair Value gains / (loses) holding Non-Fungible tokens (smart contract).
    P4342009_0000000,
    /// Fair Value gains / (loses) holding Non-Fungible tokens (Third-party custodian).
    P4342009_0000001,
    /// Fair Value gains / (loses) holding Liquidity Pool Pair (smart contract).
    P4342010_0000000,
    /// Fair Value gains / (loses) holding Crypto Loan Collateral.
    P4342011_0000000,
    /// Gains / (Losses) on sale / disposal of Cryptocurrency Assets.
    P4342012_0000000,
    /// Raw materials - changes in inventories.
    P5050001_0000000,
    /// Work in progress - changes in inventories.
    P5050001_0000001,
    /// Finished goods - changes in inventories.
    P5050001_0000002,
    /// Purchases - direct material costs.
    P5050001_0000003,
    /// Direct labour costs.
    P5050001_0000004,
    /// Manufacturing / Factory overhead.
    P5050001_0000005,
    /// Purchases returns and allowances.
    P5050001_0000006,
    /// Purchase discount.
    P5050001_0000007,
    /// Inventory write-down / Reversal of inventory write-down.
    P5050001_0000008,
    /// Inter-Entity Charge In.
    P5050002_0000000,
    /// Inter-Entity Charge Out.
    P5050002_0000001,
    /// IT Distribution.
    P5050002_0000002,
    /// Distribution To Capital.
    P5050002_0000003,
    /// Allocations.
    P5050002_0000004,
    /// Allocation By Country.
    P5050002_0000005,
    /// Facilities Allocation.
    P5050002_0000006,
    /// Assess Research And Development Recharges.
    P5050002_0000007,
    /// General Project Costs.
    P5050003_0000000,
    /// Depreciation Land.
    P5050003_0000001,
    /// Depreciation Buildings.
    P5050003_0000002,
    /// Depreciation Furniture fixtures and fittings.
    P5050003_0000003,
    /// Depreciation Plant and equipment.
    P5050003_0000004,
    /// Depreciation Motor vehicles.
    P5050003_0000005,
    /// Depreciation Supplies.
    P5050003_0000006,
    /// Depreciation Computer and IT equipment.
    P5050003_0000007,
    /// Depreciation Right-of-use assets.
    P5050003_0000008,
    /// Depreciation Leasehold improvements.
    P5050003_0000009,
    /// Amortisation of Others intangible assets.
    P5050003_0000010,
    /// Amortisation of Trademark and patents.
    P5050003_0000011,
    /// Amortisation of Computer software licences.
    P5050003_0000012,
    /// Amortization Government Grants.
    P5050003_0000013,
    /// Gain Loss On Miscellaneous Sales.
    P5050003_0000014,
    /// Non Capital Project Expense.
    P5050003_0000015,
    /// Consumption Other.
    P5050004_0000000,
    /// Trials.
    P5050004_0000001,
    /// Samples.
    P5050004_0000002,
    /// Warranties.
    P5050005_0000000,
    /// Tax (Corporation Tax).
    P5050006_0000000,
    /// Employee Tax.
    P5050007_0000000,
    /// Tax Property.
    P5050007_0000001,
    /// Tax Trade Licences.
    P5050007_0000002,
    /// Tax Enviromental.
    P5050007_0000003,
    /// Road Tax.
    P5050007_0000004,
    /// Taxes Other.
    P5050007_0000005,
    /// Taxes Other Alternate Acct.
    P5050007_0000006,
    /// Non-Deductible Fines Penalties.
    P5050007_0000007,
    /// Penalties.
    P5050007_0000008,
    /// Licenses Fees Certificates Other Taxes.
    P5050007_0000009,
    /// Car Tax.
    P5050007_0000010,
    /// Capital Gains Tax on Crypto Assets.
    P5050007_0000011,
    /// Claims  .
    P5050008_0000000,
    /// Commissions Manual.
    P5050009_0000000,
    /// Commissions Intercompany Expense.
    P5050009_0000001,
    /// Commissions Intercompany Income.
    P5050009_0000002,
    /// Samples Consumption Marketing.
    P5050010_0000000,
    /// Promotions.
    P5050010_0000001,
    /// Advertising Expenses.
    P5050010_0000002,
    /// Advertising Promotions Plant Visit.
    P5050010_0000003,
    /// Design And Artwork.
    P5050010_0000004,
    /// Advertising Promotions Gifts.
    P5050010_0000005,
    /// Advertising Promotions Meetings Seminars.
    P5050010_0000006,
    /// Advertising Promotions Printed Material.
    P5050010_0000007,
    /// Advertising Promotions Outdoor Advertising.
    P5050010_0000008,
    /// Advertising Promotions Presentation Material.
    P5050010_0000009,
    /// Distributor Events.
    P5050010_0000010,
    /// Market Research.
    P5050010_0000011,
    /// Fairs And Exhibitions.
    P5050010_0000012,
    /// Media Print.
    P5050010_0000013,
    /// Media Radio.
    P5050010_0000014,
    /// Media Television.
    P5050010_0000015,
    /// Media Other.
    P5050010_0000016,
    /// Samples.
    P5050010_0000017,
    /// Custom Gifts Taxable.
    P5050010_0000018,
    /// Custom Gifts Non Taxable.
    P5050010_0000019,
    /// Technical Service Fees.
    P5050010_0000020,
    /// Literature.
    P5050010_0000021,
    /// Brand Merchandise.
    P5050010_0000022,
    /// Trade Shows Exhibit Fees.
    P5050010_0000023,
    /// Agency Fees.
    P5050010_0000024,
    /// Audio Visuals.
    P5050010_0000025,
    /// Marketing Events Days In The Field.
    P5050010_0000026,
    /// Marketing Miscellaneous Expense.
    P5050010_0000027,
    /// Marketing Cost Sharing.
    P5050010_0000028,
    /// Consulting Fees  .
    P5050011_0000000,
    /// Miscellaneous Service.
    P5050012_0000000,
    /// Analytical Testing.
    P5050012_0000001,
    /// Communications Tarriffs.
    P5050012_0000002,
    /// Experimental Testing.
    P5050012_0000003,
    /// Financial Services.
    P5050012_0000004,
    /// Freight Expenses.
    P5050012_0000005,
    /// Housekeeping & Laundry.
    P5050012_0000006,
    /// Language Translation Services.
    P5050012_0000007,
    /// Mail Delivery Services.
    P5050012_0000008,
    /// Miscellaneous Fees.
    P5050012_0000009,
    /// Contractors.
    P5050012_0000010,
    /// Technical Assistance.
    P5050012_0000011,
    /// Laboratory Expense.
    P5050012_0000012,
    /// Labour.
    P5050012_0000013,
    /// Printing.
    P5050012_0000014,
    /// Warehouse.
    P5050012_0000015,
    /// Photographs Graphics.
    P5050012_0000016,
    /// Project Engineering.
    P5050012_0000017,
    /// Research Assistance.
    P5050012_0000018,
    /// Research Shopping Expense.
    P5050012_0000019,
    /// Other service for Production.
    P5050012_0000020,
    /// Software Maintenance Fees.
    P5050012_0000021,
    /// Tax Preparation Fees.
    P5050012_0000022,
    /// Testing Services.
    P5050012_0000023,
    /// Travel Expenses Per Diem.
    P5050013_0000000,
    /// Travel General.
    P5050013_0000001,
    /// Travel Expenses Air Tickets.
    P5050013_0000002,
    /// Travel Expenses Air Tickets Outside Country Of Residence.
    P5050013_0000003,
    /// Travel Expenses Miles.
    P5050013_0000004,
    /// Taxi & Other Personal Transport.
    P5050013_0000005,
    /// Lodging Hotel Costs.
    P5050014_0000000,
    /// Meetings Conferences.
    P5050015_0000000,
    /// Other Grower Meetings.
    P5050015_0000001,
    /// Key Distributor Meetings.
    P5050015_0000002,
    /// Travel Foreign.
    P5050015_0000003,
    /// Travel Non Employee.
    P5050015_0000004,
    /// Travel Tech Meetings Conferences.
    P5050015_0000005,
    /// Business Meals Entertainment.
    P5050016_0000000,
    /// Business Meals Within Country Of Residence.
    P5050016_0000001,
    /// Business Meals Outside Country Of Residence.
    P5050016_0000002,
    /// Entertainment Non-Deductable.
    P5050016_0000003,
    /// Meals Entertainment Field Sales.
    P5050016_0000004,
    /// Meals Entertainment Training Education.
    P5050016_0000005,
    /// Meals Entertainment Business Development (No Meeting).
    P5050016_0000006,
    /// Meals Entertainment Business Development.
    P5050016_0000007,
    /// Other Travel Expenses  .
    P5050017_0000000,
    /// Public Relations.
    P5050018_0000000,
    /// Automobile Expense.
    P5050019_0000000,
    /// Automobile Expense Parking.
    P5050019_0000001,
    /// Automobile Expense Company Car.
    P5050019_0000002,
    /// Automobile Expense Telephone.
    P5050019_0000003,
    /// Automobile Expense Rental.
    P5050019_0000004,
    /// Automobile Expense Repairs.
    P5050019_0000005,
    /// Automobile Expense Car Financing.
    P5050019_0000006,
    /// Automobile Expense Fuel.
    P5050019_0000007,
    /// Leased Autos.
    P5050019_0000008,
    /// Equipment.
    P5050020_0000000,
    /// Plant Maintenance .
    P5050021_0000000,
    /// Cellular Usage.
    P5050022_0000000,
    /// Communication Remote Access.
    P5050022_0000001,
    /// Miscellaneous Data Communications.
    P5050022_0000002,
    /// Telephone Facsimiles.
    P5050022_0000003,
    /// Rent Warehouse.
    P5050023_0000000,
    /// Non Lease Rental.
    P5050023_0000001,
    /// Rent Building Office.
    P5050023_0000002,
    /// Rent Land.
    P5050023_0000003,
    /// Rent Equipment.
    P5050023_0000004,
    /// Sub Lease Income.
    P5050023_0000005,
    /// Rent Other Facilities.
    P5050023_0000006,
    /// Office Lab Furniture.
    P5050024_0000000,
    /// Service Contracts.
    P5050024_0000001,
    /// Repairs.
    P5050024_0000002,
    /// Repairs Equipment.
    P5050024_0000003,
    /// IT Maintenance.
    P5050024_0000004,
    /// Maintenance.
    P5050024_0000005,
    /// Maintenance Equip.
    P5050024_0000006,
    /// Maint Other.
    P5050024_0000007,
    /// Maintenance Other Miscellaneous Hardware.
    P5050024_0000008,
    /// Maintenance Other Office Cleaning.
    P5050024_0000009,
    /// Maintenance Building.
    P5050024_0000010,
    /// Tools.
    P5050024_0000011,
    /// Services.
    P5050024_0000012,
    /// Purchased Supplies.
    P5050024_0000013,
    /// Pm Purchased Equipment Non Capital.
    P5050024_0000014,
    /// Miscellaneous Maintenance.
    P5050024_0000015,
    /// Supplies Other.
    P5050025_0000000,
    /// Supplies Office.
    P5050025_0000001,
    /// Supplies Office Photcopy Document Handling.
    P5050025_0000002,
    /// Supplies Lab.
    P5050025_0000003,
    /// Supplies Safety.
    P5050025_0000004,
    /// Supplies Production.
    P5050025_0000005,
    /// Supplies Maintenance.
    P5050025_0000006,
    /// Supplies Industrial (Chemicals etc...).
    P5050025_0000007,
    /// Supplies Cleaning.
    P5050025_0000008,
    /// Equipment Electrical Equipment.
    P5050026_0000000,
    /// Computer Related Equipment.
    P5050026_0000001,
    /// Software Licenses.
    P5050026_0000002,
    /// Electricity Purchased.
    P5050026_0000003,
    /// Water Purchased.
    P5050026_0000004,
    /// Gas Purchased.
    P5050026_0000005,
    /// Fuel.
    P5050026_0000006,
    /// Utilities Combined.
    P5050026_0000007,
    /// Cloud Storage.
    P5050026_0000008,
    /// Datacentres.
    P5050026_0000009,
    /// Insurance Other.
    P5050027_0000000,
    /// Insurance Other Export Credit.
    P5050027_0000001,
    /// Self Insurance.
    P5050027_0000002,
    /// Motor vehicle insurance.
    P5050027_0000003,
    /// Medical insurance.
    P5050027_0000004,
    /// Life insurance.
    P5050027_0000005,
    /// Travel insurance.
    P5050027_0000006,
    /// Non-exec Directors Fees, & Other Governance Costs.
    P5050028_0000000,
    /// External Corporate Secretarial fees.
    P5050029_0000000,
    /// Litigation.
    P5050029_0000001,
    /// Outside Counsel Fees.
    P5050029_0000002,
    /// Licenses Permits.
    P5050029_0000003,
    /// Legal Fees International.
    P5050029_0000004,
    /// Legal Fees Other.
    P5050029_0000005,
    /// FCPA Gifts.
    P5050029_0000006,
    /// FCPA Travel.
    P5050029_0000007,
    /// FCPA Meals Entertainment.
    P5050029_0000008,
    /// Business Contributions.
    P5050029_0000009,
    /// Charitable Contributions.
    P5050029_0000010,
    /// Donations Disallowed.
    P5050029_0000011,
    /// Local Tax.
    P5050029_0000012,
    /// Audit Fees External.
    P5050029_0000013,
    /// Auditor Firm Tax Fees.
    P5050029_0000014,
    /// Auditor Firm Other Fees.
    P5050029_0000015,
    /// Audit Fees.
    P5050029_0000016,
    /// Patents.
    P5050029_0000017,
    /// Product Registrations.
    P5050029_0000018,
    /// Trademark and patents (P&L).
    P5050029_0000019,
    /// Design Rights.
    P5050029_0000020,
    /// Network Transaction Fees.
    P5050030_0000000,
    /// Bank Charges.
    P5050030_0000001,
    /// Administration Business Tax.
    P5050030_0000002,
    /// Corporate Membership.
    P5050030_0000003,
    /// Memberships Subscriptions.
    P5050030_0000004,
    /// Other Non Deductible.
    P5050030_0000005,
    /// Postage Expenses.
    P5050030_0000006,
    /// Courrier Expenses.
    P5050030_0000007,
    /// Management Adjustment.
    P5050030_0000008,
    /// Collection Expense.
    P5050030_0000009,
    /// Credit Reports.
    P5050030_0000010,
    /// Other Miscellaneous Expense.
    P5050030_0000011,
    /// Miscellaneous Month End Accruals.
    P5050030_0000012,
    /// Sponsorship.
    P5050030_0000013,
    /// Books Library.
    P5050030_0000014,
    /// Equipment Purchased Non-Capitalised.
    P5050030_0000015,
    /// Software Purchased Non-Capitalised.
    P5050030_0000016,
    /// Miscellaneous Reimbursement Expenses.
    P5050030_0000017,
    /// Recovered Expenses Canteen Receipts.
    P5050030_0000018,
    /// Recovered Expense Miscellaneous Charge Out.
    P5050030_0000019,
    /// Recovered Expenses Insurance.
    P5050030_0000020,
    /// Recovered Expenses Car Contract.
    P5050030_0000021,
    /// Recovered Expenses Other.
    P5050030_0000022,
    /// Recovered Expenses Salaries.
    P5050030_0000023,
    /// Recovered Expenses Wages.
    P5050030_0000024,
    /// Procurement Card Purchases.
    P5050030_0000025,
    /// Administration Service Fees.
    P5050030_0000026,
    /// Other Miscellaneous Income Expense.
    P5050030_0000027,
    /// Bad Debt Expense / Reversal.
    P5050031_0000000,
    /// Not Used.
    P5050032_0000000,
    /// Shipping Cost.
    P5050033_0000000,
    /// Other Costs.
    P5050033_0000001,
    /// Standard Cost Intercompany.
    P5050033_0000002,
    /// Finished Goods Consumption Each.
    P5050033_0000003,
    /// Samples Consumption.
    P5050033_0000004,
    /// Semifinished Consumption.
    P5050033_0000005,
    /// Freight Non-Customer.
    P5050033_0000006,
    /// Freight From or To Warehouse.
    P5050033_0000007,
    /// Duties On Purchases.
    P5050033_0000008,
    /// Handling Warehouse.
    P5050033_0000009,
    /// Transportation.
    P5050033_0000010,
    /// Waste Disposal.
    P5050033_0000011,
    /// Customs Fees.
    P5050033_0000013,
    /// Sponsorship / Donations.
    P5050033_0000014,
    /// Financial guarantee fees.
    P5050034_0000001,
    /// Royalty expenses.
    P5050035_0000002,
    /// Extraordinary expenses.
    P5050037_0000004,
    /// Impairment loss on financial assets.
    P5050038_0000005,
    /// Impairment loss on contract assets.
    P5050039_0000006,
    /// Warranty / Legal claims / Others - Expenses (P&L).
    P5050040_0000000,
    /// Unused leave - Expenses (P&L).
    P5050040_0000001,
    /// Property, plant and equipment written off.
    P5050041_0000000,
    /// Information Technology Miscellaneous.
    P5151001_0000000,
    /// Secondary Costs.
    P5151001_0000001,
    /// Reporting Adjustment.
    P5151001_0000002,
    /// Salaries.
    P5252001_0000000,
    /// Salaries 13th Month.
    P5252001_0000001,
    /// Salaries Achievement Awards.
    P5252001_0000002,
    /// Salaries Representation Allowance.
    P5252001_0000003,
    /// Salaries Fees.
    P5252001_0000004,
    /// Salaried Operations.
    P5252001_0000005,
    /// Wages 13th Month.
    P5252001_0000006,
    /// Payroll Fees.
    P5252001_0000007,
    /// Payroll Luncheon Vouchers.
    P5252001_0000008,
    /// Social Security Employer Contribution.
    P5252001_0000009,
    /// Fringes Variable.
    P5252001_0000010,
    /// Social Security Other.
    P5252001_0000011,
    /// Social Security Provision.
    P5252001_0000012,
    /// Payroll Social Security.
    P5252001_0000013,
    /// Accruals Social Security Charges 13th Month.
    P5252001_0000014,
    /// Accruals Social Security Charges Holiday Pay.
    P5252001_0000015,
    /// Housing Tax.
    P5252001_0000016,
    /// Apprenticeship Tax.
    P5252001_0000017,
    /// Work Council.
    P5252001_0000018,
    /// Severence Pay.
    P5252001_0000019,
    /// Transportation Subsidy.
    P5252001_0000020,
    /// Wages Hourly.
    P5252001_0000021,
    /// Salaries Wages Per Diem.
    P5252001_0000022,
    /// Overtime Payments.
    P5252001_0000023,
    /// Sick Pay.
    P5252001_0000024,
    /// Holiday Pay.
    P5252001_0000025,
    /// Vacation Pay.
    P5252001_0000026,
    /// Other Employee Compensation.
    P5252001_0000027,
    /// Savings Plan.
    P5252001_0000028,
    /// Field Incentive Plan.
    P5252001_0000029,
    /// Pension.
    P5252001_0000030,
    /// Insurance Plan Live Medical.
    P5252001_0000031,
    /// Insurance Work Accident.
    P5252001_0000032,
    /// Insurance Private Collective.
    P5252001_0000033,
    /// Pension Costs Company Portion.
    P5252001_0000034,
    /// Private Patient Plan.
    P5252001_0000035,
    /// Early Retirement Plans.
    P5252001_0000036,
    /// Employee share option expense.
    P5252001_0000037,
    /// School Expenses.
    P5252002_0000000,
    /// Home Leave -Expat expenses.
    P5252002_0000001,
    /// Expat Cost.
    P5252002_0000002,
    /// Assignee Allowances Other.
    P5252002_0000003,
    /// Employe Language Lessons.
    P5252002_0000004,
    /// Employee Moving Expense.
    P5252002_0000005,
    /// Employee Assistance Program.
    P5252002_0000006,
    /// Employee Housing Expense.
    P5252002_0000007,
    /// Expat Other Benefits.
    P5252002_0000008,
    /// Assignee Tax.
    P5252002_0000009,
    /// Incentive Plan.
    P5252004_0000000,
    /// Incentive Plan Overhead.
    P5252005_0000000,
    /// Employee Service Awards.
    P5252006_0000000,
    /// Security Services.
    P5252006_0000001,
    /// Health Safety.
    P5252006_0000002,
    /// Cleaning And Security Services Alternate Account.
    P5252006_0000003,
    /// Cafeteria Service.
    P5252006_0000004,
    /// Medical Benefits.
    P5252006_0000005,
    /// Medical Services and Supplies.
    P5252006_0000006,
    /// Recreation Sports Social.
    P5252006_0000007,
    /// Recruiting.
    P5252006_0000008,
    /// Employee Agency Fees.
    P5252006_0000009,
    /// Transport Of Personal.
    P5252006_0000010,
    /// Education Training.
    P5252006_0000011,
    /// Education Training Associated Travel Costs.
    P5252006_0000012,
    /// Training Tax.
    P5252006_0000013,
    /// Tuition.
    P5252006_0000014,
    /// Tuition Taxable.
    P5252006_0000015,
    /// Laundry Clothing.
    P5252006_0000016,
    /// Meal Allowance.
    P5252006_0000017,
    /// Celebrations.
    P5252006_0000018,
    /// Social Security  .
    P5252007_0000000,
    /// Bank borrowings.
    P5353001_0000000,
    /// Convertible bonds - Interest Expense (P&L).
    P5353001_0000001,
    /// Dividends on redeemable preference shares.
    P5353001_0000002,
    /// Lease liabilities.
    P5353001_0000003,
    /// Unwinding of discount on provisions.
    P5353001_0000004,
    /// Purchase Control.
    C6060001_0000000,
    /// Sales Control.
    C6060002_0000000,
    /// Tax Control.
    C6060003_0000000,
    /// Escrowed Funds Control.
    C6060004_0000000,
    /// Borrowings Control.
    C6060005_0000000,
    /// Defi Borrowings Control.
    C6060006_0000000,
    /// Liquidity Pool Control.
    C6060007_0000000,
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

impl EncodeLike for Ledger {}
impl EncodeLike<u16> for Ledger {}
