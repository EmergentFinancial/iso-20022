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
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_9_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:catm.003.001.11".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm25Code {
    #[serde(rename = "ERS2")]
    Ers2,
    #[serde(rename = "ERS1")]
    Ers1,
    #[serde(rename = "RPSS")]
    Rpss,
    #[serde(rename = "ERS3")]
    Ers3,
    #[serde(rename = "ED32")]
    Ed32,
    #[serde(rename = "ED33")]
    Ed33,
    #[serde(rename = "ED35")]
    Ed35,
    #[serde(rename = "ED23")]
    Ed23,
    #[serde(rename = "ED25")]
    Ed25,
    #[serde(rename = "ES22")]
    Es22,
    #[serde(rename = "ES32")]
    Es32,
    #[serde(rename = "ES33")]
    Es33,
    #[serde(rename = "ES35")]
    Es35,
    #[serde(rename = "ES23")]
    Es23,
    #[serde(rename = "ES25")]
    Es25,
    #[serde(rename = "ED22")]
    Ed22,
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
pub struct KekIdentifier7 {
    #[validate]
    #[serde(rename = "KeyId")]
    pub key_id: Max140Text,
    #[validate]
    #[serde(rename = "KeyVrsn")]
    pub key_vrsn: Max140Text,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "DerivtnId", skip_serializing_if = "Option::is_none")]
    pub derivtn_id: Option<Max500Binary>,
}
#[derive(
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
pub enum PartyType15Code {
    #[serde(rename = "PGRP")]
    Pgrp,
    #[serde(rename = "PSYS")]
    Psys,
    #[serde(rename = "PSNG")]
    Psng,
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
pub struct AlgorithmIdentification30 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm25Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CryptographicKey16 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max350Text,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Binary>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max256Text>,
    #[serde(rename = "SctyPrfl", skip_serializing_if = "Option::is_none")]
    pub scty_prfl: Option<Max35Text>,
    #[serde(rename = "ItmNb", skip_serializing_if = "Option::is_none")]
    pub itm_nb: Option<Max35Text>,
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
    pub key_val: Option<ContentInformationType30>,
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
pub struct ClockSynchronisation3 {
    #[validate]
    #[serde(rename = "POITmZone")]
    pub poi_tm_zone: Max70Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "SynctnSvr", default)]
    pub synctn_svr: Vec<NetworkParameters7>,
    #[serde(rename = "Dely", skip_serializing_if = "Option::is_none")]
    pub dely: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient12ChoiceEnum {
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber2>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient12Choice {
    #[serde(flatten)]
    pub value: Recipient12ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataSetCategory10Code {
    #[serde(rename = "AQPR")]
    Aqpr,
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "MTMG")]
    Mtmg,
    #[serde(rename = "MRPR")]
    Mrpr,
    #[serde(rename = "MTOR")]
    Mtor,
    #[serde(rename = "SCPR")]
    Scpr,
    #[serde(rename = "SWPK")]
    Swpk,
    #[serde(rename = "TRPR")]
    Trpr,
    #[serde(rename = "CRTF")]
    Crtf,
    #[serde(rename = "TMSP")]
    Tmsp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataSetCategory17Code {
    #[serde(rename = "AQPR")]
    Aqpr,
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "TXCP")]
    Txcp,
    #[serde(rename = "AKCP")]
    Akcp,
    #[serde(rename = "DLGT")]
    Dlgt,
    #[serde(rename = "MGTP")]
    Mgtp,
    #[serde(rename = "MRPR")]
    Mrpr,
    #[serde(rename = "SCPR")]
    Scpr,
    #[serde(rename = "SWPK")]
    Swpk,
    #[serde(rename = "STRP")]
    Strp,
    #[serde(rename = "TRPR")]
    Trpr,
    #[serde(rename = "VDPR")]
    Vdpr,
    #[serde(rename = "PARA")]
    Para,
    #[serde(rename = "TMSP")]
    Tmsp,
    #[serde(rename = "CRTF")]
    Crtf,
    #[serde(rename = "LOGF")]
    Logf,
    #[serde(rename = "CMRQ")]
    Cmrq,
    #[serde(rename = "MDFL")]
    Mdfl,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "RPFL")]
    Rpfl,
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
pub struct EnvelopedData9 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "OrgtrInf", skip_serializing_if = "Option::is_none")]
    pub orgtr_inf: Option<OriginatorInformation1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient11Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KeyTransport8 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient12Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification19,
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
pub struct KekIdentifier5 {
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
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CryptographicKeyType3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Fctn", default)]
    pub fctn: Vec<KeyUsage1Code>,
}
#[derive(
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
pub struct MerchantConfigurationParameters6 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[serde(rename = "MrchntId", skip_serializing_if = "Option::is_none")]
    pub mrchnt_id: Option<Max35Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max256Text>,
    #[serde(rename = "ParamFrmtIdr", skip_serializing_if = "Option::is_none")]
    pub param_frmt_idr: Option<Max8Text>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<NetworkParameters8>,
    #[serde(rename = "OthrParamsLngth", skip_serializing_if = "Option::is_none")]
    pub othr_params_lngth: Option<PositiveNumber>,
    #[serde(rename = "OffsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<PositiveNumber>,
    #[serde(rename = "OffsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<PositiveNumber>,
    #[serde(rename = "OthrParams", skip_serializing_if = "Option::is_none")]
    pub othr_params: Option<Max10000Binary>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReconciliationCriteria1Code {
    #[serde(rename = "BRND")]
    Brnd,
    #[serde(rename = "PROF")]
    Prof,
    #[serde(rename = "GRUP")]
    Grup,
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
pub struct ServiceProviderParameters2 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "SvcPrvdrId", default)]
    pub svc_prvdr_id: Vec<GenericIdentification176>,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "ApplId", default)]
    pub appl_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Hst", default)]
    pub hst: Vec<AcquirerHostConfiguration9>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NonFinActnSpprtd", default)]
    pub non_fin_actn_spprtd: Vec<NonFinancialRequestType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcceptorConfiguration11 {
    #[validate]
    #[serde(rename = "TermnlMgrId")]
    pub termnl_mgr_id: GenericIdentification176,
    #[validate(length(min = 0,))]
    #[serde(rename = "POIGrpId", default)]
    pub poi_grp_id: Vec<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "DataSet", default)]
    pub data_set: Vec<AcceptorConfigurationDataSet3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessRetry3 {
    #[validate]
    #[serde(rename = "Dely")]
    pub dely: Max9NumericText,
    #[serde(rename = "MaxNb", skip_serializing_if = "Option::is_none")]
    pub max_nb: Option<Number>,
    #[serde(rename = "UnitOfTm", skip_serializing_if = "Option::is_none")]
    pub unit_of_tm: Option<TimeUnit1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetworkParameters7 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Adr", default)]
    pub adr: Vec<NetworkParameters9>,
    #[serde(rename = "UsrNm", skip_serializing_if = "Option::is_none")]
    pub usr_nm: Option<Max35Text>,
    #[serde(rename = "AccsCd", skip_serializing_if = "Option::is_none")]
    pub accs_cd: Option<Max35Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvrCert", default)]
    pub svr_cert: Vec<Max10KBinary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvrCertIdr", default)]
    pub svr_cert_idr: Vec<Max140Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntCert", default)]
    pub clnt_cert: Vec<Max10KBinary>,
    #[serde(rename = "SctyPrfl", skip_serializing_if = "Option::is_none")]
    pub scty_prfl: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Traceability8 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification177,
    #[serde(rename = "PrtcolNm", skip_serializing_if = "Option::is_none")]
    pub prtcol_nm: Option<Max35Text>,
    #[serde(rename = "PrtcolVrsn", skip_serializing_if = "Option::is_none")]
    pub prtcol_vrsn: Option<Max6Text>,
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
pub struct Max8Text {
    #[validate(length(min = 1, max = 8,))]
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
pub struct TerminalPackageType3 {
    #[validate(length(min = 0,))]
    #[serde(rename = "POICmpntId", default)]
    pub poi_cmpnt_id: Vec<PointOfInteractionComponentIdentification2>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Packg", default)]
    pub packg: Vec<PackageType3>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NonFinancialRequestType1Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "PARQ")]
    Parq,
    #[serde(rename = "RISK")]
    Risk,
    #[serde(rename = "TOKN")]
    Tokn,
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
pub struct PointOfInteractionComponentIdentification2 {
    #[serde(rename = "ItmNb", skip_serializing_if = "Option::is_none")]
    pub itm_nb: Option<Max35Text>,
    #[serde(rename = "PrvdrId", skip_serializing_if = "Option::is_none")]
    pub prvdr_id: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max256Text>,
    #[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
    pub srl_nb: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AuthenticatedData8 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient11Choice>,
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
pub enum FinancialCapture1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "BTCH")]
    Btch,
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
pub struct ApplicationParameters11 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate]
    #[serde(rename = "ApplId")]
    pub appl_id: Max35Text,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max256Text>,
    #[serde(rename = "ParamFrmtIdr", skip_serializing_if = "Option::is_none")]
    pub param_frmt_idr: Option<Max8Text>,
    #[serde(rename = "ParamsLngth", skip_serializing_if = "Option::is_none")]
    pub params_lngth: Option<PositiveNumber>,
    #[serde(rename = "OffsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<PositiveNumber>,
    #[serde(rename = "OffsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<PositiveNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Params", default)]
    pub params: Vec<Max100KBinary>,
    #[serde(rename = "NcrptdParams", skip_serializing_if = "Option::is_none")]
    pub ncrptd_params: Option<ContentInformationType32>,
}
#[derive(
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
pub struct SignedData7 {
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
    pub sgnr: Vec<Signer6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeolocationGeographicCoordinates1 {
    #[validate]
    #[serde(rename = "Lat")]
    pub lat: Max35Text,
    #[validate]
    #[serde(rename = "Long")]
    pub long: Max35Text,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetworkParameters9 {
    #[serde(rename = "NtwkTp")]
    pub ntwk_tp: NetworkType1Code,
    #[validate]
    #[serde(rename = "AdrVal")]
    pub adr_val: Max500Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetworkType1Code {
    #[serde(rename = "IPNW")]
    Ipnw,
    #[serde(rename = "PSTN")]
    Pstn,
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
pub struct PackageType3 {
    #[serde(rename = "PackgId", skip_serializing_if = "Option::is_none")]
    pub packg_id: Option<GenericIdentification176>,
    #[serde(rename = "PackgLngth", skip_serializing_if = "Option::is_none")]
    pub packg_lngth: Option<PositiveNumber>,
    #[serde(rename = "OffsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<PositiveNumber>,
    #[serde(rename = "OffsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<PositiveNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PackgBlck", default)]
    pub packg_blck: Vec<ExternallyDefinedData3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcceptorConfigurationUpdateV11 {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: TmsHeader1,
    #[validate]
    #[serde(rename = "AccptrCfgtn")]
    pub accptr_cfgtn: AcceptorConfiguration11,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType29>,
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
pub struct AlgorithmIdentification18 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter9>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageItemCondition1Code {
    #[serde(rename = "MNDT")]
    Mndt,
    #[serde(rename = "CFVL")]
    Cfvl,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "ALWV")]
    Alwv,
    #[serde(rename = "IFAV")]
    Ifav,
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "UNSP")]
    Unsp,
    #[default]
    Unknown,
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
pub struct AcceptorConfigurationContent11 {
    #[serde(rename = "RplcCfgtn", skip_serializing_if = "Option::is_none")]
    pub rplc_cfgtn: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TMSPrtcolParams", default)]
    pub tms_prtcol_params: Vec<TmsProtocolParameters6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AcqrrPrtcolParams", default)]
    pub acqrr_prtcol_params: Vec<AcquirerProtocolParameters15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvcPrvdrParams", default)]
    pub svc_prvdr_params: Vec<ServiceProviderParameters2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MrchntParams", default)]
    pub mrchnt_params: Vec<MerchantConfigurationParameters6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TermnlParams", default)]
    pub termnl_params: Vec<PaymentTerminalParameters8>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ApplParams", default)]
    pub appl_params: Vec<ApplicationParameters11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "HstComParams", default)]
    pub hst_com_params: Vec<HostCommunicationParameter6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyParams", default)]
    pub scty_params: Vec<SecurityParameters14>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SaleToPOIParams", default)]
    pub sale_to_poi_params: Vec<SaleToPoiProtocolParameter2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TermnlPackg", default)]
    pub termnl_packg: Vec<TerminalPackageType3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancellationProcess2Code {
    #[serde(rename = "ADVC")]
    Advc,
    #[serde(rename = "NALW")]
    Nalw,
    #[serde(rename = "REQU")]
    Requ,
    #[serde(rename = "APPL")]
    Appl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExchangePolicy2Code {
    #[serde(rename = "ONDM")]
    Ondm,
    #[serde(rename = "IMMD")]
    Immd,
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "AGRP")]
    Agrp,
    #[serde(rename = "NBLT")]
    Nblt,
    #[serde(rename = "TTLT")]
    Ttlt,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "BLCK")]
    Blck,
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
pub struct Max1025Text {
    #[validate(length(min = 1, max = 1025,))]
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
pub struct DataSetIdentification9 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max256Text>,
    #[serde(rename = "Tp")]
    pub tp: DataSetCategory17Code,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max256Text>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
}
#[derive(
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
pub struct SaleToPoiProtocolParameter2 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[serde(rename = "MrchntId", skip_serializing_if = "Option::is_none")]
    pub mrchnt_id: Option<Organisation26>,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[validate]
    #[serde(rename = "HstId")]
    pub hst_id: Max35Text,
    #[serde(rename = "MrchntPOIId", skip_serializing_if = "Option::is_none")]
    pub mrchnt_poi_id: Option<Max35Text>,
    #[serde(rename = "SaleId", skip_serializing_if = "Option::is_none")]
    pub sale_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XtrnlyTpSpprtd", default)]
    pub xtrnly_tp_spprtd: Vec<Max1025Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TimeUnit1Code {
    #[serde(rename = "DAYC")]
    Dayc,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "MINU")]
    Minu,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "SECO")]
    Seco,
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
pub struct ContentInformationType30 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "EnvlpdData", skip_serializing_if = "Option::is_none")]
    pub envlpd_data: Option<EnvelopedData9>,
    #[serde(rename = "AuthntcdData", skip_serializing_if = "Option::is_none")]
    pub authntcd_data: Option<AuthenticatedData8>,
    #[serde(rename = "SgndData", skip_serializing_if = "Option::is_none")]
    pub sgnd_data: Option<SignedData7>,
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
pub struct Max2KBinary {
    #[validate(length(min = 1, max = 2048,), regex = "MAX_2_K_BINARY_REGEX")]
    pub value: String,
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
pub struct GeolocationUtmCoordinates1 {
    #[validate]
    #[serde(rename = "UTMZone")]
    pub utm_zone: Max35Text,
    #[validate]
    #[serde(rename = "UTMEstwrd")]
    pub utm_estwrd: Max35Text,
    #[validate]
    #[serde(rename = "UTMNrthwrd")]
    pub utm_nrthwrd: Max35Text,
}
#[derive(
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
pub struct ExternallyDefinedData3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max1025Text,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max100KBinary>,
    #[serde(rename = "PrtctdVal", skip_serializing_if = "Option::is_none")]
    pub prtctd_val: Option<ContentInformationType30>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max1025Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TerminalManagementAction3Code {
    #[serde(rename = "CREA")]
    Crea,
    #[serde(rename = "DELT")]
    Delt,
    #[serde(rename = "UPDT")]
    Updt,
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
pub struct Recipient11ChoiceEnum {
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport8>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek8>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient11Choice {
    #[serde(flatten)]
    pub value: Recipient11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExchangeConfiguration10 {
    #[validate(length(min = 1,))]
    #[serde(rename = "XchgPlcy", default)]
    pub xchg_plcy: Vec<ExchangePolicy2Code>,
    #[serde(rename = "MaxNb", skip_serializing_if = "Option::is_none")]
    pub max_nb: Option<Number>,
    #[serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none")]
    pub max_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "ReTry", skip_serializing_if = "Option::is_none")]
    pub re_try: Option<ProcessRetry3>,
    #[serde(rename = "TmCond", skip_serializing_if = "Option::is_none")]
    pub tm_cond: Option<ProcessTiming6>,
    #[serde(rename = "XchgFaild", skip_serializing_if = "Option::is_none")]
    pub xchg_faild: Option<TrueFalseIndicator>,
    #[serde(rename = "XchgDclnd", skip_serializing_if = "Option::is_none")]
    pub xchg_dclnd: Option<TrueFalseIndicator>,
}
#[derive(
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
pub struct GenericIdentification176 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType33Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType33Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Min2Max3AlphaText>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount8Code {
    #[serde(rename = "ACTL")]
    Actl,
    #[serde(rename = "ESTM")]
    Estm,
    #[serde(rename = "MAXI")]
    Maxi,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "RPLT")]
    Rplt,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "RESD")]
    Resd,
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
pub struct AcquirerProtocolExchangeBehavior2 {
    #[serde(rename = "FinCaptr")]
    pub fin_captr: FinancialCapture1Code,
    #[serde(rename = "BtchTrf", skip_serializing_if = "Option::is_none")]
    pub btch_trf: Option<ExchangeConfiguration9>,
    #[serde(rename = "CmpltnXchg", skip_serializing_if = "Option::is_none")]
    pub cmpltn_xchg: Option<ExchangeConfiguration10>,
    #[serde(rename = "CxlXchg", skip_serializing_if = "Option::is_none")]
    pub cxl_xchg: Option<CancellationProcess2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityParameters14 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[serde(rename = "POIChllng", skip_serializing_if = "Option::is_none")]
    pub poi_chllng: Option<Max140Binary>,
    #[serde(rename = "TMChllng", skip_serializing_if = "Option::is_none")]
    pub tm_chllng: Option<Max140Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyElmt", default)]
    pub scty_elmt: Vec<CryptographicKey16>,
}
#[derive(
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
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct LocalDateTime1 {
    #[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "UTCOffset")]
    pub utc_offset: Number,
}
#[derive(
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
pub struct Kek8 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier7,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification29,
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
pub struct Parameter15 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm16Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification12>,
    #[serde(rename = "SaltLngth", skip_serializing_if = "Option::is_none")]
    pub salt_lngth: Option<Number>,
    #[serde(rename = "TrlrFld", skip_serializing_if = "Option::is_none")]
    pub trlr_fld: Option<Number>,
    #[serde(rename = "OIDCrvNm", skip_serializing_if = "Option::is_none")]
    pub oid_crv_nm: Option<Max140Text>,
}
#[derive(
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
pub struct TmsProtocolParameters6 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate]
    #[serde(rename = "TermnlMgrId")]
    pub termnl_mgr_id: GenericIdentification176,
    #[serde(rename = "PrtcolVrsn", skip_serializing_if = "Option::is_none")]
    pub prtcol_vrsn: Option<Max8Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "MntncSvc", default)]
    pub mntnc_svc: Vec<DataSetCategory10Code>,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "ApplId", default)]
    pub appl_id: Vec<Max35Text>,
    #[validate]
    #[serde(rename = "HstId")]
    pub hst_id: Max35Text,
    #[serde(rename = "POIId", skip_serializing_if = "Option::is_none")]
    pub poi_id: Option<Max35Text>,
    #[serde(rename = "InitgPtyId", skip_serializing_if = "Option::is_none")]
    pub initg_pty_id: Option<Max35Text>,
    #[serde(rename = "RcptPtyId", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty_id: Option<Max35Text>,
    #[serde(rename = "FileTrf", skip_serializing_if = "Option::is_none")]
    pub file_trf: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgItm", default)]
    pub msg_itm: Vec<MessageItemCondition1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XtrnlyTpSpprtd", default)]
    pub xtrnly_tp_spprtd: Vec<Max1025Text>,
}
#[derive(
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
pub struct AcceptorConfigurationDataSet3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: DataSetIdentification9,
    #[serde(rename = "SeqCntr", skip_serializing_if = "Option::is_none")]
    pub seq_cntr: Option<Max9NumericText>,
    #[serde(rename = "LastSeq", skip_serializing_if = "Option::is_none")]
    pub last_seq: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "POIId", default)]
    pub poi_id: Vec<GenericIdentification176>,
    #[serde(rename = "CfgtnScp", skip_serializing_if = "Option::is_none")]
    pub cfgtn_scp: Option<PartyType15Code>,
    #[validate]
    #[serde(rename = "Cntt")]
    pub cntt: AcceptorConfigurationContent11,
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
pub struct ExchangeConfiguration9 {
    #[validate(length(min = 1,))]
    #[serde(rename = "XchgPlcy", default)]
    pub xchg_plcy: Vec<ExchangePolicy2Code>,
    #[serde(rename = "MaxNb", skip_serializing_if = "Option::is_none")]
    pub max_nb: Option<Number>,
    #[serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none")]
    pub max_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "ReTry", skip_serializing_if = "Option::is_none")]
    pub re_try: Option<ProcessRetry3>,
    #[serde(rename = "TmCond", skip_serializing_if = "Option::is_none")]
    pub tm_cond: Option<ProcessTiming6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncryptedContent6 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "CnttNcrptnAlgo", skip_serializing_if = "Option::is_none")]
    pub cntt_ncrptn_algo: Option<AlgorithmIdentification29>,
    #[validate]
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: Max100KBinary,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BatchTransactionType1Code {
    #[serde(rename = "DTCT")]
    Dtct,
    #[serde(rename = "CNCL")]
    Cncl,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "DCLN")]
    Dcln,
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
pub struct Max9NumericText {
    #[validate(regex = "MAX_9_NUMERIC_TEXT_REGEX")]
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
pub struct AlgorithmIdentification29 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm24Code,
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
pub struct HostCommunicationParameter6 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate]
    #[serde(rename = "HstId")]
    pub hst_id: Max35Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<NetworkParameters7>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Key", default)]
    pub key: Vec<KekIdentifier5>,
    #[serde(rename = "NtwkSvcPrvdr", skip_serializing_if = "Option::is_none")]
    pub ntwk_svc_prvdr: Option<NetworkParameters7>,
    #[serde(rename = "PhysIntrfc", skip_serializing_if = "Option::is_none")]
    pub phys_intrfc: Option<PhysicalInterfaceParameter1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetworkParameters8 {
    #[serde(rename = "Tp")]
    pub tp: NetworkType2Code,
    #[validate]
    #[serde(rename = "Accs")]
    pub accs: NetworkParameters7,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10000Binary {
    #[validate(length(min = 1, max = 10000,), regex = "MAX_10000_BINARY_REGEX")]
    pub value: String,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType29 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "AuthntcdData", skip_serializing_if = "Option::is_none")]
    pub authntcd_data: Option<AuthenticatedData8>,
    #[serde(rename = "SgndData", skip_serializing_if = "Option::is_none")]
    pub sgnd_data: Option<SignedData7>,
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
pub struct Min3Max4Text {
    #[validate(length(min = 3, max = 4,))]
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
pub struct Max10KBinary {
    #[validate(length(min = 1, max = 10240,), regex = "MAX_10_K_BINARY_REGEX")]
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
pub struct PaymentTerminalParameters8 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[serde(rename = "VndrId", skip_serializing_if = "Option::is_none")]
    pub vndr_id: Option<Max35Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max256Text>,
    #[serde(rename = "ParamFrmtIdr", skip_serializing_if = "Option::is_none")]
    pub param_frmt_idr: Option<Max8Text>,
    #[serde(rename = "ClckSynctn", skip_serializing_if = "Option::is_none")]
    pub clck_synctn: Option<ClockSynchronisation3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TmZoneLine", default)]
    pub tm_zone_line: Vec<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LclDtTm", default)]
    pub lcl_dt_tm: Vec<LocalDateTime1>,
    #[serde(rename = "OthrParamsLngth", skip_serializing_if = "Option::is_none")]
    pub othr_params_lngth: Option<PositiveNumber>,
    #[serde(rename = "OffsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<PositiveNumber>,
    #[serde(rename = "OffsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<PositiveNumber>,
    #[serde(rename = "OthrParams", skip_serializing_if = "Option::is_none")]
    pub othr_params: Option<Max10000Binary>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction43Code {
    #[serde(rename = "FAUQ")]
    Fauq,
    #[serde(rename = "CCAQ")]
    Ccaq,
    #[serde(rename = "CMPV")]
    Cmpv,
    #[serde(rename = "DGNP")]
    Dgnp,
    #[serde(rename = "RCLQ")]
    Rclq,
    #[serde(rename = "CCAV")]
    Ccav,
    #[serde(rename = "BTCH")]
    Btch,
    #[serde(rename = "FRVA")]
    Frva,
    #[serde(rename = "AUTQ")]
    Autq,
    #[serde(rename = "FCMV")]
    Fcmv,
    #[serde(rename = "DCCQ")]
    Dccq,
    #[serde(rename = "RVRA")]
    Rvra,
    #[serde(rename = "DCAV")]
    Dcav,
    #[serde(rename = "TRNA")]
    Trna,
    #[serde(rename = "NFRQ")]
    Nfrq,
    #[serde(rename = "TRPQ")]
    Trpq,
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
pub struct GenericIdentification177 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType33Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType33Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Min2Max3AlphaText>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "RmotAccs", skip_serializing_if = "Option::is_none")]
    pub rmot_accs: Option<NetworkParameters7>,
    #[serde(rename = "Glctn", skip_serializing_if = "Option::is_none")]
    pub glctn: Option<Geolocation1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType33Code {
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
    #[serde(rename = "MTMG")]
    Mtmg,
    #[serde(rename = "TAXH")]
    Taxh,
    #[serde(rename = "TMGT")]
    Tmgt,
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
pub struct ProcessTiming6 {
    #[serde(rename = "StartTm", skip_serializing_if = "Option::is_none")]
    pub start_tm: Option<IsoDateTime>,
    #[serde(rename = "EndTm", skip_serializing_if = "Option::is_none")]
    pub end_tm: Option<IsoDateTime>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Max9NumericText>,
    #[serde(rename = "UnitOfTm", skip_serializing_if = "Option::is_none")]
    pub unit_of_tm: Option<TimeUnit1Code>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Organisation26 {
    #[validate]
    #[serde(rename = "CmonNm")]
    pub cmon_nm: Max70Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Max140Text>,
    #[serde(rename = "CtryCd")]
    pub ctry_cd: Iso3NumericCountryCode,
    #[validate]
    #[serde(rename = "MrchntCtgyCd")]
    pub mrchnt_ctgy_cd: Min3Max4Text,
    #[validate]
    #[serde(rename = "RegdIdr")]
    pub regd_idr: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TmsHeader1 {
    #[validate]
    #[serde(rename = "DwnldTrf")]
    pub dwnld_trf: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "FrmtVrsn")]
    pub frmt_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Number,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification176,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification177>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability8>,
}
#[derive(
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
pub struct PositiveNumber {
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
pub struct Parameter5 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Geolocation1 {
    #[serde(rename = "GeogcCordints", skip_serializing_if = "Option::is_none")]
    pub geogc_cordints: Option<GeolocationGeographicCoordinates1>,
    #[serde(rename = "UTMCordints", skip_serializing_if = "Option::is_none")]
    pub utm_cordints: Option<GeolocationUtmCoordinates1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType32 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "EnvlpdData")]
    pub envlpd_data: EnvelopedData9,
}
#[derive(
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
#[derive(
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
    #[serde(rename = "AccptrCfgtnUpd")]
    pub accptr_cfgtn_upd: AcceptorConfigurationUpdateV11,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetworkType2Code {
    #[serde(rename = "SCK5")]
    Sck5,
    #[serde(rename = "SCK4")]
    Sck4,
    #[serde(rename = "HTTP")]
    Http,
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
pub struct IssuerAndSerialNumber2 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: CertificateIssuer1,
    #[validate]
    #[serde(rename = "SrlNb")]
    pub srl_nb: Max500Binary,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcquirerHostConfiguration9 {
    #[validate]
    #[serde(rename = "HstId")]
    pub hst_id: Max35Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgToSnd", default)]
    pub msg_to_snd: Vec<MessageFunction43Code>,
    #[serde(rename = "PrtcolVrsn", skip_serializing_if = "Option::is_none")]
    pub prtcol_vrsn: Option<Max8Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XtrnlyTpSpprtd", default)]
    pub xtrnly_tp_spprtd: Vec<Max1025Text>,
}
#[derive(
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
pub struct MessageItemCondition1 {
    #[validate]
    #[serde(rename = "ItmId")]
    pub itm_id: Max140Text,
    #[serde(rename = "Cond")]
    pub cond: MessageItemCondition1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "Val", default)]
    pub val: Vec<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PhysicalInterfaceParameter1 {
    #[validate]
    #[serde(rename = "IntrfcNm")]
    pub intrfc_nm: Max35Text,
    #[serde(rename = "IntrfcTp", skip_serializing_if = "Option::is_none")]
    pub intrfc_tp: Option<PoiCommunicationType2Code>,
    #[serde(rename = "UsrNm", skip_serializing_if = "Option::is_none")]
    pub usr_nm: Option<Max35Text>,
    #[serde(rename = "AccsCd", skip_serializing_if = "Option::is_none")]
    pub accs_cd: Option<Max35Binary>,
    #[serde(rename = "SctyPrfl", skip_serializing_if = "Option::is_none")]
    pub scty_prfl: Option<Max35Text>,
    #[serde(rename = "AddtlParams", skip_serializing_if = "Option::is_none")]
    pub addtl_params: Option<Max2KBinary>,
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
#[derive(
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
pub struct Signer6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "SgnrId", skip_serializing_if = "Option::is_none")]
    pub sgnr_id: Option<Recipient12Choice>,
    #[validate]
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: AlgorithmIdentification21,
    #[validate(length(min = 0,))]
    #[serde(rename = "SgndAttrbts", default)]
    pub sgnd_attrbts: Vec<GenericInformation1>,
    #[validate]
    #[serde(rename = "SgntrAlgo")]
    pub sgntr_algo: AlgorithmIdentification30,
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
pub struct AcquirerProtocolParameters15 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: TerminalManagementAction3Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "AcqrrId", default)]
    pub acqrr_id: Vec<GenericIdentification176>,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max256Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "ApplId", default)]
    pub appl_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Hst", default)]
    pub hst: Vec<AcquirerHostConfiguration9>,
    #[serde(rename = "OnLineTx", skip_serializing_if = "Option::is_none")]
    pub on_line_tx: Option<AcquirerProtocolExchangeBehavior2>,
    #[serde(rename = "OffLineTx", skip_serializing_if = "Option::is_none")]
    pub off_line_tx: Option<AcquirerProtocolExchangeBehavior2>,
    #[serde(rename = "RcncltnXchg", skip_serializing_if = "Option::is_none")]
    pub rcncltn_xchg: Option<ExchangeConfiguration9>,
    #[serde(rename = "RcncltnByAcqrr", skip_serializing_if = "Option::is_none")]
    pub rcncltn_by_acqrr: Option<TrueFalseIndicator>,
    #[serde(rename = "TtlsPerCcy", skip_serializing_if = "Option::is_none")]
    pub ttls_per_ccy: Option<TrueFalseIndicator>,
    #[serde(rename = "SpltTtls", skip_serializing_if = "Option::is_none")]
    pub splt_ttls: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpltTtlCrit", default)]
    pub splt_ttl_crit: Vec<ReconciliationCriteria1Code>,
    #[serde(rename = "CmpltnAdvcMndtd", skip_serializing_if = "Option::is_none")]
    pub cmpltn_advc_mndtd: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AmtQlfrForRsvatn", default)]
    pub amt_qlfr_for_rsvatn: Vec<TypeOfAmount8Code>,
    #[serde(rename = "RcncltnErr", skip_serializing_if = "Option::is_none")]
    pub rcncltn_err: Option<TrueFalseIndicator>,
    #[serde(rename = "CardDataVrfctn", skip_serializing_if = "Option::is_none")]
    pub card_data_vrfctn: Option<TrueFalseIndicator>,
    #[serde(rename = "NtfyOffLineCxl", skip_serializing_if = "Option::is_none")]
    pub ntfy_off_line_cxl: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BtchTrfCntt", default)]
    pub btch_trf_cntt: Vec<BatchTransactionType1Code>,
    #[serde(rename = "FileTrfBtch", skip_serializing_if = "Option::is_none")]
    pub file_trf_btch: Option<TrueFalseIndicator>,
    #[serde(rename = "BtchDgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub btch_dgtl_sgntr: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgItm", default)]
    pub msg_itm: Vec<MessageItemCondition1>,
    #[validate]
    #[serde(rename = "PrtctCardData")]
    pub prtct_card_data: TrueFalseIndicator,
    #[serde(rename = "PrvtCardData", skip_serializing_if = "Option::is_none")]
    pub prvt_card_data: Option<TrueFalseIndicator>,
    #[serde(rename = "MndtrySctyTrlr", skip_serializing_if = "Option::is_none")]
    pub mndtry_scty_trlr: Option<TrueFalseIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm24Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "DKP9")]
    Dkp9,
    #[serde(rename = "UKPT")]
    Ukpt,
    #[serde(rename = "UKA2")]
    Uka2,
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
    #[serde(rename = "UKA1")]
    Uka1,
    #[serde(rename = "UKA3")]
    Uka3,
    #[default]
    Unknown,
}
