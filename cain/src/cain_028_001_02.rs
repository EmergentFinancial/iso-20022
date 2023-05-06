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
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_12_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_19_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,19}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_11_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_AMOUNT_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_23_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,23}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTERNAL_ENCRYPTED_ELEMENT_IDENTIFICATION_1_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_MESSAGE_REASON_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_RESPONSE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_18245_MERCHANT_CATEGORY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_32_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_9999_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,9999}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:cain.028.001.02".to_string()
}

#[derive(
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
pub struct Terminal6 {
    #[validate]
    #[serde(rename = "TermnlId")]
    pub termnl_id: TerminalIdentification3,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Transaction154 {
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgRsn", default)]
    pub msg_rsn: Vec<Iso8583MessageReasonCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnMsgRsn", default)]
    pub altrn_msg_rsn: Vec<Max256Text>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentification17,
    #[validate]
    #[serde(rename = "TxAmts")]
    pub tx_amts: TransactionAmounts2,
    #[validate]
    #[serde(rename = "DsptData")]
    pub dspt_data: DisputeData3,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlFee", default)]
    pub addtl_fee: Vec<AdditionalFee2>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation20>,
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
pub struct Max4000Text {
    #[validate(length(min = 1, max = 4000,))]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionInitiator1Code {
    #[serde(rename = "MERC")]
    Merc,
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
pub struct Max500Binary {
    #[validate(length(min = 1, max = 500,), regex = "MAX_500_BINARY_REGEX")]
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
#[derive(
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
pub struct Traceability9 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification183,
    #[serde(rename = "TracDtTmIn", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_in: Option<IsoDateTime>,
    #[serde(rename = "TracDtTmOut", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_out: Option<IsoDateTime>,
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
pub struct CardData10 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Max19NumericText>,
    #[serde(rename = "PrtctdPANInd", skip_serializing_if = "Option::is_none")]
    pub prtctd_pan_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "PmtAcctRef", skip_serializing_if = "Option::is_none")]
    pub pmt_acct_ref: Option<Max35Text>,
    #[serde(rename = "PANAcctRg", skip_serializing_if = "Option::is_none")]
    pub pan_acct_rg: Option<Max19NumericText>,
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
pub struct Max19NumericText {
    #[validate(regex = "MAX_19_NUMERIC_TEXT_REGEX")]
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
pub struct ChargeBackResponseV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header61,
    #[validate]
    #[serde(rename = "Body")]
    pub body: ChargeBackResponse2<A>,
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
pub struct Max11NumericText {
    #[validate(regex = "MAX_11_NUMERIC_TEXT_REGEX")]
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
pub struct Recipient5ChoiceEnum {
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber1>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
}
#[derive(
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
pub struct ChargeBackResponse2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: Environment29,
    #[validate]
    #[serde(rename = "Cntxt")]
    pub cntxt: Context16,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: Transaction154,
    #[validate]
    #[serde(rename = "PrcgRslt")]
    pub prcg_rslt: ProcessingResult15,
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
pub struct Context16 {
    #[serde(rename = "TxCntxt", skip_serializing_if = "Option::is_none")]
    pub tx_cntxt: Option<TransactionContext8>,
}
#[derive(
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
    #[serde(rename = "HexBinryVal", skip_serializing_if = "Option::is_none")]
    pub hex_binry_val: Option<Max9999HexBinaryText>,
    #[serde(rename = "BinryData", skip_serializing_if = "Option::is_none")]
    pub binry_data: Option<Max100KBinary>,
}
#[derive(
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
#[derive(
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
#[derive(
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
pub struct Header61 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: MessageFunction20Code,
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
#[derive(
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
pub struct EnvelopedData6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient7Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent5>,
}
#[derive(
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
#[derive(
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
pub struct Reconciliation3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "ChckptRef", skip_serializing_if = "Option::is_none")]
    pub chckpt_ref: Option<Max35Text>,
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
pub struct Exact15Text {
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
pub struct Exact2NumericText {
    #[validate(regex = "EXACT_2_NUMERIC_TEXT_REGEX")]
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
pub struct SettlementReportingEntity1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
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
#[derive(
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
pub struct Max100KBinary {
    #[validate(length(min = 1, max = 102400,), regex = "MAX_100_K_BINARY_REGEX")]
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
pub struct Max20KText {
    #[validate(length(min = 1, max = 20000,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Max35Binary {
    #[validate(length(min = 1, max = 35,), regex = "MAX_35_BINARY_REGEX")]
    pub value: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FeeCollectionInitiator1Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
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
pub struct AdditionalData1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max2048Text>,
}
#[derive(
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
pub struct Iso18245MerchantCategoryCode {
    #[validate(regex = "ISO_18245_MERCHANT_CATEGORY_CODE_REGEX")]
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
pub struct TransactionIdentification17 {
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
#[derive(
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
    #[serde(rename = "ChrgBckRspn")]
    pub chrg_bck_rspn: ChargeBackResponseV02<A>,
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
pub struct TransactionAmount1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy")]
    pub ccy: Iso3NumericCurrencyCode,
}
#[derive(
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
pub struct Min2Max3NumericText {
    #[validate(regex = "MIN_2_MAX_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Max8NumericText {
    #[validate(regex = "MAX_8_NUMERIC_TEXT_REGEX")]
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
pub struct TransactionContext8 {
    #[serde(rename = "MrchntCtgyCd", skip_serializing_if = "Option::is_none")]
    pub mrchnt_ctgy_cd: Option<Iso18245MerchantCategoryCode>,
    #[serde(
        rename = "MrchntCtgySpcfcData",
        skip_serializing_if = "Option::is_none"
    )]
    pub mrchnt_ctgy_spcfc_data: Option<Max35Text>,
    #[serde(rename = "FeeColltnInitr", skip_serializing_if = "Option::is_none")]
    pub fee_colltn_initr: Option<FeeCollectionInitiator1Code>,
    #[serde(rename = "TxInitr", skip_serializing_if = "Option::is_none")]
    pub tx_initr: Option<TransactionInitiator1Code>,
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
    #[serde(rename = "CardPrgrmmApld", skip_serializing_if = "Option::is_none")]
    pub card_prgrmm_apld: Option<CardProgrammeMode3>,
    #[serde(rename = "Jursdctn", skip_serializing_if = "Option::is_none")]
    pub jursdctn: Option<Jurisdiction2>,
    #[serde(rename = "SttlmSvc", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc: Option<SettlementService4>,
    #[serde(rename = "Rcncltn", skip_serializing_if = "Option::is_none")]
    pub rcncltn: Option<Reconciliation3>,
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(
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
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingResult15 {
    #[serde(rename = "RspnSrc", skip_serializing_if = "Option::is_none")]
    pub rspn_src: Option<ApprovalEntity2>,
    #[validate]
    #[serde(rename = "RsltData")]
    pub rslt_data: ResultData7,
    #[validate(length(min = 0,))]
    #[serde(rename = "ErrDtl", default)]
    pub err_dtl: Vec<ErrorDetails2>,
    #[serde(rename = "OrgnlRsltData", skip_serializing_if = "Option::is_none")]
    pub orgnl_rslt_data: Option<ResultData7>,
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
pub struct Exact12Text {
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
#[derive(
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
pub struct AlgorithmIdentification25 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm23Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter7>,
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
pub enum Algorithm8Code {
    #[serde(rename = "MGF1")]
    Mgf1,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Max5000Binary {
    #[validate(length(min = 1, max = 5000,), regex = "MAX_5000_BINARY_REGEX")]
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
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
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
#[derive(
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
pub enum MessageFunction20Code {
    #[serde(rename = "ADVC")]
    Advc,
    #[serde(rename = "NOTI")]
    Noti,
    #[serde(rename = "REQU")]
    Requ,
    #[serde(rename = "RVRA")]
    Rvra,
    #[serde(rename = "RVNO")]
    Rvno,
    #[serde(rename = "STAD")]
    Stad,
    #[serde(rename = "STNO")]
    Stno,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ContentType3Code {
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "IFSE")]
    Ifse,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementService4 {
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
#[derive(
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
pub struct Recipient7ChoiceEnum {
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek6>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier6>,
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport6>,
}
#[derive(
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
pub struct Max2NumericText {
    #[validate(regex = "MAX_2_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Environment29 {
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
    pub accptr: Option<PartyIdentification258>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyIdentification263>,
    #[serde(rename = "Termnl", skip_serializing_if = "Option::is_none")]
    pub termnl: Option<Terminal6>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification263>,
    #[validate]
    #[serde(rename = "Card")]
    pub card: CardData10,
    #[serde(rename = "Tkn", skip_serializing_if = "Option::is_none")]
    pub tkn: Option<Token2>,
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
pub struct IsoMax3ALanguageCode {
    #[validate(regex = "ISO_MAX_3_A_LANGUAGE_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
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
