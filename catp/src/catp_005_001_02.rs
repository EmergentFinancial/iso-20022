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
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:catp.005.001.02".to_string()
}

#[derive(
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
    #[serde(rename = "ATMRjct")]
    pub atm_rjct: AtmRejectV02,
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
pub struct AtmRejectV02 {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header33,
    #[validate]
    #[serde(rename = "ATMRjct")]
    pub atm_rjct: AtmReject2,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmStatus1Code {
    #[serde(rename = "INSV")]
    Insv,
    #[serde(rename = "OUTS")]
    Outs,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min2Max3AlphaText {
    #[validate(regex = "MIN_2_MAX_3_ALPHA_TEXT_REGEX")]
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
pub struct GenericIdentification77 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: PartyType12Code,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType12Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Min2Max3AlphaText>,
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
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectReason1Code {
    #[serde(rename = "UNPR")]
    Unpr,
    #[serde(rename = "IMSG")]
    Imsg,
    #[serde(rename = "PARS")]
    Pars,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "INTP")]
    Intp,
    #[serde(rename = "RCPP")]
    Rcpp,
    #[serde(rename = "DPMG")]
    Dpmg,
    #[serde(rename = "VERS")]
    Vers,
    #[serde(rename = "MSGT")]
    Msgt,
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction11Code {
    #[serde(rename = "BALN")]
    Baln,
    #[serde(rename = "CMPA")]
    Cmpa,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "ACMD")]
    Acmd,
    #[serde(rename = "DVCC")]
    Dvcc,
    #[serde(rename = "DIAQ")]
    Diaq,
    #[serde(rename = "DIAP")]
    Diap,
    #[serde(rename = "GSTS")]
    Gsts,
    #[serde(rename = "INQQ")]
    Inqq,
    #[serde(rename = "INQP")]
    Inqp,
    #[serde(rename = "KYAQ")]
    Kyaq,
    #[serde(rename = "KYAP")]
    Kyap,
    #[serde(rename = "PINQ")]
    Pinq,
    #[serde(rename = "PINP")]
    Pinp,
    #[serde(rename = "RJAQ")]
    Rjaq,
    #[serde(rename = "RJAP")]
    Rjap,
    #[serde(rename = "WITV")]
    Witv,
    #[serde(rename = "WITK")]
    Witk,
    #[serde(rename = "WITQ")]
    Witq,
    #[serde(rename = "WITP")]
    Witp,
    #[serde(rename = "INQC")]
    Inqc,
    #[serde(rename = "H2AP")]
    H2Ap,
    #[serde(rename = "H2AQ")]
    H2Aq,
    #[serde(rename = "TMOP")]
    Tmop,
    #[serde(rename = "CSEC")]
    Csec,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "SKSC")]
    Sksc,
    #[serde(rename = "SSTS")]
    Ssts,
    #[serde(rename = "DPSK")]
    Dpsk,
    #[serde(rename = "DPSV")]
    Dpsv,
    #[serde(rename = "DPSQ")]
    Dpsq,
    #[serde(rename = "DPSP")]
    Dpsp,
    #[serde(rename = "EXPK")]
    Expk,
    #[serde(rename = "EXPV")]
    Expv,
    #[serde(rename = "TRFQ")]
    Trfq,
    #[serde(rename = "TRFP")]
    Trfp,
    #[serde(rename = "RPTC")]
    Rptc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TmsContactLevel2Code {
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "CRIT")]
    Crit,
    #[serde(rename = "DTIM")]
    Dtim,
    #[serde(rename = "ENCS")]
    Encs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType12Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ATMG")]
    Atmg,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "HSTG")]
    Hstg,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "OATM")]
    Oatm,
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
pub struct AtmCommand7 {
    #[serde(rename = "Tp")]
    pub tp: AtmCommand4Code,
    #[serde(rename = "Urgcy")]
    pub urgcy: TmsContactLevel2Code,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "CmdId", skip_serializing_if = "Option::is_none")]
    pub cmd_id: Option<AtmCommandIdentification1>,
    #[serde(rename = "CmdParams", skip_serializing_if = "Option::is_none")]
    pub cmd_params: Option<AtmCommandParameters1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmCommandIdentification1 {
    #[serde(rename = "Orgn", skip_serializing_if = "Option::is_none")]
    pub orgn: Option<Max35Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "Prcr", skip_serializing_if = "Option::is_none")]
    pub prcr: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Header33 {
    #[validate]
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: AtmMessageFunction2,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max6Text,
    #[serde(rename = "XchgId", skip_serializing_if = "Option::is_none")]
    pub xchg_id: Option<Max3NumericText>,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<Max35Text>,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<Max35Text>,
    #[serde(rename = "PrcStat", skip_serializing_if = "Option::is_none")]
    pub prc_stat: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataSetCategory7Code {
    #[serde(rename = "ATMC")]
    Atmc,
    #[serde(rename = "ATMP")]
    Atmp,
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "CRAP")]
    Crap,
    #[serde(rename = "CPRC")]
    Cprc,
    #[serde(rename = "OEXR")]
    Oexr,
    #[serde(rename = "AMNT")]
    Amnt,
    #[serde(rename = "LOCC")]
    Locc,
    #[serde(rename = "MNOC")]
    Mnoc,
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
pub struct AtmConfigurationParameter1 {
    #[serde(rename = "Tp")]
    pub tp: DataSetCategory7Code,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmReject2 {
    #[serde(rename = "RjctInitrId", skip_serializing_if = "Option::is_none")]
    pub rjct_initr_id: Option<Max35Text>,
    #[serde(rename = "RjctRsn")]
    pub rjct_rsn: RejectReason1Code,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmd", default)]
    pub cmd: Vec<AtmCommand7>,
    #[serde(rename = "MsgInErr", skip_serializing_if = "Option::is_none")]
    pub msg_in_err: Option<Max100KBinary>,
}
#[derive(
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
pub struct AtmCommandParameters1ChoiceEnum {
    #[serde(rename = "ReqrdCfgtnParam", skip_serializing_if = "Option::is_none")]
    pub reqrd_cfgtn_param: Option<AtmConfigurationParameter1>,
    #[serde(rename = "ATMReqrdGblSts", skip_serializing_if = "Option::is_none")]
    pub atm_reqrd_gbl_sts: Option<AtmStatus1Code>,
    #[serde(rename = "XpctdMsgFctn", skip_serializing_if = "Option::is_none")]
    pub xpctd_msg_fctn: Option<MessageFunction8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmCommandParameters1Choice {
    #[serde(flatten)]
    pub value: AtmCommandParameters1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction8Code {
    #[serde(rename = "BALN")]
    Baln,
    #[serde(rename = "GSTS")]
    Gsts,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "INQC")]
    Inqc,
    #[serde(rename = "KEYQ")]
    Keyq,
    #[serde(rename = "SSTS")]
    Ssts,
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
pub struct Traceability4 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification77,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TracDtTmIn")]
    pub trac_dt_tm_in: IsoDateTime,
    #[validate]
    #[serde(rename = "TracDtTmOut")]
    pub trac_dt_tm_out: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmMessageFunction2 {
    #[serde(rename = "Fctn")]
    pub fctn: MessageFunction11Code,
    #[serde(rename = "ATMSvcCd", skip_serializing_if = "Option::is_none")]
    pub atm_svc_cd: Option<Max35Text>,
    #[serde(rename = "HstSvcCd", skip_serializing_if = "Option::is_none")]
    pub hst_svc_cd: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmCommand4Code {
    #[serde(rename = "ABAL")]
    Abal,
    #[serde(rename = "ASTS")]
    Asts,
    #[serde(rename = "CFGT")]
    Cfgt,
    #[serde(rename = "CCNT")]
    Ccnt,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "SNDM")]
    Sndm,
    #[serde(rename = "RPTC")]
    Rptc,
    #[default]
    Unknown,
}
