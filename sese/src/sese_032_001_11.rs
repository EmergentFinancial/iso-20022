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
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_6_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.032.001.11".to_string()
}

#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType42ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType42Choice {
    #[serde(flatten)]
    pub value: IdentificationType42ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberCount2ChoiceEnum {
    #[serde(rename = "TtlNb", skip_serializing_if = "Option::is_none")]
    pub ttl_nb: Option<TotalNumber2>,
    #[serde(rename = "CurInstrNb", skip_serializing_if = "Option::is_none")]
    pub cur_instr_nb: Option<Max6NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberCount2Choice {
    #[serde(flatten)]
    pub value: NumberCount2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PairedOrTurnedQuantity5ChoiceEnum {
    #[serde(rename = "PairdOffQty", skip_serializing_if = "Option::is_none")]
    pub paird_off_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "TrndQty", skip_serializing_if = "Option::is_none")]
    pub trnd_qty: Option<FinancialInstrumentQuantity33Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PairedOrTurnedQuantity5Choice {
    #[serde(flatten)]
    pub value: PairedOrTurnedQuantity5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice29ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice14Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice29Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice29ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms23 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[validate]
    #[serde(rename = "RsltgAmt")]
    pub rsltg_amt: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation1 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Registration1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration9Choice {
    #[serde(flatten)]
    pub value: Registration9ChoiceEnum,
}
#[derive(
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
pub struct SecuritiesRtgs4ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesRtgs4Choice {
    #[serde(flatten)]
    pub value: SecuritiesRtgs4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FxStandingInstruction4ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FxStandingInstruction4Choice {
    #[serde(flatten)]
    pub value: FxStandingInstruction4ChoiceEnum,
}
#[derive(
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
pub struct UnmatchedStatus16ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<UnmatchedReason15>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedStatus16Choice {
    #[serde(flatten)]
    pub value: UnmatchedStatus16ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketClientSide1Code {
    #[serde(rename = "CLNT")]
    Clnt,
    #[serde(rename = "MAKT")]
    Makt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryReceiptType2Code {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "APMT")]
    Apmt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxLiability1Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "AGEN")]
    Agen,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClassificationType32ChoiceEnum {
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification36>,
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClassificationType32Choice {
    #[serde(flatten)]
    pub value: ClassificationType32ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OpeningClosing1Code {
    #[serde(rename = "CLOP")]
    Clop,
    #[serde(rename = "OPEP")]
    Opep,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification120ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification120Choice {
    #[serde(flatten)]
    pub value: PartyIdentification120ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification127ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
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
pub struct PartyIdentification127Choice {
    #[serde(flatten)]
    pub value: PartyIdentification127ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus52ChoiceEnum {
    #[serde(rename = "PdgCxl", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl: Option<PendingStatus38Choice>,
    #[serde(rename = "Rpr", skip_serializing_if = "Option::is_none")]
    pub rpr: Option<RepairStatus12Choice>,
    #[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg: Option<PendingProcessingStatus11Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason6>,
    #[serde(rename = "CxlReqd", skip_serializing_if = "Option::is_none")]
    pub cxl_reqd: Option<ProprietaryReason4>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus21Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus52Choice {
    #[serde(flatten)]
    pub value: ProcessingStatus52ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LetterOfGuarantee4ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LetterOfGuarantee4Choice {
    #[serde(flatten)]
    pub value: LetterOfGuarantee4ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "SctiesSttlmTxGnrtnNtfctn")]
    pub scties_sttlm_tx_gnrtn_ntfctn: SecuritiesSettlementTransactionGenerationNotificationV11<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
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
pub struct SettlementStatus17ChoiceEnum {
    #[serde(rename = "Flng", skip_serializing_if = "Option::is_none")]
    pub flng: Option<FailingStatus10Choice>,
    #[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
    pub pdg: Option<PendingStatus37Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStatus17Choice {
    #[serde(flatten)]
    pub value: SettlementStatus17ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus21ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason9>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus21Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus21ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReceiveDelivery1Code {
    #[serde(rename = "DELI")]
    Deli,
    #[serde(rename = "RECE")]
    Rece,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity33ChoiceEnum {
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity33Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity33ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tracking4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tracking4Choice {
    #[serde(flatten)]
    pub value: Tracking4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate8ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<TradeDateCode3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate8Choice {
    #[serde(flatten)]
    pub value: TradeDate8ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepairReason4Code {
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICUS")]
    Icus,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatusAndReason28 {
    #[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
    pub prcg_sts: Option<ProcessingStatus52Choice>,
    #[serde(rename = "IfrrdMtchgSts", skip_serializing_if = "Option::is_none")]
    pub ifrrd_mtchg_sts: Option<MatchingStatus24Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus24Choice>,
    #[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
    pub sttlm_sts: Option<SettlementStatus17Choice>,
}
#[derive(
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
pub struct PlaceOfTradeIdentification1 {
    #[serde(rename = "MktTpAndId", skip_serializing_if = "Option::is_none")]
    pub mkt_tp_and_id: Option<MarketIdentification84>,
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
pub struct PartyIdentificationAndAccount196 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition5Code {
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NPAR")]
    Npar,
    #[serde(rename = "PARC")]
    Parc,
    #[serde(rename = "PARQ")]
    Parq,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus11ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingProcessingReason8>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus11Choice {
    #[serde(flatten)]
    pub value: PendingProcessingStatus11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection94 {
    #[serde(rename = "AcrdIntrstInd", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_ind: Option<YesNoIndicator>,
    #[serde(rename = "StmpDtyInd", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_ind: Option<YesNoIndicator>,
    #[serde(rename = "BrkrgAmtInd", skip_serializing_if = "Option::is_none")]
    pub brkrg_amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "RsrchFeeInd", skip_serializing_if = "Option::is_none")]
    pub rsrch_fee_ind: Option<YesNoIndicator>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms23>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateAndDateTime2Choice>,
}
#[derive(
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
pub struct UnmatchedReason15 {
    #[serde(rename = "Cd")]
    pub cd: UnmatchedReason21Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
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
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryReturn1Code {
    #[serde(rename = "UNRE")]
    Unre,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DUEB")]
    Dueb,
    #[serde(rename = "PARD")]
    Pard,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalAndCurrentQuantities1 {
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace3Code {
    #[serde(rename = "SHHE")]
    Shhe,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxCapacityParty4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxLiability1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxCapacityParty4Choice {
    #[serde(flatten)]
    pub value: TaxCapacityParty4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Registration1Code {
    #[serde(rename = "NREG")]
    Nreg,
    #[serde(rename = "YREG")]
    Yreg,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat4Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethodFormat4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AutoBorrowing1Code {
    #[serde(rename = "LAMI")]
    Lami,
    #[serde(rename = "NBOR")]
    Nbor,
    #[serde(rename = "YBOR")]
    Ybor,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDate4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode7Choice {
    #[serde(flatten)]
    pub value: SettlementDateCode7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
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
pub struct IdentificationSource3Choice {
    #[serde(flatten)]
    pub value: IdentificationSource3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition20ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition20Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingPosition3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition7Choice {
    #[serde(flatten)]
    pub value: ProcessingPosition7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Registration2Code {
    #[serde(rename = "PTYH")]
    Ptyh,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CDEL")]
    Cdel,
    #[serde(rename = "CVAL")]
    Cval,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockChainAddressWallet3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
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
pub struct RepairReason10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepairReason4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason10Choice {
    #[serde(flatten)]
    pub value: RepairReason10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus10ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<FailingReason8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus10Choice {
    #[serde(flatten)]
    pub value: FailingStatus10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification84 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification1Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType8Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType12Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "NEGA")]
    Nega,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProprietaryStatusAndReason6 {
    #[validate]
    #[serde(rename = "PrtrySts")]
    pub prtry_sts: GenericIdentification30,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtryRsn", default)]
    pub prtry_rsn: Vec<ProprietaryReason4>,
}
#[derive(
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
pub enum PriceValueType1Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AcknowledgementReason5Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "RQWV")]
    Rqwv,
    #[serde(rename = "LATE")]
    Late,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max34Text>,
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
pub struct CashAccountIdentification5Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OriginatorRole2Code {
    #[serde(rename = "SINT")]
    Sint,
    #[serde(rename = "MLTF")]
    Mltf,
    #[serde(rename = "RMKT")]
    Rmkt,
    #[serde(rename = "MKTM")]
    Mktm,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "TAGT")]
    Tagt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification133ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification133Choice {
    #[serde(flatten)]
    pub value: PartyIdentification133ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus38ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus38Choice {
    #[serde(flatten)]
    pub value: PendingStatus38ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityBreakdown62 {
    #[serde(rename = "LotNb", skip_serializing_if = "Option::is_none")]
    pub lot_nb: Option<GenericIdentification37>,
    #[serde(rename = "LotQty", skip_serializing_if = "Option::is_none")]
    pub lot_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "LotDtTm", skip_serializing_if = "Option::is_none")]
    pub lot_dt_tm: Option<DateAndDateTime2Choice>,
    #[serde(rename = "LotPric", skip_serializing_if = "Option::is_none")]
    pub lot_pric: Option<Price7>,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice29Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketClientSide6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketClientSide1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketClientSide6Choice {
    #[serde(flatten)]
    pub value: MarketClientSide6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneratedReason5 {
    #[serde(rename = "Cd")]
    pub cd: GeneratedReasons5Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation2 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityAndAccount95 {
    #[serde(rename = "SttlmQty")]
    pub sttlm_qty: Quantity51Choice,
    #[serde(rename = "DnmtnChc", skip_serializing_if = "Option::is_none")]
    pub dnmtn_chc: Option<Max210Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification144>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafeKeepingPlace3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "QtyBrkdwn", default)]
    pub qty_brkdwn: Vec<QuantityBreakdown62>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency23ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency23Choice {
    #[serde(flatten)]
    pub value: Frequency23ChoiceEnum,
}
#[derive(
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
pub struct RepurchaseType23ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepurchaseType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepurchaseType23Choice {
    #[serde(flatten)]
    pub value: RepurchaseType23ChoiceEnum,
}
#[derive(
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
pub struct SafekeepingPlaceTypeAndText8 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashSettlementSystem4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashSettlementSystem2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashSettlementSystem4Choice {
    #[serde(flatten)]
    pub value: CashSettlementSystem4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed4ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed4Choice {
    #[serde(flatten)]
    pub value: ModificationCancellationAllowed4ChoiceEnum,
}
#[derive(
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
pub struct AcknowledgementReason9 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason12Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate17ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<SettlementDateCode7Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate17Choice {
    #[serde(flatten)]
    pub value: SettlementDate17ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmount3ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
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
pub struct PriceRateOrAmount3Choice {
    #[serde(flatten)]
    pub value: PriceRateOrAmount3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafeKeepingPlace3 {
    #[serde(rename = "SfkpgPlcFrmt", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc_frmt: Option<SafekeepingPlaceFormat29Choice>,
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
pub struct OtherParties43 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Invstr", default)]
    pub invstr: Vec<PartyIdentificationAndAccount197>,
    #[serde(rename = "QlfdFrgnIntrmy", skip_serializing_if = "Option::is_none")]
    pub qlfd_frgn_intrmy: Option<PartyIdentificationAndAccount198>,
    #[serde(rename = "StockXchg", skip_serializing_if = "Option::is_none")]
    pub stock_xchg: Option<PartyIdentificationAndAccount165>,
    #[serde(rename = "TradRgltr", skip_serializing_if = "Option::is_none")]
    pub trad_rgltr: Option<PartyIdentificationAndAccount165>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<PartyIdentificationAndAccount198>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<PartyIdentificationAndAccount198>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification146 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification122Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification144 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification127Choice,
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
pub struct References73ChoiceEnum {
    #[serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none")]
    pub intra_pos_mvmnt_id: Option<Max35Text>,
    #[serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none")]
    pub intra_bal_mvmnt_id: Option<Max35Text>,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<Max35Text>,
    #[serde(
        rename = "CtrPtyMktInfrstrctrTxId",
        skip_serializing_if = "Option::is_none"
    )]
    pub ctr_pty_mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
    pub othr_tx_id: Option<Max35Text>,
    #[serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_tx_id: Option<Max35Text>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References73Choice {
    #[serde(flatten)]
    pub value: References73ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeliveryReturn3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeliveryReturn1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeliveryReturn3Choice {
    #[serde(flatten)]
    pub value: DeliveryReturn3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegistrationReason5 {
    #[serde(rename = "Cd")]
    pub cd: Registration10Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TotalNumber2 {
    #[validate]
    #[serde(rename = "CurInstrNb")]
    pub cur_instr_nb: Max6NumericText,
    #[validate]
    #[serde(rename = "TtlOfLkdInstrs")]
    pub ttl_of_lkd_instrs: Max6NumericText,
}
#[derive(
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
pub struct PendingReason15 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason27Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessingPosition3Code {
    #[serde(rename = "AFTE")]
    Afte,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "BEFO")]
    Befo,
    #[serde(rename = "INFO")]
    Info,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct CurrencyToBuyOrSell1ChoiceEnum {
    #[serde(rename = "CcyToSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_sell: Option<ActiveCurrencyCode>,
    #[serde(rename = "CcyToBuy", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyToBuyOrSell1Choice {
    #[serde(flatten)]
    pub value: CurrencyToBuyOrSell1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionStyle2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle8Choice {
    #[serde(flatten)]
    pub value: OptionStyle8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOfClearingIdentification2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AnyBicDec2014Identifier>,
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
pub struct ProprietaryReason4 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<GenericIdentification30>,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingProcessingReason2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason10Choice {
    #[serde(flatten)]
    pub value: PendingProcessingReason10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicfiDec2014Identifier {
    #[validate(regex = "BICFI_DEC_2014_IDENTIFIER_REGEX")]
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
pub struct SecuritiesSettlementTransactionGenerationNotificationV11<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxIdDtls")]
    pub tx_id_dtls: SettlementTypeAndIdentification27,
    #[serde(rename = "NbCounts", skip_serializing_if = "Option::is_none")]
    pub nb_counts: Option<NumberCount2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Lnkgs", default)]
    pub lnkgs: Vec<Linkages61>,
    #[validate]
    #[serde(rename = "TradDtls")]
    pub trad_dtls: SecuritiesTradeDetails117,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_attrbts: Option<FinancialInstrumentAttributes111>,
    #[validate(length(min = 1,))]
    #[serde(rename = "QtyAndAcctDtls", default)]
    pub qty_and_acct_dtls: Vec<QuantityAndAccount95>,
    #[validate]
    #[serde(rename = "SttlmParams")]
    pub sttlm_params: SettlementDetails204,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties100>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties100>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties36>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection94>,
    #[serde(rename = "OthrAmts", skip_serializing_if = "Option::is_none")]
    pub othr_amts: Option<OtherAmounts39>,
    #[serde(rename = "OthrBizPties", skip_serializing_if = "Option::is_none")]
    pub othr_biz_pties: Option<OtherParties43>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GnrtdRsn", default)]
    pub gnrtd_rsn: Vec<GeneratedReason5>,
    #[serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none")]
    pub sts_and_rsn: Option<StatusAndReason28>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification78 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification30,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency3Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "WEEK")]
    Week,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnmatchedReason11Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DMCT")]
    Dmct,
    #[serde(rename = "DCMX")]
    Dcmx,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "MIME")]
    Mime,
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "DTRA")]
    Dtra,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "FRAP")]
    Frap,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLIS")]
    Plis,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "REGD")]
    Regd,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "UNBR")]
    Unbr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso20022MessageIdentificationText {
    #[validate(regex = "ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX")]
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
pub struct PartyIdentification122ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
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
pub struct PartyIdentification122Choice {
    #[serde(flatten)]
    pub value: PartyIdentification122ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FailingReason2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason8Choice {
    #[serde(flatten)]
    pub value: FailingReason8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutomaticBorrowing6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AutoBorrowing1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutomaticBorrowing6Choice {
    #[serde(flatten)]
    pub value: AutomaticBorrowing6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason27ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason27Choice {
    #[serde(flatten)]
    pub value: PendingReason27ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriorityNumeric4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Nmrc", skip_serializing_if = "Option::is_none")]
    pub nmrc: Option<Exact4NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriorityNumeric4Choice {
    #[serde(flatten)]
    pub value: PriorityNumeric4ChoiceEnum,
}
#[derive(
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesTransactionType26Code {
    #[serde(rename = "BSBK")]
    Bsbk,
    #[serde(rename = "COLI")]
    Coli,
    #[serde(rename = "COLO")]
    Colo,
    #[serde(rename = "MKDW")]
    Mkdw,
    #[serde(rename = "MKUP")]
    Mkup,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "NSYN")]
    Nsyn,
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PORT")]
    Port,
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REPU")]
    Repu,
    #[serde(rename = "RODE")]
    Rode,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SECB")]
    Secb,
    #[serde(rename = "SECL")]
    Secl,
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "SYND")]
    Synd,
    #[serde(rename = "TBAC")]
    Tbac,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "TRPO")]
    Trpo,
    #[serde(rename = "TRVO")]
    Trvo,
    #[serde(rename = "TURN")]
    Turn,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CNCB")]
    Cncb,
    #[serde(rename = "OWNE")]
    Owne,
    #[serde(rename = "FCTA")]
    Fcta,
    #[serde(rename = "OWNI")]
    Owni,
    #[serde(rename = "RELE")]
    Rele,
    #[serde(rename = "SBRE")]
    Sbre,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CLAI")]
    Clai,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "SWIF")]
    Swif,
    #[serde(rename = "SWIT")]
    Swit,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "ETFT")]
    Etft,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "SLRE")]
    Slre,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "SBBK")]
    Sbbk,
    #[serde(rename = "REDI")]
    Redi,
    #[serde(rename = "REBL")]
    Rebl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesPaymentStatus5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesPaymentStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesPaymentStatus5Choice {
    #[serde(flatten)]
    pub value: SecuritiesPaymentStatus5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType8Choice {
    #[serde(flatten)]
    pub value: MarketType8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason8 {
    #[serde(rename = "Cd")]
    pub cd: FailingReason8Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason28ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason28Choice {
    #[serde(flatten)]
    pub value: PendingReason28ChoiceEnum,
}
#[derive(
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
pub struct Restriction5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OwnershipLegalRestrictions1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Restriction5Choice {
    #[serde(flatten)]
    pub value: Restriction5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesCertificate4 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementSystemMethod1Code {
    #[serde(rename = "NSET")]
    Nset,
    #[serde(rename = "YSET")]
    Yset,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties100 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification146>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount196>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount196>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount196>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount196>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount196>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Registration2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration10Choice {
    #[serde(flatten)]
    pub value: Registration10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact4NumericText {
    #[validate(regex = "EXACT_4_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashSettlementSystem2Code {
    #[serde(rename = "GROS")]
    Gros,
    #[serde(rename = "NETS")]
    Nets,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CentralCounterPartyEligibility4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CentralCounterPartyEligibility4Choice {
    #[serde(flatten)]
    pub value: CentralCounterPartyEligibility4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceType4ChoiceEnum {
    #[serde(rename = "Mkt", skip_serializing_if = "Option::is_none")]
    pub mkt: Option<Price7>,
    #[serde(rename = "Indctv", skip_serializing_if = "Option::is_none")]
    pub indctv: Option<Price7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceType4Choice {
    #[serde(flatten)]
    pub value: PriceType4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount19 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
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
pub struct PendingProcessingReason8 {
    #[serde(rename = "Cd")]
    pub cd: PendingProcessingReason10Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice14Code {
    #[serde(rename = "AVER")]
    Aver,
    #[default]
    Unknown,
}
#[derive(
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
pub struct OtherIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification1Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "FIIN")]
    Fiin,
    #[serde(rename = "TXID")]
    Txid,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureType22ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExposureType12Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureType22Choice {
    #[serde(flatten)]
    pub value: ExposureType22ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OwnershipLegalRestrictions1Code {
    #[serde(rename = "A144")]
    A144,
    #[serde(rename = "NRST")]
    Nrst,
    #[serde(rename = "RSTR")]
    Rstr,
    #[default]
    Unknown,
}
#[derive(
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
pub struct SettlementDetails204 {
    #[serde(rename = "HldInd", skip_serializing_if = "Option::is_none")]
    pub hld_ind: Option<HoldIndicator6>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<PriorityNumeric4Choice>,
    #[serde(rename = "SctiesTxTp")]
    pub scties_tx_tp: SecuritiesTransactionType44Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition20Choice>,
    #[serde(rename = "PrtlSttlmInd", skip_serializing_if = "Option::is_none")]
    pub prtl_sttlm_ind: Option<SettlementTransactionCondition5Code>,
    #[serde(rename = "BnfclOwnrsh", skip_serializing_if = "Option::is_none")]
    pub bnfcl_ownrsh: Option<BeneficialOwnership4Choice>,
    #[serde(rename = "BlckTrad", skip_serializing_if = "Option::is_none")]
    pub blck_trad: Option<BlockTrade4Choice>,
    #[serde(rename = "CCPElgblty", skip_serializing_if = "Option::is_none")]
    pub ccp_elgblty: Option<CentralCounterPartyEligibility4Choice>,
    #[serde(rename = "DlvryRtrRsn", skip_serializing_if = "Option::is_none")]
    pub dlvry_rtr_rsn: Option<DeliveryReturn3Choice>,
    #[serde(rename = "CshClrSys", skip_serializing_if = "Option::is_none")]
    pub csh_clr_sys: Option<CashSettlementSystem4Choice>,
    #[serde(rename = "XpsrTp", skip_serializing_if = "Option::is_none")]
    pub xpsr_tp: Option<ExposureType22Choice>,
    #[serde(rename = "FxStgInstr", skip_serializing_if = "Option::is_none")]
    pub fx_stg_instr: Option<FxStandingInstruction4Choice>,
    #[serde(rename = "MktClntSd", skip_serializing_if = "Option::is_none")]
    pub mkt_clnt_sd: Option<MarketClientSide6Choice>,
    #[serde(rename = "NetgElgblty", skip_serializing_if = "Option::is_none")]
    pub netg_elgblty: Option<NettingEligibility4Choice>,
    #[serde(rename = "Regn", skip_serializing_if = "Option::is_none")]
    pub regn: Option<Registration9Choice>,
    #[serde(rename = "RpTp", skip_serializing_if = "Option::is_none")]
    pub rp_tp: Option<RepurchaseType23Choice>,
    #[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctns: Option<Restriction5Choice>,
    #[serde(rename = "SctiesRTGS", skip_serializing_if = "Option::is_none")]
    pub scties_rtgs: Option<SecuritiesRtgs4Choice>,
    #[serde(rename = "SttlgCpcty", skip_serializing_if = "Option::is_none")]
    pub sttlg_cpcty: Option<SettlingCapacity7Choice>,
    #[serde(rename = "SttlmSysMtd", skip_serializing_if = "Option::is_none")]
    pub sttlm_sys_mtd: Option<SettlementSystemMethod4Choice>,
    #[serde(rename = "TaxCpcty", skip_serializing_if = "Option::is_none")]
    pub tax_cpcty: Option<TaxCapacityParty4Choice>,
    #[serde(rename = "StmpDtyTaxBsis", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_tax_bsis: Option<GenericIdentification30>,
    #[serde(rename = "Trckg", skip_serializing_if = "Option::is_none")]
    pub trckg: Option<Tracking4Choice>,
    #[serde(rename = "AutomtcBrrwg", skip_serializing_if = "Option::is_none")]
    pub automtc_brrwg: Option<AutomaticBorrowing6Choice>,
    #[serde(rename = "LttrOfGrnt", skip_serializing_if = "Option::is_none")]
    pub lttr_of_grnt: Option<LetterOfGuarantee4Choice>,
    #[serde(rename = "RtrLeg", skip_serializing_if = "Option::is_none")]
    pub rtr_leg: Option<YesNoIndicator>,
    #[serde(rename = "ModCxlAllwd", skip_serializing_if = "Option::is_none")]
    pub mod_cxl_allwd: Option<ModificationCancellationAllowed4Choice>,
    #[serde(rename = "ElgblForColl", skip_serializing_if = "Option::is_none")]
    pub elgbl_for_coll: Option<YesNoIndicator>,
    #[serde(rename = "SctiesSubBalTp", skip_serializing_if = "Option::is_none")]
    pub scties_sub_bal_tp: Option<GenericIdentification30>,
    #[serde(rename = "CshSubBalTp", skip_serializing_if = "Option::is_none")]
    pub csh_sub_bal_tp: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternatePartyIdentification7 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType42Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BlockTrade1Code {
    #[serde(rename = "BLPA")]
    Blpa,
    #[serde(rename = "BLCH")]
    Blch,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NettingEligibility4ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NettingEligibility4Choice {
    #[serde(flatten)]
    pub value: NettingEligibility4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionType6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionType6Choice {
    #[serde(flatten)]
    pub value: OptionType6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingProcessingReason2Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "CDEL")]
    Cdel,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "CDAC")]
    Cdac,
    #[serde(rename = "INBC")]
    Inbc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reporting6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Reporting2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reporting6Choice {
    #[serde(flatten)]
    pub value: Reporting6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount171 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification133Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ComssnAcct", skip_serializing_if = "Option::is_none")]
    pub comssn_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesPaymentStatus1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NILL")]
    Nill,
    #[serde(rename = "PART")]
    Part,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionType44ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesTransactionType26Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionType44Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionType44ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeOriginator3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OriginatorRole2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeOriginator3Choice {
    #[serde(flatten)]
    pub value: TradeOriginator3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlingCapacity2Code {
    #[serde(rename = "SAGE")]
    Sage,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "SPRI")]
    Spri,
    #[serde(rename = "RISP")]
    Risp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle2Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification3ChoiceEnum {
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification3Choice {
    #[serde(flatten)]
    pub value: MarketIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType1ChoiceEnum {
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<YesNoIndicator>,
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType1Choice {
    #[serde(flatten)]
    pub value: YieldedOrValueType1ChoiceEnum,
}
#[derive(
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
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition4Code {
    #[serde(rename = "CBNS")]
    Cbns,
    #[serde(rename = "XBNS")]
    Xbns,
    #[serde(rename = "CCPN")]
    Ccpn,
    #[serde(rename = "XCPN")]
    Xcpn,
    #[serde(rename = "CDIV")]
    Cdiv,
    #[serde(rename = "XDIV")]
    Xdiv,
    #[serde(rename = "CRTS")]
    Crts,
    #[serde(rename = "XRTS")]
    Xrts,
    #[serde(rename = "CWAR")]
    Cwar,
    #[serde(rename = "XWAR")]
    Xwar,
    #[serde(rename = "SPCU")]
    Spcu,
    #[serde(rename = "SPEX")]
    Spex,
    #[serde(rename = "GTDL")]
    Gtdl,
    #[serde(rename = "BCRO")]
    Bcro,
    #[serde(rename = "BCRP")]
    Bcrp,
    #[serde(rename = "BCFD")]
    Bcfd,
    #[serde(rename = "BCBL")]
    Bcbl,
    #[serde(rename = "BCBN")]
    Bcbn,
    #[serde(rename = "MAPR")]
    Mapr,
    #[serde(rename = "NEGO")]
    Nego,
    #[serde(rename = "NMPR")]
    Nmpr,
    #[serde(rename = "BCPD")]
    Bcpd,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity51ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "OrgnlAndCurFace", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face: Option<OriginalAndCurrentQuantities1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity51Choice {
    #[serde(flatten)]
    pub value: Quantity51ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Linkages61 {
    #[serde(rename = "PrcgPos", skip_serializing_if = "Option::is_none")]
    pub prcg_pos: Option<ProcessingPosition7Choice>,
    #[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
    pub msg_nb: Option<DocumentNumber5Choice>,
    #[serde(rename = "Ref")]
    pub r#ref: References73Choice,
    #[serde(rename = "LkdQty", skip_serializing_if = "Option::is_none")]
    pub lkd_qty: Option<PairedOrTurnedQuantity5Choice>,
    #[serde(rename = "RefOwnr", skip_serializing_if = "Option::is_none")]
    pub ref_ownr: Option<PartyIdentification127Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficialOwnership4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficialOwnership4Choice {
    #[serde(flatten)]
    pub value: BeneficialOwnership4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepurchaseType10Code {
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "ROLP")]
    Rolp,
    #[serde(rename = "RATE")]
    Rate,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "CADJ")]
    Cadj,
    #[serde(rename = "TOPU")]
    Topu,
    #[serde(rename = "WTHD")]
    Wthd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType3Code {
    #[serde(rename = "VARI")]
    Vari,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct HoldIndicator6 {
    #[validate]
    #[serde(rename = "Ind")]
    pub ind: YesNoIndicator,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<RegistrationReason5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason8 {
    #[serde(rename = "Cd")]
    pub cd: RepairReason10Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Reporting2Code {
    #[serde(rename = "STEX")]
    Stex,
    #[serde(rename = "REGU")]
    Regu,
    #[serde(rename = "DEFR")]
    Defr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTypeAndIdentification27 {
    #[validate]
    #[serde(rename = "AcctOwnrTxId")]
    pub acct_ownr_tx_id: Max35Text,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<Max35Text>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(
        rename = "CtrPtyMktInfrstrctrTxId",
        skip_serializing_if = "Option::is_none"
    )]
    pub ctr_pty_mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none")]
    pub prcr_tx_id: Option<Max35Text>,
    #[serde(rename = "SctiesMvmntTp")]
    pub scties_mvmnt_tp: ReceiveDelivery1Code,
    #[serde(rename = "Pmt")]
    pub pmt: DeliveryReceiptType2Code,
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<Max35Text>,
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<Max35Text>,
    #[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_id: Option<Max35Text>,
}
#[derive(
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
pub struct RegistrationParameters6 {
    #[serde(rename = "CertfctnId", skip_serializing_if = "Option::is_none")]
    pub certfctn_id: Option<Max35Text>,
    #[serde(rename = "CertfctnDtTm", skip_serializing_if = "Option::is_none")]
    pub certfctn_dt_tm: Option<DateAndDateTime2Choice>,
    #[serde(rename = "RegarAcct", skip_serializing_if = "Option::is_none")]
    pub regar_acct: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<SecuritiesCertificate4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationStatus8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AffirmationStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationStatus8Choice {
    #[serde(flatten)]
    pub value: AffirmationStatus8ChoiceEnum,
}
#[derive(
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
pub enum GeneratedReason3Code {
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "CLAI")]
    Clai,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "RODE")]
    Rode,
    #[serde(rename = "SPLI")]
    Spli,
    #[serde(rename = "THRD")]
    Thrd,
    #[serde(rename = "TRAN")]
    Tran,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDate4Code {
    #[serde(rename = "WISS")]
    Wiss,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketType2Code {
    #[serde(rename = "PRIM")]
    Prim,
    #[serde(rename = "SECM")]
    Secm,
    #[serde(rename = "OTCO")]
    Otco,
    #[serde(rename = "VARI")]
    Vari,
    #[serde(rename = "EXCH")]
    Exch,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AffirmationStatus1Code {
    #[serde(rename = "AFFI")]
    Affi,
    #[serde(rename = "NAFI")]
    Nafi,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus24ChoiceEnum {
    #[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
    pub mtchd: Option<ProprietaryReason4>,
    #[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
    pub umtchd: Option<UnmatchedStatus16Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus24Choice {
    #[serde(flatten)]
    pub value: MatchingStatus24ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Eligibility1Code {
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PROF")]
    Prof,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus37ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus37Choice {
    #[serde(flatten)]
    pub value: PendingStatus37ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection44 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms23>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition8Code {
    #[serde(rename = "ASGN")]
    Asgn,
    #[serde(rename = "BUTC")]
    Butc,
    #[serde(rename = "CLEN")]
    Clen,
    #[serde(rename = "DIRT")]
    Dirt,
    #[serde(rename = "DLWM")]
    Dlwm,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "FRCL")]
    Frcl,
    #[serde(rename = "KNOC")]
    Knoc,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "RESI")]
    Resi,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "SPDL")]
    Spdl,
    #[serde(rename = "SPST")]
    Spst,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "PENS")]
    Pens,
    #[serde(rename = "UNEX")]
    Unex,
    #[serde(rename = "TRIP")]
    Trip,
    #[serde(rename = "NOMC")]
    Nomc,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "RHYP")]
    Rhyp,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "RPTO")]
    Rpto,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTradeDetails117 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TradId", default)]
    pub trad_id: Vec<Max52Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollTxId", default)]
    pub coll_tx_id: Vec<Max35Text>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification1>,
    #[serde(rename = "PlcOfClr", skip_serializing_if = "Option::is_none")]
    pub plc_of_clr: Option<PlaceOfClearingIdentification2>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate8Choice>,
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: SettlementDate17Choice,
    #[serde(rename = "LateDlvryDt", skip_serializing_if = "Option::is_none")]
    pub late_dlvry_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "AckdStsTmStmp", skip_serializing_if = "Option::is_none")]
    pub ackd_sts_tm_stmp: Option<IsoDateTime>,
    #[serde(rename = "MtchdStsTmStmp", skip_serializing_if = "Option::is_none")]
    pub mtchd_sts_tm_stmp: Option<IsoDateTime>,
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price10>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "OpngClsg", skip_serializing_if = "Option::is_none")]
    pub opng_clsg: Option<OpeningClosing3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rptg", default)]
    pub rptg: Vec<Reporting6Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition5Choice>,
    #[serde(rename = "InvstrCpcty", skip_serializing_if = "Option::is_none")]
    pub invstr_cpcty: Option<InvestorCapacity4Choice>,
    #[serde(rename = "TradOrgtrRole", skip_serializing_if = "Option::is_none")]
    pub trad_orgtr_role: Option<TradeOriginator3Choice>,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice29Choice>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus27Choice>,
    #[serde(rename = "AffirmSts", skip_serializing_if = "Option::is_none")]
    pub affirm_sts: Option<AffirmationStatus8Choice>,
    #[serde(rename = "FxAddtlDtls", skip_serializing_if = "Option::is_none")]
    pub fx_addtl_dtls: Option<Max350Text>,
    #[serde(
        rename = "SttlmInstrPrcgAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_instr_prcg_addtl_dtls: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebitCode {
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
pub struct FinancialInstrumentAttributes111 {
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification3Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat4Choice>,
    #[serde(rename = "RegnForm", skip_serializing_if = "Option::is_none")]
    pub regn_form: Option<FormOfSecurity6Choice>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency23Choice>,
    #[serde(rename = "PmtSts", skip_serializing_if = "Option::is_none")]
    pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
    #[serde(rename = "VarblRateChngFrqcy", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_chng_frqcy: Option<Frequency23Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle8Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType6Choice>,
    #[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
    pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "CpnDt", skip_serializing_if = "Option::is_none")]
    pub cpn_dt: Option<IsoDate>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[serde(rename = "FltgRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fltg_rate_fxg_dt: Option<IsoDate>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoDate>,
    #[serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cllbl_dt: Option<IsoDate>,
    #[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
    pub putbl_dt: Option<IsoDate>,
    #[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
    pub dtd_dt: Option<IsoDate>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<BaseOneRate>,
    #[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
    pub cur_fctr: Option<BaseOneRate>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<BaseOneRate>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<PercentageRate>,
    #[serde(rename = "YldToMtrtyRate", skip_serializing_if = "Option::is_none")]
    pub yld_to_mtrty_rate: Option<PercentageRate>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<PercentageRate>,
    #[serde(rename = "IndxRateBsis", skip_serializing_if = "Option::is_none")]
    pub indx_rate_bsis: Option<PercentageRate>,
    #[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
    pub cpn_attchd_nb: Option<Number22Choice>,
    #[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
    pub pool_nb: Option<GenericIdentification37>,
    #[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_ind: Option<YesNoIndicator>,
    #[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
    pub putbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "MktOrIndctvPric", skip_serializing_if = "Option::is_none")]
    pub mkt_or_indctv_pric: Option<PriceType4Choice>,
    #[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
    pub exrc_pric: Option<Price7>,
    #[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
    pub sbcpt_pric: Option<Price7>,
    #[serde(rename = "ConvsPric", skip_serializing_if = "Option::is_none")]
    pub convs_pric: Option<Price7>,
    #[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
    pub strk_pric: Option<Price7>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity33Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygFinInstrmId", default)]
    pub undrlyg_fin_instrm_id: Vec<SecurityIdentification19>,
    #[serde(
        rename = "FinInstrmAttrAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub fin_instrm_attr_addtl_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorCapacity4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Eligibility1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorCapacity4Choice {
    #[serde(flatten)]
    pub value: InvestorCapacity4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress1 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
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
pub struct PendingReason16 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason28Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason12ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason5Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason12Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason12ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification30 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
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
pub struct SettlementSystemMethod4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementSystemMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementSystemMethod4Choice {
    #[serde(flatten)]
    pub value: SettlementSystemMethod4ChoiceEnum,
}
#[derive(
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
pub struct GeneratedReasons5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GeneratedReason3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneratedReasons5Choice {
    #[serde(flatten)]
    pub value: GeneratedReasons5ChoiceEnum,
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
pub struct CashParties36 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentificationAndAccount164>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<PartyIdentificationAndAccount171>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentificationAndAccount164>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<PartyIdentificationAndAccount171>,
    #[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
    pub intrmy: Option<PartyIdentificationAndAccount171>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType12Code {
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CRDS")]
    Crds,
    #[serde(rename = "CRTL")]
    Crtl,
    #[serde(rename = "CRSP")]
    Crsp,
    #[serde(rename = "CCIR")]
    Ccir,
    #[serde(rename = "CRPR")]
    Crpr,
    #[serde(rename = "EQPT")]
    Eqpt,
    #[serde(rename = "EXTD")]
    Extd,
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXPT")]
    Expt,
    #[serde(rename = "FIXI")]
    Fixi,
    #[serde(rename = "FORX")]
    Forx,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "OTCD")]
    Otcd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SLOA")]
    Sloa,
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "SCRP")]
    Scrp,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "SHSL")]
    Shsl,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "UDMS")]
    Udms,
    #[serde(rename = "TRCP")]
    Trcp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeTransactionCondition4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition5Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType2ChoiceEnum {
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<YesNoIndicator>,
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType12Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType2Choice {
    #[serde(flatten)]
    pub value: YieldedOrValueType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FormOfSecurity6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FormOfSecurity1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FormOfSecurity6Choice {
    #[serde(flatten)]
    pub value: FormOfSecurity6ChoiceEnum,
}
#[derive(
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
pub enum FormOfSecurity1Code {
    #[serde(rename = "BEAR")]
    Bear,
    #[serde(rename = "REGD")]
    Regd,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairStatus12ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RepairReason8>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairStatus12Choice {
    #[serde(flatten)]
    pub value: RepairStatus12ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1ChoiceEnum {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1Choice {
    #[serde(flatten)]
    pub value: MarketIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max30DecimalNumber {
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
pub struct Exact3NumericText {
    #[validate(regex = "EXACT_3_NUMERIC_TEXT_REGEX")]
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
pub struct DocumentNumber5ChoiceEnum {
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification36>,
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber5Choice {
    #[serde(flatten)]
    pub value: DocumentNumber5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ICSD")]
    Icsd,
    #[serde(rename = "NCSD")]
    Ncsd,
    #[serde(rename = "SHHE")]
    Shhe,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PartyIdentificationAndAccount198 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockTrade4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BlockTrade1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockTrade4Choice {
    #[serde(flatten)]
    pub value: BlockTrade4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlingCapacity2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity7Choice {
    #[serde(flatten)]
    pub value: SettlingCapacity7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MatchingStatus1Code {
    #[serde(rename = "MACH")]
    Mach,
    #[serde(rename = "NMAT")]
    Nmat,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmounts39 {
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection44>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<AmountAndDirection44>,
    #[serde(rename = "CtryNtlFdrlTax", skip_serializing_if = "Option::is_none")]
    pub ctry_ntl_fdrl_tax: Option<AmountAndDirection44>,
    #[serde(rename = "TradAmt", skip_serializing_if = "Option::is_none")]
    pub trad_amt: Option<AmountAndDirection44>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<AmountAndDirection44>,
    #[serde(rename = "IsseDscntAllwnc", skip_serializing_if = "Option::is_none")]
    pub isse_dscnt_allwnc: Option<AmountAndDirection44>,
    #[serde(rename = "PmtLevyTax", skip_serializing_if = "Option::is_none")]
    pub pmt_levy_tax: Option<AmountAndDirection44>,
    #[serde(rename = "LclTax", skip_serializing_if = "Option::is_none")]
    pub lcl_tax: Option<AmountAndDirection44>,
    #[serde(rename = "LclTaxCtrySpcfc", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc: Option<AmountAndDirection44>,
    #[serde(rename = "LclBrkrComssn", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn: Option<AmountAndDirection44>,
    #[serde(rename = "Mrgn", skip_serializing_if = "Option::is_none")]
    pub mrgn: Option<AmountAndDirection44>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AmountAndDirection44>,
    #[serde(rename = "RgltryAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_amt: Option<AmountAndDirection44>,
    #[serde(rename = "ShppgAmt", skip_serializing_if = "Option::is_none")]
    pub shppg_amt: Option<AmountAndDirection44>,
    #[serde(rename = "SpclCncssn", skip_serializing_if = "Option::is_none")]
    pub spcl_cncssn: Option<AmountAndDirection44>,
    #[serde(rename = "StmpDty", skip_serializing_if = "Option::is_none")]
    pub stmp_dty: Option<AmountAndDirection44>,
    #[serde(rename = "StockXchgTax", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax: Option<AmountAndDirection44>,
    #[serde(rename = "TrfTax", skip_serializing_if = "Option::is_none")]
    pub trf_tax: Option<AmountAndDirection44>,
    #[serde(rename = "TxTax", skip_serializing_if = "Option::is_none")]
    pub tx_tax: Option<AmountAndDirection44>,
    #[serde(rename = "ValAddedTax", skip_serializing_if = "Option::is_none")]
    pub val_added_tax: Option<AmountAndDirection44>,
    #[serde(rename = "WhldgTax", skip_serializing_if = "Option::is_none")]
    pub whldg_tax: Option<AmountAndDirection44>,
    #[serde(rename = "NetGnLoss", skip_serializing_if = "Option::is_none")]
    pub net_gn_loss: Option<AmountAndDirection44>,
    #[serde(rename = "CsmptnTax", skip_serializing_if = "Option::is_none")]
    pub csmptn_tax: Option<AmountAndDirection44>,
    #[serde(rename = "AcrdCptlstnAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_cptlstn_amt: Option<AmountAndDirection44>,
    #[serde(rename = "RsrchFee", skip_serializing_if = "Option::is_none")]
    pub rsrch_fee: Option<AmountAndDirection44>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount165 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount197 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification120Choice>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<CountryCode>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason2Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "TAMM")]
    Tamm,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "INBC")]
    Inbc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
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
pub struct UnmatchedReason21ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnmatchedReason11Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason21Choice {
    #[serde(flatten)]
    pub value: UnmatchedReason21ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FailingReason2Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CANR")]
    Canr,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "OBJT")]
    Objt,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "STCD")]
    Stcd,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "MLAT")]
    Mlat,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "INBC")]
    Inbc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification37 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct MatchingStatus27ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MatchingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus27Choice {
    #[serde(flatten)]
    pub value: MatchingStatus27ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress5 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpeningClosing3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OpeningClosing1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpeningClosing3Choice {
    #[serde(flatten)]
    pub value: OpeningClosing3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateCode3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateCode3Choice {
    #[serde(flatten)]
    pub value: TradeDateCode3ChoiceEnum,
}
#[derive(
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
pub struct Number22ChoiceEnum {
    #[serde(rename = "Shrt", skip_serializing_if = "Option::is_none")]
    pub shrt: Option<Exact3NumericText>,
    #[serde(rename = "Lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number22Choice {
    #[serde(flatten)]
    pub value: Number22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price10 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType2Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount164 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ComssnAcct", skip_serializing_if = "Option::is_none")]
    pub comssn_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat29ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText8>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat29Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat29ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndIdentification1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AnyBicDec2014Identifier,
}
