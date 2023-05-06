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
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_2021_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.016.001.02".to_string()
}

#[derive(
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
pub struct SecuritiesTransactionPrice4ChoiceEnum {
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<SecuritiesTransactionPrice2Choice>,
    #[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
    pub no_pric: Option<SecuritiesTransactionPrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice4Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason15Code {
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "CSUB")]
    Csub,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeInstrument9 {
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "PricMltplr")]
    pub pric_mltplr: NonNegativeDecimalNumber,
    #[serde(rename = "UndrlygInstrm")]
    pub undrlyg_instrm: UnderlyingIdentification3Choice,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType2Code>,
    #[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
    pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
    #[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
    pub optn_exrc_style: Option<OptionStyle7Code>,
    #[serde(rename = "DlvryTp")]
    pub dlvry_tp: PhysicalTransferType4Code,
    #[serde(
        rename = "AsstClssSpcfcAttrbts",
        skip_serializing_if = "Option::is_none"
    )]
    pub asst_clss_spcfc_attrbts: Option<AssetClassAttributes1Choice>,
}
#[derive(
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
pub struct UnderlyingIdentification3ChoiceEnum {
    #[serde(rename = "Swp", skip_serializing_if = "Option::is_none")]
    pub swp: Option<SwapLegIdentification3>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<FinancialInstrumentIdentification8Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnderlyingIdentification3Choice {
    #[serde(flatten)]
    pub value: UnderlyingIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonOrOrganisation1ChoiceEnum {
    #[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
    pub mic: Option<MicIdentifier>,
    #[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
    pub prsn: Option<PersonIdentification10>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Intl", skip_serializing_if = "Option::is_none")]
    pub intl: Option<InternalPartyRole1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonOrOrganisation1Choice {
    #[serde(flatten)]
    pub value: PersonOrOrganisation1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassAttributes1 {
    #[validate]
    #[serde(rename = "Intrst")]
    pub intrst: DerivativeInterest2,
    #[validate]
    #[serde(rename = "FX")]
    pub fx: DerivativeForeignExchange2,
}
#[derive(
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
pub struct FinancialInstrumentQuantity25ChoiceEnum {
    #[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
    pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
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
pub struct FinancialInstrumentQuantity25Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity25ChoiceEnum,
}
#[derive(
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
pub struct SecuritiesTransactionReport2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max52Text,
    #[validate]
    #[serde(rename = "ExctgPty")]
    pub exctg_pty: LeiIdentifier,
    #[validate]
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: LeiIdentifier,
    #[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
    pub tech_attrbts: Option<RecordTechnicalData2>,
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
pub struct FinancialInstrument98 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<Isin2021Identifier>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: FloatingInterestRate8,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentAttributes4ChoiceEnum {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Isin2021Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<SecurityInstrumentDescription19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentAttributes4Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentAttributes4ChoiceEnum,
}
#[derive(
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RegulatoryTradingCapacity1Code {
    #[serde(rename = "MTCH")]
    Mtch,
    #[serde(rename = "DEAL")]
    Deal,
    #[serde(rename = "AOTC")]
    Aotc,
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
pub struct DebtInstrument4 {
    #[validate]
    #[serde(rename = "MtrtyDt")]
    pub mtrty_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPersonIdentification1Code {
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
pub struct FinancialInstrumentIdentification8ChoiceEnum {
    #[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
    pub sngl: Option<FinancialInstrumentIdentification9Choice>,
    #[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
    pub bskt: Option<BasketDescription4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentIdentification8Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentIdentification8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeInterest2 {
    #[serde(rename = "OthrNtnlCcy")]
    pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportingWaiverType1Code {
    #[serde(rename = "OILQ")]
    Oilq,
    #[serde(rename = "NLIQ")]
    Nliq,
    #[serde(rename = "PRIC")]
    Pric,
    #[serde(rename = "ILQD")]
    Ilqd,
    #[serde(rename = "RFPT")]
    Rfpt,
    #[serde(rename = "SIZE")]
    Size,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonNegativeDecimalNumber {
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
pub struct FinancialInstrumentReportingTransactionReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 1,))]
    #[serde(rename = "Tx", default)]
    pub tx: Vec<ReportingTransactionType2Choice<A, B, C>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<D>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransaction1 {
    #[validate]
    #[serde(rename = "TradDt")]
    pub trad_dt: IsoDateTime,
    #[serde(rename = "TradgCpcty")]
    pub tradg_cpcty: RegulatoryTradingCapacity1Code,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity25Choice,
    #[serde(rename = "DerivNtnlChng", skip_serializing_if = "Option::is_none")]
    pub deriv_ntnl_chng: Option<VariationType1Code>,
    #[serde(rename = "Pric")]
    pub pric: SecuritiesTransactionPrice4Choice,
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate]
    #[serde(rename = "TradVn")]
    pub trad_vn: MicIdentifier,
    #[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
    pub ctry_of_brnch: Option<CountryCode>,
    #[serde(rename = "UpFrntPmt", skip_serializing_if = "Option::is_none")]
    pub up_frnt_pmt: Option<AmountAndDirection53>,
    #[serde(rename = "TradPlcMtchgId", skip_serializing_if = "Option::is_none")]
    pub trad_plc_mtchg_id: Option<Max52Text>,
    #[serde(rename = "CmplxTradCmpntId", skip_serializing_if = "Option::is_none")]
    pub cmplx_trad_cmpnt_id: Option<Max35Text>,
}
#[derive(
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
pub struct RecordTechnicalData5 {
    #[validate]
    #[serde(rename = "RctDtTm")]
    pub rct_dt_tm: IsoDateTime,
    #[validate(length(min = 1,))]
    #[serde(rename = "XchgRsn", default)]
    pub xchg_rsn: Vec<ExternalAuthorityExchangeReason1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
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
pub struct SecuritiesTransactionReport6<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max52Text,
    #[validate]
    #[serde(rename = "ExctgPty")]
    pub exctg_pty: LeiIdentifier,
    #[validate]
    #[serde(rename = "InvstmtPtyInd")]
    pub invstmt_pty_ind: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: LeiIdentifier,
    #[validate]
    #[serde(rename = "Buyr")]
    pub buyr: PartyIdentification79,
    #[validate]
    #[serde(rename = "Sellr")]
    pub sellr: PartyIdentification79,
    #[validate]
    #[serde(rename = "OrdrTrnsmssn")]
    pub ordr_trnsmssn: SecuritiesTransactionTransmission2,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: SecuritiesTransaction1,
    #[serde(rename = "FinInstrm")]
    pub fin_instrm: FinancialInstrumentAttributes4Choice,
    #[serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none")]
    pub invstmt_dcsn_prsn: Option<InvestmentParty1Choice>,
    #[serde(rename = "ExctgPrsn")]
    pub exctg_prsn: ExecutingParty1Choice,
    #[validate]
    #[serde(rename = "AddtlAttrbts")]
    pub addtl_attrbts: SecuritiesTransactionIndicator2,
    #[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
    pub tech_attrbts: Option<RecordTechnicalData5>,
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
pub struct SecuritiesTransactionPrice1 {
    #[serde(rename = "Pdg")]
    pub pdg: PriceStatus1Code,
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
pub struct SecuritiesTransactionTransmission2 {
    #[validate]
    #[serde(rename = "TrnsmssnInd")]
    pub trnsmssn_ind: TrueFalseIndicator,
    #[serde(rename = "TrnsmttgBuyr", skip_serializing_if = "Option::is_none")]
    pub trnsmttg_buyr: Option<LeiIdentifier>,
    #[serde(rename = "TrnsmttgSellr", skip_serializing_if = "Option::is_none")]
    pub trnsmttg_sellr: Option<LeiIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Isin2021Identifier {
    #[validate(regex = "ISIN_2021_IDENTIFIER_REGEX")]
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
pub struct SecurityInstrumentDescription19 {
    #[validate]
    #[serde(rename = "FinInstrmGnlAttrbts")]
    pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription20,
    #[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub debt_instrm_attrbts: Option<DebtInstrument4>,
    #[validate]
    #[serde(rename = "DerivInstrmAttrbts")]
    pub deriv_instrm_attrbts: DerivativeInstrument9,
}
#[derive(
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
pub struct DerivativeForeignExchange2 {
    #[serde(rename = "OthrNtnlCcy")]
    pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
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
pub struct SecurityInstrumentDescription20 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Isin2021Identifier>,
    #[validate]
    #[serde(rename = "FullNm")]
    pub full_nm: Max350Text,
    #[validate]
    #[serde(rename = "ClssfctnTp")]
    pub clssfctn_tp: CfiOct2015Identifier,
    #[serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none")]
    pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification76 {
    #[serde(rename = "Id")]
    pub id: PersonOrOrganisation1Choice,
    #[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
    pub ctry_of_brnch: Option<CountryCode>,
}
#[derive(
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
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FinInstrmRptgTxRpt")]
    pub fin_instrm_rptg_tx_rpt: FinancialInstrumentReportingTransactionReportV02<A, B, C, D>,
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
pub struct SecuritiesTransactionIndicator2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "WvrInd", default)]
    pub wvr_ind: Vec<ReportingWaiverType1Code>,
    #[serde(rename = "ShrtSellgInd", skip_serializing_if = "Option::is_none")]
    pub shrt_sellg_ind: Option<Side5Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OTCPstTradInd", default)]
    pub otc_pst_trad_ind: Vec<ReportingWaiverType3Code>,
    #[serde(rename = "RskRdcgTx", skip_serializing_if = "Option::is_none")]
    pub rsk_rdcg_tx: Option<TrueFalseIndicator>,
    #[validate]
    #[serde(rename = "SctiesFincgTxInd")]
    pub scties_fincg_tx_ind: TrueFalseIndicator,
}
#[derive(
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
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle7Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "BERM")]
    Berm,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
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
pub struct SwapLegIdentification3 {
    #[serde(rename = "SwpIn", skip_serializing_if = "Option::is_none")]
    pub swp_in: Option<FinancialInstrumentIdentification8Choice>,
    #[serde(rename = "SwpOut", skip_serializing_if = "Option::is_none")]
    pub swp_out: Option<FinancialInstrumentIdentification8Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPersonIdentification1Code>,
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
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: PersonIdentificationSchemeName1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VariationType1Code {
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "INCR")]
    Incr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification79 {
    #[validate(length(min = 1,))]
    #[serde(rename = "AcctOwnr", default)]
    pub acct_ownr: Vec<PartyIdentification76>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DcsnMakr", default)]
    pub dcsn_makr: Vec<PersonOrOrganisation2Choice>,
}
#[derive(
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
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InternalPartyRole1Code {
    #[serde(rename = "INTC")]
    Intc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExecutingParty1ChoiceEnum {
    #[serde(rename = "Clnt", skip_serializing_if = "Option::is_none")]
    pub clnt: Option<NoReasonCode>,
    #[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
    pub algo: Option<Max50Text>,
    #[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
    pub prsn: Option<PersonIdentification12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExecutingParty1Choice {
    #[serde(flatten)]
    pub value: ExecutingParty1ChoiceEnum,
}
#[derive(
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
pub struct ExternalAuthorityExchangeReason1Code {
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
pub struct GenericPersonIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
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
pub struct AssetClassAttributes1ChoiceEnum {
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<DerivativeInterest2>,
    #[serde(rename = "Both", skip_serializing_if = "Option::is_none")]
    pub both: Option<AssetClassAttributes1>,
    #[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
    pub fx: Option<DerivativeForeignExchange2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassAttributes1Choice {
    #[serde(flatten)]
    pub value: AssetClassAttributes1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentification10 {
    #[validate]
    #[serde(rename = "FrstNm")]
    pub frst_nm: Max140Text,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[validate]
    #[serde(rename = "BirthDt")]
    pub birth_dt: IsoDate,
    #[validate]
    #[serde(rename = "Othr")]
    pub othr: GenericPersonIdentification1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RecordTechnicalData2 {
    #[validate]
    #[serde(rename = "RctDtTm")]
    pub rct_dt_tm: IsoDateTime,
    #[serde(rename = "CxlRsn")]
    pub cxl_rsn: CancelledStatusReason15Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportingWaiverType3Code {
    #[serde(rename = "BENC")]
    Benc,
    #[serde(rename = "ACTX")]
    Actx,
    #[serde(rename = "ILQD")]
    Ilqd,
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "AMND")]
    Amnd,
    #[serde(rename = "SDIV")]
    Sdiv,
    #[serde(rename = "RPRI")]
    Rpri,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "LRGS")]
    Lrgs,
    #[serde(rename = "TNCP")]
    Tncp,
    #[serde(rename = "TPAC")]
    Tpac,
    #[serde(rename = "XFPH")]
    Xfph,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Side5Code {
    #[serde(rename = "SESH")]
    Sesh,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "SSEX")]
    Ssex,
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
pub struct FinancialInstrumentIdentification9ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<FinancialInstrument98>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<Isin2021Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentIdentification9Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentIdentification9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection61 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
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
pub struct InvestmentParty1ChoiceEnum {
    #[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
    pub prsn: Option<PersonIdentification12>,
    #[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
    pub algo: Option<Max50Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentParty1Choice {
    #[serde(flatten)]
    pub value: InvestmentParty1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice2ChoiceEnum {
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection61>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
    pub bsis_pts: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice2Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice2ChoiceEnum,
}
#[derive(
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
pub struct PersonIdentification12 {
    #[serde(rename = "CtryOfBrnch")]
    pub ctry_of_brnch: CountryCode,
    #[validate]
    #[serde(rename = "Othr")]
    pub othr: GenericPersonIdentification1,
}
#[derive(
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
pub struct BasketDescription4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ISIN", default)]
    pub isin: Vec<Isin2021Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Indx", default)]
    pub indx: Vec<FinancialInstrument98>,
}
#[derive(
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
pub struct AmountAndDirection53 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
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
pub struct PersonOrOrganisation2ChoiceEnum {
    #[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
    pub prsn: Option<PersonIdentification10>,
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
pub struct PersonOrOrganisation2Choice {
    #[serde(flatten)]
    pub value: PersonOrOrganisation2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReportingTransactionType2ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
    pub cxl: Option<SecuritiesTransactionReport2<A>>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<SupplementaryData1<B>>,
    #[serde(rename = "New", skip_serializing_if = "Option::is_none")]
    pub new: Option<SecuritiesTransactionReport6<C>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReportingTransactionType2Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: ReportingTransactionType2ChoiceEnum<A, B, C>,
}
#[derive(
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
