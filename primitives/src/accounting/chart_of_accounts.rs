//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.


//*****************************************************************************//
// The structure of the hierarchy of the Chart of Accounts is listed below the code.
//*****************************************************************************//

use codec::{Decode, Encode,};
use scale_info::TypeInfo;

/// Cost of Goods Sold (COGS)
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0001_ {
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS)  
    /// 000: Raw materials - changes in inventories 
    P50_0001_D000,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS)  
    /// 001: Work in progress - changes in inventories 
    P50_0001_D001,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS)  
    /// 002: Finished goods - changes in inventories 
    P50_0001_D002,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS)  
    /// 003: Purchases - direct material costs 
    P50_0001_D003,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS) 
    /// 004: Direct labour costs 
    P50_0001_D004,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS) 
    /// 005: Manufacturing / Factory overhead 
    P50_0001_D005,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS)  
    /// 006: Purchases returns and allowances 
    P50_0001_D006,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS) 
    /// 007: Purchase discount 
    P50_0001_D007,
    
    /// Operating  Expenses
    /// _0001_: Cost of Goods Sold (COGS) 
    /// 008: Inventory write-down / Reversal of inventory write-down 
    P50_0001_D008,
}	
/// Charges In Out
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0002_ {
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 000: Inter-Entity Charge In 
    P50_0002_D000,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 001: Inter-Entity Charge Out 
    P50_0002_D001,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 002: IT Distribution 
    P50_0002_D002,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 003: Distribution To Capital 
    P50_0002_D003,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 004: Allocations 
    P50_0002_D004,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 005: Allocation By Country 
    P50_0002_D005,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 006: Facilities Allocation 
    P50_0002_D006,
    
    /// Operating  Expenses
    /// _0002_: Charges In Out 
    /// 007: Assess Research And Development Recharges 
    P50_0002_D007,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 000: General Project Costs 
    P50_0003_D000,
}	
/// Depreciation Depletion Amortization  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0003_ {
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 001: Depreciation Land 
    P50_0003_D001,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 002: Depreciation Buildings 
    P50_0003_D002,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 003: Depreciation Furniture fixtures and fittings 
    P50_0003_D003,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 004: Depreciation Plant and equipment 
    P50_0003_D004,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 005: Depreciation Motor vehicles 
    P50_0003_D005,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 006: Depreciation Supplies 
    P50_0003_D006,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 007: Depreciation Computer and IT equipment 
    P50_0003_D007,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 008: Depreciation Right-of-use assets 
    P50_0003_D008,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 009: Depreciation Leasehold improvements 
    P50_0003_D009,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 010: Amortisation of Others intangible assets 
    P50_0003_D010,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 011: Amortisation of Trademark and patents 
    P50_0003_D011,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 012: Amortisation of Computer software licences 
    P50_0003_D012,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 013: Amortization Government Grants 
    P50_0003_D013,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 014: Gain Loss On Miscellaneous Sales 
    P50_0003_D014,
    
    /// Operating  Expenses
    /// _0003_: Depreciation Depletion Amortization   
    /// 015: Non Capital Project Expense 
    P50_0003_D015,
}	
/// Field Trials  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0004_ {
    /// Operating  Expenses
    /// _0004_: Field Trials   
    /// 000: Consumption Other 
    P50_0004_D000,
    
    /// Operating  Expenses
    /// _0004_: Field Trials   
    /// 001: Trials 
    P50_0004_D001,
    
    /// Operating  Expenses
    /// _0004_: Field Trials   
    /// 002: Samples 
    P50_0004_D002,
}	
/// Warranties
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0005_ {
    /// _0005_: Warranties 
    P50_0005_D000,
}	
/// Tax (Corporation Tax)
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0006_ {
    /// _0006_: Tax (Corporation Tax) 
    P50_0006_D000,
    
}	
/// Tax Fines & Penalties
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0007_ {
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 000: Employee Tax 
    P50_0007_D000,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 001: Tax Property 
    P50_0007_D001,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 002: Tax Trade Licences 
    P50_0007_D002,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 003: Tax Enviromental 
    P50_0007_D003,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 004: Road Tax 
    P50_0007_D004,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 005: Taxes Other 
    P50_0007_D005,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 006: Taxes Other Alternate Acct 
    P50_0007_D006,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 007: Non-Deductible Fines Penalties 
    P50_0007_D007,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 008: Penalties 
    P50_0007_D008,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 009: Licenses Fees Certificates Other Taxes 
    P50_0007_D009,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 010: Car Tax 
    P50_0007_D010,
    
    /// Operating  Expenses
    /// _0007_: Tax Fines & Penalties 
    /// 011: Capital Gains Tax on Crypto Assets 
    P50_0007_D011,
}	
/// Claims  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0008_ {
    /// _0008_: Claims   
    P50_0008_D000,
    
}	
/// Commissions  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0009_ {
    /// Operating  Expenses
    /// _0009_: Commissions   
    /// 000: Commissions Manual 
    P50_0009_D000,
    
    /// Operating  Expenses
    /// _0009_: Commissions   
    /// 001: Commissions Intercompany Expense 
    P50_0009_D001,
    
    /// Operating  Expenses
    /// _0009_: Commissions   
    /// 002: Commissions Intercompany Income 
    P50_0009_D002,
    
}	
/// Marketing Programs  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0010_ {
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 000: Samples Consumption Marketing 
    P50_0010_D000,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 001: Promotions 
    P50_0010_D001,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 002: Advertising Expenses 
    P50_0010_D002,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 003: Advertising Promotions Plant Visit 
    P50_0010_D003,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 004: Design And Artwork 
    P50_0010_D004,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 005: Advertising Promotions Gifts 
    P50_0010_D005,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 006: Advertising Promotions Meetings Seminars 
    P50_0010_D006,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 007: Advertising Promotions Printed Material 
    P50_0010_D007,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 008: Advertising Promotions Outdoor Advertising 
    P50_0010_D008,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 009: Advertising Promotions Presentation Material 
    P50_0010_D009,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 010: Distributor Events 
    P50_0010_D010,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 011: Market Research 
    P50_0010_D011,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 012: Fairs And Exhibitions 
    P50_0010_D012,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 013: Media Print 
    P50_0010_D013,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 014: Media Radio 
    P50_0010_D014,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 015: Media Television 
    P50_0010_D015,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 016: Media Other 
    P50_0010_D016,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 017: Samples 
    P50_0010_D017,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 018: Custom Gifts Taxable 
    P50_0010_D018,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 019: Custom Gifts Non Taxable 
    P50_0010_D019,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 020: Technical Service Fees 
    P50_0010_D020,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 021: Literature 
    P50_0010_D021,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 022: Brand Merchandise 
    P50_0010_D022,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 023: Trade Shows Exhibit Fees 
    P50_0010_D023,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 024: Agency Fees 
    P50_0010_D024,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 025: Audio Visuals 
    P50_0010_D025,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 026: Marketing Events Days In The Field 
    P50_0010_D026,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 027: Marketing Miscellaneous Expense 
    P50_0010_D027,
    
    /// Operating  Expenses
    /// _0010_: Marketing Programs   
    /// 028: Marketing Cost Sharing 
    P50_0010_D028,
}	
/// Consulting Fees  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0011_ {
    /// _0011_: Consulting Fees   
    P50_0011_D000,
    
}	
/// Services  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0012_ {
    /// Operating  Expenses
    /// _0012_: Services   
    /// 000: Miscellaneous Service 
    P50_0012_D000,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 001: Analytical Testing 
    P50_0012_D001,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 002: Communications Tarriffs 
    P50_0012_D002,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 003: Experimental Testing 
    P50_0012_D003,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 004: Financial Services 
    P50_0012_D004,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 005: Freight Expenses 
    P50_0012_D005,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 006: Housekeeping & Laundry 
    P50_0012_D006,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 007: Language Translation Services 
    P50_0012_D007,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 008: Mail Delivery Services 
    P50_0012_D008,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 009: Miscellaneous Fees 
    P50_0012_D009,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 010: Contractors 
    P50_0012_D010,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 011: Technical Assistance 
    P50_0012_D011,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 012: Laboratory Expense 
    P50_0012_D012,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 013: Labour 
    P50_0012_D013,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 014: Printing 
    P50_0012_D014,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 015: Warehouse 
    P50_0012_D015,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 016: Photographs Graphics 
    P50_0012_D016,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 017: Project Engineering 
    P50_0012_D017,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 018: Research Assistance 
    P50_0012_D018,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 019: Research Shopping Expense 
    P50_0012_D019,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 020: Other service for Production 
    P50_0012_D020,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 021: Software Maintenance Fees 
    P50_0012_D021,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 022: Tax Preparation Fees 
    P50_0012_D022,
    
    /// Operating  Expenses
    /// _0012_: Services   
    /// 023: Testing Services 
    P50_0012_D023,
    
}	
/// Travel Expenses  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0013_ {
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 000: Travel Expenses Per Diem 
    P50_0013_D000,
    
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 001: Travel General 
    P50_0013_D001,
    
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 002: Travel Expenses Air Tickets 
    P50_0013_D002,
    
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 003: Travel Expenses Air Tickets Outside Country Of Residence 
    P50_0013_D003,
    
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 004: Travel Expenses Miles 
    P50_0013_D004,
    
    /// Operating  Expenses
    /// _0013_: Travel Expenses   
    /// 005: Taxi & Other Personal Transport 
    P50_0013_D005,
}	
/// Hotels  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0014_ {
    /// Operating  Expenses
    /// _0014_: Hotels   
    /// 000: Lodging Hotel Costs 
    P50_0014_D000,
}	
/// Meetings & Conferences  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0015_ {
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 000: Meetings Conferences 
    P50_0015_D000,
    
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 001: Other Grower Meetings 
    P50_0015_D001,
    
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 002: Key Distributor Meetings 
    P50_0015_D002,
    
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 003: Travel Foreign 
    P50_0015_D003,
    
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 004: Travel Non Employee 
    P50_0015_D004,
    
    /// Operating  Expenses
    /// _0015_: Meetings & Conferences   
    /// 005: Travel Tech Meetings Conferences 
    P50_0015_D005,
}	
/// Restaurant Meals  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0016_ {
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 000: Business Meals Entertainment 
    P50_0016_D000,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 001: Business Meals Within Country Of Residence 
    P50_0016_D001,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 002: Business Meals Outside Country Of Residence 
    P50_0016_D002,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 003: Entertainment Non-Deductable 
    P50_0016_D003,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 004: Meals Entertainment Field Sales 
    P50_0016_D004,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 005: Meals Entertainment Training Education 
    P50_0016_D005,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 006: Meals Entertainment Business Development (No Meeting) 
    P50_0016_D006,
    
    /// Operating  Expenses
    /// _0016_: Restaurant Meals   
    /// 007: Meals Entertainment Business Development 
    P50_0016_D007,
}	
/// Other Travel Expenses  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0017_ {
    /// _0017_: Other Travel Expenses   
    P50_0017_D000,
}	
/// Cost Pooling
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0018_ {
    /// Operating  Expenses
    /// _0018_: Cost Pooling 
    /// 000: Public Relations 
    P50_0018_D000,
    
}	
/// Car Expenses  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0019_ {
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 000: Automobile Expense 
    P50_0019_D000,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 001: Automobile Expense Parking 
    P50_0019_D001,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 002: Automobile Expense Company Car 
    P50_0019_D002,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 003: Automobile Expense Telephone 
    P50_0019_D003,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 004: Automobile Expense Rental 
    P50_0019_D004,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 005: Automobile Expense Repairs 
    P50_0019_D005,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 006: Automobile Expense Car Financing 
    P50_0019_D006,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 007: Automobile Expense Fuel 
    P50_0019_D007,
    
    /// Operating  Expenses
    /// _0019_: Car Expenses   
    /// 008: Leased Autos 
    P50_0019_D008,
}	
/// Equipment
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0020_ {
    /// Operating  Expenses
    /// _0020_: Equipment 
    P50_0020_D000,
    
}	
/// Plant Maintenance 
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0021_ {
    /// Operating  Expenses
    /// _0021_: Plant Maintenance  
    P50_0021_D000,
}	
/// Phones Telecom  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0022_ {
    /// Operating  Expenses
    /// _0022_: Phones Telecom   
    /// 000: Cellular Usage 
    P50_0022_D000,
    
    /// Operating  Expenses
    /// _0022_: Phones Telecom   
    /// 001: Communication Remote Access 
    P50_0022_D001,
    
    /// Operating  Expenses
    /// _0022_: Phones Telecom   
    /// 002: Miscellaneous Data Communications 
    P50_0022_D002,
    
    /// Operating  Expenses
    /// _0022_: Phones Telecom   
    /// 003: Telephone Facsimiles 
    P50_0022_D003,
}	
/// Rental Leases  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0023_ {
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 000: Rent Warehouse 
    P50_0023_D000,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 001: Non Lease Rental 
    P50_0023_D001,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 002: Rent Building Office 
    P50_0023_D002,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 003: Rent Land 
    P50_0023_D003,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 004: Rent Equipment 
    P50_0023_D004,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 005: Sub Lease Income 
    P50_0023_D005,
    
    /// Operating  Expenses
    /// _0023_: Rental Leases   
    /// 006: Rent Other Facilities 
    P50_0023_D006,
    
}	
/// Repair Maintenance  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0024_ {
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 000: Office Lab Furniture 
    P50_0024_D000,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 001: Service Contracts 
    P50_0024_D001,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 002: Repairs 
    P50_0024_D002,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 003: Repairs Equipment 
    P50_0024_D003,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 004: IT Maintenance 
    P50_0024_D004,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 005: Maintenance 
    P50_0024_D005,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 006: Maintenance Equip 
    P50_0024_D006,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 007: Maint Other 
    P50_0024_D007,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 008: Maintenance Other Miscellaneous Hardware 
    P50_0024_D008,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 009: Maintenance Other Office Cleaning 
    P50_0024_D009,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 010: Maintenance Building 
    P50_0024_D010,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 011: Tools 
    P50_0024_D011,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 012: Services 
    P50_0024_D012,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 013: Purchased Supplies 
    P50_0024_D013,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 014: Pm Purchased Equipment Non Capital 
    P50_0024_D014,
    
    /// Operating  Expenses
    /// _0024_: Repair Maintenance   
    /// 015: Miscellaneous Maintenance 
    P50_0024_D015,
    
}	
/// Supplies  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0025_ {
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 000: Supplies Other 
    P50_0025_D000,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 001: Supplies Office 
    P50_0025_D001,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 002: Supplies Office Photcopy Document Handling 
    P50_0025_D002,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 003: Supplies Lab 
    P50_0025_D003,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 004: Supplies Safety 
    P50_0025_D004,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 005: Supplies Production 
    P50_0025_D005,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 006: Supplies Maintenance 
    P50_0025_D006,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 007: Supplies Industrial (Chemicals etc...) 
    P50_0025_D007,
    
    /// Operating  Expenses
    /// _0025_: Supplies   
    /// 008: Supplies Cleaning 
    P50_0025_D008,
}	
/// Utilities
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0026_ {
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 000: Equipment Electrical Equipment 
    P50_0026_D000,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 001: Computer Related Equipment 
    P50_0026_D001,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 002: Software Licenses 
    P50_0026_D002,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 003: Electricity Purchased 
    P50_0026_D003,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 004: Water Purchased 
    P50_0026_D004,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 005: Gas Purchased 
    P50_0026_D005,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 006: Fuel 
    P50_0026_D006,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 007: Utilities Combined 
    P50_0026_D007,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 008: Cloud Storage 
    P50_0026_D008,
    
    /// Operating  Expenses
    /// _0026_: Utilities 
    /// 009: Datacentres 
    P50_0026_D009,
    
}	
/// Insurance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0027_ {
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 000: Insurance Other 
    P50_0027_D000,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 001: Insurance Other Export Credit 
    P50_0027_D001,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 002: Self Insurance 
    P50_0027_D002,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 003: Motor vehicle insurance 
    P50_0027_D003,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 004: Medical insurance 
    P50_0027_D004,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 005: Life insurance 
    P50_0027_D005,
    
    /// Operating  Expenses
    /// _0027_: Insurance 
    /// 006: Travel insurance 
    P50_0027_D006,
}	
/// Corporate Governance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0028_ {
    /// Operating  Expenses
    /// _0028_: Corporate Governance 
    /// 000: Non-exec Directors Fees, & Other Governance Costs 
    P50_0028_D000,
}	
/// Legal Fees
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0029_ {
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 000: External Corporate Secretarial fees 
    P50_0029_D000,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 001: Litigation 
    P50_0029_D001,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 002: Outside Counsel Fees 
    P50_0029_D002,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 003: Licenses Permits 
    P50_0029_D003,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 004: Legal Fees International 
    P50_0029_D004,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 005: Legal Fees Other 
    P50_0029_D005,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 006: FCPA Gifts 
    P50_0029_D006,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 007: FCPA Travel 
    P50_0029_D007,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 008: FCPA Meals Entertainment 
    P50_0029_D008,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 009: Business Contributions 
    P50_0029_D009,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 010: Charitable Contributions 
    P50_0029_D010,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 011: Donations Disallowed 
    P50_0029_D011,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 012: Local Tax 
    P50_0029_D012,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 013: Audit Fees External 
    P50_0029_D013,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 014: Auditor Firm Tax Fees 
    P50_0029_D014,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 015: Auditor Firm Other Fees 
    P50_0029_D015,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 016: Audit Fees 
    P50_0029_D016,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 017: Patents 
    P50_0029_D017,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 018: Product Registrations 
    P50_0029_D018,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 019: Trademark and patents (P&L) 
    P50_0029_D019,
    
    /// Operating  Expenses
    /// _0029_: Legal Fees 
    /// 020: Design Rights 
    P50_0029_D020,
    
}	
/// Administration Cost
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0030_ {
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 000: Network Transaction Fees 
    P50_0030_D000,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 001: Bank Charges 
    P50_0030_D001,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 002: Administration Business Tax 
    P50_0030_D002,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 003: Corporate Membership 
    P50_0030_D003,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 004: Memberships Subscriptions 
    P50_0030_D004,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 005: Other Non Deductible 
    P50_0030_D005,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 006: Postage Expenses 
    P50_0030_D006,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 007: Courrier Expenses 
    P50_0030_D007,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 008: Management Adjustment 
    P50_0030_D008,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 009: Collection Expense 
    P50_0030_D009,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 010: Credit Reports 
    P50_0030_D010,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 011: Other Miscellaneous Expense 
    P50_0030_D011,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 012: Miscellaneous Month End Accruals 
    P50_0030_D012,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 013: Sponsorship 
    P50_0030_D013,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 014: Books Library 
    P50_0030_D014,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 015: Equipment Purchased Non-Capitalised 
    P50_0030_D015,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 016: Software Purchased Non-Capitalised 
    P50_0030_D016,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 017: Miscellaneous Reimbursement Expenses 
    P50_0030_D017,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 018: Recovered Expenses Canteen Receipts 
    P50_0030_D018,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 019: Recovered Expense Miscellaneous Charge Out 
    P50_0030_D019,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 020: Recovered Expenses Insurance 
    P50_0030_D020,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 021: Recovered Expenses Car Contract 
    P50_0030_D021,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 022: Recovered Expenses Other 
    P50_0030_D022,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 023: Recovered Expenses Salaries 
    P50_0030_D023,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 024: Recovered Expenses Wages 
    P50_0030_D024,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 025: Procurement Card Purchases 
    P50_0030_D025,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 026: Administration Service Fees 
    P50_0030_D026,
    
    /// Operating  Expenses
    /// _0030_: Administration Cost 
    /// 027: Other Miscellaneous Income Expense 
    P50_0030_D027,
}	
/// Bad Debts
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0031_ {
    /// Operating  Expenses
    /// _0031_: Bad Debts 
    /// 000: Bad Debt Expense / Reversal 
    P50_0031_D000,
}	
/// Miscellaneous Expenses  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0032_ {
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 000: Shipping Cost 
    P50_0032_D000,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 001: Other Costs 
    P50_0032_D001,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 002: Standard Cost Intercompany 
    P50_0032_D002,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 003: Finished Goods Consumption Each 
    P50_0032_D003,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 004: Samples Consumption 
    P50_0032_D004,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 005: Semifinished Consumption 
    P50_0032_D005,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 006: Freight Non-Customer 
    P50_0032_D006,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 007: Freight From or To Warehouse 
    P50_0032_D007,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 008: Duties On Purchases 
    P50_0032_D008,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 009: Handling Warehouse 
    P50_0032_D009,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 010: Transportation 
    P50_0032_D010,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 011: Waste Disposal 
    P50_0032_D011,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 013: Customs Fees 
    P50_0032_D013,
    
    /// Operating  Expenses
    /// _0032_: Miscellaneous Expenses   
    /// 014: Sponsorship / Donations 
    P50_0032_D014,
}	
/// Financial guarantee fees
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0033_ {
    /// Operating  Expenses
    /// _0033_: Financial guarantee fees 
    P50_0033_D000,
}	
/// Royalty expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0034_ {
    /// Operating  Expenses
    /// _0034_: Royalty expenses 
    P50_0034_D000,
}	
/// Extraordinary expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0035_ {
    /// Operating  Expenses
    /// _0035_: Extraordinary expenses 
    P50_0035_D000,
}	
/// Impairment loss on financial assets
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0036_ {
    /// Operating  Expenses
    /// _0036_: Impairment loss on financial assets 
    P50_0036_D000,
}	
/// Impairment loss on contract assets
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0037_ {
    /// Operating  Expenses
    /// _0037_: Impairment loss on contract assets 
    P50_0037_D000,
}	
/// Provisions 
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0038_ {
    /// Operating  Expenses
    /// _0038_: Provisions  
    /// 000: Warranty / Legal claims / Others - Expenses (P&L) 
    P50_0038_D000,
    
    /// Operating  Expenses
    /// _0038_: Provisions  
    /// 001: Unused leave - Expenses (P&L) 
    P50_0038_D001,
}	
/// Property, plant and equipment written off
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _0039_ {
    /// Operating  Expenses
    /// _0039_: Property, plant and equipment written off 
    P50_0039_D000,
}	
/// Other Miscellaneous Charges
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _1001_ {
    /// Other Operating Expense
    /// _1001_: Other Miscellaneous Charges 
    /// 000: Information Technology Miscellaneous 
    P51_1001_D000,
    
    /// Other Operating Expense
    /// _1001_: Other Miscellaneous Charges 
    /// 001: Secondary Costs 
    P51_1001_D001,
    
    /// Other Operating Expense
    /// _1001_: Other Miscellaneous Charges 
    /// 002: Reporting Adjustment 
    P51_1001_D002,
}	
/// Salaries  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2001_ {
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 000: Salaries 
    P52_2001_D000,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 001: Salaries 13th Month 
    P52_2001_D001,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 002: Salaries Achievement Awards 
    P52_2001_D002,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 003: Salaries Representation Allowance 
    P52_2001_D003,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 004: Salaries Fees 
    P52_2001_D004,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 005: Salaried Operations 
    P52_2001_D005,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 006: Wages 13th Month 
    P52_2001_D006,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 007: Payroll Fees 
    P52_2001_D007,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 008: Payroll Luncheon Vouchers 
    P52_2001_D008,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 009: Social Security Employer Contribution 
    P52_2001_D009,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 010: Fringes Variable 
    P52_2001_D010,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 011: Social Security Other 
    P52_2001_D011,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 012: Social Security Provision 
    P52_2001_D012,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 013: Payroll Social Security 
    P52_2001_D013,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 014: Accruals Social Security Charges 13th Month 
    P52_2001_D014,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 015: Accruals Social Security Charges Holiday Pay 
    P52_2001_D015,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 016: Housing Tax 
    P52_2001_D016,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 017: Apprenticeship Tax 
    P52_2001_D017,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 018: Work Council 
    P52_2001_D018,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 019: Severence Pay 
    P52_2001_D019,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 020: Transportation Subsidy 
    P52_2001_D020,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 021: Wages Hourly 
    P52_2001_D021,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 022: Salaries Wages Per Diem 
    P52_2001_D022,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 023: Overtime Payments 
    P52_2001_D023,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 024: Sick Pay 
    P52_2001_D024,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 025: Holiday Pay 
    P52_2001_D025,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 026: Vacation Pay 
    P52_2001_D026,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 027: Other Employee Compensation 
    P52_2001_D027,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 028: Savings Plan 
    P52_2001_D028,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 029: Field Incentive Plan 
    P52_2001_D029,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 030: Pension 
    P52_2001_D030,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 031: Insurance Plan Live Medical 
    P52_2001_D031,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 032: Insurance Work Accident 
    P52_2001_D032,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 033: Insurance Private Collective 
    P52_2001_D033,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 034: Pension Costs Company Portion 
    P52_2001_D034,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 035: Private Patient Plan 
    P52_2001_D035,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 036: Early Retirement Plans 
    P52_2001_D036,
    
    /// Personnel Costs
    /// _2001_: Salaries   
    /// 037: Employee share option expense 
    P52_2001_D037,
}	
/// Expat Expenses  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2002_ {
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 000: School Expenses 
    P52_2002_D000,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 001: Home Leave -Expat expenses 
    P52_2002_D001,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 002: Expat Cost 
    P52_2002_D002,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 003: Assignee Allowances Other 
    P52_2002_D003,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 004: Employe Language Lessons 
    P52_2002_D004,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 005: Employee Moving Expense 
    P52_2002_D005,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 006: Employee Assistance Program 
    P52_2002_D006,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 007: Employee Housing Expense 
    P52_2002_D007,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 008: Expat Other Benefits 
    P52_2002_D008,
    
    /// Personnel Costs
    /// _2002_: Expat Expenses   
    /// 009: Assignee Tax 
    P52_2002_D009,
    
}	
/// Incentive Plan
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2003_ {
    /// Personnel Costs
    /// _2003_: Incentive Plan 
    P52_2003_D000,
}	
/// Incentive Plan Overhead
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2004_ {
    /// Personnel Costs
    /// _2004_: Incentive Plan Overhead 
    P52_2004_D000,
}	
/// Employee Services  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2005_ {
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 000: Employee Service Awards 
    P52_2005_D000,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 001: Security Services 
    P52_2005_D001,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 002: Health Safety 
    P52_2005_D002,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 003: Cleaning And Security Services Alternate Account 
    P52_2005_D003,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 004: Cafeteria Service 
    P52_2005_D004,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 005: Medical Benefits 
    P52_2005_D005,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 006: Medical Services and Supplies 
    P52_2005_D006,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 007: Recreation Sports Social 
    P52_2005_D007,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 008: Recruiting 
    P52_2005_D008,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 009: Employee Agency Fees 
    P52_2005_D009,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 010: Transport Of Personal 
    P52_2005_D010,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 011: Education Training 
    P52_2005_D011,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 012: Education Training Associated Travel Costs 
    P52_2005_D012,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 013: Training Tax 
    P52_2005_D013,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 014: Tuition 
    P52_2005_D014,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 015: Tuition Taxable 
    P52_2005_D015,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 016: Laundry Clothing 
    P52_2005_D016,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 017: Meal Allowance 
    P52_2005_D017,
    
    /// Personnel Costs
    /// _2005_: Employee Services   
    /// 018: Celebrations 
    P52_2005_D018,
    
}	
/// Social Security  
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _2006_ {
    /// Personnel Costs
    /// _2006_: Social Security   
    P52_2006_D000,
    
}	
/// Interest expense
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum _3001_ {
    /// Finance Costs
    /// _3001_: Interest expense 
    /// 000: Bank borrowings 
    P53_3001_D000,
    
    /// Finance Costs
    /// _3001_: Interest expense 
    /// 001: Convertible bonds - Interest Expense (P&L) 
    P53_3001_D001,
    
    /// Finance Costs
    /// _3001_: Interest expense 
    /// 002: Dividends on redeemable preference shares 
    P53_3001_D002,
    
    /// Finance Costs
    /// _3001_: Interest expense 
    /// 003: Lease liabilities 
    P53_3001_D003,
    
    /// Finance Costs
    /// _3001_: Interest expense 
    /// 004: Unwinding of discount on provisions 
    P53_3001_D004,
}	
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum CurrentAssets {
    /// _0001_: Bank Current Account 
    B10_0001_D000,
    
    /// _0002_: Bank Savings Account 
    B10_0002_D000,
    
    /// _0003_: Petty Cash 
    B10_0003_D000,
    
    /// _0004_: Internal Balance 
    B10_0004_D000,
    
    /// _0005_: Totem Escrow Deposit 
    B10_0005_D000,
    
    /// _0006_: Fixed deposits 
    B10_0006_D000,
    
    /// _0007_: Prepaid Expenses on account (Operating Expense) 
    B10_0007_D000,
    
    /// _0008_: Director's loan account (asset until repaid) 
    B10_0008_D000,
    
    /// _0009_: Accounts receivable (Sales Control Account or Trade Debtor's Account) 
    /// 000: Trade receivables - non-related parties 
    B10_0009_D000,
    
    /// _0009_: Accounts receivable (Sales Control Account or Trade Debtor's Account) 
    /// 001: Trade receivables - related parties 
    B10_0009_D001,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 000: Loan to non-related parties - Current Assets 
    B10_0010_D000,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 001: Loan to related parties - Current Assets 
    B10_0010_D001,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 002: Other receivables - Current Assets 
    B10_0010_D002,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 003: Finance lease receivables - Current Assets 
    B10_0010_D003,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 004: Staff loans - Current Assets 
    B10_0010_D004,
    
    /// _0010_: Accrued Revenues / Receivables 
    /// 005: Government grant receivables 
    B10_0010_D005,
    
    /// _0011_: Impairment loss on financial assets 
    /// 000: Allowance for doubtful debts  
    B10_0011_C000,
    
    /// _0011_: Impairment loss on financial assets 
    /// 001: Other  
    B10_0011_C001,
    
    /// _0012_: Impairment loss on contract assets  
    B10_0012_C000,
    
    /// _0013_: Inventory 
    /// 000: Raw materials 
    B10_0013_D000,
    
    /// _0013_: Inventory 
    /// 001: Work in progress 
    B10_0013_D001,
    
    /// _0013_: Inventory 
    /// 002: Finished goods 
    B10_0013_D002,
    
    /// _0014_: Advances to suppliers (Trade Supplies) 
    B10_0014_D000,
    
    /// _0015_: Contract assets 
    B10_0015_D000,
    
    /// _0016_: Financial assets, at fair value through profit or loss (FVTPL) 
    /// 000: Equity securities - Fair value through P&L 
    B10_0016_D000,
    
    /// _0017_: Assets of disposal group classified as held-for-sale 
    B10_0017_D000,
    
    /// _0018_: Other investments at amortised cost 
    /// 000: Loan notes - floating or fixed 
    B10_0018_D000,
    
    /// _0019_: Derivative financial instruments 
    /// 000: Interest rate swaps - Current Assets 
    B10_0019_D000,
    
    /// _0019_: Derivative financial instruments 
    /// 001: Currency forwards - Current Assets 
    B10_0019_D001,
    
    /// _0019_: Derivative financial instruments 
    /// 002: Commodity forwards - Current Assets 
    B10_0019_D002,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum CurrentAssetsCrypto {
    /// _4001_: Cryptocurrency 
    /// 000: UTXO 
    B11_4001_D000,
    
    /// _4001_: Cryptocurrency 
    /// 001: Nonce 
    B11_4001_D001,
    
    /// _4001_: Cryptocurrency 
    /// 002: Parachain 
    B11_4001_D002,
    
    /// _4001_: Cryptocurrency 
    /// 003: Third-party custodian 
    B11_4001_D003,
    
    /// _4001_: Cryptocurrency 
    /// 004: Other 
    B11_4001_D004,
    
    /// _4002_: Fungible tokens 
    /// 000: Smart contract 
    B11_4002_D000,
    
    /// _4002_: Fungible tokens 
    /// 001: Third-party custodian 
    B11_4002_D001,
    
    /// _4003_: Non-Fungible tokens 
    /// 000: Smart contract 
    B11_4003_D000,
    
    /// _4003_: Non-Fungible tokens 
    /// 001: Third-party custodian 
    B11_4003_D001,
    
    /// _4004_: Liquidity Pool Pair 
    /// 000: Smart Contract 
    B11_4004_D000,
    
    /// _4005_: Crypto Loan Collateral 
    B11_4005_D000,
    
    /// _4006_: Impairment - Cryptocurrency 
    /// 000: UTXO 
    B11_4006_C000,
    
    /// _4006_: Impairment - Cryptocurrency 
    /// 001: Nonce 
    B11_4006_C001,
    
    /// _4006_: Impairment - Cryptocurrency 
    /// 002: Parachain 
    B11_4006_C002,
    
    /// _4006_: Impairment - Cryptocurrency 
    /// 003: Third-party custodian 
    B11_4006_C003,
    
    /// _4006_: Impairment - Cryptocurrency 
    /// 004: Other 
    B11_4006_C004,
    
    /// _4007_: Impairment - Fungible tokens 
    /// 000: Smart contract 
    B11_4007_C000,
    
    /// _4007_: Impairment - Fungible tokens 
    /// 001: Third-party custodian 
    B11_4007_C001,
    
    /// _4008_: Impairment - Non-Fungible tokens 
    /// 000: Smart contract 
    B11_4008_C000,
    
    /// _4008_: Impairment - Non-Fungible tokens 
    /// 001: Third-party custodian 
    B11_4008_C001,
    
    /// _4009_: Impairment - Liquidity Pool Pair 
    /// 000: Smart Contract 
    B11_4009_C000,
    
    /// _4010_: Impairment - Crypto Loan Collateral 
    B11_4010_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum FixedAssets {
    /// _1001_: Property, plant and equipment 
    /// 000: Land 
    B12_1001_D000,
    
    /// _1001_: Property, plant and equipment 
    /// 001: Buildings 
    B12_1001_D001,
    
    /// _1001_: Property, plant and equipment 
    /// 002: Furniture fixtures and fittings 
    B12_1001_D002,
    
    /// _1001_: Property, plant and equipment 
    /// 003: Plant and equipment 
    B12_1001_D003,
    
    /// _1001_: Property, plant and equipment 
    /// 004: Motor vehicles 
    B12_1001_D004,
    
    /// _1001_: Property, plant and equipment 
    /// 005: Supplies 
    B12_1001_D005,
    
    /// _1001_: Property, plant and equipment 
    /// 006: Computer and IT equipment 
    B12_1001_D006,
    
    /// _1001_: Property, plant and equipment 
    /// 007: Right-of-use assets 
    B12_1001_D007,
    
    /// _1001_: Property, plant and equipment 
    /// 008: Assets under conenumion 
    B12_1001_D008,
    
    /// _1001_: Property, plant and equipment 
    /// 009: Leasehold improvements 
    B12_1001_D009,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 000: Land 
    B12_1002_C000,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 001: Buildings 
    B12_1002_C001,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 002: Furniture fixtures and fittings 
    B12_1002_C002,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 003: Plant and equipment 
    B12_1002_C003,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 004: Motor vehicles 
    B12_1002_C004,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 005: Supplies 
    B12_1002_C005,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 006: Computer and IT equipment 
    B12_1002_C006,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 007: Right-of-use assets 
    B12_1002_C007,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 008: Assets under conenumion 
    B12_1002_C008,
    
    /// _1002_: Property, plant and equipment - Accumulated Depreciation 
    /// 009: Leasehold improvements 
    B12_1002_C009,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum IntagibleAssets {
    /// _2001_: Other Accumulated Depreciation 
    /// 000: Other Films, copyrights or other intellectual property rights - Accumulated depreciation 
    B13_2001_D000,
    
    /// _2002_: Trademark and patents 
    /// 000: Trademark and patents (BS) 
    B13_2002_D000,
    
    /// _2003_: Computer software licences 
    B13_2003_D000,
    
    /// _2004_: External Network Cryptocurrency 
    B13_2004_D000,
    
    /// _2005_: Goodwill 
    B13_2005_D000,
    
    /// _2006_: Trademark and patents 
    /// 000: Trademark and patents - Accumulated amortisation 
    B13_2006_C000,
    
    /// _2006_: Accumulated amortisation 
    /// 001: Computer software licences - Accumulated amortisation 
    B13_2006_C001,
    
    /// _2006_: Accumulated amortisation 
    /// 002: Others 
    B13_2006_C002,
    
    /// _2007_: Impairment loss on intangible assets 
    B13_2007_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum NonCurrentAssets {
    /// _3001_: Investment in subsidiaries 
    B14_3001_D000,
    
    /// _3002_: Investment in joint venture 
    B14_3002_D000,
    
    /// _3003_: Investment in associates 
    B14_3003_D000,
    
    /// _3004_: Investment properties 
    B14_3004_D000,
    
    /// _3005_: Financial assets, at fair value through other comprehensive income (FVOCI) 
    /// 000: Equity securities  - Fair value through other comprehensive income 
    B14_3005_D000,
    
    /// _3005_: Financial assets, at fair value through other comprehensive income (FVOCI) 
    /// 001: Debt securities 
    B14_3005_D001,
    
    /// _3006_: Deferred income tax - Non-current Assets 
    B14_3006_D000,
    
    /// _3007_: Impairment loss on non-financial assets 
    /// 000: Impairment of fixed assets 
    B14_3007_C000,
    
    /// _3007_: Impairment loss on non-financial assets 
    /// 001: Impairment investment in subsidiaries 
    B14_3007_C001,
    
    /// _3007_: Impairment loss on non-financial assets 
    /// 002: Impairment investment in joint venture 
    B14_3007_C002,
    
    /// _3007_: Impairment loss on non-financial assets 
    /// 003: Impairment investment in associates 
    B14_3007_C003,
    
    /// _3008_: Financial assets, at fair value through profit or loss 
    /// 000: Non-listed debt instruments 
    B14_3008_D000,
    
    /// _3008_: Financial assets, at fair value through profit or loss 
    /// 001: Mandatorily measured at FVTPL (e.g. redeemable preference shares) 
    B14_3008_D001,
    
    /// _3008_: Financial assets, at fair value through profit or loss 
    /// 002: Convertible bonds - Non-current Assets (BS) 
    B14_3008_D002,
    
    /// _3009_: Other receivables 
    /// 000: Loan to non-related parties - Non-current Assets 
    B14_3009_D000,
    
    /// _3009_: Other receivables 
    /// 001: Loan to related parties - Non-current Assets 
    B14_3009_D001,
    
    /// _3009_: Other receivables 
    /// 002: Other receivables - Non-current Assets 
    B14_3009_D002,
    
    /// _3009_: Other receivables 
    /// 003: Finance lease receivables - Non-current Assets 
    B14_3009_D003,
    
    /// _3009_: Other receivables 
    /// 004: Staff loans - Non-current Assets 
    B14_3009_D004,
    
    /// _3009_: Other receivables 
    /// 005: Indemnification assets 
    B14_3009_D005,
    
    /// _3010_: Derivative financial instruments 
    /// 000: Interest rate swaps - Non-current Assets 
    B14_3010_D000,
    
    /// _3010_: Derivative financial instruments 
    /// 001: Currency forwards - Non-current Assets 
    B14_3010_D001,
    
    /// _3010_: Derivative financial instruments 
    /// 002: Commodity forwards - Non-current Assets 
    B14_3010_D002,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum CurrentLiabilities {
    /// _0001_: Sales Tax by Jurisdiction  (Can be a negative balance for reclaims [technically an asset]) 
    B20_0001_C000,
    
    /// _0002_: Federal or State Tax By Jurisdiction on Sales 
    B20_0002_C000,
    
    /// _0003_: Related parties 
    /// 000: Accounts payable (Trade creditors) 
    B20_0003_C000,
    
    /// _0003_: Related parties 
    /// 001: Current portion of long-term Debt (non-trade) 
    B20_0003_C001,
    
    /// _0003_: Related parties 
    /// 002: Short-term Loans (Payable) 
    B20_0003_C002,
    
    /// _0004_: Salaries Payable 
    B20_0004_C000,
    
    /// _0005_: Wages Payable 
    B20_0005_C000,
    
    /// _0006_: Commission Payable 
    B20_0006_C000,
    
    /// _0007_: Freight Payable 
    B20_0007_C000,
    
    /// _0008_: Other Accrued Expenses Payable 
    B20_0008_C000,
    
    /// _0009_: Payroll Tax By Jurisdiction Liabilities 
    B20_0009_C000,
    
    /// _0010_: Interest Payable 
    B20_0010_C000,
    
    /// _0011_: Advances from customers 
    B20_0011_C000,
    
    /// _0012_: Lawsuits and Legal Costs Payable 
    B20_0012_C000,
    
    /// _0013_: Non-related parties 
    /// 000: Accounts payable (Trade creditors) 
    B20_0013_C000,
    
    /// _0013_: Non-related parties 
    /// 001: Short-term Loans (Payable) 
    B20_0013_C001,
    
    /// _0013_: Non-related parties 
    /// 002: Current portion of long-term Debt (non-trade) 
    B20_0013_C002,
    
    /// _0014_: Financial guarantees 
    B20_0014_C000,
    
    /// _0015_: Contract liabilities 
    B20_0015_C000,
    
    /// _0016_: Bank overdrafts 
    B20_0016_C000,
    
    /// _0017_: Current portion of lease liabilities 
    B20_0017_C000,
    
    /// _0018_: Provisions  
    /// 000: Warranty / Legal claims / Others - Current Liabilities (BS) 
    B20_0018_C000,
    
    /// _0018_: Provisions  
    /// 001: Unused leave - Current Liabilities (BS) 
    B20_0018_C001,
    
    /// _0019_: Derivative financial instruments 
    /// 000: Interest rate swaps - Current Liabilities 
    B20_0019_C000,
    
    /// _0019_: Derivative financial instruments 
    /// 001: Currency forwards - Current Liabilities 
    B20_0019_C001,
    
    /// _0019_: Derivative financial instruments 
    /// 002: Commodity forwards - Current Liabilities 
    B20_0019_C002,
    
    /// _0020_: Dividend payable 
    B20_0020_C000,
    
    /// _0021_: Current portion of bank borrowings (Payable) 
    B20_0021_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum NonCurrentLiabilities {
    /// _1001_: Income Tax By Jurisdiction Payable (Witholding Tax) 
    B21_1001_C000,
    
    /// _1002_: Deferred Revenues (Unearned Revenues) 
    B21_1002_C000,
    
    /// _1003_: Bonds Payable 
    B21_1003_C000,
    
    /// _1004_: Notes (Loans) Payable 
    B21_1004_C000,
    
    /// _1005_: Waranty Liabilities 
    B21_1005_C000,
    
    /// _1006_: Corporation tax by jurisdiction payable (posted at end of period or end of year) 
    B21_1006_C000,
    
    /// _1007_: Installment Loans (Payable) 
    B21_1007_C000,
    
    /// _1008_: Portion of bank borrowings (Payable) - Non-current Liabilities 
    B21_1008_C000,
    
    /// _1009_: Mortgage or Property Loans (Payable) 
    B21_1009_C000,
    
    /// _1010_: Redeemable preference shares 
    B21_1010_C000,
    
    /// _1011_: Portion of lease liabilities - Non-current Liabilities 
    B21_1011_C000,
    
    /// _1012_: Contingent consideration payable 
    B21_1012_C000,
    
    /// _1013_: Deferred income tax - Non-current Liabilities 
    B21_1013_C000,
    
    /// _1014_: Related parties 
    /// 000: Non-current portion of long-term Debt (non-trade) 
    B21_1014_C000,
    
    /// _1014_: Non-related parties 
    /// 001: Non-current portion of long-term Debt (non-trade) 
    B21_1014_C001,
    
    /// _1015_: Provisions  
    /// 000: Warranty / Legal claims / Others - Non-current Liabilities (BS) 
    B21_1015_C000,
    
    /// _1016_: Derivative financial instruments 
    /// 000: Interest rate swaps - Non-current Liabilities 
    B21_1016_C000,
    
    /// _1016_: Derivative financial instruments 
    /// 001: Currency forwards - Non-current Liabilities 
    B21_1016_C001,
    
    /// _1016_: Derivative financial instruments 
    /// 002: Commodity forwards - Non-current Liabilities 
    B21_1016_C002,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum ShareholdersEquity {
    /// _0001_: Personal Net Worth 
    B30_0001_C000,
    
    /// _0002_: Owner's Equity 
    B30_0002_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherEquity {
    /// _1001_: Corporation tax by jurisdiction (calculated after P&L) 
    B31_1001_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum CapitalStock {
    /// _2001_: Ordinary shares 
    B32_2001_C000,
    
    /// _2002_: Preference shares 
    B32_2002_C000,
    
    /// _2003_: Treasury shares 
    B32_2003_D000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherReserves {
    /// _3001_: Share application monies 
    B33_3001_C000,
    
    /// _3002_: Share option reserve 
    B33_3002_C000,
    
    /// _3003_: Fair value reserve 
    /// 000: Fair value gains / (losses) on financial assets at FVOCI (debt instruments) 
    B33_3003_C000,
    
    /// _3003_: Fair value reserve 
    /// 001: Fair value gains / (losses) on financial assets at FVOCI (equity instruments) 
    B33_3003_C001,
    
    /// _3004_: Hedging reserve 
    /// 000: Cash flow hedges 
    B33_3004_C000,
    
    /// _3005_: Equity component of convertible bonds 
    B33_3005_C000,
    
    /// _3006_: Asset revaluation reserve 
    /// 000: Revaluation of property, plant and equipment 
    B33_3006_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum RetainedEarnings {
    /// _4001_: Dividend paid 
    B34_4001_D000,
    
    /// _4002_: Retained Earnings 
    B34_4002_C000,              
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum Sales {
    /// _0001_: Sales of services 
    P40_0001_C000,
    
    /// _0002_: Sales of goods 
    P40_0002_C000,
    
    /// _0003_: Sales returns and allowances 
    P40_0003_D000,
    
    /// _0004_: Sales discounts 
    P40_0004_C000,
    
    /// _0005_: Freight Billable 
    P40_0005_C000,
    
    /// _0006_: Commission Billable 
    P40_0006_C000,
    
    /// _0007_: Miscellaneous Income 
    P40_0007_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherIncome {
    /// _1001_: Royalty income 
    P41_1001_C000,
    
    /// _1002_: Interest income 
    /// 000: FInancial assets measured at amortised cost 
    P41_1002_C000,
    
    /// _1002_: Interest income 
    /// 001: Investments 
    P41_1002_C001,
    
    /// _1002_: Interest income 
    /// 002: Trade receivables 
    P41_1002_C002,
    
    /// _1002_: Interest income 
    /// 003: Bank deposits 
    P41_1002_C003,
    
    /// _1002_: Interest income 
    /// 004: Loans to an associate / subsidiary 
    P41_1002_C004,
    
    /// _1002_: Interest income 
    /// 005: Debt investments measured at FVOCI 
    P41_1002_C005,
    
    /// _1003_: Dividend income 
    P41_1003_C000,
    
    /// _1004_: Rental income 
    P41_1004_C000,
    
    /// _1005_: Grant income 
    P41_1005_C000,
    
    /// _1006_: Fair value gains / (losses) on financial assets and liabilities at FVTPL 
    P41_1006_C000,
    
    /// _1007_: Fair value gains / (losses) on derivative financial instruments 
    P41_1007_C000,
    
    /// _1008_: Ineffectiveness on fair value / cash flow hedges 
    P41_1008_C000,
    
    /// _1009_: Financial assets at FVOCI, reclassified from OCI on disposal 
    P41_1009_C000,
    
    /// _1010_: Fair value gains / (losses) on investment properties 
    P41_1010_C000,
    
    /// _1011_: Fair value gains / (loss) on contingent consideration  
    P41_1011_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherOperatingIncome {
    /// _1001_: Share of profit / (loss) of associates and joint ventures 
    P42_1001_C000,
    
    /// _1002_: Gains / (Losses) on sale / disposal of property, plant and equipment 
    P42_1002_C000,
    
    /// _1003_: Profit / (loss) from discontinued operations 
    P42_1003_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherComprehensiveIncome {
    /// _2001_: Fair value gains / (losses) on financial assets at FVOCI 
    /// 000: Debt instruments  
    P43_2001_C000,
    
    /// _2001_: Fair value gains / (losses) on financial assets at FVOCI 
    /// 001: Equity instruments 
    P43_2001_C001,
    
    /// _2002_: Fair value gains / (losses) on cash flow hedges 
    P43_2002_C000,
    
    /// _2003_: Share of other comprehensive income of associates 
    P43_2003_C000,
    
    /// _2004_: Reclassification adjustments (for items that may be reclassified subsequently to profit or loss) 
    P43_2004_C000,
    
    /// _2005_: Revaluation gains on property, plant and equipment 
    P43_2005_C000,
    
    /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (UTXO) 
    P43_2006_C000,
    
    /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Nonce) 
    /// 001:  
    P43_2006_C001,
    
    /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Parachain) 
    /// 002:  
    P43_2006_C002,
    
    /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Third-party custodian) 
    /// 003:  
    P43_2006_C003,
    
    /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Other) 
    /// 004:  
    P43_2006_C004,
    
    /// _2007_: Fair Value gains / (loses) holding Fungible tokens (smart contract) 
    P43_2007_C000,
    
    /// _2007_: Fair Value gains / (loses) holding Fungible tokens (Third-party custodian) 
    /// 001:  
    P43_2007_C001,
    
    /// _2008_: Fair Value gains / (loses) holding Non-Fungible tokens (smart contract) 
    P43_2008_C000,
    
    /// _2009_: Fair Value gains / (loses) holding Non-Fungible tokens (Third-party custodian) 
    /// 001:  
    P43_2009_C001,
    
    /// _2009_: Fair Value gains / (loses) holding Liquidity Pool Pair (smart contract) 
    P43_2009_C000,
    
    /// _2010_: Fair Value gains / (loses) holding Crypto Loan Collateral 
    P43_2010_C000,
    
    /// _2011_: Gains / (Losses) on sale / disposal of Cryptocurrency Assets 
    P43_2011_C000,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OperatingExpenses {
    /// _0001_: Cost of Goods Sold (COGS)  
    _0001_(_0001_),
    /// _0002_: Charges In Out 
    _0002_(_0002_),
    /// _0003_: Depreciation Depletion Amortization   
    _0003_(_0003_),
    /// _0004_: Field Trials   
    _0004_(_0004_),
    /// _0005_: Warranties 
    _0005_(_0005_),
    /// _0006_: Tax Corporation Tax 
    _0006_(_0006_),
    /// _0007_: Tax Fines & Penalties 
    _0007_(_0007_),
    /// _0008_: Claims   
    _0008_(_0008_),
    /// _0009_: Commissions   
    _0009_(_0009_),
    /// _0010_: Marketing Programs   
    _0010_(_0010_),
    /// _0011_: Consulting Fees 
    _0011_(_0011_),
    /// _0012_: Services   
    _0012_(_0012_),
    /// _0013_: Travel Expenses   
    _0013_(_0013_),
    /// _0014_: Hotels   
    _0014_(_0014_),
    /// _0015_: Meetings & Conferences   
    _0015_(_0015_),
    /// _0016_: Restaurant Meals   
    _0016_(_0016_),
    /// _0017_: Other Travel Expenses   
    _0017_(_0017_),
    /// Cost Pooling
    _0018_(_0018_),
    /// _0019_: Car Expenses   
    _0019_(_0019_),
    /// _0020_: Equipment 
    _0020_(_0020_),
    /// Plant Maintenance 
    _0021_(_0021_),
    /// _0022_: Phones Telecom   
    _0022_(_0022_),
    /// _0023_: Rental Leases   
    _0023_(_0023_),
    /// _0024_: Repair Maintenance   
    _0024_(_0024_),
    /// _0025_: Supplies   
    _0025_(_0025_),
    /// _0026_: Utilities 
    _0026_(_0026_),
    /// _0027_: Insurance 
    _0027_(_0027_),
    /// _0028_: Corporate Governance 
    _0028_(_0028_),
    /// _0029_: Legal Fees 
    _0029_(_0029_),
    /// _0030_: Administration Cost 
    _0030_(_0030_),
    /// _0031_: Bad Debts 
    _0031_(_0031_),
    /// _0032_: Miscellaneous Expenses   
    _0032_(_0032_),
    /// _0033_: Financial guarantee fees 
    _0033_(_0033_),
    /// _0034_: Royalty expenses 
    _0034_(_0034_),
    /// _0035_: Extraordinary expenses 
    _0035_(_0035_),
    /// _0036_: Impairment loss on financial assets 
    _0036_(_0036_),
    /// _0037_: Impairment loss on contract assets 
    _0037_(_0037_),
    /// _0038_: Provisions  
    _0038_(_0038_),
    /// Property, plant and equipment written off
    _0039_(_0039_) 
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum OtherOperatingExpenses {
    /// _1001_: Other Miscellaneous Charges 
    _1001_(_1001_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum PersonnelCosts {
    /// _2001_: Salaries   
    _2001_(_2001_),
    /// _2002_: Expat Expenses   
    _2002_(_2002_),
    /// Incentive Plan
    _2003_(_2003_),
    /// Incentive Plan Overhead
    _2004_(_2004_),
    /// _2005_: Employee Services   
    _2005_(_2005_),
    /// _2006_: Social Security   
    _2006_(_2006_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum FinanceCosts {
    /// _3001_: Interest expense 
    _3001_(_3001_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum ControlAccounts {
    /// Purchase Control.
    C60_0001_000D,
    
    /// Sales Control.
    C60_0002_000D,
    
    /// Tax Control.
    C60_0003_000D,
    
    /// Escrowed Funds Control.
    C60_0004_000D,
    
    /// Borrowings Control.
    C60_0005_000D,
    
    /// Defi Borrowings Control.
    C60_0006_000D,
    
    /// Liquidity Pool Control.
    C60_0007_000D,
}
/// Assets
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum A {
    /// Cuurent Assets.
    CurrentAssets(CurrentAssets),
    /// Current Assets Crypto.
    CurrentAssetsCrypto(CurrentAssetsCrypto),
    /// Fixed Assets.
    FixedAssets(FixedAssets),
    /// Intangible Assets.
    IntagibleAssets(IntagibleAssets),
    /// Non-current Assets.
    NonCurrentAssets(NonCurrentAssets),
}
/// Liabilities
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum L {
    /// Current Liabilities.
    CurrentLiabilities(CurrentLiabilities),
    /// Non-current Liabilities.
    NonCurrentLiabilities(NonCurrentLiabilities),
}
/// Equity
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum E {
    /// Shareholders' Equity.
    ShareholdersEquity(ShareholdersEquity),
    /// Other Equity.
    OtherEquity(OtherEquity),
    /// Capital Stock.
    CapitalStock(CapitalStock),
    /// Other Reserves
    OtherReserves(OtherReserves),
    /// Retained Earnings.
    RetainedEarnings(RetainedEarnings), 
}
/// Income 
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum I {
    /// Sales
    Sales(Sales),
    /// Other Income
    OtherIncome(OtherIncome),
    /// Other Operating Income
    OtherOperatingIncome(OtherOperatingIncome),
    // Other Comprehensive Income
    OtherComprehensiveIncome(OtherComprehensiveIncome),
}
/// Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum X {
    /// Operating Expenses.
    OperatingExpenses(OperatingExpenses),
    /// Other Operating Expenses.
    OtherOperatingExpenses(OtherOperatingExpenses),
    /// Personnel Costs.
    PersonnelCosts(PersonnelCosts),
    /// Finance Costs.
    FinanceCosts(FinanceCosts),
}
/// Profit and Loss
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum P {
    /// Income
    I(I),
    /// Expenses
    X(X),
}
/// Balance Sheet
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum B {
    /// Assets.
    A(A),
    /// Liabilities.
    L(L),
    /// Equity.
    E(E),
}
/// The top level Chart of Accounts.
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
pub enum Ledger {
    /// Balance Sheet.
    B(B),
    /// Profit and Loss.
    P(P),
    /// Control Accounts.
    C(ControlAccounts),
}

// The following is the actual structure of the hierarchy.

// pub struct Ledger {
//     pub enum B {
//         pub enum A {
//             pub enum CurrentAssets {
//                 /// _0001_: Bank Current Account 
//                 B10_0001_D000,
                
//                 /// _0002_: Bank Savings Account 
//                 B10_0002_D000,
                
//                 /// _0003_: Petty Cash 
//                 B10_0003_D000,
                
//                 /// _0004_: Internal Balance 
//                 B10_0004_D000,
                
//                 /// _0005_: Totem Escrow Deposit 
//                 B10_0005_D000,
                
//                 /// _0006_: Fixed deposits 
//                 B10_0006_D000,
                
//                 /// _0007_: Prepaid Expenses on account (Operating Expense) 
//                 B10_0007_D000,
                
//                 /// _0008_: Director's loan account (asset until repaid) 
//                 B10_0008_D000,
                
//                 /// _0009_: Accounts receivable (Sales Control Account or Trade Debtor's Account) 
//                 /// 000: Trade receivables - non-related parties 
//                 B10_0009_D000,
                
//                 /// _0009_: Accounts receivable (Sales Control Account or Trade Debtor's Account) 
//                 /// 001: Trade receivables - related parties 
//                 B10_0009_D001,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 000: Loan to non-related parties - Current Assets 
//                 B10_0010_D000,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 001: Loan to related parties - Current Assets 
//                 B10_0010_D001,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 002: Other receivables - Current Assets 
//                 B10_0010_D002,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 003: Finance lease receivables - Current Assets 
//                 B10_0010_D003,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 004: Staff loans - Current Assets 
//                 B10_0010_D004,
                
//                 /// _0010_: Accrued Revenues / Receivables 
//                 /// 005: Government grant receivables 
//                 B10_0010_D005,
                
//                 /// _0011_: Impairment loss on financial assets 
//                 /// 000: Allowance for doubtful debts  
//                 B10_0011_C000,
                
//                 /// _0011_: Impairment loss on financial assets 
//                 /// 001: Other  
//                 B10_0011_C001,
                
//                 /// _0012_: Impairment loss on contract assets  
//                 B10_0012_C000,
                
//                 /// _0013_: Inventory 
//                 /// 000: Raw materials 
//                 B10_0013_D000,
                
//                 /// _0013_: Inventory 
//                 /// 001: Work in progress 
//                 B10_0013_D001,
                
//                 /// _0013_: Inventory 
//                 /// 002: Finished goods 
//                 B10_0013_D002,
                
//                 /// _0014_: Advances to suppliers (Trade Supplies) 
//                 B10_0014_D000,
                
//                 /// _0015_: Contract assets 
//                 B10_0015_D000,
                
//                 /// _0016_: Financial assets, at fair value through profit or loss (FVTPL) 
//                 /// 000: Equity securities - Fair value through P&L 
//                 B10_0016_D000,
                
//                 /// _0017_: Assets of disposal group classified as held-for-sale 
//                 B10_0017_D000,
                
//                 /// _0018_: Other investments at amortised cost 
//                 /// 000: Loan notes - floating or fixed 
//                 B10_0018_D000,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 000: Interest rate swaps - Current Assets 
//                 B10_0019_D000,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 001: Currency forwards - Current Assets 
//                 B10_0019_D001,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 002: Commodity forwards - Current Assets 
//                 B10_0019_D002,
//             }
//             pub enum CurrentAssetsCrypto {
//                     /// _4001_: Cryptocurrency 
//                     /// 000: UTXO 
//                     B11_4001_D000,
                    
//                     /// _4001_: Cryptocurrency 
//                     /// 001: Nonce 
//                     B11_4001_D001,
                    
//                     /// _4001_: Cryptocurrency 
//                     /// 002: Parachain 
//                     B11_4001_D002,
                    
//                     /// _4001_: Cryptocurrency 
//                     /// 003: Third-party custodian 
//                     B11_4001_D003,
                    
//                     /// _4001_: Cryptocurrency 
//                     /// 004: Other 
//                     B11_4001_D004,
                    
//                     /// _4002_: Fungible tokens 
//                     /// 000: Smart contract 
//                     B11_4002_D000,
                    
//                     /// _4002_: Fungible tokens 
//                     /// 001: Third-party custodian 
//                     B11_4002_D001,
                    
//                     /// _4003_: Non-Fungible tokens 
//                     /// 000: Smart contract 
//                     B11_4003_D000,
                    
//                     /// _4003_: Non-Fungible tokens 
//                     /// 001: Third-party custodian 
//                     B11_4003_D001,
                    
//                     /// _4004_: Liquidity Pool Pair 
//                     /// 000: Smart Contract 
//                     B11_4004_D000,
                    
//                     /// _4005_: Crypto Loan Collateral 
//                     B11_4005_D000,
                    
//                     /// _4006_: Impairment - Cryptocurrency 
//                     /// 000: UTXO 
//                     B11_4006_C000,
                    
//                     /// _4006_: Impairment - Cryptocurrency 
//                     /// 001: Nonce 
//                     B11_4006_C001,
                    
//                     /// _4006_: Impairment - Cryptocurrency 
//                     /// 002: Parachain 
//                     B11_4006_C002,
                    
//                     /// _4006_: Impairment - Cryptocurrency 
//                     /// 003: Third-party custodian 
//                     B11_4006_C003,
                    
//                     /// _4006_: Impairment - Cryptocurrency 
//                     /// 004: Other 
//                     B11_4006_C004,
                    
//                     /// _4007_: Impairment - Fungible tokens 
//                     /// 000: Smart contract 
//                     B11_4007_C000,
                    
//                     /// _4007_: Impairment - Fungible tokens 
//                     /// 001: Third-party custodian 
//                     B11_4007_C001,
                    
//                     /// _4008_: Impairment - Non-Fungible tokens 
//                     /// 000: Smart contract 
//                     B11_4008_C000,
                    
//                     /// _4008_: Impairment - Non-Fungible tokens 
//                     /// 001: Third-party custodian 
//                     B11_4008_C001,
                    
//                     /// _4009_: Impairment - Liquidity Pool Pair 
//                     /// 000: Smart Contract 
//                     B11_4009_C000,
                    
//                     /// _4010_: Impairment - Crypto Loan Collateral 
//                     B11_4010_C000,
//             }
//             pub enum FixedAssets {
//                 /// _1001_: Property, plant and equipment 
//                 /// 000: Land 
//                 B12_1001_D000,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 001: Buildings 
//                 B12_1001_D001,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 002: Furniture fixtures and fittings 
//                 B12_1001_D002,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 003: Plant and equipment 
//                 B12_1001_D003,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 004: Motor vehicles 
//                 B12_1001_D004,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 005: Supplies 
//                 B12_1001_D005,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 006: Computer and IT equipment 
//                 B12_1001_D006,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 007: Right-of-use assets 
//                 B12_1001_D007,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 008: Assets under conenumion 
//                 B12_1001_D008,
                
//                 /// _1001_: Property, plant and equipment 
//                 /// 009: Leasehold improvements 
//                 B12_1001_D009,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 000: Land 
//                 B12_1002_C000,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 001: Buildings 
//                 B12_1002_C001,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 002: Furniture fixtures and fittings 
//                 B12_1002_C002,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 003: Plant and equipment 
//                 B12_1002_C003,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 004: Motor vehicles 
//                 B12_1002_C004,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 005: Supplies 
//                 B12_1002_C005,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 006: Computer and IT equipment 
//                 B12_1002_C006,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 007: Right-of-use assets 
//                 B12_1002_C007,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 008: Assets under conenumion 
//                 B12_1002_C008,
                
//                 /// _1002_: Property, plant and equipment - Accumulated Depreciation 
//                 /// 009: Leasehold improvements 
//                 B12_1002_C009,
//             }
//             pub enum IntagibleAssets {
//                 /// _2001_: Other Accumulated Depreciation 
//                 /// 000: Other Films, copyrights or other intellectual property rights - Accumulated depreciation 
//                 B13_2001_D000,
                
//                 /// _2002_: Trademark and patents 
//                 /// 000: Trademark and patents (BS) 
//                 B13_2002_D000,
                
//                 /// _2003_: Computer software licences 
//                 B13_2003_D000,
                
//                 /// _2004_: External Network Cryptocurrency 
//                 B13_2004_D000,
                
//                 /// _2005_: Goodwill 
//                 B13_2005_D000,
                
//                 /// _2006_: Trademark and patents 
//                 /// 000: Trademark and patents - Accumulated amortisation 
//                 B13_2006_C000,
                
//                 /// _2006_: Accumulated amortisation 
//                 /// 001: Computer software licences - Accumulated amortisation 
//                 B13_2006_C001,
                
//                 /// _2006_: Accumulated amortisation 
//                 /// 002: Others 
//                 B13_2006_C002,
                
//                 /// _2007_: Impairment loss on intangible assets 
//                 B13_2007_C000,
//             }
//             pub enum NonCurrentAssets {
//                 /// _3001_: Investment in subsidiaries 
//                 B14_3001_D000,
                
//                 /// _3002_: Investment in joint venture 
//                 B14_3002_D000,
                
//                 /// _3003_: Investment in associates 
//                 B14_3003_D000,
                
//                 /// _3004_: Investment properties 
//                 B14_3004_D000,
                
//                 /// _3005_: Financial assets, at fair value through other comprehensive income (FVOCI) 
//                 /// 000: Equity securities  - Fair value through other comprehensive income 
//                 B14_3005_D000,
                
//                 /// _3005_: Financial assets, at fair value through other comprehensive income (FVOCI) 
//                 /// 001: Debt securities 
//                 B14_3005_D001,
                
//                 /// _3006_: Deferred income tax - Non-current Assets 
//                 B14_3006_D000,
                
//                 /// _3007_: Impairment loss on non-financial assets 
//                 /// 000: Impairment of fixed assets 
//                 B14_3007_C000,
                
//                 /// _3007_: Impairment loss on non-financial assets 
//                 /// 001: Impairment investment in subsidiaries 
//                 B14_3007_C001,
                
//                 /// _3007_: Impairment loss on non-financial assets 
//                 /// 002: Impairment investment in joint venture 
//                 B14_3007_C002,
                
//                 /// _3007_: Impairment loss on non-financial assets 
//                 /// 003: Impairment investment in associates 
//                 B14_3007_C003,
                
//                 /// _3008_: Financial assets, at fair value through profit or loss 
//                 /// 000: Non-listed debt instruments 
//                 B14_3008_D000,
                
//                 /// _3008_: Financial assets, at fair value through profit or loss 
//                 /// 001: Mandatorily measured at FVTPL (e.g. redeemable preference shares) 
//                 B14_3008_D001,
                
//                 /// _3008_: Financial assets, at fair value through profit or loss 
//                 /// 002: Convertible bonds - Non-current Assets (BS) 
//                 B14_3008_D002,
                
//                 /// _3009_: Other receivables 
//                 /// 000: Loan to non-related parties - Non-current Assets 
//                 B14_3009_D000,
                
//                 /// _3009_: Other receivables 
//                 /// 001: Loan to related parties - Non-current Assets 
//                 B14_3009_D001,
                
//                 /// _3009_: Other receivables 
//                 /// 002: Other receivables - Non-current Assets 
//                 B14_3009_D002,
                
//                 /// _3009_: Other receivables 
//                 /// 003: Finance lease receivables - Non-current Assets 
//                 B14_3009_D003,
                
//                 /// _3009_: Other receivables 
//                 /// 004: Staff loans - Non-current Assets 
//                 B14_3009_D004,
                
//                 /// _3009_: Other receivables 
//                 /// 005: Indemnification assets 
//                 B14_3009_D005,
                
//                 /// _3010_: Derivative financial instruments 
//                 /// 000: Interest rate swaps - Non-current Assets 
//                 B14_3010_D000,
                
//                 /// _3010_: Derivative financial instruments 
//                 /// 001: Currency forwards - Non-current Assets 
//                 B14_3010_D001,
                
//                 /// _3010_: Derivative financial instruments 
//                 /// 002: Commodity forwards - Non-current Assets 
//                 B14_3010_D002,
//             }
//         }	
//         pub enum L {
//             pub enum CurrentLiabilities {
//                 /// _0001_: Sales Tax by Jurisdiction  (Can be a negative balance for reclaims [technically an asset]) 
//                 B20_0001_C000,
                
//                 /// _0002_: Federal or State Tax By Jurisdiction on Sales 
//                 B20_0002_C000,
                
//                 /// _0003_: Related parties 
//                 /// 000: Accounts payable (Trade creditors) 
//                 B20_0003_C000,
                
//                 /// _0003_: Related parties 
//                 /// 001: Current portion of long-term Debt (non-trade) 
//                 B20_0003_C001,
                
//                 /// _0003_: Related parties 
//                 /// 002: Short-term Loans (Payable) 
//                 B20_0003_C002,
                
//                 /// _0004_: Salaries Payable 
//                 B20_0004_C000,
                
//                 /// _0005_: Wages Payable 
//                 B20_0005_C000,
                
//                 /// _0006_: Commission Payable 
//                 B20_0006_C000,
                
//                 /// _0007_: Freight Payable 
//                 B20_0007_C000,
                
//                 /// _0008_: Other Accrued Expenses Payable 
//                 B20_0008_C000,
                
//                 /// _0009_: Payroll Tax By Jurisdiction Liabilities 
//                 B20_0009_C000,
                
//                 /// _0010_: Interest Payable 
//                 B20_0010_C000,
                
//                 /// _0011_: Advances from customers 
//                 B20_0011_C000,
                
//                 /// _0012_: Lawsuits and Legal Costs Payable 
//                 B20_0012_C000,
                
//                 /// _0013_: Non-related parties 
//                 /// 000: Accounts payable (Trade creditors) 
//                 B20_0013_C000,
                
//                 /// _0013_: Non-related parties 
//                 /// 001: Short-term Loans (Payable) 
//                 B20_0013_C001,
                
//                 /// _0013_: Non-related parties 
//                 /// 002: Current portion of long-term Debt (non-trade) 
//                 B20_0013_C002,
                
//                 /// _0014_: Financial guarantees 
//                 B20_0014_C000,
                
//                 /// _0015_: Contract liabilities 
//                 B20_0015_C000,
                
//                 /// _0016_: Bank overdrafts 
//                 B20_0016_C000,
                
//                 /// _0017_: Current portion of lease liabilities 
//                 B20_0017_C000,
                
//                 /// _0018_: Provisions  
//                 /// 000: Warranty / Legal claims / Others - Current Liabilities (BS) 
//                 B20_0018_C000,
                
//                 /// _0018_: Provisions  
//                 /// 001: Unused leave - Current Liabilities (BS) 
//                 B20_0018_C001,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 000: Interest rate swaps - Current Liabilities 
//                 B20_0019_C000,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 001: Currency forwards - Current Liabilities 
//                 B20_0019_C001,
                
//                 /// _0019_: Derivative financial instruments 
//                 /// 002: Commodity forwards - Current Liabilities 
//                 B20_0019_C002,
                
//                 /// _0020_: Dividend payable 
//                 B20_0020_C000,
                
//                 /// _0021_: Current portion of bank borrowings (Payable) 
//                 B20_0021_C000,
//             }
//             pub enum NonCurrentLiabilities {
//                 /// _1001_: Income Tax By Jurisdiction Payable (Witholding Tax) 
//                 B21_1001_C000,
                
//                 /// _1002_: Deferred Revenues (Unearned Revenues) 
//                 B21_1002_C000,
                
//                 /// _1003_: Bonds Payable 
//                 B21_1003_C000,
                
//                 /// _1004_: Notes (Loans) Payable 
//                 B21_1004_C000,
                
//                 /// _1005_: Waranty Liabilities 
//                 B21_1005_C000,
                
//                 /// _1006_: Corporation tax by jurisdiction payable (posted at end of period or end of year) 
//                 B21_1006_C000,
                
//                 /// _1007_: Installment Loans (Payable) 
//                 B21_1007_C000,
                
//                 /// _1008_: Portion of bank borrowings (Payable) - Non-current Liabilities 
//                 B21_1008_C000,
                
//                 /// _1009_: Mortgage or Property Loans (Payable) 
//                 B21_1009_C000,
                
//                 /// _1010_: Redeemable preference shares 
//                 B21_1010_C000,
                
//                 /// _1011_: Portion of lease liabilities - Non-current Liabilities 
//                 B21_1011_C000,
                
//                 /// _1012_: Contingent consideration payable 
//                 B21_1012_C000,
                
//                 /// _1013_: Deferred income tax - Non-current Liabilities 
//                 B21_1013_C000,
                
//                 /// _1014_: Related parties 
//                 /// 000: Non-current portion of long-term Debt (non-trade) 
//                 B21_1014_C000,
                
//                 /// _1014_: Non-related parties 
//                 /// 001: Non-current portion of long-term Debt (non-trade) 
//                 B21_1014_C001,
                
//                 /// _1015_: Provisions  
//                 /// 000: Warranty / Legal claims / Others - Non-current Liabilities (BS) 
//                 B21_1015_C000,
                
//                 /// _1016_: Derivative financial instruments 
//                 /// 000: Interest rate swaps - Non-current Liabilities 
//                 B21_1016_C000,
                
//                 /// _1016_: Derivative financial instruments 
//                 /// 001: Currency forwards - Non-current Liabilities 
//                 B21_1016_C001,
                
//                 /// _1016_: Derivative financial instruments 
//                 /// 002: Commodity forwards - Non-current Liabilities 
//                 B21_1016_C002,
//             }
//         }
//         pub enum E {
//             pub enum ShareholdersEquity {
//                 /// _0001_: Personal Net Worth 
//                 B30_0001_C000,
                
//                 /// _0002_: Owner's Equity 
//                 B30_0002_C000,
//             }
//             pub enum OtherEquity {
//                 /// _1001_: Corporation tax by jurisdiction (calculated after P&L) 
//                 B31_1001_C000,
//             }
//             pub enum CapitalStock {
//                 /// _2001_: Ordinary shares 
//                 B32_2001_C000,
                
//                 /// _2002_: Preference shares 
//                 B32_2002_C000,
                
//                 /// _2003_: Treasury shares 
//                 B32_2003_D000,
//             }
//             pub enum OtherReserves {
//                 /// _3001_: Share application monies 
//                 B33_3001_C000,
                
//                 /// _3002_: Share option reserve 
//                 B33_3002_C000,
                
//                 /// _3003_: Fair value reserve 
//                 /// 000: Fair value gains / (losses) on financial assets at FVOCI (debt instruments) 
//                 B33_3003_C000,
                
//                 /// _3003_: Fair value reserve 
//                 /// 001: Fair value gains / (losses) on financial assets at FVOCI (equity instruments) 
//                 B33_3003_C001,
                
//                 /// _3004_: Hedging reserve 
//                 /// 000: Cash flow hedges 
//                 B33_3004_C000,
                
//                 /// _3005_: Equity component of convertible bonds 
//                 B33_3005_C000,
                
//                 /// _3006_: Asset revaluation reserve 
//                 /// 000: Revaluation of property, plant and equipment 
//                 B33_3006_C000,
//             }
//             pub enum RetainedEarnings {
//                 /// _4001_: Dividend paid 
//                 B34_4001_D000,
                
//                 /// _4002_: Retained Earnings 
//                 B34_4002_C000,              
//             }
//         }
//     }
//     pub enum P {
//         pub enum I {
//             pub enum Sales {
//                 /// _0001_: Sales of services 
//                 P40_0001_C000,
                
//                 /// _0002_: Sales of goods 
//                 P40_0002_C000,
                
//                 /// _0003_: Sales returns and allowances 
//                 P40_0003_D000,
                
//                 /// _0004_: Sales discounts 
//                 P40_0004_C000,
                
//                 /// _0005_: Freight Billable 
//                 P40_0005_C000,
                
//                 /// _0006_: Commission Billable 
//                 P40_0006_C000,
                
//                 /// _0007_: Miscellaneous Income 
//                 P40_0007_C000,
//             }
//             pub enum OtherIncome {
//                 /// _1001_: Royalty income 
//                 P41_1001_C000,
                
//                 /// _1002_: Interest income 
//                 /// 000: FInancial assets measured at amortised cost 
//                 P41_1002_C000,
                
//                 /// _1002_: Interest income 
//                 /// 001: Investments 
//                 P41_1002_C001,
                
//                 /// _1002_: Interest income 
//                 /// 002: Trade receivables 
//                 P41_1002_C002,
                
//                 /// _1002_: Interest income 
//                 /// 003: Bank deposits 
//                 P41_1002_C003,
                
//                 /// _1002_: Interest income 
//                 /// 004: Loans to an associate / subsidiary 
//                 P41_1002_C004,
                
//                 /// _1002_: Interest income 
//                 /// 005: Debt investments measured at FVOCI 
//                 P41_1002_C005,
                
//                 /// _1003_: Dividend income 
//                 P41_1003_C000,
                
//                 /// _1004_: Rental income 
//                 P41_1004_C000,
                
//                 /// _1005_: Grant income 
//                 P41_1005_C000,
                
//                 /// _1006_: Fair value gains / (losses) on financial assets and liabilities at FVTPL 
//                 P41_1006_C000,
                
//                 /// _1007_: Fair value gains / (losses) on derivative financial instruments 
//                 P41_1007_C000,
                
//                 /// _1008_: Ineffectiveness on fair value / cash flow hedges 
//                 P41_1008_C000,
                
//                 /// _1009_: Financial assets at FVOCI, reclassified from OCI on disposal 
//                 P41_1009_C000,
                
//                 /// _1010_: Fair value gains / (losses) on investment properties 
//                 P41_1010_C000,
                
//                 /// _1011_: Fair value gains / (loss) on contingent consideration  
//                 P41_1011_C000,
//             }
//             pub enum OtherOperatingIncome {
//                 /// _1001_: Share of profit / (loss) of associates and joint ventures 
//                 P42_1001_C000,
                
//                 /// _1002_: Gains / (Losses) on sale / disposal of property, plant and equipment 
//                 P42_1002_C000,
                
//                 /// _1003_: Profit / (loss) from discontinued operations 
//                 P42_1003_C000,
//             }
//             pub enum OtherComprehensiveIncome {
//                 /// _2001_: Fair value gains / (losses) on financial assets at FVOCI 
//                 /// 000: Debt instruments  
//                 P43_2001_C000,
                
//                 /// _2001_: Fair value gains / (losses) on financial assets at FVOCI 
//                 /// 001: Equity instruments 
//                 P43_2001_C001,
                
//                 /// _2002_: Fair value gains / (losses) on cash flow hedges 
//                 P43_2002_C000,
                
//                 /// _2003_: Share of other comprehensive income of associates 
//                 P43_2003_C000,
                
//                 /// _2004_: Reclassification adjustments (for items that may be reclassified subsequently to profit or loss) 
//                 P43_2004_C000,
                
//                 /// _2005_: Revaluation gains on property, plant and equipment 
//                 P43_2005_C000,
                
//                 /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (UTXO) 
//                 P43_2006_C000,
                
//                 /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Nonce) 
//                 /// 001:  
//                 P43_2006_C001,
                
//                 /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Parachain) 
//                 /// 002:  
//                 P43_2006_C002,
                
//                 /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Third-party custodian) 
//                 /// 003:  
//                 P43_2006_C003,
                
//                 /// _2006_: Fair Value gains / (loses) holding Cryptocurrency (Other) 
//                 /// 004:  
//                 P43_2006_C004,
                
//                 /// _2007_: Fair Value gains / (loses) holding Fungible tokens (smart contract) 
//                 P43_2007_C000,
                
//                 /// _2007_: Fair Value gains / (loses) holding Fungible tokens (Third-party custodian) 
//                 /// 001:  
//                 P43_2007_C001,
                
//                 /// _2008_: Fair Value gains / (loses) holding Non-Fungible tokens (smart contract) 
//                 P43_2008_C000,
                
//                 /// _2009_: Fair Value gains / (loses) holding Non-Fungible tokens (Third-party custodian) 
//                 /// 001:  
//                 P43_2009_C001,
                
//                 /// _2009_: Fair Value gains / (loses) holding Liquidity Pool Pair (smart contract) 
//                 P43_2009_C000,
                
//                 /// _2010_: Fair Value gains / (loses) holding Crypto Loan Collateral 
//                 P43_2010_C000,
                
//                 /// _2011_: Gains / (Losses) on sale / disposal of Cryptocurrency Assets 
//                 P43_2011_C000,
//             }
//         }
//         pub enum X {
//             pub enum OperatingExpenses {
//                 /// Cost of Goods Sold (COGS)
//                 pub enum _0001_ {
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS)  
//                     /// 000: Raw materials - changes in inventories 
//                     P50_0001_D000,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS)  
//                     /// 001: Work in progress - changes in inventories 
//                     P50_0001_D001,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS)  
//                     /// 002: Finished goods - changes in inventories 
//                     P50_0001_D002,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS)  
//                     /// 003: Purchases - direct material costs 
//                     P50_0001_D003,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS) 
//                     /// 004: Direct labour costs 
//                     P50_0001_D004,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS) 
//                     /// 005: Manufacturing / Factory overhead 
//                     P50_0001_D005,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS)  
//                     /// 006: Purchases returns and allowances 
//                     P50_0001_D006,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS) 
//                     /// 007: Purchase discount 
//                     P50_0001_D007,
                    
//                     /// Operating  Expenses
//                     /// _0001_: Cost of Goods Sold (COGS) 
//                     /// 008: Inventory write-down / Reversal of inventory write-down 
//                     P50_0001_D008,
//                 }	
//                 /// Charges In Out
//                 pub enum _0002_ {
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 000: Inter-Entity Charge In 
//                     P50_0002_D000,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 001: Inter-Entity Charge Out 
//                     P50_0002_D001,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 002: IT Distribution 
//                     P50_0002_D002,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 003: Distribution To Capital 
//                     P50_0002_D003,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 004: Allocations 
//                     P50_0002_D004,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 005: Allocation By Country 
//                     P50_0002_D005,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 006: Facilities Allocation 
//                     P50_0002_D006,
                    
//                     /// Operating  Expenses
//                     /// _0002_: Charges In Out 
//                     /// 007: Assess Research And Development Recharges 
//                     P50_0002_D007,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 000: General Project Costs 
//                     P50_0003_D000,
//                 }	
//                 /// Depreciation Depletion Amortization  
//                 pub enum _0003_ {
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 001: Depreciation Land 
//                     P50_0003_D001,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 002: Depreciation Buildings 
//                     P50_0003_D002,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 003: Depreciation Furniture fixtures and fittings 
//                     P50_0003_D003,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 004: Depreciation Plant and equipment 
//                     P50_0003_D004,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 005: Depreciation Motor vehicles 
//                     P50_0003_D005,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 006: Depreciation Supplies 
//                     P50_0003_D006,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 007: Depreciation Computer and IT equipment 
//                     P50_0003_D007,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 008: Depreciation Right-of-use assets 
//                     P50_0003_D008,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 009: Depreciation Leasehold improvements 
//                     P50_0003_D009,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 010: Amortisation of Others intangible assets 
//                     P50_0003_D010,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 011: Amortisation of Trademark and patents 
//                     P50_0003_D011,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 012: Amortisation of Computer software licences 
//                     P50_0003_D012,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 013: Amortization Government Grants 
//                     P50_0003_D013,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 014: Gain Loss On Miscellaneous Sales 
//                     P50_0003_D014,
                    
//                     /// Operating  Expenses
//                     /// _0003_: Depreciation Depletion Amortization   
//                     /// 015: Non Capital Project Expense 
//                     P50_0003_D015,
//                 }	
//                 /// Field Trials  
//                 pub enum _0004_ {
//                     /// Operating  Expenses
//                     /// _0004_: Field Trials   
//                     /// 000: Consumption Other 
//                     P50_0004_D000,
                    
//                     /// Operating  Expenses
//                     /// _0004_: Field Trials   
//                     /// 001: Trials 
//                     P50_0004_D001,
                    
//                     /// Operating  Expenses
//                     /// _0004_: Field Trials   
//                     /// 002: Samples 
//                     P50_0004_D002,
//                 }	
//                 /// Warranties
//                 pub enum _0005_ {
//                     /// _0005_: Warranties 
//                     P50_0005_D000,
//                 }	
//                 /// Tax (Corporation Tax)
//                 pub enum _0006_ {
//                     /// _0006_: Tax (Corporation Tax) 
//                     P50_0006_D000,
                    
//                 }	
//                 /// Tax Fines & Penalties
//                 pub enum _0007_ {
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 000: Employee Tax 
//                     P50_0007_D000,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 001: Tax Property 
//                     P50_0007_D001,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 002: Tax Trade Licences 
//                     P50_0007_D002,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 003: Tax Enviromental 
//                     P50_0007_D003,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 004: Road Tax 
//                     P50_0007_D004,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 005: Taxes Other 
//                     P50_0007_D005,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 006: Taxes Other Alternate Acct 
//                     P50_0007_D006,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 007: Non-Deductible Fines Penalties 
//                     P50_0007_D007,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 008: Penalties 
//                     P50_0007_D008,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 009: Licenses Fees Certificates Other Taxes 
//                     P50_0007_D009,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 010: Car Tax 
//                     P50_0007_D010,
                    
//                     /// Operating  Expenses
//                     /// _0007_: Tax Fines & Penalties 
//                     /// 011: Capital Gains Tax on Crypto Assets 
//                     P50_0007_D011,
//                 }	
//                 /// Claims  
//                 pub enum _0008_ {
//                     /// _0008_: Claims   
//                     P50_0008_D000,
                    
//                 }	
//                 /// Commissions  
//                 pub enum _0009_ {
//                     /// Operating  Expenses
//                     /// _0009_: Commissions   
//                     /// 000: Commissions Manual 
//                     P50_0009_D000,
                    
//                     /// Operating  Expenses
//                     /// _0009_: Commissions   
//                     /// 001: Commissions Intercompany Expense 
//                     P50_0009_D001,
                    
//                     /// Operating  Expenses
//                     /// _0009_: Commissions   
//                     /// 002: Commissions Intercompany Income 
//                     P50_0009_D002,
                    
//                 }	
//                 /// Marketing Programs  
//                 pub enum _0010_ {
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 000: Samples Consumption Marketing 
//                     P50_0010_D000,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 001: Promotions 
//                     P50_0010_D001,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 002: Advertising Expenses 
//                     P50_0010_D002,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 003: Advertising Promotions Plant Visit 
//                     P50_0010_D003,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 004: Design And Artwork 
//                     P50_0010_D004,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 005: Advertising Promotions Gifts 
//                     P50_0010_D005,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 006: Advertising Promotions Meetings Seminars 
//                     P50_0010_D006,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 007: Advertising Promotions Printed Material 
//                     P50_0010_D007,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 008: Advertising Promotions Outdoor Advertising 
//                     P50_0010_D008,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 009: Advertising Promotions Presentation Material 
//                     P50_0010_D009,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 010: Distributor Events 
//                     P50_0010_D010,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 011: Market Research 
//                     P50_0010_D011,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 012: Fairs And Exhibitions 
//                     P50_0010_D012,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 013: Media Print 
//                     P50_0010_D013,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 014: Media Radio 
//                     P50_0010_D014,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 015: Media Television 
//                     P50_0010_D015,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 016: Media Other 
//                     P50_0010_D016,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 017: Samples 
//                     P50_0010_D017,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 018: Custom Gifts Taxable 
//                     P50_0010_D018,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 019: Custom Gifts Non Taxable 
//                     P50_0010_D019,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 020: Technical Service Fees 
//                     P50_0010_D020,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 021: Literature 
//                     P50_0010_D021,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 022: Brand Merchandise 
//                     P50_0010_D022,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 023: Trade Shows Exhibit Fees 
//                     P50_0010_D023,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 024: Agency Fees 
//                     P50_0010_D024,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 025: Audio Visuals 
//                     P50_0010_D025,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 026: Marketing Events Days In The Field 
//                     P50_0010_D026,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 027: Marketing Miscellaneous Expense 
//                     P50_0010_D027,
                    
//                     /// Operating  Expenses
//                     /// _0010_: Marketing Programs   
//                     /// 028: Marketing Cost Sharing 
//                     P50_0010_D028,
//                 }	
//                 /// Consulting Fees  
//                 pub enum _0011_ {
//                     /// _0011_: Consulting Fees   
//                     P50_0011_D000,
                    
//                 }	
//                 /// Services  
//                 pub enum _0012_ {
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 000: Miscellaneous Service 
//                     P50_0012_D000,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 001: Analytical Testing 
//                     P50_0012_D001,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 002: Communications Tarriffs 
//                     P50_0012_D002,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 003: Experimental Testing 
//                     P50_0012_D003,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 004: Financial Services 
//                     P50_0012_D004,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 005: Freight Expenses 
//                     P50_0012_D005,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 006: Housekeeping & Laundry 
//                     P50_0012_D006,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 007: Language Translation Services 
//                     P50_0012_D007,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 008: Mail Delivery Services 
//                     P50_0012_D008,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 009: Miscellaneous Fees 
//                     P50_0012_D009,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 010: Contractors 
//                     P50_0012_D010,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 011: Technical Assistance 
//                     P50_0012_D011,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 012: Laboratory Expense 
//                     P50_0012_D012,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 013: Labour 
//                     P50_0012_D013,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 014: Printing 
//                     P50_0012_D014,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 015: Warehouse 
//                     P50_0012_D015,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 016: Photographs Graphics 
//                     P50_0012_D016,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 017: Project Engineering 
//                     P50_0012_D017,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 018: Research Assistance 
//                     P50_0012_D018,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 019: Research Shopping Expense 
//                     P50_0012_D019,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 020: Other service for Production 
//                     P50_0012_D020,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 021: Software Maintenance Fees 
//                     P50_0012_D021,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 022: Tax Preparation Fees 
//                     P50_0012_D022,
                    
//                     /// Operating  Expenses
//                     /// _0012_: Services   
//                     /// 023: Testing Services 
//                     P50_0012_D023,
                    
//                 }	
//                 /// Travel Expenses  
//                 pub enum _0013_ {
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 000: Travel Expenses Per Diem 
//                     P50_0013_D000,
                    
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 001: Travel General 
//                     P50_0013_D001,
                    
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 002: Travel Expenses Air Tickets 
//                     P50_0013_D002,
                    
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 003: Travel Expenses Air Tickets Outside Country Of Residence 
//                     P50_0013_D003,
                    
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 004: Travel Expenses Miles 
//                     P50_0013_D004,
                    
//                     /// Operating  Expenses
//                     /// _0013_: Travel Expenses   
//                     /// 005: Taxi & Other Personal Transport 
//                     P50_0013_D005,
//                 }	
//                 /// Hotels  
//                 pub enum _0014_ {
//                     /// Operating  Expenses
//                     /// _0014_: Hotels   
//                     /// 000: Lodging Hotel Costs 
//                     P50_0014_D000,
//                 }	
//                 /// Meetings & Conferences  
//                 pub enum _0015_ {
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 000: Meetings Conferences 
//                     P50_0015_D000,
                    
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 001: Other Grower Meetings 
//                     P50_0015_D001,
                    
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 002: Key Distributor Meetings 
//                     P50_0015_D002,
                    
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 003: Travel Foreign 
//                     P50_0015_D003,
                    
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 004: Travel Non Employee 
//                     P50_0015_D004,
                    
//                     /// Operating  Expenses
//                     /// _0015_: Meetings & Conferences   
//                     /// 005: Travel Tech Meetings Conferences 
//                     P50_0015_D005,
//                 }	
//                 /// Restaurant Meals  
//                 pub enum _0016_ {
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 000: Business Meals Entertainment 
//                     P50_0016_D000,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 001: Business Meals Within Country Of Residence 
//                     P50_0016_D001,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 002: Business Meals Outside Country Of Residence 
//                     P50_0016_D002,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 003: Entertainment Non-Deductable 
//                     P50_0016_D003,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 004: Meals Entertainment Field Sales 
//                     P50_0016_D004,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 005: Meals Entertainment Training Education 
//                     P50_0016_D005,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 006: Meals Entertainment Business Development (No Meeting) 
//                     P50_0016_D006,
                    
//                     /// Operating  Expenses
//                     /// _0016_: Restaurant Meals   
//                     /// 007: Meals Entertainment Business Development 
//                     P50_0016_D007,
//                 }	
//                 /// Other Travel Expenses  
//                 pub enum _0017_ {
//                     /// _0017_: Other Travel Expenses   
//                     P50_0017_D000,
//                 }	
//                 /// Cost Pooling
//                 pub enum _0018_ {
//                     /// Operating  Expenses
//                     /// _0018_: Cost Pooling 
//                     /// 000: Public Relations 
//                     P50_0018_D000,
                    
//                 }	
//                 /// Car Expenses  
//                 pub enum _0019_ {
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 000: Automobile Expense 
//                     P50_0019_D000,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 001: Automobile Expense Parking 
//                     P50_0019_D001,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 002: Automobile Expense Company Car 
//                     P50_0019_D002,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 003: Automobile Expense Telephone 
//                     P50_0019_D003,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 004: Automobile Expense Rental 
//                     P50_0019_D004,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 005: Automobile Expense Repairs 
//                     P50_0019_D005,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 006: Automobile Expense Car Financing 
//                     P50_0019_D006,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 007: Automobile Expense Fuel 
//                     P50_0019_D007,
                    
//                     /// Operating  Expenses
//                     /// _0019_: Car Expenses   
//                     /// 008: Leased Autos 
//                     P50_0019_D008,
//                 }	
//                 /// Equipment
//                 pub enum _0020_ {
//                     /// Operating  Expenses
//                     /// _0020_: Equipment 
//                     P50_0020_D000,
                    
//                 }	
//                 /// Plant Maintenance 
//                 pub enum _0021_ {
//                     /// Operating  Expenses
//                     /// _0021_: Plant Maintenance  
//                     P50_0021_D000,
//                 }	
//                 /// Phones Telecom  
//                 pub enum _0022_ {
//                     /// Operating  Expenses
//                     /// _0022_: Phones Telecom   
//                     /// 000: Cellular Usage 
//                     P50_0022_D000,
                    
//                     /// Operating  Expenses
//                     /// _0022_: Phones Telecom   
//                     /// 001: Communication Remote Access 
//                     P50_0022_D001,
                    
//                     /// Operating  Expenses
//                     /// _0022_: Phones Telecom   
//                     /// 002: Miscellaneous Data Communications 
//                     P50_0022_D002,
                    
//                     /// Operating  Expenses
//                     /// _0022_: Phones Telecom   
//                     /// 003: Telephone Facsimiles 
//                     P50_0022_D003,
//                 }	
//                 /// Rental Leases  
//                 pub enum _0023_ {
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 000: Rent Warehouse 
//                     P50_0023_D000,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 001: Non Lease Rental 
//                     P50_0023_D001,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 002: Rent Building Office 
//                     P50_0023_D002,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 003: Rent Land 
//                     P50_0023_D003,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 004: Rent Equipment 
//                     P50_0023_D004,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 005: Sub Lease Income 
//                     P50_0023_D005,
                    
//                     /// Operating  Expenses
//                     /// _0023_: Rental Leases   
//                     /// 006: Rent Other Facilities 
//                     P50_0023_D006,
                    
//                 }	
//                 /// Repair Maintenance  
//                 pub enum _0024_ {
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 000: Office Lab Furniture 
//                     P50_0024_D000,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 001: Service Contracts 
//                     P50_0024_D001,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 002: Repairs 
//                     P50_0024_D002,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 003: Repairs Equipment 
//                     P50_0024_D003,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 004: IT Maintenance 
//                     P50_0024_D004,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 005: Maintenance 
//                     P50_0024_D005,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 006: Maintenance Equip 
//                     P50_0024_D006,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 007: Maint Other 
//                     P50_0024_D007,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 008: Maintenance Other Miscellaneous Hardware 
//                     P50_0024_D008,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 009: Maintenance Other Office Cleaning 
//                     P50_0024_D009,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 010: Maintenance Building 
//                     P50_0024_D010,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 011: Tools 
//                     P50_0024_D011,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 012: Services 
//                     P50_0024_D012,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 013: Purchased Supplies 
//                     P50_0024_D013,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 014: Pm Purchased Equipment Non Capital 
//                     P50_0024_D014,
                    
//                     /// Operating  Expenses
//                     /// _0024_: Repair Maintenance   
//                     /// 015: Miscellaneous Maintenance 
//                     P50_0024_D015,
                    
//                 }	
//                 /// Supplies  
//                 pub enum _0025_ {
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 000: Supplies Other 
//                     P50_0025_D000,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 001: Supplies Office 
//                     P50_0025_D001,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 002: Supplies Office Photcopy Document Handling 
//                     P50_0025_D002,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 003: Supplies Lab 
//                     P50_0025_D003,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 004: Supplies Safety 
//                     P50_0025_D004,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 005: Supplies Production 
//                     P50_0025_D005,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 006: Supplies Maintenance 
//                     P50_0025_D006,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 007: Supplies Industrial (Chemicals etc...) 
//                     P50_0025_D007,
                    
//                     /// Operating  Expenses
//                     /// _0025_: Supplies   
//                     /// 008: Supplies Cleaning 
//                     P50_0025_D008,
//                 }	
//                 /// Utilities
//                 pub enum _0026_ {
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 000: Equipment Electrical Equipment 
//                     P50_0026_D000,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 001: Computer Related Equipment 
//                     P50_0026_D001,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 002: Software Licenses 
//                     P50_0026_D002,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 003: Electricity Purchased 
//                     P50_0026_D003,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 004: Water Purchased 
//                     P50_0026_D004,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 005: Gas Purchased 
//                     P50_0026_D005,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 006: Fuel 
//                     P50_0026_D006,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 007: Utilities Combined 
//                     P50_0026_D007,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 008: Cloud Storage 
//                     P50_0026_D008,
                    
//                     /// Operating  Expenses
//                     /// _0026_: Utilities 
//                     /// 009: Datacentres 
//                     P50_0026_D009,
                    
//                 }	
//                 /// Insurance
//                 pub enum _0027_ {
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 000: Insurance Other 
//                     P50_0027_D000,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 001: Insurance Other Export Credit 
//                     P50_0027_D001,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 002: Self Insurance 
//                     P50_0027_D002,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 003: Motor vehicle insurance 
//                     P50_0027_D003,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 004: Medical insurance 
//                     P50_0027_D004,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 005: Life insurance 
//                     P50_0027_D005,
                    
//                     /// Operating  Expenses
//                     /// _0027_: Insurance 
//                     /// 006: Travel insurance 
//                     P50_0027_D006,
//                 }	
//                 /// Corporate Governance
//                 pub enum _0028_ {
//                     /// Operating  Expenses
//                     /// _0028_: Corporate Governance 
//                     /// 000: Non-exec Directors Fees, & Other Governance Costs 
//                     P50_0028_D000,
//                 }	
//                 /// Legal Fees
//                 pub enum _0029_ {
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 000: External Corporate Secretarial fees 
//                     P50_0029_D000,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 001: Litigation 
//                     P50_0029_D001,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 002: Outside Counsel Fees 
//                     P50_0029_D002,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 003: Licenses Permits 
//                     P50_0029_D003,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 004: Legal Fees International 
//                     P50_0029_D004,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 005: Legal Fees Other 
//                     P50_0029_D005,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 006: FCPA Gifts 
//                     P50_0029_D006,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 007: FCPA Travel 
//                     P50_0029_D007,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 008: FCPA Meals Entertainment 
//                     P50_0029_D008,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 009: Business Contributions 
//                     P50_0029_D009,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 010: Charitable Contributions 
//                     P50_0029_D010,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 011: Donations Disallowed 
//                     P50_0029_D011,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 012: Local Tax 
//                     P50_0029_D012,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 013: Audit Fees External 
//                     P50_0029_D013,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 014: Auditor Firm Tax Fees 
//                     P50_0029_D014,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 015: Auditor Firm Other Fees 
//                     P50_0029_D015,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 016: Audit Fees 
//                     P50_0029_D016,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 017: Patents 
//                     P50_0029_D017,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 018: Product Registrations 
//                     P50_0029_D018,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 019: Trademark and patents (P&L) 
//                     P50_0029_D019,
                    
//                     /// Operating  Expenses
//                     /// _0029_: Legal Fees 
//                     /// 020: Design Rights 
//                     P50_0029_D020,
                    
//                 }	
//                 /// Administration Cost
//                 pub enum _0030_ {
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 000: Network Transaction Fees 
//                     P50_0030_D000,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 001: Bank Charges 
//                     P50_0030_D001,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 002: Administration Business Tax 
//                     P50_0030_D002,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 003: Corporate Membership 
//                     P50_0030_D003,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 004: Memberships Subscriptions 
//                     P50_0030_D004,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 005: Other Non Deductible 
//                     P50_0030_D005,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 006: Postage Expenses 
//                     P50_0030_D006,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 007: Courrier Expenses 
//                     P50_0030_D007,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 008: Management Adjustment 
//                     P50_0030_D008,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 009: Collection Expense 
//                     P50_0030_D009,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 010: Credit Reports 
//                     P50_0030_D010,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 011: Other Miscellaneous Expense 
//                     P50_0030_D011,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 012: Miscellaneous Month End Accruals 
//                     P50_0030_D012,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 013: Sponsorship 
//                     P50_0030_D013,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 014: Books Library 
//                     P50_0030_D014,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 015: Equipment Purchased Non-Capitalised 
//                     P50_0030_D015,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 016: Software Purchased Non-Capitalised 
//                     P50_0030_D016,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 017: Miscellaneous Reimbursement Expenses 
//                     P50_0030_D017,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 018: Recovered Expenses Canteen Receipts 
//                     P50_0030_D018,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 019: Recovered Expense Miscellaneous Charge Out 
//                     P50_0030_D019,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 020: Recovered Expenses Insurance 
//                     P50_0030_D020,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 021: Recovered Expenses Car Contract 
//                     P50_0030_D021,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 022: Recovered Expenses Other 
//                     P50_0030_D022,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 023: Recovered Expenses Salaries 
//                     P50_0030_D023,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 024: Recovered Expenses Wages 
//                     P50_0030_D024,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 025: Procurement Card Purchases 
//                     P50_0030_D025,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 026: Administration Service Fees 
//                     P50_0030_D026,
                    
//                     /// Operating  Expenses
//                     /// _0030_: Administration Cost 
//                     /// 027: Other Miscellaneous Income Expense 
//                     P50_0030_D027,
//                 }	
//                 /// Bad Debts
//                 pub enum _0031_ {
//                     /// Operating  Expenses
//                     /// _0031_: Bad Debts 
//                     /// 000: Bad Debt Expense / Reversal 
//                     P50_0031_D000,
//                 }	
//                 /// Miscellaneous Expenses  
//                 pub enum _0032_ {
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 000: Shipping Cost 
//                     P50_0032_D000,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 001: Other Costs 
//                     P50_0032_D001,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 002: Standard Cost Intercompany 
//                     P50_0032_D002,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 003: Finished Goods Consumption Each 
//                     P50_0032_D003,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 004: Samples Consumption 
//                     P50_0032_D004,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 005: Semifinished Consumption 
//                     P50_0032_D005,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 006: Freight Non-Customer 
//                     P50_0032_D006,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 007: Freight From or To Warehouse 
//                     P50_0032_D007,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 008: Duties On Purchases 
//                     P50_0032_D008,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 009: Handling Warehouse 
//                     P50_0032_D009,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 010: Transportation 
//                     P50_0032_D010,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 011: Waste Disposal 
//                     P50_0032_D011,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 013: Customs Fees 
//                     P50_0032_D013,
                    
//                     /// Operating  Expenses
//                     /// _0032_: Miscellaneous Expenses   
//                     /// 014: Sponsorship / Donations 
//                     P50_0032_D014,
//                 }	
//                 /// Financial guarantee fees
//                 pub enum _0033_ {
//                     /// Operating  Expenses
//                     /// _0033_: Financial guarantee fees 
//                     P50_0033_D000,
//                 }	
//                 /// Royalty expenses
//                 pub enum _0034_ {
//                     /// Operating  Expenses
//                     /// _0034_: Royalty expenses 
//                     P50_0034_D000,
//                 }	
//                 /// Extraordinary expenses
//                 pub enum _0035_ {
//                     /// Operating  Expenses
//                     /// _0035_: Extraordinary expenses 
//                     P50_0035_D000,
//                 }	
//                 /// Impairment loss on financial assets
//                 pub enum _0036_ {
//                     /// Operating  Expenses
//                     /// _0036_: Impairment loss on financial assets 
//                     P50_0036_D000,
//                 }	
//                 /// Impairment loss on contract assets
//                 pub enum _0037_ {
//                     /// Operating  Expenses
//                     /// _0037_: Impairment loss on contract assets 
//                     P50_0037_D000,
//                 }	
//                 /// Provisions 
//                 pub enum _0038_ {
//                     /// Operating  Expenses
//                     /// _0038_: Provisions  
//                     /// 000: Warranty / Legal claims / Others - Expenses (P&L) 
//                     P50_0038_D000,
                    
//                     /// Operating  Expenses
//                     /// _0038_: Provisions  
//                     /// 001: Unused leave - Expenses (P&L) 
//                     P50_0038_D001,
//                 }	
//                 /// Property, plant and equipment written off
//                 pub enum _0039_ {
//                     /// Operating  Expenses
//                     /// _0039_: Property, plant and equipment written off 
//                     P50_0039_D000,
//                 }	
//             }
//             pub enum OtherOperatingExpenses {
//                 /// _1001_: Other Miscellaneous Charges 
//                 pub enum _1001_ {
//                     /// Other Operating Expense
//                     /// _1001_: Other Miscellaneous Charges 
//                     /// 000: Information Technology Miscellaneous 
//                     P51_1001_D000,
                    
//                     /// Other Operating Expense
//                     /// _1001_: Other Miscellaneous Charges 
//                     /// 001: Secondary Costs 
//                     P51_1001_D001,
                    
//                     /// Other Operating Expense
//                     /// _1001_: Other Miscellaneous Charges 
//                     /// 002: Reporting Adjustment 
//                     P51_1001_D002,
//                 }	
//             }
//             pub enum PersonnelCosts {
//                 pub enum _2001_ {
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 000: Salaries 
//                     P52_2001_D000,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 001: Salaries 13th Month 
//                     P52_2001_D001,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 002: Salaries Achievement Awards 
//                     P52_2001_D002,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 003: Salaries Representation Allowance 
//                     P52_2001_D003,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 004: Salaries Fees 
//                     P52_2001_D004,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 005: Salaried Operations 
//                     P52_2001_D005,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 006: Wages 13th Month 
//                     P52_2001_D006,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 007: Payroll Fees 
//                     P52_2001_D007,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 008: Payroll Luncheon Vouchers 
//                     P52_2001_D008,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 009: Social Security Employer Contribution 
//                     P52_2001_D009,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 010: Fringes Variable 
//                     P52_2001_D010,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 011: Social Security Other 
//                     P52_2001_D011,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 012: Social Security Provision 
//                     P52_2001_D012,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 013: Payroll Social Security 
//                     P52_2001_D013,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 014: Accruals Social Security Charges 13th Month 
//                     P52_2001_D014,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 015: Accruals Social Security Charges Holiday Pay 
//                     P52_2001_D015,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 016: Housing Tax 
//                     P52_2001_D016,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 017: Apprenticeship Tax 
//                     P52_2001_D017,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 018: Work Council 
//                     P52_2001_D018,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 019: Severence Pay 
//                     P52_2001_D019,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 020: Transportation Subsidy 
//                     P52_2001_D020,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 021: Wages Hourly 
//                     P52_2001_D021,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 022: Salaries Wages Per Diem 
//                     P52_2001_D022,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 023: Overtime Payments 
//                     P52_2001_D023,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 024: Sick Pay 
//                     P52_2001_D024,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 025: Holiday Pay 
//                     P52_2001_D025,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 026: Vacation Pay 
//                     P52_2001_D026,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 027: Other Employee Compensation 
//                     P52_2001_D027,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 028: Savings Plan 
//                     P52_2001_D028,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 029: Field Incentive Plan 
//                     P52_2001_D029,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 030: Pension 
//                     P52_2001_D030,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 031: Insurance Plan Live Medical 
//                     P52_2001_D031,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 032: Insurance Work Accident 
//                     P52_2001_D032,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 033: Insurance Private Collective 
//                     P52_2001_D033,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 034: Pension Costs Company Portion 
//                     P52_2001_D034,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 035: Private Patient Plan 
//                     P52_2001_D035,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 036: Early Retirement Plans 
//                     P52_2001_D036,
                    
//                     /// Personnel Costs
//                     /// _2001_: Salaries   
//                     /// 037: Employee share option expense 
//                     P52_2001_D037,
//                 }	
//                 /// Expat Expenses  
//                 pub enum _2002_ {
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 000: School Expenses 
//                     P52_2002_D000,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 001: Home Leave -Expat expenses 
//                     P52_2002_D001,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 002: Expat Cost 
//                     P52_2002_D002,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 003: Assignee Allowances Other 
//                     P52_2002_D003,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 004: Employe Language Lessons 
//                     P52_2002_D004,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 005: Employee Moving Expense 
//                     P52_2002_D005,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 006: Employee Assistance Program 
//                     P52_2002_D006,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 007: Employee Housing Expense 
//                     P52_2002_D007,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 008: Expat Other Benefits 
//                     P52_2002_D008,
                    
//                     /// Personnel Costs
//                     /// _2002_: Expat Expenses   
//                     /// 009: Assignee Tax 
//                     P52_2002_D009,
                    
//                 }	
//                 /// Incentive Plan
//                 pub enum _2003_ {
//                     /// Personnel Costs
//                     /// _2003_: Incentive Plan 
//                     P52_2003_D000,
//                 }	
//                 /// Incentive Plan Overhead
//                 pub enum _2004_ {
//                     /// Personnel Costs
//                     /// _2004_: Incentive Plan Overhead 
//                     P52_2004_D000,
//                 }	
//                 /// Employee Services  
//                 pub enum _2005_ {
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 000: Employee Service Awards 
//                     P52_2005_D000,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 001: Security Services 
//                     P52_2005_D001,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 002: Health Safety 
//                     P52_2005_D002,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 003: Cleaning And Security Services Alternate Account 
//                     P52_2005_D003,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 004: Cafeteria Service 
//                     P52_2005_D004,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 005: Medical Benefits 
//                     P52_2005_D005,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 006: Medical Services and Supplies 
//                     P52_2005_D006,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 007: Recreation Sports Social 
//                     P52_2005_D007,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 008: Recruiting 
//                     P52_2005_D008,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 009: Employee Agency Fees 
//                     P52_2005_D009,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 010: Transport Of Personal 
//                     P52_2005_D010,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 011: Education Training 
//                     P52_2005_D011,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 012: Education Training Associated Travel Costs 
//                     P52_2005_D012,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 013: Training Tax 
//                     P52_2005_D013,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 014: Tuition 
//                     P52_2005_D014,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 015: Tuition Taxable 
//                     P52_2005_D015,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 016: Laundry Clothing 
//                     P52_2005_D016,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 017: Meal Allowance 
//                     P52_2005_D017,
                    
//                     /// Personnel Costs
//                     /// _2005_: Employee Services   
//                     /// 018: Celebrations 
//                     P52_2005_D018,
                    
//                 }	
//                 /// Social Security  
//                 pub enum _2006_ {
//                     /// Personnel Costs
//                     /// _2006_: Social Security   
//                     P52_2006_D000,
                    
//                 }	
//             }
//             pub enum FinanceCosts {
//                 /// _3001_: Interest expense 
//                 pub enum _3001_ {
//                     /// Finance Costs
//                     /// _3001_: Interest expense 
//                     /// 000: Bank borrowings 
//                     P53_3001_D000,
                    
//                     /// Finance Costs
//                     /// _3001_: Interest expense 
//                     /// 001: Convertible bonds - Interest Expense (P&L) 
//                     P53_3001_D001,
                    
//                     /// Finance Costs
//                     /// _3001_: Interest expense 
//                     /// 002: Dividends on redeemable preference shares 
//                     P53_3001_D002,
                    
//                     /// Finance Costs
//                     /// _3001_: Interest expense 
//                     /// 003: Lease liabilities 
//                     P53_3001_D003,
                    
//                     /// Finance Costs
//                     /// _3001_: Interest expense 
//                     /// 004: Unwinding of discount on provisions 
//                     P53_3001_D004,
//                 }	
//             }
//         }	
//     }	
//     pub enum C {
//         /// Purchase Control.
//         C60_0001_000D,
        
//         /// Sales Control.
//         C60_0002_000D,
        
//         /// Tax Control.
//         C60_0003_000D,
        
//         /// Escrowed Funds Control.
//         C60_0004_000D,
        
//         /// Borrowings Control.
//         C60_0005_000D,
        
//         /// Defi Borrowings Control.
//         C60_0006_000D,
        
//         /// Liquidity Pool Control.
//         C60_0007_000D,
//     }
// }