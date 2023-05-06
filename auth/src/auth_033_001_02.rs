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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_SUB_DIVISION_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}\-[0-9A-Z]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.033.001.02".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FloatingInterestRate8 {
    #[serde(rename = "RefRate")]
    pub ref_rate: BenchmarkCurveName5Choice,
    #[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
    pub term: Option<InterestRateContractTerm2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditDefaultSwapSingleName2 {
    #[validate]
    #[serde(rename = "SvrgnIssr")]
    pub svrgn_issr: TrueFalseIndicator,
    #[serde(rename = "RefPty", skip_serializing_if = "Option::is_none")]
    pub ref_pty: Option<DerivativePartyIdentification1Choice>,
    #[serde(rename = "NtnlCcy")]
    pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
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
pub enum NonEquityInstrumentReportingClassification1Code {
    #[serde(rename = "SFPS")]
    Sfps,
    #[serde(rename = "SDRV")]
    Sdrv,
    #[serde(rename = "DERV")]
    Derv,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "ETCS")]
    Etcs,
    #[serde(rename = "ETNS")]
    Etns,
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
pub struct InterestRateDerivative5 {
    #[serde(rename = "UndrlygTp")]
    pub undrlyg_tp: InterestRateDerivative2Choice,
    #[serde(rename = "UndrlygBd", skip_serializing_if = "Option::is_none")]
    pub undrlyg_bd: Option<BondDerivative2>,
    #[serde(rename = "SwptnNtnlCcy", skip_serializing_if = "Option::is_none")]
    pub swptn_ntnl_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "UndrlygSwpMtrtyDt", skip_serializing_if = "Option::is_none")]
    pub undrlyg_swp_mtrty_dt: Option<IsoDate>,
    #[serde(rename = "InfltnIndx", skip_serializing_if = "Option::is_none")]
    pub infltn_indx: Option<InflationIndex1Choice>,
    #[validate]
    #[serde(rename = "IntrstRateRef")]
    pub intrst_rate_ref: FloatingInterestRate8,
}
#[derive(
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
pub struct CommodityDerivative5 {
    #[validate]
    #[serde(rename = "Sz")]
    pub sz: Max25Text,
    #[validate]
    #[serde(rename = "AvrgTmChrtr")]
    pub avrg_tm_chrtr: Max25Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EquityReturnParameter1Code {
    #[serde(rename = "PRDV")]
    Prdv,
    #[serde(rename = "PRVA")]
    Prva,
    #[serde(rename = "PRVO")]
    Prvo,
    #[serde(rename = "PRBP")]
    Prbp,
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
pub struct TradingVenueIdentification1ChoiceEnum {
    #[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
    pub ntl_cmptnt_authrty: Option<CountryCode>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<TradingVenueIdentification2>,
    #[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
    pub mkt_id_cd: Option<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingVenueIdentification1Choice {
    #[serde(flatten)]
    pub value: TradingVenueIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommodityDerivative4 {
    #[serde(rename = "ClssSpcfc", skip_serializing_if = "Option::is_none")]
    pub clss_spcfc: Option<CommodityDerivative2Choice>,
    #[serde(rename = "NtnlCcy")]
    pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditDefaultSwapsDerivative4ChoiceEnum {
    #[serde(
        rename = "SnglNmCdtDfltSwpDeriv",
        skip_serializing_if = "Option::is_none"
    )]
    pub sngl_nm_cdt_dflt_swp_deriv: Option<CreditDefaultSwapDerivative6>,
    #[serde(
        rename = "CdtDfltSwpIndxDeriv",
        skip_serializing_if = "Option::is_none"
    )]
    pub cdt_dflt_swp_indx_deriv: Option<CreditDefaultSwapDerivative5>,
    #[serde(rename = "CdtDfltSwpIndx", skip_serializing_if = "Option::is_none")]
    pub cdt_dflt_swp_indx: Option<CreditDefaultSwapIndex3>,
    #[serde(rename = "SnglNmCdtDfltSwp", skip_serializing_if = "Option::is_none")]
    pub sngl_nm_cdt_dflt_swp: Option<CreditDefaultSwapSingleName2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditDefaultSwapsDerivative4Choice {
    #[serde(flatten)]
    pub value: CreditDefaultSwapsDerivative4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingEquityType3Code {
    #[serde(rename = "BSKT")]
    Bskt,
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
pub struct EquityDerivative3ChoiceEnum {
    #[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
    pub bskt: Option<UnderlyingEquityType3Code>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<UnderlyingEquityType6Code>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<UnderlyingEquityType4Code>,
    #[serde(rename = "SnglNm", skip_serializing_if = "Option::is_none")]
    pub sngl_nm: Option<UnderlyingEquityType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EquityDerivative3Choice {
    #[serde(flatten)]
    pub value: EquityDerivative3ChoiceEnum,
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
pub enum EmissionAllowanceProductType2Code {
    #[serde(rename = "CERE")]
    Cere,
    #[serde(rename = "ERUE")]
    Erue,
    #[serde(rename = "EUAE")]
    Euae,
    #[serde(rename = "EUAA")]
    Euaa,
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
pub struct InterestRateContractTerm2 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max3Number,
}
#[derive(
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
pub enum UnderlyingEquityType6Code {
    #[serde(rename = "BSKT")]
    Bskt,
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "ETFS")]
    Etfs,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SHRS")]
    Shrs,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "STIX")]
    Stix,
    #[serde(rename = "VOLI")]
    Voli,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BondType1Code {
    #[serde(rename = "EUSB")]
    Eusb,
    #[serde(rename = "OEPB")]
    Oepb,
    #[serde(rename = "CVTB")]
    Cvtb,
    #[serde(rename = "CRPB")]
    Crpb,
    #[serde(rename = "CVDB")]
    Cvdb,
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
pub struct ForeignExchangeDerivative2 {
    #[serde(rename = "CtrctSubTp")]
    pub ctrct_sub_tp: AssetClassSubProductType19Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialInstrumentContractType1Code {
    #[serde(rename = "CFDS")]
    Cfds,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FRAS")]
    Fras,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SPDB")]
    Spdb,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "FONS")]
    Fons,
    #[serde(rename = "PSWP")]
    Pswp,
    #[serde(rename = "FFAS")]
    Ffas,
    #[serde(rename = "FWOS")]
    Fwos,
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
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingContractForDifferenceType3Code {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "FTEQ")]
    Fteq,
    #[serde(rename = "OPEQ")]
    Opeq,
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
pub struct CreditDefaultSwapDerivative6 {
    #[serde(
        rename = "UndrlygCdtDfltSwpId",
        skip_serializing_if = "Option::is_none"
    )]
    pub undrlyg_cdt_dflt_swp_id: Option<IsinOct2015Identifier>,
    #[validate]
    #[serde(rename = "OblgtnId")]
    pub oblgtn_id: IsinOct2015Identifier,
    #[validate]
    #[serde(rename = "SnglNm")]
    pub sngl_nm: CreditDefaultSwapSingleName2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractForDifference2 {
    #[serde(rename = "UndrlygTp")]
    pub undrlyg_tp: UnderlyingContractForDifferenceType3Code,
    #[serde(rename = "NtnlCcy1", skip_serializing_if = "Option::is_none")]
    pub ntnl_ccy_1: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "NtnlCcy2", skip_serializing_if = "Option::is_none")]
    pub ntnl_ccy_2: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SwapType1Code {
    #[serde(rename = "OSSC")]
    Ossc,
    #[serde(rename = "XFSC")]
    Xfsc,
    #[serde(rename = "XFMC")]
    Xfmc,
    #[serde(rename = "XXSC")]
    Xxsc,
    #[serde(rename = "XXMC")]
    Xxmc,
    #[serde(rename = "IFMC")]
    Ifmc,
    #[serde(rename = "FFSC")]
    Ffsc,
    #[serde(rename = "FFMC")]
    Ffmc,
    #[serde(rename = "IFSC")]
    Ifsc,
    #[serde(rename = "OSMC")]
    Osmc,
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
pub struct CommodityDerivative2ChoiceEnum {
    #[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
    pub frght: Option<CommodityDerivative5>,
    #[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
    pub nrgy: Option<CommodityDerivative6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommodityDerivative2Choice {
    #[serde(flatten)]
    pub value: CommodityDerivative2ChoiceEnum,
}
#[derive(
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
pub struct SecuritiesMarketReportHeader1 {
    #[serde(rename = "RptgNtty")]
    pub rptg_ntty: TradingVenueIdentification1Choice,
    #[serde(rename = "RptgPrd")]
    pub rptg_prd: Period4Choice,
    #[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
    pub submissn_dt_tm: Option<IsoDateTime>,
}
#[derive(
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
pub struct Max25Text {
    #[validate(length(min = 1, max = 25,))]
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
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType19Code {
    #[serde(rename = "DLVR")]
    Dlvr,
    #[serde(rename = "NDLV")]
    Ndlv,
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
pub struct InflationIndex1ChoiceEnum {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InflationIndex1Choice {
    #[serde(flatten)]
    pub value: InflationIndex1ChoiceEnum,
}
#[derive(
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
pub struct Period2 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditDefaultSwapIndex3 {
    #[serde(rename = "UndrlygIndxId", skip_serializing_if = "Option::is_none")]
    pub undrlyg_indx_id: Option<IsinOct2015Identifier>,
    #[serde(rename = "UndrlygIndxNm", skip_serializing_if = "Option::is_none")]
    pub undrlyg_indx_nm: Option<Max25Text>,
    #[serde(rename = "Srs", skip_serializing_if = "Option::is_none")]
    pub srs: Option<Number>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 0, max = 12,))]
    #[serde(rename = "RollMnth", default)]
    pub roll_mnth: Vec<RestrictedMonthExact2Number>,
    #[serde(rename = "NxtRollDt", skip_serializing_if = "Option::is_none")]
    pub nxt_roll_dt: Option<IsoDate>,
    #[serde(rename = "NtnlCcy")]
    pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingVenueIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max50Text,
    #[serde(rename = "Tp")]
    pub tp: TradingVenue2Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingEquityType5Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "ETFS")]
    Etfs,
    #[serde(rename = "SHRS")]
    Shrs,
    #[serde(rename = "DVSE")]
    Dvse,
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
pub struct FinancialInstrumentReportingNonEquityTransparencyDataReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: SecuritiesMarketReportHeader1,
    #[validate(length(min = 1,))]
    #[serde(rename = "NonEqtyTrnsprncyData", default)]
    pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport16>,
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
pub struct BenchmarkCurveName5ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName5Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName5ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct TransparencyDataReport16 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: IsinOct2015Identifier,
    #[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
    pub full_nm: Option<Max350Text>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<MicIdentifier>,
    #[serde(rename = "RptgDt", skip_serializing_if = "Option::is_none")]
    pub rptg_dt: Option<IsoDate>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "FinInstrmClssfctn")]
    pub fin_instrm_clssfctn: NonEquityInstrumentReportingClassification1Code,
    #[serde(
        rename = "UndrlygInstrmAsstClss",
        skip_serializing_if = "Option::is_none"
    )]
    pub undrlyg_instrm_asst_clss: Option<ProductType5Code>,
    #[serde(rename = "DerivCtrctTp", skip_serializing_if = "Option::is_none")]
    pub deriv_ctrct_tp: Option<FinancialInstrumentContractType1Code>,
    #[serde(rename = "Bd", skip_serializing_if = "Option::is_none")]
    pub bd: Option<DebtInstrument5>,
    #[serde(rename = "EmssnAllwncTp", skip_serializing_if = "Option::is_none")]
    pub emssn_allwnc_tp: Option<EmissionAllowanceProductType2Code>,
    #[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
    pub deriv: Option<Derivative3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BondDerivative2 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: LeiIdentifier,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "IssncDt", skip_serializing_if = "Option::is_none")]
    pub issnc_dt: Option<IsoDate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EmissionAllowanceProductType1Code {
    #[serde(rename = "EUAA")]
    Euaa,
    #[serde(rename = "EUAE")]
    Euae,
    #[serde(rename = "ERUE")]
    Erue,
    #[serde(rename = "CERE")]
    Cere,
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
pub struct EquityDerivative2 {
    #[serde(rename = "UndrlygTp")]
    pub undrlyg_tp: EquityDerivative3Choice,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<EquityReturnParameter1Code>,
}
#[derive(
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
    #[serde(rename = "FinInstrmRptgNonEqtyTrnsprncyDataRpt")]
    pub fin_instrm_rptg_non_eqty_trnsprncy_data_rpt:
        FinancialInstrumentReportingNonEquityTransparencyDataReportV02<A>,
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
pub struct CommodityDerivative6 {
    #[validate]
    #[serde(rename = "SttlmLctn")]
    pub sttlm_lctn: Max25Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingInterestRateType3Code {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "BNDF")]
    Bndf,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "IFUT")]
    Ifut,
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
pub struct CreditDefaultSwapDerivative5 {
    #[serde(
        rename = "UndrlygCdtDfltSwpId",
        skip_serializing_if = "Option::is_none"
    )]
    pub undrlyg_cdt_dflt_swp_id: Option<IsinOct2015Identifier>,
    #[validate]
    #[serde(rename = "UndrlygCdtDfltSwpIndx")]
    pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateDerivative2ChoiceEnum {
    #[serde(rename = "SwpRltd", skip_serializing_if = "Option::is_none")]
    pub swp_rltd: Option<SwapType1Code>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<UnderlyingInterestRateType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateDerivative2Choice {
    #[serde(flatten)]
    pub value: InterestRateDerivative2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period4ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt_to_dt: Option<Period2>,
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
pub struct Period4Choice {
    #[serde(flatten)]
    pub value: Period4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductType5Code {
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName2Code {
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "BBSW")]
    Bbsw,
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
pub struct DebtInstrument5 {
    #[serde(rename = "Tp")]
    pub tp: BondType1Code,
    #[validate]
    #[serde(rename = "IssncDt")]
    pub issnc_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Derivative3ChoiceEnum {
    #[serde(rename = "EmssnAllwnc", skip_serializing_if = "Option::is_none")]
    pub emssn_allwnc: Option<EmissionAllowanceProductType1Code>,
    #[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
    pub fx: Option<ForeignExchangeDerivative2>,
    #[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
    pub eqty: Option<EquityDerivative2>,
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<CommodityDerivative4>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<InterestRateDerivative5>,
    #[serde(rename = "CtrctForDiff", skip_serializing_if = "Option::is_none")]
    pub ctrct_for_diff: Option<ContractForDifference2>,
    #[serde(rename = "Cdt", skip_serializing_if = "Option::is_none")]
    pub cdt: Option<CreditDefaultSwapsDerivative4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Derivative3Choice {
    #[serde(flatten)]
    pub value: Derivative3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingEquityType4Code {
    #[serde(rename = "STIX")]
    Stix,
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "VOLI")]
    Voli,
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
pub enum TradingVenue2Code {
    #[serde(rename = "APPA")]
    Appa,
    #[serde(rename = "CTPS")]
    Ctps,
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
pub struct RestrictedMonthExact2Number {
    #[validate(range(min = 1, max = 12,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
