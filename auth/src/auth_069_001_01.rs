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
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.069.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralCollateral2 {
    #[validate(length(min = 1,))]
    #[serde(rename = "ElgblFinInstrmId", default)]
    pub elgbl_fin_instrm_id: Vec<Max35Text>,
}
#[derive(
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
pub struct FinancialInstrumentAttributes89 {
    #[validate]
    #[serde(rename = "CtrctSz")]
    pub ctrct_sz: ContractSize1,
    #[serde(rename = "DlvryTp")]
    pub dlvry_tp: PhysicalTransferType4Code,
    #[validate]
    #[serde(rename = "UndrlygId")]
    pub undrlyg_id: GenericIdentification165,
    #[serde(rename = "PricCcy")]
    pub pric_ccy: ActiveCurrencyCode,
}
#[derive(
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
pub struct ClearedProduct1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "TradgVn", default)]
    pub tradg_vn: Vec<MicIdentifier>,
    #[validate]
    #[serde(rename = "CCPPdctId")]
    pub ccp_pdct_id: GenericIdentification168,
    #[serde(rename = "UvrslPdctId", skip_serializing_if = "Option::is_none")]
    pub uvrsl_pdct_id: Option<GenericIdentification168>,
    #[serde(rename = "Pdct")]
    pub pdct: Product1Choice,
    #[validate]
    #[serde(rename = "OpnIntrst")]
    pub opn_intrst: OpenInterest1,
    #[serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none")]
    pub trds_clrd: Option<NonNegativeNumber>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionEventType1Code {
    #[serde(rename = "CLST")]
    Clst,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "KNIN")]
    Knin,
    #[serde(rename = "KNOC")]
    Knoc,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "TRIG")]
    Trig,
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
pub struct ContractSize1 {
    #[validate]
    #[serde(rename = "LotSz")]
    pub lot_sz: PositiveNumber,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitOfMeasure5Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAnd24Amount {
    #[serde(rename = "ActiveCurrencyAnd24Amount")]
    pub value: ActiveCurrencyAnd24AmountSimpleType,
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
pub struct Product1ChoiceEnum {
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<FinancialInstrument59>,
    #[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
    pub deriv: Option<Derivative3>,
    #[serde(rename = "SctiesFincgTx", skip_serializing_if = "Option::is_none")]
    pub scties_fincg_tx: Option<RepurchaseAgreement3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Product1Choice {
    #[serde(flatten)]
    pub value: Product1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnitOfMeasure8Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure5Choice {
    #[serde(flatten)]
    pub value: UnitOfMeasure5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAndAmountSimpleType {
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
pub struct GenericIdentification168 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "ActiveCurrencyAndAmount")]
    pub value: ActiveCurrencyAndAmountSimpleType,
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
pub struct RepurchaseAgreement3 {
    #[validate]
    #[serde(rename = "PdctClssfctn")]
    pub pdct_clssfctn: ProductClassification1,
    #[serde(rename = "RpAgrmtTp")]
    pub rp_agrmt_tp: RepurchaseAgreementType1Choice,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency11Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "UPFR")]
    Upfr,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "CRED")]
    Cred,
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
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
pub struct InterestRateContractTerm1 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Option14 {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "XprtnStyle", default)]
    pub xprtn_style: Vec<OptionStyle5Code>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<ExoticOptionStyle1Code>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType1Code>,
    #[serde(rename = "BrrrInd", skip_serializing_if = "Option::is_none")]
    pub brrr_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
    pub evt_tp: Option<OptionEvent2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification36 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Standardisation1Code {
    #[serde(rename = "FLEX")]
    Flex,
    #[serde(rename = "NSTA")]
    Nsta,
    #[serde(rename = "STAN")]
    Stan,
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
pub struct ActiveCurrencyAnd24AmountSimpleType {
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
pub struct PositiveNumber {
    #[validate(range(min = 1,))]
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
pub struct OptionEvent2 {
    #[serde(rename = "Tp")]
    pub tp: OptionEventType1Choice,
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonNegativeNumber {
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
pub struct Derivative3 {
    #[validate]
    #[serde(rename = "DerivClssfctn")]
    pub deriv_clssfctn: DerivativeClassification1,
    #[validate(length(min = 1, max = 2,))]
    #[serde(rename = "DerivUndrlygLeg", default)]
    pub deriv_undrlyg_leg: Vec<DerivativeUnderlyingLeg1>,
    #[serde(rename = "OptnAttrbts", skip_serializing_if = "Option::is_none")]
    pub optn_attrbts: Option<Option14>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepurchaseAgreementType1ChoiceEnum {
    #[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
    pub spcfc_coll: Option<SpecificCollateral2>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<GeneralCollateral2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepurchaseAgreementType1Choice {
    #[serde(flatten)]
    pub value: RepurchaseAgreementType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod2Code {
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
    #[serde(rename = "NARR")]
    Narr,
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
pub enum ExoticOptionStyle1Code {
    #[serde(rename = "BINA")]
    Bina,
    #[serde(rename = "DIGI")]
    Digi,
    #[serde(rename = "NOTO")]
    Noto,
    #[serde(rename = "VANI")]
    Vani,
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
pub struct DefinedAttributes1ChoiceEnum {
    #[serde(rename = "ValDfndAttrbts", skip_serializing_if = "Option::is_none")]
    pub val_dfnd_attrbts: Option<FinancialInstrumentAttributes90>,
    #[serde(rename = "QtyDfndAttrbts", skip_serializing_if = "Option::is_none")]
    pub qty_dfnd_attrbts: Option<FinancialInstrumentAttributes89>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DefinedAttributes1Choice {
    #[serde(flatten)]
    pub value: DefinedAttributes1ChoiceEnum,
}
#[derive(
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
pub struct SpecificCollateral2 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: FinancialInstrument59,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentAttributes90 {
    #[serde(rename = "Ntnl", skip_serializing_if = "Option::is_none")]
    pub ntnl: Option<ActiveCurrencyAndAmount>,
    #[validate]
    #[serde(rename = "UnitVal")]
    pub unit_val: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "IndxId")]
    pub indx_id: GenericIdentification168,
    #[validate]
    #[serde(rename = "IndxUnit")]
    pub indx_unit: Max35Text,
    #[serde(rename = "IntrstRateTerms", skip_serializing_if = "Option::is_none")]
    pub intrst_rate_terms: Option<InterestComputationMethod2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpenInterest1 {
    #[validate]
    #[serde(rename = "GrssNtnlAmt")]
    pub grss_ntnl_amt: ActiveCurrencyAnd24Amount,
    #[serde(rename = "NbOfLots", skip_serializing_if = "Option::is_none")]
    pub nb_of_lots: Option<PositiveNumber>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionType1Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
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
pub struct OptionEventType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionEventType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionEventType1Choice {
    #[serde(flatten)]
    pub value: OptionEventType1ChoiceEnum,
}
#[derive(
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
pub struct CcpClearedProductReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 1,))]
    #[serde(rename = "ClrdPdct", default)]
    pub clrd_pdct: Vec<ClearedProduct1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure8Code {
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "KMET")]
    Kmet,
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
    #[serde(rename = "LITR")]
    Litr,
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
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "MIBA")]
    Miba,
    #[serde(rename = "MBTU")]
    Mbtu,
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "PWRD")]
    Pwrd,
    #[serde(rename = "SHAS")]
    Shas,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "THMS")]
    Thms,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "TOCD")]
    Tocd,
    #[serde(rename = "OZTR")]
    Oztr,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "UCWT")]
    Ucwt,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "ACRE")]
    Acre,
    #[serde(rename = "ALOW")]
    Alow,
    #[serde(rename = "ACCY")]
    Accy,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "BARL")]
    Barl,
    #[serde(rename = "BCUF")]
    Bcuf,
    #[serde(rename = "BDFT")]
    Bdft,
    #[serde(rename = "BUSL")]
    Busl,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "CEER")]
    Ceer,
    #[serde(rename = "CLRT")]
    Clrt,
    #[serde(rename = "CBME")]
    Cbme,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "DGEU")]
    Dgeu,
    #[serde(rename = "DMET")]
    Dmet,
    #[serde(rename = "ENVC")]
    Envc,
    #[serde(rename = "ENVO")]
    Envo,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GGEU")]
    Ggeu,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "HUWG")]
    Huwg,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "IPNT")]
    Ipnt,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "USTN")]
    Ustn,
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
pub struct FinancialInstrument59 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: IsinOct2015Identifier,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: LeiIdentifier,
    #[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
    pub sctr: Option<Sna2008SectorIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SchemeIdentificationType1Code {
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "POSI")]
    Posi,
    #[serde(rename = "CLIM")]
    Clim,
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
pub struct DerivativeUnderlyingLeg1 {
    #[validate]
    #[serde(rename = "CtrctAttrbts")]
    pub ctrct_attrbts: FinancialInstrumentAttributes88,
    #[serde(rename = "DfndAttrbts", skip_serializing_if = "Option::is_none")]
    pub dfnd_attrbts: Option<DefinedAttributes1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentAttributes88 {
    #[serde(rename = "CtrctTerm", skip_serializing_if = "Option::is_none")]
    pub ctrct_term: Option<InterestRateContractTerm1>,
    #[validate(length(min = 0, max = 3,))]
    #[serde(rename = "Stdstn", default)]
    pub stdstn: Vec<Standardisation1Code>,
    #[serde(rename = "PmtFrqcy")]
    pub pmt_frqcy: Frequency11Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Sna2008SectorIdentifier {
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
pub struct DerivativeClassification1 {
    #[validate]
    #[serde(rename = "AsstClss")]
    pub asst_clss: Max35Text,
    #[serde(rename = "BasePdct", skip_serializing_if = "Option::is_none")]
    pub base_pdct: Option<Max35Text>,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<Max35Text>,
    #[serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none")]
    pub sub_cmmdty: Option<Max35Text>,
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<Max35Text>,
}
#[derive(
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct ProductClassification1 {
    #[validate]
    #[serde(rename = "AsstClss")]
    pub asst_clss: Max35Text,
    #[serde(rename = "BasePdct", skip_serializing_if = "Option::is_none")]
    pub base_pdct: Option<Max35Text>,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<Max35Text>,
    #[serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none")]
    pub sub_cmmdty: Option<Max35Text>,
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<Max35Text>,
}
#[derive(
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
    #[serde(rename = "CCPClrdPdctRpt")]
    pub ccp_clrd_pdct_rpt: CcpClearedProductReportV01<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle5Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "BERM")]
    Berm,
    #[serde(rename = "EURO")]
    Euro,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification165 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<SchemeIdentificationType1Code>,
}
