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
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.019.001.05".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdjustmentDirection1Code {
    #[serde(rename = "ADDD")]
    Addd,
    #[serde(rename = "SUBS")]
    Subs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BankRole1Code {
    #[serde(rename = "BUYB")]
    Buyb,
    #[serde(rename = "OBLB")]
    Oblb,
    #[serde(rename = "RECB")]
    Recb,
    #[serde(rename = "SELB")]
    Selb,
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
pub struct ShipmentDateRange2 {
    #[validate]
    #[serde(rename = "SubQtyVal")]
    pub sub_qty_val: DecimalNumber,
    #[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub earlst_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub latst_shipmnt_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max34Text {
    #[validate(length(min = 1, max = 34,))]
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct UserDefinedInformation1 {
    #[validate]
    #[serde(rename = "Labl")]
    pub labl: Max35Text,
    #[validate]
    #[serde(rename = "Inf")]
    pub inf: Max140Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicIdentification1 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: BicIdentifier,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
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
pub struct CountrySubdivision1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max35Text>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountrySubdivision1Choice {
    #[serde(flatten)]
    pub value: CountrySubdivision1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification26 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
    #[validate]
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: PostalAddress5,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequiredSubmission2 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Submitr", default)]
    pub submitr: Vec<BicIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BpoApplicableRules1ChoiceEnum {
    #[serde(rename = "URBPOVrsn", skip_serializing_if = "Option::is_none")]
    pub urbpo_vrsn: Option<DecimalNumber>,
    #[serde(rename = "OthrRulesAndVrsn", skip_serializing_if = "Option::is_none")]
    pub othr_rules_and_vrsn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BpoApplicableRules1Choice {
    #[serde(flatten)]
    pub value: BpoApplicableRules1ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification27 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicIdentifier {
    #[validate(regex = "BIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeCertificateType1Code {
    #[serde(rename = "ANLY")]
    Anly,
    #[serde(rename = "QUAL")]
    Qual,
    #[serde(rename = "QUAN")]
    Quan,
    #[serde(rename = "WEIG")]
    Weig,
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "HEAL")]
    Heal,
    #[serde(rename = "PHYT")]
    Phyt,
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
pub struct InitialBaselineSubmissionV05 {
    #[validate]
    #[serde(rename = "SubmissnId")]
    pub submissn_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "SubmitrTxRef")]
    pub submitr_tx_ref: SimpleIdentificationInformation,
    #[validate]
    #[serde(rename = "Instr")]
    pub instr: InstructionType1,
    #[validate]
    #[serde(rename = "Baseln")]
    pub baseln: Baseline5,
    #[validate(length(min = 0,))]
    #[serde(rename = "BuyrCtctPrsn", default)]
    pub buyr_ctct_prsn: Vec<ContactIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SellrCtctPrsn", default)]
    pub sellr_ctct_prsn: Vec<ContactIdentification1>,
    #[serde(rename = "BkCtctPrsn")]
    pub bk_ctct_prsn: BankContactPerson1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrBkCtctPrsn", default)]
    pub othr_bk_ctct_prsn: Vec<ContactIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Incoterms4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalIncoterms1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Incoterms4Choice {
    #[serde(flatten)]
    pub value: Incoterms4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionType1 {
    #[serde(rename = "Tp")]
    pub tp: InstructionType1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity9 {
    #[serde(rename = "UnitOfMeasr")]
    pub unit_of_measr: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: DecimalNumber,
    #[serde(rename = "Fctr", skip_serializing_if = "Option::is_none")]
    pub fctr: Option<Max15NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MultimodalTransport3 {
    #[validate]
    #[serde(rename = "TakngInChrg")]
    pub takng_in_chrg: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfFnlDstn")]
    pub plc_of_fnl_dstn: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitPrice18 {
    #[serde(rename = "UnitPric")]
    pub unit_pric: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
    #[serde(rename = "Fctr", skip_serializing_if = "Option::is_none")]
    pub fctr: Option<Max15NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Baseline5 {
    #[validate]
    #[serde(rename = "SubmitrBaselnId")]
    pub submitr_baseln_id: DocumentIdentification1,
    #[serde(rename = "SvcCd")]
    pub svc_cd: TradeFinanceService2Code,
    #[validate]
    #[serde(rename = "PurchsOrdrRef")]
    pub purchs_ordr_ref: DocumentIdentification7,
    #[validate]
    #[serde(rename = "Buyr")]
    pub buyr: PartyIdentification26,
    #[validate]
    #[serde(rename = "Sellr")]
    pub sellr: PartyIdentification26,
    #[validate]
    #[serde(rename = "BuyrBk")]
    pub buyr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "SellrBk")]
    pub sellr_bk: BicIdentification1,
    #[validate(length(min = 0,))]
    #[serde(rename = "BuyrSdSubmitgBk", default)]
    pub buyr_sd_submitg_bk: Vec<BicIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SellrSdSubmitgBk", default)]
    pub sellr_sd_submitg_bk: Vec<BicIdentification1>,
    #[serde(rename = "BllTo", skip_serializing_if = "Option::is_none")]
    pub bll_to: Option<PartyIdentification26>,
    #[serde(rename = "ShipTo", skip_serializing_if = "Option::is_none")]
    pub ship_to: Option<PartyIdentification26>,
    #[serde(rename = "Consgn", skip_serializing_if = "Option::is_none")]
    pub consgn: Option<PartyIdentification26>,
    #[validate]
    #[serde(rename = "Goods")]
    pub goods: LineItem13,
    #[validate(length(min = 1,))]
    #[serde(rename = "PmtTerms", default)]
    pub pmt_terms: Vec<PaymentTerms5>,
    #[serde(rename = "SttlmTerms", skip_serializing_if = "Option::is_none")]
    pub sttlm_terms: Option<SettlementTerms3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtOblgtn", default)]
    pub pmt_oblgtn: Vec<PaymentObligation2>,
    #[serde(rename = "LatstMtchDt", skip_serializing_if = "Option::is_none")]
    pub latst_mtch_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "ComrclDataSetReqrd")]
    pub comrcl_data_set_reqrd: RequiredSubmission2,
    #[serde(
        rename = "TrnsprtDataSetReqrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub trnsprt_data_set_reqrd: Option<RequiredSubmission2>,
    #[serde(rename = "InsrncDataSetReqrd", skip_serializing_if = "Option::is_none")]
    pub insrnc_data_set_reqrd: Option<RequiredSubmission3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertDataSetReqrd", default)]
    pub cert_data_set_reqrd: Vec<RequiredSubmission4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrCertDataSetReqrd", default)]
    pub othr_cert_data_set_reqrd: Vec<RequiredSubmission6>,
    #[validate]
    #[serde(rename = "InttToPayXpctd")]
    pub intt_to_pay_xpctd: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentage2ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct AmountOrPercentage2Choice {
    #[serde(flatten)]
    pub value: AmountOrPercentage2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyAndAmount {
    #[serde(rename = "CurrencyAndAmount")]
    pub value: CurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: CurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
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
pub struct DocumentIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
    #[validate]
    #[serde(rename = "Submitr")]
    pub submitr: BicIdentification1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Charges5 {
    #[serde(rename = "ChrgsPyer")]
    pub chrgs_pyer: BankRole1Code,
    #[serde(rename = "ChrgsPyee")]
    pub chrgs_pyee: BankRole1Code,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<CurrencyAndAmount>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
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
pub struct PaymentObligation2 {
    #[validate]
    #[serde(rename = "OblgrBk")]
    pub oblgr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "RcptBk")]
    pub rcpt_bk: BicIdentification1,
    #[serde(rename = "PmtOblgtnAmt")]
    pub pmt_oblgtn_amt: AmountOrPercentage2Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "Chrgs", default)]
    pub chrgs: Vec<Charges5>,
    #[validate]
    #[serde(rename = "XpryDt")]
    pub xpry_dt: IsoDate,
    #[serde(rename = "AplblRules", skip_serializing_if = "Option::is_none")]
    pub aplbl_rules: Option<BpoApplicableRules1Choice>,
    #[serde(rename = "AplblLaw", skip_serializing_if = "Option::is_none")]
    pub aplbl_law: Option<CountryCode>,
    #[serde(rename = "PlcOfJursdctn", skip_serializing_if = "Option::is_none")]
    pub plc_of_jursdctn: Option<Location2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtTerms", default)]
    pub pmt_terms: Vec<PaymentTerms4>,
    #[serde(rename = "SttlmTerms", skip_serializing_if = "Option::is_none")]
    pub sttlm_terms: Option<SettlementTerms3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductCategory1Code {
    #[serde(rename = "HRTR")]
    Hrtr,
    #[serde(rename = "QOTA")]
    Qota,
    #[serde(rename = "PRGP")]
    Prgp,
    #[serde(rename = "LOBU")]
    Lobu,
    #[serde(rename = "GNDR")]
    Gndr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductCharacteristics1Code {
    #[serde(rename = "BISP")]
    Bisp,
    #[serde(rename = "CHNR")]
    Chnr,
    #[serde(rename = "CLOR")]
    Clor,
    #[serde(rename = "EDSP")]
    Edsp,
    #[serde(rename = "ENNR")]
    Ennr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "ORCR")]
    Orcr,
    #[serde(rename = "PCTV")]
    Pctv,
    #[serde(rename = "SISP")]
    Sisp,
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "SZRG")]
    Szrg,
    #[serde(rename = "SPRM")]
    Sprm,
    #[serde(rename = "STOR")]
    Stor,
    #[serde(rename = "VINR")]
    Vinr,
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
pub struct PaymentPeriod3 {
    #[serde(rename = "Cd")]
    pub cd: PaymentTime3Code,
    #[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
    pub nb_of_days: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress2 {
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[validate]
    #[serde(rename = "PstCdId")]
    pub pst_cd_id: Max16Text,
    #[validate]
    #[serde(rename = "TwnNm")]
    pub twn_nm: Max35Text,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxType9Code {
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "NATI")]
    Nati,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "VATA")]
    Vata,
    #[serde(rename = "CUST")]
    Cust,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
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
pub struct PaymentPeriod4 {
    #[serde(rename = "Cd")]
    pub cd: PaymentTime4Code,
    #[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
    pub nb_of_days: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LineItemDetails13 {
    #[validate]
    #[serde(rename = "LineItmId")]
    pub line_itm_id: Max70Text,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: Quantity9,
    #[serde(rename = "QtyTlrnce", skip_serializing_if = "Option::is_none")]
    pub qty_tlrnce: Option<PercentageTolerance1>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<UnitPrice18>,
    #[serde(rename = "PricTlrnce", skip_serializing_if = "Option::is_none")]
    pub pric_tlrnce: Option<PercentageTolerance1>,
    #[serde(rename = "PdctNm", skip_serializing_if = "Option::is_none")]
    pub pdct_nm: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctIdr", default)]
    pub pdct_idr: Vec<ProductIdentifier2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctChrtcs", default)]
    pub pdct_chrtcs: Vec<ProductCharacteristics1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctCtgy", default)]
    pub pdct_ctgy: Vec<ProductCategory1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctOrgn", default)]
    pub pdct_orgn: Vec<CountryCode>,
    #[serde(rename = "ShipmntSchdl", skip_serializing_if = "Option::is_none")]
    pub shipmnt_schdl: Option<ShipmentSchedule2Choice>,
    #[serde(rename = "RtgSummry", skip_serializing_if = "Option::is_none")]
    pub rtg_summry: Option<TransportMeans5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment7>,
    #[serde(rename = "FrghtChrgs", skip_serializing_if = "Option::is_none")]
    pub frght_chrgs: Option<Charge24>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax23>,
    #[validate]
    #[serde(rename = "TtlAmt")]
    pub ttl_amt: CurrencyAndAmount,
    #[serde(rename = "Incotrms", skip_serializing_if = "Option::is_none")]
    pub incotrms: Option<Incoterms4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalCashAccountType1Code>,
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
pub struct CashAccountType2Choice {
    #[serde(flatten)]
    pub value: CashAccountType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequiredSubmission3 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Submitr", default)]
    pub submitr: Vec<BicIdentification1>,
    #[serde(rename = "MtchIssr", skip_serializing_if = "Option::is_none")]
    pub mtch_issr: Option<PartyIdentification27>,
    #[validate]
    #[serde(rename = "MtchIsseDt")]
    pub mtch_isse_dt: YesNoIndicator,
    #[validate]
    #[serde(rename = "MtchTrnsprt")]
    pub mtch_trnsprt: YesNoIndicator,
    #[validate]
    #[serde(rename = "MtchAmt")]
    pub mtch_amt: YesNoIndicator,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClausesReqrd", default)]
    pub clauses_reqrd: Vec<InsuranceClauses1Code>,
    #[serde(rename = "MtchAssrdPty", skip_serializing_if = "Option::is_none")]
    pub mtch_assrd_pty: Option<AssuredType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LineItem13 {
    #[serde(rename = "GoodsAndOrSvcsDesc", skip_serializing_if = "Option::is_none")]
    pub goods_and_or_svcs_desc: Option<Max70Text>,
    #[validate]
    #[serde(rename = "PrtlShipmnt")]
    pub prtl_shipmnt: YesNoIndicator,
    #[serde(rename = "TrnsShipmnt", skip_serializing_if = "Option::is_none")]
    pub trns_shipmnt: Option<YesNoIndicator>,
    #[serde(rename = "ShipmntDtRg", skip_serializing_if = "Option::is_none")]
    pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "LineItmDtls", default)]
    pub line_itm_dtls: Vec<LineItemDetails13>,
    #[validate]
    #[serde(rename = "LineItmsTtlAmt")]
    pub line_itms_ttl_amt: CurrencyAndAmount,
    #[serde(rename = "RtgSummry", skip_serializing_if = "Option::is_none")]
    pub rtg_summry: Option<TransportMeans5>,
    #[serde(rename = "Incotrms", skip_serializing_if = "Option::is_none")]
    pub incotrms: Option<Incoterms4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment7>,
    #[serde(rename = "FrghtChrgs", skip_serializing_if = "Option::is_none")]
    pub frght_chrgs: Option<Charge24>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax23>,
    #[validate]
    #[serde(rename = "TtlNetAmt")]
    pub ttl_net_amt: CurrencyAndAmount,
    #[validate(length(min = 0,))]
    #[serde(rename = "BuyrDfndInf", default)]
    pub buyr_dfnd_inf: Vec<UserDefinedInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SellrDfndInf", default)]
    pub sellr_dfnd_inf: Vec<UserDefinedInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType2ChoiceEnum {
    #[serde(rename = "OthrTaxTp", skip_serializing_if = "Option::is_none")]
    pub othr_tax_tp: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxType9Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType2Choice {
    #[serde(flatten)]
    pub value: TaxType2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FreightCharges1Code {
    #[serde(rename = "CLCT")]
    Clct,
    #[serde(rename = "PRPD")]
    Prpd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix1Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MADM")]
    Madm,
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
pub struct Iban2007Identifier {
    #[validate(regex = "IBAN_2007_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeType8Code {
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "STDE")]
    Stde,
    #[serde(rename = "STOR")]
    Stor,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PICK")]
    Pick,
    #[serde(rename = "DNGR")]
    Dngr,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "COLF")]
    Colf,
    #[serde(rename = "CHOR")]
    Chor,
    #[serde(rename = "CHDE")]
    Chde,
    #[serde(rename = "AIRF")]
    Airf,
    #[serde(rename = "TRPT")]
    Trpt,
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
pub struct GenericAccountIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice>,
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
pub struct NameAndAddress6 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress2,
}
#[derive(
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
pub struct AirportName1ChoiceEnum {
    #[serde(rename = "AirprtCd", skip_serializing_if = "Option::is_none")]
    pub airprt_cd: Option<Max6Text>,
    #[serde(rename = "OthrAirprtDesc", skip_serializing_if = "Option::is_none")]
    pub othr_airprt_desc: Option<AirportDescription1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AirportName1Choice {
    #[serde(flatten)]
    pub value: AirportName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdTp")]
    pub id_tp: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure3ChoiceEnum {
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "UnitOfMeasrCd", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr_cd: Option<UnitOfMeasure4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure3Choice {
    #[serde(flatten)]
    pub value: UnitOfMeasure3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Incoterms4 {
    #[serde(rename = "IncotrmsCd")]
    pub incotrms_cd: Incoterms4Choice,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyCode {
    #[validate(regex = "CURRENCY_CODE_REGEX")]
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
pub struct ShipmentDateRange1 {
    #[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub earlst_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub latst_shipmnt_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4Choice {
    #[serde(flatten)]
    pub value: AccountIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification4ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<BicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification4Choice {
    #[serde(flatten)]
    pub value: FinancialInstitutionIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AirportDescription1 {
    #[validate]
    #[serde(rename = "Twn")]
    pub twn: Max35Text,
    #[serde(rename = "AirprtNm", skip_serializing_if = "Option::is_none")]
    pub airprt_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tax23 {
    #[serde(rename = "Tp")]
    pub tp: TaxType2Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequiredSubmission4 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Submitr", default)]
    pub submitr: Vec<BicIdentification1>,
    #[serde(rename = "CertTp")]
    pub cert_tp: TradeCertificateType1Code,
    #[serde(rename = "MtchIssr", skip_serializing_if = "Option::is_none")]
    pub mtch_issr: Option<PartyIdentification27>,
    #[validate]
    #[serde(rename = "MtchIsseDt")]
    pub mtch_isse_dt: YesNoIndicator,
    #[validate]
    #[serde(rename = "MtchInspctnDt")]
    pub mtch_inspctn_dt: YesNoIndicator,
    #[validate]
    #[serde(rename = "AuthrsdInspctrInd")]
    pub authrsd_inspctr_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "MtchConsgn")]
    pub mtch_consgn: YesNoIndicator,
    #[serde(rename = "MtchManfctr", skip_serializing_if = "Option::is_none")]
    pub mtch_manfctr: Option<PartyIdentification27>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItmId", default)]
    pub line_itm_id: Vec<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeFinanceService2Code {
    #[serde(rename = "LEV1")]
    Lev1,
    #[serde(rename = "LEV2")]
    Lev2,
    #[serde(rename = "LEV3")]
    Lev3,
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
pub struct TransportBySea6 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PortOfLoadng", default)]
    pub port_of_loadng: Vec<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "PortOfDschrge", default)]
    pub port_of_dschrge: Vec<Max35Text>,
    #[serde(rename = "VsslNm", skip_serializing_if = "Option::is_none")]
    pub vssl_nm: Option<Max70Text>,
    #[serde(rename = "SeaCrrierNm", skip_serializing_if = "Option::is_none")]
    pub sea_crrier_nm: Option<Max70Text>,
    #[serde(rename = "SeaCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub sea_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SingleTransport7 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByAir", default)]
    pub trnsprt_by_air: Vec<TransportByAir5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtBySea", default)]
    pub trnsprt_by_sea: Vec<TransportBySea6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByRoad", default)]
    pub trnsprt_by_road: Vec<TransportByRoad5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByRail", default)]
    pub trnsprt_by_rail: Vec<TransportByRail5>,
}
#[derive(
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
pub struct ProductCharacteristics1 {
    #[serde(rename = "Tp")]
    pub tp: ProductCharacteristics1Code,
    #[validate]
    #[serde(rename = "Chrtcs")]
    pub chrtcs: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCharacteristics1ChoiceEnum {
    #[serde(rename = "StrdPdctChrtcs", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_chrtcs: Option<ProductCharacteristics1>,
    #[serde(rename = "OthrPdctChrtcs", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_chrtcs: Option<GenericIdentification4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCharacteristics1Choice {
    #[serde(flatten)]
    pub value: ProductCharacteristics1ChoiceEnum,
}
#[derive(
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
pub struct ContactIdentification1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Max35Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCategory1ChoiceEnum {
    #[serde(rename = "OthrPdctCtgy", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_ctgy: Option<GenericIdentification4>,
    #[serde(rename = "StrdPdctCtgy", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_ctgy: Option<ProductCategory1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCategory1Choice {
    #[serde(flatten)]
    pub value: ProductCategory1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Charge24 {
    #[serde(rename = "Tp")]
    pub tp: FreightCharges1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "Chrgs", default)]
    pub chrgs: Vec<ChargesDetails3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContactIdentification3 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: BicIdentifier,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Max35Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentTerms4 {
    #[serde(rename = "PmtTerms")]
    pub pmt_terms: PaymentCodeOrOther1Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportByRoad5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PlcOfRct", default)]
    pub plc_of_rct: Vec<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "PlcOfDlvry", default)]
    pub plc_of_dlvry: Vec<Max35Text>,
    #[serde(rename = "RoadCrrierNm", skip_serializing_if = "Option::is_none")]
    pub road_crrier_nm: Option<Max70Text>,
    #[serde(rename = "RoadCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub road_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstructionType1Code {
    #[serde(rename = "LODG")]
    Lodg,
    #[serde(rename = "FPTR")]
    Fptr,
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
pub struct CashAccount24 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(
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
pub struct ExternalIncoterms1Code {
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
pub struct ExternalAccountIdentification1Code {
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
pub struct CurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentTime3Code {
    #[serde(rename = "EMTD")]
    Emtd,
    #[serde(rename = "EMTR")]
    Emtr,
    #[serde(rename = "EPBE")]
    Epbe,
    #[serde(rename = "EPRD")]
    Eprd,
    #[serde(rename = "PRMD")]
    Prmd,
    #[serde(rename = "PRMR")]
    Prmr,
    #[serde(rename = "EPIN")]
    Epin,
    #[serde(rename = "EPAM")]
    Epam,
    #[serde(rename = "EPPO")]
    Eppo,
    #[serde(rename = "EPRR")]
    Eprr,
    #[serde(rename = "EPSD")]
    Epsd,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "IREC")]
    Irec,
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
pub struct BankContactPerson1ChoiceEnum {
    #[serde(rename = "SellrBkCtctPrsn", skip_serializing_if = "Option::is_none")]
    pub sellr_bk_ctct_prsn: Option<ContactIdentification1>,
    #[serde(rename = "BuyrBkCtctPrsn", skip_serializing_if = "Option::is_none")]
    pub buyr_bk_ctct_prsn: Option<ContactIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BankContactPerson1Choice {
    #[serde(flatten)]
    pub value: BankContactPerson1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalAccountIdentification1Code>,
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
pub struct AccountSchemeName1Choice {
    #[serde(flatten)]
    pub value: AccountSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PercentageTolerance1 {
    #[validate]
    #[serde(rename = "PlusPct")]
    pub plus_pct: PercentageRate,
    #[validate]
    #[serde(rename = "MnsPct")]
    pub mns_pct: PercentageRate,
}
#[derive(
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
pub struct Document {
    #[validate]
    #[serde(rename = "InitlBaselnSubmissn")]
    pub initl_baseln_submissn: InitialBaselineSubmissionV05,
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
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "ActiveCurrencyAndAmount")]
    pub value: ActiveCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentTime4Code {
    #[serde(rename = "IREC")]
    Irec,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "EPSD")]
    Epsd,
    #[serde(rename = "EPRR")]
    Eprr,
    #[serde(rename = "EPPO")]
    Eppo,
    #[serde(rename = "EPIN")]
    Epin,
    #[serde(rename = "PRMR")]
    Prmr,
    #[serde(rename = "PRMD")]
    Prmd,
    #[serde(rename = "EPRD")]
    Eprd,
    #[serde(rename = "EPBE")]
    Epbe,
    #[serde(rename = "EMTR")]
    Emtr,
    #[serde(rename = "EMTD")]
    Emtd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InsuranceClauses1Code {
    #[serde(rename = "ICCA")]
    Icca,
    #[serde(rename = "ICCB")]
    Iccb,
    #[serde(rename = "ICCC")]
    Iccc,
    #[serde(rename = "ICAI")]
    Icai,
    #[serde(rename = "IWCC")]
    Iwcc,
    #[serde(rename = "ISCC")]
    Iscc,
    #[serde(rename = "IREC")]
    Irec,
    #[serde(rename = "ICLC")]
    Iclc,
    #[serde(rename = "ISMC")]
    Ismc,
    #[serde(rename = "CMCC")]
    Cmcc,
    #[serde(rename = "IRCE")]
    Irce,
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
pub struct GenericIdentification13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
}
#[derive(
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
pub struct PostalAddress5 {
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "PstCdId", skip_serializing_if = "Option::is_none")]
    pub pst_cd_id: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(
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
pub struct SimpleIdentificationInformation {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCategory1 {
    #[serde(rename = "Tp")]
    pub tp: ProductCategory1Code,
    #[validate]
    #[serde(rename = "Ctgy")]
    pub ctgy: Max35Text,
}
#[derive(
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
pub enum AssuredType1Code {
    #[serde(rename = "BUYE")]
    Buye,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BUBA")]
    Buba,
    #[serde(rename = "SEBA")]
    Seba,
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
pub struct TransportMeans5 {
    #[validate]
    #[serde(rename = "IndvTrnsprt")]
    pub indv_trnsprt: SingleTransport7,
    #[serde(rename = "MltmdlTrnsprt", skip_serializing_if = "Option::is_none")]
    pub mltmdl_trnsprt: Option<MultimodalTransport3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure4Code {
    #[serde(rename = "KGM")]
    Kgm,
    #[serde(rename = "EA")]
    Ea,
    #[serde(rename = "LTN")]
    Ltn,
    #[serde(rename = "MTR")]
    Mtr,
    #[serde(rename = "INH")]
    Inh,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "GLI")]
    Gli,
    #[serde(rename = "GRM")]
    Grm,
    #[serde(rename = "CMT")]
    Cmt,
    #[serde(rename = "MTK")]
    Mtk,
    #[serde(rename = "FOT")]
    Fot,
    #[serde(rename = "1A")]
    X1A,
    #[serde(rename = "INK")]
    Ink,
    #[serde(rename = "FTK")]
    Ftk,
    #[serde(rename = "MIK")]
    Mik,
    #[serde(rename = "ONZ")]
    Onz,
    #[serde(rename = "PTI")]
    Pti,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "QTI")]
    Qti,
    #[serde(rename = "QT")]
    Qt,
    #[serde(rename = "GLL")]
    Gll,
    #[serde(rename = "MMT")]
    Mmt,
    #[serde(rename = "KTM")]
    Ktm,
    #[serde(rename = "YDK")]
    Ydk,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "CMK")]
    Cmk,
    #[serde(rename = "KMK")]
    Kmk,
    #[serde(rename = "MMQ")]
    Mmq,
    #[serde(rename = "CLT")]
    Clt,
    #[serde(rename = "LTR")]
    Ltr,
    #[serde(rename = "LBR")]
    Lbr,
    #[serde(rename = "STN")]
    Stn,
    #[serde(rename = "BLL")]
    Bll,
    #[serde(rename = "BX")]
    Bx,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "CT")]
    Ct,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "INQ")]
    Inq,
    #[serde(rename = "MTQ")]
    Mtq,
    #[serde(rename = "OZI")]
    Ozi,
    #[serde(rename = "OZA")]
    Oza,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "TNE")]
    Tne,
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
pub struct TransportByAir5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "DprtureAirprt", default)]
    pub dprture_airprt: Vec<AirportName1Choice>,
    #[validate(length(min = 1,))]
    #[serde(rename = "DstnAirprt", default)]
    pub dstn_airprt: Vec<AirportName1Choice>,
    #[serde(rename = "AirCrrierNm", skip_serializing_if = "Option::is_none")]
    pub air_crrier_nm: Option<Max70Text>,
    #[serde(rename = "AirCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub air_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCodeOrOther2ChoiceEnum {
    #[serde(rename = "PmtDueDt", skip_serializing_if = "Option::is_none")]
    pub pmt_due_dt: Option<IsoDate>,
    #[serde(rename = "OthrPmtTerms", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_terms: Option<Max140Text>,
    #[serde(rename = "PmtCd", skip_serializing_if = "Option::is_none")]
    pub pmt_cd: Option<PaymentPeriod4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCodeOrOther2Choice {
    #[serde(flatten)]
    pub value: PaymentCodeOrOther2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Adjustment7 {
    #[serde(rename = "Tp")]
    pub tp: AdjustmentType1Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
    #[serde(rename = "Drctn")]
    pub drctn: AdjustmentDirection1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdjustmentType2Code {
    #[serde(rename = "REBA")]
    Reba,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "SURC")]
    Surc,
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
pub struct ChargesType1ChoiceEnum {
    #[serde(rename = "OthrChrgsTp", skip_serializing_if = "Option::is_none")]
    pub othr_chrgs_tp: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ChargeType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargesType1Choice {
    #[serde(flatten)]
    pub value: ChargesType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Location2 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<CountrySubdivision1Choice>,
    #[serde(rename = "Txt", skip_serializing_if = "Option::is_none")]
    pub txt: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct PaymentCodeOrOther1ChoiceEnum {
    #[serde(rename = "PmtCd", skip_serializing_if = "Option::is_none")]
    pub pmt_cd: Option<PaymentPeriod3>,
    #[serde(rename = "OthrPmtTerms", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_terms: Option<Max140Text>,
    #[serde(rename = "PmtDueDt", skip_serializing_if = "Option::is_none")]
    pub pmt_due_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCodeOrOther1Choice {
    #[serde(flatten)]
    pub value: PaymentCodeOrOther1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequiredSubmission6 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Submitr", default)]
    pub submitr: Vec<BicIdentification1>,
    #[validate]
    #[serde(rename = "CertTp")]
    pub cert_tp: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "CertTpDesc")]
    pub cert_tp_desc: Max140Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTerms3 {
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<FinancialInstitutionIdentification4Choice>,
    #[validate]
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: CashAccount24,
}
#[derive(
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
pub struct ChargesDetails3 {
    #[serde(rename = "Tp")]
    pub tp: ChargesType1Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductIdentifier2 {
    #[serde(rename = "Tp")]
    pub tp: ProductIdentifier2Code,
    #[validate]
    #[serde(rename = "Idr")]
    pub idr: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductIdentifier2ChoiceEnum {
    #[serde(rename = "OthrPdctIdr", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_idr: Option<GenericIdentification4>,
    #[serde(rename = "StrdPdctIdr", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_idr: Option<ProductIdentifier2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductIdentifier2Choice {
    #[serde(flatten)]
    pub value: ProductIdentifier2ChoiceEnum,
}
#[derive(
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
pub struct DocumentIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "DtOfIsse")]
    pub dt_of_isse: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentSchedule2ChoiceEnum {
    #[serde(rename = "ShipmntSubSchdl", skip_serializing_if = "Option::is_none")]
    pub shipmnt_sub_schdl: Option<ShipmentDateRange2>,
    #[serde(rename = "ShipmntDtRg", skip_serializing_if = "Option::is_none")]
    pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentSchedule2Choice {
    #[serde(flatten)]
    pub value: ShipmentSchedule2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCashAccountType1Code {
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
pub struct TransportByRail5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PlcOfRct", default)]
    pub plc_of_rct: Vec<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "PlcOfDlvry", default)]
    pub plc_of_dlvry: Vec<Max35Text>,
    #[serde(rename = "RailCrrierNm", skip_serializing_if = "Option::is_none")]
    pub rail_crrier_nm: Option<Max70Text>,
    #[serde(rename = "RailCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub rail_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PhoneNumber {
    #[validate(regex = "PHONE_NUMBER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductIdentifier2Code {
    #[serde(rename = "BINR")]
    Binr,
    #[serde(rename = "COMD")]
    Comd,
    #[serde(rename = "EANC")]
    Eanc,
    #[serde(rename = "HRTR")]
    Hrtr,
    #[serde(rename = "MANI")]
    Mani,
    #[serde(rename = "MODL")]
    Modl,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "QOTA")]
    Qota,
    #[serde(rename = "STYL")]
    Styl,
    #[serde(rename = "SUPI")]
    Supi,
    #[serde(rename = "UPCC")]
    Upcc,
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
pub struct MessageIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdjustmentType1ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<AdjustmentType2Code>,
    #[serde(rename = "OthrAdjstmntTp", skip_serializing_if = "Option::is_none")]
    pub othr_adjstmnt_tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdjustmentType1Choice {
    #[serde(flatten)]
    pub value: AdjustmentType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentTerms5 {
    #[serde(rename = "PmtTerms")]
    pub pmt_terms: PaymentCodeOrOther2Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
}
