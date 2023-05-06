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
    static ref ISO_COUNTRY_SUB_DIVISION_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_6_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_TRANSACTION_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_AMOUNT_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_19_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,19}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_18245_MERCHANT_CATEGORY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_ACCOUNT_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GEOGRAPHIC_POINT_IN_DECIMAL_DEGREES_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(\+|-)?[\d]{1,3}(\.[\d]{1,8})?/(\+|-)?[\d]{1,3}(\.[\d]{1,8})?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_32_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_6_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9\s]{6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_23_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,23}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_K_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,10000}  "#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_9999_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,9999}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_19_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,19}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_12_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTERNAL_ENCRYPTED_ELEMENT_IDENTIFICATION_1_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_16_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,16}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_11_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_RESPONSE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_MESSAGE_REASON_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:cain.003.001.03".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TemporaryServicesLabor1 {
    #[serde(rename = "TmSheetNb", skip_serializing_if = "Option::is_none")]
    pub tm_sheet_nb: Option<Max35Text>,
    #[serde(rename = "WkEndg", skip_serializing_if = "Option::is_none")]
    pub wk_endg: Option<Max10NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Chrg", default)]
    pub chrg: Vec<Amount12>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentAmountDetailsType2Code {
    #[serde(rename = "AFCO")]
    Afco,
    #[serde(rename = "EXPN")]
    Expn,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "FUNA")]
    Funa,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "OTHC")]
    Othc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRNC")]
    Prnc,
    #[serde(rename = "RQST")]
    Rqst,
    #[serde(rename = "TAXX")]
    Taxx,
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
pub struct Address3 {
    #[serde(rename = "AdrLine1", skip_serializing_if = "Option::is_none")]
    pub adr_line_1: Option<Max200Text>,
    #[serde(rename = "AdrLine2", skip_serializing_if = "Option::is_none")]
    pub adr_line_2: Option<Max200Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max200Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max35Text>,
    #[serde(rename = "PstlCd", skip_serializing_if = "Option::is_none")]
    pub pstl_cd: Option<Max35Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max100Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max100Text>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max100Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Transaction147 {
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<Iso8583TransactionTypeCode>,
    #[serde(rename = "TxSubTp", skip_serializing_if = "Option::is_none")]
    pub tx_sub_tp: Option<Max35Text>,
    #[serde(rename = "TxAttr", skip_serializing_if = "Option::is_none")]
    pub tx_attr: Option<TransactionAttribute2Code>,
    #[serde(rename = "OthrTxAttr", skip_serializing_if = "Option::is_none")]
    pub othr_tx_attr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgRsn", default)]
    pub msg_rsn: Vec<Iso8583MessageReasonCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnMsgRsn", default)]
    pub altrn_msg_rsn: Vec<Max35Text>,
    #[serde(rename = "PreAuthstnTmLmt", skip_serializing_if = "Option::is_none")]
    pub pre_authstn_tm_lmt: Option<Max6NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSvc", default)]
    pub addtl_svc: Vec<AdditionalService2>,
    #[serde(rename = "AssoctdDataRef", skip_serializing_if = "Option::is_none")]
    pub assoctd_data_ref: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpclPrgrmmQlfctn", default)]
    pub spcl_prgrmm_qlfctn: Vec<SpecialProgrammeQualification1>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<TransactionIdentification18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DsptData", default)]
    pub dspt_data: Vec<DisputeData3>,
    #[serde(rename = "TxAmts", skip_serializing_if = "Option::is_none")]
    pub tx_amts: Option<TransactionAmounts2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<AdditionalAmounts3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlFee", default)]
    pub addtl_fee: Vec<AdditionalFee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OrgnlAddtlFee", default)]
    pub orgnl_addtl_fee: Vec<AdditionalFee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DpstDtls", default)]
    pub dpst_dtls: Vec<DepositDetails2>,
    #[serde(rename = "FndsSvcs", skip_serializing_if = "Option::is_none")]
    pub fnds_svcs: Option<FundingService2>,
    #[serde(rename = "AcctFr", skip_serializing_if = "Option::is_none")]
    pub acct_fr: Option<AccountDetails3>,
    #[serde(rename = "AcctTo", skip_serializing_if = "Option::is_none")]
    pub acct_to: Option<AccountDetails3>,
    #[serde(rename = "TxDesc", skip_serializing_if = "Option::is_none")]
    pub tx_desc: Option<Max1000Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType17Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ACQP")]
    Acqp,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct PointOfInteractionComponentStatus3 {
    #[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
    pub vrsn_nb: Option<Max256Text>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<PoiComponentStatus1Code>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DepartureOrArrival1 {
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<Max70Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeviceIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DeviceIdentificationType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max70Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoCountrySubDivisionCode {
    #[validate(regex = "ISO_COUNTRY_SUB_DIVISION_CODE_REGEX")]
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
pub struct PaymentTransaction141 {
    #[serde(rename = "PurchsTp", skip_serializing_if = "Option::is_none")]
    pub purchs_tp: Option<FleetPurchaseType1Code>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "DscntTtl", skip_serializing_if = "Option::is_none")]
    pub dscnt_ttl: Option<FleetDiscountTotals1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxTtl", default)]
    pub tax_ttl: Vec<Tax39>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementServiceDate2 {
    #[serde(rename = "ReqdSttlmDt", skip_serializing_if = "Option::is_none")]
    pub reqd_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "DfrrdSttlmInd", skip_serializing_if = "Option::is_none")]
    pub dfrrd_sttlm_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<IsoDate>,
    #[serde(rename = "SttlmTm", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm: Option<IsoTime>,
    #[serde(rename = "SttlmPrd", skip_serializing_if = "Option::is_none")]
    pub sttlm_prd: Option<Max35Text>,
    #[serde(rename = "SttlmCutOffTm", skip_serializing_if = "Option::is_none")]
    pub sttlm_cut_off_tm: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UserInterface1Code {
    #[serde(rename = "CDSP")]
    Cdsp,
    #[serde(rename = "CRCP")]
    Crcp,
    #[serde(rename = "MDSP")]
    Mdsp,
    #[serde(rename = "MRCP")]
    Mrcp,
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
pub struct Max20KText {
    #[validate(length(min = 1, max = 20000,))]
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
pub struct Context12 {
    #[serde(rename = "PtOfSvcCntxt", skip_serializing_if = "Option::is_none")]
    pub pt_of_svc_cntxt: Option<PointOfServiceContext3>,
    #[validate]
    #[serde(rename = "TxCntxt")]
    pub tx_cntxt: TransactionContext7,
    #[validate(length(min = 0,))]
    #[serde(rename = "Vrfctn", default)]
    pub vrfctn: Vec<Verification4>,
    #[serde(rename = "RskCntxt", skip_serializing_if = "Option::is_none")]
    pub rsk_cntxt: Option<RiskContext2>,
    #[serde(rename = "SaleCntxt", skip_serializing_if = "Option::is_none")]
    pub sale_cntxt: Option<SaleContext8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max2NumericText {
    #[validate(regex = "MAX_2_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority3Code {
    #[serde(rename = "URGT")]
    Urgt,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ContentType2Code {
    #[serde(rename = "DATA")]
    Data,
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "DGST")]
    Dgst,
    #[serde(rename = "AUTH")]
    Auth,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedDataElement1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ExternalEncryptedElementIdentification1Code>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<Max35Text>,
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: EncryptedData1Choice,
    #[serde(rename = "ClearTxtDataFrmt", skip_serializing_if = "Option::is_none")]
    pub clear_txt_data_frmt: Option<EncryptedDataFormat1Code>,
    #[serde(
        rename = "OthrClearTxtDataFrmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_clear_txt_data_frmt: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoiCommunicationType2Code {
    #[serde(rename = "BLTH")]
    Blth,
    #[serde(rename = "ETHR")]
    Ethr,
    #[serde(rename = "GPRS")]
    Gprs,
    #[serde(rename = "GSMF")]
    Gsmf,
    #[serde(rename = "PSTN")]
    Pstn,
    #[serde(rename = "RS23")]
    Rs23,
    #[serde(rename = "USBD")]
    Usbd,
    #[serde(rename = "USBH")]
    Usbh,
    #[serde(rename = "WIFI")]
    Wifi,
    #[serde(rename = "WT2G")]
    Wt2G,
    #[serde(rename = "WT3G")]
    Wt3G,
    #[serde(rename = "WT4G")]
    Wt4G,
    #[serde(rename = "WT5G")]
    Wt5G,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm19Code {
    #[serde(rename = "ERS2")]
    Ers2,
    #[serde(rename = "ERS1")]
    Ers1,
    #[serde(rename = "RPSS")]
    Rpss,
    #[serde(rename = "ECC5")]
    Ecc5,
    #[serde(rename = "ECC1")]
    Ecc1,
    #[serde(rename = "ECC4")]
    Ecc4,
    #[serde(rename = "ECC2")]
    Ecc2,
    #[serde(rename = "ECC3")]
    Ecc3,
    #[serde(rename = "ERS3")]
    Ers3,
    #[serde(rename = "ECP2")]
    Ecp2,
    #[serde(rename = "ECP3")]
    Ecp3,
    #[serde(rename = "ECP5")]
    Ecp5,
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
pub struct AmountDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TypeOfAmount20Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max280Text {
    #[validate(length(min = 1, max = 280,))]
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
pub struct Exact1NumericText {
    #[validate(regex = "EXACT_1_NUMERIC_TEXT_REGEX")]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType7Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "PCPT")]
    Pcpt,
    #[serde(rename = "TMGT")]
    Tmgt,
    #[serde(rename = "SALE")]
    Sale,
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
pub struct PassengerTransportSummary2 {
    #[serde(rename = "DocNb", skip_serializing_if = "Option::is_none")]
    pub doc_nb: Option<Max35Text>,
    #[serde(rename = "Rsvatn", skip_serializing_if = "Option::is_none")]
    pub rsvatn: Option<ReservationDetails3>,
    #[serde(rename = "TrvlAuthstnCd", skip_serializing_if = "Option::is_none")]
    pub trvl_authstn_cd: Option<Max70Text>,
    #[serde(rename = "TcktIssr", skip_serializing_if = "Option::is_none")]
    pub tckt_issr: Option<Max35Text>,
    #[serde(rename = "OpnTcktInd", skip_serializing_if = "Option::is_none")]
    pub opn_tckt_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CstmrRef", default)]
    pub cstmr_ref: Vec<CustomerReference1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Pssngr", default)]
    pub pssngr: Vec<Customer8>,
    #[serde(rename = "Dprture", skip_serializing_if = "Option::is_none")]
    pub dprture: Option<DepartureOrArrival1>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TtlAmt", default)]
    pub ttl_amt: Vec<AmountDetails2>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "LltyPrgrmm", skip_serializing_if = "Option::is_none")]
    pub llty_prgrmm: Option<LoyaltyProgramme2>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedAmount22 {
    #[serde(rename = "Tp")]
    pub tp: Iso8583AmountTypeCode,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CrdhldrBllgAmt", skip_serializing_if = "Option::is_none")]
    pub crdhldr_bllg_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "RcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub rcncltn_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount18 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CarRentalServiceType2Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "CstmrNtfdInd", skip_serializing_if = "Option::is_none")]
    pub cstmr_ntfd_ind: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Header62 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: MessageFunction16Code,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max2048Text,
    #[serde(rename = "XchgId", skip_serializing_if = "Option::is_none")]
    pub xchg_id: Option<Max35Text>,
    #[serde(rename = "ReTrnsmssnCntr", skip_serializing_if = "Option::is_none")]
    pub re_trnsmssn_cntr: Option<Max3NumericText>,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[serde(rename = "BtchMgmtInf", skip_serializing_if = "Option::is_none")]
    pub btch_mgmt_inf: Option<BatchManagementInformation1>,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification183,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification183>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TracData", default)]
    pub trac_data: Vec<AdditionalData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionIdentification16 {
    #[validate]
    #[serde(rename = "LclDt")]
    pub lcl_dt: IsoDate,
    #[serde(rename = "LclTm", skip_serializing_if = "Option::is_none")]
    pub lcl_tm: Option<IsoTime>,
    #[serde(rename = "TmZone", skip_serializing_if = "Option::is_none")]
    pub tm_zone: Option<Max70Text>,
    #[serde(rename = "TxRef", skip_serializing_if = "Option::is_none")]
    pub tx_ref: Option<Max35Text>,
    #[serde(rename = "TrnsmssnDtTm", skip_serializing_if = "Option::is_none")]
    pub trnsmssn_dt_tm: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "SysTracAudtNb")]
    pub sys_trac_audt_nb: Max12NumericText,
    #[validate]
    #[serde(rename = "RtrvlRefNb")]
    pub rtrvl_ref_nb: Exact12Text,
    #[serde(rename = "LifeCyclSpprt", skip_serializing_if = "Option::is_none")]
    pub life_cycl_spprt: Option<LifeCycleSupport1Code>,
    #[serde(rename = "LifeCyclTracIdData", skip_serializing_if = "Option::is_none")]
    pub life_cycl_trac_id_data: Option<TransactionLifeCycleIdentification1>,
    #[serde(
        rename = "LifeCyclTracIdMssng",
        skip_serializing_if = "Option::is_none"
    )]
    pub life_cycl_trac_id_mssng: Option<Max70Text>,
    #[serde(rename = "AcqrrRefData", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_data: Option<Max140Text>,
    #[serde(rename = "AcqrrRefNb", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_nb: Option<Max23NumericText>,
    #[serde(rename = "CardIssrRefData", skip_serializing_if = "Option::is_none")]
    pub card_issr_ref_data: Option<Max1000Text>,
    #[serde(rename = "OrgnlDataElmts", skip_serializing_if = "Option::is_none")]
    pub orgnl_data_elmts: Option<OriginalDataElements2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ErrorDetails2 {
    #[serde(rename = "MsgErrTp")]
    pub msg_err_tp: MessageError1Code,
    #[serde(rename = "OthrMsgErrTp", skip_serializing_if = "Option::is_none")]
    pub othr_msg_err_tp: Option<Max35Text>,
    #[serde(rename = "ErrCd", skip_serializing_if = "Option::is_none")]
    pub err_cd: Option<Max35Text>,
    #[serde(rename = "ErrDesc", skip_serializing_if = "Option::is_none")]
    pub err_desc: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DataElmtInErr", default)]
    pub data_elmt_in_err: Vec<Max4000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalInformation29 {
    #[serde(rename = "Rcpt", skip_serializing_if = "Option::is_none")]
    pub rcpt: Option<PartyType22Code>,
    #[serde(rename = "OthrRcpt", skip_serializing_if = "Option::is_none")]
    pub othr_rcpt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Trgt", default)]
    pub trgt: Vec<UserInterface6Code>,
    #[serde(rename = "OthrTrgt", skip_serializing_if = "Option::is_none")]
    pub othr_trgt: Option<Max35Text>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat4Code>,
    #[serde(rename = "OthrFrmt", skip_serializing_if = "Option::is_none")]
    pub othr_frmt: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20KText,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Vehicle2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "NtryMd", skip_serializing_if = "Option::is_none")]
    pub ntry_md: Option<CardDataReading5Code>,
    #[validate]
    #[serde(rename = "Data")]
    pub data: Max35Text,
}
#[derive(
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
pub struct ApprovalEntity2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp")]
    pub tp: PartyType26Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType9Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max6NumericText {
    #[validate(regex = "MAX_6_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LodgingService1Code {
    #[serde(rename = "ACCO")]
    Acco,
    #[serde(rename = "AUDI")]
    Audi,
    #[serde(rename = "BANQ")]
    Banq,
    #[serde(rename = "BREK")]
    Brek,
    #[serde(rename = "BUSS")]
    Buss,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "EARA")]
    Eara,
    #[serde(rename = "EARD")]
    Eard,
    #[serde(rename = "ENTR")]
    Entr,
    #[serde(rename = "FCAA")]
    Fcaa,
    #[serde(rename = "GAME")]
    Game,
    #[serde(rename = "GARA")]
    Gara,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "HEAL")]
    Heal,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "LAUN")]
    Laun,
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "MINI")]
    Mini,
    #[serde(rename = "NOSH")]
    Nosh,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PARK")]
    Park,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "RMSE")]
    Rmse,
    #[serde(rename = "SPAS")]
    Spas,
    #[serde(rename = "THRD")]
    Thrd,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "VODS")]
    Vods,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageError1Code {
    #[serde(rename = "IDEF")]
    Idef,
    #[serde(rename = "IDEL")]
    Idel,
    #[serde(rename = "IDEV")]
    Idev,
    #[serde(rename = "INME")]
    Inme,
    #[serde(rename = "INMF")]
    Inmf,
    #[serde(rename = "MEPE")]
    Mepe,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRVE")]
    Prve,
    #[serde(rename = "RDEM")]
    Rdem,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "UDFD")]
    Udfd,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "ITDE")]
    Itde,
    #[serde(rename = "DUME")]
    Dume,
    #[serde(rename = "IDWM")]
    Idwm,
    #[serde(rename = "IDRM")]
    Idrm,
    #[serde(rename = "IBAT")]
    Ibat,
    #[serde(rename = "ICOL")]
    Icol,
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
pub struct OnBoardDiagnostics1 {
    #[serde(rename = "NgnIdleTm", skip_serializing_if = "Option::is_none")]
    pub ngn_idle_tm: Option<Max10NumberFraction2>,
    #[serde(rename = "NgnTtlIdleTm", skip_serializing_if = "Option::is_none")]
    pub ngn_ttl_idle_tm: Option<Max10NumberFraction2>,
    #[serde(rename = "NgnHrs", skip_serializing_if = "Option::is_none")]
    pub ngn_hrs: Option<Max10NumberFraction2>,
    #[serde(rename = "NgnTtlTm", skip_serializing_if = "Option::is_none")]
    pub ngn_ttl_tm: Option<Max6NumberFraction2>,
    #[serde(rename = "NgnLd", skip_serializing_if = "Option::is_none")]
    pub ngn_ld: Option<Max12NumericText>,
    #[serde(rename = "NgnRPM", skip_serializing_if = "Option::is_none")]
    pub ngn_rpm: Option<Max5NumericText>,
    #[serde(rename = "NgnOilTmprtr", skip_serializing_if = "Option::is_none")]
    pub ngn_oil_tmprtr: Option<Max6NumberFraction2>,
    #[serde(rename = "NgnOilPrssr", skip_serializing_if = "Option::is_none")]
    pub ngn_oil_prssr: Option<Max3NumericText>,
    #[serde(rename = "NgnOilLifeRmng", skip_serializing_if = "Option::is_none")]
    pub ngn_oil_life_rmng: Option<Max3NumericText>,
    #[serde(rename = "ChckNgnWrngSts", skip_serializing_if = "Option::is_none")]
    pub chck_ngn_wrng_sts: Option<Max35Text>,
    #[serde(rename = "FuelTankLvlStart", skip_serializing_if = "Option::is_none")]
    pub fuel_tank_lvl_start: Option<Max4NumericText>,
    #[serde(rename = "FuelGaugeLvl", skip_serializing_if = "Option::is_none")]
    pub fuel_gauge_lvl: Option<Max4NumericText>,
    #[serde(rename = "FuelEcnmy", skip_serializing_if = "Option::is_none")]
    pub fuel_ecnmy: Option<Max6NumberFraction2>,
    #[serde(rename = "RfrgrtnHrs", skip_serializing_if = "Option::is_none")]
    pub rfrgrtn_hrs: Option<Max10NumberFraction2>,
    #[serde(rename = "RfrgrtnTmprtr", skip_serializing_if = "Option::is_none")]
    pub rfrgrtn_tmprtr: Option<Max6NumberFraction2>,
    #[serde(rename = "CoolntTmprtr", skip_serializing_if = "Option::is_none")]
    pub coolnt_tmprtr: Option<Max6NumberFraction2>,
    #[serde(rename = "BttryVltg", skip_serializing_if = "Option::is_none")]
    pub bttry_vltg: Option<Max4NumericText>,
    #[serde(rename = "HardBrakg", skip_serializing_if = "Option::is_none")]
    pub hard_brakg: Option<Max4NumericText>,
    #[serde(rename = "HardAcclrtn", skip_serializing_if = "Option::is_none")]
    pub hard_acclrtn: Option<Max4NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VerificationValue1ChoiceEnum {
    #[serde(rename = "PINData", skip_serializing_if = "Option::is_none")]
    pub pin_data: Option<PinData1>,
    #[serde(rename = "BinryVal", skip_serializing_if = "Option::is_none")]
    pub binry_val: Option<Max5000Binary>,
    #[serde(rename = "HexBinryVal", skip_serializing_if = "Option::is_none")]
    pub hex_binry_val: Option<Max9999HexBinaryText>,
    #[serde(rename = "TxtVal", skip_serializing_if = "Option::is_none")]
    pub txt_val: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VerificationValue1Choice {
    #[serde(flatten)]
    pub value: VerificationValue1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
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
pub struct RentalRate1 {
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<PeriodUnit3Code>,
    #[serde(rename = "OthrPrd", skip_serializing_if = "Option::is_none")]
    pub othr_prd: Option<Max35Text>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "PrdCnt", skip_serializing_if = "Option::is_none")]
    pub prd_cnt: Option<Max4NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Kek6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier6,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification28,
    #[serde(rename = "NcrptdKey", skip_serializing_if = "Option::is_none")]
    pub ncrptd_key: Option<Max500Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PointOfInteractionComponent13 {
    #[serde(rename = "Tp")]
    pub tp: PoiComponentType5Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: PointOfInteractionComponentIdentification3,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<PointOfInteractionComponentStatus3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "StdCmplc", default)]
    pub std_cmplc: Vec<GenericIdentification48>,
    #[serde(rename = "Chrtcs", skip_serializing_if = "Option::is_none")]
    pub chrtcs: Option<PointOfInteractionComponentCharacteristics4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Assmnt", default)]
    pub assmnt: Vec<PointOfInteractionComponentIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification21 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm16Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IccFallbackReason1Code {
    #[serde(rename = "CIIA")]
    Ciia,
    #[serde(rename = "EDIP")]
    Edip,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "TERI")]
    Teri,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDepositType1Code {
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "ENVL")]
    Envl,
    #[serde(rename = "CHEC")]
    Chec,
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
pub struct FinancialInstitution7 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<Max35Text>,
    #[serde(rename = "CstmrSvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_svc: Option<Max35Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max256Text>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeviceOperatingSystemType1Code {
    #[serde(rename = "DROI")]
    Droi,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "IOSS")]
    Ioss,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "TIZN")]
    Tizn,
    #[serde(rename = "WIND")]
    Wind,
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
pub struct Product7 {
    #[serde(rename = "PdctCd", skip_serializing_if = "Option::is_none")]
    pub pdct_cd: Option<Max70Text>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VehicleRentalCustomer2 {
    #[validate]
    #[serde(rename = "RntrNm")]
    pub rntr_nm: Max70Text,
    #[serde(rename = "CorpNm", skip_serializing_if = "Option::is_none")]
    pub corp_nm: Option<Max70Text>,
    #[validate]
    #[serde(rename = "CorpIdr")]
    pub corp_idr: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<CustomerAssigner1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmryDrvr", default)]
    pub pmry_drvr: Vec<DriverInParty2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlDrvr", default)]
    pub addtl_drvr: Vec<DriverInParty2>,
    #[serde(rename = "LltyPrgrmm", skip_serializing_if = "Option::is_none")]
    pub llty_prgrmm: Option<LoyaltyProgramme2>,
}
#[derive(
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
pub struct Customer8 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification208,
    #[serde(rename = "CstmrFileRefNb", skip_serializing_if = "Option::is_none")]
    pub cstmr_file_ref_nb: Option<Max70Text>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<Max2NumericText>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionDestination1Code {
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "MOBL")]
    Mobl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PECR")]
    Pecr,
    #[serde(rename = "POFS")]
    Pofs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType32Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "ISUR")]
    Isur,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PlanOwner1Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ISSR")]
    Issr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct ProtectedData1 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType3Code,
    #[serde(rename = "EnvlpdData", skip_serializing_if = "Option::is_none")]
    pub envlpd_data: Option<EnvelopedData6>,
    #[serde(rename = "NcrptdData", skip_serializing_if = "Option::is_none")]
    pub ncrptd_data: Option<EncryptedData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RiskAssessment2 {
    #[serde(rename = "RskAssmntNtty", skip_serializing_if = "Option::is_none")]
    pub rsk_assmnt_ntty: Option<PartyIdentification200>,
    #[serde(rename = "RskAssmntTp", skip_serializing_if = "Option::is_none")]
    pub rsk_assmnt_tp: Option<Max35Text>,
    #[serde(rename = "HghRskTx", skip_serializing_if = "Option::is_none")]
    pub hgh_rsk_tx: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RskCond", default)]
    pub rsk_cond: Vec<AdditionalData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlRskData", default)]
    pub addtl_rsk_data: Vec<AdditionalRiskData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RcmmnddActn", default)]
    pub rcmmndd_actn: Vec<ActionType8Code>,
    #[serde(rename = "OthrRcmmnddActn", skip_serializing_if = "Option::is_none")]
    pub othr_rcmmndd_actn: Option<Max35Text>,
    #[serde(rename = "RcmmnddActnDtls", skip_serializing_if = "Option::is_none")]
    pub rcmmndd_actn_dtls: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification26 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Algorithm5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Vehicle5 {
    #[serde(rename = "VhclNb", skip_serializing_if = "Option::is_none")]
    pub vhcl_nb: Option<Max35NumericText>,
    #[serde(rename = "VhclIdNb", skip_serializing_if = "Option::is_none")]
    pub vhcl_id_nb: Option<Max35NumericText>,
    #[serde(rename = "FleetNb", skip_serializing_if = "Option::is_none")]
    pub fleet_nb: Option<Max10Text>,
    #[serde(rename = "SubFleetNb", skip_serializing_if = "Option::is_none")]
    pub sub_fleet_nb: Option<Max35Text>,
    #[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
    pub unit_nb: Option<Max35NumericText>,
    #[serde(rename = "TrlrNb", skip_serializing_if = "Option::is_none")]
    pub trlr_nb: Option<Max35NumericText>,
    #[serde(rename = "VhclTag", skip_serializing_if = "Option::is_none")]
    pub vhcl_tag: Option<Max35Text>,
    #[serde(rename = "VhclTagNtryMd", skip_serializing_if = "Option::is_none")]
    pub vhcl_tag_ntry_md: Option<CardDataReading5Code>,
    #[serde(rename = "RplcmntVhclInd", skip_serializing_if = "Option::is_none")]
    pub rplcmnt_vhcl_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "Odmtr", skip_serializing_if = "Option::is_none")]
    pub odmtr: Option<DecimalNumber>,
    #[serde(rename = "Hbmtr", skip_serializing_if = "Option::is_none")]
    pub hbmtr: Option<DecimalNumber>,
    #[serde(rename = "MntncId", skip_serializing_if = "Option::is_none")]
    pub mntnc_id: Option<Max35Text>,
    #[serde(rename = "OnBrdDgnstcs", skip_serializing_if = "Option::is_none")]
    pub on_brd_dgnstcs: Option<OnBoardDiagnostics1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlVhclData", default)]
    pub addtl_vhcl_data: Vec<Vehicle2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount13 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
}
#[derive(
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
pub struct RentalDetails2 {
    #[serde(rename = "RntlId", skip_serializing_if = "Option::is_none")]
    pub rntl_id: Option<Max70Text>,
    #[serde(rename = "RntlDtTm", skip_serializing_if = "Option::is_none")]
    pub rntl_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "RntlStart", skip_serializing_if = "Option::is_none")]
    pub rntl_start: Option<ServiceStartEnd2>,
    #[serde(rename = "RntlRtr", skip_serializing_if = "Option::is_none")]
    pub rntl_rtr: Option<ServiceStartEnd2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RntlTmPrd", default)]
    pub rntl_tm_prd: Vec<PeriodUnit2Code>,
    #[serde(rename = "TmPrdUnit", skip_serializing_if = "Option::is_none")]
    pub tm_prd_unit: Option<Max4NumericText>,
    #[serde(rename = "TmPrdRate", skip_serializing_if = "Option::is_none")]
    pub tm_prd_rate: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact12Text {
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
pub struct Max4NumericText {
    #[validate(regex = "MAX_4_NUMERIC_TEXT_REGEX")]
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
pub struct Jurisdiction2 {
    #[serde(rename = "DmstInd", skip_serializing_if = "Option::is_none")]
    pub dmst_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DmstQlfctn", skip_serializing_if = "Option::is_none")]
    pub dmst_qlfctn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10PositiveNumber {
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
pub struct Parameter11 {
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: Algorithm16Code,
    #[validate]
    #[serde(rename = "MskGnrtrAlgo")]
    pub msk_gnrtr_algo: AlgorithmIdentification12,
    #[validate]
    #[serde(rename = "SaltLngth")]
    pub salt_lngth: Number,
    #[serde(rename = "TrlrFld", skip_serializing_if = "Option::is_none")]
    pub trlr_fld: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification263 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max11NumericText,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CustomerDeviceType2Code {
    #[serde(rename = "MOBL")]
    Mobl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PECR")]
    Pecr,
    #[serde(rename = "TBLT")]
    Tblt,
    #[serde(rename = "NSCR")]
    Nscr,
    #[serde(rename = "SECR")]
    Secr,
    #[serde(rename = "EMBD")]
    Embd,
    #[serde(rename = "VHCL")]
    Vhcl,
    #[serde(rename = "WRBL")]
    Wrbl,
    #[serde(rename = "WATC")]
    Watc,
    #[serde(rename = "GAMB")]
    Gamb,
    #[serde(rename = "JEWL")]
    Jewl,
    #[serde(rename = "KFOB")]
    Kfob,
    #[serde(rename = "STIC")]
    Stic,
    #[serde(rename = "UNKW")]
    Unkw,
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
pub struct Token2 {
    #[serde(rename = "PmtTkn", skip_serializing_if = "Option::is_none")]
    pub pmt_tkn: Option<Max19NumericText>,
    #[serde(rename = "TknXpryDt", skip_serializing_if = "Option::is_none")]
    pub tkn_xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "TknRqstrId", skip_serializing_if = "Option::is_none")]
    pub tkn_rqstr_id: Option<Max11NumericText>,
    #[serde(rename = "TknAssrncData", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_data: Option<Max140Text>,
    #[serde(rename = "TknAssrncMtd", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_mtd: Option<Max2NumericText>,
    #[serde(rename = "TknInittdInd", skip_serializing_if = "Option::is_none")]
    pub tkn_inittd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "StorgLctn", skip_serializing_if = "Option::is_none")]
    pub storg_lctn: Option<StorageLocation1Code>,
    #[serde(rename = "OthrStorgLctn", skip_serializing_if = "Option::is_none")]
    pub othr_storg_lctn: Option<Max35Text>,
    #[serde(rename = "PrtcnMtd", skip_serializing_if = "Option::is_none")]
    pub prtcn_mtd: Option<ProtectionMethod1Code>,
    #[serde(rename = "OthrPrtcnMtd", skip_serializing_if = "Option::is_none")]
    pub othr_prtcn_mtd: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedData1 {
    #[serde(rename = "Ctrl", skip_serializing_if = "Option::is_none")]
    pub ctrl: Option<Exact1HexBinaryText>,
    #[serde(rename = "KeySetIdr", skip_serializing_if = "Option::is_none")]
    pub key_set_idr: Option<Max8NumericText>,
    #[serde(rename = "DrvdInf", skip_serializing_if = "Option::is_none")]
    pub drvd_inf: Option<Max32HexBinaryText>,
    #[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
    pub algo: Option<Max2NumericText>,
    #[serde(rename = "KeyLngth", skip_serializing_if = "Option::is_none")]
    pub key_lngth: Option<Max4NumericText>,
    #[serde(rename = "KeyPrtcn", skip_serializing_if = "Option::is_none")]
    pub key_prtcn: Option<Max2NumericText>,
    #[serde(rename = "KeyIndx", skip_serializing_if = "Option::is_none")]
    pub key_indx: Option<Max5NumericText>,
    #[serde(rename = "PddgMtd", skip_serializing_if = "Option::is_none")]
    pub pddg_mtd: Option<Max2NumericText>,
    #[serde(rename = "NcrptdDataFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_data_frmt: Option<Max2NumericText>,
    #[validate(length(min = 1,))]
    #[serde(rename = "NcrptdDataElmt", default)]
    pub ncrptd_data_elmt: Vec<EncryptedDataElement1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max6NumberFraction2 {
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
pub struct Iso8583TransactionTypeCode {
    #[validate(regex = "ISO_8583_TRANSACTION_TYPE_CODE_REGEX")]
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
pub struct Lodging3 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<LodgingSummary2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<LodgingLineItem2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso8583AmountTypeCode {
    #[validate(regex = "ISO_8583_AMOUNT_TYPE_CODE_REGEX")]
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
pub struct CustomerReference1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Dtl", skip_serializing_if = "Option::is_none")]
    pub dtl: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max19NumericText {
    #[validate(regex = "MAX_19_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType18Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CSCH")]
    Csch,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct CardWritingCapabilities1 {
    #[serde(rename = "Cpblty")]
    pub cpblty: CardDataWriting1Code,
    #[serde(rename = "OthrCpblty", skip_serializing_if = "Option::is_none")]
    pub othr_cpblty: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso3NumericCountryCode {
    #[validate(regex = "ISO_3_NUMERIC_COUNTRY_CODE_REGEX")]
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
pub struct PlainCardData21 {
    #[serde(rename = "DrvrOrVhclId", skip_serializing_if = "Option::is_none")]
    pub drvr_or_vhcl_id: Option<Max20Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlCardData", default)]
    pub addtl_card_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm20Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
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
pub struct ExchangeRateDetail1 {
    #[serde(rename = "EndPt", skip_serializing_if = "Option::is_none")]
    pub end_pt: Option<Endpoint1Code>,
    #[serde(rename = "OthrEndPt", skip_serializing_if = "Option::is_none")]
    pub othr_end_pt: Option<Max35Text>,
    #[serde(rename = "CntrCcyCd", skip_serializing_if = "Option::is_none")]
    pub cntr_ccy_cd: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "BaseCcyCd", skip_serializing_if = "Option::is_none")]
    pub base_ccy_cd: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<BaseOne25Rate>,
    #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<ExchangeRateType2Code>,
    #[serde(rename = "OthrRateTp", skip_serializing_if = "Option::is_none")]
    pub othr_rate_tp: Option<Max35Text>,
    #[serde(rename = "AgrmtTp", skip_serializing_if = "Option::is_none")]
    pub agrmt_tp: Option<ExchangeRateAgreementType1Code>,
    #[serde(rename = "OthrAgrmtTp", skip_serializing_if = "Option::is_none")]
    pub othr_agrmt_tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Location4 {
    #[serde(rename = "LctnCd", skip_serializing_if = "Option::is_none")]
    pub lctn_cd: Option<Max35Text>,
    #[serde(rename = "LctnNm", skip_serializing_if = "Option::is_none")]
    pub lctn_nm: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "LclTmZone", skip_serializing_if = "Option::is_none")]
    pub lcl_tm_zone: Option<Max70Text>,
    #[serde(rename = "LclCcy", skip_serializing_if = "Option::is_none")]
    pub lcl_ccy: Option<Iso3NumericCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoMax3ACountryCode {
    #[validate(regex = "ISO_MAX_3_A_COUNTRY_CODE_REGEX")]
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
pub struct TemporaryServicesCompany2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification258>,
    #[serde(rename = "Sprvsr", skip_serializing_if = "Option::is_none")]
    pub sprvsr: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Customer6 {
    #[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
    pub acct_nb: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max512Text {
    #[validate(length(min = 1, max = 512,))]
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
pub struct ClaimInformation1 {
    #[serde(rename = "ClmCrdntls", skip_serializing_if = "Option::is_none")]
    pub clm_crdntls: Option<Max500Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SaleSummary1 {
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "LltyPrgrmm", skip_serializing_if = "Option::is_none")]
    pub llty_prgrmm: Option<LoyaltyProgramme2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardData7 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Max19NumericText>,
    #[serde(rename = "PrtctdPANInd", skip_serializing_if = "Option::is_none")]
    pub prtctd_pan_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<IsoYearMonth>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "PmtAcctRef", skip_serializing_if = "Option::is_none")]
    pub pmt_acct_ref: Option<Max35Text>,
    #[serde(rename = "PANAcctRg", skip_serializing_if = "Option::is_none")]
    pub pan_acct_rg: Option<Max19NumericText>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<Iso3NumericCountryCode>,
    #[serde(rename = "CardPdctTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_tp: Option<Max35Text>,
    #[serde(rename = "CardPdctSubTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_sub_tp: Option<Max35Text>,
    #[serde(rename = "CardPrtflIdr", skip_serializing_if = "Option::is_none")]
    pub card_prtfl_idr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlCardData", default)]
    pub addtl_card_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementServiceMode1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso18245MerchantCategoryCode {
    #[validate(regex = "ISO_18245_MERCHANT_CATEGORY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransportType1Code {
    #[serde(rename = "AIRR")]
    Airr,
    #[serde(rename = "BUSS")]
    Buss,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "RAIL")]
    Rail,
    #[serde(rename = "SHIP")]
    Ship,
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
pub struct TelecomServicesSummary2 {
    #[serde(rename = "Cstmr", skip_serializing_if = "Option::is_none")]
    pub cstmr: Option<Customer6>,
    #[serde(rename = "BllgStmtPrdStart", skip_serializing_if = "Option::is_none")]
    pub bllg_stmt_prd_start: Option<IsoDate>,
    #[serde(rename = "BllgStmtPrdEnd", skip_serializing_if = "Option::is_none")]
    pub bllg_stmt_prd_end: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BllgEvt", default)]
    pub bllg_evt: Vec<Amount19>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TtlTax", default)]
    pub ttl_tax: Vec<Tax39>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DepositDetails2 {
    #[serde(rename = "Tp")]
    pub tp: CardDepositType1Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<Amount16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max500Binary {
    #[validate(length(min = 1, max = 500,), regex = "MAX_500_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading5Code {
    #[serde(rename = "TAGC")]
    Tagc,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "BRCD")]
    Brcd,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "CICC")]
    Cicc,
    #[serde(rename = "DFLE")]
    Dfle,
    #[serde(rename = "CTLS")]
    Ctls,
    #[serde(rename = "ECTL")]
    Ectl,
    #[serde(rename = "CDFL")]
    Cdfl,
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
pub struct AlgorithmIdentification19 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm7Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso8583AccountTypeCode {
    #[validate(regex = "ISO_8583_ACCOUNT_TYPE_CODE_REGEX")]
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
pub struct TelecomServices2 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<TelecomServicesSummary2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<TelecomServicesLineItem2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max99Text {
    #[validate(length(min = 1, max = 99,))]
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
pub struct CardholderName2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max70Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max70Text>,
    #[serde(rename = "LastNm", skip_serializing_if = "Option::is_none")]
    pub last_nm: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min2Max3NumericText {
    #[validate(regex = "MIN_2_MAX_3_NUMERIC_TEXT_REGEX")]
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
pub struct SpecialProgrammeDetails1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedContent5 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "CnttNcrptnAlgo")]
    pub cntt_ncrptn_algo: AlgorithmIdentification25,
    #[validate(length(min = 1,))]
    #[serde(rename = "NcrptdDataElmt", default)]
    pub ncrptd_data_elmt: Vec<EncryptedDataElement1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardData6 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Max19NumericText>,
    #[serde(rename = "PrtctdPANInd", skip_serializing_if = "Option::is_none")]
    pub prtctd_pan_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<IsoYearMonth>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "SvcCd", skip_serializing_if = "Option::is_none")]
    pub svc_cd: Option<Exact3NumericText>,
    #[serde(rename = "Trck1", skip_serializing_if = "Option::is_none")]
    pub trck_1: Option<Max76Text>,
    #[serde(rename = "Trck2", skip_serializing_if = "Option::is_none")]
    pub trck_2: Option<Track2Data1Choice>,
    #[serde(rename = "Trck3", skip_serializing_if = "Option::is_none")]
    pub trck_3: Option<Max104Text>,
    #[serde(rename = "PmtAcctRef", skip_serializing_if = "Option::is_none")]
    pub pmt_acct_ref: Option<Max35Text>,
    #[serde(rename = "PANAcctRg", skip_serializing_if = "Option::is_none")]
    pub pan_acct_rg: Option<Max19NumericText>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<Iso3NumericCountryCode>,
    #[serde(rename = "CardPdctTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_tp: Option<Max35Text>,
    #[serde(rename = "CardPdctSubTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_sub_tp: Option<Max35Text>,
    #[serde(rename = "CardPrtflIdr", skip_serializing_if = "Option::is_none")]
    pub card_prtfl_idr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlCardData", default)]
    pub addtl_card_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardProgramme2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "CardPrgrmmPropsd", default)]
    pub card_prgrmm_propsd: Vec<CardProgrammeMode2>,
    #[serde(rename = "CardPrgrmmApld", skip_serializing_if = "Option::is_none")]
    pub card_prgrmm_apld: Option<CardProgrammeMode3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FleetSummary2 {
    #[serde(rename = "Drvr", skip_serializing_if = "Option::is_none")]
    pub drvr: Option<Driver2>,
    #[serde(rename = "Vhcl", skip_serializing_if = "Option::is_none")]
    pub vhcl: Option<Vehicle5>,
    #[serde(rename = "DrvrOrVhclCard", skip_serializing_if = "Option::is_none")]
    pub drvr_or_vhcl_card: Option<PlainCardData21>,
    #[serde(rename = "CardFuelPrmptCd", skip_serializing_if = "Option::is_none")]
    pub card_fuel_prmpt_cd: Option<Max1Number>,
    #[serde(rename = "AgtFuelPrmptCd", skip_serializing_if = "Option::is_none")]
    pub agt_fuel_prmpt_cd: Option<Max35Text>,
    #[serde(rename = "TripInf", skip_serializing_if = "Option::is_none")]
    pub trip_inf: Option<TripInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LclAmnty", default)]
    pub lcl_amnty: Vec<LocalAmenity1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxRltdData", default)]
    pub tx_rltd_data: Vec<PaymentTransaction141>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<AdditionalInformation19>,
}
#[derive(
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
pub struct AlgorithmIdentification24 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm18Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter12>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency18Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FRTN")]
    Frtn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "TWWK")]
    Twwk,
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
pub struct GeographicPointInDecimalDegrees {
    #[validate(length(max = 27,), regex = "GEOGRAPHIC_POINT_IN_DECIMAL_DEGREES_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AttributeType1Code {
    #[serde(rename = "CNAT")]
    Cnat,
    #[serde(rename = "LATT")]
    Latt,
    #[serde(rename = "OATT")]
    Oatt,
    #[serde(rename = "OUAT")]
    Ouat,
    #[serde(rename = "CATT")]
    Catt,
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
pub struct FinancialInitiationV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header62,
    #[validate]
    #[serde(rename = "Body")]
    pub body: FinancialInitiation2<A>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType20>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max32HexBinaryText {
    #[validate(regex = "MAX_32_HEX_BINARY_TEXT_REGEX")]
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
pub struct DisputeReference1 {
    #[serde(rename = "AssgnrNtty", skip_serializing_if = "Option::is_none")]
    pub assgnr_ntty: Option<PartyType32Code>,
    #[serde(rename = "OthrAssgnrNtty", skip_serializing_if = "Option::is_none")]
    pub othr_assgnr_ntty: Option<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "DsptId", default)]
    pub dspt_id: Vec<DisputeIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LodgingLineItem2 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<IsoTime>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<LodgingService1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "PstChckOutInd", skip_serializing_if = "Option::is_none")]
    pub pst_chck_out_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "UnitAmt", skip_serializing_if = "Option::is_none")]
    pub unit_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
    pub sub_ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Endpoint1Code {
    #[serde(rename = "DEST")]
    Dest,
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LegalStructure1Code {
    #[serde(rename = "STAE")]
    Stae,
    #[serde(rename = "PVIN")]
    Pvin,
    #[serde(rename = "MUNI")]
    Muni,
    #[serde(rename = "CNTY")]
    Cnty,
    #[serde(rename = "NATI")]
    Nati,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType9Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ACQP")]
    Acqp,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "CSCH")]
    Csch,
    #[serde(rename = "SCHP")]
    Schp,
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
pub struct GracePeriod2 {
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<Max3NumericText>,
    #[serde(rename = "UnitTp", skip_serializing_if = "Option::is_none")]
    pub unit_tp: Option<GracePeriodUnitType1Code>,
    #[serde(rename = "OthrUnitTp", skip_serializing_if = "Option::is_none")]
    pub othr_unit_tp: Option<Max35Text>,
    #[serde(
        rename = "CstmrSelctdGracePrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub cstmr_selctd_grace_prd: Option<bool>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType11Code {
    #[serde(rename = "CNTI")]
    Cnti,
    #[serde(rename = "CNIS")]
    Cnis,
    #[serde(rename = "CNTA")]
    Cnta,
    #[serde(rename = "CNAS")]
    Cnas,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "CHDV")]
    Chdv,
    #[serde(rename = "VIPM")]
    Vipm,
    #[serde(rename = "TRCK")]
    Trck,
    #[serde(rename = "TRXR")]
    Trxr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SIGN")]
    Sign,
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
pub struct KeyTransport5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient5Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification19,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max5000Binary,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OutputFormat4Code {
    #[serde(rename = "FLNM")]
    Flnm,
    #[serde(rename = "MREF")]
    Mref,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SMSI")]
    Smsi,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "URLI")]
    Urli,
    #[serde(rename = "HTML")]
    Html,
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
pub struct FinancialInitiation2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: Environment17,
    #[validate]
    #[serde(rename = "Cntxt")]
    pub cntxt: Context10,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: Transaction145,
    #[validate]
    #[serde(rename = "OrgnlTx")]
    pub orgnl_tx: OriginalTransaction2,
    #[serde(rename = "AdddmData", skip_serializing_if = "Option::is_none")]
    pub adddm_data: Option<AddendumData3>,
    #[serde(rename = "PrcgRslt", skip_serializing_if = "Option::is_none")]
    pub prcg_rslt: Option<ProcessingResult16>,
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max10KHexBinaryText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtctdData", default)]
    pub prtctd_data: Vec<ProtectedData1>,
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
pub struct LanguageCode {
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
pub struct AdditionalRiskData1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max10KText,
}
#[derive(
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
    #[serde(rename = "FinInitn")]
    pub fin_initn: FinancialInitiationV03<A>,
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
pub struct Max5000Binary {
    #[validate(length(min = 1, max = 5000,), regex = "MAX_5000_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType20Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct SpecialConditions1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter10 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat2Code>,
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm16Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification18>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType10Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "DEAC")]
    Deac,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "FUPD")]
    Fupd,
    #[serde(rename = "PRNT")]
    Prnt,
    #[serde(rename = "SNDM")]
    Sndm,
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
pub struct Iso2ALanguageCode {
    #[validate(regex = "ISO_2_A_LANGUAGE_CODE_REGEX")]
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
pub struct MacData1 {
    #[validate]
    #[serde(rename = "Ctrl")]
    pub ctrl: Exact1HexBinaryText,
    #[validate]
    #[serde(rename = "KeySetIdr")]
    pub key_set_idr: Max8NumericText,
    #[serde(rename = "DrvdInf", skip_serializing_if = "Option::is_none")]
    pub drvd_inf: Option<Max32HexBinaryText>,
    #[validate]
    #[serde(rename = "Algo")]
    pub algo: Max2NumericText,
    #[serde(rename = "KeyLngth", skip_serializing_if = "Option::is_none")]
    pub key_lngth: Option<Max4NumericText>,
    #[serde(rename = "KeyPrtcn", skip_serializing_if = "Option::is_none")]
    pub key_prtcn: Option<Max2NumericText>,
    #[serde(rename = "KeyIndx", skip_serializing_if = "Option::is_none")]
    pub key_indx: Option<Max5NumericText>,
    #[serde(rename = "PddgMtd", skip_serializing_if = "Option::is_none")]
    pub pddg_mtd: Option<Max2NumericText>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max32HexBinaryText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10KText {
    #[validate(length(min = 1, max = 10000,))]
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
pub struct LocalData7 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "CrdhldrNm", skip_serializing_if = "Option::is_none")]
    pub crdhldr_nm: Option<CardholderName2>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SponsoredMerchant2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification262,
    #[serde(rename = "CmonNm", skip_serializing_if = "Option::is_none")]
    pub cmon_nm: Option<Max140Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max256Text>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificateIssuer1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "RltvDstngshdNm", default)]
    pub rltv_dstngshd_nm: Vec<RelativeDistinguishedName1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FleetData4 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<FleetSummary2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<FleetLineItem4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading9Code {
    #[serde(rename = "UNKW")]
    Unkw,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "CAMR")]
    Camr,
    #[serde(rename = "KEEN")]
    Keen,
    #[serde(rename = "ICPY")]
    Icpy,
    #[serde(rename = "OPTC")]
    Optc,
    #[serde(rename = "CDFL")]
    Cdfl,
    #[serde(rename = "MBNK")]
    Mbnk,
    #[serde(rename = "TOKN")]
    Tokn,
    #[serde(rename = "ICCY")]
    Iccy,
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
pub struct Max200Text {
    #[validate(length(min = 1, max = 200,))]
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
pub struct TripInformation1 {
    #[serde(rename = "TripNb", skip_serializing_if = "Option::is_none")]
    pub trip_nb: Option<Max35Text>,
    #[serde(rename = "JobNb", skip_serializing_if = "Option::is_none")]
    pub job_nb: Option<Max10Text>,
    #[serde(rename = "WorkOrdr", skip_serializing_if = "Option::is_none")]
    pub work_ordr: Option<Max70Text>,
    #[serde(rename = "InvcNb", skip_serializing_if = "Option::is_none")]
    pub invc_nb: Option<Max70Text>,
    #[serde(rename = "BllgId", skip_serializing_if = "Option::is_none")]
    pub bllg_id: Option<Max70Text>,
    #[serde(rename = "CtrlNb", skip_serializing_if = "Option::is_none")]
    pub ctrl_nb: Option<Max35Text>,
    #[serde(rename = "DlvryTcktNb", skip_serializing_if = "Option::is_none")]
    pub dlvry_tckt_nb: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm7Code {
    #[serde(rename = "ERSA")]
    Ersa,
    #[serde(rename = "RSAO")]
    Rsao,
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
pub struct ImpliedCurrencyAndAmount {
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
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm11Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
    #[serde(rename = "HS01")]
    Hs01,
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
pub struct RateLock1 {
    #[serde(rename = "ReqdInd", skip_serializing_if = "Option::is_none")]
    pub reqd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ElgblInd", skip_serializing_if = "Option::is_none")]
    pub elgbl_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ApldInd", skip_serializing_if = "Option::is_none")]
    pub apld_ind: Option<TrueFalseIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TerminalIntegrationCategory1Code {
    #[serde(rename = "MPOI")]
    Mpoi,
    #[serde(rename = "MSLE")]
    Msle,
    #[serde(rename = "SSLE")]
    Ssle,
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
pub struct BaseOne25Rate {
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
pub struct Environment17 {
    #[validate]
    #[serde(rename = "Acqrr")]
    pub acqrr: PartyIdentification263,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification263>,
    #[serde(rename = "Sndr", skip_serializing_if = "Option::is_none")]
    pub sndr: Option<PartyIdentification263>,
    #[serde(rename = "Rcvr", skip_serializing_if = "Option::is_none")]
    pub rcvr: Option<PartyIdentification263>,
    #[serde(rename = "Accptr", skip_serializing_if = "Option::is_none")]
    pub accptr: Option<PartyIdentification255>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyIdentification263>,
    #[serde(rename = "Pyer", skip_serializing_if = "Option::is_none")]
    pub pyer: Option<PartyIdentification257>,
    #[serde(rename = "Pyee", skip_serializing_if = "Option::is_none")]
    pub pyee: Option<PartyIdentification257>,
    #[serde(rename = "Termnl", skip_serializing_if = "Option::is_none")]
    pub termnl: Option<Terminal4>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification263>,
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<CardData6>,
    #[serde(rename = "CstmrDvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_dvc: Option<CustomerDevice4>,
    #[serde(rename = "Wllt", skip_serializing_if = "Option::is_none")]
    pub wllt: Option<Wallet2>,
    #[serde(rename = "Tkn", skip_serializing_if = "Option::is_none")]
    pub tkn: Option<Token2>,
    #[serde(rename = "Crdhldr", skip_serializing_if = "Option::is_none")]
    pub crdhldr: Option<Cardholder19>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CompanyAssigner2Code {
    #[serde(rename = "ASSO")]
    Asso,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "CPNY")]
    Cpny,
    #[serde(rename = "LEII")]
    Leii,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
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
pub struct AdditionalCharacteristics1 {
    #[serde(rename = "BizTp", skip_serializing_if = "Option::is_none")]
    pub biz_tp: Option<AdditionalCharacteristicDetails1>,
    #[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
    pub ownr: Option<AdditionalCharacteristicDetails1>,
    #[serde(rename = "Certfctn", skip_serializing_if = "Option::is_none")]
    pub certfctn: Option<AdditionalCharacteristicDetails1>,
    #[serde(rename = "OwnrEthncty", skip_serializing_if = "Option::is_none")]
    pub ownr_ethncty: Option<AdditionalCharacteristicDetails1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DisplayCapabilities6 {
    #[serde(rename = "Dstn")]
    pub dstn: UserInterface1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "AvlblFrmt", default)]
    pub avlbl_frmt: Vec<OutputFormat1Code>,
    #[serde(rename = "NbOfLines", skip_serializing_if = "Option::is_none")]
    pub nb_of_lines: Option<Number>,
    #[serde(rename = "LineWidth", skip_serializing_if = "Option::is_none")]
    pub line_width: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AvlblLang", default)]
    pub avlbl_lang: Vec<LanguageCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingResult10 {
    #[serde(rename = "RspnSrc", skip_serializing_if = "Option::is_none")]
    pub rspn_src: Option<ApprovalEntity2>,
    #[validate]
    #[serde(rename = "RsltData")]
    pub rslt_data: ResultData7,
    #[serde(rename = "ApprvlCd", skip_serializing_if = "Option::is_none")]
    pub apprvl_cd: Option<Exact6AlphaNumericText>,
    #[serde(rename = "ErrDtl", skip_serializing_if = "Option::is_none")]
    pub err_dtl: Option<ErrorDetails2>,
    #[serde(rename = "OrgnlRsltData", skip_serializing_if = "Option::is_none")]
    pub orgnl_rslt_data: Option<ResultData7>,
    #[serde(rename = "ActnReqrd", skip_serializing_if = "Option::is_none")]
    pub actn_reqrd: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Actn", default)]
    pub actn: Vec<Action13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlActn", default)]
    pub addtl_actn: Vec<AdditionalAction1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType19 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "EnvlpdData", skip_serializing_if = "Option::is_none")]
    pub envlpd_data: Option<EnvelopedData5>,
    #[serde(rename = "AuthntcdData", skip_serializing_if = "Option::is_none")]
    pub authntcd_data: Option<AuthenticatedData5>,
    #[serde(rename = "SgndData", skip_serializing_if = "Option::is_none")]
    pub sgnd_data: Option<SignedData5>,
    #[serde(rename = "DgstdData", skip_serializing_if = "Option::is_none")]
    pub dgstd_data: Option<DigestedData5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionContext7 {
    #[serde(rename = "MrchntCtgyCd")]
    pub mrchnt_ctgy_cd: Iso18245MerchantCategoryCode,
    #[serde(
        rename = "MrchntCtgySpcfcData",
        skip_serializing_if = "Option::is_none"
    )]
    pub mrchnt_ctgy_spcfc_data: Option<Max35Text>,
    #[serde(rename = "CstmrCnsnt", skip_serializing_if = "Option::is_none")]
    pub cstmr_cnsnt: Option<TrueFalseIndicator>,
    #[serde(rename = "PINPadInprtv", skip_serializing_if = "Option::is_none")]
    pub pin_pad_inprtv: Option<TrueFalseIndicator>,
    #[serde(rename = "PINNtryBpssInd", skip_serializing_if = "Option::is_none")]
    pub pin_ntry_bpss_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ICCFllbckInd", skip_serializing_if = "Option::is_none")]
    pub icc_fllbck_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ICCFllbckRsnCd", skip_serializing_if = "Option::is_none")]
    pub icc_fllbck_rsn_cd: Option<IccFallbackReason1Code>,
    #[serde(rename = "OthrICCFllbckRsnCd", skip_serializing_if = "Option::is_none")]
    pub othr_icc_fllbck_rsn_cd: Option<Max35Text>,
    #[serde(rename = "MgntcStrpFllbckInd", skip_serializing_if = "Option::is_none")]
    pub mgntc_strp_fllbck_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "LatePresntmntInd", skip_serializing_if = "Option::is_none")]
    pub late_presntmnt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "FnlAuthstnInd", skip_serializing_if = "Option::is_none")]
    pub fnl_authstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DfrrdDlvryInd", skip_serializing_if = "Option::is_none")]
    pub dfrrd_dlvry_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "PrtlShipmntInd", skip_serializing_if = "Option::is_none")]
    pub prtl_shipmnt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DelydChrgsInd", skip_serializing_if = "Option::is_none")]
    pub delyd_chrgs_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "NoShowInd", skip_serializing_if = "Option::is_none")]
    pub no_show_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ReauthstnInd", skip_serializing_if = "Option::is_none")]
    pub reauthstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ReSubmissnInd", skip_serializing_if = "Option::is_none")]
    pub re_submissn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "TxInitr", skip_serializing_if = "Option::is_none")]
    pub tx_initr: Option<TransactionInitiator1Code>,
    #[serde(rename = "AuthntcnOutgInd", skip_serializing_if = "Option::is_none")]
    pub authntcn_outg_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CardPrgrmm", skip_serializing_if = "Option::is_none")]
    pub card_prgrmm: Option<CardProgramme2>,
    #[serde(rename = "Jursdctn", skip_serializing_if = "Option::is_none")]
    pub jursdctn: Option<Jurisdiction2>,
    #[serde(rename = "SttlmSvc", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc: Option<SettlementService3>,
    #[serde(rename = "Rcncltn", skip_serializing_if = "Option::is_none")]
    pub rcncltn: Option<Reconciliation3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XchgRateInf", default)]
    pub xchg_rate_inf: Vec<ExchangeRateInformation2>,
    #[serde(rename = "CaptrDt", skip_serializing_if = "Option::is_none")]
    pub captr_dt: Option<IsoDate>,
    #[serde(rename = "DtAntcptd", skip_serializing_if = "Option::is_none")]
    pub dt_antcptd: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SpecialProgrammeQualification1 {
    #[serde(rename = "Prgrmm", skip_serializing_if = "Option::is_none")]
    pub prgrmm: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dtl", default)]
    pub dtl: Vec<SpecialProgrammeDetails1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification20 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm19Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShippingData2 {
    #[serde(rename = "InvcNb", skip_serializing_if = "Option::is_none")]
    pub invc_nb: Option<Max70Text>,
    #[serde(rename = "InvcCreDtTm", skip_serializing_if = "Option::is_none")]
    pub invc_cre_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "SvcDscrptrCd", skip_serializing_if = "Option::is_none")]
    pub svc_dscrptr_cd: Option<Max40Text>,
    #[serde(rename = "IncntivAmt", skip_serializing_if = "Option::is_none")]
    pub incntiv_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MiscExpnss", skip_serializing_if = "Option::is_none")]
    pub misc_expnss: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "NbOfPackgs", skip_serializing_if = "Option::is_none")]
    pub nb_of_packgs: Option<Max6NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Packg", default)]
    pub packg: Vec<ShippingPackage2>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min5Max16Binary {
    #[validate(length(min = 5, max = 16,), regex = "MIN_5_MAX_16_BINARY_REGEX")]
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
pub struct PointOfServiceContext3 {
    #[serde(rename = "CardPres", skip_serializing_if = "Option::is_none")]
    pub card_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none")]
    pub crdhldr_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "CrdhldrActvtd", skip_serializing_if = "Option::is_none")]
    pub crdhldr_actvtd: Option<TrueFalseIndicator>,
    #[serde(rename = "TrnspndrInittd", skip_serializing_if = "Option::is_none")]
    pub trnspndr_inittd: Option<TrueFalseIndicator>,
    #[serde(rename = "AttnddInd", skip_serializing_if = "Option::is_none")]
    pub attndd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "UattnddLvlCtgy", skip_serializing_if = "Option::is_none")]
    pub uattndd_lvl_ctgy: Option<Max35NumericText>,
    #[serde(rename = "EComrcInd", skip_serializing_if = "Option::is_none")]
    pub e_comrc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EComrcData", default)]
    pub e_comrc_data: Vec<ECommerceData1>,
    #[serde(rename = "MOTOCd", skip_serializing_if = "Option::is_none")]
    pub moto_cd: Option<Moto1Code>,
    #[serde(rename = "TrnstInd", skip_serializing_if = "Option::is_none")]
    pub trnst_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "PrtlApprvlSpprtd", skip_serializing_if = "Option::is_none")]
    pub prtl_apprvl_spprtd: Option<TrueFalseIndicator>,
    #[serde(rename = "DelydAuthstnInd", skip_serializing_if = "Option::is_none")]
    pub delyd_authstn_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyChrtcs", default)]
    pub scty_chrtcs: Vec<SecurityCharacteristics1Code>,
    #[serde(rename = "OthrSctyChrtcs", skip_serializing_if = "Option::is_none")]
    pub othr_scty_chrtcs: Option<Max35Text>,
    #[serde(rename = "CardDataNtryMd")]
    pub card_data_ntry_md: CardDataReading10Code,
    #[serde(rename = "OthrCardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub othr_card_data_ntry_md: Option<Max35Text>,
    #[serde(rename = "QRCdPresntmntMd", skip_serializing_if = "Option::is_none")]
    pub qr_cd_presntmnt_md: Option<QrCodePresentmentMode1Code>,
    #[serde(
        rename = "OthrQRCdPresntmntMd",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_qr_cd_presntmnt_md: Option<Max35Text>,
    #[serde(
        rename = "TempScrCardDataReusd",
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_scr_card_data_reusd: Option<TrueFalseIndicator>,
    #[serde(rename = "StorgLctn", skip_serializing_if = "Option::is_none")]
    pub storg_lctn: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpclConds", default)]
    pub spcl_conds: Vec<SpecialConditions1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SaleItem3 {
    #[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
    pub pdct_tp: Option<Max35Text>,
    #[serde(rename = "PdctCd", skip_serializing_if = "Option::is_none")]
    pub pdct_cd: Option<Max70Text>,
    #[serde(rename = "PdctCdTp", skip_serializing_if = "Option::is_none")]
    pub pdct_cd_tp: Option<ProductCodeType1Code>,
    #[serde(rename = "AddtlPdctCd", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_cd: Option<Max70Text>,
    #[serde(rename = "AddtlPdctCdTp", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_cd_tp: Option<Max35Text>,
    #[serde(rename = "PdctCdModfr", skip_serializing_if = "Option::is_none")]
    pub pdct_cd_modfr: Option<DecimalNumber>,
    #[serde(rename = "PdctDesc", skip_serializing_if = "Option::is_none")]
    pub pdct_desc: Option<Max140Text>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<DecimalNumber>,
    #[serde(rename = "NonAdjstdUnitPric", skip_serializing_if = "Option::is_none")]
    pub non_adjstd_unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NonAdjstdTtlAmt", skip_serializing_if = "Option::is_none")]
    pub non_adjstd_ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment10>,
    #[serde(rename = "AdjstdAmt", skip_serializing_if = "Option::is_none")]
    pub adjstd_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UserInterface6Code {
    #[serde(rename = "CDSP")]
    Cdsp,
    #[serde(rename = "CRCP")]
    Crcp,
    #[serde(rename = "MDSP")]
    Mdsp,
    #[serde(rename = "MRCP")]
    Mrcp,
    #[serde(rename = "CRDO")]
    Crdo,
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "CHAP")]
    Chap,
    #[serde(rename = "MRAP")]
    Mrap,
    #[serde(rename = "MRIN")]
    Mrin,
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
pub struct SettlementService3 {
    #[serde(rename = "SttlmSvcPropsd", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc_propsd: Option<SettlementServiceMode1>,
    #[serde(rename = "SttlmSvcApld", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc_apld: Option<SettlementServiceMode1>,
    #[serde(rename = "SttlmSvcDts", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc_dts: Option<SettlementServiceDate2>,
    #[serde(rename = "SttlmRptgNtty", skip_serializing_if = "Option::is_none")]
    pub sttlm_rptg_ntty: Option<SettlementReportingEntity1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSttlmInf", default)]
    pub addtl_sttlm_inf: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataWriting1Code {
    #[serde(rename = "ICPY")]
    Icpy,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "ICCY")]
    Iccy,
    #[serde(rename = "MSIP")]
    Msip,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct Max1Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency12Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FRTN")]
    Frtn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "WEEK")]
    Week,
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
pub struct Verification4 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAuthenticationMethod1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VrfctnRslt", default)]
    pub vrfctn_rslt: Vec<VerificationResult2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm5Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
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
pub struct AdditionalIdentification1 {
    #[serde(rename = "Tp")]
    pub tp: AdditionalIdentificationType1Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification210 {
    #[serde(rename = "PrsnlId", skip_serializing_if = "Option::is_none")]
    pub prsnl_id: Option<Max35Text>,
    #[serde(rename = "MplyeeId", skip_serializing_if = "Option::is_none")]
    pub mplyee_id: Option<PhoneNumber>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "PrfssnlLvl", skip_serializing_if = "Option::is_none")]
    pub prfssnl_lvl: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter7 {
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
}
#[derive(
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
pub enum TimeSegment1Code {
    #[serde(rename = "AMBN")]
    Ambn,
    #[serde(rename = "PMAN")]
    Pman,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QrCodePresentmentMode1Code {
    #[serde(rename = "CPMD")]
    Cpmd,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "MPMD")]
    Mpmd,
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
pub struct Max8NumericText {
    #[validate(regex = "MAX_8_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BytePadding1Code {
    #[serde(rename = "LNGT")]
    Lngt,
    #[serde(rename = "NUL8")]
    Nul8,
    #[serde(rename = "NULG")]
    Nulg,
    #[serde(rename = "NULL")]
    Null,
    #[serde(rename = "RAND")]
    Rand,
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
pub struct InstalmentAmountDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InstalmentAmountDetailsType2Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<Amount16>,
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
pub struct Amount12 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TemporaryServicesCharge1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Hrs", skip_serializing_if = "Option::is_none")]
    pub hrs: Option<Max6NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalTransactionAmount2 {
    #[serde(rename = "AmtQlfr", skip_serializing_if = "Option::is_none")]
    pub amt_qlfr: Option<TypeOfAmount22Code>,
    #[validate]
    #[serde(rename = "TxAmt")]
    pub tx_amt: TransactionAmount1,
    #[serde(rename = "CrdhldrBllgAmt", skip_serializing_if = "Option::is_none")]
    pub crdhldr_bllg_amt: Option<Amount15>,
    #[serde(rename = "RcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub rcncltn_amt: Option<Amount15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact6AlphaNumericText {
    #[validate(regex = "EXACT_6_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GoodsAndServicesSubType1Code {
    #[serde(rename = "CRCU")]
    Crcu,
    #[serde(rename = "FORX")]
    Forx,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SECS")]
    Secs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CarRentalActivity1Code {
    #[serde(rename = "CARS")]
    Cars,
    #[serde(rename = "GLBL")]
    Glbl,
    #[serde(rename = "INDV")]
    Indv,
    #[serde(rename = "PETP")]
    Petp,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalServiceType2Code {
    #[serde(rename = "CACT")]
    Cact,
    #[serde(rename = "CSHB")]
    Cshb,
    #[serde(rename = "DCCV")]
    Dccv,
    #[serde(rename = "INTP")]
    Intp,
    #[serde(rename = "INTT")]
    Intt,
    #[serde(rename = "LOYT")]
    Loyt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRST")]
    Prst,
    #[serde(rename = "BALC")]
    Balc,
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
pub struct ECommerceData1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max2048Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptedDataFormat1Code {
    #[serde(rename = "ASCI")]
    Asci,
    #[serde(rename = "BINF")]
    Binf,
    #[serde(rename = "EBCD")]
    Ebcd,
    #[serde(rename = "HEXF")]
    Hexf,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LifeCycleSupport1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "FINC")]
    Finc,
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
pub struct ExchangeRateInformation2 {
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max70Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<IsoTime>,
    #[serde(rename = "XchgRateDtl", skip_serializing_if = "Option::is_none")]
    pub xchg_rate_dtl: Option<ExchangeRateDetail1>,
    #[serde(rename = "RateLck", skip_serializing_if = "Option::is_none")]
    pub rate_lck: Option<RateLock1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalData5 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "CmonNm", skip_serializing_if = "Option::is_none")]
    pub cmon_nm: Option<Max280Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max512Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reconciliation3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "ChckptRef", skip_serializing_if = "Option::is_none")]
    pub chckpt_ref: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SignedData5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DgstAlgo", default)]
    pub dgst_algo: Vec<AlgorithmIdentification21>,
    #[serde(rename = "NcpsltdCntt", skip_serializing_if = "Option::is_none")]
    pub ncpsltd_cntt: Option<EncapsulatedContent3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cert", default)]
    pub cert: Vec<Max5000Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Sgnr", default)]
    pub sgnr: Vec<Signer4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount21Code {
    #[serde(rename = "INTC")]
    Intc,
    #[serde(rename = "FEEP")]
    Feep,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "FEEA")]
    Feea,
    #[serde(rename = "CSIF")]
    Csif,
    #[serde(rename = "MXIF")]
    Mxif,
    #[serde(rename = "MNIF")]
    Mnif,
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
pub struct IsoMax3ALanguageCode {
    #[validate(regex = "ISO_MAX_3_A_LANGUAGE_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptionFormat2Code {
    #[serde(rename = "TR31")]
    Tr31,
    #[serde(rename = "TR34")]
    Tr34,
    #[serde(rename = "I238")]
    I238,
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
pub struct EnvelopedData5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "OrgtrInf", skip_serializing_if = "Option::is_none")]
    pub orgtr_inf: Option<OriginatorInformation1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient6Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max4Text {
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
pub struct VerificationResult2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Ntty", skip_serializing_if = "Option::is_none")]
    pub ntty: Option<VerificationEntity2Code>,
    #[serde(rename = "OthrNtty", skip_serializing_if = "Option::is_none")]
    pub othr_ntty: Option<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Verification3Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RsltDtls", default)]
    pub rslt_dtls: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddendumTaxType2Code {
    #[serde(rename = "TOTL")]
    Totl,
    #[serde(rename = "ALMI")]
    Almi,
    #[serde(rename = "ASNT")]
    Asnt,
    #[serde(rename = "BPTX")]
    Bptx,
    #[serde(rename = "KAPA")]
    Kapa,
    #[serde(rename = "NKAP")]
    Nkap,
    #[serde(rename = "CRTX")]
    Crtx,
    #[serde(rename = "CSTX")]
    Cstx,
    #[serde(rename = "CITX")]
    Citx,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "CPST")]
    Cpst,
    #[serde(rename = "CORT")]
    Cort,
    #[serde(rename = "COTX")]
    Cotx,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DLTX")]
    Dltx,
    #[serde(rename = "DUTY")]
    Duty,
    #[serde(rename = "EMIT")]
    Emit,
    #[serde(rename = "EMUT")]
    Emut,
    #[serde(rename = "EMET")]
    Emet,
    #[serde(rename = "EMST")]
    Emst,
    #[serde(rename = "EMRT")]
    Emrt,
    #[serde(rename = "ENTX")]
    Entx,
    #[serde(rename = "ESET")]
    Eset,
    #[serde(rename = "ENVT")]
    Envt,
    #[serde(rename = "EQUL")]
    Equl,
    #[serde(rename = "EQTX")]
    Eqtx,
    #[serde(rename = "EUTR")]
    Eutr,
    #[serde(rename = "EXEC")]
    Exec,
    #[serde(rename = "FEXT")]
    Fext,
    #[serde(rename = "FNST")]
    Fnst,
    #[serde(rename = "FETX")]
    Fetx,
    #[serde(rename = "FVAT")]
    Fvat,
    #[serde(rename = "FVTS")]
    Fvts,
    #[serde(rename = "FIMT")]
    Fimt,
    #[serde(rename = "FSST")]
    Fsst,
    #[serde(rename = "FICA")]
    Fica,
    #[serde(rename = "FRTX")]
    Frtx,
    #[serde(rename = "FSTX")]
    Fstx,
    #[serde(rename = "FSFT")]
    Fsft,
    #[serde(rename = "FUVT")]
    Fuvt,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "GCAT")]
    Gcat,
    #[serde(rename = "GRTX")]
    Grtx,
    #[serde(rename = "HVAT")]
    Hvat,
    #[serde(rename = "HATX")]
    Hatx,
    #[serde(rename = "HSTX")]
    Hstx,
    #[serde(rename = "HWTX")]
    Hwtx,
    #[serde(rename = "INHT")]
    Inht,
    #[serde(rename = "INPO")]
    Inpo,
    #[serde(rename = "LTTX")]
    Lttx,
    #[serde(rename = "FLST")]
    Flst,
    #[serde(rename = "LITX")]
    Litx,
    #[serde(rename = "LOCO")]
    Loco,
    #[serde(rename = "LSTX")]
    Lstx,
    #[serde(rename = "LOCL")]
    Locl,
    #[serde(rename = "LUTX")]
    Lutx,
    #[serde(rename = "MATX")]
    Matx,
    #[serde(rename = "METX")]
    Metx,
    #[serde(rename = "MITX")]
    Mitx,
    #[serde(rename = "MUTX")]
    Mutx,
    #[serde(rename = "MUDE")]
    Mude,
    #[serde(rename = "COUN")]
    Coun,
    #[serde(rename = "NATI")]
    Nati,
    #[serde(rename = "OCTX")]
    Octx,
    #[serde(rename = "OPTX")]
    Optx,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTTX")]
    Ottx,
    #[serde(rename = "LEVY")]
    Levy,
    #[serde(rename = "POTX")]
    Potx,
    #[serde(rename = "PRTX")]
    Prtx,
    #[serde(rename = "PSTX")]
    Pstx,
    #[serde(rename = "PPTX")]
    Pptx,
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "PHET")]
    Phet,
    #[serde(rename = "QUST")]
    Qust,
    #[serde(rename = "ROCI")]
    Roci,
    #[serde(rename = "RCSB")]
    Rcsb,
    #[serde(rename = "RCSD")]
    Rcsd,
    #[serde(rename = "ROTX")]
    Rotx,
    #[serde(rename = "RVAT")]
    Rvat,
    #[serde(rename = "ROVI")]
    Rovi,
    #[serde(rename = "SAUT")]
    Saut,
    #[serde(rename = "SCTX")]
    Sctx,
    #[serde(rename = "SPTX")]
    Sptx,
    #[serde(rename = "STTA")]
    Stta,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "SLST")]
    Slst,
    #[serde(rename = "SLTX")]
    Sltx,
    #[serde(rename = "SETX")]
    Setx,
    #[serde(rename = "STPT")]
    Stpt,
    #[serde(rename = "SPTS")]
    Spts,
    #[serde(rename = "SPFT")]
    Spft,
    #[serde(rename = "SPTG")]
    Sptg,
    #[serde(rename = "SRTX")]
    Srtx,
    #[serde(rename = "SSTX")]
    Sstx,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "STSL")]
    Stsl,
    #[serde(rename = "STEX")]
    Stex,
    #[serde(rename = "SUTX")]
    Sutx,
    #[serde(rename = "CTAX")]
    Ctax,
    #[serde(rename = "TDDT")]
    Tddt,
    #[serde(rename = "TELT")]
    Telt,
    #[serde(rename = "THTX")]
    Thtx,
    #[serde(rename = "TRAX")]
    Trax,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "UUTX")]
    Uutx,
    #[serde(rename = "VATA")]
    Vata,
    #[serde(rename = "VATB")]
    Vatb,
    #[serde(rename = "WTAX")]
    Wtax,
    #[serde(rename = "WESV")]
    Wesv,
    #[serde(rename = "WITF")]
    Witf,
    #[serde(rename = "WITL")]
    Witl,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "TNRT")]
    Tnrt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateTaxType1Code {
    #[serde(rename = "SMBS")]
    Smbs,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CORP")]
    Corp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LocationAmenity1Code {
    #[serde(rename = "AADA")]
    Aada,
    #[serde(rename = "AFLA")]
    Afla,
    #[serde(rename = "AVLA")]
    Avla,
    #[serde(rename = "CRWA")]
    Crwa,
    #[serde(rename = "CVSA")]
    Cvsa,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "EWAA")]
    Ewaa,
    #[serde(rename = "MRLA")]
    Mrla,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PAPA")]
    Papa,
    #[serde(rename = "TSHA")]
    Tsha,
    #[serde(rename = "TSRA")]
    Tsra,
    #[serde(rename = "TSSA")]
    Tssa,
    #[serde(rename = "TFHA")]
    Tfha,
    #[serde(rename = "VSWA")]
    Vswa,
    #[serde(rename = "VHPA")]
    Vhpa,
    #[serde(rename = "VMRA")]
    Vmra,
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
pub struct Context10 {
    #[validate]
    #[serde(rename = "PtOfSvcCntxt")]
    pub pt_of_svc_cntxt: PointOfServiceContext3,
    #[validate]
    #[serde(rename = "TxCntxt")]
    pub tx_cntxt: TransactionContext7,
    #[validate(length(min = 0,))]
    #[serde(rename = "Vrfctn", default)]
    pub vrfctn: Vec<Verification5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RskCntxt", default)]
    pub rsk_cntxt: Vec<RiskContext2>,
    #[serde(rename = "SaleCntxt", skip_serializing_if = "Option::is_none")]
    pub sale_cntxt: Option<SaleContext8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient6ChoiceEnum {
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek5>,
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient6Choice {
    #[serde(flatten)]
    pub value: Recipient6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncapsulatedContent3 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "Cntt", skip_serializing_if = "Option::is_none")]
    pub cntt: Option<Max100KBinary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount19 {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalService2 {
    #[serde(rename = "Tp")]
    pub tp: AdditionalServiceType2Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<AdditionalServiceResult1Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvcDtl", default)]
    pub svc_dtl: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact15Text {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TemporaryServicesCharge1Code {
    #[serde(rename = "WKND")]
    Wknd,
    #[serde(rename = "REGL")]
    Regl,
    #[serde(rename = "OVRT")]
    Ovrt,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "ONCL")]
    Oncl,
    #[serde(rename = "NITE")]
    Nite,
    #[serde(rename = "HOLI")]
    Holi,
    #[serde(rename = "HAZD")]
    Hazd,
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
pub struct DeliveryInformation4 {
    #[serde(rename = "DlvryNoteNb", skip_serializing_if = "Option::is_none")]
    pub dlvry_note_nb: Option<Max35Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
    #[serde(rename = "Instrs", skip_serializing_if = "Option::is_none")]
    pub instrs: Option<Max350Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PointOfInteractionComponentIdentification3 {
    #[serde(rename = "ItmNb", skip_serializing_if = "Option::is_none")]
    pub itm_nb: Option<Max35Text>,
    #[serde(rename = "PrvdrId", skip_serializing_if = "Option::is_none")]
    pub prvdr_id: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
    pub srl_nb: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundingSource2 {
    #[serde(rename = "SrcTp", skip_serializing_if = "Option::is_none")]
    pub src_tp: Option<FundingSourceType2Code>,
    #[serde(rename = "OthrSrcTp", skip_serializing_if = "Option::is_none")]
    pub othr_src_tp: Option<Max35Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount17 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationCharacteristics3 {
    #[serde(rename = "ComTp")]
    pub com_tp: PoiCommunicationType2Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "RmotPty", default)]
    pub rmot_pty: Vec<PartyType7Code>,
    #[validate]
    #[serde(rename = "Actv")]
    pub actv: TrueFalseIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max3000Binary {
    #[validate(length(min = 1, max = 3000,), regex = "MAX_3000_BINARY_REGEX")]
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
pub struct AlgorithmIdentification22 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm17Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter13 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm20Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification26>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Vehicle4 {
    #[serde(rename = "Clss", skip_serializing_if = "Option::is_none")]
    pub clss: Option<Max35Text>,
    #[serde(rename = "Make", skip_serializing_if = "Option::is_none")]
    pub make: Option<Max35NumericText>,
    #[serde(rename = "Mdl", skip_serializing_if = "Option::is_none")]
    pub mdl: Option<Max35NumericText>,
    #[serde(rename = "RegnNb", skip_serializing_if = "Option::is_none")]
    pub regn_nb: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardholderVerificationCapability5Code {
    #[serde(rename = "APKI")]
    Apki,
    #[serde(rename = "NOVF")]
    Novf,
    #[serde(rename = "FBIG")]
    Fbig,
    #[serde(rename = "FBIO")]
    Fbio,
    #[serde(rename = "FDSG")]
    Fdsg,
    #[serde(rename = "FCPN")]
    Fcpn,
    #[serde(rename = "FEPN")]
    Fepn,
    #[serde(rename = "NBIO")]
    Nbio,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "VORN")]
    Vorn,
    #[serde(rename = "PKIS")]
    Pkis,
    #[serde(rename = "NOPN")]
    Nopn,
    #[serde(rename = "NOOP")]
    Noop,
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
pub struct Max35NumericText {
    #[validate(regex = "MAX_35_NUMERIC_TEXT_REGEX")]
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
pub struct Contact6 {
    #[serde(rename = "HomePhneNb", skip_serializing_if = "Option::is_none")]
    pub home_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "BizPhneNb", skip_serializing_if = "Option::is_none")]
    pub biz_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobPhneNb", skip_serializing_if = "Option::is_none")]
    pub mob_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "OthrPhneNb", skip_serializing_if = "Option::is_none")]
    pub othr_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "PrsnlEmailAdr", skip_serializing_if = "Option::is_none")]
    pub prsnl_email_adr: Option<Max256Text>,
    #[serde(rename = "BizEmailAdr", skip_serializing_if = "Option::is_none")]
    pub biz_email_adr: Option<Max256Text>,
    #[serde(rename = "OthrEmailAdr", skip_serializing_if = "Option::is_none")]
    pub othr_email_adr: Option<Max256Text>,
    #[serde(rename = "HomeFaxNb", skip_serializing_if = "Option::is_none")]
    pub home_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "BizFaxNb", skip_serializing_if = "Option::is_none")]
    pub biz_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DepartureOrArrival2 {
    #[serde(rename = "CrrierCd", skip_serializing_if = "Option::is_none")]
    pub crrier_cd: Option<Max35Text>,
    #[serde(rename = "RouteNb", skip_serializing_if = "Option::is_none")]
    pub route_nb: Option<Max35NumericText>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalTransaction2 {
    #[serde(rename = "Envt", skip_serializing_if = "Option::is_none")]
    pub envt: Option<Environment20>,
    #[serde(rename = "Cntxt", skip_serializing_if = "Option::is_none")]
    pub cntxt: Option<Context12>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<Transaction147>,
    #[serde(rename = "PrcgRslt", skip_serializing_if = "Option::is_none")]
    pub prcg_rslt: Option<ProcessingResult10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Destination3 {
    #[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
    pub nm_and_lctn: Option<Max99Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Binary {
    #[validate(length(min = 1, max = 35,), regex = "MAX_35_BINARY_REGEX")]
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
pub struct AdditionalData1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max2048Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionInitiator1Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "CUST")]
    Cust,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RiskAssessment1Code {
    #[serde(rename = "APPC")]
    Appc,
    #[serde(rename = "APPH")]
    Apph,
    #[serde(rename = "APPU")]
    Appu,
    #[serde(rename = "DONT")]
    Dont,
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
pub struct LocalAmenity1 {
    #[serde(rename = "Tp")]
    pub tp: LocationAmenity1Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "AvlblInd", skip_serializing_if = "Option::is_none")]
    pub avlbl_ind: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max8HexBinaryText {
    #[validate(regex = "MAX_8_HEX_BINARY_TEXT_REGEX")]
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
pub struct UnitOfMeasure2 {
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "NbOfUnits", skip_serializing_if = "Option::is_none")]
    pub nb_of_units: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Cardholder19 {
    #[serde(rename = "CrdhldrNm", skip_serializing_if = "Option::is_none")]
    pub crdhldr_nm: Option<CardholderName3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Id", default)]
    pub id: Vec<Credentials2>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "CtctInf", skip_serializing_if = "Option::is_none")]
    pub ctct_inf: Option<Contact1>,
    #[serde(rename = "DtOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_of_birth: Option<IsoDate>,
    #[serde(rename = "HghValCstmrInd", skip_serializing_if = "Option::is_none")]
    pub hgh_val_cstmr_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TelecomCallDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TelephonyCallType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max70Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<Max35Text>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub stat: Option<Max16Text>,
    #[serde(rename = "Prvc", skip_serializing_if = "Option::is_none")]
    pub prvc: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Action13 {
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyType20Code>,
    #[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
    pub actn_tp: Option<ActionType11Code>,
    #[serde(rename = "OthrActnTp", skip_serializing_if = "Option::is_none")]
    pub othr_actn_tp: Option<Max35Text>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CarrierIdentification1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max35Text>,
    #[serde(rename = "IATACd", skip_serializing_if = "Option::is_none")]
    pub iata_cd: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MemoryCharacteristics1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "TtlSz")]
    pub ttl_sz: DecimalNumber,
    #[validate]
    #[serde(rename = "FreeSz")]
    pub free_sz: DecimalNumber,
    #[serde(rename = "Unit")]
    pub unit: MemoryUnit1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PresentationMedium2Code {
    #[serde(rename = "BIOM")]
    Biom,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "PAPR")]
    Papr,
    #[serde(rename = "BOTH")]
    Both,
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
pub struct LodgingSummary2 {
    #[serde(rename = "FolioNb", skip_serializing_if = "Option::is_none")]
    pub folio_nb: Option<Max35Text>,
    #[serde(rename = "Prprty", skip_serializing_if = "Option::is_none")]
    pub prprty: Option<LodgingProperty2>,
    #[serde(rename = "Cstmr", skip_serializing_if = "Option::is_none")]
    pub cstmr: Option<Customer8>,
    #[serde(rename = "NbOfRooms", skip_serializing_if = "Option::is_none")]
    pub nb_of_rooms: Option<Max4NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Room", default)]
    pub room: Vec<LodgingRoom1>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "Arrvl", skip_serializing_if = "Option::is_none")]
    pub arrvl: Option<DepartureOrArrival2>,
    #[serde(rename = "Dprture", skip_serializing_if = "Option::is_none")]
    pub dprture: Option<DepartureOrArrival1>,
    #[serde(rename = "NoShowInd", skip_serializing_if = "Option::is_none")]
    pub no_show_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TtlTax", default)]
    pub ttl_tax: Vec<Tax39>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AuthrsdAmt", default)]
    pub authrsd_amt: Vec<AuthorisedAmount1>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LltyPrgrmm", default)]
    pub llty_prgrmm: Vec<LoyaltyProgramme2>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
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
pub struct PartyIdentification260 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification258>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
    #[serde(rename = "Instrs", skip_serializing_if = "Option::is_none")]
    pub instrs: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundingSourceType2Code {
    #[serde(rename = "LOYL")]
    Loyl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SVNG")]
    Svng,
    #[serde(rename = "UVRL")]
    Uvrl,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CDBT")]
    Cdbt,
    #[serde(rename = "EPRS")]
    Eprs,
    #[serde(rename = "DBAC")]
    Dbac,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CHQE")]
    Chqe,
    #[serde(rename = "PRPD")]
    Prpd,
    #[serde(rename = "LCDT")]
    Lcdt,
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
pub struct FundingService2 {
    #[serde(rename = "FndgSvc", skip_serializing_if = "Option::is_none")]
    pub fndg_svc: Option<TransferService2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FndgSrc", default)]
    pub fndg_src: Vec<FundingSource2>,
    #[serde(rename = "ClmInf", skip_serializing_if = "Option::is_none")]
    pub clm_inf: Option<ClaimInformation1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm17Code {
    #[serde(rename = "MACC")]
    Macc,
    #[serde(rename = "MCCS")]
    Mccs,
    #[serde(rename = "CMA1")]
    Cma1,
    #[serde(rename = "MCC1")]
    Mcc1,
    #[serde(rename = "CMA9")]
    Cma9,
    #[serde(rename = "CMA5")]
    Cma5,
    #[serde(rename = "CMA2")]
    Cma2,
    #[serde(rename = "CM31")]
    Cm31,
    #[serde(rename = "CM32")]
    Cm32,
    #[serde(rename = "CM33")]
    Cm33,
    #[serde(rename = "MCS3")]
    Mcs3,
    #[serde(rename = "CCA1")]
    Cca1,
    #[serde(rename = "CCA2")]
    Cca2,
    #[serde(rename = "CCA3")]
    Cca3,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount20Code {
    #[serde(rename = "AMTH")]
    Amth,
    #[serde(rename = "BAGG")]
    Bagg,
    #[serde(rename = "CARG")]
    Carg,
    #[serde(rename = "CHTC")]
    Chtc,
    #[serde(rename = "CLUB")]
    Club,
    #[serde(rename = "DUTY")]
    Duty,
    #[serde(rename = "EXTK")]
    Extk,
    #[serde(rename = "EXTF")]
    Extf,
    #[serde(rename = "EXTR")]
    Extr,
    #[serde(rename = "FARE")]
    Fare,
    #[serde(rename = "FDBV")]
    Fdbv,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "MISC")]
    Misc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PETC")]
    Petc,
    #[serde(rename = "PHNE")]
    Phne,
    #[serde(rename = "PRPY")]
    Prpy,
    #[serde(rename = "TOTL")]
    Totl,
    #[serde(rename = "TOUR")]
    Tour,
    #[serde(rename = "UPGD")]
    Upgd,
    #[serde(rename = "TKDL")]
    Tkdl,
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
pub struct VehicleRentalCompany2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification258>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact3>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CarRentalActivity1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardholderVerificationCapabilities1 {
    #[serde(rename = "Cpblty")]
    pub cpblty: CardholderVerificationCapability5Code,
    #[serde(rename = "OthrCpblty", skip_serializing_if = "Option::is_none")]
    pub othr_cpblty: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType21Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct PointOfInteractionComponentCharacteristics4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Mmry", default)]
    pub mmry: Vec<MemoryCharacteristics1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Com", default)]
    pub com: Vec<CommunicationCharacteristics3>,
    #[serde(rename = "SctyAccsMdls", skip_serializing_if = "Option::is_none")]
    pub scty_accs_mdls: Option<Number>,
    #[serde(rename = "SbcbrIdntyMdls", skip_serializing_if = "Option::is_none")]
    pub sbcbr_idnty_mdls: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyElmt", default)]
    pub scty_elmt: Vec<CryptographicKey13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementReportingEntity1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TelephonyCallType1Code {
    #[serde(rename = "VCML")]
    Vcml,
    #[serde(rename = "TFPC")]
    Tfpc,
    #[serde(rename = "PAGE")]
    Page,
    #[serde(rename = "OGPC")]
    Ogpc,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTCL")]
    Otcl,
    #[serde(rename = "ICPC")]
    Icpc,
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
pub struct Tax39 {
    #[serde(rename = "Tp")]
    pub tp: AddendumTaxType2Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
    pub tax_xmptn: Option<TrueFalseIndicator>,
    #[serde(rename = "TaxXmptRsn", skip_serializing_if = "Option::is_none")]
    pub tax_xmpt_rsn: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "InclInTtlInd", skip_serializing_if = "Option::is_none")]
    pub incl_in_ttl_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Traceability9 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification183,
    #[serde(rename = "TracDtTmIn", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_in: Option<IsoDateTime>,
    #[serde(rename = "TracDtTmOut", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_out: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DigestedData5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: AlgorithmIdentification21,
    #[validate]
    #[serde(rename = "NcpsltdCntt")]
    pub ncpsltd_cntt: EncapsulatedContent3,
    #[validate]
    #[serde(rename = "Dgst")]
    pub dgst: Max140Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KeyTransport6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient5Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification27,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max5000Binary,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductCodeType1Code {
    #[serde(rename = "EA13")]
    Ea13,
    #[serde(rename = "EAN8")]
    Ean8,
    #[serde(rename = "GTIN")]
    Gtin,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PLUP")]
    Plup,
    #[serde(rename = "RS14")]
    Rs14,
    #[serde(rename = "UPCA")]
    Upca,
    #[serde(rename = "UPCE")]
    Upce,
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
pub struct LocalData1 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max70Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction16Code {
    #[serde(rename = "ADVC")]
    Advc,
    #[serde(rename = "NOTI")]
    Noti,
    #[serde(rename = "CAAD")]
    Caad,
    #[serde(rename = "CANO")]
    Cano,
    #[serde(rename = "REQU")]
    Requ,
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
pub struct VerificationInformation1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<VerificationValue1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<Max35Text>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "VldtyEndDt", skip_serializing_if = "Option::is_none")]
    pub vldty_end_dt: Option<IsoDate>,
    #[serde(rename = "VldtyEndTm", skip_serializing_if = "Option::is_none")]
    pub vldty_end_tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount16 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalInformation19 {
    #[serde(rename = "NtrdDataNmrc", skip_serializing_if = "Option::is_none")]
    pub ntrd_data_nmrc: Option<Max35NumericText>,
    #[serde(rename = "NtrdDataAlphaNmrc", skip_serializing_if = "Option::is_none")]
    pub ntrd_data_alpha_nmrc: Option<Max350Text>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalAction1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ActionType10Code>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyType21Code>,
    #[serde(rename = "OthrDstn", skip_serializing_if = "Option::is_none")]
    pub othr_dstn: Option<Max35Text>,
    #[serde(rename = "DstnTp", skip_serializing_if = "Option::is_none")]
    pub dstn_tp: Option<ActionDestination1Code>,
    #[serde(rename = "OthrDstnTp", skip_serializing_if = "Option::is_none")]
    pub othr_dstn_tp: Option<Max35Text>,
    #[serde(rename = "DstnAdr", skip_serializing_if = "Option::is_none")]
    pub dstn_adr: Option<Max70Text>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat4Code>,
    #[serde(rename = "OthrFrmt", skip_serializing_if = "Option::is_none")]
    pub othr_frmt: Option<Max35Text>,
    #[serde(rename = "Cntt", skip_serializing_if = "Option::is_none")]
    pub cntt: Option<Content1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Capabilities2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "CardRdngCpblty", default)]
    pub card_rdng_cpblty: Vec<CardReadingCapabilities1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CardWrtgCpblty", default)]
    pub card_wrtg_cpblty: Vec<CardWritingCapabilities1>,
    #[serde(rename = "PINLngthCpblty", skip_serializing_if = "Option::is_none")]
    pub pin_lngth_cpblty: Option<Number>,
    #[serde(rename = "PINNtrySctyChrtc", skip_serializing_if = "Option::is_none")]
    pub pin_ntry_scty_chrtc: Option<PinEntrySecurityCharacteristic1Code>,
    #[serde(
        rename = "OthrPINNtrySctyChrtc",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_pin_ntry_scty_chrtc: Option<Max35Text>,
    #[serde(rename = "ApprvlCdLngth", skip_serializing_if = "Option::is_none")]
    pub apprvl_cd_lngth: Option<Number>,
    #[serde(rename = "MxScrptLngth", skip_serializing_if = "Option::is_none")]
    pub mx_scrpt_lngth: Option<Number>,
    #[serde(rename = "CardCaptrCpbl", skip_serializing_if = "Option::is_none")]
    pub card_captr_cpbl: Option<TrueFalseIndicator>,
    #[serde(rename = "OnLineCpblty", skip_serializing_if = "Option::is_none")]
    pub on_line_cpblty: Option<OnLineCapability2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgCpblty", default)]
    pub msg_cpblty: Vec<DisplayCapabilities6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CrdhldrVrfctnCpblty", default)]
    pub crdhldr_vrfctn_cpblty: Vec<CardholderVerificationCapabilities1>,
    #[serde(
        rename = "TempScrCardDataStorg",
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_scr_card_data_storg: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TravelAgencyPackage1 {
    #[serde(rename = "RsvatnNb", skip_serializing_if = "Option::is_none")]
    pub rsvatn_nb: Option<Max35Text>,
    #[serde(rename = "TrvlPackgTp", skip_serializing_if = "Option::is_none")]
    pub trvl_packg_tp: Option<Max70Text>,
    #[serde(rename = "NbInPty", skip_serializing_if = "Option::is_none")]
    pub nb_in_pty: Option<Max10NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CstmrRef", default)]
    pub cstmr_ref: Vec<CustomerReference1>,
    #[serde(rename = "DataSrc", skip_serializing_if = "Option::is_none")]
    pub data_src: Option<Max35Text>,
    #[serde(rename = "DlvryOrdrNb", skip_serializing_if = "Option::is_none")]
    pub dlvry_ordr_nb: Option<Max35Text>,
    #[serde(rename = "CdtCardSlipNb", skip_serializing_if = "Option::is_none")]
    pub cdt_card_slip_nb: Option<Max35Text>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DisputeIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max70Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Plan2 {
    #[serde(rename = "PlanId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<Max70Text>,
    #[serde(rename = "PlanOwnr", skip_serializing_if = "Option::is_none")]
    pub plan_ownr: Option<PlanOwner1Code>,
    #[serde(rename = "OthrPlanOwnr", skip_serializing_if = "Option::is_none")]
    pub othr_plan_ownr: Option<Max35Text>,
    #[serde(rename = "InstlmtPmtTp", skip_serializing_if = "Option::is_none")]
    pub instlmt_pmt_tp: Option<Max35Text>,
    #[serde(rename = "DfrrdInstlmtInd", skip_serializing_if = "Option::is_none")]
    pub dfrrd_instlmt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "PrdUnit", skip_serializing_if = "Option::is_none")]
    pub prd_unit: Option<Frequency18Code>,
    #[serde(rename = "NbOfPrds", skip_serializing_if = "Option::is_none")]
    pub nb_of_prds: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRate", default)]
    pub intrst_rate: Vec<InterestRateDetails2>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "FrstAmt", skip_serializing_if = "Option::is_none")]
    pub frst_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NrmlPmtAmt", skip_serializing_if = "Option::is_none")]
    pub nrml_pmt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TtlNbOfPmts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_pmts: Option<Number>,
    #[serde(rename = "InstlmtCcy", skip_serializing_if = "Option::is_none")]
    pub instlmt_ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "GracePrd", skip_serializing_if = "Option::is_none")]
    pub grace_prd: Option<GracePeriod2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AmtDtls", default)]
    pub amt_dtls: Vec<InstalmentAmountDetails2>,
    #[serde(rename = "GrdTtlAmt", skip_serializing_if = "Option::is_none")]
    pub grd_ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm23Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
    #[serde(rename = "EA2R")]
    Ea2R,
    #[serde(rename = "EA9R")]
    Ea9R,
    #[serde(rename = "EA5R")]
    Ea5R,
    #[serde(rename = "E3DR")]
    E3Dr,
    #[serde(rename = "E36C")]
    E36C,
    #[serde(rename = "E36R")]
    E36R,
    #[serde(rename = "SD5C")]
    Sd5C,
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
pub struct RiskInputData1 {
    #[serde(rename = "Ntty", skip_serializing_if = "Option::is_none")]
    pub ntty: Option<RiskAssessmentDataEntityProvider1>,
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max10KText,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TemporaryServicesJob1 {
    #[serde(rename = "JobCd", skip_serializing_if = "Option::is_none")]
    pub job_cd: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max5NumericText>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<IsoDate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MemoryUnit1Code {
    #[serde(rename = "BYTE")]
    Byte,
    #[serde(rename = "EXAB")]
    Exab,
    #[serde(rename = "GIGA")]
    Giga,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "MEGA")]
    Mega,
    #[serde(rename = "PETA")]
    Peta,
    #[serde(rename = "TERA")]
    Tera,
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
pub struct OriginatorInformation1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Cert", default)]
    pub cert: Vec<Max5000Binary>,
}
#[derive(
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
pub struct Adjustment10 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "AddtlTp", skip_serializing_if = "Option::is_none")]
    pub addtl_tp: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max70Text>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max35Text>,
    #[serde(rename = "PrmtnElgblty", skip_serializing_if = "Option::is_none")]
    pub prmtn_elgblty: Option<TrueFalseIndicator>,
    #[serde(rename = "PrmtnCd", skip_serializing_if = "Option::is_none")]
    pub prmtn_cd: Option<Max35Text>,
    #[serde(rename = "PrmtnCpnNb", skip_serializing_if = "Option::is_none")]
    pub prmtn_cpn_nb: Option<Max35Text>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<DecimalNumber>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "AdjstmntAmt", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TaxClctdOnOrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub tax_clctd_on_orgnl_amt: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso3NumericCurrencyCode {
    #[validate(regex = "ISO_3_NUMERIC_CURRENCY_CODE_REGEX")]
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
pub struct GenericIdentification48 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max35Text,
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
pub struct Max40Text {
    #[validate(length(min = 1, max = 40,))]
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
pub struct PartyIdentification258 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification208 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max4Text,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max70Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CryptographicKeyType3Code {
    #[serde(rename = "AES2")]
    Aes2,
    #[serde(rename = "EDE3")]
    Ede3,
    #[serde(rename = "DKP9")]
    Dkp9,
    #[serde(rename = "AES9")]
    Aes9,
    #[serde(rename = "AES5")]
    Aes5,
    #[serde(rename = "EDE4")]
    Ede4,
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
pub struct Parameter14 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat3Code>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LodgingProperty2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<LodgingActivity1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "PrstgsPrprty", skip_serializing_if = "Option::is_none")]
    pub prstgs_prprty: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification258,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<Location4>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<CompanyAssigner2Code>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact3>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "FireSftyActInd", skip_serializing_if = "Option::is_none")]
    pub fire_sfty_act_ind: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max37Text {
    #[validate(length(min = 1, max = 37,))]
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
pub struct Ticket2 {
    #[serde(rename = "TcktNb", skip_serializing_if = "Option::is_none")]
    pub tckt_nb: Option<Max35Text>,
    #[serde(rename = "TcktIssr", skip_serializing_if = "Option::is_none")]
    pub tckt_issr: Option<PartyIdentification258>,
    #[serde(rename = "TcktIsseDt", skip_serializing_if = "Option::is_none")]
    pub tckt_isse_dt: Option<IsoDate>,
    #[serde(rename = "TcktIsseLctn", skip_serializing_if = "Option::is_none")]
    pub tckt_isse_lctn: Option<Max140Text>,
    #[serde(rename = "CnjnctnTcktNb", skip_serializing_if = "Option::is_none")]
    pub cnjnctn_tckt_nb: Option<Max35Text>,
    #[serde(rename = "RstrctdTcktInd", skip_serializing_if = "Option::is_none")]
    pub rstrctd_tckt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "OpnTcktInd", skip_serializing_if = "Option::is_none")]
    pub opn_tckt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "Rstrctns", skip_serializing_if = "Option::is_none")]
    pub rstrctns: Option<Max70Text>,
    #[serde(rename = "XchgdTcktInd", skip_serializing_if = "Option::is_none")]
    pub xchgd_tckt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "XchgdTcktNb", skip_serializing_if = "Option::is_none")]
    pub xchgd_tckt_nb: Option<Max35Text>,
    #[serde(rename = "RcrdLctrNb", skip_serializing_if = "Option::is_none")]
    pub rcrd_lctr_nb: Option<Max35Text>,
    #[serde(rename = "Rsvatn", skip_serializing_if = "Option::is_none")]
    pub rsvatn: Option<ReservationDetails3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReservationDetails3 {
    #[serde(rename = "Sys", skip_serializing_if = "Option::is_none")]
    pub sys: Option<Max4Text>,
    #[serde(rename = "RsvatnNb", skip_serializing_if = "Option::is_none")]
    pub rsvatn_nb: Option<Max35Text>,
    #[serde(rename = "OrgnlSys", skip_serializing_if = "Option::is_none")]
    pub orgnl_sys: Option<Max4Text>,
    #[serde(rename = "OrgnlRsvatnNb", skip_serializing_if = "Option::is_none")]
    pub orgnl_rsvatn_nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Address2 {
    #[serde(rename = "AdrLine1", skip_serializing_if = "Option::is_none")]
    pub adr_line_1: Option<Max99Text>,
    #[serde(rename = "AdrLine2", skip_serializing_if = "Option::is_none")]
    pub adr_line_2: Option<Max99Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max99Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstlCd", skip_serializing_if = "Option::is_none")]
    pub pstl_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max50Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoiComponentStatus1Code {
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "OUTD")]
    Outd,
    #[serde(rename = "OPER")]
    Oper,
    #[serde(rename = "DACT")]
    Dact,
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
pub struct CardProgrammeMode3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlId", default)]
    pub addtl_id: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionIdentification18 {
    #[serde(rename = "LclDt", skip_serializing_if = "Option::is_none")]
    pub lcl_dt: Option<IsoDate>,
    #[serde(rename = "LclTm", skip_serializing_if = "Option::is_none")]
    pub lcl_tm: Option<IsoTime>,
    #[serde(rename = "TmZone", skip_serializing_if = "Option::is_none")]
    pub tm_zone: Option<Max70Text>,
    #[serde(rename = "TxRef", skip_serializing_if = "Option::is_none")]
    pub tx_ref: Option<Max35Text>,
    #[serde(rename = "TrnsmssnDtTm", skip_serializing_if = "Option::is_none")]
    pub trnsmssn_dt_tm: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "SysTracAudtNb")]
    pub sys_trac_audt_nb: Max12NumericText,
    #[validate]
    #[serde(rename = "RtrvlRefNb")]
    pub rtrvl_ref_nb: Exact12Text,
    #[serde(rename = "LifeCyclSpprtInd", skip_serializing_if = "Option::is_none")]
    pub life_cycl_spprt_ind: Option<Exact2NumericText>,
    #[serde(rename = "LifeCyclTracIdData", skip_serializing_if = "Option::is_none")]
    pub life_cycl_trac_id_data: Option<TransactionLifeCycleIdentification1>,
    #[serde(
        rename = "LifeCyclTracIdMssng",
        skip_serializing_if = "Option::is_none"
    )]
    pub life_cycl_trac_id_mssng: Option<Max70Text>,
    #[serde(rename = "AcqrrRefData", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_data: Option<Max140Text>,
    #[serde(rename = "AcqrrRefNb", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_nb: Option<Max23NumericText>,
    #[serde(rename = "CardIssrRefData", skip_serializing_if = "Option::is_none")]
    pub card_issr_ref_data: Option<Max1000Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum KeyUsage1Code {
    #[serde(rename = "ENCR")]
    Encr,
    #[serde(rename = "DCPT")]
    Dcpt,
    #[serde(rename = "DENC")]
    Denc,
    #[serde(rename = "DDEC")]
    Ddec,
    #[serde(rename = "TRNI")]
    Trni,
    #[serde(rename = "TRNX")]
    Trnx,
    #[serde(rename = "MACG")]
    Macg,
    #[serde(rename = "MACV")]
    Macv,
    #[serde(rename = "SIGG")]
    Sigg,
    #[serde(rename = "SUGV")]
    Sugv,
    #[serde(rename = "PINE")]
    Pine,
    #[serde(rename = "PIND")]
    Pind,
    #[serde(rename = "PINV")]
    Pinv,
    #[serde(rename = "KEYG")]
    Keyg,
    #[serde(rename = "KEYI")]
    Keyi,
    #[serde(rename = "KEYX")]
    Keyx,
    #[serde(rename = "KEYD")]
    Keyd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount22Code {
    #[serde(rename = "ACTL")]
    Actl,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "DPST")]
    Dpst,
    #[serde(rename = "ESTM")]
    Estm,
    #[serde(rename = "MAXI")]
    Maxi,
    #[serde(rename = "PRXY")]
    Prxy,
    #[serde(rename = "RESD")]
    Resd,
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
pub struct DriverInParty2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
    #[serde(rename = "DtOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_of_birth: Option<IsoDate>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<Max2NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DrvrCrdntl", default)]
    pub drvr_crdntl: Vec<TravelDocument2>,
    #[serde(rename = "DrvgLic", skip_serializing_if = "Option::is_none")]
    pub drvg_lic: Option<DrivingLicense2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure1Code {
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "ACRE")]
    Acre,
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
pub struct Wallet2 {
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<PartyIdentification258>,
    #[serde(rename = "PANAge", skip_serializing_if = "Option::is_none")]
    pub pan_age: Option<Max5PositiveNumber>,
    #[serde(rename = "UsrAcctAge", skip_serializing_if = "Option::is_none")]
    pub usr_acct_age: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltAcctAge", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_age: Option<Max5PositiveNumber>,
    #[serde(rename = "DaysSncLastActvty", skip_serializing_if = "Option::is_none")]
    pub days_snc_last_actvty: Option<Max5PositiveNumber>,
    #[serde(rename = "Actvty", skip_serializing_if = "Option::is_none")]
    pub actvty: Option<Max10PositiveNumber>,
    #[serde(rename = "ActvtyIntrvl", skip_serializing_if = "Option::is_none")]
    pub actvty_intrvl: Option<Frequency12Code>,
    #[serde(rename = "LastWlltChng", skip_serializing_if = "Option::is_none")]
    pub last_wllt_chng: Option<Max5PositiveNumber>,
    #[serde(rename = "SspdCrds", skip_serializing_if = "Option::is_none")]
    pub sspd_crds: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltAcctCtry", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "CardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub card_data_ntry_md: Option<CardDataReading9Code>,
    #[serde(rename = "OthrCardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub othr_card_data_ntry_md: Option<Max35Text>,
    #[serde(rename = "WlltAcctEmailAge", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_email_age: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrRskAssmnt", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_rsk_assmnt: Option<RiskAssessment1Code>,
    #[serde(
        rename = "WlltPrvdrRskAssmntMdlVrsn",
        skip_serializing_if = "Option::is_none"
    )]
    pub wllt_prvdr_rsk_assmnt_mdl_vrsn: Option<Max35Text>,
    #[serde(rename = "WlltPrvdrPhneScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_phne_score: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrDvcScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_dvc_score: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrAcctScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_acct_score: Option<Max5PositiveNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedContent4 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "CnttNcrptnAlgo", skip_serializing_if = "Option::is_none")]
    pub cntt_ncrptn_algo: Option<AlgorithmIdentification24>,
    #[validate]
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: Max100KBinary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification254 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Text>,
    #[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
    pub nm_and_lctn: Option<Max99Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max256Text>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<Max35Text>,
    #[serde(rename = "CstmrSvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_svc: Option<Max35Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max256Text>,
    #[serde(rename = "TaxRegnId", skip_serializing_if = "Option::is_none")]
    pub tax_regn_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpnsrdMrchnt", default)]
    pub spnsrd_mrchnt: Vec<SponsoredMerchant2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GoodAndServiceDeliverySchedule1Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "ONDL")]
    Ondl,
    #[serde(rename = "SDDL")]
    Sddl,
    #[serde(rename = "TDDL")]
    Tddl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeviceIdentificationType1Code {
    #[serde(rename = "IMEI")]
    Imei,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SEID")]
    Seid,
    #[serde(rename = "SENU")]
    Senu,
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
pub struct GenericInformation1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter12 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat2Code>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max23NumericText {
    #[validate(regex = "MAX_23_NUMERIC_TEXT_REGEX")]
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
pub struct ServiceStartEnd2 {
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<Max35Text>,
    #[serde(rename = "LctnCd", skip_serializing_if = "Option::is_none")]
    pub lctn_cd: Option<Max35Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact2>,
    #[serde(rename = "DtAndTm", skip_serializing_if = "Option::is_none")]
    pub dt_and_tm: Option<IsoDateTime>,
    #[serde(rename = "TmSgmt", skip_serializing_if = "Option::is_none")]
    pub tm_sgmt: Option<TimeSegment1Code>,
    #[serde(rename = "JrnyInf", skip_serializing_if = "Option::is_none")]
    pub jrny_inf: Option<JourneyInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10NumericText {
    #[validate(regex = "MAX_10_NUMERIC_TEXT_REGEX")]
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
pub struct Environment20 {
    #[serde(rename = "Acqrr", skip_serializing_if = "Option::is_none")]
    pub acqrr: Option<PartyIdentification263>,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification263>,
    #[serde(rename = "Sndr", skip_serializing_if = "Option::is_none")]
    pub sndr: Option<PartyIdentification263>,
    #[serde(rename = "Rcvr", skip_serializing_if = "Option::is_none")]
    pub rcvr: Option<PartyIdentification263>,
    #[serde(rename = "Accptr", skip_serializing_if = "Option::is_none")]
    pub accptr: Option<PartyIdentification254>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyIdentification263>,
    #[serde(rename = "Pyer", skip_serializing_if = "Option::is_none")]
    pub pyer: Option<PartyIdentification257>,
    #[serde(rename = "Pyee", skip_serializing_if = "Option::is_none")]
    pub pyee: Option<PartyIdentification257>,
    #[serde(rename = "Termnl", skip_serializing_if = "Option::is_none")]
    pub termnl: Option<Terminal5>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification263>,
    #[validate]
    #[serde(rename = "Card")]
    pub card: CardData7,
    #[serde(rename = "CstmrDvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_dvc: Option<CustomerDevice4>,
    #[serde(rename = "Wllt", skip_serializing_if = "Option::is_none")]
    pub wllt: Option<Wallet2>,
    #[serde(rename = "Tkn", skip_serializing_if = "Option::is_none")]
    pub tkn: Option<Token2>,
    #[serde(rename = "Crdhldr", skip_serializing_if = "Option::is_none")]
    pub crdhldr: Option<Cardholder19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TerminalIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max16Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VehicleRentalService2 {
    #[serde(rename = "VhclRntlCpny", skip_serializing_if = "Option::is_none")]
    pub vhcl_rntl_cpny: Option<VehicleRentalCompany2>,
    #[serde(rename = "Cstmr", skip_serializing_if = "Option::is_none")]
    pub cstmr: Option<VehicleRentalCustomer2>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "RntlAgrmt", skip_serializing_if = "Option::is_none")]
    pub rntl_agrmt: Option<VehicleRentalAgreement2>,
    #[serde(rename = "RntlInvc", skip_serializing_if = "Option::is_none")]
    pub rntl_invc: Option<VehicleRentalInvoice2>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max5PositiveNumber {
    #[validate(range(min = 1,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptionFormat3Code {
    #[serde(rename = "TR34")]
    Tr34,
    #[serde(rename = "TR31")]
    Tr31,
    #[serde(rename = "CTCE")]
    Ctce,
    #[serde(rename = "CBCE")]
    Cbce,
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
pub struct PartyIdentification200 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType28Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType18Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CustomerAssigner1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "CRCY")]
    Crcy,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ONFL")]
    Onfl,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "TRAY")]
    Tray,
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
pub struct Instalment4 {
    #[serde(rename = "PmtSeqNb", skip_serializing_if = "Option::is_none")]
    pub pmt_seq_nb: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Plan", default)]
    pub plan: Vec<Plan2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DisputeData3 {
    #[serde(rename = "PresntmntCycl", skip_serializing_if = "Option::is_none")]
    pub presntmnt_cycl: Option<Exact1NumericText>,
    #[serde(rename = "DsptCond", skip_serializing_if = "Option::is_none")]
    pub dspt_cond: Option<Max35Text>,
    #[serde(rename = "DsptSts", skip_serializing_if = "Option::is_none")]
    pub dspt_sts: Option<Max35Text>,
    #[serde(rename = "PrtlDspt", skip_serializing_if = "Option::is_none")]
    pub prtl_dspt: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DsptRef", default)]
    pub dspt_ref: Vec<DisputeReference1>,
    #[serde(rename = "DcmnttnSts", skip_serializing_if = "Option::is_none")]
    pub dcmnttn_sts: Option<Max35Text>,
    #[serde(rename = "AddtlDsptData", skip_serializing_if = "Option::is_none")]
    pub addtl_dspt_data: Option<AdditionalData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DsptRjctRsn", default)]
    pub dspt_rjct_rsn: Vec<Max35Text>,
    #[serde(rename = "ChrgbckElgblty", skip_serializing_if = "Option::is_none")]
    pub chrgbck_elgblty: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TravelAgency3 {
    #[serde(rename = "Cpny", skip_serializing_if = "Option::is_none")]
    pub cpny: Option<PartyIdentification261>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrvlPackg", default)]
    pub trvl_packg: Vec<TravelAgencyPackage1>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FleetPurchaseType1Code {
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "NONF")]
    Nonf,
    #[serde(rename = "FANF")]
    Fanf,
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
pub struct AlgorithmIdentification23 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm18Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LodgingRoom1 {
    #[serde(rename = "RoomTp", skip_serializing_if = "Option::is_none")]
    pub room_tp: Option<Max35Text>,
    #[serde(rename = "RoomLctn", skip_serializing_if = "Option::is_none")]
    pub room_lctn: Option<Max35Text>,
    #[serde(rename = "BedTp", skip_serializing_if = "Option::is_none")]
    pub bed_tp: Option<Max70Text>,
    #[serde(rename = "GstsPerRoom", skip_serializing_if = "Option::is_none")]
    pub gsts_per_room: Option<Max3NumericText>,
    #[serde(rename = "AdltsInRoom", skip_serializing_if = "Option::is_none")]
    pub adlts_in_room: Option<Max3NumericText>,
    #[serde(rename = "ChldrnInRoom", skip_serializing_if = "Option::is_none")]
    pub chldrn_in_room: Option<Max3NumericText>,
    #[serde(rename = "DalyRoomRate", skip_serializing_if = "Option::is_none")]
    pub daly_room_rate: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification255 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[validate]
    #[serde(rename = "NmAndLctn")]
    pub nm_and_lctn: Max99Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max256Text>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<Max35Text>,
    #[serde(rename = "CstmrSvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_svc: Option<Max35Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max256Text>,
    #[serde(rename = "TaxRegnId", skip_serializing_if = "Option::is_none")]
    pub tax_regn_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpnsrdMrchnt", default)]
    pub spnsrd_mrchnt: Vec<SponsoredMerchant2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "HomePhneNb", skip_serializing_if = "Option::is_none")]
    pub home_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "BizPhneNb", skip_serializing_if = "Option::is_none")]
    pub biz_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobPhneNb", skip_serializing_if = "Option::is_none")]
    pub mob_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "OthrPhneNb", skip_serializing_if = "Option::is_none")]
    pub othr_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "PrsnlEmailAdr", skip_serializing_if = "Option::is_none")]
    pub prsnl_email_adr: Option<Max256Text>,
    #[serde(rename = "BizEmailAdr", skip_serializing_if = "Option::is_none")]
    pub biz_email_adr: Option<Max256Text>,
    #[serde(rename = "OthrEmailAdr", skip_serializing_if = "Option::is_none")]
    pub othr_email_adr: Option<Max256Text>,
    #[serde(rename = "HomeFaxNb", skip_serializing_if = "Option::is_none")]
    pub home_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "BizFaxNb", skip_serializing_if = "Option::is_none")]
    pub biz_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<Iso2ALanguageCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Content1 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20KText,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<Max140Binary>,
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OnLineCapability2Code {
    #[serde(rename = "OFLN")]
    Ofln,
    #[serde(rename = "ONLN")]
    Onln,
    #[serde(rename = "BOTH")]
    Both,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OutputFormat1Code {
    #[serde(rename = "MREF")]
    Mref,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "HTML")]
    Html,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReceiptType1Code {
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PAPR")]
    Papr,
    #[serde(rename = "SMSM")]
    Smsm,
    #[serde(rename = "URID")]
    Urid,
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
pub struct Customer7 {
    #[serde(rename = "CstmrId", skip_serializing_if = "Option::is_none")]
    pub cstmr_id: Option<Max35Text>,
    #[serde(rename = "CstmrDsgnt", skip_serializing_if = "Option::is_none")]
    pub cstmr_dsgnt: Option<Max2NumericText>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<CardholderName3>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "CtctInf", skip_serializing_if = "Option::is_none")]
    pub ctct_inf: Option<Contact6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Crdntls", default)]
    pub crdntls: Vec<Credentials2>,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<IsoMax3ACountryCode>,
    #[serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none")]
    pub ctry_of_birth: Option<IsoMax3ACountryCode>,
    #[serde(rename = "DtOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_of_birth: Option<IsoDate>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact2NumericText {
    #[validate(regex = "EXACT_2_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading10Code {
    #[serde(rename = "ICPY")]
    Icpy,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "ICCY")]
    Iccy,
    #[serde(rename = "MICR")]
    Micr,
    #[serde(rename = "MLEY")]
    Mley,
    #[serde(rename = "OCRR")]
    Ocrr,
    #[serde(rename = "MSIP")]
    Msip,
    #[serde(rename = "OPTC")]
    Optc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "RFID")]
    Rfid,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "KEEN")]
    Keen,
    #[serde(rename = "DFLE")]
    Dfle,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum JourneyType1Code {
    #[serde(rename = "COAC")]
    Coac,
    #[serde(rename = "EARL")]
    Earl,
    #[serde(rename = "FLGT")]
    Flgt,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "ONTM")]
    Ontm,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "VESS")]
    Vess,
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
pub struct Amount15 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "FctvXchgRate", skip_serializing_if = "Option::is_none")]
    pub fctv_xchg_rate: Option<BaseOne25Rate>,
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "ConvsTm", skip_serializing_if = "Option::is_none")]
    pub convs_tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification12 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter5>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebit3Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
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
pub struct Kek5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier2,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification23,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max500Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoyaltyProgramme2 {
    #[serde(rename = "PrgrmmTp", skip_serializing_if = "Option::is_none")]
    pub prgrmm_tp: Option<Max35Text>,
    #[serde(rename = "PrgmId", skip_serializing_if = "Option::is_none")]
    pub prgm_id: Option<Max70Text>,
    #[serde(rename = "PtcptId", skip_serializing_if = "Option::is_none")]
    pub ptcpt_id: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10KHexBinaryText {
    #[validate(regex = "MAX_10_K_HEX_BINARY_TEXT_REGEX")]
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
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProtectionMethod1Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SELM")]
    Selm,
    #[serde(rename = "SNCL")]
    Sncl,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "TEEN")]
    Teen,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm16Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
    #[serde(rename = "HS01")]
    Hs01,
    #[serde(rename = "SH31")]
    Sh31,
    #[serde(rename = "SH32")]
    Sh32,
    #[serde(rename = "SH33")]
    Sh33,
    #[serde(rename = "SH35")]
    Sh35,
    #[serde(rename = "SHK1")]
    Shk1,
    #[serde(rename = "SHK2")]
    Shk2,
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
pub struct Recipient7ChoiceEnum {
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier6>,
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport6>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient7Choice {
    #[serde(flatten)]
    pub value: Recipient7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TelecomServicesLineItem2 {
    #[serde(rename = "StartDtTm", skip_serializing_if = "Option::is_none")]
    pub start_dt_tm: Option<IsoDate>,
    #[serde(rename = "TmPrd", skip_serializing_if = "Option::is_none")]
    pub tm_prd: Option<Max35Text>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<IsoTime>,
    #[serde(rename = "CallFr", skip_serializing_if = "Option::is_none")]
    pub call_fr: Option<TelecomCallDetails2>,
    #[serde(rename = "CallTo", skip_serializing_if = "Option::is_none")]
    pub call_to: Option<TelecomCallDetails2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Chrg", default)]
    pub chrg: Vec<Amount20>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TtlTax", default)]
    pub ttl_tax: Vec<Tax39>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VehicleRentalAgreement2 {
    #[serde(rename = "AgrmtNb", skip_serializing_if = "Option::is_none")]
    pub agrmt_nb: Option<Max35Text>,
    #[serde(rename = "AdjstdInd", skip_serializing_if = "Option::is_none")]
    pub adjstd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "RntlLctn", skip_serializing_if = "Option::is_none")]
    pub rntl_lctn: Option<Address2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PckpLctn", default)]
    pub pckp_lctn: Vec<Address2>,
    #[serde(rename = "ChckOutDt", skip_serializing_if = "Option::is_none")]
    pub chck_out_dt: Option<IsoDate>,
    #[serde(rename = "ChckOutTm", skip_serializing_if = "Option::is_none")]
    pub chck_out_tm: Option<IsoTime>,
    #[serde(rename = "RtrLctn", skip_serializing_if = "Option::is_none")]
    pub rtr_lctn: Option<Address2>,
    #[serde(rename = "ChckInDt", skip_serializing_if = "Option::is_none")]
    pub chck_in_dt: Option<IsoDate>,
    #[serde(rename = "ChckInTm", skip_serializing_if = "Option::is_none")]
    pub chck_in_tm: Option<IsoTime>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "VhclClssDtls", skip_serializing_if = "Option::is_none")]
    pub vhcl_clss_dtls: Option<Vehicle4>,
    #[serde(rename = "TrvlDstnc", skip_serializing_if = "Option::is_none")]
    pub trvl_dstnc: Option<Distance1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RntlRate", default)]
    pub rntl_rate: Vec<RentalRate1>,
    #[serde(rename = "RntlDtls", skip_serializing_if = "Option::is_none")]
    pub rntl_dtls: Option<RentalDetails2>,
    #[serde(rename = "VhclRegnNb", skip_serializing_if = "Option::is_none")]
    pub vhcl_regn_nb: Option<Max70Text>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<Amount18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EstmtdTax", default)]
    pub estmtd_tax: Vec<Tax39>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DscntPrgrmm", default)]
    pub dscnt_prgrmm: Vec<Discount3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LltyPrgrmm", default)]
    pub llty_prgrmm: Vec<LoyaltyProgramme3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalServiceResult1Code {
    #[serde(rename = "NOPF")]
    Nopf,
    #[serde(rename = "NOSP")]
    Nosp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PERF")]
    Perf,
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
pub struct ResultData7 {
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Response8Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max35Text>,
    #[serde(rename = "RsltDtls")]
    pub rslt_dtls: Iso8583ResponseCode,
    #[serde(rename = "OthrRsltDtls", skip_serializing_if = "Option::is_none")]
    pub othr_rslt_dtls: Option<Max35Text>,
    #[serde(
        rename = "TempScrCardDataReusePrtd",
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_scr_card_data_reuse_prtd: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlRsltInf", default)]
    pub addtl_rslt_inf: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestRate1Code {
    #[serde(rename = "GSRT")]
    Gsrt,
    #[serde(rename = "NTRT")]
    Ntrt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct Max9999HexBinaryText {
    #[validate(regex = "MAX_9999_HEX_BINARY_TEXT_REGEX")]
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
pub struct RelativeDistinguishedName1 {
    #[serde(rename = "AttrTp")]
    pub attr_tp: AttributeType1Code,
    #[validate]
    #[serde(rename = "AttrVal")]
    pub attr_val: Max140Text,
}
#[derive(
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
pub struct Max4000Text {
    #[validate(length(min = 1, max = 4000,))]
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
pub struct ShippingPackage2 {
    #[serde(rename = "TrckgNb", skip_serializing_if = "Option::is_none")]
    pub trckg_nb: Option<Max70Text>,
    #[serde(rename = "Spplr", skip_serializing_if = "Option::is_none")]
    pub spplr: Option<PartyIdentification260>,
    #[serde(rename = "PckpDt", skip_serializing_if = "Option::is_none")]
    pub pckp_dt: Option<IsoDate>,
    #[serde(rename = "PckpTm", skip_serializing_if = "Option::is_none")]
    pub pckp_tm: Option<IsoTime>,
    #[serde(rename = "Dlvry", skip_serializing_if = "Option::is_none")]
    pub dlvry: Option<DeliveryInformation4>,
    #[serde(rename = "Wght", skip_serializing_if = "Option::is_none")]
    pub wght: Option<UnitOfMeasure2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Pdct", default)]
    pub pdct: Vec<Product7>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FleetLineItem4 {
    #[validate]
    #[serde(rename = "FuelInd")]
    pub fuel_ind: TrueFalseIndicator,
    #[serde(rename = "SvcTp", skip_serializing_if = "Option::is_none")]
    pub svc_tp: Option<FleetServiceType1Code>,
    #[serde(rename = "FuelBrndCd", skip_serializing_if = "Option::is_none")]
    pub fuel_brnd_cd: Option<Max4Text>,
    #[serde(rename = "FleetPdctCd", skip_serializing_if = "Option::is_none")]
    pub fleet_pdct_cd: Option<Max4Text>,
    #[serde(rename = "FleetPdctCtgy", skip_serializing_if = "Option::is_none")]
    pub fleet_pdct_ctgy: Option<Max35Text>,
    #[serde(rename = "FleetPdctQlfr", skip_serializing_if = "Option::is_none")]
    pub fleet_pdct_qlfr: Option<Max6Text>,
    #[serde(rename = "FleetPdctCdAssgnr", skip_serializing_if = "Option::is_none")]
    pub fleet_pdct_cd_assgnr: Option<Max35Text>,
    #[serde(rename = "UnitPricTaxInd", skip_serializing_if = "Option::is_none")]
    pub unit_pric_tax_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<DecimalNumber>,
    #[serde(rename = "DscntAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NonTaxblInd", skip_serializing_if = "Option::is_none")]
    pub non_taxbl_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "TtlAmtExclgTax", skip_serializing_if = "Option::is_none")]
    pub ttl_amt_exclg_tax: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TtlAmtInclgTax", skip_serializing_if = "Option::is_none")]
    pub ttl_amt_inclg_tax: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification257 {
    #[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
    pub fi: Option<FinancialInstitution7>,
    #[serde(rename = "Cstmr", skip_serializing_if = "Option::is_none")]
    pub cstmr: Option<Customer7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvelopedData6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient7Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent5>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PinEntrySecurityCharacteristic1Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SECS")]
    Secs,
    #[serde(rename = "SECH")]
    Sech,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType26Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ICCA")]
    Icca,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct Driver2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max70Text>,
    #[serde(rename = "DrvgLic", skip_serializing_if = "Option::is_none")]
    pub drvg_lic: Option<DrivingLicense2>,
    #[serde(rename = "Mplyr", skip_serializing_if = "Option::is_none")]
    pub mplyr: Option<Max70Text>,
    #[serde(rename = "MplyeeId", skip_serializing_if = "Option::is_none")]
    pub mplyee_id: Option<Max70Text>,
    #[serde(rename = "DeptNb", skip_serializing_if = "Option::is_none")]
    pub dept_nb: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlId", default)]
    pub addtl_id: Vec<TravelDocument2>,
    #[serde(rename = "DtOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_of_birth: Option<IsoDate>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FleetServiceType1Code {
    #[serde(rename = "SLSV")]
    Slsv,
    #[serde(rename = "HSDI")]
    Hsdi,
    #[serde(rename = "FLSV")]
    Flsv,
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
pub struct Contact3 {
    #[serde(rename = "CntrlPhneNb", skip_serializing_if = "Option::is_none")]
    pub cntrl_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "PrprtyPhneNb", skip_serializing_if = "Option::is_none")]
    pub prprty_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "TollFreePhneNb", skip_serializing_if = "Option::is_none")]
    pub toll_free_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoyaltyMember2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<CardholderName3>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "MmbSts", skip_serializing_if = "Option::is_none")]
    pub mmb_sts: Option<Max35Text>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDate>,
    #[serde(rename = "LltyVal", skip_serializing_if = "Option::is_none")]
    pub llty_val: Option<Max10NumericText>,
    #[serde(rename = "LltyValTp", skip_serializing_if = "Option::is_none")]
    pub llty_val_tp: Option<LoyaltyValueType1Code>,
    #[serde(rename = "OthrLltyValTp", skip_serializing_if = "Option::is_none")]
    pub othr_llty_val_tp: Option<Max35Text>,
    #[serde(rename = "ValToCdt", skip_serializing_if = "Option::is_none")]
    pub val_to_cdt: Option<Max10NumericText>,
    #[serde(rename = "ValToDbt", skip_serializing_if = "Option::is_none")]
    pub val_to_dbt: Option<Max10NumericText>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<Max10NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType20 {
    #[validate]
    #[serde(rename = "MACData")]
    pub mac_data: MacData1,
    #[validate]
    #[serde(rename = "MAC")]
    pub mac: Max8HexBinaryText,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max104Text {
    #[validate(length(min = 1, max = 104,))]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType8Code {
    #[serde(rename = "APPV")]
    Appv,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "DCLN")]
    Dcln,
    #[serde(rename = "RQID")]
    Rqid,
    #[serde(rename = "NDCL")]
    Ndcl,
    #[serde(rename = "RFRL")]
    Rfrl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "STUA")]
    Stua,
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
pub struct AdditionalInformation20 {
    #[serde(rename = "Rcpt", skip_serializing_if = "Option::is_none")]
    pub rcpt: Option<PartyType22Code>,
    #[serde(rename = "OthrRcpt", skip_serializing_if = "Option::is_none")]
    pub othr_rcpt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Trgt", default)]
    pub trgt: Vec<UserInterface6Code>,
    #[serde(rename = "OthrTrgt", skip_serializing_if = "Option::is_none")]
    pub othr_trgt: Option<Max35Text>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat4Code>,
    #[serde(rename = "OthrFrmt", skip_serializing_if = "Option::is_none")]
    pub othr_frmt: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20KText,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddendumData3 {
    #[serde(rename = "PurchsIdrTp", skip_serializing_if = "Option::is_none")]
    pub purchs_idr_tp: Option<PurchaseIdentifierType1Code>,
    #[serde(rename = "OthrPurchsIdrTp", skip_serializing_if = "Option::is_none")]
    pub othr_purchs_idr_tp: Option<Max35Text>,
    #[serde(rename = "PurchsIdr", skip_serializing_if = "Option::is_none")]
    pub purchs_idr: Option<Max99Text>,
    #[serde(rename = "AddtlAccptrData", skip_serializing_if = "Option::is_none")]
    pub addtl_accptr_data: Option<AdditionalAcceptorData1>,
    #[serde(rename = "Cstmr", skip_serializing_if = "Option::is_none")]
    pub cstmr: Option<Customer4>,
    #[serde(rename = "Sale", skip_serializing_if = "Option::is_none")]
    pub sale: Option<Sale2>,
    #[serde(rename = "Fleet", skip_serializing_if = "Option::is_none")]
    pub fleet: Option<FleetData4>,
    #[serde(rename = "Invc", skip_serializing_if = "Option::is_none")]
    pub invc: Option<Invoice2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrvlAgcy", default)]
    pub trvl_agcy: Vec<TravelAgency3>,
    #[serde(rename = "PssngrTrnsprt", skip_serializing_if = "Option::is_none")]
    pub pssngr_trnsprt: Option<PassengerTransport2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VhclRntl", default)]
    pub vhcl_rntl: Vec<VehicleRentalService2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ldgg", default)]
    pub ldgg: Vec<Lodging3>,
    #[serde(rename = "ShppgData", skip_serializing_if = "Option::is_none")]
    pub shppg_data: Option<ShippingData2>,
    #[serde(rename = "TelecomSvcs", skip_serializing_if = "Option::is_none")]
    pub telecom_svcs: Option<TelecomServices2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TempSvcs", default)]
    pub temp_svcs: Vec<TemporaryServices2>,
    #[serde(rename = "Instlmt", skip_serializing_if = "Option::is_none")]
    pub instlmt: Option<Instalment4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification183 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType17Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType18Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TemporaryServices2 {
    #[serde(rename = "CtrctgCpny", skip_serializing_if = "Option::is_none")]
    pub ctrctg_cpny: Option<TemporaryServicesCompany2>,
    #[serde(rename = "TempMplyee", skip_serializing_if = "Option::is_none")]
    pub temp_mplyee: Option<PartyIdentification210>,
    #[serde(rename = "Job", skip_serializing_if = "Option::is_none")]
    pub job: Option<TemporaryServicesJob1>,
    #[serde(rename = "FlatRateInd", skip_serializing_if = "Option::is_none")]
    pub flat_rate_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DscntAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "Labr", skip_serializing_if = "Option::is_none")]
    pub labr: Option<TemporaryServicesLabor1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MiscExpnss", default)]
    pub misc_expnss: Vec<Amount13>,
    #[serde(rename = "SbttlAmt", skip_serializing_if = "Option::is_none")]
    pub sbttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount19Code {
    #[serde(rename = "CONN")]
    Conn,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "LNDS")]
    Lnds,
    #[serde(rename = "MISC")]
    Misc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "USGE")]
    Usge,
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
pub struct TravelDocument2 {
    #[serde(rename = "Tp")]
    pub tp: OfficialDocumentType1Code,
    #[serde(rename = "Form", skip_serializing_if = "Option::is_none")]
    pub form: Option<PresentationMedium2Code>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max70Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max70Text>,
    #[serde(rename = "IssncDt", skip_serializing_if = "Option::is_none")]
    pub issnc_dt: Option<IsoDate>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Sale2 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<SaleSummary1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<SaleItem3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Credentials2 {
    #[serde(rename = "IdCd")]
    pub id_cd: Identification3Code,
    #[serde(rename = "OthrIdCd", skip_serializing_if = "Option::is_none")]
    pub othr_id_cd: Option<Max35Text>,
    #[validate]
    #[serde(rename = "IdVal")]
    pub id_val: Max70Text,
    #[serde(rename = "IdXpryDt", skip_serializing_if = "Option::is_none")]
    pub id_xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "AssgnrAuthrty", skip_serializing_if = "Option::is_none")]
    pub assgnr_authrty: Option<Authority1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm8Code {
    #[serde(rename = "MGF1")]
    Mgf1,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionAttribute2Code {
    #[serde(rename = "AGGR")]
    Aggr,
    #[serde(rename = "CADB")]
    Cadb,
    #[serde(rename = "CPLT")]
    Cplt,
    #[serde(rename = "DBRC")]
    Dbrc,
    #[serde(rename = "DBRP")]
    Dbrp,
    #[serde(rename = "DFRD")]
    Dfrd,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "FRCP")]
    Frcp,
    #[serde(rename = "INST")]
    Inst,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PAUT")]
    Paut,
    #[serde(rename = "PACP")]
    Pacp,
    #[serde(rename = "PPYT")]
    Ppyt,
    #[serde(rename = "RCPT")]
    Rcpt,
    #[serde(rename = "SUBR")]
    Subr,
    #[serde(rename = "TPUP")]
    Tpup,
    #[serde(rename = "UCOF")]
    Ucof,
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
pub struct AlgorithmIdentification28 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm13Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter14>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max140Binary {
    #[validate(length(min = 1, max = 140,), regex = "MAX_140_BINARY_REGEX")]
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
pub struct KekIdentifier2 {
    #[validate]
    #[serde(rename = "KeyId")]
    pub key_id: Max140Text,
    #[validate]
    #[serde(rename = "KeyVrsn")]
    pub key_vrsn: Max140Text,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "DerivtnId", skip_serializing_if = "Option::is_none")]
    pub derivtn_id: Option<Min5Max16Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "HomePhneNb", skip_serializing_if = "Option::is_none")]
    pub home_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "BizPhneNb", skip_serializing_if = "Option::is_none")]
    pub biz_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobPhneNb", skip_serializing_if = "Option::is_none")]
    pub mob_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "OthrPhneNb", skip_serializing_if = "Option::is_none")]
    pub othr_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "PrsnlEmailAdr", skip_serializing_if = "Option::is_none")]
    pub prsnl_email_adr: Option<Max256Text>,
    #[serde(rename = "BizEmailAdr", skip_serializing_if = "Option::is_none")]
    pub biz_email_adr: Option<Max256Text>,
    #[serde(rename = "OthrEmailAdr", skip_serializing_if = "Option::is_none")]
    pub othr_email_adr: Option<Max256Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<Iso2ALanguageCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionAmounts2 {
    #[serde(rename = "AmtQlfr", skip_serializing_if = "Option::is_none")]
    pub amt_qlfr: Option<TypeOfAmount22Code>,
    #[validate]
    #[serde(rename = "TxAmt")]
    pub tx_amt: TransactionAmount1,
    #[serde(rename = "CrdhldrBllgAmt", skip_serializing_if = "Option::is_none")]
    pub crdhldr_bllg_amt: Option<Amount15>,
    #[serde(rename = "RcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub rcncltn_amt: Option<Amount15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DtldAmt", default)]
    pub dtld_amt: Vec<DetailedAmount22>,
    #[serde(rename = "OrgnlTxAmts", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_amts: Option<OriginalTransactionAmount2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Transaction145 {
    #[serde(rename = "TxTp")]
    pub tx_tp: Iso8583TransactionTypeCode,
    #[serde(rename = "TxSubTp", skip_serializing_if = "Option::is_none")]
    pub tx_sub_tp: Option<Max35Text>,
    #[serde(rename = "TxAttr", skip_serializing_if = "Option::is_none")]
    pub tx_attr: Option<TransactionAttribute2Code>,
    #[serde(rename = "OthrTxAttr", skip_serializing_if = "Option::is_none")]
    pub othr_tx_attr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgRsn", default)]
    pub msg_rsn: Vec<Iso8583MessageReasonCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnMsgRsn", default)]
    pub altrn_msg_rsn: Vec<Max256Text>,
    #[serde(rename = "PreAuthstnTmLmt", skip_serializing_if = "Option::is_none")]
    pub pre_authstn_tm_lmt: Option<Max6NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSvc", default)]
    pub addtl_svc: Vec<AdditionalService2>,
    #[serde(rename = "AssoctdDataInd", skip_serializing_if = "Option::is_none")]
    pub assoctd_data_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "AssoctdDataRef", skip_serializing_if = "Option::is_none")]
    pub assoctd_data_ref: Option<Max70Text>,
    #[serde(rename = "AssoctdDataDstn", skip_serializing_if = "Option::is_none")]
    pub assoctd_data_dstn: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpclPrgrmmQlfctn", default)]
    pub spcl_prgrmm_qlfctn: Vec<SpecialProgrammeQualification1>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentification16,
    #[validate(length(min = 0,))]
    #[serde(rename = "DsptData", default)]
    pub dspt_data: Vec<DisputeData3>,
    #[validate]
    #[serde(rename = "TxAmts")]
    pub tx_amts: TransactionAmounts2,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<AdditionalAmounts3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlFee", default)]
    pub addtl_fee: Vec<AdditionalFee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OrgnlAddtlFee", default)]
    pub orgnl_addtl_fee: Vec<AdditionalFee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DpstDtls", default)]
    pub dpst_dtls: Vec<DepositDetails2>,
    #[serde(rename = "FndsSvcs", skip_serializing_if = "Option::is_none")]
    pub fnds_svcs: Option<FundingService2>,
    #[serde(rename = "AcctFr", skip_serializing_if = "Option::is_none")]
    pub acct_fr: Option<AccountDetails3>,
    #[serde(rename = "AcctTo", skip_serializing_if = "Option::is_none")]
    pub acct_to: Option<AccountDetails3>,
    #[serde(rename = "TxDesc", skip_serializing_if = "Option::is_none")]
    pub tx_desc: Option<Max1000Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation20>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardholderName3 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max35Text>,
    #[serde(rename = "LastNm", skip_serializing_if = "Option::is_none")]
    pub last_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalCharacteristicDetails1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "PrvddBy", skip_serializing_if = "Option::is_none")]
    pub prvdd_by: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalAcceptorData1 {
    #[serde(rename = "AddtlTxRefNb", skip_serializing_if = "Option::is_none")]
    pub addtl_tx_ref_nb: Option<Max70Text>,
    #[serde(rename = "TaxRegnId", skip_serializing_if = "Option::is_none")]
    pub tax_regn_id: Option<Max70Text>,
    #[serde(rename = "CorpTaxId", skip_serializing_if = "Option::is_none")]
    pub corp_tax_id: Option<Max35Text>,
    #[serde(rename = "CorpTaxIdTp", skip_serializing_if = "Option::is_none")]
    pub corp_tax_id_tp: Option<CorporateTaxType1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlId", default)]
    pub addtl_id: Vec<AdditionalIdentification1>,
    #[serde(rename = "Chrtcs", skip_serializing_if = "Option::is_none")]
    pub chrtcs: Option<AdditionalCharacteristics1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GoodAndServiceDeliveryChannel1Code {
    #[serde(rename = "EDEL")]
    Edel,
    #[serde(rename = "PULC")]
    Pulc,
    #[serde(rename = "NDEL")]
    Ndel,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SCBA")]
    Scba,
    #[serde(rename = "SCSA")]
    Scsa,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Verification3Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "FUTA")]
    Futa,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "NOSP")]
    Nosp,
    #[serde(rename = "NOVF")]
    Novf,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SUCC")]
    Succ,
    #[serde(rename = "ERRR")]
    Errr,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StorageLocation1Code {
    #[serde(rename = "CAWL")]
    Cawl,
    #[serde(rename = "DVCE")]
    Dvce,
    #[serde(rename = "ISWL")]
    Iswl,
    #[serde(rename = "ONFL")]
    Onfl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "TPWL")]
    Tpwl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LoyaltyValueType1Code {
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "MONE")]
    Mone,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "POIN")]
    Poin,
    #[serde(rename = "PRIV")]
    Priv,
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
pub struct Parameter9 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm16Code>,
}
#[derive(
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
pub struct Verification5 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAuthenticationMethod1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VrfctnInf", default)]
    pub vrfctn_inf: Vec<VerificationInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VrfctnRslt", default)]
    pub vrfctn_rslt: Vec<VerificationResult2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Adjustment12 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvoiceLineItem2 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "OrdrDt", skip_serializing_if = "Option::is_none")]
    pub ordr_dt: Option<IsoDate>,
    #[serde(rename = "CtrctNb", skip_serializing_if = "Option::is_none")]
    pub ctrct_nb: Option<Max70Text>,
    #[serde(rename = "ShppgDt", skip_serializing_if = "Option::is_none")]
    pub shppg_dt: Option<IsoDate>,
    #[serde(rename = "RbllgInd", skip_serializing_if = "Option::is_none")]
    pub rbllg_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "MdclSvcsInd", skip_serializing_if = "Option::is_none")]
    pub mdcl_svcs_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ShipToIndstryCd", skip_serializing_if = "Option::is_none")]
    pub ship_to_indstry_cd: Option<Max50Text>,
    #[serde(rename = "PdctCd", skip_serializing_if = "Option::is_none")]
    pub pdct_cd: Option<Max70Text>,
    #[serde(rename = "PdctQlfr", skip_serializing_if = "Option::is_none")]
    pub pdct_qlfr: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
    #[serde(rename = "TpOfSpply", skip_serializing_if = "Option::is_none")]
    pub tp_of_spply: Option<Max10Text>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<DecimalNumber>,
    #[serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none")]
    pub adjstmnt: Option<Adjustment12>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "InsrncAmt", skip_serializing_if = "Option::is_none")]
    pub insrnc_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "UnqVATInvcRef", skip_serializing_if = "Option::is_none")]
    pub unq_vat_invc_ref: Option<Max35Text>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "ZeroCostToCstmrInd", skip_serializing_if = "Option::is_none")]
    pub zero_cost_to_cstmr_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Signer4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "SgnrId", skip_serializing_if = "Option::is_none")]
    pub sgnr_id: Option<Recipient5Choice>,
    #[validate]
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: AlgorithmIdentification21,
    #[validate(length(min = 0,))]
    #[serde(rename = "SgndAttrbts", default)]
    pub sgnd_attrbts: Vec<GenericInformation1>,
    #[validate]
    #[serde(rename = "SgntrAlgo")]
    pub sgntr_algo: AlgorithmIdentification20,
    #[validate]
    #[serde(rename = "Sgntr")]
    pub sgntr: Max3000Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestRate1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<InstalmentPeriod1Code>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: BaseOneRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvoiceSummary2 {
    #[serde(rename = "InvcNb", skip_serializing_if = "Option::is_none")]
    pub invc_nb: Option<Max70Text>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<PartyIdentification259>,
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<PartyIdentification259>,
    #[serde(rename = "InvcDt", skip_serializing_if = "Option::is_none")]
    pub invc_dt: Option<IsoDate>,
    #[serde(rename = "InvcCreDtTm", skip_serializing_if = "Option::is_none")]
    pub invc_cre_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "FrghtAmt", skip_serializing_if = "Option::is_none")]
    pub frght_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxTtl", default)]
    pub tax_ttl: Vec<Tax39>,
    #[serde(rename = "TaxRclmMtd", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_mtd: Option<TaxReclaimMethod1Code>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxReclaimMethod1Code {
    #[serde(rename = "INPP")]
    Inpp,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "INSU")]
    Insu,
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
pub struct OriginalDataElements2 {
    #[serde(rename = "MsgClss", skip_serializing_if = "Option::is_none")]
    pub msg_clss: Option<MessageClass1Code>,
    #[serde(rename = "MsgFctn", skip_serializing_if = "Option::is_none")]
    pub msg_fctn: Option<MessageFunction16Code>,
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<Iso8583TransactionTypeCode>,
    #[serde(rename = "AcqrrId", skip_serializing_if = "Option::is_none")]
    pub acqrr_id: Option<Max11NumericText>,
    #[serde(rename = "SndrId", skip_serializing_if = "Option::is_none")]
    pub sndr_id: Option<Max11NumericText>,
    #[serde(rename = "LclDt", skip_serializing_if = "Option::is_none")]
    pub lcl_dt: Option<IsoDate>,
    #[serde(rename = "LclTm", skip_serializing_if = "Option::is_none")]
    pub lcl_tm: Option<IsoTime>,
    #[serde(rename = "TmZone", skip_serializing_if = "Option::is_none")]
    pub tm_zone: Option<Max70Text>,
    #[serde(rename = "TxRef", skip_serializing_if = "Option::is_none")]
    pub tx_ref: Option<Max35Text>,
    #[serde(rename = "TrnsmssnDtTm", skip_serializing_if = "Option::is_none")]
    pub trnsmssn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "SysTracAudtNb", skip_serializing_if = "Option::is_none")]
    pub sys_trac_audt_nb: Option<Max12NumericText>,
    #[serde(rename = "RtrvlRefNb", skip_serializing_if = "Option::is_none")]
    pub rtrvl_ref_nb: Option<Exact12Text>,
    #[serde(rename = "LifeCyclSpprt", skip_serializing_if = "Option::is_none")]
    pub life_cycl_spprt: Option<LifeCycleSupport1Code>,
    #[serde(rename = "LifeCyclTracIdData", skip_serializing_if = "Option::is_none")]
    pub life_cycl_trac_id_data: Option<TransactionLifeCycleIdentification1>,
    #[serde(
        rename = "LifeCyclTracIdMssng",
        skip_serializing_if = "Option::is_none"
    )]
    pub life_cycl_trac_id_mssng: Option<Max70Text>,
    #[serde(rename = "AcqrrRefData", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_data: Option<Max140Text>,
    #[serde(rename = "AcqrrRefNb", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_nb: Option<Max23NumericText>,
    #[serde(rename = "CardIssrRefData", skip_serializing_if = "Option::is_none")]
    pub card_issr_ref_data: Option<Max1000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification259 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification258>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact3>,
    #[serde(rename = "TaxRegnId", skip_serializing_if = "Option::is_none")]
    pub tax_regn_id: Option<Max70Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max76Text {
    #[validate(length(min = 1, max = 76,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Response8Code {
    #[serde(rename = "PRCS")]
    Prcs,
    #[serde(rename = "UNPR")]
    Unpr,
    #[serde(rename = "UNRV")]
    Unrv,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "TECH")]
    Tech,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct DrivingLicense2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max70Text>,
    #[serde(rename = "Form", skip_serializing_if = "Option::is_none")]
    pub form: Option<PresentationMedium2Code>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max70Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<LegalStructure1Code>,
    #[serde(rename = "IssncDt", skip_serializing_if = "Option::is_none")]
    pub issnc_dt: Option<IsoDate>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub stat: Option<Max16Text>,
    #[serde(rename = "Prvc", skip_serializing_if = "Option::is_none")]
    pub prvc: Option<Max16Text>,
    #[serde(rename = "OthrAuthrty", skip_serializing_if = "Option::is_none")]
    pub othr_authrty: Option<Max16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionLifeCycleIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact15Text,
    #[serde(rename = "AuthstnSeqNb", skip_serializing_if = "Option::is_none")]
    pub authstn_seq_nb: Option<Exact2NumericText>,
    #[serde(rename = "PresntmntSeqNb", skip_serializing_if = "Option::is_none")]
    pub presntmnt_seq_nb: Option<Exact2NumericText>,
    #[serde(rename = "PresntmntSeqCnt", skip_serializing_if = "Option::is_none")]
    pub presntmnt_seq_cnt: Option<Exact2NumericText>,
    #[serde(rename = "AuthntcnTkn", skip_serializing_if = "Option::is_none")]
    pub authntcn_tkn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Distance1 {
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure10Code>,
    #[serde(rename = "OdmtrStart", skip_serializing_if = "Option::is_none")]
    pub odmtr_start: Option<Max10NumericText>,
    #[serde(rename = "OdmtrRtr", skip_serializing_if = "Option::is_none")]
    pub odmtr_rtr: Option<Max10NumericText>,
    #[serde(rename = "TtlDstnc", skip_serializing_if = "Option::is_none")]
    pub ttl_dstnc: Option<Max10NumericText>,
    #[serde(rename = "FreeDstnc", skip_serializing_if = "Option::is_none")]
    pub free_dstnc: Option<Max10NumericText>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact3NumericText {
    #[validate(regex = "EXACT_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExchangeRateAgreementType1Code {
    #[serde(rename = "FWCT")]
    Fwct,
    #[serde(rename = "NORM")]
    Norm,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SPOT")]
    Spot,
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
pub struct Max20Text {
    #[validate(length(min = 1, max = 20,))]
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
pub struct Invoice2 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<InvoiceSummary2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<InvoiceLineItem2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AncillaryPurchase2 {
    #[serde(rename = "AncllryDocNb", skip_serializing_if = "Option::is_none")]
    pub ancllry_doc_nb: Option<Max15Text>,
    #[serde(rename = "RltdDocNb", skip_serializing_if = "Option::is_none")]
    pub rltd_doc_nb: Option<Max15Text>,
    #[serde(rename = "SvcCtgyCd", skip_serializing_if = "Option::is_none")]
    pub svc_ctgy_cd: Option<Max4Text>,
    #[serde(rename = "SvcSubCtgyCd", skip_serializing_if = "Option::is_none")]
    pub svc_sub_ctgy_cd: Option<Max4Text>,
    #[serde(rename = "SvcPrvdrSvcTp", skip_serializing_if = "Option::is_none")]
    pub svc_prvdr_svc_tp: Option<Max35Text>,
    #[serde(rename = "CdtRsnCd", skip_serializing_if = "Option::is_none")]
    pub cdt_rsn_cd: Option<Max35Text>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<Amount16>,
    #[serde(rename = "Fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification261 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "IATACd", skip_serializing_if = "Option::is_none")]
    pub iata_cd: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient5ChoiceEnum {
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient5Choice {
    #[serde(flatten)]
    pub value: Recipient5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingResult16 {
    #[serde(rename = "RspnSrc", skip_serializing_if = "Option::is_none")]
    pub rspn_src: Option<ApprovalEntity2>,
    #[serde(rename = "RsltData", skip_serializing_if = "Option::is_none")]
    pub rslt_data: Option<ResultData7>,
    #[serde(rename = "ApprvlCd", skip_serializing_if = "Option::is_none")]
    pub apprvl_cd: Option<Exact6AlphaNumericText>,
    #[serde(rename = "OrgnlRsltData", skip_serializing_if = "Option::is_none")]
    pub orgnl_rslt_data: Option<ResultData7>,
    #[serde(rename = "ActnReqrd", skip_serializing_if = "Option::is_none")]
    pub actn_reqrd: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Actn", default)]
    pub actn: Vec<Action13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlActn", default)]
    pub addtl_actn: Vec<AdditionalAction1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CryptographicKey13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Binary>,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CryptographicKeyType3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Fctn", default)]
    pub fctn: Vec<KeyUsage1Code>,
    #[serde(rename = "ActvtnDt", skip_serializing_if = "Option::is_none")]
    pub actvtn_dt: Option<IsoDateTime>,
    #[serde(rename = "DeactvtnDt", skip_serializing_if = "Option::is_none")]
    pub deactvtn_dt: Option<IsoDateTime>,
    #[serde(rename = "KeyVal", skip_serializing_if = "Option::is_none")]
    pub key_val: Option<ContentInformationType19>,
    #[serde(rename = "KeyChckVal", skip_serializing_if = "Option::is_none")]
    pub key_chck_val: Option<Max35Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlMgmtInf", default)]
    pub addtl_mgmt_inf: Vec<GenericInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max15Text {
    #[validate(length(min = 1, max = 15,))]
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
pub struct AuthenticatedData5 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient6Choice>,
    #[validate]
    #[serde(rename = "MACAlgo")]
    pub mac_algo: AlgorithmIdentification22,
    #[validate]
    #[serde(rename = "NcpsltdCntt")]
    pub ncpsltd_cntt: EncapsulatedContent3,
    #[validate]
    #[serde(rename = "MAC")]
    pub mac: Max140Binary,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CarRentalServiceType2Code {
    #[serde(rename = "ADJM")]
    Adjm,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BAST")]
    Bast,
    #[serde(rename = "CLEA")]
    Clea,
    #[serde(rename = "DMGS")]
    Dmgs,
    #[serde(rename = "DLVR")]
    Dlvr,
    #[serde(rename = "DPOF")]
    Dpof,
    #[serde(rename = "ENTE")]
    Ente,
    #[serde(rename = "EXTC")]
    Extc,
    #[serde(rename = "EXDY")]
    Exdy,
    #[serde(rename = "EXDI")]
    Exdi,
    #[serde(rename = "EXHR")]
    Exhr,
    #[serde(rename = "FINE")]
    Fine,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "GARA")]
    Gara,
    #[serde(rename = "GPSY")]
    Gpsy,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIIN")]
    Liin,
    #[serde(rename = "LDIN")]
    Ldin,
    #[serde(rename = "MISC")]
    Misc,
    #[serde(rename = "NAVI")]
    Navi,
    #[serde(rename = "NOSH")]
    Nosh,
    #[serde(rename = "ONEW")]
    Onew,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PARK")]
    Park,
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "PFIN")]
    Pfin,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "REGD")]
    Regd,
    #[serde(rename = "SMOK")]
    Smok,
    #[serde(rename = "TOLL")]
    Toll,
    #[serde(rename = "TOWI")]
    Towi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoiComponentType5Code {
    #[serde(rename = "AQPP")]
    Aqpp,
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "TLPR")]
    Tlpr,
    #[serde(rename = "SCPR")]
    Scpr,
    #[serde(rename = "SERV")]
    Serv,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "DVCE")]
    Dvce,
    #[serde(rename = "SECM")]
    Secm,
    #[serde(rename = "APLI")]
    Apli,
    #[serde(rename = "EMVK")]
    Emvk,
    #[serde(rename = "EMVO")]
    Emvo,
    #[serde(rename = "MDWR")]
    Mdwr,
    #[serde(rename = "DRVR")]
    Drvr,
    #[serde(rename = "OPST")]
    Opst,
    #[serde(rename = "MRPR")]
    Mrpr,
    #[serde(rename = "CRTF")]
    Crtf,
    #[serde(rename = "TMSP")]
    Tmsp,
    #[serde(rename = "SACP")]
    Sacp,
    #[serde(rename = "SAPR")]
    Sapr,
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
pub struct Max19HexBinaryText {
    #[validate(regex = "MAX_19_HEX_BINARY_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CustomerType2Code {
    #[serde(rename = "CSMR")]
    Csmr,
    #[serde(rename = "CPNY")]
    Cpny,
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
pub struct Discount3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter5 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm11Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PeriodUnit3Code {
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "EXDY")]
    Exdy,
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
pub struct Terminal4 {
    #[validate]
    #[serde(rename = "TermnlId")]
    pub termnl_id: TerminalIdentification3,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TerminalType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<Capabilities2>,
    #[serde(rename = "TermnlIntgtn", skip_serializing_if = "Option::is_none")]
    pub termnl_intgtn: Option<TerminalIntegrationCategory1Code>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "OutdrInd", skip_serializing_if = "Option::is_none")]
    pub outdr_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "OffPrmissInd", skip_serializing_if = "Option::is_none")]
    pub off_prmiss_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "OnBrdInd", skip_serializing_if = "Option::is_none")]
    pub on_brd_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "POICmpnt", default)]
    pub poi_cmpnt: Vec<PointOfInteractionComponent13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerAndSerialNumber1 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: CertificateIssuer1,
    #[validate]
    #[serde(rename = "SrlNb")]
    pub srl_nb: Max35Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max12NumericText {
    #[validate(regex = "MAX_12_NUMERIC_TEXT_REGEX")]
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
pub struct AdditionalAmounts3 {
    #[serde(rename = "Tp")]
    pub tp: Iso8583AmountTypeCode,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: Amount17,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KekIdentifier6 {
    #[validate]
    #[serde(rename = "KeyId")]
    pub key_id: Max140Text,
    #[serde(rename = "KeyVrsn", skip_serializing_if = "Option::is_none")]
    pub key_vrsn: Option<Max140Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "DerivtnId", skip_serializing_if = "Option::is_none")]
    pub derivtn_id: Option<Min5Max16Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoyaltyProgramme3 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PrgrmmElgbltyInd", default)]
    pub prgrmm_elgblty_ind: Vec<TrueFalseIndicator>,
    #[serde(rename = "PrgrmmIssr", skip_serializing_if = "Option::is_none")]
    pub prgrmm_issr: Option<Max35Text>,
    #[serde(rename = "LltyMmb", skip_serializing_if = "Option::is_none")]
    pub llty_mmb: Option<LoyaltyMember2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GoodsAndServices1Code {
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "ELPH")]
    Elph,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecurityCharacteristics1Code {
    #[serde(rename = "CETE")]
    Cete,
    #[serde(rename = "CPTE")]
    Cpte,
    #[serde(rename = "CENC")]
    Cenc,
    #[serde(rename = "CMAC")]
    Cmac,
    #[serde(rename = "ETEE")]
    Etee,
    #[serde(rename = "METE")]
    Mete,
    #[serde(rename = "MPTE")]
    Mpte,
    #[serde(rename = "OPNN")]
    Opnn,
    #[serde(rename = "PMAC")]
    Pmac,
    #[serde(rename = "PKIE")]
    Pkie,
    #[serde(rename = "PRAE")]
    Prae,
    #[serde(rename = "PRAM")]
    Pram,
    #[serde(rename = "PRVN")]
    Prvn,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "APTE")]
    Apte,
    #[serde(rename = "AETE")]
    Aete,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct DeviceOperatingSystem1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max70Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DeviceOperatingSystemType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max35Text>,
    #[serde(rename = "Bld", skip_serializing_if = "Option::is_none")]
    pub bld: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalData3 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<CardholderName2>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FleetDiscountTotals1 {
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "FuelAmt", skip_serializing_if = "Option::is_none")]
    pub fuel_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NonFuelAmt", skip_serializing_if = "Option::is_none")]
    pub non_fuel_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionAmount1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy")]
    pub ccy: Iso3NumericCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ContentType3Code {
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "IFSE")]
    Ifse,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OfficialDocumentType1Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "DIPL")]
    Dipl,
    #[serde(rename = "DVLC")]
    Dvlc,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "IDEN")]
    Iden,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "INPO")]
    Inpo,
    #[serde(rename = "LZPR")]
    Lzpr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "PERM")]
    Perm,
    #[serde(rename = "REFU")]
    Refu,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Moto1Code {
    #[serde(rename = "MAOR")]
    Maor,
    #[serde(rename = "MOTO")]
    Moto,
    #[serde(rename = "TPOR")]
    Tpor,
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
pub struct SaleContext8 {
    #[serde(rename = "SaleId", skip_serializing_if = "Option::is_none")]
    pub sale_id: Option<Max35Text>,
    #[serde(rename = "SaleRefId", skip_serializing_if = "Option::is_none")]
    pub sale_ref_id: Option<Max35Text>,
    #[serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none")]
    pub sale_ref_nb: Option<Max35Text>,
    #[serde(rename = "GoodsAndSvcsTp", skip_serializing_if = "Option::is_none")]
    pub goods_and_svcs_tp: Option<GoodsAndServices1Code>,
    #[serde(rename = "GoodAndSvcsSubTp", skip_serializing_if = "Option::is_none")]
    pub good_and_svcs_sub_tp: Option<GoodsAndServicesSubType1Code>,
    #[serde(
        rename = "GoodAndSvcsOthrSubTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub good_and_svcs_othr_sub_tp: Option<Max35Text>,
    #[serde(
        rename = "GoodAndSvcDlvryChanl",
        skip_serializing_if = "Option::is_none"
    )]
    pub good_and_svc_dlvry_chanl: Option<GoodAndServiceDeliveryChannel1Code>,
    #[serde(
        rename = "OthrGoodAndSvcDlvryChanl",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_good_and_svc_dlvry_chanl: Option<Max35Text>,
    #[serde(
        rename = "GoodAndSvcDlvrySchdl",
        skip_serializing_if = "Option::is_none"
    )]
    pub good_and_svc_dlvry_schdl: Option<GoodAndServiceDeliverySchedule1Code>,
    #[serde(
        rename = "OthrGoodAndSvcDlvrySchdl",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_good_and_svc_dlvry_schdl: Option<Max35Text>,
    #[serde(rename = "SpltPmtInd", skip_serializing_if = "Option::is_none")]
    pub splt_pmt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "RctReqInd", skip_serializing_if = "Option::is_none")]
    pub rct_req_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RctTp", default)]
    pub rct_tp: Vec<ReceiptType1Code>,
    #[serde(rename = "RctDstn", skip_serializing_if = "Option::is_none")]
    pub rct_dstn: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardProgrammeMode2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Id", default)]
    pub id: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10NumberFraction2 {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm13Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "DKP9")]
    Dkp9,
    #[serde(rename = "UKPT")]
    Ukpt,
    #[serde(rename = "UKA1")]
    Uka1,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
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
pub struct RiskContext2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "RskInptData", default)]
    pub rsk_inpt_data: Vec<RiskInputData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RskAssmnt", default)]
    pub rsk_assmnt: Vec<RiskAssessment2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RiskAssessmentDataEntityProvider1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType28Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalFee2 {
    #[serde(rename = "Tp")]
    pub tp: TypeOfAmount21Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "FeePrgm", skip_serializing_if = "Option::is_none")]
    pub fee_prgm: Option<Max35Text>,
    #[serde(rename = "FeeDscrptr", skip_serializing_if = "Option::is_none")]
    pub fee_dscrptr: Option<Max35Text>,
    #[validate]
    #[serde(rename = "FeeAmt")]
    pub fee_amt: FeeAmount3,
    #[serde(rename = "FeeRcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub fee_rcncltn_amt: Option<FeeAmount3>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TerminalType1Code {
    #[serde(rename = "ATMT")]
    Atmt,
    #[serde(rename = "MPOS")]
    Mpos,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "POST")]
    Post,
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
pub struct Max100KBinary {
    #[validate(length(min = 1, max = 102400,), regex = "MAX_100_K_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Identification3Code {
    #[serde(rename = "DRID")]
    Drid,
    #[serde(rename = "NTID")]
    Ntid,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "SSYN")]
    Ssyn,
    #[serde(rename = "ARNB")]
    Arnb,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "PHNB")]
    Phnb,
    #[serde(rename = "CUID")]
    Cuid,
    #[serde(rename = "TXID")]
    Txid,
    #[serde(rename = "PRXY")]
    Prxy,
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
pub struct AlgorithmIdentification25 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm23Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PeriodUnit2Code {
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "HFDA")]
    Hfda,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MINU")]
    Minu,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalIdentificationType1Code {
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "ICSN")]
    Icsn,
    #[serde(rename = "ATNU")]
    Atnu,
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
pub struct HiredVehicle2 {
    #[serde(rename = "CpnyTp", skip_serializing_if = "Option::is_none")]
    pub cpny_tp: Option<Max35Text>,
    #[serde(rename = "CpnyNm", skip_serializing_if = "Option::is_none")]
    pub cpny_nm: Option<Max70Text>,
    #[serde(rename = "TpOfVhcl", skip_serializing_if = "Option::is_none")]
    pub tp_of_vhcl: Option<Max35Text>,
    #[serde(rename = "VhclId", skip_serializing_if = "Option::is_none")]
    pub vhcl_id: Option<Max35Text>,
    #[serde(rename = "DrvrId", skip_serializing_if = "Option::is_none")]
    pub drvr_id: Option<Max35Text>,
    #[serde(rename = "DrvrTaxId", skip_serializing_if = "Option::is_none")]
    pub drvr_tax_id: Option<Max35Text>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<Destination3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType28Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "WLPR")]
    Wlpr,
    #[serde(rename = "ISUR")]
    Isur,
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
pub struct Exact1HexBinaryText {
    #[validate(regex = "EXACT_1_HEX_BINARY_TEXT_REGEX")]
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
pub struct Adjustment9 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "AddtlTp", skip_serializing_if = "Option::is_none")]
    pub addtl_tp: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max70Text>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max35Text>,
    #[serde(rename = "PrmtnCd", skip_serializing_if = "Option::is_none")]
    pub prmtn_cd: Option<Max35Text>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "AdjstmntAmt", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TaxClctdOnOrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub tax_clctd_on_orgnl_amt: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BatchManagementInformation1 {
    #[serde(rename = "ColltnId", skip_serializing_if = "Option::is_none")]
    pub colltn_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "BtchId")]
    pub btch_id: Max35Text,
    #[serde(rename = "MsgSeqNb", skip_serializing_if = "Option::is_none")]
    pub msg_seq_nb: Option<Max15NumericText>,
    #[serde(rename = "MsgChcksmInptVal", skip_serializing_if = "Option::is_none")]
    pub msg_chcksm_inpt_val: Option<Max140Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PinData1 {
    #[serde(rename = "Ctrl", skip_serializing_if = "Option::is_none")]
    pub ctrl: Option<Exact1HexBinaryText>,
    #[serde(rename = "KeySetIdr", skip_serializing_if = "Option::is_none")]
    pub key_set_idr: Option<Max8NumericText>,
    #[serde(rename = "DrvdInf", skip_serializing_if = "Option::is_none")]
    pub drvd_inf: Option<Max32HexBinaryText>,
    #[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
    pub algo: Option<Max2NumericText>,
    #[serde(rename = "KeyLngth", skip_serializing_if = "Option::is_none")]
    pub key_lngth: Option<Max4NumericText>,
    #[serde(rename = "KeyPrtcn", skip_serializing_if = "Option::is_none")]
    pub key_prtcn: Option<Max2NumericText>,
    #[serde(rename = "KeyIndx", skip_serializing_if = "Option::is_none")]
    pub key_indx: Option<Max5NumericText>,
    #[validate]
    #[serde(rename = "PINBlckFrmt")]
    pub pin_blck_frmt: Max2NumericText,
    #[validate]
    #[serde(rename = "NcrptdPINBlck")]
    pub ncrptd_pin_blck: Max16HexBinaryText,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Track2Data1ChoiceEnum {
    #[serde(rename = "TxtVal", skip_serializing_if = "Option::is_none")]
    pub txt_val: Option<Max37Text>,
    #[serde(rename = "HexBinryVal", skip_serializing_if = "Option::is_none")]
    pub hex_binry_val: Option<Max19HexBinaryText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Track2Data1Choice {
    #[serde(flatten)]
    pub value: Track2Data1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Device2 {
    #[serde(rename = "Manfctr", skip_serializing_if = "Option::is_none")]
    pub manfctr: Option<Max70Text>,
    #[serde(rename = "ManfctrMdlId", skip_serializing_if = "Option::is_none")]
    pub manfctr_mdl_id: Option<Max70Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CustomerDeviceType2Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "LctnCtryCd", skip_serializing_if = "Option::is_none")]
    pub lctn_ctry_cd: Option<Iso3NumericCountryCode>,
    #[serde(rename = "IPAdr", skip_serializing_if = "Option::is_none")]
    pub ip_adr: Option<Max70Text>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "DvcNm", skip_serializing_if = "Option::is_none")]
    pub dvc_nm: Option<Max100Text>,
    #[serde(rename = "DvcNmNrmlzd", skip_serializing_if = "Option::is_none")]
    pub dvc_nm_nrmlzd: Option<Max100Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Terminal5 {
    #[serde(rename = "TermnlId", skip_serializing_if = "Option::is_none")]
    pub termnl_id: Option<TerminalIdentification3>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TerminalType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<Capabilities2>,
    #[serde(rename = "TermnlIntgtn", skip_serializing_if = "Option::is_none")]
    pub termnl_intgtn: Option<TerminalIntegrationCategory1Code>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "OutdrInd", skip_serializing_if = "Option::is_none")]
    pub outdr_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "OffPrmissInd", skip_serializing_if = "Option::is_none")]
    pub off_prmiss_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "OnBrdInd", skip_serializing_if = "Option::is_none")]
    pub on_brd_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "POICmpnt", default)]
    pub poi_cmpnt: Vec<PointOfInteractionComponent13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoYearMonth {
    #[validate(regex = "ISO_YEAR_MONTH_REGEX")]
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
pub struct ExternalEncryptedElementIdentification1Code {
    #[validate(regex = "EXTERNAL_ENCRYPTED_ELEMENT_IDENTIFICATION_1_CODE_REGEX")]
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
pub struct CardReadingCapabilities1 {
    #[serde(rename = "Cpblty")]
    pub cpblty: CardDataReading10Code,
    #[serde(rename = "OthrCpblty", skip_serializing_if = "Option::is_none")]
    pub othr_cpblty: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentPeriod1Code {
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ANNU")]
    Annu,
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
pub struct TripLeg2 {
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35NumericText>,
    #[serde(rename = "Tckt", skip_serializing_if = "Option::is_none")]
    pub tckt: Option<Ticket2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Doc", default)]
    pub doc: Vec<DocumentReference1>,
    #[serde(rename = "PrcdrRef", skip_serializing_if = "Option::is_none")]
    pub prcdr_ref: Option<Max35Text>,
    #[serde(rename = "TrnsprtTp", skip_serializing_if = "Option::is_none")]
    pub trnsprt_tp: Option<TransportType1Code>,
    #[serde(rename = "OthrTrnsprtTp", skip_serializing_if = "Option::is_none")]
    pub othr_trnsprt_tp: Option<Max35Text>,
    #[serde(rename = "CmmdtyCd", skip_serializing_if = "Option::is_none")]
    pub cmmdty_cd: Option<Max4Text>,
    #[serde(rename = "Crrier", skip_serializing_if = "Option::is_none")]
    pub crrier: Option<CarrierIdentification1>,
    #[serde(rename = "RouteNb", skip_serializing_if = "Option::is_none")]
    pub route_nb: Option<Max35Text>,
    #[serde(rename = "SvcClss", skip_serializing_if = "Option::is_none")]
    pub svc_clss: Option<Max35Text>,
    #[serde(rename = "Dprture", skip_serializing_if = "Option::is_none")]
    pub dprture: Option<DepartureOrArrival1>,
    #[serde(rename = "Arrvl", skip_serializing_if = "Option::is_none")]
    pub arrvl: Option<DepartureOrArrival1>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "StopOverInd", skip_serializing_if = "Option::is_none")]
    pub stop_over_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "NonDrctRouteCd", skip_serializing_if = "Option::is_none")]
    pub non_drct_route_cd: Option<Max35Text>,
    #[serde(rename = "FairBsisCd", skip_serializing_if = "Option::is_none")]
    pub fair_bsis_cd: Option<Max35Text>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TripLegAmt", default)]
    pub trip_leg_amt: Vec<AmountDetails2>,
    #[serde(rename = "CdtRsnCd", skip_serializing_if = "Option::is_none")]
    pub cdt_rsn_cd: Option<Max35Text>,
    #[serde(rename = "PrcdrId", skip_serializing_if = "Option::is_none")]
    pub prcdr_id: Option<Max35Text>,
    #[serde(rename = "LltyPrgrmm", skip_serializing_if = "Option::is_none")]
    pub llty_prgrmm: Option<LoyaltyProgramme2>,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageClass1Code {
    #[serde(rename = "ADDE")]
    Adde,
    #[serde(rename = "AMDT")]
    Amdt,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "CMGT")]
    Cmgt,
    #[serde(rename = "CBCK")]
    Cbck,
    #[serde(rename = "FEEC")]
    Feec,
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "INQY")]
    Inqy,
    #[serde(rename = "VERI")]
    Veri,
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
pub struct LocalData4 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max70Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
    pub nm_and_lctn: Option<Max200Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max512Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max512Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification262 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Text>,
    #[serde(rename = "FrgnMrchnt", skip_serializing_if = "Option::is_none")]
    pub frgn_mrchnt: Option<TrueFalseIndicator>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AuthorisedAmount1 {
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
    pub addtl_data: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransferService2 {
    #[serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none")]
    pub svc_prvdr: Option<Max35Text>,
    #[serde(rename = "SvcNm", skip_serializing_if = "Option::is_none")]
    pub svc_nm: Option<Max35Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "BizPurp", skip_serializing_if = "Option::is_none")]
    pub biz_purp: Option<Max500Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max256Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure10Code {
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "MILE")]
    Mile,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExchangeRateType2Code {
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "MIDL")]
    Midl,
    #[serde(rename = "BUYR")]
    Buyr,
    #[serde(rename = "AGRD")]
    Agrd,
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
pub struct Max16HexBinaryText {
    #[validate(regex = "MAX_16_HEX_BINARY_TEXT_REGEX")]
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
pub struct EncryptedData1ChoiceEnum {
    #[serde(rename = "BinryData", skip_serializing_if = "Option::is_none")]
    pub binry_data: Option<Max100KBinary>,
    #[serde(rename = "HexBinryVal", skip_serializing_if = "Option::is_none")]
    pub hex_binry_val: Option<Max9999HexBinaryText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedData1Choice {
    #[serde(flatten)]
    pub value: EncryptedData1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GracePeriodUnitType1Code {
    #[serde(rename = "WEKS")]
    Weks,
    #[serde(rename = "PMTS")]
    Pmts,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "DAYS")]
    Days,
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
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
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
pub struct CustomerDevice4 {
    #[serde(rename = "Dvc", skip_serializing_if = "Option::is_none")]
    pub dvc: Option<Device2>,
    #[serde(rename = "DvcId", skip_serializing_if = "Option::is_none")]
    pub dvc_id: Option<DeviceIdentification1>,
    #[serde(rename = "OprgSys", skip_serializing_if = "Option::is_none")]
    pub oprg_sys: Option<DeviceOperatingSystem1>,
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentReference1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FeeAmount3 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "FctvXchgRate", skip_serializing_if = "Option::is_none")]
    pub fctv_xchg_rate: Option<BaseOne25Rate>,
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "ConvsTm", skip_serializing_if = "Option::is_none")]
    pub convs_tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PassengerTransport2 {
    #[serde(rename = "Summry", skip_serializing_if = "Option::is_none")]
    pub summry: Option<PassengerTransportSummary2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TripLeg", default)]
    pub trip_leg: Vec<TripLeg2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AncllryPurchs", default)]
    pub ancllry_purchs: Vec<AncillaryPurchase2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "HirdVhclDtls", default)]
    pub hird_vhcl_dtls: Vec<HiredVehicle2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalData2 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max512Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Customer4 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CustomerType2Code>,
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRegnId", default)]
    pub tax_regn_id: Vec<Max70Text>,
    #[serde(rename = "AuthrsdCtctCpny", skip_serializing_if = "Option::is_none")]
    pub authrsd_ctct_cpny: Option<Max70Text>,
    #[serde(rename = "AuthrsdCtctNm", skip_serializing_if = "Option::is_none")]
    pub authrsd_ctct_nm: Option<Max70Text>,
    #[serde(rename = "AuthrsdCtctPhneNb", skip_serializing_if = "Option::is_none")]
    pub authrsd_ctct_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "VIPInd", skip_serializing_if = "Option::is_none")]
    pub vip_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CstmrRltsh", skip_serializing_if = "Option::is_none")]
    pub cstmr_rltsh: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LodgingActivity1Code {
    #[serde(rename = "APTM")]
    Aptm,
    #[serde(rename = "BEBR")]
    Bebr,
    #[serde(rename = "COTT")]
    Cott,
    #[serde(rename = "CRUI")]
    Crui,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "HOST")]
    Host,
    #[serde(rename = "HOTL")]
    Hotl,
    #[serde(rename = "LODG")]
    Lodg,
    #[serde(rename = "MOTL")]
    Motl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "RESO")]
    Reso,
    #[serde(rename = "ROAB")]
    Roab,
    #[serde(rename = "TOSH")]
    Tosh,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PurchaseIdentifierType1Code {
    #[serde(rename = "CONU")]
    Conu,
    #[serde(rename = "CUOR")]
    Cuor,
    #[serde(rename = "CUPO")]
    Cupo,
    #[serde(rename = "FONU")]
    Fonu,
    #[serde(rename = "INNU")]
    Innu,
    #[serde(rename = "ORNU")]
    Ornu,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRNU")]
    Prnu,
    #[serde(rename = "PUID")]
    Puid,
    #[serde(rename = "RELO")]
    Relo,
    #[serde(rename = "REAG")]
    Reag,
    #[serde(rename = "RENU")]
    Renu,
    #[serde(rename = "RSNU")]
    Rsnu,
    #[serde(rename = "SUOR")]
    Suor,
    #[serde(rename = "TINU")]
    Tinu,
    #[serde(rename = "TRNU")]
    Trnu,
    #[serde(rename = "SUIN")]
    Suin,
    #[serde(rename = "TNID")]
    Tnid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm18Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "DKP9")]
    Dkp9,
    #[serde(rename = "UKPT")]
    Ukpt,
    #[serde(rename = "UKA1")]
    Uka1,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
    #[serde(rename = "DA12")]
    Da12,
    #[serde(rename = "DA19")]
    Da19,
    #[serde(rename = "DA25")]
    Da25,
    #[serde(rename = "N108")]
    N108,
    #[serde(rename = "EA5R")]
    Ea5R,
    #[serde(rename = "EA9R")]
    Ea9R,
    #[serde(rename = "EA2R")]
    Ea2R,
    #[serde(rename = "E3DR")]
    E3Dr,
    #[serde(rename = "E36C")]
    E36C,
    #[serde(rename = "E36R")]
    E36R,
    #[serde(rename = "SD5C")]
    Sd5C,
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
pub struct AlgorithmIdentification27 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm7Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter13>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VerificationEntity2Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "ISSR")]
    Issr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "CDAD")]
    Cdad,
    #[serde(rename = "ICCA")]
    Icca,
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
pub struct ExternalAuthenticationMethod1Code {
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
pub struct AccountDetails3 {
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max70Text>,
    #[serde(rename = "AcctTp", skip_serializing_if = "Option::is_none")]
    pub acct_tp: Option<Iso8583AccountTypeCode>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max11NumericText {
    #[validate(regex = "MAX_11_NUMERIC_TEXT_REGEX")]
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
pub struct Iso8583ResponseCode {
    #[validate(regex = "ISO_8583_RESPONSE_CODE_REGEX")]
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
pub struct Amount20 {
    #[serde(rename = "TpOfChrg", skip_serializing_if = "Option::is_none")]
    pub tp_of_chrg: Option<TypeOfAmount19Code>,
    #[serde(rename = "OthrTpOfChrg", skip_serializing_if = "Option::is_none")]
    pub othr_tp_of_chrg: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct JourneyInformation1 {
    #[serde(rename = "JrnyTp", skip_serializing_if = "Option::is_none")]
    pub jrny_tp: Option<JourneyType1Code>,
    #[serde(rename = "JrnyData", skip_serializing_if = "Option::is_none")]
    pub jrny_data: Option<Max35Text>,
    #[serde(rename = "DtAndTm", skip_serializing_if = "Option::is_none")]
    pub dt_and_tm: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType22Code {
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct AlgorithmIdentification18 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VehicleRentalInvoice2 {
    #[serde(rename = "NoShowInd", skip_serializing_if = "Option::is_none")]
    pub no_show_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "AdjstdInd", skip_serializing_if = "Option::is_none")]
    pub adjstd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "RtrLctn", skip_serializing_if = "Option::is_none")]
    pub rtr_lctn: Option<Address2>,
    #[serde(rename = "ChckOutDt", skip_serializing_if = "Option::is_none")]
    pub chck_out_dt: Option<IsoDate>,
    #[serde(rename = "ChckOutTm", skip_serializing_if = "Option::is_none")]
    pub chck_out_tm: Option<IsoTime>,
    #[serde(rename = "ChckInDt", skip_serializing_if = "Option::is_none")]
    pub chck_in_dt: Option<IsoDate>,
    #[serde(rename = "ChckInTm", skip_serializing_if = "Option::is_none")]
    pub chck_in_tm: Option<IsoTime>,
    #[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
    pub drtn: Option<Max4NumericText>,
    #[serde(rename = "VhclClssInvcd", skip_serializing_if = "Option::is_none")]
    pub vhcl_clss_invcd: Option<Vehicle4>,
    #[serde(rename = "VhclClssPrvdd", skip_serializing_if = "Option::is_none")]
    pub vhcl_clss_prvdd: Option<Vehicle4>,
    #[serde(rename = "TrvlDstnc", skip_serializing_if = "Option::is_none")]
    pub trvl_dstnc: Option<Distance1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RntlChrg", default)]
    pub rntl_chrg: Vec<RentalRate1>,
    #[serde(rename = "SummryCmmdtyId", skip_serializing_if = "Option::is_none")]
    pub summry_cmmdty_id: Option<Max35Text>,
    #[serde(rename = "InsrncInd", skip_serializing_if = "Option::is_none")]
    pub insrnc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<Amount18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax39>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso8583MessageReasonCode {
    #[validate(regex = "ISO_8583_MESSAGE_REASON_CODE_REGEX")]
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
pub struct Authority1 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "CtrySubDvsnMjr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMnr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max50Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max50Text>,
}
