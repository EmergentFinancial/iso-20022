// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
// This software is dual-licensed under the MIT License OR the Apache License, Version 2.0.
//
// MIT License
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the “Software”),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NACE_DOMAIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-U]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.079.001.02".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCounterpartyData11 {
    #[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
    pub bnfcry: Option<PartyIdentification236Choice>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
    pub clr_mmb: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "SttlmPties", skip_serializing_if = "Option::is_none")]
    pub sttlm_pties: Option<SettlementParties34Choice>,
    #[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
    pub agt_lndr: Option<OrganisationIdentification15Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyIdentification12 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification236Choice,
    #[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
    pub brnch: Option<Branch6Choice>,
    #[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
    pub ctry_cd: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType29Code {
    #[serde(rename = "LAMP")]
    Lamp,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType5Code {
    #[serde(rename = "GRIN")]
    Grin,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification38 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification175,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType4Code {
    #[serde(rename = "FRGT")]
    Frgt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Branch5ChoiceEnum {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Branch5Choice {
    #[serde(flatten)]
    pub value: Branch5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType29Code {
    #[serde(rename = "CRBR")]
    Crbr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType14Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsinOct2015Identifier {
    #[validate(regex = "ISIN_OCT_2015_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType16Code {
    #[serde(rename = "PRME")]
    Prme,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max105Text {
    #[validate(length(min = 1, max = 105,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod1Code {
    #[serde(rename = "A001")]
    A001,
    #[serde(rename = "A002")]
    A002,
    #[serde(rename = "A003")]
    A003,
    #[serde(rename = "A004")]
    A004,
    #[serde(rename = "A005")]
    A005,
    #[serde(rename = "A006")]
    A006,
    #[serde(rename = "A007")]
    A007,
    #[serde(rename = "A008")]
    A008,
    #[serde(rename = "A009")]
    A009,
    #[serde(rename = "A010")]
    A010,
    #[serde(rename = "A011")]
    A011,
    #[serde(rename = "A012")]
    A012,
    #[serde(rename = "A013")]
    A013,
    #[serde(rename = "A014")]
    A014,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType25Code {
    #[serde(rename = "DIST")]
    Dist,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType36Code {
    #[serde(rename = "NSPT")]
    Nspt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityDry2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType31Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType33Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice18ChoiceEnum {
    #[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
    pub bsis_pts: Option<DecimalNumber>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection107>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice18Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice18ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType44Code {
    #[serde(rename = "UAAN")]
    Uaan,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice5 {
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityAgricultural5ChoiceEnum {
    #[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
    pub ptt: Option<AgriculturalCommodityPotato1>,
    #[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
    pub frstry: Option<AgriculturalCommodityForestry1>,
    #[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
    pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
    #[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
    pub soft: Option<AgriculturalCommoditySoft1>,
    #[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
    pub sfd: Option<AgriculturalCommoditySeafood1>,
    #[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
    pub grn: Option<AgriculturalCommodityGrain2>,
    #[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
    pub dairy: Option<AgriculturalCommodityDairy1>,
    #[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
    pub live_stock: Option<AgriculturalCommodityLiveStock1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AgriculturalCommodityOther1>,
    #[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
    pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityAgricultural5Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityAgricultural5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityOliveOil2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType3Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType29Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountHaircutMargin1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: AmountAndDirection53,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyIdentification11 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification15Choice,
    #[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
    pub ntr: Option<CounterpartyTradeNature7Choice>,
    #[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
    pub brnch: Option<Branch5Choice>,
    #[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
    pub sd: Option<CollateralRole1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityDairy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType20Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType31Code {
    #[serde(rename = "DRYF")]
    Dryf,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType24Code {
    #[serde(rename = "COAL")]
    Coal,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceStatus1Code {
    #[serde(rename = "PNDG")]
    Pndg,
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityCoal1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType24Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralRole1Code {
    #[serde(rename = "GIVE")]
    Give,
    #[serde(rename = "TAKE")]
    Take,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Branch6ChoiceEnum {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification236Choice>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Branch6Choice {
    #[serde(flatten)]
    pub value: Branch6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType37Code {
    #[serde(rename = "PULP")]
    Pulp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateBasis1Code {
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType39Code {
    #[serde(rename = "AMMO")]
    Ammo,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max72Text {
    #[validate(length(min = 1, max = 72,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType34Code {
    #[serde(rename = "MFTG")]
    Mftg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotAvailable1Code {
    #[serde(rename = "NTAV")]
    Ntav,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties34ChoiceEnum {
    #[serde(rename = "IndrctPtcpt", skip_serializing_if = "Option::is_none")]
    pub indrct_ptcpt: Option<OrganisationIdentification15Choice>,
    #[serde(
        rename = "CntrlSctiesDpstryPtcpt",
        skip_serializing_if = "Option::is_none"
    )]
    pub cntrl_scties_dpstry_ptcpt: Option<OrganisationIdentification15Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties34Choice {
    #[serde(flatten)]
    pub value: SettlementParties34ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType45Code {
    #[serde(rename = "POTA")]
    Pota,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate6 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: AmountAndDirection53,
    #[serde(rename = "IntrstRate")]
    pub intrst_rate: InterestRate27Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractModification3 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TransactionOperationType6Code,
    #[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
    pub lvl: Option<ModificationLevel1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAdjustment1 {
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
    #[validate]
    #[serde(rename = "AdjstmntDt")]
    pub adjstmnt_dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType33Code {
    #[serde(rename = "CSTR")]
    Cstr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityInterEnergy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType26Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeRepositoryReportingType1Code {
    #[serde(rename = "SWOS")]
    Swos,
    #[serde(rename = "TWOS")]
    Twos,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType33Code {
    #[serde(rename = "DBCR")]
    Dbcr,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityCarbonRelated1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType29Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType35Code {
    #[serde(rename = "CBRD")]
    Cbrd,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndustrialProductCommodityConstruction1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType33Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityAmmonia1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType39Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType15Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType11Code {
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PLDM")]
    Pldm,
    #[serde(rename = "PTNM")]
    Ptnm,
    #[serde(rename = "SLVR")]
    Slvr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PercentageRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FixedRate11 {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType49Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityLiveStock1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType22Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFreight3ChoiceEnum {
    #[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
    pub cntnr_ship: Option<FreightCommodityContainerShip1>,
    #[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
    pub wet: Option<FreightCommodityWet2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FreightCommodityOther1>,
    #[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
    pub dry: Option<FreightCommodityDry2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFreight3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFreight3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType5Code {
    #[serde(rename = "BSLD")]
    Bsld,
    #[serde(rename = "FITR")]
    Fitr,
    #[serde(rename = "PKLD")]
    Pkld,
    #[serde(rename = "OFFP")]
    Offp,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalAgreementType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityContainerShip1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType46Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType7Code {
    #[serde(rename = "NGAS")]
    Ngas,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType34Code {
    #[serde(rename = "TNKR")]
    Tnkr,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType32Code {
    #[serde(rename = "WETF")]
    Wetf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalSecuritiesLendingType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Security52 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<CfiOct2015Identifier>,
    #[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
    pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CollateralQualityType1Code>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<IsoDate>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<SecurityIssuer4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tp", default)]
    pub tp: Vec<SecuritiesLendingType3Choice>,
    #[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
    pub exclsv_arrgmnt: Option<TrueFalseIndicator>,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<PercentageRate>,
    #[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
    pub avlbl_for_coll_reuse: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnvironmental2ChoiceEnum {
    #[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
    pub wthr: Option<EnvironmentalCommodityWeather1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnvironmentCommodityOther1>,
    #[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
    pub emssns: Option<EnvironmentalCommodityEmission2>,
    #[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
    pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnvironmental2Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnvironmental2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType2Code {
    #[serde(rename = "ROBU")]
    Robu,
    #[serde(rename = "CCOA")]
    Ccoa,
    #[serde(rename = "BRWN")]
    Brwn,
    #[serde(rename = "WHSG")]
    Whsg,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityDistillates1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType25Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityUrea1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType43Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20PositiveNumber {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndustrialProductCommodityManufacturing1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType34Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanData140 {
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<Max52Text>,
    #[validate]
    #[serde(rename = "EvtDt")]
    pub evt_dt: IsoDate,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
    pub clr_sts: Option<Cleared16Choice>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<MicIdentifier>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement7>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<SpecialCollateral1Code>,
    #[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
    pub prncpl_amt: Option<PrincipalAmount3>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification15ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OrganisationIdentification38>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification15Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityWeather1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType30Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Security51 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<CfiOct2015Identifier>,
    #[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
    pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CollateralQualityType1Code>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<IsoDate>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<SecurityIssuer4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tp", default)]
    pub tp: Vec<SecuritiesLendingType3Choice>,
    #[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
    pub exclsv_arrgmnt: Option<TrueFalseIndicator>,
    #[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
    pub avlbl_for_coll_reuse: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesLendingType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalSecuritiesLendingType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesLendingType3Choice {
    #[serde(flatten)]
    pub value: SecuritiesLendingType3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType8Code {
    #[serde(rename = "CERE")]
    Cere,
    #[serde(rename = "ERUE")]
    Erue,
    #[serde(rename = "EUAE")]
    Euae,
    #[serde(rename = "EUAA")]
    Euaa,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityGrain2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType5Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType30Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType8Code {
    #[serde(rename = "PAPR")]
    Papr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName3Code {
    #[serde(rename = "ESTR")]
    Estr,
    #[serde(rename = "BBSW")]
    Bbsw,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "SOFR")]
    Sofr,
    #[serde(rename = "SONA")]
    Sona,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LongFraction19DecimalNumber {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType1Code {
    #[serde(rename = "AGRI")]
    Agri,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialPartyClassification1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Clssfctn", default)]
    pub clssfctn: Vec<FinancialPartySectorType2Code>,
    #[serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none")]
    pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityContainerBoard1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType35Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Security55 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<CfiOct2015Identifier>,
    #[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
    pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CollateralQualityType1Code>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<IsoDate>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<SecurityIssuer4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tp", default)]
    pub tp: Vec<SecuritiesLendingType3Choice>,
    #[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
    pub exclsv_arrgmnt: Option<TrueFalseIndicator>,
    #[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
    pub avlbl_for_coll_reuse: Option<TrueFalseIndicator>,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityForestry1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType21Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityNaturalGas2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType7Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType31Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeStateReport16<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max140Text>,
    #[validate]
    #[serde(rename = "CtrPtySpcfcData")]
    pub ctr_pty_spcfc_data: CounterpartyData88,
    #[serde(rename = "LnData", skip_serializing_if = "Option::is_none")]
    pub ln_data: Option<TransactionLoanData31Choice>,
    #[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
    pub coll_data: Option<TransactionCollateralData18Choice>,
    #[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
    pub rcncltn_flg: Option<ReconciliationFlag2>,
    #[validate]
    #[serde(rename = "CtrctMod")]
    pub ctrct_mod: ContractModification3,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityRecoveredPaper1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType38Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName10ChoiceEnum {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName10Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundType2Code {
    #[serde(rename = "ETFT")]
    Etft,
    #[serde(rename = "MMFT")]
    Mmft,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "REIT")]
    Reit,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportPeriodActivity1Code {
    #[serde(rename = "NOTX")]
    Notx,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityCommodity9 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Scty", default)]
    pub scty: Vec<Security51>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmmdty", default)]
    pub cmmdty: Vec<Commodity43>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityOilSeed1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType1Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Cleared16ChoiceEnum {
    #[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
    pub clrd: Option<ClearingPartyAndTime14>,
    #[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
    pub non_clrd: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Cleared16Choice {
    #[serde(flatten)]
    pub value: Cleared16ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityIndustrialProduct1ChoiceEnum {
    #[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
    pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
    #[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
    pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityIndustrialProduct1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType7Code {
    #[serde(rename = "METL")]
    Metl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType2Code {
    #[serde(rename = "NRGY")]
    Nrgy,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType23Code {
    #[serde(rename = "SEAF")]
    Seaf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMetal1ChoiceEnum {
    #[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
    pub non_prcs: Option<MetalCommodityNonPrecious1>,
    #[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
    pub prcs: Option<MetalCommodityPrecious1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMetal1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityMetal1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlusOrMinusIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType22Code {
    #[serde(rename = "LSTK")]
    Lstk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAndAmount")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyData88 {
    #[validate]
    #[serde(rename = "RptgDtTm")]
    pub rptg_dt_tm: IsoDateTime,
    #[serde(rename = "RptSubmitgNtty")]
    pub rpt_submitg_ntty: OrganisationIdentification15Choice,
    #[validate(length(min = 1, max = 2,))]
    #[serde(rename = "CtrPty", default)]
    pub ctr_pty: Vec<CounterpartyData89>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LeiIdentifier {
    #[validate(regex = "LEI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFertilizer3ChoiceEnum {
    #[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
    pub slphr: Option<FertilizerCommoditySulphur1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FertilizerCommodityOther1>,
    #[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
    pub urea: Option<FertilizerCommodityUrea1>,
    #[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
    pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
    #[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
    pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
    #[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
    pub ptsh: Option<FertilizerCommodityPotash1>,
    #[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
    pub ammn: Option<FertilizerCommodityAmmonia1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFertilizer3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFertilizer3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType2Code {
    #[serde(rename = "SOFT")]
    Soft,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Commodity43 {
    #[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
    pub clssfctn: Option<AssetClassCommodity5Choice>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<Quantity17>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType42Code {
    #[serde(rename = "SLPH")]
    Slph,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType30Code {
    #[serde(rename = "MWHT")]
    Mwht,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType6Code {
    #[serde(rename = "INDP")]
    Indp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType27Code {
    #[serde(rename = "LGHT")]
    Lght,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType31Code {
    #[serde(rename = "GASP")]
    Gasp,
    #[serde(rename = "LNGG")]
    Lngg,
    #[serde(rename = "NCGG")]
    Ncgg,
    #[serde(rename = "TTFG")]
    Ttfg,
    #[serde(rename = "NBPG")]
    Nbpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure11Code {
    #[serde(rename = "ALOW")]
    Alow,
    #[serde(rename = "ACCY")]
    Accy,
    #[serde(rename = "BARL")]
    Barl,
    #[serde(rename = "BCUF")]
    Bcuf,
    #[serde(rename = "BDFT")]
    Bdft,
    #[serde(rename = "BUSL")]
    Busl,
    #[serde(rename = "CEER")]
    Ceer,
    #[serde(rename = "CLRT")]
    Clrt,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "ACRE")]
    Acre,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "CBME")]
    Cbme,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "DMET")]
    Dmet,
    #[serde(rename = "ENVC")]
    Envc,
    #[serde(rename = "ENVO")]
    Envo,
    #[serde(rename = "HUWG")]
    Huwg,
    #[serde(rename = "KWDC")]
    Kwdc,
    #[serde(rename = "KWHO")]
    Kwho,
    #[serde(rename = "KWHC")]
    Kwhc,
    #[serde(rename = "KMOC")]
    Kmoc,
    #[serde(rename = "KWMC")]
    Kwmc,
    #[serde(rename = "KWYC")]
    Kwyc,
    #[serde(rename = "MWDC")]
    Mwdc,
    #[serde(rename = "MWHO")]
    Mwho,
    #[serde(rename = "MWHC")]
    Mwhc,
    #[serde(rename = "MWMC")]
    Mwmc,
    #[serde(rename = "MMOC")]
    Mmoc,
    #[serde(rename = "MWYC")]
    Mwyc,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "MIBA")]
    Miba,
    #[serde(rename = "MBTU")]
    Mbtu,
    #[serde(rename = "OZTR")]
    Oztr,
    #[serde(rename = "UCWT")]
    Ucwt,
    #[serde(rename = "IPNT")]
    Ipnt,
    #[serde(rename = "PWRD")]
    Pwrd,
    #[serde(rename = "DGEU")]
    Dgeu,
    #[serde(rename = "TOCD")]
    Tocd,
    #[serde(rename = "GGEU")]
    Ggeu,
    #[serde(rename = "USQA")]
    Usqa,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType8Code {
    #[serde(rename = "OILP")]
    Oilp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType14Code {
    #[serde(rename = "OEST")]
    Oest,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType3Code {
    #[serde(rename = "OOLI")]
    Ooli,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeStateReport5ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub stat: Option<TradeStateReport16<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeStateReport5Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: TradeStateReport5ChoiceEnum<A>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection107 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType10Code {
    #[serde(rename = "EMIS")]
    Emis,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Collateral52 {
    #[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
    pub coll_val_dt: Option<IsoDate>,
    #[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
    pub asst_tp: Option<CollateralType21>,
    #[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
    pub net_xpsr_collstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
    pub bskt_idr: Option<SecurityIdentification26Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralFlag13ChoiceEnum {
    #[serde(rename = "Uncollsd", skip_serializing_if = "Option::is_none")]
    pub uncollsd: Option<NoReasonCode>,
    #[serde(rename = "Collsd", skip_serializing_if = "Option::is_none")]
    pub collsd: Option<CollaterisedData12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralFlag13Choice {
    #[serde(flatten)]
    pub value: CollateralFlag13ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DecimalNumber {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType15Code {
    #[serde(rename = "NPRM")]
    Nprm,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyTradeNature7ChoiceEnum {
    #[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
    pub nfi: Option<FinancialPartyClassification2>,
    #[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
    pub fi: Option<FinancialPartyClassification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyTradeNature7Choice {
    #[serde(flatten)]
    pub value: CounterpartyTradeNature7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommoditySeafood1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType23Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType20Code {
    #[serde(rename = "DIRY")]
    Diry,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityOil2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType8Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType32Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnergy2ChoiceEnum {
    #[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
    pub lght_end: Option<EnergyCommodityLightEnd1>,
    #[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
    pub elctrcty: Option<EnergyCommodityElectricity1>,
    #[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
    pub oil: Option<EnergyCommodityOil2>,
    #[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
    pub coal: Option<EnergyCommodityCoal1>,
    #[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
    pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnergyCommodityOther1>,
    #[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
    pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
    #[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
    pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
    #[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
    pub dstllts: Option<EnergyCommodityDistillates1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnergy2Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnergy2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType1Code {
    #[serde(rename = "GROS")]
    Gros,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType13Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MetalCommodityPrecious1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType16Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType11Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SpecialCollateral1Code {
    #[serde(rename = "GENE")]
    Gene,
    #[serde(rename = "SPEC")]
    Spec,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralType21 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Scty", default)]
    pub scty: Vec<Security52>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Csh", default)]
    pub csh: Vec<AmountHaircutMargin1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmmdty", default)]
    pub cmmdty: Vec<Commodity43>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType38Code {
    #[serde(rename = "RCVP")]
    Rcvp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MetalCommodityNonPrecious1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType15Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType10Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityNewsprint1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType36Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType48Code {
    #[serde(rename = "NDLV")]
    Ndlv,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralDeliveryMethod1Code {
    #[serde(rename = "SICA")]
    Sica,
    #[serde(rename = "SIUR")]
    Siur,
    #[serde(rename = "TTCA")]
    Ttca,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat6Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethodFormat6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialPartySectorType2Code {
    #[serde(rename = "AIFD")]
    Aifd,
    #[serde(rename = "CSDS")]
    Csds,
    #[serde(rename = "CCPS")]
    Ccps,
    #[serde(rename = "CDTI")]
    Cdti,
    #[serde(rename = "INUN")]
    Inun,
    #[serde(rename = "ORPI")]
    Orpi,
    #[serde(rename = "INVF")]
    Invf,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "UCIT")]
    Ucit,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PolypropyleneCommodityPlastic1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType9Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType18Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoTerminationOption2Code {
    #[serde(rename = "EGRN")]
    Egrn,
    #[serde(rename = "EGAE")]
    Egae,
    #[serde(rename = "ETSB")]
    Etsb,
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType3Code {
    #[serde(rename = "ENVR")]
    Envr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification175 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max72Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate27ChoiceEnum {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<FixedRate11>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<FloatingInterestRate22>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate27Choice {
    #[serde(flatten)]
    pub value: InterestRate27ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityLightEnd1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType27Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanData141 {
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<Max52Text>,
    #[validate]
    #[serde(rename = "EvtDt")]
    pub evt_dt: IsoDate,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
    pub clr_sts: Option<Cleared16Choice>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<MicIdentifier>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement7>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<SpecialCollateral1Code>,
    #[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
    pub dlvry_by_val: Option<TrueFalseIndicator>,
    #[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
    pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Term", default)]
    pub term: Vec<ContractTerm7Choice>,
    #[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
    pub asst_tp: Option<SecurityCommodity9>,
    #[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
    pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "RbtRate", skip_serializing_if = "Option::is_none")]
    pub rbt_rate: Option<InterestRate27Choice>,
    #[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
    pub lndg_fee: Option<PercentageRate>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingPartyAndTime14 {
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none")]
    pub rpt_trckg_nb: Option<Max52Text>,
    #[serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none")]
    pub prtfl_cd: Option<Max52Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType18Code {
    #[serde(rename = "PLST")]
    Plst,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType46Code {
    #[serde(rename = "CSHP")]
    Cshp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType32Code {
    #[serde(rename = "BAKK")]
    Bakk,
    #[serde(rename = "BDSL")]
    Bdsl,
    #[serde(rename = "BRNT")]
    Brnt,
    #[serde(rename = "BRNX")]
    Brnx,
    #[serde(rename = "CNDA")]
    Cnda,
    #[serde(rename = "COND")]
    Cond,
    #[serde(rename = "DSEL")]
    Dsel,
    #[serde(rename = "DUBA")]
    Duba,
    #[serde(rename = "ESPO")]
    Espo,
    #[serde(rename = "ETHA")]
    Etha,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "FOIL")]
    Foil,
    #[serde(rename = "GOIL")]
    Goil,
    #[serde(rename = "GSLN")]
    Gsln,
    #[serde(rename = "HEAT")]
    Heat,
    #[serde(rename = "JTFL")]
    Jtfl,
    #[serde(rename = "KERO")]
    Kero,
    #[serde(rename = "LLSO")]
    Llso,
    #[serde(rename = "MARS")]
    Mars,
    #[serde(rename = "NAPH")]
    Naph,
    #[serde(rename = "NGLO")]
    Nglo,
    #[serde(rename = "TAPI")]
    Tapi,
    #[serde(rename = "WTIO")]
    Wtio,
    #[serde(rename = "URAL")]
    Ural,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaturalPersonIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification175,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PolypropyleneCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType9Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType44Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType10Code {
    #[serde(rename = "ALUM")]
    Alum,
    #[serde(rename = "ALUA")]
    Alua,
    #[serde(rename = "CBLT")]
    Cblt,
    #[serde(rename = "COPR")]
    Copr,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "MOLY")]
    Moly,
    #[serde(rename = "NASC")]
    Nasc,
    #[serde(rename = "NICK")]
    Nick,
    #[serde(rename = "STEL")]
    Stel,
    #[serde(rename = "TINN")]
    Tinn,
    #[serde(rename = "ZINC")]
    Zinc,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "LEAD")]
    Lead,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOtherC102ChoiceEnum {
    #[serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none")]
    pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
    #[serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none")]
    pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOtherC102Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityOtherC102ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPaper3ChoiceEnum {
    #[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
    pub nwsprnt: Option<PaperCommodityNewsprint1>,
    #[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
    pub pulp: Option<PaperCommodityPulp1>,
    #[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
    pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PaperCommodityRecoveredPaper2>,
    #[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
    pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPaper3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPaper3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanData142 {
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<Max52Text>,
    #[validate]
    #[serde(rename = "EvtDt")]
    pub evt_dt: IsoDate,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<MicIdentifier>,
    #[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
    pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
    #[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
    pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
    pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MrgnLnAttr", default)]
    pub mrgn_ln_attr: Vec<InterestRate6>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType15Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType6Code {
    #[serde(rename = "ELEC")]
    Elec,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "SctiesFincgRptgTxStatRpt")]
    pub scties_fincg_rptg_tx_stat_rpt: SecuritiesFinancingReportingTransactionStateReportV02<A, B>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType1Code {
    #[serde(rename = "FWHT")]
    Fwht,
    #[serde(rename = "SOYB")]
    Soyb,
    #[serde(rename = "RPSD")]
    Rpsd,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CORN")]
    Corn,
    #[serde(rename = "RICE")]
    Rice,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType30Code {
    #[serde(rename = "WTHR")]
    Wthr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralQualityType1Code {
    #[serde(rename = "INVG")]
    Invg,
    #[serde(rename = "NIVG")]
    Nivg,
    #[serde(rename = "NOTR")]
    Notr,
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationFlag2 {
    #[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
    pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
    #[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
    pub both_ctr_pties_rptg: Option<TrueFalseIndicator>,
    #[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
    pub paird_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
    pub ln_rcncltn_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
    pub coll_rcncltn_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
    pub mod_sts: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCollateralData18ChoiceEnum {
    #[serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none")]
    pub scties_lndg: Option<CollateralFlag13Choice>,
    #[serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none")]
    pub mrgn_lndg: Option<Security55>,
    #[serde(rename = "RpTrad", skip_serializing_if = "Option::is_none")]
    pub rp_trad: Option<Collateral52>,
    #[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
    pub buy_sell_bck: Option<Collateral52>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCollateralData18Choice {
    #[serde(flatten)]
    pub value: TransactionCollateralData18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyData89 {
    #[validate]
    #[serde(rename = "RptgCtrPty")]
    pub rptg_ctr_pty: CounterpartyIdentification11,
    #[validate]
    #[serde(rename = "OthrCtrPty")]
    pub othr_ctr_pty: CounterpartyIdentification12,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "OthrPtyData", skip_serializing_if = "Option::is_none")]
    pub othr_pty_data: Option<TransactionCounterpartyData11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityRecoveredPaper2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType49Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommoditySoft1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType2Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType2Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaceDomainIdentifier {
    #[validate(regex = "NACE_DOMAIN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityNominalValue2ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<DecimalNumber>,
    #[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
    pub nmnl_val: Option<AmountAndDirection53>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityNominalValue2Choice {
    #[serde(flatten)]
    pub value: QuantityNominalValue2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialPartyClassification2 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Clssfctn", default)]
    pub clssfctn: Vec<NaceDomainIdentifier>,
    #[serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none")]
    pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity17 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: DecimalNumber,
    #[serde(rename = "UnitOfMeasr")]
    pub unit_of_measr: UnitOfMeasure11Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType43Code {
    #[serde(rename = "UREA")]
    Urea,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractTerm7ChoiceEnum {
    #[serde(rename = "Opn", skip_serializing_if = "Option::is_none")]
    pub opn: Option<FixedOpenTermContract2>,
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<FixedOpenTermContract2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractTerm7Choice {
    #[serde(flatten)]
    pub value: ContractTerm7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification26ChoiceEnum {
    #[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
    pub not_avlbl: Option<NotAvailable1Code>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification26Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification26ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType11Code {
    #[serde(rename = "OTHC")]
    Othc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice19ChoiceEnum {
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection107>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
    pub pdg_pric: Option<PriceStatus1Code>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<SecuritiesTransactionPrice5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice19Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityWet2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType32Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType34Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityEmission2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType10Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType8Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherC10CommodityDeliverable2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType11Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType47Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max50Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAgreementType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementType2Choice {
    #[serde(flatten)]
    pub value: AgreementType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityPotato1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType45Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityPotash1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType41Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType5Code {
    #[serde(rename = "FRTL")]
    Frtl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FloatingInterestRate22 {
    #[serde(rename = "RefRate", skip_serializing_if = "Option::is_none")]
    pub ref_rate: Option<BenchmarkCurveName10Choice>,
    #[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
    pub term: Option<InterestRateContractTerm2>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<InterestRateContractTerm2>,
    #[serde(rename = "RstFrqcy", skip_serializing_if = "Option::is_none")]
    pub rst_frqcy: Option<InterestRateContractTerm2>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<SecuritiesTransactionPrice18Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RateAdjstmnt", default)]
    pub rate_adjstmnt: Vec<RateAdjustment1>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIssuer4 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "JursdctnCtry")]
    pub jursdctn_ctry: CountryCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType28Code {
    #[serde(rename = "RNNG")]
    Rnng,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesFinancingReportingTransactionStateReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "TradData")]
    pub trad_data: TradeStateReport5Choice<A>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FixedOpenTermContract2 {
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
    pub termntn_optn: Option<RepoTerminationOption2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max50Text {
    #[validate(length(min = 1, max = 50,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanData139 {
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<Max52Text>,
    #[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
    pub evt_dt: Option<IsoDate>,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
    pub clr_sts: Option<Cleared16Choice>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<MicIdentifier>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement7>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
    pub min_ntce_prd: Option<Max20PositiveNumber>,
    #[serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none")]
    pub earlst_call_bck_dt: Option<IsoDate>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<SpecialCollateral1Code>,
    #[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
    pub dlvry_by_val: Option<TrueFalseIndicator>,
    #[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
    pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Term", default)]
    pub term: Vec<ContractTerm7Choice>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<InterestRate27Choice>,
    #[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
    pub prncpl_amt: Option<PrincipalAmount3>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType41Code {
    #[serde(rename = "PTSH")]
    Ptsh,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ModificationLevel1Code {
    #[serde(rename = "PSTN")]
    Pstn,
    #[serde(rename = "TCTN")]
    Tctn,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityInflation1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType12Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherC10CommodityNonDeliverable2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType11Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType48Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityPulp1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType37Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommoditySulphur1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType42Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification236ChoiceEnum {
    #[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
    pub lgl: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
    pub ntrl: Option<NaturalPersonIdentification2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification236Choice {
    #[serde(flatten)]
    pub value: PartyIdentification236ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PrincipalAmount3 {
    #[serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none")]
    pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollaterisedData12 {
    #[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
    pub coll_val_dt: Option<IsoDate>,
    #[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
    pub asst_tp: Option<CollateralType21>,
    #[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
    pub net_xpsr_collstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
    pub bskt_idr: Option<SecurityIdentification26Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MasterAgreement7 {
    #[serde(rename = "Tp")]
    pub tp: AgreementType2Choice,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max50Text>,
    #[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
    pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityRenewableEnergy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType28Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection53 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType40Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateContractTerm2 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max3Number,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType12Code {
    #[serde(rename = "INFL")]
    Infl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CfiOct2015Identifier {
    #[validate(regex = "CFI_OCT_2015_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityElectricity1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType6Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType5Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodity5ChoiceEnum {
    #[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
    pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
    #[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
    pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
    #[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
    pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
    #[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
    pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
    #[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
    pub ppr: Option<AssetClassCommodityPaper3Choice>,
    #[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
    pub metl: Option<AssetClassCommodityMetal1Choice>,
    #[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
    pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
    #[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
    pub othr_c_10: Option<AssetClassCommodityOtherC102Choice>,
    #[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
    pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
    #[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
    pub infltn: Option<AssetClassCommodityInflation1>,
    #[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
    pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AssetClassCommodityOther1>,
    #[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
    pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
    #[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
    pub frght: Option<AssetClassCommodityFreight3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodity5Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodity5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType40Code {
    #[serde(rename = "DAPH")]
    Daph,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType9Code {
    #[serde(rename = "POLY")]
    Poly,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPolypropylene3ChoiceEnum {
    #[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
    pub plstc: Option<PolypropyleneCommodityPlastic1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PolypropyleneCommodityOther1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPolypropylene3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPolypropylene3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType6Code {
    #[serde(rename = "REUU")]
    Reuu,
    #[serde(rename = "COLU")]
    Colu,
    #[serde(rename = "CORR")]
    Corr,
    #[serde(rename = "ETRM")]
    Etrm,
    #[serde(rename = "VALU")]
    Valu,
    #[serde(rename = "POSC")]
    Posc,
    #[serde(rename = "NEWT")]
    Newt,
    #[serde(rename = "MODI")]
    Modi,
    #[serde(rename = "MARU")]
    Maru,
    #[serde(rename = "EROR")]
    Eror,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionLoanData31ChoiceEnum {
    #[serde(rename = "RpTrad", skip_serializing_if = "Option::is_none")]
    pub rp_trad: Option<LoanData139>,
    #[serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none")]
    pub scties_lndg: Option<LoanData141>,
    #[serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none")]
    pub mrgn_lndg: Option<LoanData142>,
    #[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
    pub buy_sell_bck: Option<LoanData140>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionLoanData31Choice {
    #[serde(flatten)]
    pub value: TransactionLoanData31ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType47Code {
    #[serde(rename = "DLVR")]
    Dlvr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType13Code {
    #[serde(rename = "MCEX")]
    Mcex,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType21Code {
    #[serde(rename = "FRST")]
    Frst,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType26Code {
    #[serde(rename = "INRG")]
    Inrg,
    #[default]
    Unknown,
}
