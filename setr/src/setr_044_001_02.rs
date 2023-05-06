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
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:setr.044.001.02".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SimpleIdentificationInformation2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity6Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "BAGN")]
    Bagn,
    #[serde(rename = "CAGN")]
    Cagn,
    #[serde(rename = "CPRN")]
    Cprn,
    #[serde(rename = "OAGN")]
    Oagn,
    #[serde(rename = "PRAG")]
    Prag,
    #[serde(rename = "PRIN")]
    Prin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingDate1Code {
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
pub struct CurrencyToBuyOrSell1ChoiceEnum {
    #[serde(rename = "CcyToBuy", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy: Option<ActiveCurrencyCode>,
    #[serde(rename = "CcyToSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_sell: Option<ActiveCurrencyCode>,
}
#[derive(
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
pub struct ForeignExchangeTerms18 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[validate]
    #[serde(rename = "ConvtdAmt")]
    pub convtd_amt: ActiveCurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CallIn1Code {
    #[serde(rename = "CFAV")]
    Cfav,
    #[serde(rename = "CFST")]
    Cfst,
    #[serde(rename = "CFCC")]
    Cfcc,
    #[default]
    Unknown,
}
#[derive(
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
pub struct RejectionReason9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason28Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason9Choice {
    #[serde(flatten)]
    pub value: RejectionReason9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate4ChoiceEnum {
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<TradingDateCode1Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate4Choice {
    #[serde(flatten)]
    pub value: TradeDate4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationReason1 {
    #[serde(rename = "Cd")]
    pub cd: UnaffirmedReason2Choice,
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
pub struct AwaitingAffirmationReason1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AwaitingAffirmationReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AwaitingAffirmationReason1Choice {
    #[serde(flatten)]
    pub value: AwaitingAffirmationReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection29 {
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
    pub fx_dtls: Option<ForeignExchangeTerms18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationProcessingStatus6ChoiceEnum {
    #[serde(rename = "CxlReqd", skip_serializing_if = "Option::is_none")]
    pub cxl_reqd: Option<ProprietaryReason1>,
    #[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
    pub prtry_sts: Option<ProprietaryStatusAndReason1>,
    #[serde(rename = "CxlPdg", skip_serializing_if = "Option::is_none")]
    pub cxl_pdg: Option<CancellationReason11Choice>,
    #[serde(rename = "CxlCmpltd", skip_serializing_if = "Option::is_none")]
    pub cxl_cmpltd: Option<ProprietaryReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationProcessingStatus6Choice {
    #[serde(flatten)]
    pub value: CancellationProcessingStatus6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTime1ChoiceEnum {
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
pub struct DateAndDateTime1Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime1ChoiceEnum,
}
#[derive(
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
pub struct AffirmationStatus6ChoiceEnum {
    #[serde(rename = "Uaffrmd", skip_serializing_if = "Option::is_none")]
    pub uaffrmd: Option<AffirmationReason1Choice>,
    #[serde(rename = "Affrmd", skip_serializing_if = "Option::is_none")]
    pub affrmd: Option<ProprietaryReason1>,
    #[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
    pub prtry_sts: Option<ProprietaryStatusAndReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationStatus6Choice {
    #[serde(flatten)]
    pub value: AffirmationStatus6ChoiceEnum,
}
#[derive(
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
pub enum DeliveryReceiptType2Code {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "APMT")]
    Apmt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
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
pub enum PositionEffect2Code {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "ROLL")]
    Roll,
    #[serde(rename = "FIFO")]
    Fifo,
    #[serde(rename = "CLOA")]
    Cloa,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProprietaryStatusAndReason1 {
    #[validate]
    #[serde(rename = "PrtrySts")]
    pub prtry_sts: GenericIdentification20,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtryRsn", default)]
    pub prtry_rsn: Vec<ProprietaryReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason5 {
    #[serde(rename = "Cd")]
    pub cd: RepairReason9Choice,
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
pub struct SecuritiesTradeConfirmationStatusAdviceV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Id")]
    pub id: TransactiontIdentification4,
    #[validate(length(min = 1,))]
    #[serde(rename = "Refs", default)]
    pub refs: Vec<Linkages18>,
    #[serde(rename = "AffirmSts", skip_serializing_if = "Option::is_none")]
    pub affirm_sts: Option<AffirmationStatus6Choice>,
    #[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
    pub prcg_sts: Option<ProcessingStatus17Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus23Choice>,
    #[serde(rename = "RplcmntPrcgSts", skip_serializing_if = "Option::is_none")]
    pub rplcmnt_prcg_sts: Option<ReplacementProcessingStatus7Choice>,
    #[serde(rename = "CxlPrcgSts", skip_serializing_if = "Option::is_none")]
    pub cxl_prcg_sts: Option<CancellationProcessingStatus6Choice>,
    #[serde(rename = "PtyTradgDtls", skip_serializing_if = "Option::is_none")]
    pub pty_tradg_dtls: Option<Order18>,
    #[serde(rename = "CtrPtyTradgDtls", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_tradg_dtls: Option<Order18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ConfPties", default)]
    pub conf_pties: Vec<ConfirmationParties4>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties23>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties23>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice3Code {
    #[serde(rename = "AVER")]
    Aver,
    #[serde(rename = "AVOV")]
    Avov,
    #[serde(rename = "GREX")]
    Grex,
    #[serde(rename = "NET2")]
    Net2,
    #[serde(rename = "NET1")]
    Net1,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "RDAV")]
    Rdav,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AllegementReason1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AllegementReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AllegementReason1Choice {
    #[serde(flatten)]
    pub value: AllegementReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification54 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: AnyBicIdentifier,
    #[validate]
    #[serde(rename = "PrtryId")]
    pub prtry_id: GenericIdentification29,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessing1ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AwaitingAffirmationReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessing1Choice {
    #[serde(flatten)]
    pub value: PendingProcessing1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnmatchedReason4Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DEAL")]
    Deal,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "EXEC")]
    Exec,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "MCAN")]
    Mcan,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "POSE")]
    Pose,
    #[serde(rename = "BORT")]
    Bort,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "OTHI")]
    Othi,
    #[serde(rename = "BOFE")]
    Bofe,
    #[serde(rename = "TACR")]
    Tacr,
    #[serde(rename = "SDAT")]
    Sdat,
    #[serde(rename = "COID")]
    Coid,
    #[serde(rename = "SCRA")]
    Scra,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "SHAI")]
    Shai,
    #[serde(rename = "ACRS")]
    Acrs,
    #[serde(rename = "DEAS")]
    Deas,
    #[serde(rename = "CATI")]
    Cati,
    #[serde(rename = "TACS")]
    Tacs,
    #[serde(rename = "DBNM")]
    Dbnm,
    #[serde(rename = "MADA")]
    Mada,
    #[serde(rename = "OLID")]
    Olid,
    #[serde(rename = "TRSA")]
    Trsa,
    #[serde(rename = "TRTE")]
    Trte,
    #[serde(rename = "BOIA")]
    Boia,
    #[serde(rename = "OPLI")]
    Opli,
    #[serde(rename = "TRTR")]
    Trtr,
    #[serde(rename = "LWCO")]
    Lwco,
    #[serde(rename = "INTT")]
    Intt,
    #[serde(rename = "CUFC")]
    Cufc,
    #[serde(rename = "LTME")]
    Ltme,
    #[serde(rename = "ENFC")]
    Enfc,
    #[serde(rename = "CLSE")]
    Clse,
    #[default]
    Unknown,
}
#[derive(
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
    #[serde(rename = "SctiesTradConfStsAdvc")]
    pub scties_trad_conf_sts_advc: SecuritiesTradeConfirmationStatusAdviceV02<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AwaitingAffirmationReason1Code {
    #[serde(rename = "WAFF")]
    Waff,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason5 {
    #[serde(rename = "Cd")]
    pub cd: UnmatchedReason7Choice,
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
pub struct Quantity6ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity1Choice>,
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
pub struct Quantity6Choice {
    #[serde(flatten)]
    pub value: Quantity6ChoiceEnum,
}
#[derive(
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
pub enum TradingCapacity4Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "CPRN")]
    Cprn,
    #[serde(rename = "RISP")]
    Risp,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "CAGN")]
    Cagn,
    #[serde(rename = "OAGN")]
    Oagn,
    #[serde(rename = "PRAG")]
    Prag,
    #[serde(rename = "BAGN")]
    Bagn,
    #[serde(rename = "INFI")]
    Infi,
    #[serde(rename = "MKTM")]
    Mktm,
    #[serde(rename = "MLTF")]
    Mltf,
    #[serde(rename = "RMKT")]
    Rmkt,
    #[serde(rename = "SINT")]
    Sint,
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
pub struct CommissionType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CommissionType9Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommissionType2Choice {
    #[serde(flatten)]
    pub value: CommissionType2ChoiceEnum,
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
pub struct IsinIdentifier {
    #[validate(regex = "ISIN_IDENTIFIER_REGEX")]
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
pub struct MatchingStatus8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct MatchingStatus8Choice {
    #[serde(flatten)]
    pub value: MatchingStatus8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeType3Choice {
    #[serde(flatten)]
    pub value: TradeType3ChoiceEnum,
}
#[derive(
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
pub struct MarketType12ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType12Choice {
    #[serde(flatten)]
    pub value: MarketType12ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification55 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification68Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
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
pub struct YieldCalculation2 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
    #[serde(rename = "ClctnTp")]
    pub clctn_tp: CalculationType1Code,
    #[serde(rename = "RedPric", skip_serializing_if = "Option::is_none")]
    pub red_pric: Option<Price4>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "ValPrd", skip_serializing_if = "Option::is_none")]
    pub val_prd: Option<DateTimePeriodChoice>,
    #[serde(rename = "ClctnDt", skip_serializing_if = "Option::is_none")]
    pub clctn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionProcessingReason2ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RepairReason5>,
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
pub struct InstructionProcessingReason2Choice {
    #[serde(flatten)]
    pub value: InstructionProcessingReason2ChoiceEnum,
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
pub struct ConfirmationPartyDetails2 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(rename = "InvstrCpcty", skip_serializing_if = "Option::is_none")]
    pub invstr_cpcty: Option<InvestorCapacity3Choice>,
    #[serde(rename = "TradgPtyCpcty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty_cpcty: Option<TradingPartyCapacity1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification32ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification32Choice {
    #[serde(flatten)]
    pub value: PartyIdentification32ChoiceEnum,
}
#[derive(
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
pub struct IdentificationType40ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType40Choice {
    #[serde(flatten)]
    pub value: IdentificationType40ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress8 {
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
pub struct RegistrationParameters3 {
    #[serde(rename = "CertfctnId", skip_serializing_if = "Option::is_none")]
    pub certfctn_id: Option<Max35Text>,
    #[serde(rename = "CertfctnDtTm", skip_serializing_if = "Option::is_none")]
    pub certfctn_dt_tm: Option<DateAndDateTime1Choice>,
    #[serde(rename = "RegarAcct", skip_serializing_if = "Option::is_none")]
    pub regar_acct: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<SecuritiesCertificate3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingReason4ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AllegementMatchingReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingReason4Choice {
    #[serde(flatten)]
    pub value: MatchingReason4ChoiceEnum,
}
#[derive(
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
pub struct SettlementDateCode5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDate5Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode5Choice {
    #[serde(flatten)]
    pub value: SettlementDateCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price4 {
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmountChoice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PriceValueType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityOrAmount1ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantityChoice>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityOrAmount1Choice {
    #[serde(flatten)]
    pub value: QuantityOrAmount1ChoiceEnum,
}
#[derive(
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
pub struct ConfirmationPartyDetails6 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount3>,
    #[serde(rename = "CshDtls", skip_serializing_if = "Option::is_none")]
    pub csh_dtls: Option<AccountIdentification3Choice>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(rename = "PtyCpcty", skip_serializing_if = "Option::is_none")]
    pub pty_cpcty: Option<TradingPartyCapacity2Choice>,
    #[serde(
        rename = "InvstrPrtcnAssoctnMmbsh",
        skip_serializing_if = "Option::is_none"
    )]
    pub invstr_prtcn_assoctn_mmbsh: Option<YesNoIndicator>,
}
#[derive(
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
pub struct TradeTransactionCondition4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalTradeTransactionCondition1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition4Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDate5Code {
    #[serde(rename = "REGU")]
    Regu,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "NXTD")]
    Nxtd,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "TTWO")]
    Ttwo,
    #[serde(rename = "TTRE")]
    Ttre,
    #[serde(rename = "TFOR")]
    Tfor,
    #[serde(rename = "TFIV")]
    Tfiv,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "WDIS")]
    Wdis,
    #[serde(rename = "WHID")]
    Whid,
    #[serde(rename = "TBAT")]
    Tbat,
    #[serde(rename = "WISS")]
    Wiss,
    #[default]
    Unknown,
}
#[derive(
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
pub struct FinancialInstrumentQuantityChoiceEnum {
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantityChoice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantityChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodChoiceEnum {
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
    pub dt_tm_rg: Option<DateTimePeriodDetails>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodChoice {
    #[serde(flatten)]
    pub value: DateTimePeriodChoiceEnum,
}
#[derive(
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
pub struct ProprietaryReason1 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<GenericIdentification20>,
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
pub struct Order18 {
    #[serde(rename = "BizPrcTp", skip_serializing_if = "Option::is_none")]
    pub biz_prc_tp: Option<BusinessProcessType1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OrdrId", default)]
    pub ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntOrdrId", default)]
    pub clnt_ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndryClntOrdrId", default)]
    pub scndry_clnt_ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ListId", default)]
    pub list_id: Vec<Max35Text>,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification14,
    #[serde(rename = "Sd")]
    pub sd: Side3Code,
    #[serde(rename = "Pmt", skip_serializing_if = "Option::is_none")]
    pub pmt: Option<DeliveryReceiptType2Code>,
    #[serde(rename = "TradTxTp", skip_serializing_if = "Option::is_none")]
    pub trad_tx_tp: Option<TradeType3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition4Choice>,
    #[serde(rename = "PreAdvc", skip_serializing_if = "Option::is_none")]
    pub pre_advc: Option<YesNoIndicator>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<MarketIdentification79>,
    #[serde(rename = "OrdrBookgDt", skip_serializing_if = "Option::is_none")]
    pub ordr_bookg_dt: Option<IsoDate>,
    #[serde(rename = "TradOrgtnDt", skip_serializing_if = "Option::is_none")]
    pub trad_orgtn_dt: Option<IsoDateTime>,
    #[serde(rename = "TradDt")]
    pub trad_dt: TradeDate4Choice,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<TradeDate4Choice>,
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: SettlementDate8Choice,
    #[serde(rename = "NAVDt", skip_serializing_if = "Option::is_none")]
    pub nav_dt: Option<DateAndDateTime1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtlFillDtls", default)]
    pub prtl_fill_dtls: Vec<PartialFill2>,
    #[serde(rename = "ConfQty")]
    pub conf_qty: Quantity6Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "QtyBrkdwn", default)]
    pub qty_brkdwn: Vec<QuantityBreakdown11>,
    #[serde(rename = "GrssTradAmt", skip_serializing_if = "Option::is_none")]
    pub grss_trad_amt: Option<AmountAndDirection29>,
    #[validate]
    #[serde(rename = "DealPric")]
    pub deal_pric: Price4,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice10Choice>,
    #[serde(rename = "CshMrgn", skip_serializing_if = "Option::is_none")]
    pub csh_mrgn: Option<CashMarginOrder1Code>,
    #[serde(rename = "Comssn", skip_serializing_if = "Option::is_none")]
    pub comssn: Option<Commission16>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "GvUpNbOfDays", skip_serializing_if = "Option::is_none")]
    pub gv_up_nb_of_days: Option<Max3Number>,
    #[serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none")]
    pub intrst_tp: Option<InterestType2Code>,
    #[serde(rename = "AcrdIntrstPctg", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_pctg: Option<PercentageRate>,
    #[serde(rename = "TradRgltryCondsTp", skip_serializing_if = "Option::is_none")]
    pub trad_rgltry_conds_tp: Option<TradeRegulatoryConditions1Code>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "OrdrOrgtrElgblty", skip_serializing_if = "Option::is_none")]
    pub ordr_orgtr_elgblty: Option<Eligibility1Code>,
    #[serde(rename = "PosFct", skip_serializing_if = "Option::is_none")]
    pub pos_fct: Option<PositionEffect2Code>,
    #[serde(rename = "DerivCvrd", skip_serializing_if = "Option::is_none")]
    pub deriv_cvrd: Option<YesNoIndicator>,
    #[serde(rename = "ChrgTaxBsisTp", skip_serializing_if = "Option::is_none")]
    pub chrg_tax_bsis_tp: Option<ChargeTaxBasisType1Choice>,
    #[serde(rename = "CptlGnTp", skip_serializing_if = "Option::is_none")]
    pub cptl_gn_tp: Option<EuCapitalGainType2Choice>,
    #[serde(rename = "MtchSts", skip_serializing_if = "Option::is_none")]
    pub mtch_sts: Option<MatchingStatus8Choice>,
    #[serde(rename = "CallInTp", skip_serializing_if = "Option::is_none")]
    pub call_in_tp: Option<CallIn1Code>,
    #[serde(rename = "YldTp", skip_serializing_if = "Option::is_none")]
    pub yld_tp: Option<YieldCalculation2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rptg", default)]
    pub rptg: Vec<Reporting5Choice>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters3>,
    #[serde(
        rename = "AddtlTradInstrPrcgInf",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_trad_instr_prcg_inf: Option<Max350Text>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmountChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmountChoice {
    #[serde(flatten)]
    pub value: PriceRateOrAmountChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingCapacity6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity2Choice {
    #[serde(flatten)]
    pub value: TradingPartyCapacity2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationReference11ChoiceEnum {
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<Max35Text>,
    #[serde(rename = "CmplcId", skip_serializing_if = "Option::is_none")]
    pub cmplc_id: Option<Max35Text>,
    #[serde(rename = "CollTxId", skip_serializing_if = "Option::is_none")]
    pub coll_tx_id: Option<Max35Text>,
    #[serde(rename = "InstgPtyTxId", skip_serializing_if = "Option::is_none")]
    pub instg_pty_tx_id: Option<Max35Text>,
    #[serde(rename = "ScndryAllcnId", skip_serializing_if = "Option::is_none")]
    pub scndry_allcn_id: Option<Max35Text>,
    #[serde(rename = "IndxId", skip_serializing_if = "Option::is_none")]
    pub indx_id: Option<Max35Text>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "ClntOrdrLkId", skip_serializing_if = "Option::is_none")]
    pub clnt_ordr_lk_id: Option<Max35Text>,
    #[serde(rename = "CxlReqId", skip_serializing_if = "Option::is_none")]
    pub cxl_req_id: Option<Max35Text>,
    #[serde(rename = "ExctgPtyTxId", skip_serializing_if = "Option::is_none")]
    pub exctg_pty_tx_id: Option<Max35Text>,
    #[serde(rename = "BlckId", skip_serializing_if = "Option::is_none")]
    pub blck_id: Option<Max35Text>,
    #[serde(rename = "AllcnId", skip_serializing_if = "Option::is_none")]
    pub allcn_id: Option<Max35Text>,
    #[serde(rename = "IndvAllcnId", skip_serializing_if = "Option::is_none")]
    pub indv_allcn_id: Option<Max35Text>,
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationReference11Choice {
    #[serde(flatten)]
    pub value: IdentificationReference11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrRate2ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct AmountOrRate2Choice {
    #[serde(flatten)]
    pub value: AmountOrRate2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingCapacity4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity1Choice {
    #[serde(flatten)]
    pub value: TradingPartyCapacity1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepairReason7Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "BUSE")]
    Buse,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "COMC")]
    Comc,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "FEEE")]
    Feee,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "NRGM")]
    Nrgm,
    #[serde(rename = "NRGN")]
    Nrgn,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "CTRA")]
    Ctra,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "ULNK")]
    Ulnk,
    #[serde(rename = "VASU")]
    Vasu,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PurposeCode5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesAccountPurposeType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PurposeCode5Choice {
    #[serde(flatten)]
    pub value: PurposeCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties23 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification55>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount34>,
}
#[derive(
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
pub struct GenericIdentification38 {
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
pub enum TradeRegulatoryConditions1Code {
    #[serde(rename = "SOLI")]
    Soli,
    #[serde(rename = "USOL")]
    Usol,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpicIdentifier {
    #[validate(regex = "UPIC_IDENTIFIER_REGEX")]
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
pub struct UnmatchedReason7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnmatchedReason4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason7Choice {
    #[serde(flatten)]
    pub value: UnmatchedReason7ChoiceEnum,
}
#[derive(
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
pub struct ProcessingStatus17ChoiceEnum {
    #[serde(
        rename = "TradgSspdByStockXchg",
        skip_serializing_if = "Option::is_none"
    )]
    pub tradg_sspd_by_stock_xchg: Option<ProprietaryReason1>,
    #[serde(rename = "Trtd", skip_serializing_if = "Option::is_none")]
    pub trtd: Option<ProprietaryReason1>,
    #[serde(rename = "InRpr", skip_serializing_if = "Option::is_none")]
    pub in_rpr: Option<InstructionProcessingReason2Choice>,
    #[serde(rename = "DfltActn", skip_serializing_if = "Option::is_none")]
    pub dflt_actn: Option<ProprietaryReason1>,
    #[serde(rename = "StgInstr", skip_serializing_if = "Option::is_none")]
    pub stg_instr: Option<ProprietaryReason1>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<InstructionProcessingReason1Choice>,
    #[serde(rename = "Futr", skip_serializing_if = "Option::is_none")]
    pub futr: Option<ProprietaryReason1>,
    #[serde(rename = "RcvdAtIntrmy", skip_serializing_if = "Option::is_none")]
    pub rcvd_at_intrmy: Option<ProprietaryReason1>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<ProprietaryReason1>,
    #[serde(
        rename = "AlrdyMtchdAndAffrmd",
        skip_serializing_if = "Option::is_none"
    )]
    pub alrdy_mtchd_and_affrmd: Option<ProprietaryReason1>,
    #[serde(rename = "Gnrtd", skip_serializing_if = "Option::is_none")]
    pub gnrtd: Option<ProprietaryReason1>,
    #[serde(rename = "ForcdRjctn", skip_serializing_if = "Option::is_none")]
    pub forcd_rjctn: Option<ProprietaryReason1>,
    #[serde(rename = "SttlmInstrSnt", skip_serializing_if = "Option::is_none")]
    pub sttlm_instr_snt: Option<ProprietaryReason1>,
    #[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
    pub prtry_sts: Option<ProprietaryStatusAndReason1>,
    #[serde(rename = "NoInstr", skip_serializing_if = "Option::is_none")]
    pub no_instr: Option<ProprietaryReason1>,
    #[serde(rename = "FullyExctdConfSnt", skip_serializing_if = "Option::is_none")]
    pub fully_exctd_conf_snt: Option<ProprietaryReason1>,
    #[serde(rename = "OpnOrdr", skip_serializing_if = "Option::is_none")]
    pub opn_ordr: Option<ProprietaryReason1>,
    #[serde(rename = "Done", skip_serializing_if = "Option::is_none")]
    pub done: Option<ProprietaryReason1>,
    #[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg: Option<PendingProcessing1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus17Choice {
    #[serde(flatten)]
    pub value: ProcessingStatus17ChoiceEnum,
}
#[derive(
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
pub enum InterestType2Code {
    #[serde(rename = "CINT")]
    Cint,
    #[serde(rename = "XINT")]
    Xint,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuCapitalGainType2ChoiceEnum {
    #[serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none")]
    pub eu_cptl_gn: Option<EuCapitalGain2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuCapitalGainType2Choice {
    #[serde(flatten)]
    pub value: EuCapitalGainType2ChoiceEnum,
}
#[derive(
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
pub struct ConfirmationPartyDetails1 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartialFill2 {
    #[serde(rename = "ConfQty")]
    pub conf_qty: Quantity6Choice,
    #[validate]
    #[serde(rename = "DealPric")]
    pub deal_pric: Price4,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate4Choice>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<MarketIdentification80>,
    #[serde(rename = "OrgnlOrdrdQty")]
    pub orgnl_ordrd_qty: QuantityOrAmount1Choice,
    #[serde(rename = "PrevslyExctdQty")]
    pub prevsly_exctd_qty: QuantityOrAmount1Choice,
    #[serde(rename = "RmngQty")]
    pub rmng_qty: QuantityOrAmount1Choice,
    #[serde(rename = "MtchIncrmtQty", skip_serializing_if = "Option::is_none")]
    pub mtch_incrmt_qty: Option<QuantityOrAmount1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepairReason7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason9Choice {
    #[serde(flatten)]
    pub value: RepairReason9ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification2Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
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
pub struct TypeOfPrice10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice10Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification20 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType7Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "YIEL")]
    Yiel,
    #[serde(rename = "SPRE")]
    Spre,
    #[serde(rename = "PEUN")]
    Peun,
    #[serde(rename = "ABSO")]
    Abso,
    #[serde(rename = "TEDP")]
    Tedp,
    #[serde(rename = "TEDY")]
    Tedy,
    #[serde(rename = "FICT")]
    Fict,
    #[serde(rename = "VACT")]
    Vact,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "ACTU")]
    Actu,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType11ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType11Choice {
    #[serde(flatten)]
    pub value: MarketType11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodDetails {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BusinessProcessType1Code {
    #[serde(rename = "ISUP")]
    Isup,
    #[serde(rename = "NISP")]
    Nisp,
    #[serde(rename = "PRAC")]
    Prac,
    #[serde(rename = "RSAL")]
    Rsal,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "THRU")]
    Thru,
    #[serde(rename = "IDEL")]
    Idel,
    #[serde(rename = "DPLX")]
    Dplx,
    #[default]
    Unknown,
}
#[derive(
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
pub struct InvestorCapacity3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Eligibility1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorCapacity3Choice {
    #[serde(flatten)]
    pub value: InvestorCapacity3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesAccountPurposeType1Code {
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "ABRD")]
    Abrd,
    #[serde(rename = "CEND")]
    Cend,
    #[serde(rename = "DVPA")]
    Dvpa,
    #[serde(rename = "PHYS")]
    Phys,
    #[default]
    Unknown,
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
pub struct GenericIdentification29 {
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
pub struct ReplacementProcessingStatus7ChoiceEnum {
    #[serde(rename = "InRpr", skip_serializing_if = "Option::is_none")]
    pub in_rpr: Option<ProprietaryReason1>,
    #[serde(rename = "RcvdAtIntrmy", skip_serializing_if = "Option::is_none")]
    pub rcvd_at_intrmy: Option<ProprietaryReason1>,
    #[serde(rename = "Dnd", skip_serializing_if = "Option::is_none")]
    pub dnd: Option<ProprietaryReason1>,
    #[serde(rename = "Cmpltd", skip_serializing_if = "Option::is_none")]
    pub cmpltd: Option<ProprietaryReason1>,
    #[serde(rename = "ModReqd", skip_serializing_if = "Option::is_none")]
    pub mod_reqd: Option<ProprietaryReason1>,
    #[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
    pub prtry_sts: Option<ProprietaryStatusAndReason1>,
    #[serde(rename = "PrtlRplcmntAccptd", skip_serializing_if = "Option::is_none")]
    pub prtl_rplcmnt_accptd: Option<ProprietaryReason1>,
    #[serde(rename = "RcvdAtStockXchg", skip_serializing_if = "Option::is_none")]
    pub rcvd_at_stock_xchg: Option<ProprietaryReason1>,
    #[serde(rename = "Accptd", skip_serializing_if = "Option::is_none")]
    pub accptd: Option<ProprietaryReason1>,
    #[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
    pub pdg: Option<ProprietaryReason1>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<ProprietaryReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReplacementProcessingStatus7Choice {
    #[serde(flatten)]
    pub value: ReplacementProcessingStatus7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AllegementReason1Code {
    #[serde(rename = "ALG1")]
    Alg1,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PartyIdentificationAndAccount34 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeType3Code {
    #[serde(rename = "BSKT")]
    Bskt,
    #[serde(rename = "INDX")]
    Indx,
    #[serde(rename = "IPOO")]
    Ipoo,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "PRAL")]
    Pral,
    #[serde(rename = "PROG")]
    Prog,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "BRBR")]
    Brbr,
    #[serde(rename = "RISK")]
    Risk,
    #[serde(rename = "VWAP")]
    Vwap,
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "GUAR")]
    Guar,
    #[serde(rename = "EMTR")]
    Emtr,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "BOST")]
    Bost,
    #[serde(rename = "BOEN")]
    Boen,
    #[serde(rename = "LABO")]
    Labo,
    #[serde(rename = "BORE")]
    Bore,
    #[serde(rename = "OFIT")]
    Ofit,
    #[serde(rename = "BOSU")]
    Bosu,
    #[serde(rename = "FBBT")]
    Fbbt,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "FUOP")]
    Fuop,
    #[serde(rename = "FUTR")]
    Futr,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PartyIdentificationAndAccount79 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification32Choice>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus23ChoiceEnum {
    #[serde(rename = "MtchdWthTlrnce", skip_serializing_if = "Option::is_none")]
    pub mtchd_wth_tlrnce: Option<ProprietaryReason1>,
    #[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
    pub prtry_sts: Option<ProprietaryStatusAndReason1>,
    #[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
    pub mtchd: Option<ProprietaryReason1>,
    #[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
    pub umtchd: Option<MatchingReason1Choice>,
    #[serde(rename = "MtchgAllgd", skip_serializing_if = "Option::is_none")]
    pub mtchg_allgd: Option<MatchingReason4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus23Choice {
    #[serde(flatten)]
    pub value: MatchingStatus23ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Side3Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "CROS")]
    Cros,
    #[serde(rename = "CRSH")]
    Crsh,
    #[serde(rename = "CSHE")]
    Cshe,
    #[serde(rename = "DEFI")]
    Defi,
    #[serde(rename = "OPPO")]
    Oppo,
    #[serde(rename = "UNDI")]
    Undi,
    #[serde(rename = "TWOS")]
    Twos,
    #[serde(rename = "BUMI")]
    Bumi,
    #[serde(rename = "SEPL")]
    Sepl,
    #[serde(rename = "SESH")]
    Sesh,
    #[serde(rename = "SSEX")]
    Ssex,
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORW")]
    Borw,
    #[serde(rename = "OPEX")]
    Opex,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AwaitingCancellationReason1Code {
    #[serde(rename = "WCAN")]
    Wcan,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PurposeCode5Choice>,
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
pub struct SecuritiesCertificate3 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
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
pub struct Linkages18 {
    #[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
    pub msg_nb: Option<DocumentNumber4Choice>,
    #[serde(rename = "Ref")]
    pub r#ref: IdentificationReference11Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BbanIdentifier {
    #[validate(regex = "BBAN_IDENTIFIER_REGEX")]
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
pub struct CancellationReason11ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AwaitingCancellationReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason11Choice {
    #[serde(flatten)]
    pub value: CancellationReason11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingReason1ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<UnmatchedReason5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingReason1Choice {
    #[serde(flatten)]
    pub value: MatchingReason1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnaffirmedReason1Code {
    #[serde(rename = "NAFF")]
    Naff,
    #[default]
    Unknown,
}
#[derive(
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
pub struct InstructionProcessingReason1ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionReason9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionProcessingReason1Choice {
    #[serde(flatten)]
    pub value: InstructionProcessingReason1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CalculationType1Code {
    #[serde(rename = "AFTX")]
    Aftx,
    #[serde(rename = "ANNU")]
    Annu,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "AVMA")]
    Avma,
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "YTNC")]
    Ytnc,
    #[serde(rename = "CHCL")]
    Chcl,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "CUYI")]
    Cuyi,
    #[serde(rename = "TRGR")]
    Trgr,
    #[serde(rename = "GVEQ")]
    Gveq,
    #[serde(rename = "FLAS")]
    Flas,
    #[serde(rename = "NVFL")]
    Nvfl,
    #[serde(rename = "LSCL")]
    Lscl,
    #[serde(rename = "LSMT")]
    Lsmt,
    #[serde(rename = "LSQR")]
    Lsqr,
    #[serde(rename = "LSYR")]
    Lsyr,
    #[serde(rename = "LGAL")]
    Lgal,
    #[serde(rename = "MARK")]
    Mark,
    #[serde(rename = "YTMA")]
    Ytma,
    #[serde(rename = "NXRF")]
    Nxrf,
    #[serde(rename = "PNAV")]
    Pnav,
    #[serde(rename = "NXPT")]
    Nxpt,
    #[serde(rename = "PRCL")]
    Prcl,
    #[serde(rename = "PRYL")]
    Pryl,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "SHLF")]
    Shlf,
    #[serde(rename = "SPLL")]
    Spll,
    #[serde(rename = "TXQV")]
    Txqv,
    #[serde(rename = "TTDT")]
    Ttdt,
    #[serde(rename = "TRYL")]
    Tryl,
    #[serde(rename = "WRST")]
    Wrst,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification68ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
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
pub struct PartyIdentification68Choice {
    #[serde(flatten)]
    pub value: PartyIdentification68ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessProcessType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BusinessProcessType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessProcessType1Choice {
    #[serde(flatten)]
    pub value: BusinessProcessType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AwaitingCancellationReason1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AwaitingCancellationReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AwaitingCancellationReason1Choice {
    #[serde(flatten)]
    pub value: AwaitingCancellationReason1ChoiceEnum,
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
pub struct AllegementMatchingReason1 {
    #[serde(rename = "Cd")]
    pub cd: AllegementReason1Choice,
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
pub struct Reporting5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Reporting2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reporting5Choice {
    #[serde(flatten)]
    pub value: Reporting5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingDateCode1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingDate1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingDateCode1Choice {
    #[serde(flatten)]
    pub value: TradingDateCode1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification80 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification3Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<MarketType12Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate8ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDateCode5Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate8Choice {
    #[serde(flatten)]
    pub value: SettlementDate8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnaffirmedReason2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnaffirmedReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnaffirmedReason2Choice {
    #[serde(flatten)]
    pub value: UnaffirmedReason2ChoiceEnum,
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
pub struct OriginalAndCurrentQuantities1 {
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
#[derive(
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
pub struct AccountIdentification3ChoiceEnum {
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "PrtryAcct", skip_serializing_if = "Option::is_none")]
    pub prtry_acct: Option<SimpleIdentificationInformation2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification3Choice {
    #[serde(flatten)]
    pub value: AccountIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationReason1ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AffirmationReason1>,
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
pub struct AffirmationReason1Choice {
    #[serde(flatten)]
    pub value: AffirmationReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AwaitingCancellationReason1 {
    #[serde(rename = "Cd")]
    pub cd: AwaitingCancellationReason1Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeTaxBasis1Code {
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PERU")]
    Peru,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationPartyDetails3 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount3>,
    #[serde(rename = "CshDtls", skip_serializing_if = "Option::is_none")]
    pub csh_dtls: Option<AccountIdentification3Choice>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(rename = "PtyCpcty", skip_serializing_if = "Option::is_none")]
    pub pty_cpcty: Option<TradingPartyCapacity2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation5 {
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
pub struct TransactiontIdentification4 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AwaitingAffirmationReason1 {
    #[serde(rename = "Cd")]
    pub cd: AwaitingAffirmationReason1Choice,
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
pub struct AlternatePartyIdentification5 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType40Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CommissionType9Code {
    #[serde(rename = "CLDI")]
    Cldi,
    #[serde(rename = "STEP")]
    Step,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "PERN")]
    Pern,
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PERU")]
    Peru,
    #[serde(rename = "PWCD")]
    Pwcd,
    #[serde(rename = "PWEU")]
    Pweu,
    #[serde(rename = "BRKR")]
    Brkr,
    #[serde(rename = "DFDP")]
    Dfdp,
    #[serde(rename = "PBOC")]
    Pboc,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashMarginOrder1Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "MRGO")]
    Mrgo,
    #[serde(rename = "MRGC")]
    Mrgc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketType6Code {
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
pub struct NameAndAddress13 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationParties4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Invstr", default)]
    pub invstr: Vec<PartyIdentificationAndAccount79>,
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Brrwr", skip_serializing_if = "Option::is_none")]
    pub brrwr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Lndr", skip_serializing_if = "Option::is_none")]
    pub lndr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "BrkrOfCdt", skip_serializing_if = "Option::is_none")]
    pub brkr_of_cdt: Option<ConfirmationPartyDetails3>,
    #[serde(rename = "IntrdcgFirm", skip_serializing_if = "Option::is_none")]
    pub intrdcg_firm: Option<ConfirmationPartyDetails3>,
    #[serde(rename = "StepInFirm", skip_serializing_if = "Option::is_none")]
    pub step_in_firm: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "StepOutFirm", skip_serializing_if = "Option::is_none")]
    pub step_out_firm: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "ClrFirm", skip_serializing_if = "Option::is_none")]
    pub clr_firm: Option<ConfirmationPartyDetails6>,
    #[serde(rename = "ExctgBrkr", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr: Option<ConfirmationPartyDetails6>,
    #[serde(rename = "AffrmgPty", skip_serializing_if = "Option::is_none")]
    pub affrmg_pty: Option<ConfirmationPartyDetails3>,
    #[serde(rename = "TradBnfcryPty", skip_serializing_if = "Option::is_none")]
    pub trad_bnfcry_pty: Option<ConfirmationPartyDetails3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalTradeTransactionCondition1Code {
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
pub struct FinancialInstrumentQuantity1ChoiceEnum {
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
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
pub struct FinancialInstrumentQuantity1Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityBreakdown11 {
    #[serde(rename = "LotNb", skip_serializing_if = "Option::is_none")]
    pub lot_nb: Option<GenericIdentification37>,
    #[serde(rename = "LotQty", skip_serializing_if = "Option::is_none")]
    pub lot_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "LotDtTm", skip_serializing_if = "Option::is_none")]
    pub lot_dt_tm: Option<DateAndDateTime1Choice>,
    #[serde(rename = "LotPric", skip_serializing_if = "Option::is_none")]
    pub lot_pric: Option<Price4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EuCapitalGain2Code {
    #[serde(rename = "EUSI")]
    Eusi,
    #[serde(rename = "EUSO")]
    Euso,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason28Code {
    #[serde(rename = "ASTM")]
    Astm,
    #[serde(rename = "BUSE")]
    Buse,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "COMC")]
    Comc,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "FEEE")]
    Feee,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "ODNP")]
    Odnp,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "QUNP")]
    Qunp,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "CTRA")]
    Ctra,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "TNAR")]
    Tnar,
    #[serde(rename = "TNIU")]
    Tniu,
    #[serde(rename = "TQHI")]
    Tqhi,
    #[serde(rename = "TQNP")]
    Tqnp,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "ULNK")]
    Ulnk,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "INIR")]
    Inir,
    #[serde(rename = "OPNM")]
    Opnm,
    #[serde(rename = "OPTY")]
    Opty,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "EXLI")]
    Exli,
    #[serde(rename = "INPR")]
    Inpr,
    #[serde(rename = "PRIC")]
    Pric,
    #[serde(rename = "EQTY")]
    Eqty,
    #[serde(rename = "SIDE")]
    Side,
    #[serde(rename = "CADI")]
    Cadi,
    #[serde(rename = "CPTY")]
    Cpty,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "DISE")]
    Dise,
    #[serde(rename = "RESU")]
    Resu,
    #[serde(rename = "XRAT")]
    Xrat,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "GAMN")]
    Gamn,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "INDT")]
    Indt,
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
pub struct RejectionReason9 {
    #[serde(rename = "Cd")]
    pub cd: RejectionReason9Choice,
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
pub struct AnyBicIdentifier {
    #[validate(regex = "ANY_BIC_IDENTIFIER_REGEX")]
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
pub struct CashAccountIdentification2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max34Text>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification2Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification79 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification3Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<MarketType11Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeTaxBasisType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ChargeTaxBasis1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeTaxBasisType1Choice {
    #[serde(flatten)]
    pub value: ChargeTaxBasisType1ChoiceEnum,
}
#[derive(
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
pub struct SecurityIdentification14 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
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
pub struct DocumentNumber4ChoiceEnum {
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification38>,
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
pub struct DocumentNumber4Choice {
    #[serde(flatten)]
    pub value: DocumentNumber4ChoiceEnum,
}
#[derive(
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
pub struct Commission16 {
    #[serde(rename = "Tp")]
    pub tp: CommissionType2Choice,
    #[serde(rename = "Comssn")]
    pub comssn: AmountOrRate2Choice,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification54>,
    #[serde(rename = "ClctnDt", skip_serializing_if = "Option::is_none")]
    pub clctn_dt: Option<IsoDate>,
    #[serde(rename = "TtlComssn", skip_serializing_if = "Option::is_none")]
    pub ttl_comssn: Option<AmountAndDirection29>,
    #[serde(rename = "TtlVATAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_vat_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "VATRate", skip_serializing_if = "Option::is_none")]
    pub vat_rate: Option<BaseOneRate>,
}
#[derive(
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
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
}
