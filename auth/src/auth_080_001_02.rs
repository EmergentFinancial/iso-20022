// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.080.001.02".to_string()
}

#[derive(
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
pub struct CompareMicIdentifier3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<MicIdentifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<MicIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType42Code {
    #[serde(rename = "SLPH")]
    Slph,
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
pub struct AgriculturalCommoditySoft1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType2Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType2Code,
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
pub struct CompareDecimalNumber3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<DecimalNumber>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationStatus8ChoiceEnum {
    #[serde(rename = "RptgData", skip_serializing_if = "Option::is_none")]
    pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
    #[serde(rename = "NoRcncltnReqrd", skip_serializing_if = "Option::is_none")]
    pub no_rcncltn_reqrd: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationStatus8Choice {
    #[serde(flatten)]
    pub value: ReconciliationStatus8ChoiceEnum,
}
#[derive(
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
pub struct TradeTransactionIdentification19 {
    #[serde(rename = "RptgCtrPty")]
    pub rptg_ctr_pty: OrganisationIdentification15Choice,
    #[serde(rename = "OthrCtrPty")]
    pub othr_ctr_pty: PartyIdentification236Choice,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<Max52Text>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement7>,
    #[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
    pub agt_lndr: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<OrganisationIdentification15Choice>,
}
#[derive(
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
pub struct SecurityCommodityCash4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Scty", default)]
    pub scty: Vec<Security48>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmmdty", default)]
    pub cmmdty: Vec<Commodity42>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Csh", default)]
    pub csh: Vec<CashCompare3>,
}
#[derive(
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
pub enum PairedReconciled3Code {
    #[serde(rename = "CLRC")]
    Clrc,
    #[serde(rename = "LNRC")]
    Lnrc,
    #[serde(rename = "PARD")]
    Pard,
    #[serde(rename = "RECO")]
    Reco,
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
pub struct AssetClassCommodityEnvironmental2ChoiceEnum {
    #[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
    pub emssns: Option<EnvironmentalCommodityEmission2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnvironmentCommodityOther1>,
    #[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
    pub wthr: Option<EnvironmentalCommodityWeather1>,
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
#[derive(
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
pub struct SecuritiesLendingType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalSecuritiesLendingType1Code>,
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
pub struct SecuritiesLendingType3Choice {
    #[serde(flatten)]
    pub value: SecuritiesLendingType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompareCommodityAssetClass3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AssetClassCommodity5Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AssetClassCommodity5Choice>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationResult10 {
    #[serde(rename = "CtrPty1")]
    pub ctr_pty_1: OrganisationIdentification15Choice,
    #[serde(rename = "CtrPty2")]
    pub ctr_pty_2: OrganisationIdentification15Choice,
    #[validate]
    #[serde(rename = "MtchgCrit")]
    pub mtchg_crit: MatchingCriteria10,
}
#[derive(
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
pub struct CompareUnitPrice6 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesTransactionPrice19Choice>,
}
#[derive(
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
pub struct CompareExposureType3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ExposureType10Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ExposureType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompareAmountAndDirection1 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AmountAndDirection53>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AmountAndDirection53>,
}
#[derive(
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
pub struct MetalCommodityPrecious1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType16Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType11Code,
}
#[derive(
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
#[derive(
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
pub enum AssetClassProductType14Code {
    #[serde(rename = "OEST")]
    Oest,
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
pub struct AgreementType2ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAgreementType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max50Text>,
}
#[derive(
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
pub struct FertilizerCommodityPotash1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType41Code,
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
pub struct FreightCommodityContainerShip1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType46Code,
}
#[derive(
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
    #[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
    pub frght: Option<AssetClassCommodityFreight3Choice>,
    #[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
    pub infltn: Option<AssetClassCommodityInflation1>,
    #[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
    pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
    #[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
    pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
    #[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
    pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
    #[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
    pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
    #[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
    pub metl: Option<AssetClassCommodityMetal1Choice>,
    #[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
    pub othr_c_10: Option<AssetClassCommodityOtherC102Choice>,
    #[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
    pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
    #[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
    pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AssetClassCommodityOther1>,
    #[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
    pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
    #[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
    pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
    #[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
    pub ppr: Option<AssetClassCommodityPaper3Choice>,
}
#[derive(
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
pub struct CashCompare3 {
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<CompareAmountAndDirection2>,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanMatchingCriteria9 {
    #[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
    pub unq_trad_idr: Option<CompareText2>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<CompareDate3>,
    #[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
    pub ctrct_tp: Option<CompareExposureType3>,
    #[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
    pub clr_sts: Option<CompareClearingStatus3>,
    #[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
    pub clr_dt_tm: Option<CompareDateTime3>,
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<CompareOrganisationIdentification6>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<CompareMicIdentifier3>,
    #[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt_tp: Option<CompareAgreementType2>,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<CompareDateTime3>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<CompareDate3>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<CompareDate3>,
    #[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
    pub min_ntce_prd: Option<CompareNumber5>,
    #[serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none")]
    pub earlst_call_bck_dt: Option<CompareDate3>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<CompareSpecialCollateral3>,
    #[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
    pub dlvry_by_val: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
    pub coll_dlvry_mtd: Option<CompareDeliveryMethod3>,
    #[serde(rename = "OpnTerm", skip_serializing_if = "Option::is_none")]
    pub opn_term: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
    pub termntn_optn: Option<CompareTerminationOption3>,
    #[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
    pub fxd_intrst_rate: Option<ComparePercentageRate3>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
    #[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
    pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
    #[serde(
        rename = "FltgIntrstRateTermUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRateTermVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
    #[serde(
        rename = "FltgIntrstRatePmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRatePmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "FltgIntrstRateRstFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRateRstFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
    #[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
    pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MrgnLnAttr", default)]
    pub mrgn_ln_attr: Vec<CompareInterestRate1>,
    #[serde(rename = "PrncplAmtValDtAmt", skip_serializing_if = "Option::is_none")]
    pub prncpl_amt_val_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
    #[serde(
        rename = "PrncplAmtMtrtyDtAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub prncpl_amt_mtrty_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
    #[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
    pub asst_tp: Option<SecurityCommodity7Choice>,
    #[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
    pub ln_val: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
    #[serde(rename = "FxdRbtRefRate", skip_serializing_if = "Option::is_none")]
    pub fxd_rbt_ref_rate: Option<ComparePercentageRate3>,
    #[serde(rename = "FltgRbtRefRate", skip_serializing_if = "Option::is_none")]
    pub fltg_rbt_ref_rate: Option<CompareBenchmarkCurveName3>,
    #[serde(
        rename = "FltgRbtRateTermUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_rbt_rate_term_unit: Option<CompareRateBasis3>,
    #[serde(rename = "FltgRbtRateTermVal", skip_serializing_if = "Option::is_none")]
    pub fltg_rbt_rate_term_val: Option<CompareNumber6>,
    #[serde(
        rename = "FltgRbtRatePmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_rbt_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgRbtRatePmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_rbt_rate_pmt_frqcy_val: Option<CompareNumber6>,
    #[serde(
        rename = "FltgRbtRateRstFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_rbt_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgRbtRateRstFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_rbt_rate_rst_frqcy_val: Option<CompareNumber6>,
    #[serde(rename = "RbtRateBsisPtSprd", skip_serializing_if = "Option::is_none")]
    pub rbt_rate_bsis_pt_sprd: Option<CompareDecimalNumber3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FltgRateAdjstmnt", default)]
    pub fltg_rate_adjstmnt: Vec<ComparePercentageRate3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FltgRateAdjstmntDt", default)]
    pub fltg_rate_adjstmnt_dt: Vec<CompareDate3>,
    #[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
    pub lndg_fee: Option<ComparePercentageRate3>,
    #[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
    pub outsdng_mrgn_ln_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
    #[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
    pub shrt_mkt_val_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
    #[serde(rename = "LvlTp", skip_serializing_if = "Option::is_none")]
    pub lvl_tp: Option<CompareReportingLevelType3>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}
#[derive(
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
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
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
pub struct CompareCollateralQualityType3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CollateralQualityType1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CollateralQualityType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Commodity42 {
    #[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
    pub clssfctn: Option<CompareCommodityAssetClass3>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<CompareDecimalNumber3>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<CompareUnitPrice6>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<CompareAmountAndDirection2>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}
#[derive(
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
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType13Code {
    #[serde(rename = "MCEX")]
    Mcex,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType48Code {
    #[serde(rename = "NDLV")]
    Ndlv,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType1Code {
    #[serde(rename = "GROS")]
    Gros,
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
pub struct CompareNumber6 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max5Number>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max5Number>,
}
#[derive(
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
pub struct CounterpartyMatchingCriteria4 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
    #[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_sd: Option<CompareCounterpartySide2>,
}
#[derive(
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
pub struct AssetClassCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType15Code,
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
pub struct FreightCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
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
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
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
pub struct EnergyCommodityLightEnd1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType27Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType7Code {
    #[serde(rename = "METL")]
    Metl,
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
pub struct CompareSpecialCollateral3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SpecialCollateral1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SpecialCollateral1Code>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType25Code {
    #[serde(rename = "DIST")]
    Dist,
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
pub struct AssetClassCommodityIndustrialProduct1ChoiceEnum {
    #[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
    pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
    #[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
    pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
}
#[derive(
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
pub struct MatchingCriteria10 {
    #[serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria4>,
    #[serde(rename = "LnMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub ln_mtchg_crit: Option<LoanMatchingCriteria9>,
    #[serde(rename = "CollMtchgCrit", skip_serializing_if = "Option::is_none")]
    pub coll_mtchg_crit: Option<CollateralMatchingCriteria6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberOfReportsPerStatus4 {
    #[validate]
    #[serde(rename = "DtldNbOfRpts")]
    pub dtld_nb_of_rpts: Max15NumericText,
    #[serde(rename = "DtldSts")]
    pub dtld_sts: PairedReconciled3Code,
}
#[derive(
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
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
    #[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
    pub not_avlbl: Option<NotAvailable1Code>,
}
#[derive(
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
#[derive(
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
#[derive(
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
pub struct FertilizerCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
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
pub struct CompareRateBasis3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<RateBasis1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<RateBasis1Code>,
}
#[derive(
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
pub struct ReconciliationMatchedStatus9ChoiceEnum {
    #[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
    pub mtchd: Option<NoReasonCode>,
    #[serde(rename = "NotMtchd", skip_serializing_if = "Option::is_none")]
    pub not_mtchd: Option<ReconciliationResult10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationMatchedStatus9Choice {
    #[serde(flatten)]
    pub value: ReconciliationMatchedStatus9ChoiceEnum,
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
pub struct AssetClassCommodityPaper3ChoiceEnum {
    #[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
    pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PaperCommodityRecoveredPaper2>,
    #[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
    pub nwsprnt: Option<PaperCommodityNewsprint1>,
    #[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
    pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
    #[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
    pub pulp: Option<PaperCommodityPulp1>,
}
#[derive(
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
pub struct AssetClassCommodityFreight3ChoiceEnum {
    #[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
    pub dry: Option<FreightCommodityDry2>,
    #[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
    pub wet: Option<FreightCommodityWet2>,
    #[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
    pub cntnr_ship: Option<FreightCommodityContainerShip1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FreightCommodityOther1>,
}
#[derive(
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
pub struct CompareCfiIdentifier3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CfiOct2015Identifier>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CfiOct2015Identifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType41Code {
    #[serde(rename = "PTSH")]
    Ptsh,
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
pub struct AssetClassCommodityOtherC102ChoiceEnum {
    #[serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none")]
    pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
    #[serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none")]
    pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
}
#[derive(
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
pub struct AssetClassCommodityMultiCommodityExotic1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType13Code,
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
pub struct CompareInterestRate1 {
    #[serde(rename = "MrgnLnAmt", skip_serializing_if = "Option::is_none")]
    pub mrgn_ln_amt: Option<CompareAmountAndDirection1>,
    #[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
    pub fxd_intrst_rate: Option<ComparePercentageRate3>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
    #[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
    pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
    #[serde(
        rename = "FltgIntrstRateTermUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRateTermVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
    #[serde(
        rename = "FltgIntrstRatePmtFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRatePmtFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
    #[serde(
        rename = "FltgIntrstRateRstFrqcyUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
    #[serde(
        rename = "FltgIntrstRateRstFrqcyVal",
        skip_serializing_if = "Option::is_none"
    )]
    pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
    #[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
    pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
}
#[derive(
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
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<EnergyCommodityOther1>,
    #[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
    pub oil: Option<EnergyCommodityOil2>,
    #[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
    pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
    #[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
    pub dstllts: Option<EnergyCommodityDistillates1>,
    #[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
    pub elctrcty: Option<EnergyCommodityElectricity1>,
    #[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
    pub lght_end: Option<EnergyCommodityLightEnd1>,
    #[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
    pub coal: Option<EnergyCommodityCoal1>,
    #[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
    pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
    #[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
    pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompareCountryCode3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CountryCode>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType20Code {
    #[serde(rename = "DIRY")]
    Diry,
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
pub struct CompareUnitOfMeasure3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<UnitOfMeasure11Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<UnitOfMeasure11Code>,
}
#[derive(
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
pub struct Max15NumericText {
    #[validate(regex = "MAX_15_NUMERIC_TEXT_REGEX")]
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
pub struct CompareOrganisationIdentification7 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<PartyIdentification236Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<PartyIdentification236Choice>,
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
pub enum AssetClassSubProductType26Code {
    #[serde(rename = "INRG")]
    Inrg,
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
pub struct CompareAmountAndDirection2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AmountAndDirection53>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AmountAndDirection53>,
}
#[derive(
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
pub struct CompareSecuritiesLendingType3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecuritiesLendingType3Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecuritiesLendingType3Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType8Code {
    #[serde(rename = "OILP")]
    Oilp,
    #[default]
    Unknown,
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
pub enum ExposureType10Code {
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "REPO")]
    Repo,
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
pub struct AgriculturalCommodityOliveOil2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType3Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType29Code,
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct TradeData28<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 0,))]
    #[serde(rename = "PairgRcncltnSts", default)]
    pub pairg_rcncltn_sts: Vec<NumberOfReportsPerStatus4>,
    #[validate(length(min = 1,))]
    #[serde(rename = "RcncltnRpt", default)]
    pub rcncltn_rpt: Vec<ReconciliationReport8>,
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
pub struct CompareInterestComputationMethod3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<InterestComputationMethodFormat6Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<InterestComputationMethodFormat6Choice>,
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
pub struct TradeData34ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<TradeData28<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeData34Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: TradeData34ChoiceEnum<A>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Cleared4ChoiceEnum {
    #[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
    pub clrd: Option<NoReasonCode>,
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
pub struct Cleared4Choice {
    #[serde(flatten)]
    pub value: Cleared4ChoiceEnum,
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
pub struct CompareCounterpartySide2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CollateralRole1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CollateralRole1Code>,
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
pub struct CompareText2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Max52Text>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Max52Text>,
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
pub struct IndustrialProductCommodityManufacturing1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType34Code>,
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
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType31Code {
    #[serde(rename = "DRYF")]
    Dryf,
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
pub struct EnergyCommodityOil2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType8Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType32Code,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RcncltnData")]
    pub rcncltn_data: TradeData34Choice<A>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
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
pub struct AssetClassCommodityAgricultural5ChoiceEnum {
    #[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
    pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
    #[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
    pub grn: Option<AgriculturalCommodityGrain2>,
    #[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
    pub soft: Option<AgriculturalCommoditySoft1>,
    #[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
    pub frstry: Option<AgriculturalCommodityForestry1>,
    #[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
    pub dairy: Option<AgriculturalCommodityDairy1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AgriculturalCommodityOther1>,
    #[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
    pub live_stock: Option<AgriculturalCommodityLiveStock1>,
    #[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
    pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
    #[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
    pub sfd: Option<AgriculturalCommoditySeafood1>,
    #[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
    pub ptt: Option<AgriculturalCommodityPotato1>,
}
#[derive(
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
pub struct Max72Text {
    #[validate(length(min = 1, max = 72,))]
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
pub struct SecuritiesTransactionPrice19ChoiceEnum {
    #[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
    pub pdg_pric: Option<PriceStatus1Code>,
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection107>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
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
pub struct CollateralMatchingCriteria6 {
    #[serde(rename = "UncollsdFlg", skip_serializing_if = "Option::is_none")]
    pub uncollsd_flg: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
    pub net_xpsr_collstn_ind: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
    pub coll_val_dt: Option<CompareDate3>,
    #[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
    pub asst_tp: Option<SecurityCommodityCash4>,
    #[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
    pub bskt_idr: Option<CompareSecurityIdentification4>,
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
pub struct AssetClassCommodityFertilizer3ChoiceEnum {
    #[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
    pub ammn: Option<FertilizerCommodityAmmonia1>,
    #[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
    pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
    #[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
    pub slphr: Option<FertilizerCommoditySulphur1>,
    #[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
    pub urea: Option<FertilizerCommodityUrea1>,
    #[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
    pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FertilizerCommodityOther1>,
    #[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
    pub ptsh: Option<FertilizerCommodityPotash1>,
}
#[derive(
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
#[derive(
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
pub struct EnergyCommodityCoal1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType24Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType47Code {
    #[serde(rename = "DLVR")]
    Dlvr,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "SctiesFincgRptgRcncltnStsAdvc")]
    pub scties_fincg_rptg_rcncltn_sts_advc:
        SecuritiesFinancingReportingReconciliationStatusAdviceV02<A, B>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(
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
pub struct CompareTerminationOption3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<RepoTerminationOption2Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<RepoTerminationOption2Code>,
}
#[derive(
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
pub struct AssetClassCommodityPolypropylene3ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<PolypropyleneCommodityOther1>,
    #[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
    pub plstc: Option<PolypropyleneCommodityPlastic1>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompareReportingLevelType3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ModificationLevel1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ModificationLevel1Code>,
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
pub struct EnergyCommodityInterEnergy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType26Code,
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
pub struct CompareAgreementType2 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<AgreementType1Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<AgreementType1Choice>,
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
pub struct ReconciliationReport8 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max140Text>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TradeTransactionIdentification19,
    #[validate]
    #[serde(rename = "Modfd")]
    pub modfd: TrueFalseIndicator,
    #[serde(rename = "RcncltnSts")]
    pub rcncltn_sts: ReconciliationStatus8Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType21Code {
    #[serde(rename = "FRST")]
    Frst,
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
pub enum AssetClassProductType8Code {
    #[serde(rename = "PAPR")]
    Papr,
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
pub struct Max5Number {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct CompareBenchmarkCurveName3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<BenchmarkCurveName10Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<BenchmarkCurveName10Choice>,
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
pub struct BenchmarkCurveName10ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName3Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
}
#[derive(
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
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType14Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType37Code {
    #[serde(rename = "PULP")]
    Pulp,
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
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
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
pub struct SecurityCommodity7ChoiceEnum {
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<Security48>,
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<Commodity42>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityCommodity7Choice {
    #[serde(flatten)]
    pub value: SecurityCommodity7ChoiceEnum,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAndAmount")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
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
pub struct CompareClearingStatus3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<Cleared4Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<Cleared4Choice>,
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
pub struct AgriculturalCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType49Code,
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
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct AgreementType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
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
pub struct AgreementType1Choice {
    #[serde(flatten)]
    pub value: AgreementType1ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType44Code,
}
#[derive(
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
pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
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
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType10Code {
    #[serde(rename = "EMIS")]
    Emis,
    #[default]
    Unknown,
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
pub struct CompareDeliveryMethod3 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<CollateralDeliveryMethod1Code>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<CollateralDeliveryMethod1Code>,
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
pub struct FertilizerCommodityDiammoniumPhosphate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType40Code,
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
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub enum AssetClassSubProductType30Code {
    #[serde(rename = "WTHR")]
    Wthr,
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
pub struct CompareSecurityIdentification4 {
    #[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
    pub val_1: Option<SecurityIdentification26Choice>,
    #[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
    pub val_2: Option<SecurityIdentification26Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Security48 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<CompareIsinIdentifier4>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<CompareCfiIdentifier3>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<CompareDecimalNumber3>,
    #[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
    pub nmnl_val: Option<CompareAmountAndDirection2>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CompareCollateralQualityType3>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<CompareDate3>,
    #[serde(rename = "IssrId", skip_serializing_if = "Option::is_none")]
    pub issr_id: Option<CompareOrganisationIdentification6>,
    #[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
    pub issr_ctry: Option<CompareCountryCode3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tp", default)]
    pub tp: Vec<CompareSecuritiesLendingType3>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<CompareUnitPrice6>,
    #[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
    pub exclsv_arrgmnt: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<CompareAmountAndDirection2>,
    #[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
    pub avlbl_for_coll_reuse: Option<CompareTrueFalseIndicator3>,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}
