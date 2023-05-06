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
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UTI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.108.001.01".to_string()
}

#[derive(
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
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
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
pub struct CollateralPortfolioCode5ChoiceEnum {
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioCode3Choice>,
    #[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralPortfolioCode5Choice {
    #[serde(flatten)]
    pub value: CollateralPortfolioCode5ChoiceEnum,
}
#[derive(
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
pub struct OrganisationIdentification15ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OrganisationIdentification38>,
}
#[derive(
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
pub struct Counterparty46 {
    #[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
    pub id_tp: Option<PartyIdentification248Choice>,
    #[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
    pub ntr: Option<CounterpartyTradeNature15Choice>,
    #[serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none")]
    pub rptg_oblgtn: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode5ChoiceEnum {
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioIdentification3>,
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode5Choice {
    #[serde(flatten)]
    pub value: PortfolioCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeCounterpartyRelationshipRecord1 {
    #[serde(rename = "StartRltshPty")]
    pub start_rltsh_pty: TradeCounterpartyType1Code,
    #[serde(rename = "EndRltshPty")]
    pub end_rltsh_pty: TradeCounterpartyType1Code,
    #[serde(rename = "RltshTp")]
    pub rltsh_tp: TradeCounterpartyRelationship1Choice,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max1000Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeCounterpartyType1Code {
    #[serde(rename = "BENE")]
    Bene,
    #[serde(rename = "BROK")]
    Brok,
    #[serde(rename = "CLEM")]
    Clem,
    #[serde(rename = "EXEA")]
    Exea,
    #[serde(rename = "OTHC")]
    Othc,
    #[serde(rename = "REPC")]
    Repc,
    #[serde(rename = "SBMA")]
    Sbma,
    #[serde(rename = "ERFR")]
    Erfr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReport31ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Crrctn", skip_serializing_if = "Option::is_none")]
    pub crrctn: Option<MarginReportData7<A>>,
    #[serde(rename = "MrgnUpd", skip_serializing_if = "Option::is_none")]
    pub mrgn_upd: Option<MarginReportData7<B>>,
    #[serde(rename = "Err", skip_serializing_if = "Option::is_none")]
    pub err: Option<MarginReportData7<C>>,
    #[serde(rename = "New", skip_serializing_if = "Option::is_none")]
    pub new: Option<MarginReportData7<D>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReport31Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: TradeReport31ChoiceEnum<A, B, C, D>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialPartyClassification2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FinancialPartySectorType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialPartyClassification2Choice {
    #[serde(flatten)]
    pub value: FinancialPartyClassification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPartyRelationshipType1Code {
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
pub struct MarginReportData7<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
    pub rptg_tm_stmp: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "CtrPtyId")]
    pub ctr_pty_id: TradeCounterpartyReport20,
    #[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
    pub evt_dt: Option<IsoDate>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
    #[validate]
    #[serde(rename = "Coll")]
    pub coll: MarginCollateralReport4,
    #[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
    pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
    #[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
    pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
    #[serde(rename = "CtrPtyRatgTrggrInd", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_ratg_trggr_ind: Option<TrueFalseIndicator>,
    #[serde(
        rename = "CtrPtyRatgThrshldInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub ctr_pty_ratg_thrshld_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
    pub tech_attrbts: Option<TechnicalAttributes6>,
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
pub struct Pagination1 {
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification248ChoiceEnum {
    #[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
    pub lgl: Option<LegalPersonIdentification1>,
    #[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
    pub ntrl: Option<NaturalPersonIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification248Choice {
    #[serde(flatten)]
    pub value: PartyIdentification248ChoiceEnum,
}
#[derive(
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
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "DerivsTradMrgnDataRpt")]
    pub derivs_trad_mrgn_data_rpt: DerivativesTradeMarginDataReportV01<A, B, C, D, E>,
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
pub struct TradeCounterpartyRelationship1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max100Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPartyRelationshipType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeCounterpartyRelationship1Choice {
    #[serde(flatten)]
    pub value: TradeCounterpartyRelationship1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReportHeader4 {
    #[serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none")]
    pub rpt_exctn_dt: Option<IsoDate>,
    #[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
    pub msg_pgntn: Option<Pagination1>,
    #[validate]
    #[serde(rename = "NbRcrds")]
    pub nb_rcrds: Number,
    #[validate(length(min = 0,))]
    #[serde(rename = "CmptntAuthrty", default)]
    pub cmptnt_authrty: Vec<Max100Text>,
    #[serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none")]
    pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RptgPurp", default)]
    pub rptg_purp: Vec<Max100Text>,
}
#[derive(
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
pub struct TradeData55ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<TradeReport31Choice<A, B, C, D>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeData55Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: TradeData55ChoiceEnum<A, B, C, D>,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralisationType3Code {
    #[serde(rename = "FLCL")]
    Flcl,
    #[serde(rename = "OWCL")]
    Owcl,
    #[serde(rename = "OWC1")]
    Owc1,
    #[serde(rename = "OWC2")]
    Owc2,
    #[serde(rename = "OWP1")]
    Owp1,
    #[serde(rename = "OWP2")]
    Owp2,
    #[serde(rename = "PRCL")]
    Prcl,
    #[serde(rename = "PRC1")]
    Prc1,
    #[serde(rename = "PRC2")]
    Prc2,
    #[serde(rename = "UNCL")]
    Uncl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialPartySectorType3Code {
    #[serde(rename = "AIFD")]
    Aifd,
    #[serde(rename = "CSDS")]
    Csds,
    #[serde(rename = "CCPS")]
    Ccps,
    #[serde(rename = "CDTI")]
    Cdti,
    #[serde(rename = "INUN")]
    Inun,
    #[serde(rename = "ORPI")]
    Orpi,
    #[serde(rename = "INVF")]
    Invf,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "UCIT")]
    Ucit,
    #[serde(rename = "ASSU")]
    Assu,
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
pub struct Direction2 {
    #[serde(rename = "DrctnOfTheFrstLeg")]
    pub drctn_of_the_frst_leg: OptionParty3Code,
    #[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
    pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionSector1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Sctr", default)]
    pub sctr: Vec<FinancialPartyClassification2Choice>,
    #[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
    pub clr_thrshld: Option<TrueFalseIndicator>,
}
#[derive(
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
pub struct PostedMarginOrCollateral6 {
    #[serde(
        rename = "InitlMrgnPstdPreHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "InitlMrgnPstdPstHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "VartnMrgnPstdPreHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "VartnMrgnPstdPstHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
    pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}
#[derive(
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
pub struct MarginCollateralReport4 {
    #[serde(rename = "CollPrtflCd")]
    pub coll_prtfl_cd: CollateralPortfolioCode5Choice,
    #[serde(rename = "CollstnCtgy")]
    pub collstn_ctgy: CollateralisationType3Code,
    #[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
    pub tm_stmp: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max52Text>,
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode3Choice {
    #[serde(flatten)]
    pub value: PortfolioCode3ChoiceEnum,
}
#[derive(
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
pub struct PortfolioIdentification3 {
    #[validate]
    #[serde(rename = "Cd")]
    pub cd: Max52Text,
    #[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
    pub prtfl_tx_xmptn: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TechnicalAttributes6 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max140Text>,
    #[serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none")]
    pub rpt_rct_tm_stmp: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotApplicable1Code {
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity7Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "PRIN")]
    Prin,
    #[default]
    Unknown,
}
#[derive(
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
pub struct UniqueTransactionIdentifier2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<UtiIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UniqueTransactionIdentifier2Choice {
    #[serde(flatten)]
    pub value: UniqueTransactionIdentifier2ChoiceEnum,
}
#[derive(
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
pub struct TradeCounterpartyReport20 {
    #[validate]
    #[serde(rename = "RptgCtrPty")]
    pub rptg_ctr_pty: Counterparty45,
    #[validate]
    #[serde(rename = "OthrCtrPty")]
    pub othr_ctr_pty: Counterparty46,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
    pub submitg_agt: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
    pub clr_mmb: Option<PartyIdentification248Choice>,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "Bnfcry", default)]
    pub bnfcry: Vec<PartyIdentification248Choice>,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "ExctnAgt", default)]
    pub exctn_agt: Vec<OrganisationIdentification15Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltshRcrd", default)]
    pub rltsh_rcrd: Vec<TradeCounterpartyRelationshipRecord1>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Direction4ChoiceEnum {
    #[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_sd: Option<OptionParty1Code>,
    #[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
    pub drctn: Option<Direction2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Direction4Choice {
    #[serde(flatten)]
    pub value: Direction4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativesTradeMarginDataReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: TradeReportHeader4,
    #[serde(rename = "TradData")]
    pub trad_data: TradeData55Choice<A, B, C, D>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<E>>,
}
#[derive(
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
pub struct UtiIdentifier {
    #[validate(regex = "UTI_IDENTIFIER_REGEX")]
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
pub struct NonFinancialInstitutionSector10 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Sctr", default)]
    pub sctr: Vec<GenericIdentification175>,
    #[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
    pub clr_thrshld: Option<TrueFalseIndicator>,
    #[serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none")]
    pub drctly_lkd_actvty: Option<TrueFalseIndicator>,
    #[serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none")]
    pub fdrl_instn: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaturalPersonIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: NaturalPersonIdentification2,
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
pub struct Counterparty45 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification248Choice,
    #[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
    pub ntr: Option<CounterpartyTradeNature15Choice>,
    #[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
    pub tradg_cpcty: Option<TradingCapacity7Code>,
    #[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
    pub drctn_or_sd: Option<Direction4Choice>,
    #[serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none")]
    pub tradr_lctn: Option<CountryCode>,
    #[serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none")]
    pub bookg_lctn: Option<CountryCode>,
    #[serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none")]
    pub rptg_xmptn: Option<ReportingExemption1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReceivedMarginOrCollateral6 {
    #[serde(
        rename = "InitlMrgnRcvdPreHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "InitlMrgnRcvdPstHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "VartnMrgnRcvdPreHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(
        rename = "VartnMrgnRcvdPstHrcut",
        skip_serializing_if = "Option::is_none"
    )]
    pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
    #[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
    pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegalPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification15Choice,
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
pub struct CounterpartyTradeNature15ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<NoReasonCode>,
    #[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
    pub fi: Option<FinancialInstitutionSector1>,
    #[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
    pub nfi: Option<NonFinancialInstitutionSector10>,
    #[serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none")]
    pub cntrl_cntr_pty: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyTradeNature15Choice {
    #[serde(flatten)]
    pub value: CounterpartyTradeNature15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarginPortfolio3 {
    #[serde(rename = "InitlMrgnPrtflCd")]
    pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
    #[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
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
pub struct OrganisationIdentification38 {
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
pub struct ReportingExemption1 {
    #[validate]
    #[serde(rename = "Rsn")]
    pub rsn: Max4Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max1000Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
    #[default]
    Unknown,
}
