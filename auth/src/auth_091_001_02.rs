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
    static ref UTI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
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
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9\-]{16}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_SUB_DIVISION_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}\-[0-9A-Z]{1,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.091.001.02".to_string()
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
pub struct CompareReportingLevelType2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ModificationLevel1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ModificationLevel1Code>,
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
pub struct PortfolioCode5ChoiceEnum {
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioIdentification3>,
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
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
pub struct PortfolioCode5Choice {
    #[serde(flatten)]
    pub value: PortfolioCode5ChoiceEnum,
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
pub struct CompareUnitPrice4 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesTransactionPrice17Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesTransactionPrice17Choice>,
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
pub struct CompareCfiIdentifier3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CfiOct2015Identifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CfiOct2015Identifier>,
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
pub struct CounterpartyMatchingCriteria6 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
    #[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
    pub drctn_or_sd: Option<CompareLegDirection2>,
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
pub struct AssetClassCommodityIndustrialProduct2ChoiceEnum {
    #[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
    pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
    #[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
    pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
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
pub struct AssetClassCommodityIndustrialProduct2Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityIndustrialProduct2ChoiceEnum,
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
pub struct CompareFrequencyUnit1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Frequency13Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Frequency13Code>,
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
pub struct EnergyCommodityDistillates2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType25Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DebtInstrumentSeniorityType2Code {
    #[serde(rename = "SBOD")]
    Sbod,
    #[serde(rename = "SNDB")]
    Sndb,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType7Code {
    #[serde(rename = "NGAS")]
    Ngas,
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
pub struct NonClearingReason2 {
    #[validate(length(min = 1,))]
    #[serde(rename = "ClrXmptnXcptn", default)]
    pub clr_xmptn_xcptn: Vec<ClearingExemptionException1Code>,
    #[serde(rename = "NonClrRsnInf", skip_serializing_if = "Option::is_none")]
    pub non_clr_rsn_inf: Option<Max350Text>,
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
pub struct CompareAmountAndDirection3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AmountAndDirection106>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AmountAndDirection106>,
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
pub struct PaperCommodityNewsprint2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType36Code>,
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
pub struct AssetClassCommodityMetal2ChoiceEnum {
    #[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
    pub non_prcs: Option<MetalCommodityNonPrecious2>,
    #[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
    pub prcs: Option<MetalCommodityPrecious2>,
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
pub struct AssetClassCommodityMetal2Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityMetal2ChoiceEnum,
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
pub struct CompareNumber7 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Number>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Number>,
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
pub struct CompareOtherPaymentType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PaymentType5Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PaymentType5Choice>,
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
pub struct CompareActiveOrHistoricCurrencyAndAmount4 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
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
pub struct EnvironmentalCommodityCarbonRelated2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType29Code>,
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
pub struct MasterAgreement8 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<AgreementType2Choice>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max50Text>,
    #[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
    pub othr_mstr_agrmt_dtls: Option<Max350Text>,
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
pub struct CompareDateTime3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<IsoDateTime>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<IsoDateTime>,
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
pub struct IndexIdentification1 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<ExternalBenchmarkCurveName1Code>,
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
pub struct AssetClassCommodity6ChoiceEnum {
    #[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
    pub metl: Option<AssetClassCommodityMetal2Choice>,
    #[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
    pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
    #[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
    pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
    #[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
    pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
    #[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
    pub infltn: Option<AssetClassCommodityInflation1>,
    #[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
    pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
    #[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
    pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AssetClassCommodityOther1>,
    #[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
    pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
    #[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
    pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<AssetClassCommodityIndex1>,
    #[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
    pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
    #[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
    pub frght: Option<AssetClassCommodityFreight4Choice>,
    #[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
    pub othr_c_10: Option<AssetClassCommodityC10Other1>,
    #[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
    pub ppr: Option<AssetClassCommodityPaper4Choice>,
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
pub struct AssetClassCommodity6Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodity6ChoiceEnum,
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
pub struct ClearingExceptionOrExemption3ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<NoReasonCode>,
    #[serde(rename = "CtrPties", skip_serializing_if = "Option::is_none")]
    pub ctr_pties: Option<ClearingExceptionOrExemption2>,
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
pub struct ClearingExceptionOrExemption3Choice {
    #[serde(flatten)]
    pub value: ClearingExceptionOrExemption3ChoiceEnum,
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
pub struct CompareUnitPrice7 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesTransactionPrice14Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesTransactionPrice14Choice>,
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
pub struct CompareUnitPrice5 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesTransactionPrice17Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesTransactionPrice17Choice>,
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
pub struct EnergyCommodityElectricity2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType6Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType5Code>,
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
pub struct EnergyCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType49Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType25Code {
    #[serde(rename = "DIST")]
    Dist,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DerivativeEventType3Code {
    #[serde(rename = "ALOC")]
    Aloc,
    #[serde(rename = "CLRG")]
    Clrg,
    #[serde(rename = "CLAL")]
    Clal,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CREV")]
    Crev,
    #[serde(rename = "ETRM")]
    Etrm,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "INCP")]
    Incp,
    #[serde(rename = "NOVA")]
    Nova,
    #[serde(rename = "PTNG")]
    Ptng,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "UPDT")]
    Updt,
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
pub struct UtiIdentifier {
    #[validate(regex = "UTI_IDENTIFIER_REGEX")]
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
pub struct IsoTime {
    #[serde(rename = "$value")]
    pub value: ::chrono::naive::NaiveTime,
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
pub struct ExchangeRateBasis1ChoiceEnum {
    #[serde(rename = "CcyPair", skip_serializing_if = "Option::is_none")]
    pub ccy_pair: Option<ExchangeRateBasis1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max52Text>,
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
pub struct ExchangeRateBasis1Choice {
    #[serde(flatten)]
    pub value: ExchangeRateBasis1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeConfirmationType1Code {
    #[serde(rename = "ECNF")]
    Ecnf,
    #[serde(rename = "YCNF")]
    Ycnf,
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
pub struct FertilizerCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
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
pub struct AgriculturalCommodityPotato2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType45Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType14Code {
    #[serde(rename = "OEST")]
    Oest,
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
pub struct DerivativesTradeReconciliationStatisticalReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RcncltnSttstcs")]
    pub rcncltn_sttstcs: StatisticsPerCounterparty15Choice,
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
pub struct ReconciliationCategory2 {
    #[validate]
    #[serde(rename = "Rvvd")]
    pub rvvd: YesNoIndicator,
    #[validate]
    #[serde(rename = "FrthrMod")]
    pub frthr_mod: YesNoIndicator,
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
pub struct CompareEnergyQuantityUnit1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<EnergyQuantityUnit2Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<EnergyQuantityUnit2Choice>,
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
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd19DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
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
pub struct AgriculturalCommodityDairy2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType20Code>,
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
pub struct CompareText2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max52Text>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max52Text>,
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
pub struct CompareTradeConfirmation2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<TradeConfirmation3Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<TradeConfirmation3Choice>,
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
pub struct UniqueTransactionIdentifier1ChoiceEnum {
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<UtiIdentifier>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification179>,
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
pub struct UniqueTransactionIdentifier1Choice {
    #[serde(flatten)]
    pub value: UniqueTransactionIdentifier1ChoiceEnum,
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
pub struct CompareBenchmarkCode1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ExternalBenchmarkCurveName1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ExternalBenchmarkCurveName1Code>,
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
pub struct CompareSeniorityType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DebtInstrumentSeniorityType2Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DebtInstrumentSeniorityType2Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionType2Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
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
pub struct CompareNotionalAmount1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<NotionalAmount6>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<NotionalAmount6>,
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
pub struct CompareAssetClass1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ProductType4Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ProductType4Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReconciliationStatus2Code {
    #[serde(rename = "NREC")]
    Nrec,
    #[serde(rename = "RECO")]
    Reco,
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
pub struct Max1000Text {
    #[validate(length(min = 1, max = 1000,))]
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
pub struct AgriculturalCommodityLiveStock2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType22Code>,
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
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
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
pub struct CompareText1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max52Text>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max52Text>,
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
pub struct UniqueProductIdentifier2ChoiceEnum {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max52Text>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification185>,
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
pub struct UniqueProductIdentifier2Choice {
    #[serde(flatten)]
    pub value: UniqueProductIdentifier2ChoiceEnum,
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
pub struct MetalCommodityPrecious2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType16Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType11Code>,
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
pub struct ClearingExceptionOrExemption2 {
    #[validate]
    #[serde(rename = "RptgCtrPty")]
    pub rptg_ctr_pty: NonClearingReason2,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<NonClearingReason2>,
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
pub struct FertilizerCommodityPotash2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType41Code>,
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
pub struct ActiveCurrencyCode {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
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
pub struct TradeTransactionIdentification24 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max140Text>,
    #[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
    pub actn_tp: Option<TransactionOperationType10Code>,
    #[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
    pub rptg_tm_stmp: Option<IsoDateTime>,
    #[serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none")]
    pub deriv_evt_tp: Option<DerivativeEventType3Code>,
    #[serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none")]
    pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<PartyIdentification248Choice>,
    #[serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none")]
    pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement8>,
    #[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
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
pub struct TrancheIndicator3ChoiceEnum {
    #[serde(rename = "Trnchd", skip_serializing_if = "Option::is_none")]
    pub trnchd: Option<Tranche3>,
    #[serde(rename = "Utrnchd", skip_serializing_if = "Option::is_none")]
    pub utrnchd: Option<NoReasonCode>,
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
pub struct TrancheIndicator3Choice {
    #[serde(flatten)]
    pub value: TrancheIndicator3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType21Code {
    #[serde(rename = "FRST")]
    Frst,
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
pub struct IndustrialProductCommodityManufacturing2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType34Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReconciliationStatus1Code {
    #[serde(rename = "NREC")]
    Nrec,
    #[serde(rename = "RECO")]
    Reco,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType15Code {
    #[serde(rename = "NPRM")]
    Nprm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType41Code {
    #[serde(rename = "PTSH")]
    Ptsh,
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
pub struct ClearingPartyAndTime22 {
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_rct_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none")]
    pub clr_idr: Option<UniqueTransactionIdentifier2Choice>,
    #[serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none")]
    pub orgnl_idr: Option<UniqueTransactionIdentifier2Choice>,
    #[serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none")]
    pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrAcctOrgn", skip_serializing_if = "Option::is_none")]
    pub clr_acct_orgn: Option<ClearingAccountType4Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType24Code {
    #[serde(rename = "COAL")]
    Coal,
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
pub struct CompareUnitPrice8 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesTransactionPrice13Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesTransactionPrice13Choice>,
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
pub struct CompareDurationType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DurationType1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DurationType1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RiskReductionService1Code {
    #[serde(rename = "NORR")]
    Norr,
    #[serde(rename = "PWOS")]
    Pwos,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PRBM")]
    Prbm,
    #[serde(rename = "PWAS")]
    Pwas,
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
pub struct AssetClassCommodityIndex1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType16Code,
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
pub struct FreightCommodityDry3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType31Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType33Code>,
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
pub struct StatisticsPerCounterparty15ChoiceEnum {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<ReconciliationStatisticsPerCounterparty3>,
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
pub struct StatisticsPerCounterparty15Choice {
    #[serde(flatten)]
    pub value: StatisticsPerCounterparty15ChoiceEnum,
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
> {
    #[validate]
    #[serde(rename = "DerivsTradRcncltnSttstclRpt")]
    pub derivs_trad_rcncltn_sttstcl_rpt: DerivativesTradeReconciliationStatisticalReportV02<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
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
pub struct CompareOrganisationIdentification6 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<OrganisationIdentification15Choice>,
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
pub struct EnergyCommodityRenewableEnergy2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType28Code>,
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
pub struct GenericIdentification179 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max52Text,
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
pub struct ClearingPartyAndTime22ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<NoReasonCode>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<ClearingPartyAndTime23>,
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
pub struct ClearingPartyAndTime22Choice {
    #[serde(flatten)]
    pub value: ClearingPartyAndTime22ChoiceEnum,
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
pub struct UniqueTransactionIdentifier2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<UtiIdentifier>,
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
pub struct UniqueTransactionIdentifier2Choice {
    #[serde(flatten)]
    pub value: UniqueTransactionIdentifier2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty1Code {
    #[serde(rename = "SLLR")]
    Sllr,
    #[serde(rename = "BYER")]
    Byer,
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
pub struct CompareNumber5 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max3Number>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max3Number>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType8Code {
    #[serde(rename = "PAPR")]
    Papr,
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
pub struct ExchangeRateBasis1 {
    #[serde(rename = "BaseCcy")]
    pub base_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
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
pub struct BasketConstituents3 {
    #[serde(rename = "InstrmId")]
    pub instrm_id: InstrumentIdentification6Choice,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure8Choice>,
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
pub struct DatePeriod4 {
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<IsoDate>,
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
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
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
pub struct AgriculturalCommoditySoft2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType2Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType2Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType34Code {
    #[serde(rename = "MFTG")]
    Mftg,
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
pub struct PortfolioCode3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max52Text>,
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
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
pub struct PortfolioCode3Choice {
    #[serde(flatten)]
    pub value: PortfolioCode3ChoiceEnum,
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
pub struct FreightCommodityWet3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType32Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType34Code>,
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
pub struct AssetClassCommodityC10Other1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType11Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType13Code {
    #[serde(rename = "MCEX")]
    Mcex,
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
pub struct CustomBasket4 {
    #[serde(rename = "Strr", skip_serializing_if = "Option::is_none")]
    pub strr: Option<LeiIdentifier>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max52Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cnsttnts", default)]
    pub cnsttnts: Vec<BasketConstituents3>,
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
    #[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
    pub ntrl: Option<NaturalPersonIdentification2>,
    #[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
    pub lgl: Option<OrganisationIdentification15Choice>,
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
pub struct ReconciliationCategory3 {
    #[serde(rename = "RptgTp")]
    pub rptg_tp: TradeRepositoryReportingType1Code,
    #[serde(rename = "Pairg")]
    pub pairg: PairingStatus1Code,
    #[serde(rename = "Rcncltn")]
    pub rcncltn: ReconciliationStatus1Code,
    #[serde(rename = "ValtnRcncltn")]
    pub valtn_rcncltn: ReconciliationStatus2Code,
    #[validate]
    #[serde(rename = "Rvvd")]
    pub rvvd: YesNoIndicator,
    #[validate]
    #[serde(rename = "FrthrMod")]
    pub frthr_mod: YesNoIndicator,
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
pub struct EnergyQuantityUnit2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max52Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EnergyQuantityUnit2Code>,
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
pub struct EnergyQuantityUnit2Choice {
    #[serde(flatten)]
    pub value: EnergyQuantityUnit2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType37Code {
    #[serde(rename = "PULP")]
    Pulp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeConfirmationType2Code {
    #[serde(rename = "NCNF")]
    Ncnf,
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
pub struct CompareUniqueTransactionIdentifier2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<UniqueTransactionIdentifier2Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<UniqueTransactionIdentifier2Choice>,
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
pub struct Max210Text {
    #[validate(length(min = 1, max = 210,))]
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
pub struct AssetClassCommodityFreight4ChoiceEnum {
    #[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
    pub dry: Option<FreightCommodityDry3>,
    #[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
    pub cntnr_ship: Option<FreightCommodityContainerShip2>,
    #[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
    pub wet: Option<FreightCommodityWet3>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FreightCommodityOther2>,
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
pub struct AssetClassCommodityFreight4Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFreight4ChoiceEnum,
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
pub struct CompareFinancialInstrumentContractType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<FinancialInstrumentContractType2Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<FinancialInstrumentContractType2Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod4Code {
    #[serde(rename = "A004")]
    A004,
    #[serde(rename = "A019")]
    A019,
    #[serde(rename = "A017")]
    A017,
    #[serde(rename = "A005")]
    A005,
    #[serde(rename = "A009")]
    A009,
    #[serde(rename = "A014")]
    A014,
    #[serde(rename = "A010")]
    A010,
    #[serde(rename = "A006")]
    A006,
    #[serde(rename = "A008")]
    A008,
    #[serde(rename = "A015")]
    A015,
    #[serde(rename = "A018")]
    A018,
    #[serde(rename = "A011")]
    A011,
    #[serde(rename = "A001")]
    A001,
    #[serde(rename = "A002")]
    A002,
    #[serde(rename = "A003")]
    A003,
    #[serde(rename = "A012")]
    A012,
    #[serde(rename = "A013")]
    A013,
    #[serde(rename = "A007")]
    A007,
    #[serde(rename = "A016")]
    A016,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "A020")]
    A020,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType10Code {
    #[serde(rename = "EMIS")]
    Emis,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
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
pub enum AssetClassProductType16Code {
    #[serde(rename = "INDX")]
    Indx,
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
pub struct CompareDayCount1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<InterestComputationMethodFormat7>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<InterestComputationMethodFormat7>,
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
pub struct CompareTradeClearingObligation1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ClearingObligationType1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ClearingObligationType1Code>,
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
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType2Code {
    #[serde(rename = "NRGY")]
    Nrgy,
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
pub struct AgriculturalCommodityForestry2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType21Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType5Code {
    #[serde(rename = "GRIN")]
    Grin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WeekDay3Code {
    #[serde(rename = "ALLD")]
    Alld,
    #[serde(rename = "XBHL")]
    Xbhl,
    #[serde(rename = "IBHL")]
    Ibhl,
    #[serde(rename = "FRID")]
    Frid,
    #[serde(rename = "MOND")]
    Mond,
    #[serde(rename = "SATD")]
    Satd,
    #[serde(rename = "SUND")]
    Sund,
    #[serde(rename = "THUD")]
    Thud,
    #[serde(rename = "TUED")]
    Tued,
    #[serde(rename = "WEDD")]
    Wedd,
    #[serde(rename = "WDAY")]
    Wday,
    #[serde(rename = "WEND")]
    Wend,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductType4Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "COMM")]
    Comm,
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
pub struct PaperCommodityContainerBoard2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType35Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingIdentification1Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "BSKT")]
    Bskt,
    #[serde(rename = "INDX")]
    Indx,
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
pub struct EnergyCommodityOil3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType8Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType32Code>,
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
pub struct CompareUniqueProductIdentifier2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<UniqueProductIdentifier2Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<UniqueProductIdentifier2Choice>,
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
pub struct AssetClassCommodityFertilizer4ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FertilizerCommodityOther2>,
    #[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
    pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
    #[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
    pub ptsh: Option<FertilizerCommodityPotash2>,
    #[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
    pub slphr: Option<FertilizerCommoditySulphur2>,
    #[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
    pub urea: Option<FertilizerCommodityUrea2>,
    #[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
    pub ammn: Option<FertilizerCommodityAmmonia2>,
    #[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
    pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
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
pub struct AssetClassCommodityFertilizer4Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFertilizer4ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EnergyLoadType1Code {
    #[serde(rename = "BSLD")]
    Bsld,
    #[serde(rename = "GASD")]
    Gasd,
    #[serde(rename = "HABH")]
    Habh,
    #[serde(rename = "OFFP")]
    Offp,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PKLD")]
    Pkld,
    #[serde(rename = "SHPD")]
    Shpd,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType12Code {
    #[serde(rename = "INFL")]
    Infl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType30Code {
    #[serde(rename = "WTHR")]
    Wthr,
    #[default]
    Unknown,
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
pub struct CompareTrancheIndicator1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<TrancheIndicator3Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<TrancheIndicator3Choice>,
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
pub struct TimePeriod3 {
    #[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
    pub fr_tm: Option<IsoTime>,
    #[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
    pub to_tm: Option<IsoTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DurationType1Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "SEAS")]
    Seas,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "MNUT")]
    Mnut,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "DASD")]
    Dasd,
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
pub struct InterestComputationMethodFormat7 {
    #[serde(rename = "Cd")]
    pub cd: InterestComputationMethod4Code,
    #[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
    pub nrrtv: Option<Max1000Text>,
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
pub struct GenericIdentification184 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max210Text,
    #[validate]
    #[serde(rename = "Src")]
    pub src: Max100Text,
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
pub struct FertilizerCommodityDiammoniumPhosphate2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType40Code>,
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
pub struct ClearingPartyAndTime23 {
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_rct_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none")]
    pub clr_idr: Option<UniqueTransactionIdentifier1Choice>,
    #[serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none")]
    pub orgnl_idr: Option<UniqueTransactionIdentifier1Choice>,
    #[serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none")]
    pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
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
pub struct AgriculturalCommodityOliveOil3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType3Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType29Code>,
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
pub struct PortfolioIdentification3 {
    #[validate]
    #[serde(rename = "Cd")]
    pub cd: Max52Text,
    #[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
    pub prtfl_tx_xmptn: Option<TrueFalseIndicator>,
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
pub struct MetalCommodityNonPrecious2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType15Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType10Code>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType31Code {
    #[serde(rename = "DRYF")]
    Dryf,
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
pub struct ClearingPartyAndTime21ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<NoReasonCode>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<ClearingPartyAndTime22>,
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
pub struct ClearingPartyAndTime21Choice {
    #[serde(flatten)]
    pub value: ClearingPartyAndTime21ChoiceEnum,
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
pub struct AgriculturalCommodityGrain3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType5Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType30Code>,
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
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType44Code>,
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
pub struct CompareLegDirection2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Direction4Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Direction4Choice>,
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
pub struct CompareLongFraction19DecimalNumber1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<LongFraction19DecimalNumber>,
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
pub struct CompareMasterAgreementType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AgreementType2Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AgreementType2Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PairingStatus1Code {
    #[serde(rename = "PARD")]
    Pard,
    #[serde(rename = "UNPR")]
    Unpr,
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
pub struct CompareDeliveryInterconnectionPoint1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DeliveryInterconnectionPoint1Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DeliveryInterconnectionPoint1Choice>,
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
pub struct Direction2 {
    #[serde(rename = "DrctnOfTheFrstLeg")]
    pub drctn_of_the_frst_leg: OptionParty3Code,
    #[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
    pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
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
pub struct CompareValuationType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ValuationType1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ValuationType1Code>,
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
pub struct ExternalUnitOfMeasure1Code {
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
pub struct ContractMatchingCriteria3 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<CompareIsinIdentifier2>,
    #[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
    pub unq_pdct_idr: Option<CompareUniqueProductIdentifier2>,
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<CompareText1>,
    #[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
    pub pdct_clssfctn: Option<CompareCfiIdentifier3>,
    #[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
    pub ctrct_tp: Option<CompareFinancialInstrumentContractType1>,
    #[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
    pub asst_clss: Option<CompareAssetClass1>,
    #[serde(
        rename = "DerivBasedOnCrptAsst",
        skip_serializing_if = "Option::is_none"
    )]
    pub deriv_based_on_crpt_asst: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
    pub undrlyg_instrm: Option<CompareUnderlyingInstrument3>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<CompareActiveOrHistoricCurrencyCode1>,
    #[serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy_scnd_leg: Option<CompareActiveOrHistoricCurrencyCode1>,
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
pub struct PaperCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType49Code>,
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
pub struct DerivativeEvent6 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DerivativeEventType3Code>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<EventIdentifier1Choice>,
    #[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
    pub tm_stmp: Option<DateAndDateTime2Choice>,
    #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<TrueFalseIndicator>,
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
pub struct NotionalAmount6 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountAndDirection106>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SchdlPrd", default)]
    pub schdl_prd: Vec<Schedule11>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
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
pub struct AssetClassCommodityAgricultural6ChoiceEnum {
    #[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
    pub frstry: Option<AgriculturalCommodityForestry2>,
    #[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
    pub dairy: Option<AgriculturalCommodityDairy2>,
    #[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
    pub sfd: Option<AgriculturalCommoditySeafood2>,
    #[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
    pub live_stock: Option<AgriculturalCommodityLiveStock2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AgriculturalCommodityOther2>,
    #[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
    pub grn: Option<AgriculturalCommodityGrain3>,
    #[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
    pub ptt: Option<AgriculturalCommodityPotato2>,
    #[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
    pub soft: Option<AgriculturalCommoditySoft2>,
    #[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
    pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
    #[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
    pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
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
pub struct AssetClassCommodityAgricultural6Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityAgricultural6ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType26Code {
    #[serde(rename = "INRG")]
    Inrg,
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
pub struct BaseOne18Rate {
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
pub struct CompareOtherPayment1 {
    #[serde(rename = "OthrPmtTp", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_tp: Option<CompareOtherPaymentType1>,
    #[serde(rename = "OthrPmtAmt", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_amt: Option<CompareAmountAndDirection3>,
    #[serde(rename = "OthrPmtDt", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_dt: Option<CompareDate3>,
    #[serde(rename = "OthrPmtPyer", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_pyer: Option<CompareOrganisationIdentification7>,
    #[serde(rename = "OthrPmtRcvr", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_rcvr: Option<CompareOrganisationIdentification7>,
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
pub struct FertilizerCommodityUrea2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType43Code>,
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
pub struct CompareWeekDay1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<WeekDay3Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<WeekDay3Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingExemptionException1Code {
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "ENDU")]
    Endu,
    #[serde(rename = "AFFL")]
    Affl,
    #[serde(rename = "NOAL")]
    Noal,
    #[serde(rename = "NORE")]
    Nore,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SMBK")]
    Smbk,
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
pub struct PaperCommodityPulp2 {
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
pub struct PostTradeRiskReductionIdentifier1 {
    #[validate]
    #[serde(rename = "Strr")]
    pub strr: LeiIdentifier,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max52Text,
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
pub struct CompareExchangeRateBasis1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ExchangeRateBasis1Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ExchangeRateBasis1Choice>,
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
pub struct ReconciliationCounterpartyPairStatistics6 {
    #[validate]
    #[serde(rename = "CtrPtyId")]
    pub ctr_pty_id: CounterpartyData91,
    #[validate]
    #[serde(rename = "TtlNbOfTxs")]
    pub ttl_nb_of_txs: Number,
    #[validate(length(min = 1,))]
    #[serde(rename = "RcncltnRpt", default)]
    pub rcncltn_rpt: Vec<ReconciliationReport14>,
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
pub struct FertilizerCommodityAmmonia2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType39Code>,
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
pub struct UnitOfMeasure8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUnitOfMeasure1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
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
pub struct UnitOfMeasure8Choice {
    #[serde(flatten)]
    pub value: UnitOfMeasure8ChoiceEnum,
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
pub struct UniqueProductIdentifier1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max52Text>,
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
pub struct UniqueProductIdentifier1Choice {
    #[serde(flatten)]
    pub value: UniqueProductIdentifier1ChoiceEnum,
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
pub struct AgriculturalCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
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
pub struct NaturalPersonIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: NaturalPersonIdentification2,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
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
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType14Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType39Code {
    #[serde(rename = "AMMO")]
    Ammo,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType4Code {
    #[serde(rename = "FRGT")]
    Frgt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EnergyQuantityUnit2Code {
    #[serde(rename = "BTUD")]
    Btud,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "GJDD")]
    Gjdd,
    #[serde(rename = "GWAT")]
    Gwat,
    #[serde(rename = "GWHD")]
    Gwhd,
    #[serde(rename = "GWHH")]
    Gwhh,
    #[serde(rename = "HMJD")]
    Hmjd,
    #[serde(rename = "KTMD")]
    Ktmd,
    #[serde(rename = "KWAT")]
    Kwat,
    #[serde(rename = "KWHD")]
    Kwhd,
    #[serde(rename = "KWHH")]
    Kwhh,
    #[serde(rename = "MCMD")]
    Mcmd,
    #[serde(rename = "MJDD")]
    Mjdd,
    #[serde(rename = "MBTD")]
    Mbtd,
    #[serde(rename = "MMJD")]
    Mmjd,
    #[serde(rename = "MTMD")]
    Mtmd,
    #[serde(rename = "MWAT")]
    Mwat,
    #[serde(rename = "MWHD")]
    Mwhd,
    #[serde(rename = "MWHH")]
    Mwhh,
    #[serde(rename = "THMD")]
    Thmd,
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
pub struct IndustrialProductCommodityConstruction2 {
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
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty3Code {
    #[serde(rename = "MAKE")]
    Make,
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
pub struct PartyIdentification248ChoiceEnum {
    #[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
    pub ntrl: Option<NaturalPersonIdentification3>,
    #[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
    pub lgl: Option<LegalPersonIdentification1>,
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
pub struct PartyIdentification248Choice {
    #[serde(flatten)]
    pub value: PartyIdentification248ChoiceEnum,
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
pub struct ReconciliationReport14 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TradeTransactionIdentification24,
    #[validate]
    #[serde(rename = "MtchgCrit")]
    pub mtchg_crit: MatchingCriteria16,
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
pub struct Cleared23ChoiceEnum {
    #[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
    pub clrd: Option<ClearingPartyAndTime21Choice>,
    #[serde(rename = "IntndToClear", skip_serializing_if = "Option::is_none")]
    pub intnd_to_clear: Option<ClearingPartyAndTime22Choice>,
    #[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
    pub non_clrd: Option<ClearingExceptionOrExemption3Choice>,
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
pub struct Cleared23Choice {
    #[serde(flatten)]
    pub value: Cleared23ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType27Code {
    #[serde(rename = "LGHT")]
    Lght,
    #[default]
    Unknown,
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
pub struct EnergyCommodityNaturalGas3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType7Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType31Code>,
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
pub struct Schedule11 {
    #[validate]
    #[serde(rename = "UadjstdFctvDt")]
    pub uadjstd_fctv_dt: IsoDate,
    #[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
    pub uadjstd_end_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: AmountAndDirection106,
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
pub struct CompareDatePeriod2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DatePeriod4>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DatePeriod4>,
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
pub struct TradeConfirmation4 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TradeConfirmationType1Code>,
    #[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
    pub tm_stmp: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ValuationType1Code {
    #[serde(rename = "CCPV")]
    Ccpv,
    #[serde(rename = "MTMA")]
    Mtma,
    #[serde(rename = "MTMO")]
    Mtmo,
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
pub struct CompareMax350Text1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max350Text>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency13Code {
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "QURT")]
    Qurt,
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
pub struct EnvironmentCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
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
pub struct OrganisationIdentification38 {
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
pub struct EnergyCommodityLightEnd2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType27Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentType4Code {
    #[serde(rename = "UFRO")]
    Ufro,
    #[serde(rename = "UWIN")]
    Uwin,
    #[serde(rename = "PEXH")]
    Pexh,
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
pub struct AssetClassCommodityPaper4ChoiceEnum {
    #[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
    pub nwsprnt: Option<PaperCommodityNewsprint2>,
    #[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
    pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
    #[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
    pub rcvrd_ppr: Option<PaperCommodityOther1>,
    #[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
    pub pulp: Option<PaperCommodityPulp2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PaperCommodityOther1>,
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
pub struct AssetClassCommodityPaper4Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPaper4ChoiceEnum,
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
pub struct CompareExchangeRate1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<BaseOne18Rate>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<BaseOne18Rate>,
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
pub struct FertilizerCommoditySulphur2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType42Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotApplicable1Code {
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
pub struct CompareDerivativeEvent1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DerivativeEvent6>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DerivativeEvent6>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PhysicalTransferType4Code {
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "OPTL")]
    Optl,
    #[serde(rename = "CASH")]
    Cash,
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
pub struct PolypropyleneCommodityPlastic2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType9Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType18Code>,
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
pub struct AmountAndDirection106 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
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
pub struct CompareActiveOrHistoricCurrencyCode1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ActiveOrHistoricCurrencyCode>,
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
pub struct EicIdentifier {
    #[validate(regex = "EIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType18Code {
    #[serde(rename = "PLST")]
    Plst,
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
pub struct AssetClassCommodityEnvironmental3ChoiceEnum {
    #[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
    pub wthr: Option<EnvironmentalCommodityWeather2>,
    #[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
    pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnvironmentCommodityOther2>,
    #[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
    pub emssns: Option<EnvironmentalCommodityEmission3>,
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
pub struct AssetClassCommodityEnvironmental3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnvironmental3ChoiceEnum,
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
pub struct CompareOptionStyle1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<OptionStyle6Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<OptionStyle6Code>,
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
pub struct MatchingCriteria16 {
    #[serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria6>,
    #[serde(rename = "ValtnMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub valtn_mtchg_crit: Option<ValuationMatchingCriteria1>,
    #[serde(rename = "CtrctMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub ctrct_mtchg_crit: Option<ContractMatchingCriteria3>,
    #[serde(rename = "TxMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub tx_mtchg_crit: Option<TransactionMatchingCriteria6>,
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
pub struct MarginPortfolio3 {
    #[serde(rename = "InitlMrgnPrtflCd")]
    pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
    #[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType42Code {
    #[serde(rename = "SLPH")]
    Slph,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType6Code {
    #[serde(rename = "ELEC")]
    Elec,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType43Code {
    #[serde(rename = "UREA")]
    Urea,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingObligationType1Code {
    #[serde(rename = "FLSE")]
    Flse,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "TRUE")]
    True,
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
pub struct CompareUnderlyingInstrument3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecurityIdentification41Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecurityIdentification41Choice>,
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
pub struct PaymentType5ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PaymentType4Code>,
    #[serde(rename = "PrtryTp", skip_serializing_if = "Option::is_none")]
    pub prtry_tp: Option<Max4AlphaNumericText>,
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
pub struct PaymentType5Choice {
    #[serde(flatten)]
    pub value: PaymentType5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType15Code {
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct TradeConfirmation3ChoiceEnum {
    #[serde(rename = "Confd", skip_serializing_if = "Option::is_none")]
    pub confd: Option<TradeConfirmation4>,
    #[serde(rename = "NonConfd", skip_serializing_if = "Option::is_none")]
    pub non_confd: Option<TradeNonConfirmation1>,
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
pub struct TradeConfirmation3Choice {
    #[serde(flatten)]
    pub value: TradeConfirmation3ChoiceEnum,
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
pub struct AgriculturalCommoditySeafood2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType23Code>,
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
pub struct CompareDeliveryType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PhysicalTransferType4Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PhysicalTransferType4Code>,
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
pub struct Tranche3 {
    #[serde(rename = "AttchmntPt", skip_serializing_if = "Option::is_none")]
    pub attchmnt_pt: Option<BaseOneRate>,
    #[serde(rename = "DtchmntPt", skip_serializing_if = "Option::is_none")]
    pub dtchmnt_pt: Option<BaseOneRate>,
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
pub struct CompareDate3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<IsoDate>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<IsoDate>,
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
pub struct InstrumentIdentification6ChoiceEnum {
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<GenericIdentification184>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<Max52Text>,
    #[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
    pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
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
pub struct InstrumentIdentification6Choice {
    #[serde(flatten)]
    pub value: InstrumentIdentification6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType1Code {
    #[serde(rename = "AGRI")]
    Agri,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType40Code {
    #[serde(rename = "DAPH")]
    Daph,
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
pub struct CompareEnergyDeliveryAttribute1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "NrgyDlvryIntrvl", default)]
    pub nrgy_dlvry_intrvl: Vec<CompareTimePeriod2>,
    #[serde(rename = "NrgyDt", skip_serializing_if = "Option::is_none")]
    pub nrgy_dt: Option<CompareDatePeriod2>,
    #[serde(rename = "NrgyDrtn", skip_serializing_if = "Option::is_none")]
    pub nrgy_drtn: Option<CompareDurationType1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrgyWkDay", default)]
    pub nrgy_wk_day: Vec<CompareWeekDay1>,
    #[serde(rename = "NrgyDlvryCpcty", skip_serializing_if = "Option::is_none")]
    pub nrgy_dlvry_cpcty: Option<CompareLongFraction19DecimalNumber1>,
    #[serde(rename = "NrgyQtyUnit", skip_serializing_if = "Option::is_none")]
    pub nrgy_qty_unit: Option<CompareEnergyQuantityUnit1>,
    #[serde(
        rename = "NrgyPricTmIntrvlQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub nrgy_pric_tm_intrvl_qty: Option<CompareAmountAndDirection3>,
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
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
pub struct AgriculturalCommodityOilSeed2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType1Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType1Code>,
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
pub struct TransactionMatchingCriteria6 {
    #[serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none")]
    pub rpt_trckg_nb: Option<CompareText2>,
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
    #[serde(rename = "PrrUnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub prr_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
    #[serde(rename = "SbsqntPosUnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub sbsqnt_pos_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
    #[serde(rename = "Dlta", skip_serializing_if = "Option::is_none")]
    pub dlta: Option<CompareLongFraction19DecimalNumber1>,
    #[serde(rename = "TradConf", skip_serializing_if = "Option::is_none")]
    pub trad_conf: Option<CompareTradeConfirmation2>,
    #[serde(rename = "TradClrOblgtn", skip_serializing_if = "Option::is_none")]
    pub trad_clr_oblgtn: Option<CompareTradeClearingObligation1>,
    #[serde(rename = "TradClrSts", skip_serializing_if = "Option::is_none")]
    pub trad_clr_sts: Option<CompareTradeClearingStatus3>,
    #[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt_tp: Option<CompareMasterAgreementType1>,
    #[serde(rename = "MstrAgrmtVrsn", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt_vrsn: Option<CompareMax50Text1>,
    #[serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none")]
    pub intra_grp: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "PstTradRskRdctn", skip_serializing_if = "Option::is_none")]
    pub pst_trad_rsk_rdctn: Option<ComparePostTradeRiskReduction2>,
    #[serde(rename = "DerivEvt", skip_serializing_if = "Option::is_none")]
    pub deriv_evt: Option<CompareDerivativeEvent1>,
    #[serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none")]
    pub pltfm_idr: Option<CompareMicIdentifier3>,
    #[serde(rename = "ExctnTmStmp", skip_serializing_if = "Option::is_none")]
    pub exctn_tm_stmp: Option<CompareDateTime3>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<CompareDate3>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<CompareDate3>,
    #[serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none")]
    pub early_termntn_dt: Option<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmDt", default)]
    pub sttlm_dt: Vec<CompareDate3>,
    #[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
    pub dlvry_tp: Option<CompareDeliveryType1>,
    #[serde(rename = "TxPric", skip_serializing_if = "Option::is_none")]
    pub tx_pric: Option<CompareUnitPrice5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PricSchdlUadjstdFctvDt", default)]
    pub pric_schdl_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PricSchdlUadjstdEndDt", default)]
    pub pric_schdl_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxSchdlPric", default)]
    pub tx_schdl_pric: Vec<CompareUnitPrice5>,
    #[serde(rename = "PackgPric", skip_serializing_if = "Option::is_none")]
    pub packg_pric: Option<CompareUnitPrice5>,
    #[serde(rename = "NtnlAmtFrstLeg", skip_serializing_if = "Option::is_none")]
    pub ntnl_amt_frst_leg: Option<CompareAmountAndDirection3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtFrstLegUadjstdFctvDt", default)]
    pub ntnl_amt_frst_leg_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtFrstLegUadjstdEndDt", default)]
    pub ntnl_amt_frst_leg_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtFrstLegSchdlAmt", default)]
    pub ntnl_amt_frst_leg_schdl_amt: Vec<CompareAmountAndDirection3>,
    #[serde(rename = "NtnlQtyFrstLeg", skip_serializing_if = "Option::is_none")]
    pub ntnl_qty_frst_leg: Option<CompareLongFraction19DecimalNumber1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyFrstLegUadjstdFctvDt", default)]
    pub ntnl_qty_frst_leg_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyFrstLegUadjstdEndDt", default)]
    pub ntnl_qty_frst_leg_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyFrstLegSchdlQty", default)]
    pub ntnl_qty_frst_leg_schdl_qty: Vec<CompareLongFraction19DecimalNumber1>,
    #[serde(rename = "NtnlAmtScndLeg", skip_serializing_if = "Option::is_none")]
    pub ntnl_amt_scnd_leg: Option<CompareNotionalAmount1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtScndLegUadjstdFctvDt", default)]
    pub ntnl_amt_scnd_leg_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtScndLegUadjstdEndDt", default)]
    pub ntnl_amt_scnd_leg_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlAmtScndLegSchdlAmt", default)]
    pub ntnl_amt_scnd_leg_schdl_amt: Vec<CompareAmountAndDirection3>,
    #[serde(rename = "NtnlQtyScndLeg", skip_serializing_if = "Option::is_none")]
    pub ntnl_qty_scnd_leg: Option<CompareLongFraction19DecimalNumber1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyScndLegUadjstdFctvDt", default)]
    pub ntnl_qty_scnd_leg_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyScndLegUadjstdEndDt", default)]
    pub ntnl_qty_scnd_leg_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtnlQtyScndLegSchdlQty", default)]
    pub ntnl_qty_scnd_leg_schdl_qty: Vec<CompareLongFraction19DecimalNumber1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrPmt", default)]
    pub othr_pmt: Vec<CompareOtherPayment1>,
    #[serde(
        rename = "IntrstFxdRateFrstLeg",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_frst_leg: Option<CompareUnitPrice7>,
    #[serde(
        rename = "IntrstFxdRateFrstLegDayCnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
    #[serde(
        rename = "IntrstFxdRateFrstLegPmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFxdRateFrstLegPmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateFrstLegId",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_id: Option<CompareIsinIdentifier4>,
    #[serde(
        rename = "IntrstFltgRateFrstLegCd",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_cd: Option<CompareBenchmarkCode1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegNm",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_nm: Option<CompareMax350Text1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegDayCnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegPmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegPmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateFrstLegRefPrdUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegRefPrdVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_ref_prd_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateFrstLegRstFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateFrstLegRstFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_rst_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateFrstLegSprd",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_frst_leg_sprd: Option<CompareUnitPrice8>,
    #[serde(
        rename = "IntrstRateFxdScndLeg",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_rate_fxd_scnd_leg: Option<CompareUnitPrice7>,
    #[serde(
        rename = "IntrstFxdRateScndLegDayCnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
    #[serde(
        rename = "IntrstFxdRateScndLegPmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFxdRateScndLegPmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fxd_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateScndLegId",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_id: Option<CompareIsinIdentifier4>,
    #[serde(
        rename = "IntrstFltgRateScndLegCd",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_cd: Option<CompareBenchmarkCode1>,
    #[serde(
        rename = "IntrstFltgRateScndLegNm",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_nm: Option<CompareMax350Text1>,
    #[serde(
        rename = "IntrstFltgRateScndLegDayCnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
    #[serde(
        rename = "IntrstFltgRateScndLegPmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateScndLegPmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateScndLegRefPrdUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateScndLegRefPrdVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_ref_prd_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateScndLegRstFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
    #[serde(
        rename = "IntrstFltgRateScndLegRstFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_rst_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "IntrstFltgRateScndLegSprd",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrst_fltg_rate_scnd_leg_sprd: Option<CompareUnitPrice8>,
    #[serde(rename = "PackgSprd", skip_serializing_if = "Option::is_none")]
    pub packg_sprd: Option<CompareUnitPrice8>,
    #[serde(rename = "CcyXchgRate", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg_rate: Option<CompareExchangeRate1>,
    #[serde(rename = "CcyFwdXchgRate", skip_serializing_if = "Option::is_none")]
    pub ccy_fwd_xchg_rate: Option<CompareExchangeRate1>,
    #[serde(rename = "CcyXchgRateBsis", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg_rate_bsis: Option<CompareExchangeRateBasis1>,
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<CompareCommodityAssetClass4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrgyDlvryPtOrZone", default)]
    pub nrgy_dlvry_pt_or_zone: Vec<CompareDeliveryInterconnectionPoint1>,
    #[serde(rename = "NrgyIntrCnnctnPt", skip_serializing_if = "Option::is_none")]
    pub nrgy_intr_cnnctn_pt: Option<CompareDeliveryInterconnectionPoint1>,
    #[serde(rename = "NrgyLdTp", skip_serializing_if = "Option::is_none")]
    pub nrgy_ld_tp: Option<CompareEnergyLoadType1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DlvryAttr", default)]
    pub dlvry_attr: Vec<CompareEnergyDeliveryAttribute1>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<CompareOptionType1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnExrcStyle", default)]
    pub optn_exrc_style: Vec<CompareOptionStyle1>,
    #[serde(rename = "OptnStrkPric", skip_serializing_if = "Option::is_none")]
    pub optn_strk_pric: Option<CompareUnitPrice4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnStrkPricSchdlUadjstdFctvDt", default)]
    pub optn_strk_pric_schdl_uadjstd_fctv_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnStrkPricSchdlUadjstdEndDt", default)]
    pub optn_strk_pric_schdl_uadjstd_end_dt: Vec<CompareDate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnStrkPricSchdlAmt", default)]
    pub optn_strk_pric_schdl_amt: Vec<CompareUnitPrice4>,
    #[serde(rename = "OptnPrmAmt", skip_serializing_if = "Option::is_none")]
    pub optn_prm_amt: Option<CompareActiveOrHistoricCurrencyAndAmount4>,
    #[serde(rename = "OptnPrmPmtDt", skip_serializing_if = "Option::is_none")]
    pub optn_prm_pmt_dt: Option<CompareDate3>,
    #[serde(
        rename = "OptnMtrtyDtOfUndrlyg",
        skip_serializing_if = "Option::is_none"
    )]
    pub optn_mtrty_dt_of_undrlyg: Option<CompareDate3>,
    #[serde(rename = "CdtSnrty", skip_serializing_if = "Option::is_none")]
    pub cdt_snrty: Option<CompareSeniorityType1>,
    #[serde(rename = "CdtRefPty", skip_serializing_if = "Option::is_none")]
    pub cdt_ref_pty: Option<CompareReferenceParty1>,
    #[serde(rename = "CdtSrs", skip_serializing_if = "Option::is_none")]
    pub cdt_srs: Option<CompareNumber7>,
    #[serde(rename = "CdtVrsn", skip_serializing_if = "Option::is_none")]
    pub cdt_vrsn: Option<CompareNumber7>,
    #[serde(rename = "CdtIndxFctr", skip_serializing_if = "Option::is_none")]
    pub cdt_indx_fctr: Option<ComparePercentageRate3>,
    #[serde(rename = "CdtTrch", skip_serializing_if = "Option::is_none")]
    pub cdt_trch: Option<CompareTrancheIndicator1>,
    #[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
    pub lvl: Option<CompareReportingLevelType2>,
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
pub struct DerivativePartyIdentification1ChoiceEnum {
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
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
pub struct DerivativePartyIdentification1Choice {
    #[serde(flatten)]
    pub value: DerivativePartyIdentification1ChoiceEnum,
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
pub struct CompareOrganisationIdentification7 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PartyIdentification236Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PartyIdentification236Choice>,
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
pub struct YesNoIndicator {
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
pub struct LegalPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification15Choice,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingAccountType4Code {
    #[serde(rename = "CLIE")]
    Clie,
    #[serde(rename = "HOUS")]
    Hous,
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
pub struct CompareIsinIdentifier4 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<IsinOct2015Identifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<IsinOct2015Identifier>,
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
pub struct CompareTradeClearingStatus3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Cleared23Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Cleared23Choice>,
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
pub struct CounterpartyData91 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<PartyIdentification236Choice>,
    #[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
    pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
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
pub struct FreightCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
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
pub struct TradeNonConfirmation1 {
    #[serde(rename = "Tp")]
    pub tp: TradeConfirmationType2Code,
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
pub struct CompareOptionType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<OptionType2Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<OptionType2Code>,
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
pub struct SecurityIdentification41ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericIdentification184>,
    #[serde(rename = "IdNotAvlbl", skip_serializing_if = "Option::is_none")]
    pub id_not_avlbl: Option<UnderlyingIdentification1Code>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
    pub bskt: Option<CustomBasket4>,
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<Max52Text>,
    #[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
    pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<IndexIdentification1>,
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
pub struct SecurityIdentification41Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification41ChoiceEnum,
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
pub struct CompareCommodityAssetClass4 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AssetClassCommodity6Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AssetClassCommodity6Choice>,
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
pub struct AssetClassCommodityPolypropylene4ChoiceEnum {
    #[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
    pub plstc: Option<PolypropyleneCommodityPlastic2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PolypropyleneCommodityOther2>,
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
pub struct AssetClassCommodityPolypropylene4Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPolypropylene4ChoiceEnum,
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
pub struct CompareMicIdentifier3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<MicIdentifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<MicIdentifier>,
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
pub struct EnvironmentalCommodityEmission3 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType10Code>,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
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
pub struct ReconciliationStatisticsPerCounterparty3 {
    #[validate]
    #[serde(rename = "RefDt")]
    pub ref_dt: IsoDate,
    #[serde(rename = "RcncltnCtgrs")]
    pub rcncltn_ctgrs: ReportingRequirement2Choice,
    #[serde(rename = "TtlNbOfTxs", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_txs: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxDtls", default)]
    pub tx_dtls: Vec<ReconciliationCounterpartyPairStatistics6>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType7Code {
    #[serde(rename = "METL")]
    Metl,
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
pub struct FreightCommodityContainerShip2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType46Code>,
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
pub struct ComparePostTradeRiskReduction2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PtrrEvent3>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PtrrEvent3>,
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
pub struct SecuritiesTransactionPrice14ChoiceEnum {
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
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
pub struct SecuritiesTransactionPrice14Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice14ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType46Code {
    #[serde(rename = "CSHP")]
    Cshp,
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
pub struct PolypropyleneCommodityOther2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType9Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType49Code>,
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
pub struct SecuritiesTransactionPrice17ChoiceEnum {
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<SecuritiesTransactionPrice5>,
    #[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
    pub pdg_pric: Option<PriceStatus1Code>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection106>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<LongFraction19DecimalNumber>,
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
pub struct SecuritiesTransactionPrice17Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice17ChoiceEnum,
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
pub struct DateAndDateTime2ChoiceEnum {
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
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
pub struct DateAndDateTime2Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime2ChoiceEnum,
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
pub struct Max100Text {
    #[validate(length(min = 1, max = 100,))]
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
pub struct ReportingRequirement2ChoiceEnum {
    #[serde(rename = "NoRptgRqrmnt", skip_serializing_if = "Option::is_none")]
    pub no_rptg_rqrmnt: Option<ReconciliationCategory2>,
    #[serde(rename = "RptgRqrmnt", skip_serializing_if = "Option::is_none")]
    pub rptg_rqrmnt: Option<ReconciliationCategory3>,
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
pub struct ReportingRequirement2Choice {
    #[serde(flatten)]
    pub value: ReportingRequirement2ChoiceEnum,
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
pub struct CollateralPortfolioCode5ChoiceEnum {
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioCode3Choice>,
    #[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
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
pub struct CollateralPortfolioCode5Choice {
    #[serde(flatten)]
    pub value: CollateralPortfolioCode5ChoiceEnum,
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
pub struct EnergyCommodityCoal2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType24Code>,
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
pub struct GenericIdentification185 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max100Text,
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
pub struct ValuationMatchingCriteria1 {
    #[serde(rename = "CtrctVal", skip_serializing_if = "Option::is_none")]
    pub ctrct_val: Option<CompareAmountAndDirection3>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CompareValuationType1>,
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
pub struct CompareTimePeriod2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<TimePeriod3>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<TimePeriod3>,
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
pub struct CountrySubDivisionCode {
    #[validate(regex = "COUNTRY_SUB_DIVISION_CODE_REGEX")]
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
pub struct SecuritiesTransactionPrice13ChoiceEnum {
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection106>,
    #[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
    pub bsis_pt_sprd: Option<Number>,
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
pub struct SecuritiesTransactionPrice13Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice13ChoiceEnum,
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
pub struct Direction4ChoiceEnum {
    #[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_sd: Option<OptionParty1Code>,
    #[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
    pub drctn: Option<Direction2>,
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
pub struct Direction4Choice {
    #[serde(flatten)]
    pub value: Direction4ChoiceEnum,
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
pub struct AssetClassCommodityEnergy3ChoiceEnum {
    #[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
    pub oil: Option<EnergyCommodityOil3>,
    #[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
    pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
    #[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
    pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
    #[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
    pub lght_end: Option<EnergyCommodityLightEnd2>,
    #[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
    pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
    #[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
    pub dstllts: Option<EnergyCommodityDistillates2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnergyCommodityOther2>,
    #[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
    pub elctrcty: Option<EnergyCommodityElectricity2>,
    #[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
    pub coal: Option<EnergyCommodityCoal2>,
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
pub struct AssetClassCommodityEnergy3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnergy3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType10Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "CORR")]
    Corr,
    #[serde(rename = "EROR")]
    Eror,
    #[serde(rename = "MODI")]
    Modi,
    #[serde(rename = "NEWT")]
    Newt,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "POSC")]
    Posc,
    #[serde(rename = "REVI")]
    Revi,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "VALU")]
    Valu,
    #[serde(rename = "MARU")]
    Maru,
    #[serde(rename = "PRTO")]
    Prto,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType44Code {
    #[serde(rename = "UAAN")]
    Uaan,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialInstrumentContractType2Code {
    #[serde(rename = "CFDS")]
    Cfds,
    #[serde(rename = "FRAS")]
    Fras,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "SPDB")]
    Spdb,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "SWPT")]
    Swpt,
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
pub struct CompareEnergyLoadType1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<EnergyLoadType1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<EnergyLoadType1Code>,
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
pub struct EnvironmentalCommodityWeather2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType30Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle6Code {
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "BERM")]
    Berm,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "AMER")]
    Amer,
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
pub struct CompareTrueFalseIndicator3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<TrueFalseIndicator>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<TrueFalseIndicator>,
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
pub struct CompareIsinIdentifier2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<IsinOct2015Identifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<IsinOct2015Identifier>,
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
pub struct Number {
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
pub struct ExternalBenchmarkCurveName1Code {
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
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
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
pub struct EnergyCommodityInterEnergy2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType26Code>,
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
pub struct PtrrEvent3 {
    #[serde(rename = "Tchnq", skip_serializing_if = "Option::is_none")]
    pub tchnq: Option<RiskReductionService1Code>,
    #[serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none")]
    pub svc_prvdr: Option<OrganisationIdentification15Choice>,
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
pub struct CompareReferenceParty1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DerivativePartyIdentification1Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DerivativePartyIdentification1Choice>,
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
pub struct EventIdentifier1ChoiceEnum {
    #[serde(rename = "PstTradRskRdctnIdr", skip_serializing_if = "Option::is_none")]
    pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
    #[serde(rename = "EvtIdr", skip_serializing_if = "Option::is_none")]
    pub evt_idr: Option<UtiIdentifier>,
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
pub struct EventIdentifier1Choice {
    #[serde(flatten)]
    pub value: EventIdentifier1ChoiceEnum,
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
pub struct ComparePercentageRate3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PercentageRate>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PercentageRate>,
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
pub struct DeliveryInterconnectionPoint1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EicIdentifier>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max52Text>,
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
pub struct DeliveryInterconnectionPoint1Choice {
    #[serde(flatten)]
    pub value: DeliveryInterconnectionPoint1ChoiceEnum,
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
pub struct CompareMax50Text1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max50Text>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max50Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType32Code {
    #[serde(rename = "WETF")]
    Wetf,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType35Code {
    #[serde(rename = "CBRD")]
    Cbrd,
    #[default]
    Unknown,
}
