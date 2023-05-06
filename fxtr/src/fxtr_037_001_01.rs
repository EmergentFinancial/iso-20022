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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.037.001.01".to_string()
}

#[derive(
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
pub struct TickerIdentifier {
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
pub struct BloombergIdentifier {
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
pub struct InstrumentLeg6 {
    #[serde(rename = "LegSd")]
    pub leg_sd: Side1Code,
    #[serde(rename = "LegSttlmTp")]
    pub leg_sttlm_tp: SettlementDateCode,
    #[validate]
    #[serde(rename = "LegSttlmDt")]
    pub leg_sttlm_dt: IsoDateTime,
    #[validate]
    #[serde(rename = "LegLastPric")]
    pub leg_last_pric: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "LegSttlmCcy")]
    pub leg_sttlm_ccy: CurrencyCode,
    #[validate]
    #[serde(rename = "LegOrdrQty")]
    pub leg_ordr_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegFwdPts")]
    pub leg_fwd_pts: DecimalNumber,
    #[validate]
    #[serde(rename = "LegClctdCtrPtyCcyLastQty")]
    pub leg_clctd_ctr_pty_ccy_last_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegRskAmt")]
    pub leg_rsk_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegValtnRate")]
    pub leg_valtn_rate: AgreedRate3,
    #[validate]
    #[serde(rename = "LegValDt")]
    pub leg_val_dt: IsoDate,
    #[serde(rename = "LegCcy")]
    pub leg_ccy: CurrencyCode,
    #[validate]
    #[serde(rename = "LegSymb")]
    pub leg_symb: Max35Text,
    #[validate]
    #[serde(rename = "LegSctyId")]
    pub leg_scty_id: SecurityIdentification18,
}
#[derive(
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
pub struct SecurityIdentification22ChoiceEnum {
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternateIdentification1>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<BloombergIdentifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification22Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification22ChoiceEnum,
}
#[derive(
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IdentificationType1Code {
    #[serde(rename = "BASC")]
    Basc,
    #[serde(rename = "BICO")]
    Bico,
    #[serde(rename = "CFET")]
    Cfet,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IdentificationType2Code {
    #[serde(rename = "CDCO")]
    Cdco,
    #[serde(rename = "CFET")]
    Cfet,
    #[serde(rename = "RICC")]
    Ricc,
    #[serde(rename = "USDE")]
    Usde,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource1ChoiceEnum {
    #[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
    pub dmst: Option<CountryCode>,
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
pub struct IdentificationSource1Choice {
    #[serde(flatten)]
    pub value: IdentificationSource1ChoiceEnum,
}
#[derive(
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
pub struct AgreedRate3 {
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "QtdCcy", skip_serializing_if = "Option::is_none")]
    pub qtd_ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingModeType1Code {
    #[serde(rename = "QUDR")]
    Qudr,
    #[serde(rename = "ORDR")]
    Ordr,
    #[serde(rename = "NETR")]
    Netr,
    #[serde(rename = "AUCT")]
    Auct,
    #[serde(rename = "MARC")]
    Marc,
    #[serde(rename = "BILA")]
    Bila,
    #[serde(rename = "ANON")]
    Anon,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountInformationType1Code {
    #[serde(rename = "IBND")]
    Ibnd,
    #[serde(rename = "IBCC")]
    Ibcc,
    #[serde(rename = "IBDC")]
    Ibdc,
    #[serde(rename = "BIBC")]
    Bibc,
    #[serde(rename = "BIBD")]
    Bibd,
    #[serde(rename = "BINC")]
    Binc,
    #[serde(rename = "BIND")]
    Bind,
    #[serde(rename = "BICC")]
    Bicc,
    #[serde(rename = "BIDC")]
    Bidc,
    #[serde(rename = "CMSA")]
    Cmsa,
    #[serde(rename = "CBBC")]
    Cbbc,
    #[serde(rename = "CBBD")]
    Cbbd,
    #[serde(rename = "CBNC")]
    Cbnc,
    #[serde(rename = "CBND")]
    Cbnd,
    #[serde(rename = "CBCC")]
    Cbcc,
    #[serde(rename = "CBDC")]
    Cbdc,
    #[serde(rename = "CUAC")]
    Cuac,
    #[serde(rename = "DEAC")]
    Deac,
    #[serde(rename = "FCAA")]
    Fcaa,
    #[serde(rename = "FCAN")]
    Fcan,
    #[serde(rename = "FCBN")]
    Fcbn,
    #[serde(rename = "IBBC")]
    Ibbc,
    #[serde(rename = "IBBD")]
    Ibbd,
    #[serde(rename = "IBNC")]
    Ibnc,
    #[serde(rename = "MCAA")]
    Mcaa,
    #[serde(rename = "MCAN")]
    Mcan,
    #[serde(rename = "MCIC")]
    Mcic,
    #[serde(rename = "MCIN")]
    Mcin,
    #[serde(rename = "MSAA")]
    Msaa,
    #[serde(rename = "MSBN")]
    Msbn,
    #[serde(rename = "MCAD")]
    Mcad,
    #[serde(rename = "NODC")]
    Nodc,
    #[serde(rename = "SCAC")]
    Scac,
    #[serde(rename = "SCAA")]
    Scaa,
    #[serde(rename = "OMSA")]
    Omsa,
    #[serde(rename = "NOCC")]
    Nocc,
    #[serde(rename = "MSBS")]
    Msbs,
    #[serde(rename = "MSAN")]
    Msan,
    #[serde(rename = "SCAN")]
    Scan,
    #[serde(rename = "SCIC")]
    Scic,
    #[serde(rename = "SCIN")]
    Scin,
    #[serde(rename = "SOCA")]
    Soca,
    #[serde(rename = "SSCA")]
    Ssca,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Confirmation1 {
    #[serde(rename = "ConfSts")]
    pub conf_sts: TradeConfirmationStatus1Code,
    #[serde(rename = "ConfTm", skip_serializing_if = "Option::is_none")]
    pub conf_tm: Option<IsoDateTime>,
    #[serde(rename = "TradPtyConfTm", skip_serializing_if = "Option::is_none")]
    pub trad_pty_conf_tm: Option<IsoDateTime>,
    #[serde(rename = "InitgPtyConfTm", skip_serializing_if = "Option::is_none")]
    pub initg_pty_conf_tm: Option<IsoDateTime>,
    #[serde(rename = "ConfTp")]
    pub conf_tp: ConfirmationRequest1Code,
    #[validate]
    #[serde(rename = "ReqId")]
    pub req_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "QryStartNb")]
    pub qry_start_nb: Max35NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfRpts")]
    pub ttl_nb_of_rpts: Number,
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max35NumericText,
    #[validate]
    #[serde(rename = "QryPgNb")]
    pub qry_pg_nb: Max35NumericText,
    #[validate]
    #[serde(rename = "MsgNbOfCurPg")]
    pub msg_nb_of_cur_pg: Number,
    #[validate]
    #[serde(rename = "ListOrdrNb")]
    pub list_ordr_nb: Number,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "LastRptReqd")]
    pub last_rpt_reqd: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuroclearClearstreamIdentifier {
    #[validate(length(min = 1, max = 12,))]
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingMethodType1Code {
    #[serde(rename = "BITR")]
    Bitr,
    #[serde(rename = "CERB")]
    Cerb,
    #[serde(rename = "CUMA")]
    Cuma,
    #[serde(rename = "LIOR")]
    Lior,
    #[serde(rename = "NETR")]
    Netr,
    #[serde(rename = "ONCT")]
    Onct,
    #[serde(rename = "QUAU")]
    Quau,
    #[serde(rename = "TEAU")]
    Teau,
    #[serde(rename = "ANCL")]
    Ancl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalReferences {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "FXTradConfStsAdvc")]
    pub fx_trad_conf_sts_advc: ForeignExchangeTradeConfirmationStatusAdviceV01<A>,
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
pub enum PartyType3Code {
    #[serde(rename = "OPOI")]
    Opoi,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[default]
    Unknown,
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
pub struct PartyIdentification44 {
    #[validate]
    #[serde(rename = "AnyBIC")]
    pub any_bic: AnyBicIdentifier,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Side1Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
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
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingProductIdentifier1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "NDFO")]
    Ndfo,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Header23 {
    #[validate]
    #[serde(rename = "FrmtVrsn")]
    pub frmt_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max3NumericText,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification32,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification32>,
    #[validate]
    #[serde(rename = "MsgSeqNb")]
    pub msg_seq_nb: Number,
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
pub struct TradePartyIdentification7 {
    #[serde(rename = "FndInf", skip_serializing_if = "Option::is_none")]
    pub fnd_inf: Option<FundIdentification3>,
    #[serde(rename = "BuyrOrSellrInd")]
    pub buyr_or_sellr_ind: OptionParty1Code,
    #[serde(rename = "InitrInd")]
    pub initr_ind: OptionParty3Code,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: PartyIdentification78,
    #[validate]
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: PartyIdentificationAndAccount119,
}
#[derive(
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
pub struct PartyIdentification19ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress8>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification19Choice {
    #[serde(flatten)]
    pub value: PartyIdentification19ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyIdentificationType1Code {
    #[serde(rename = "FXID")]
    Fxid,
    #[serde(rename = "FXSN")]
    Fxsn,
    #[serde(rename = "INGN")]
    Ingn,
    #[serde(rename = "IICS")]
    Iics,
    #[serde(rename = "IGBT")]
    Igbt,
    #[serde(rename = "MAMA")]
    Mama,
    #[serde(rename = "MEOC")]
    Meoc,
    #[serde(rename = "METY")]
    Mety,
    #[serde(rename = "NOMM")]
    Nomm,
    #[serde(rename = "OSCO")]
    Osco,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "PONU")]
    Ponu,
    #[serde(rename = "POAD")]
    Poad,
    #[serde(rename = "RMID")]
    Rmid,
    #[serde(rename = "SLCN")]
    Slcn,
    #[serde(rename = "SLNF")]
    Slnf,
    #[serde(rename = "TACN")]
    Tacn,
    #[serde(rename = "TRCO")]
    Trco,
    #[serde(rename = "TANA")]
    Tana,
    #[serde(rename = "USIT")]
    Usit,
    #[serde(rename = "USNA")]
    Usna,
    #[serde(rename = "AUIT")]
    Auit,
    #[serde(rename = "BRID")]
    Brid,
    #[serde(rename = "CLIN")]
    Clin,
    #[serde(rename = "CMID")]
    Cmid,
    #[serde(rename = "COIN")]
    Coin,
    #[serde(rename = "CMOT")]
    Cmot,
    #[serde(rename = "CONU")]
    Conu,
    #[serde(rename = "CMIN")]
    Cmin,
    #[serde(rename = "DECN")]
    Decn,
    #[serde(rename = "DEPA")]
    Depa,
    #[serde(rename = "ELCO")]
    Elco,
    #[serde(rename = "EXVE")]
    Exve,
    #[serde(rename = "FICO")]
    Fico,
    #[serde(rename = "FIID")]
    Fiid,
    #[serde(rename = "FLCN")]
    Flcn,
    #[serde(rename = "FLNF")]
    Flnf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress8 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeConfirmationStatus1Code {
    #[serde(rename = "ALST")]
    Alst,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "EMCN")]
    Emcn,
    #[serde(rename = "MISM")]
    Mism,
    #[serde(rename = "SCCN")]
    Sccn,
    #[serde(rename = "SNCC")]
    Sncc,
    #[serde(rename = "SNCN")]
    Sncn,
    #[serde(rename = "UNCN")]
    Uncn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConfirmationRequest1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "CNRR")]
    Cnrr,
    #[serde(rename = "STAT")]
    Stat,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Trade2 {
    #[validate]
    #[serde(rename = "TradId")]
    pub trad_id: Max35Text,
    #[validate]
    #[serde(rename = "TradDt")]
    pub trad_dt: IsoDate,
    #[serde(rename = "FXTradPdct")]
    pub fx_trad_pdct: UnderlyingProductIdentifier1Code,
    #[serde(rename = "TradgCcy", skip_serializing_if = "Option::is_none")]
    pub tradg_ccy: Option<CurrencyCode>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<CurrencyCode>,
    #[serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none")]
    pub tradg_mtd: Option<TradingMethodType1Code>,
    #[serde(rename = "TradgMd")]
    pub tradg_md: TradingModeType1Code,
    #[serde(rename = "ClrMtd")]
    pub clr_mtd: ClearingMethod1Code,
    #[serde(rename = "Symb", skip_serializing_if = "Option::is_none")]
    pub symb: Option<Max35Text>,
    #[serde(rename = "PlcOfConf", skip_serializing_if = "Option::is_none")]
    pub plc_of_conf: Option<AnyBicIdentifier>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<Trade3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SwpLeg", default)]
    pub swp_leg: Vec<InstrumentLeg6>,
    #[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
    pub pdct_id: Option<SecurityIdentification22Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdTradRef", default)]
    pub assoctd_trad_ref: Vec<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingMethod1Code {
    #[serde(rename = "GRNE")]
    Grne,
    #[serde(rename = "NEMA")]
    Nema,
    #[serde(rename = "NENE")]
    Nene,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType4Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "TAXH")]
    Taxh,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTradeConfirmationStatusAdviceV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header23,
    #[serde(rename = "AdvcId", skip_serializing_if = "Option::is_none")]
    pub advc_id: Option<MessageIdentification1>,
    #[validate]
    #[serde(rename = "TradgSdId")]
    pub tradg_sd_id: TradePartyIdentification7,
    #[validate]
    #[serde(rename = "CtrPtySdId")]
    pub ctr_pty_sd_id: TradePartyIdentification7,
    #[validate]
    #[serde(rename = "TradDtl")]
    pub trad_dtl: Trade2,
    #[validate]
    #[serde(rename = "ConfInf")]
    pub conf_inf: Confirmation1,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<AdditionalReferences>,
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
pub struct GenericIdentification32 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType3Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
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
pub struct PartyIdentification90 {
    #[serde(rename = "IdTp")]
    pub id_tp: PartyIdentificationType1Code,
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
pub struct ConsolidatedTapeAssociationIdentifier {
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
pub struct SimpleIdentificationInformation4 {
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
pub struct PartyIdentificationAndAccount119 {
    #[validate(length(min = 1,))]
    #[serde(rename = "PtyId", default)]
    pub pty_id: Vec<PartyIdentification90>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AcctId", default)]
    pub acct_id: Vec<AccountIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternateIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification18 {
    #[serde(rename = "SctyIdSrc")]
    pub scty_id_src: IdentificationType2Code,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundIdentification3 {
    #[validate]
    #[serde(rename = "FndId")]
    pub fnd_id: Max35Text,
    #[serde(rename = "AcctIdWthCtdn", skip_serializing_if = "Option::is_none")]
    pub acct_id_wth_ctdn: Option<Max35Text>,
    #[serde(rename = "CtdnId", skip_serializing_if = "Option::is_none")]
    pub ctdn_id: Option<PartyIdentification19Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
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
pub struct Trade3 {
    #[validate]
    #[serde(rename = "ExctnPric")]
    pub exctn_pric: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "LastQty")]
    pub last_qty: CurrencyAndAmount,
    #[serde(rename = "SttlmTp")]
    pub sttlm_tp: SettlementDateCode,
    #[validate]
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: IsoDate,
    #[validate]
    #[serde(rename = "ValtnRate")]
    pub valtn_rate: AgreedRate3,
    #[serde(rename = "FwdPts", skip_serializing_if = "Option::is_none")]
    pub fwd_pts: Option<DecimalNumber>,
    #[validate]
    #[serde(rename = "ClctdCtrPtyCcyLastQty")]
    pub clctd_ctr_pty_ccy_last_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "ValDt")]
    pub val_dt: IsoDate,
    #[validate]
    #[serde(rename = "RskAmt")]
    pub rsk_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification18,
    #[serde(rename = "FxgCcy", skip_serializing_if = "Option::is_none")]
    pub fxg_ccy: Option<CurrencyCode>,
    #[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
    pub fxg_dt: Option<IsoDate>,
    #[serde(rename = "OptnInd", skip_serializing_if = "Option::is_none")]
    pub optn_ind: Option<YesNoIndicator>,
    #[serde(rename = "DltaInd", skip_serializing_if = "Option::is_none")]
    pub dlta_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdTradRef", default)]
    pub assoctd_trad_ref: Vec<Max70Text>,
}
#[derive(
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
pub struct CurrencyAndAmountSimpleType {
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
pub struct RicIdentifier {
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
pub struct AccountIdentification30 {
    #[serde(rename = "AcctTp")]
    pub acct_tp: AccountInformationType1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AccountIdentification26,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDateCode {
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
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "ENDC")]
    Endc,
    #[serde(rename = "WHIF")]
    Whif,
    #[serde(rename = "WDIS")]
    Wdis,
    #[serde(rename = "WHID")]
    Whid,
    #[serde(rename = "TBAT")]
    Tbat,
    #[serde(rename = "MONT")]
    Mont,
    #[serde(rename = "CLEA")]
    Clea,
    #[serde(rename = "SAVE")]
    Save,
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
pub struct PartyIdentification78 {
    #[serde(rename = "PtySrc", skip_serializing_if = "Option::is_none")]
    pub pty_src: Option<IdentificationType1Code>,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: Max35Text,
}
#[derive(
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
pub struct AccountIdentification26 {
    #[validate]
    #[serde(rename = "Prtry")]
    pub prtry: SimpleIdentificationInformation4,
}
