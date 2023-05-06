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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:colr.004.001.05".to_string()
}

#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "MrgnCallRspn")]
    pub mrgn_call_rspn: MarginCallResponseV05<A>,
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
pub struct Agreement4 {
    #[validate]
    #[serde(rename = "AgrmtDtls")]
    pub agrmt_dtls: Max140Text,
    #[serde(rename = "AgrmtId", skip_serializing_if = "Option::is_none")]
    pub agrmt_id: Option<Max140Text>,
    #[validate]
    #[serde(rename = "AgrmtDt")]
    pub agrmt_dt: IsoDate,
    #[serde(rename = "BaseCcy")]
    pub base_ccy: ActiveCurrencyCode,
    #[serde(rename = "AgrmtFrmwk", skip_serializing_if = "Option::is_none")]
    pub agrmt_frmwk: Option<AgreementFramework1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralAccountType1Code {
    #[serde(rename = "HOUS")]
    Hous,
    #[serde(rename = "CLIE")]
    Clie,
    #[serde(rename = "LIPR")]
    Lipr,
    #[serde(rename = "MGIN")]
    Mgin,
    #[serde(rename = "DFLT")]
    Dflt,
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
pub struct CollateralAccount3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountIdentificationType3Choice>,
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
pub struct PartyIdentification178ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
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
pub struct PartyIdentification178Choice {
    #[serde(flatten)]
    pub value: PartyIdentification178ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Response1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "RspnTpDtls", default)]
    pub rspn_tp_dtls: Vec<ResponseType1Choice>,
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
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureConventionType1Code {
    #[serde(rename = "GROS")]
    Gros,
    #[serde(rename = "NET1")]
    Net1,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RoundingMethod1Code {
    #[serde(rename = "DRDW")]
    Drdw,
    #[serde(rename = "DRUP")]
    Drup,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "CLSR")]
    Clsr,
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
pub struct MarginCallResponseV05<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[validate]
    #[serde(rename = "Oblgtn")]
    pub oblgtn: Obligation9,
    #[serde(rename = "Agrmt", skip_serializing_if = "Option::is_none")]
    pub agrmt: Option<Agreement4>,
    #[serde(rename = "MrgnDtlsDueToA", skip_serializing_if = "Option::is_none")]
    pub mrgn_dtls_due_to_a: Option<MarginCall1>,
    #[serde(rename = "MrgnDtlsDueToB", skip_serializing_if = "Option::is_none")]
    pub mrgn_dtls_due_to_b: Option<MarginCall1>,
    #[serde(rename = "AgrdAmtDueToA", skip_serializing_if = "Option::is_none")]
    pub agrd_amt_due_to_a: Option<AgreedAmount1Choice>,
    #[serde(rename = "AgrdAmtDueToB", skip_serializing_if = "Option::is_none")]
    pub agrd_amt_due_to_b: Option<AgreedAmount1Choice>,
    #[serde(rename = "RspnDtls", skip_serializing_if = "Option::is_none")]
    pub rspn_dtls: Option<Response1>,
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
pub struct MarginCall1 {
    #[serde(rename = "XpsdAmtPtyA", skip_serializing_if = "Option::is_none")]
    pub xpsd_amt_pty_a: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpsdAmtPtyB", skip_serializing_if = "Option::is_none")]
    pub xpsd_amt_pty_b: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpsrCnvntn", skip_serializing_if = "Option::is_none")]
    pub xpsr_cnvntn: Option<ExposureConventionType1Code>,
    #[serde(rename = "IndpdntAmtPtyA", skip_serializing_if = "Option::is_none")]
    pub indpdnt_amt_pty_a: Option<AggregatedIndependentAmount1>,
    #[serde(rename = "IndpdntAmtPtyB", skip_serializing_if = "Option::is_none")]
    pub indpdnt_amt_pty_b: Option<AggregatedIndependentAmount1>,
    #[serde(rename = "MrgnTerms", skip_serializing_if = "Option::is_none")]
    pub mrgn_terms: Option<MarginTerms1Choice>,
    #[serde(rename = "CollBal", skip_serializing_if = "Option::is_none")]
    pub coll_bal: Option<CollateralBalance1Choice>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarginTerms1ChoiceEnum {
    #[serde(
        rename = "SgrtdIndpdntAmtMrgn",
        skip_serializing_if = "Option::is_none"
    )]
    pub sgrtd_indpdnt_amt_mrgn: Option<SegregatedIndependentAmountMargin1>,
    #[serde(rename = "MrgnDtls", skip_serializing_if = "Option::is_none")]
    pub mrgn_dtls: Option<Margin1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarginTerms1Choice {
    #[serde(flatten)]
    pub value: MarginTerms1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType11Code {
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CBCO")]
    Cbco,
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
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXTD")]
    Extd,
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
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "UDMS")]
    Udms,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "SHSL")]
    Shsl,
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
pub struct AgreedAmount1ChoiceEnum {
    #[serde(rename = "AgrdAmtDtls", skip_serializing_if = "Option::is_none")]
    pub agrd_amt_dtls: Option<AgreedAmount1>,
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<Amount1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreedAmount1Choice {
    #[serde(flatten)]
    pub value: AgreedAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndependentAmount1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "Cnvntn")]
    pub cnvntn: IndependentAmountConventionType1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndependentAmount2 {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "Cnvntn")]
    pub cnvntn: IndependentAmountConventionType1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResponseType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarginCallResponse1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResponseType1Choice {
    #[serde(flatten)]
    pub value: ResponseType1ChoiceEnum,
}
#[derive(
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
pub struct Obligation9 {
    #[serde(rename = "PtyA")]
    pub pty_a: PartyIdentification178Choice,
    #[serde(rename = "SvcgPtyA", skip_serializing_if = "Option::is_none")]
    pub svcg_pty_a: Option<PartyIdentification178Choice>,
    #[serde(rename = "PtyB")]
    pub pty_b: PartyIdentification178Choice,
    #[serde(rename = "SvcgPtyB", skip_serializing_if = "Option::is_none")]
    pub svcg_pty_b: Option<PartyIdentification178Choice>,
    #[serde(rename = "CollAcctId", skip_serializing_if = "Option::is_none")]
    pub coll_acct_id: Option<CollateralAccount3>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet5>,
    #[serde(rename = "XpsrTp", skip_serializing_if = "Option::is_none")]
    pub xpsr_tp: Option<ExposureType11Code>,
    #[serde(rename = "ValtnDt")]
    pub valtn_dt: DateAndDateTime2Choice,
}
#[derive(
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarginCallResponse1Code {
    #[serde(rename = "NVDA")]
    Nvda,
    #[serde(rename = "RANT")]
    Rant,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VariationMargin1 {
    #[validate]
    #[serde(rename = "ThrshldAmt")]
    pub thrshld_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "ThrshldTp", skip_serializing_if = "Option::is_none")]
    pub thrshld_tp: Option<ThresholdType1Code>,
    #[validate]
    #[serde(rename = "MinTrfAmt")]
    pub min_trf_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "RndgAmt")]
    pub rndg_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "RndgMtd")]
    pub rndg_mtd: RoundingMethod1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ThresholdType1Code {
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "UNSE")]
    Unse,
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
pub struct AgreedAmount1 {
    #[validate]
    #[serde(rename = "VartnMrgnAmt")]
    pub vartn_mrgn_amt: Amount1,
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<Amount1>,
}
#[derive(
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
pub struct CollateralAccountIdentificationType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralAccountIdentificationType3Choice {
    #[serde(flatten)]
    pub value: CollateralAccountIdentificationType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralBalance1ChoiceEnum {
    #[serde(rename = "CollDtls", skip_serializing_if = "Option::is_none")]
    pub coll_dtls: Option<Collateral1>,
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<MarginCollateral1>,
    #[serde(rename = "TtlColl", skip_serializing_if = "Option::is_none")]
    pub ttl_coll: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralBalance1Choice {
    #[serde(flatten)]
    pub value: CollateralBalance1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Margin1 {
    #[validate]
    #[serde(rename = "VartnMrgn")]
    pub vartn_mrgn: VariationMargin1,
    #[serde(
        rename = "SgrtdIndpdntAmtMrgn",
        skip_serializing_if = "Option::is_none"
    )]
    pub sgrtd_indpdnt_amt_mrgn: Option<SegregatedIndependentAmountMargin1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarginCollateral1 {
    #[serde(rename = "HeldByPtyA", skip_serializing_if = "Option::is_none")]
    pub held_by_pty_a: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "HeldByPtyB", skip_serializing_if = "Option::is_none")]
    pub held_by_pty_b: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrrAgrdToPtyA", skip_serializing_if = "Option::is_none")]
    pub prr_agrd_to_pty_a: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrrAgrdToPtyB", skip_serializing_if = "Option::is_none")]
    pub prr_agrd_to_pty_b: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "InTrnstToPtyA", skip_serializing_if = "Option::is_none")]
    pub in_trnst_to_pty_a: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "InTrnstToPtyB", skip_serializing_if = "Option::is_none")]
    pub in_trnst_to_pty_b: Option<ActiveCurrencyAndAmount>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AggregatedIndependentAmount1 {
    #[serde(rename = "Trad", skip_serializing_if = "Option::is_none")]
    pub trad: Option<IndependentAmount1>,
    #[serde(rename = "ValAtRsk", skip_serializing_if = "Option::is_none")]
    pub val_at_rsk: Option<IndependentAmount1>,
    #[serde(rename = "NetOpnPos", skip_serializing_if = "Option::is_none")]
    pub net_opn_pos: Option<IndependentAmount1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmt", default)]
    pub othr_amt: Vec<IndependentAmount2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AgreementFramework1Code {
    #[serde(rename = "FBAA")]
    Fbaa,
    #[serde(rename = "BBAA")]
    Bbaa,
    #[serde(rename = "DERV")]
    Derv,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "NONR")]
    Nonr,
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
pub enum IndependentAmountConventionType1Code {
    #[serde(rename = "NBTR")]
    Nbtr,
    #[serde(rename = "NATR")]
    Natr,
    #[serde(rename = "SEGR")]
    Segr,
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
pub struct SegregatedIndependentAmountMargin1 {
    #[validate]
    #[serde(rename = "MinTrfAmt")]
    pub min_trf_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "RndgAmt", skip_serializing_if = "Option::is_none")]
    pub rndg_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RndgMtd", skip_serializing_if = "Option::is_none")]
    pub rndg_mtd: Option<RoundingMethod1Code>,
}
#[derive(
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
pub struct AgreementFramework1ChoiceEnum {
    #[serde(rename = "AgrmtFrmwk", skip_serializing_if = "Option::is_none")]
    pub agrmt_frmwk: Option<AgreementFramework1Code>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementFramework1Choice {
    #[serde(flatten)]
    pub value: AgreementFramework1ChoiceEnum,
}
#[derive(
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
pub struct BlockChainAddressWallet5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountIdentificationType3Choice>,
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
pub struct Collateral1 {
    #[validate]
    #[serde(rename = "VartnMrgn")]
    pub vartn_mrgn: MarginCollateral1,
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<MarginCollateral1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Amount1 {
    #[validate]
    #[serde(rename = "AgrdAmt")]
    pub agrd_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "MrgnCallReqId")]
    pub mrgn_call_req_id: Max35Text,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max210Text>,
}
