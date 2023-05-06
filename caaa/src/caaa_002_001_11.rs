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
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_19_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,19}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_8_MAX_28_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,28}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_11_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:caaa.002.001.11".to_string()
}

#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Response9Code {
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "DECL")]
    Decl,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "TECH")]
    Tech,
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
pub struct Max500Binary {
    #[validate(length(min = 1, max = 500,), regex = "MAX_500_BINARY_REGEX")]
    pub value: String,
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
pub struct MobileData4 {
    #[serde(rename = "MobCtryCd", skip_serializing_if = "Option::is_none")]
    pub mob_ctry_cd: Option<Min2Max3AlphaText>,
    #[serde(rename = "MobNtwkCd", skip_serializing_if = "Option::is_none")]
    pub mob_ntwk_cd: Option<Min2Max3NumericText>,
    #[serde(rename = "MobMskdMSISDN", skip_serializing_if = "Option::is_none")]
    pub mob_mskd_msisdn: Option<Max35Text>,
    #[serde(rename = "Glctn", skip_serializing_if = "Option::is_none")]
    pub glctn: Option<Geolocation1>,
    #[serde(rename = "SnstvMobData", skip_serializing_if = "Option::is_none")]
    pub snstv_mob_data: Option<SensitiveMobileData1>,
    #[serde(rename = "PrtctdMobData", skip_serializing_if = "Option::is_none")]
    pub prtctd_mob_data: Option<ContentInformationType32>,
}
#[derive(
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
pub struct PointOfInteraction12 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification177,
    #[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<Max70Text>,
    #[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
    pub grp_id: Option<Max35Text>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<PointOfInteractionCapabilities9>,
    #[serde(rename = "TmZone", skip_serializing_if = "Option::is_none")]
    pub tm_zone: Option<Max70Text>,
    #[serde(rename = "TermnlIntgtn", skip_serializing_if = "Option::is_none")]
    pub termnl_intgtn: Option<LocationCategory3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmpnt", default)]
    pub cmpnt: Vec<PointOfInteractionComponent12>,
}
#[derive(
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
#[derive(
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
pub struct TmsTrigger1 {
    #[serde(rename = "TMSCtctLvl")]
    pub tms_ctct_lvl: TmsContactLevel1Code,
    #[serde(rename = "TMSId", skip_serializing_if = "Option::is_none")]
    pub tms_id: Option<Max35Text>,
    #[serde(rename = "TMSCtctDtTm", skip_serializing_if = "Option::is_none")]
    pub tms_ctct_dt_tm: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoiComponentAssessment1Code {
    #[serde(rename = "APPL")]
    Appl,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "EVAL")]
    Eval,
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
pub struct PointOfInteractionComponent12 {
    #[serde(rename = "Tp")]
    pub tp: PoiComponentType6Code,
    #[serde(rename = "SubTpInf", skip_serializing_if = "Option::is_none")]
    pub sub_tp_inf: Option<Max70Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: PointOfInteractionComponentIdentification2,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<PointOfInteractionComponentStatus3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "StdCmplc", default)]
    pub std_cmplc: Vec<GenericIdentification48>,
    #[serde(rename = "Chrtcs", skip_serializing_if = "Option::is_none")]
    pub chrtcs: Option<PointOfInteractionComponentCharacteristics8>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Assmnt", default)]
    pub assmnt: Vec<PointOfInteractionComponentAssessment1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Packg", default)]
    pub packg: Vec<PackageType3>,
}
#[derive(
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
pub struct DetailedAmount15 {
    #[serde(rename = "AmtGoodsAndSvcs", skip_serializing_if = "Option::is_none")]
    pub amt_goods_and_svcs: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "CshBck", skip_serializing_if = "Option::is_none")]
    pub csh_bck: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Grtty", skip_serializing_if = "Option::is_none")]
    pub grtty: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Fees", default)]
    pub fees: Vec<DetailedAmount4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rbt", default)]
    pub rbt: Vec<DetailedAmount4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ValAddedTax", default)]
    pub val_added_tax: Vec<DetailedAmount4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Srchrg", default)]
    pub srchrg: Vec<DetailedAmount4>,
}
#[derive(
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
#[derive(
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
pub enum AuthenticationEntity2Code {
    #[serde(rename = "ICCD")]
    Iccd,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ISSR")]
    Issr,
    #[serde(rename = "TRML")]
    Trml,
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
pub struct PointOfInteractionComponentAssessment1 {
    #[serde(rename = "Tp")]
    pub tp: PoiComponentAssessment1Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "Assgnr", default)]
    pub assgnr: Vec<Max35Text>,
    #[serde(rename = "DlvryDt", skip_serializing_if = "Option::is_none")]
    pub dlvry_dt: Option<IsoDateTime>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min8Max28NumericText {
    #[validate(regex = "MIN_8_MAX_28_NUMERIC_TEXT_REGEX")]
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
pub struct Max10000Binary {
    #[validate(length(min = 1, max = 10000,), regex = "MAX_10000_BINARY_REGEX")]
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
pub struct ActionMessage9 {
    #[serde(rename = "MsgDstn")]
    pub msg_dstn: UserInterface4Code,
    #[serde(rename = "InfQlfr", skip_serializing_if = "Option::is_none")]
    pub inf_qlfr: Option<InformationQualify1Code>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat3Code>,
    #[serde(rename = "MsgCntt", skip_serializing_if = "Option::is_none")]
    pub msg_cntt: Option<Max20000Text>,
    #[serde(rename = "MsgCnttSgntr", skip_serializing_if = "Option::is_none")]
    pub msg_cntt_sgntr: Option<ContentInformationType29>,
    #[serde(rename = "OutptBrcd", skip_serializing_if = "Option::is_none")]
    pub outpt_brcd: Option<OutputBarcode1>,
    #[serde(rename = "RspnReqrdFlg", skip_serializing_if = "Option::is_none")]
    pub rspn_reqrd_flg: Option<TrueFalseIndicator>,
    #[serde(rename = "MinDispTm", skip_serializing_if = "Option::is_none")]
    pub min_disp_tm: Option<Number>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType12Code {
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "NOVR")]
    Novr,
    #[serde(rename = "RQID")]
    Rqid,
    #[serde(rename = "PINL")]
    Pinl,
    #[serde(rename = "PINR")]
    Pinr,
    #[serde(rename = "PRNT")]
    Prnt,
    #[serde(rename = "RFRL")]
    Rfrl,
    #[serde(rename = "RQDT")]
    Rqdt,
    #[serde(rename = "DCCQ")]
    Dccq,
    #[serde(rename = "FLFW")]
    Flfw,
    #[serde(rename = "PINQ")]
    Pinq,
    #[serde(rename = "CDCV")]
    Cdcv,
    #[serde(rename = "CHDA")]
    Chda,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "STOR")]
    Stor,
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
#[derive(
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
pub struct PostalAddress22 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max70Text>,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "CtrySubDvsn", default)]
    pub ctry_sub_dvsn: Vec<Max35Text>,
    #[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
    pub ctry_cd: Option<Min2Max3AlphaText>,
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
pub struct Max2KBinary {
    #[validate(length(min = 1, max = 2048,), regex = "MAX_2_K_BINARY_REGEX")]
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AttendanceContext2Code {
    #[serde(rename = "ATTL")]
    Attl,
    #[serde(rename = "CARR")]
    Carr,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "SELF")]
    XSelf,
    #[default]
    Unknown,
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
pub struct CardPaymentTransaction121 {
    #[serde(rename = "SaleRefId", skip_serializing_if = "Option::is_none")]
    pub sale_ref_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentifier1,
    #[serde(rename = "InitrTxId", skip_serializing_if = "Option::is_none")]
    pub initr_tx_id: Option<Max35Text>,
    #[serde(rename = "RcptTxId", skip_serializing_if = "Option::is_none")]
    pub rcpt_tx_id: Option<Max140Text>,
    #[serde(rename = "RcncltnId", skip_serializing_if = "Option::is_none")]
    pub rcncltn_id: Option<Max35Text>,
    #[serde(rename = "IntrchngData", skip_serializing_if = "Option::is_none")]
    pub intrchng_data: Option<Max140Text>,
    #[validate]
    #[serde(rename = "TxDtls")]
    pub tx_dtls: CardPaymentTransactionDetails51,
    #[serde(rename = "MrchntRefData", skip_serializing_if = "Option::is_none")]
    pub mrchnt_ref_data: Option<Max70Text>,
}
#[derive(
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct GenericIdentification90 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp")]
    pub tp: PartyType14Code,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CheckType1Code {
    #[serde(rename = "BANK")]
    Bank,
    #[serde(rename = "BUSI")]
    Busi,
    #[serde(rename = "GOVC")]
    Govc,
    #[serde(rename = "PAYR")]
    Payr,
    #[serde(rename = "PERS")]
    Pers,
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
pub struct NetworkParameters9 {
    #[serde(rename = "NtwkTp")]
    pub ntwk_tp: NetworkType1Code,
    #[validate]
    #[serde(rename = "AdrVal")]
    pub adr_val: Max500Text,
}
#[derive(
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
pub struct PersonIdentification15 {
    #[serde(rename = "DrvrLicNb", skip_serializing_if = "Option::is_none")]
    pub drvr_lic_nb: Option<Max35Text>,
    #[serde(rename = "DrvrLicLctn", skip_serializing_if = "Option::is_none")]
    pub drvr_lic_lctn: Option<Max35Text>,
    #[serde(rename = "DrvrLicNm", skip_serializing_if = "Option::is_none")]
    pub drvr_lic_nm: Option<Max35Text>,
    #[serde(rename = "DrvrId", skip_serializing_if = "Option::is_none")]
    pub drvr_id: Option<Max35Text>,
    #[serde(rename = "CstmrNb", skip_serializing_if = "Option::is_none")]
    pub cstmr_nb: Option<Max35Text>,
    #[serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none")]
    pub scl_scty_nb: Option<Max35Text>,
    #[serde(rename = "AlnRegnNb", skip_serializing_if = "Option::is_none")]
    pub aln_regn_nb: Option<Max35Text>,
    #[serde(rename = "PsptNb", skip_serializing_if = "Option::is_none")]
    pub pspt_nb: Option<Max35Text>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "IdntyCardNb", skip_serializing_if = "Option::is_none")]
    pub idnty_card_nb: Option<Max35Text>,
    #[serde(rename = "MplyrIdNb", skip_serializing_if = "Option::is_none")]
    pub mplyr_id_nb: Option<Max35Text>,
    #[serde(rename = "MplyeeIdNb", skip_serializing_if = "Option::is_none")]
    pub mplyee_id_nb: Option<Max35Text>,
    #[serde(rename = "JobNb", skip_serializing_if = "Option::is_none")]
    pub job_nb: Option<Max35Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max35Text>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericIdentification4>,
}
#[derive(
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
#[derive(
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
pub struct OriginalAmountDetails1 {
    #[serde(rename = "ActlAmt", skip_serializing_if = "Option::is_none")]
    pub actl_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MinAmt", skip_serializing_if = "Option::is_none")]
    pub min_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none")]
    pub max_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Exemption1Code {
    #[serde(rename = "LOWA")]
    Lowa,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "RECP")]
    Recp,
    #[serde(rename = "SCPE")]
    Scpe,
    #[serde(rename = "SCAD")]
    Scad,
    #[serde(rename = "TRAE")]
    Trae,
    #[serde(rename = "PKGE")]
    Pkge,
    #[serde(rename = "TMBE")]
    Tmbe,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StoredValueAccountType1Code {
    #[serde(rename = "BNKA")]
    Bnka,
    #[serde(rename = "CWVC")]
    Cwvc,
    #[serde(rename = "CPYA")]
    Cpya,
    #[serde(rename = "ELMY")]
    Elmy,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "GCER")]
    Gcer,
    #[serde(rename = "MLVC")]
    Mlvc,
    #[serde(rename = "OLVC")]
    Olvc,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "TRVL")]
    Trvl,
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
pub struct Action12 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: ActionType12Code,
    #[serde(rename = "MsgToPres", skip_serializing_if = "Option::is_none")]
    pub msg_to_pres: Option<ActionMessage9>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure6Code {
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
    #[serde(rename = "KWHO")]
    Kwho,
    #[serde(rename = "DGEU")]
    Dgeu,
    #[serde(rename = "GGEU")]
    Ggeu,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstalmentAmountDetails1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InstalmentAmountDetailsType1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<Amount5>,
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
pub struct Max30Text {
    #[validate(length(max = 30,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct CommunicationCharacteristics5 {
    #[serde(rename = "ComTp")]
    pub com_tp: PoiCommunicationType2Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "RmotPty", default)]
    pub rmot_pty: Vec<PartyType7Code>,
    #[validate]
    #[serde(rename = "Actv")]
    pub actv: TrueFalseIndicator,
    #[serde(rename = "Params", skip_serializing_if = "Option::is_none")]
    pub params: Option<NetworkParameters7>,
    #[serde(rename = "PhysIntrfc", skip_serializing_if = "Option::is_none")]
    pub phys_intrfc: Option<PhysicalInterfaceParameter1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction42Code {
    #[serde(rename = "AUTQ")]
    Autq,
    #[serde(rename = "AUTP")]
    Autp,
    #[serde(rename = "CCAV")]
    Ccav,
    #[serde(rename = "CCAK")]
    Ccak,
    #[serde(rename = "CCAQ")]
    Ccaq,
    #[serde(rename = "CCAP")]
    Ccap,
    #[serde(rename = "CMPV")]
    Cmpv,
    #[serde(rename = "CMPK")]
    Cmpk,
    #[serde(rename = "DCAV")]
    Dcav,
    #[serde(rename = "DCRR")]
    Dcrr,
    #[serde(rename = "DCCQ")]
    Dccq,
    #[serde(rename = "DCCP")]
    Dccp,
    #[serde(rename = "DGNP")]
    Dgnp,
    #[serde(rename = "DGNQ")]
    Dgnq,
    #[serde(rename = "FAUQ")]
    Fauq,
    #[serde(rename = "FAUP")]
    Faup,
    #[serde(rename = "FCMV")]
    Fcmv,
    #[serde(rename = "FCMK")]
    Fcmk,
    #[serde(rename = "FRVA")]
    Frva,
    #[serde(rename = "FRVR")]
    Frvr,
    #[serde(rename = "RCLQ")]
    Rclq,
    #[serde(rename = "RCLP")]
    Rclp,
    #[serde(rename = "RVRA")]
    Rvra,
    #[serde(rename = "RVRR")]
    Rvrr,
    #[serde(rename = "CDDQ")]
    Cddq,
    #[serde(rename = "CDDK")]
    Cddk,
    #[serde(rename = "CDDR")]
    Cddr,
    #[serde(rename = "CDDP")]
    Cddp,
    #[serde(rename = "TRNR")]
    Trnr,
    #[serde(rename = "TRNA")]
    Trna,
    #[serde(rename = "NFRQ")]
    Nfrq,
    #[serde(rename = "NFRP")]
    Nfrp,
    #[serde(rename = "TRPQ")]
    Trpq,
    #[serde(rename = "TRPP")]
    Trpp,
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
pub enum AuthenticationMethod8Code {
    #[serde(rename = "TOKA")]
    Toka,
    #[serde(rename = "ADDB")]
    Addb,
    #[serde(rename = "BYPS")]
    Byps,
    #[serde(rename = "BIOM")]
    Biom,
    #[serde(rename = "CDHI")]
    Cdhi,
    #[serde(rename = "CRYP")]
    Cryp,
    #[serde(rename = "CSCV")]
    Cscv,
    #[serde(rename = "MANU")]
    Manu,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "MOBL")]
    Mobl,
    #[serde(rename = "FPIN")]
    Fpin,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PPSG")]
    Ppsg,
    #[serde(rename = "PSVE")]
    Psve,
    #[serde(rename = "PSWD")]
    Pswd,
    #[serde(rename = "TOKP")]
    Tokp,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SCNL")]
    Scnl,
    #[serde(rename = "CSEC")]
    Csec,
    #[serde(rename = "SNCT")]
    Snct,
    #[serde(rename = "ADDS")]
    Adds,
    #[serde(rename = "CPSG")]
    Cpsg,
    #[serde(rename = "TOKN")]
    Tokn,
    #[serde(rename = "UKNW")]
    Uknw,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BarcodeType1Code {
    #[serde(rename = "COQR")]
    Coqr,
    #[serde(rename = "C128")]
    C128,
    #[serde(rename = "C025")]
    C025,
    #[serde(rename = "C039")]
    C039,
    #[serde(rename = "EA13")]
    Ea13,
    #[serde(rename = "EAN8")]
    Ean8,
    #[serde(rename = "P417")]
    P417,
    #[serde(rename = "UPCA")]
    Upca,
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
pub struct Max37Text {
    #[validate(length(min = 1, max = 37,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LoyaltyHandling1Code {
    #[serde(rename = "ALLO")]
    Allo,
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "PRCS")]
    Prcs,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "REQU")]
    Requ,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LocationCategory4Code {
    #[serde(rename = "ABRD")]
    Abrd,
    #[serde(rename = "NMDC")]
    Nmdc,
    #[serde(rename = "FIXD")]
    Fixd,
    #[serde(rename = "VIRT")]
    Virt,
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
#[derive(
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
pub struct GracePeriod1 {
    #[validate]
    #[serde(rename = "Tm")]
    pub tm: Max3NumericText,
    #[serde(rename = "UnitTp")]
    pub unit_tp: GracePeriodUnitType1Code,
    #[serde(rename = "OthrUnitTp", skip_serializing_if = "Option::is_none")]
    pub othr_unit_tp: Option<Max35Text>,
}
#[derive(
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
pub struct Header59 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: MessageFunction42Code,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Number,
    #[serde(rename = "ReTrnsmssnCntr", skip_serializing_if = "Option::is_none")]
    pub re_trnsmssn_cntr: Option<Max3NumericText>,
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
pub struct Organisation41 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdentification32>,
    #[serde(rename = "CmonNm", skip_serializing_if = "Option::is_none")]
    pub cmon_nm: Option<Max70Text>,
    #[serde(rename = "LctnCtgy", skip_serializing_if = "Option::is_none")]
    pub lctn_ctgy: Option<LocationCategory4Code>,
    #[serde(rename = "LctnAndCtct", skip_serializing_if = "Option::is_none")]
    pub lctn_and_ctct: Option<CommunicationAddress9>,
    #[serde(rename = "SchmeData", skip_serializing_if = "Option::is_none")]
    pub schme_data: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Commission18 {
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
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
pub struct AuthorisationResult17 {
    #[serde(rename = "AuthstnNtty", skip_serializing_if = "Option::is_none")]
    pub authstn_ntty: Option<GenericIdentification90>,
    #[validate]
    #[serde(rename = "RspnToAuthstn")]
    pub rspn_to_authstn: ResponseType10,
    #[serde(rename = "AuthstnCd", skip_serializing_if = "Option::is_none")]
    pub authstn_cd: Option<Max8Text>,
    #[serde(rename = "CmpltnReqrd", skip_serializing_if = "Option::is_none")]
    pub cmpltn_reqrd: Option<TrueFalseIndicator>,
    #[serde(rename = "TMSTrggr", skip_serializing_if = "Option::is_none")]
    pub tms_trggr: Option<TmsTrigger1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LocationCategory3Code {
    #[serde(rename = "INDR")]
    Indr,
    #[serde(rename = "IPMP")]
    Ipmp,
    #[serde(rename = "MPOI")]
    Mpoi,
    #[serde(rename = "MPMP")]
    Mpmp,
    #[serde(rename = "MSLE")]
    Msle,
    #[serde(rename = "SSLE")]
    Ssle,
    #[serde(rename = "VNDG")]
    Vndg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OnLineCapability1Code {
    #[serde(rename = "OFLN")]
    Ofln,
    #[serde(rename = "ONLN")]
    Onln,
    #[serde(rename = "SMON")]
    Smon,
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
pub struct Commission19 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyDetails2 {
    #[serde(rename = "AlphaCd", skip_serializing_if = "Option::is_none")]
    pub alpha_cd: Option<ActiveCurrencyCode>,
    #[serde(rename = "NmrcCd", skip_serializing_if = "Option::is_none")]
    pub nmrc_cd: Option<Exact3NumericText>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<Number>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Acquirer10 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdentification177>,
    #[serde(rename = "ParamsVrsn", skip_serializing_if = "Option::is_none")]
    pub params_vrsn: Option<Max256Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Verification1Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "NOVF")]
    Novf,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SUCC")]
    Succ,
    #[serde(rename = "ERRR")]
    Errr,
    #[default]
    Unknown,
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
pub struct Product4 {
    #[validate]
    #[serde(rename = "PdctCd")]
    pub pdct_cd: Max70Text,
    #[serde(rename = "AddtlPdctCd", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_cd: Option<Max70Text>,
}
#[derive(
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
pub enum AuthenticationResult1Code {
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "MRCH")]
    Mrch,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "CRPT")]
    Crpt,
    #[serde(rename = "UCRP")]
    Ucrp,
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
pub struct OnLinePin9 {
    #[validate]
    #[serde(rename = "NcrptdPINBlck")]
    pub ncrptd_pin_blck: ContentInformationType32,
    #[serde(rename = "PINFrmt")]
    pub pin_frmt: PinFormat3Code,
    #[serde(rename = "AddtlInpt", skip_serializing_if = "Option::is_none")]
    pub addtl_inpt: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OutputBarcode1 {
    #[serde(rename = "BrcdTp")]
    pub brcd_tp: BarcodeType1Code,
    #[serde(rename = "BrcdVal", skip_serializing_if = "Option::is_none")]
    pub brcd_val: Option<Max8000Text>,
    #[serde(rename = "QRCdBinryVal", skip_serializing_if = "Option::is_none")]
    pub qr_cd_binry_val: Option<Max3000Binary>,
    #[serde(rename = "QRCdVrsn", skip_serializing_if = "Option::is_none")]
    pub qr_cd_vrsn: Option<Max16Text>,
    #[serde(rename = "QRCdNcodgMd")]
    pub qr_cd_ncodg_md: QrCodeEncodingMode1Code,
    #[serde(rename = "QRCdErrCrrctn", skip_serializing_if = "Option::is_none")]
    pub qr_cd_err_crrctn: Option<QrCodeErrorCorrection1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Token1 {
    #[serde(rename = "PmtTkn", skip_serializing_if = "Option::is_none")]
    pub pmt_tkn: Option<Max19NumericText>,
    #[serde(rename = "TknXpryDt", skip_serializing_if = "Option::is_none")]
    pub tkn_xpry_dt: Option<Exact4NumericText>,
    #[serde(rename = "TknRqstrId", skip_serializing_if = "Option::is_none")]
    pub tkn_rqstr_id: Option<Max11NumericText>,
    #[serde(rename = "TknAssrncData", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_data: Option<Max140Text>,
    #[serde(rename = "TknAssrncMtd", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_mtd: Option<Max2NumericText>,
    #[serde(rename = "TknInittdInd", skip_serializing_if = "Option::is_none")]
    pub tkn_inittd_ind: Option<TrueFalseIndicator>,
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
pub struct CardPaymentTransaction114 {
    #[validate]
    #[serde(rename = "AuthstnRslt")]
    pub authstn_rslt: AuthorisationResult17,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxVrfctnRslt", default)]
    pub tx_vrfctn_rslt: Vec<TransactionVerificationResult4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AllwdPdctCd", default)]
    pub allwd_pdct_cd: Vec<Product4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NotAllwdPdctCd", default)]
    pub not_allwd_pdct_cd: Vec<Product4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAvlblPdct", default)]
    pub addtl_avlbl_pdct: Vec<Product5>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<AmountAndDirection93>,
    #[serde(rename = "PrtctdBal", skip_serializing_if = "Option::is_none")]
    pub prtctd_bal: Option<ContentInformationType32>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Actn", default)]
    pub actn: Vec<Action12>,
    #[serde(rename = "CcyConvsElgblty", skip_serializing_if = "Option::is_none")]
    pub ccy_convs_elgblty: Option<CurrencyConversion23>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TrackFormat1Code {
    #[serde(rename = "AAMV")]
    Aamv,
    #[serde(rename = "CMC7")]
    Cmc7,
    #[serde(rename = "E13B")]
    E13B,
    #[serde(rename = "ISOF")]
    Isof,
    #[serde(rename = "JIS1")]
    Jis1,
    #[serde(rename = "JIS2")]
    Jis2,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountUnit1Code {
    #[serde(rename = "MONE")]
    Mone,
    #[serde(rename = "POIN")]
    Poin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PinFormat3Code {
    #[serde(rename = "ISO0")]
    Iso0,
    #[serde(rename = "ISO1")]
    Iso1,
    #[serde(rename = "ISO2")]
    Iso2,
    #[serde(rename = "ISO3")]
    Iso3,
    #[serde(rename = "ISO4")]
    Iso4,
    #[serde(rename = "ISO5")]
    Iso5,
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
pub struct PlainCardData17 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Min8Max28NumericText>,
    #[serde(rename = "Trck1", skip_serializing_if = "Option::is_none")]
    pub trck_1: Option<Max76Text>,
    #[serde(rename = "Trck2", skip_serializing_if = "Option::is_none")]
    pub trck_2: Option<Max37Text>,
    #[serde(rename = "Trck3", skip_serializing_if = "Option::is_none")]
    pub trck_3: Option<Max104Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlCardData", default)]
    pub addtl_card_data: Vec<Max35Text>,
    #[serde(rename = "NtryMd", skip_serializing_if = "Option::is_none")]
    pub ntry_md: Option<CardDataReading5Code>,
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
pub enum QrCodeEncodingMode1Code {
    #[serde(rename = "ALFA")]
    Alfa,
    #[serde(rename = "BINA")]
    Bina,
    #[serde(rename = "KANJ")]
    Kanj,
    #[serde(rename = "NUME")]
    Nume,
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
pub struct DisplayCapabilities4 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Dstn", default)]
    pub dstn: Vec<UserInterface4Code>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType31 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "AuthntcdData")]
    pub authntcd_data: AuthenticatedData8,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyConversion24 {
    #[serde(rename = "AccptdByCrdhldr", skip_serializing_if = "Option::is_none")]
    pub accptd_by_crdhldr: Option<TrueFalseIndicator>,
    #[serde(rename = "Convs", skip_serializing_if = "Option::is_none")]
    pub convs: Option<CurrencyConversion23>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "AccptrAuthstnRspn")]
    pub accptr_authstn_rspn: AcceptorAuthorisationResponseV11<A>,
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
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OutputFormat3Code {
    #[serde(rename = "BARC")]
    Barc,
    #[serde(rename = "MENT")]
    Ment,
    #[serde(rename = "MREF")]
    Mref,
    #[serde(rename = "SREF")]
    Sref,
    #[serde(rename = "TEXT")]
    Text,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrackData2 {
    #[serde(rename = "TrckNb", skip_serializing_if = "Option::is_none")]
    pub trck_nb: Option<Number>,
    #[serde(rename = "TrckFrmt", skip_serializing_if = "Option::is_none")]
    pub trck_frmt: Option<TrackFormat1Code>,
    #[validate]
    #[serde(rename = "TrckVal")]
    pub trck_val: Max140Text,
}
#[derive(
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
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek8>,
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport8>,
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
pub struct Max5000Binary {
    #[validate(length(min = 1, max = 5000,), regex = "MAX_5000_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardAccountType3Code {
    #[serde(rename = "CTDP")]
    Ctdp,
    #[serde(rename = "CHCK")]
    Chck,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CDBT")]
    Cdbt,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "EPRS")]
    Eprs,
    #[serde(rename = "HEQL")]
    Heql,
    #[serde(rename = "ISTL")]
    Istl,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "LCDT")]
    Lcdt,
    #[serde(rename = "MBNW")]
    Mbnw,
    #[serde(rename = "MNMK")]
    Mnmk,
    #[serde(rename = "MNMC")]
    Mnmc,
    #[serde(rename = "MTGL")]
    Mtgl,
    #[serde(rename = "RTRM")]
    Rtrm,
    #[serde(rename = "RVLV")]
    Rvlv,
    #[serde(rename = "SVNG")]
    Svng,
    #[serde(rename = "STBD")]
    Stbd,
    #[serde(rename = "UVRL")]
    Uvrl,
    #[serde(rename = "PRPD")]
    Prpd,
    #[serde(rename = "FLTC")]
    Fltc,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UserInterface4Code {
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
pub struct CardPaymentTransactionDetails51 {
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[validate]
    #[serde(rename = "TtlAmt")]
    pub ttl_amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CmltvAmt", skip_serializing_if = "Option::is_none")]
    pub cmltv_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtQlfr", skip_serializing_if = "Option::is_none")]
    pub amt_qlfr: Option<TypeOfAmount8Code>,
    #[serde(rename = "DtldAmt", skip_serializing_if = "Option::is_none")]
    pub dtld_amt: Option<DetailedAmount15>,
    #[serde(rename = "ReqdAmt", skip_serializing_if = "Option::is_none")]
    pub reqd_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AuthrsdAmt", skip_serializing_if = "Option::is_none")]
    pub authrsd_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "InvcAmt", skip_serializing_if = "Option::is_none")]
    pub invc_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "VldtyDt", skip_serializing_if = "Option::is_none")]
    pub vldty_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OnLineRsn", default)]
    pub on_line_rsn: Vec<OnLineReason2Code>,
    #[serde(rename = "UattnddLvlCtgy", skip_serializing_if = "Option::is_none")]
    pub uattndd_lvl_ctgy: Option<Max35NumericText>,
    #[serde(rename = "AcctTp", skip_serializing_if = "Option::is_none")]
    pub acct_tp: Option<CardAccountType3Code>,
    #[serde(rename = "CcyConvsRslt", skip_serializing_if = "Option::is_none")]
    pub ccy_convs_rslt: Option<CurrencyConversion24>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Instlmt", default)]
    pub instlmt: Vec<RecurringTransaction4>,
    #[serde(rename = "AggtnTx", skip_serializing_if = "Option::is_none")]
    pub aggtn_tx: Option<AggregationTransaction3>,
    #[serde(rename = "PdctCdSetId", skip_serializing_if = "Option::is_none")]
    pub pdct_cd_set_id: Option<Max10Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SaleItm", default)]
    pub sale_itm: Vec<Product6>,
    #[serde(rename = "DlvryLctn", skip_serializing_if = "Option::is_none")]
    pub dlvry_lctn: Option<Max35Text>,
    #[serde(rename = "ReSubmissnCntr", skip_serializing_if = "Option::is_none")]
    pub re_submissn_cntr: Option<PositiveNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<ExternallyDefinedData3>,
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max10000Binary>,
}
#[derive(
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
pub struct Max8Text {
    #[validate(length(min = 1, max = 8,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentAmountDetailsType1Code {
    #[serde(rename = "TAXX")]
    Taxx,
    #[serde(rename = "RQST")]
    Rqst,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHC")]
    Othc,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "FUNA")]
    Funa,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "EXPN")]
    Expn,
    #[serde(rename = "AFCO")]
    Afco,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency3Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "TEND")]
    Tend,
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
pub struct ResponseType10 {
    #[serde(rename = "Rspn")]
    pub rspn: Response9Code,
    #[serde(rename = "RspnRsn", skip_serializing_if = "Option::is_none")]
    pub rspn_rsn: Option<Max35Text>,
    #[serde(rename = "AddtlRspnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rspn_inf: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AggregationTransaction3 {
    #[serde(rename = "FrstPmtDtTm", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "LastPmtDtTm", skip_serializing_if = "Option::is_none")]
    pub last_pmt_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "NbOfPmts", skip_serializing_if = "Option::is_none")]
    pub nb_of_pmts: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvPmt", default)]
    pub indv_pmt: Vec<DetailedAmount21>,
}
#[derive(
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
pub struct Max8000Text {
    #[validate(length(min = 1, max = 8000,))]
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
pub struct CustomerDevice3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max70Text>,
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdTp")]
    pub id_tp: Max35Text,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading8Code {
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
    #[serde(rename = "SICC")]
    Sicc,
    #[serde(rename = "UNKW")]
    Unkw,
    #[serde(rename = "QRCD")]
    Qrcd,
    #[serde(rename = "OPTC")]
    Optc,
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
pub struct Max2NumericText {
    #[validate(regex = "MAX_2_NUMERIC_TEXT_REGEX")]
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
pub struct CurrencyCode {
    #[validate(regex = "CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardProductType1Code {
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CONS")]
    Cons,
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
pub struct Amount5 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
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
pub struct Exact3AlphaNumericText {
    #[validate(regex = "EXACT_3_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct CardholderAuthentication15 {
    #[serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none")]
    pub authntcn_mtd: Option<AuthenticationMethod8Code>,
    #[serde(rename = "AuthntcnXmptn", skip_serializing_if = "Option::is_none")]
    pub authntcn_xmptn: Option<Exemption1Code>,
    #[serde(rename = "AuthntcnVal", skip_serializing_if = "Option::is_none")]
    pub authntcn_val: Option<Max5000Binary>,
    #[serde(rename = "PrtctdAuthntcnVal", skip_serializing_if = "Option::is_none")]
    pub prtctd_authntcn_val: Option<ContentInformationType32>,
    #[serde(rename = "CrdhldrOnLinePIN", skip_serializing_if = "Option::is_none")]
    pub crdhldr_on_line_pin: Option<OnLinePin9>,
    #[serde(rename = "CrdhldrId", skip_serializing_if = "Option::is_none")]
    pub crdhldr_id: Option<PersonIdentification15>,
    #[serde(rename = "AdrVrfctn", skip_serializing_if = "Option::is_none")]
    pub adr_vrfctn: Option<AddressVerification1>,
    #[serde(rename = "AuthntcnTp", skip_serializing_if = "Option::is_none")]
    pub authntcn_tp: Option<Max35Text>,
    #[serde(rename = "AuthntcnLvl", skip_serializing_if = "Option::is_none")]
    pub authntcn_lvl: Option<Max35Text>,
    #[serde(rename = "AuthntcnRslt", skip_serializing_if = "Option::is_none")]
    pub authntcn_rslt: Option<AuthenticationResult1Code>,
    #[serde(rename = "AuthntcnAddtlInf", skip_serializing_if = "Option::is_none")]
    pub authntcn_addtl_inf: Option<ExternallyDefinedData3>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardIdentificationType1Code {
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "BARC")]
    Barc,
    #[serde(rename = "ISO2")]
    Iso2,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "CPAN")]
    Cpan,
    #[serde(rename = "PRIV")]
    Priv,
    #[serde(rename = "UUID")]
    Uuid,
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
pub struct AmountAndDirection93 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy")]
    pub ccy: ActiveCurrencyCode,
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
pub struct GenericInformation1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max140Text>,
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
#[derive(
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
pub struct AcceptorAuthorisationResponse11<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: CardPaymentEnvironment78,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: CardPaymentTransaction121,
    #[validate]
    #[serde(rename = "TxRspn")]
    pub tx_rspn: CardPaymentTransaction114,
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
pub struct TransactionVerificationResult4 {
    #[serde(rename = "Mtd")]
    pub mtd: AuthenticationMethod6Code,
    #[serde(rename = "VrfctnNtty", skip_serializing_if = "Option::is_none")]
    pub vrfctn_ntty: Option<AuthenticationEntity2Code>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Verification1Code>,
    #[serde(rename = "AddtlRslt", skip_serializing_if = "Option::is_none")]
    pub addtl_rslt: Option<Max500Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CardPaymentEnvironment78 {
    #[serde(rename = "Acqrr", skip_serializing_if = "Option::is_none")]
    pub acqrr: Option<Acquirer10>,
    #[serde(rename = "Mrchnt", skip_serializing_if = "Option::is_none")]
    pub mrchnt: Option<Organisation41>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction12>,
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard32>,
    #[serde(rename = "Chck", skip_serializing_if = "Option::is_none")]
    pub chck: Option<Check1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "StordValAcct", default)]
    pub stord_val_acct: Vec<StoredValueAccount2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LltyAcct", default)]
    pub llty_acct: Vec<LoyaltyAccount3>,
    #[serde(rename = "CstmrDvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_dvc: Option<CustomerDevice3>,
    #[serde(rename = "Wllt", skip_serializing_if = "Option::is_none")]
    pub wllt: Option<CustomerDevice3>,
    #[serde(rename = "PmtTkn", skip_serializing_if = "Option::is_none")]
    pub pmt_tkn: Option<Token1>,
    #[serde(rename = "MrchntTkn", skip_serializing_if = "Option::is_none")]
    pub mrchnt_tkn: Option<MerchantToken2>,
    #[serde(rename = "Crdhldr", skip_serializing_if = "Option::is_none")]
    pub crdhldr: Option<Cardholder18>,
    #[serde(rename = "PrtctdCrdhldrData", skip_serializing_if = "Option::is_none")]
    pub prtctd_crdhldr_data: Option<ContentInformationType32>,
    #[serde(rename = "SaleEnvt", skip_serializing_if = "Option::is_none")]
    pub sale_envt: Option<RetailerSaleEnvironment2>,
}
#[derive(
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
pub struct RecurringTransaction4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "InstlmtPlan", default)]
    pub instlmt_plan: Vec<InstalmentPlan1Code>,
    #[serde(rename = "PlanId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<Max35Text>,
    #[serde(rename = "PlanOwnr", skip_serializing_if = "Option::is_none")]
    pub plan_ownr: Option<PlanOwner1Code>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "PrdUnit", skip_serializing_if = "Option::is_none")]
    pub prd_unit: Option<Frequency3Code>,
    #[serde(rename = "InstlmtPrd", skip_serializing_if = "Option::is_none")]
    pub instlmt_prd: Option<Number>,
    #[serde(rename = "TtlNbOfPmts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_pmts: Option<Number>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<CurrencyAndAmount>,
    #[serde(rename = "FrstAmt", skip_serializing_if = "Option::is_none")]
    pub frst_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "SbsqntAmt", skip_serializing_if = "Option::is_none")]
    pub sbsqnt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "LastAmt", skip_serializing_if = "Option::is_none")]
    pub last_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DtldChrgs", default)]
    pub dtld_chrgs: Vec<InstalmentAmountDetails1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRate", default)]
    pub intrst_rate: Vec<InterestRateDetails1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GracePrd", default)]
    pub grace_prd: Vec<GracePeriod1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20000Text {
    #[validate(length(min = 1, max = 20000,))]
    #[serde(rename = "$text")]
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
pub struct Max3Text {
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType14Code {
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
pub struct TransactionIdentifier1 {
    #[validate]
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "TxRef")]
    pub tx_ref: Max35Text,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateDetails1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestRate1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Prd")]
    pub prd: InstalmentPeriod1Code,
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
pub struct PointOfInteractionCapabilities9 {
    #[validate(length(min = 0,))]
    #[serde(rename = "CardRdngCpblties", default)]
    pub card_rdng_cpblties: Vec<CardDataReading8Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CrdhldrVrfctnCpblties", default)]
    pub crdhldr_vrfctn_cpblties: Vec<CardholderVerificationCapability4Code>,
    #[serde(rename = "PINLngthCpblties", skip_serializing_if = "Option::is_none")]
    pub pin_lngth_cpblties: Option<PositiveNumber>,
    #[serde(rename = "ApprvlCdLngth", skip_serializing_if = "Option::is_none")]
    pub apprvl_cd_lngth: Option<PositiveNumber>,
    #[serde(rename = "MxScrptLngth", skip_serializing_if = "Option::is_none")]
    pub mx_scrpt_lngth: Option<PositiveNumber>,
    #[serde(rename = "CardCaptrCpbl", skip_serializing_if = "Option::is_none")]
    pub card_captr_cpbl: Option<TrueFalseIndicator>,
    #[serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none")]
    pub on_line_cpblties: Option<OnLineCapability1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgCpblties", default)]
    pub msg_cpblties: Vec<DisplayCapabilities4>,
}
#[derive(
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
pub struct Product5 {
    #[validate]
    #[serde(rename = "PdctCd")]
    pub pdct_cd: Max70Text,
    #[serde(rename = "AddtlPdctCd", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_cd: Option<Max70Text>,
    #[serde(rename = "AmtLmt", skip_serializing_if = "Option::is_none")]
    pub amt_lmt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "QtyLmt", skip_serializing_if = "Option::is_none")]
    pub qty_lmt: Option<DecimalNumber>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Check1 {
    #[serde(rename = "BkId", skip_serializing_if = "Option::is_none")]
    pub bk_id: Option<Max35Text>,
    #[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
    pub acct_nb: Option<Max35Text>,
    #[serde(rename = "ChckNb", skip_serializing_if = "Option::is_none")]
    pub chck_nb: Option<Max35Text>,
    #[serde(rename = "ChckCardNb", skip_serializing_if = "Option::is_none")]
    pub chck_card_nb: Option<Max35Text>,
    #[serde(rename = "ChckTrckData2", skip_serializing_if = "Option::is_none")]
    pub chck_trck_data_2: Option<TrackData2>,
    #[serde(rename = "ChckTp", skip_serializing_if = "Option::is_none")]
    pub chck_tp: Option<CheckType1Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Max3Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentPlan1Code {
    #[serde(rename = "EQPM")]
    Eqpm,
    #[serde(rename = "NQPM")]
    Nqpm,
    #[serde(rename = "DFRI")]
    Dfri,
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
pub struct ContentInformationType29 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "AuthntcdData", skip_serializing_if = "Option::is_none")]
    pub authntcd_data: Option<AuthenticatedData8>,
    #[serde(rename = "SgndData", skip_serializing_if = "Option::is_none")]
    pub sgnd_data: Option<SignedData7>,
}
#[derive(
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
pub struct CurrencyDetails3 {
    #[serde(rename = "AlphaCd")]
    pub alpha_cd: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "NmrcCd")]
    pub nmrc_cd: Exact3NumericText,
    #[validate]
    #[serde(rename = "Dcml")]
    pub dcml: Number,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AuthenticationMethod6Code {
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "PPSG")]
    Ppsg,
    #[serde(rename = "PSWD")]
    Pswd,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SCNL")]
    Scnl,
    #[serde(rename = "SNCT")]
    Snct,
    #[serde(rename = "CPSG")]
    Cpsg,
    #[serde(rename = "ADDB")]
    Addb,
    #[serde(rename = "BIOM")]
    Biom,
    #[serde(rename = "CDHI")]
    Cdhi,
    #[serde(rename = "CRYP")]
    Cryp,
    #[serde(rename = "CSCV")]
    Cscv,
    #[serde(rename = "PSVE")]
    Psve,
    #[serde(rename = "CSEC")]
    Csec,
    #[serde(rename = "ADDS")]
    Adds,
    #[serde(rename = "MANU")]
    Manu,
    #[serde(rename = "FPIN")]
    Fpin,
    #[serde(rename = "TOKP")]
    Tokp,
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
#[derive(
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
pub enum PoiComponentType6Code {
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
    #[serde(rename = "LOGF")]
    Logf,
    #[serde(rename = "MDFL")]
    Mdfl,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "RPFL")]
    Rpfl,
    #[default]
    Unknown,
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
pub struct PlainCardData15 {
    #[validate]
    #[serde(rename = "PAN")]
    pub pan: Min8Max28NumericText,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<Max10Text>,
    #[validate]
    #[serde(rename = "XpryDt")]
    pub xpry_dt: Max10Text,
    #[serde(rename = "SvcCd", skip_serializing_if = "Option::is_none")]
    pub svc_cd: Option<Exact3NumericText>,
    #[serde(rename = "Trck1", skip_serializing_if = "Option::is_none")]
    pub trck_1: Option<Max76Text>,
    #[serde(rename = "Trck2", skip_serializing_if = "Option::is_none")]
    pub trck_2: Option<Max37Text>,
    #[serde(rename = "Trck3", skip_serializing_if = "Option::is_none")]
    pub trck_3: Option<Max104Text>,
    #[serde(rename = "CrdhldrNm", skip_serializing_if = "Option::is_none")]
    pub crdhldr_nm: Option<Max45Text>,
}
#[derive(
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
#[derive(
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
pub struct AddressVerification1 {
    #[serde(rename = "AdrDgts", skip_serializing_if = "Option::is_none")]
    pub adr_dgts: Option<Max5NumericText>,
    #[serde(rename = "PstlCdDgts", skip_serializing_if = "Option::is_none")]
    pub pstl_cd_dgts: Option<Max5NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Vehicle1 {
    #[serde(rename = "VhclNb", skip_serializing_if = "Option::is_none")]
    pub vhcl_nb: Option<Max35NumericText>,
    #[serde(rename = "TrlrNb", skip_serializing_if = "Option::is_none")]
    pub trlr_nb: Option<Max35NumericText>,
    #[serde(rename = "VhclTag", skip_serializing_if = "Option::is_none")]
    pub vhcl_tag: Option<Max35Text>,
    #[serde(rename = "VhclTagNtryMd", skip_serializing_if = "Option::is_none")]
    pub vhcl_tag_ntry_md: Option<CardDataReading5Code>,
    #[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
    pub unit_nb: Option<Max35NumericText>,
    #[serde(rename = "RplcmntCar", skip_serializing_if = "Option::is_none")]
    pub rplcmnt_car: Option<TrueFalseIndicator>,
    #[serde(rename = "Odmtr", skip_serializing_if = "Option::is_none")]
    pub odmtr: Option<DecimalNumber>,
    #[serde(rename = "Hbmtr", skip_serializing_if = "Option::is_none")]
    pub hbmtr: Option<DecimalNumber>,
    #[serde(rename = "TrlrHrs", skip_serializing_if = "Option::is_none")]
    pub trlr_hrs: Option<Max35Text>,
    #[serde(rename = "RefrHrs", skip_serializing_if = "Option::is_none")]
    pub refr_hrs: Option<Max35Text>,
    #[serde(rename = "MntncId", skip_serializing_if = "Option::is_none")]
    pub mntnc_id: Option<Max35Text>,
    #[serde(rename = "DrvrOrVhclCard", skip_serializing_if = "Option::is_none")]
    pub drvr_or_vhcl_card: Option<PlainCardData17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlVhclData", default)]
    pub addtl_vhcl_data: Vec<Vehicle2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OnLineReason2Code {
    #[serde(rename = "RNDM")]
    Rndm,
    #[serde(rename = "ICCF")]
    Iccf,
    #[serde(rename = "MERF")]
    Merf,
    #[serde(rename = "TRMF")]
    Trmf,
    #[serde(rename = "ISSF")]
    Issf,
    #[serde(rename = "FRLT")]
    Frlt,
    #[serde(rename = "EXFL")]
    Exfl,
    #[serde(rename = "TAMT")]
    Tamt,
    #[serde(rename = "CBIN")]
    Cbin,
    #[serde(rename = "UBIN")]
    Ubin,
    #[serde(rename = "CPAN")]
    Cpan,
    #[serde(rename = "FLOW")]
    Flow,
    #[serde(rename = "CRCY")]
    Crcy,
    #[serde(rename = "IFPR")]
    Ifpr,
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
pub struct Cardholder18 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PersonIdentification15>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max45Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
    #[serde(rename = "BllgAdr", skip_serializing_if = "Option::is_none")]
    pub bllg_adr: Option<PostalAddress22>,
    #[serde(rename = "ShppgAdr", skip_serializing_if = "Option::is_none")]
    pub shppg_adr: Option<PostalAddress22>,
    #[serde(rename = "TripNb", skip_serializing_if = "Option::is_none")]
    pub trip_nb: Option<Max35Text>,
    #[serde(rename = "Vhcl", skip_serializing_if = "Option::is_none")]
    pub vhcl: Option<Vehicle1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Authntcn", default)]
    pub authntcn: Vec<CardholderAuthentication15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxVrfctnRslt", default)]
    pub tx_vrfctn_rslt: Vec<TransactionVerificationResult4>,
    #[serde(rename = "PrsnlData", skip_serializing_if = "Option::is_none")]
    pub prsnl_data: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MobData", default)]
    pub mob_data: Vec<MobileData4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Product6 {
    #[serde(rename = "ItmId", skip_serializing_if = "Option::is_none")]
    pub itm_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "PdctCd")]
    pub pdct_cd: Max70Text,
    #[serde(rename = "AddtlPdctCd", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_cd: Option<Max70Text>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure6Code>,
    #[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<DecimalNumber>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "UnitPricSgn", skip_serializing_if = "Option::is_none")]
    pub unit_pric_sgn: Option<PlusOrMinusIndicator>,
    #[validate]
    #[serde(rename = "PdctAmt")]
    pub pdct_amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "PdctAmtSgn", skip_serializing_if = "Option::is_none")]
    pub pdct_amt_sgn: Option<PlusOrMinusIndicator>,
    #[serde(rename = "ValAddedTax", skip_serializing_if = "Option::is_none")]
    pub val_added_tax: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
    #[serde(rename = "PdctDesc", skip_serializing_if = "Option::is_none")]
    pub pdct_desc: Option<Max140Text>,
    #[serde(rename = "DlvryLctn", skip_serializing_if = "Option::is_none")]
    pub dlvry_lctn: Option<Max10Text>,
    #[serde(rename = "DlvrySvc", skip_serializing_if = "Option::is_none")]
    pub dlvry_svc: Option<AttendanceContext2Code>,
    #[serde(rename = "SaleChanl", skip_serializing_if = "Option::is_none")]
    pub sale_chanl: Option<Max70Text>,
    #[serde(rename = "AddtlPdctDesc", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_desc: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RetailerSaleEnvironment2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "SaleCpblties", default)]
    pub sale_cpblties: Vec<SaleCapabilities1Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "MinAmtToDlvr", skip_serializing_if = "Option::is_none")]
    pub min_amt_to_dlvr: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MaxCshBckAmt", skip_serializing_if = "Option::is_none")]
    pub max_csh_bck_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MinSpltAmt", skip_serializing_if = "Option::is_none")]
    pub min_splt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "DbtPrefrdFlg", skip_serializing_if = "Option::is_none")]
    pub dbt_prefrd_flg: Option<TrueFalseIndicator>,
    #[serde(rename = "LltyHdlg", skip_serializing_if = "Option::is_none")]
    pub llty_hdlg: Option<LoyaltyHandling1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QrCodeErrorCorrection1Code {
    #[serde(rename = "M015")]
    M015,
    #[serde(rename = "Q025")]
    Q025,
    #[serde(rename = "H030")]
    H030,
    #[serde(rename = "L007")]
    L007,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardholderVerificationCapability4Code {
    #[serde(rename = "APKI")]
    Apki,
    #[serde(rename = "CHDT")]
    Chdt,
    #[serde(rename = "MNSG")]
    Mnsg,
    #[serde(rename = "MNVR")]
    Mnvr,
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
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "PKIS")]
    Pkis,
    #[serde(rename = "SCEC")]
    Scec,
    #[serde(rename = "NBIO")]
    Nbio,
    #[serde(rename = "NOVF")]
    Novf,
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
pub struct DetailedAmount21 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "DtTm")]
    pub dt_tm: IsoDateTime,
    #[serde(rename = "CardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub card_data_ntry_md: Option<CardDataReading8Code>,
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max10000Binary>,
    #[serde(rename = "Labl", skip_serializing_if = "Option::is_none")]
    pub labl: Option<Max140Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SaleCapabilities1Code {
    #[serde(rename = "CHDI")]
    Chdi,
    #[serde(rename = "CHER")]
    Cher,
    #[serde(rename = "CHIN")]
    Chin,
    #[serde(rename = "CHST")]
    Chst,
    #[serde(rename = "CUDI")]
    Cudi,
    #[serde(rename = "CUAS")]
    Cuas,
    #[serde(rename = "CUER")]
    Cuer,
    #[serde(rename = "CUIN")]
    Cuin,
    #[serde(rename = "POIR")]
    Poir,
    #[serde(rename = "PRDC")]
    Prdc,
    #[serde(rename = "PRRP")]
    Prrp,
    #[serde(rename = "PRVC")]
    Prvc,
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
pub struct StoredValueAccount2 {
    #[serde(rename = "AcctTp", skip_serializing_if = "Option::is_none")]
    pub acct_tp: Option<StoredValueAccountType1Code>,
    #[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
    pub id_tp: Option<CardIdentificationType1Code>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Brnd", skip_serializing_if = "Option::is_none")]
    pub brnd: Option<Max35Text>,
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max35Text>,
    #[serde(rename = "OwnrNm", skip_serializing_if = "Option::is_none")]
    pub ownr_nm: Option<Max45Text>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<Max10Text>,
    #[serde(rename = "NtryMd", skip_serializing_if = "Option::is_none")]
    pub ntry_md: Option<CardDataReading8Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedAmount4 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Labl", skip_serializing_if = "Option::is_none")]
    pub labl: Option<Max140Text>,
}
#[derive(
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
pub struct PaymentTokenIdentifiers1 {
    #[validate]
    #[serde(rename = "PrvdrId")]
    pub prvdr_id: Max35Text,
    #[validate]
    #[serde(rename = "RqstrId")]
    pub rqstr_id: Max35Text,
}
#[derive(
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
pub enum TmsContactLevel1Code {
    #[serde(rename = "CRIT")]
    Crit,
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "DTIM")]
    Dtim,
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
pub struct CurrencyConversion23 {
    #[serde(rename = "CcyConvsId", skip_serializing_if = "Option::is_none")]
    pub ccy_convs_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TrgtCcy")]
    pub trgt_ccy: CurrencyDetails3,
    #[validate]
    #[serde(rename = "RsltgAmt")]
    pub rsltg_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: PercentageRate,
    #[serde(rename = "NvrtdXchgRate", skip_serializing_if = "Option::is_none")]
    pub nvrtd_xchg_rate: Option<PercentageRate>,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<IsoDateTime>,
    #[serde(rename = "VldUntil", skip_serializing_if = "Option::is_none")]
    pub vld_until: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "SrcCcy")]
    pub src_ccy: CurrencyDetails2,
    #[validate]
    #[serde(rename = "OrgnlAmt")]
    pub orgnl_amt: OriginalAmountDetails1,
    #[validate(length(min = 0,))]
    #[serde(rename = "ComssnDtls", default)]
    pub comssn_dtls: Vec<Commission19>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MrkUpDtls", default)]
    pub mrk_up_dtls: Vec<Commission18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DclrtnDtls", default)]
    pub dclrtn_dtls: Vec<ActionMessage9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationAddress9 {
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress22>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
    pub phne: Option<PhoneNumber>,
    #[serde(rename = "CstmrSvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_svc: Option<PhoneNumber>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPlaceOfBirth1 {
    #[validate]
    #[serde(rename = "BirthDt")]
    pub birth_dt: IsoDate,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: Max35Text,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCard32 {
    #[serde(rename = "PrtctdCardData", skip_serializing_if = "Option::is_none")]
    pub prtctd_card_data: Option<ContentInformationType32>,
    #[serde(rename = "PrvtCardData", skip_serializing_if = "Option::is_none")]
    pub prvt_card_data: Option<Max100KBinary>,
    #[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
    pub plain_card_data: Option<PlainCardData15>,
    #[serde(rename = "PmtAcctRef", skip_serializing_if = "Option::is_none")]
    pub pmt_acct_ref: Option<Max70Text>,
    #[serde(rename = "MskdPAN", skip_serializing_if = "Option::is_none")]
    pub mskd_pan: Option<Max30Text>,
    #[serde(rename = "IssrBIN", skip_serializing_if = "Option::is_none")]
    pub issr_bin: Option<Max15NumericText>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<Max3Text>,
    #[serde(rename = "CardCcyCd", skip_serializing_if = "Option::is_none")]
    pub card_ccy_cd: Option<Exact3AlphaNumericText>,
    #[serde(rename = "CardPdctPrfl", skip_serializing_if = "Option::is_none")]
    pub card_pdct_prfl: Option<Max35Text>,
    #[serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none")]
    pub card_brnd: Option<Max35Text>,
    #[serde(rename = "CardPdctTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_tp: Option<CardProductType1Code>,
    #[serde(rename = "CardPdctSubTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_sub_tp: Option<Max35Text>,
    #[serde(rename = "IntrnlCard", skip_serializing_if = "Option::is_none")]
    pub intrnl_card: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AllwdPdct", default)]
    pub allwd_pdct: Vec<Max70Text>,
    #[serde(rename = "SvcOptn", skip_serializing_if = "Option::is_none")]
    pub svc_optn: Option<Max35Text>,
    #[serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none")]
    pub addtl_card_data: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InformationQualify1Code {
    #[serde(rename = "CUSA")]
    Cusa,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "ERRO")]
    Erro,
    #[serde(rename = "INPT")]
    Inpt,
    #[serde(rename = "POIR")]
    Poir,
    #[serde(rename = "RCPT")]
    Rcpt,
    #[serde(rename = "SOND")]
    Sond,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "VCHR")]
    Vchr,
    #[default]
    Unknown,
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
pub struct AcceptorAuthorisationResponseV11<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header59,
    #[validate]
    #[serde(rename = "AuthstnRspn")]
    pub authstn_rspn: AcceptorAuthorisationResponse11<A>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType31>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoyaltyAccount3 {
    #[validate]
    #[serde(rename = "LltyId")]
    pub llty_id: Max35Text,
    #[serde(rename = "NtryMd", skip_serializing_if = "Option::is_none")]
    pub ntry_md: Option<CardDataReading8Code>,
    #[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
    pub id_tp: Option<CardIdentificationType1Code>,
    #[serde(rename = "Brnd", skip_serializing_if = "Option::is_none")]
    pub brnd: Option<Max35Text>,
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max35Text>,
    #[serde(rename = "OwnrNm", skip_serializing_if = "Option::is_none")]
    pub ownr_nm: Option<Max45Text>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<AmountUnit1Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
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
#[derive(
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
pub struct SensitiveMobileData1 {
    #[validate]
    #[serde(rename = "MSISDN")]
    pub msisdn: Max35NumericText,
    #[serde(rename = "IMSI", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<Max35NumericText>,
    #[serde(rename = "IMEI", skip_serializing_if = "Option::is_none")]
    pub imei: Option<Max35NumericText>,
}
#[derive(
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
pub struct MerchantToken2 {
    #[serde(rename = "Tkn", skip_serializing_if = "Option::is_none")]
    pub tkn: Option<Max35Text>,
    #[serde(rename = "TknXpryDt", skip_serializing_if = "Option::is_none")]
    pub tkn_xpry_dt: Option<Max10Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TknChrtc", default)]
    pub tkn_chrtc: Vec<Max35Text>,
    #[serde(rename = "TknRqstr", skip_serializing_if = "Option::is_none")]
    pub tkn_rqstr: Option<PaymentTokenIdentifiers1>,
    #[serde(rename = "TknAssrncLvl", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_lvl: Option<Number>,
    #[serde(rename = "TknAssrncData", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_data: Option<Max500Binary>,
    #[serde(rename = "TknAssrncMtd", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_mtd: Option<Max2NumericText>,
    #[serde(rename = "TknInittdInd", skip_serializing_if = "Option::is_none")]
    pub tkn_inittd_ind: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PointOfInteractionComponentCharacteristics8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Mmry", default)]
    pub mmry: Vec<MemoryCharacteristics1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Com", default)]
    pub com: Vec<CommunicationCharacteristics5>,
    #[serde(rename = "SctyAccsMdls", skip_serializing_if = "Option::is_none")]
    pub scty_accs_mdls: Option<Number>,
    #[serde(rename = "SbcbrIdntyMdls", skip_serializing_if = "Option::is_none")]
    pub sbcbr_idnty_mdls: Option<Number>,
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
pub struct Max45Text {
    #[validate(length(min = 1, max = 45,))]
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
