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
    static ref MAX_3000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:caam.004.001.03".to_string()
}

#[derive(
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
pub struct AtmEquipment3 {
    #[serde(rename = "Manfctr", skip_serializing_if = "Option::is_none")]
    pub manfctr: Option<Max35Text>,
    #[serde(rename = "Mdl", skip_serializing_if = "Option::is_none")]
    pub mdl: Option<Max35Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max35Text>,
    #[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
    pub srl_nb: Option<Max35Text>,
    #[serde(rename = "SgndSrlNb", skip_serializing_if = "Option::is_none")]
    pub sgnd_srl_nb: Option<ContentInformationType14>,
    #[serde(rename = "FrmwrPrvdr", skip_serializing_if = "Option::is_none")]
    pub frmwr_prvdr: Option<Max35Text>,
    #[serde(rename = "FrmwrId", skip_serializing_if = "Option::is_none")]
    pub frmwr_id: Option<Max35Text>,
    #[serde(rename = "FrmwrVrsn", skip_serializing_if = "Option::is_none")]
    pub frmwr_vrsn: Option<Max35Text>,
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
pub struct GeographicCoordinates1 {
    #[validate]
    #[serde(rename = "Lat")]
    pub lat: Max16Text,
    #[validate]
    #[serde(rename = "Long")]
    pub long: Max16Text,
}
#[derive(
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
pub struct Min2Max3AlphaText {
    #[validate(regex = "MIN_2_MAX_3_ALPHA_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmSecurityScheme3Code {
    #[serde(rename = "APPK")]
    Appk,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "FRAN")]
    Fran,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "LUXG")]
    Luxg,
    #[serde(rename = "MANU")]
    Manu,
    #[serde(rename = "PKIP")]
    Pkip,
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "TR34")]
    Tr34,
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
pub struct Parameter8 {
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: Algorithm11Code,
    #[validate]
    #[serde(rename = "MskGnrtrAlgo")]
    pub msk_gnrtr_algo: AlgorithmIdentification12,
    #[validate]
    #[serde(rename = "SaltLngth")]
    pub salt_lngth: Number,
    #[serde(rename = "TrlrFld", skip_serializing_if = "Option::is_none")]
    pub trlr_fld: Option<Number>,
}
#[derive(
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
pub enum AtmStatus1Code {
    #[serde(rename = "INSV")]
    Insv,
    #[serde(rename = "OUTS")]
    Outs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmCommand6Code {
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
    #[serde(rename = "KACT")]
    Kact,
    #[serde(rename = "KDAC")]
    Kdac,
    #[serde(rename = "KDWL")]
    Kdwl,
    #[serde(rename = "KRMV")]
    Krmv,
    #[serde(rename = "SCFU")]
    Scfu,
    #[serde(rename = "SSCU")]
    Sscu,
    #[serde(rename = "SSTU")]
    Sstu,
    #[serde(rename = "SNDM")]
    Sndm,
    #[serde(rename = "HKCG")]
    Hkcg,
    #[serde(rename = "HKRV")]
    Hkrv,
    #[serde(rename = "KCHG")]
    Kchg,
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "ATMKeyDwnldRspn")]
    pub atm_key_dwnld_rspn: AtmKeyDownloadResponseV03,
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
pub struct AtmSecurityConfiguration5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PINFrmt", default)]
    pub pin_frmt: Vec<PinFormat4Code>,
    #[serde(rename = "PINLngthCpblties", skip_serializing_if = "Option::is_none")]
    pub pin_lngth_cpblties: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CryptographicKey12 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max140Text>,
    #[serde(rename = "SctyDomnId", skip_serializing_if = "Option::is_none")]
    pub scty_domn_id: Option<Max35Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Binary>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max256Text>,
    #[serde(rename = "SeqCntr", skip_serializing_if = "Option::is_none")]
    pub seq_cntr: Option<Number>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CryptographicKeyType3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Fctn", default)]
    pub fctn: Vec<KeyUsage1Code>,
    #[serde(rename = "ActvtnDt", skip_serializing_if = "Option::is_none")]
    pub actvtn_dt: Option<IsoDateTime>,
    #[serde(rename = "DeactvtnDt", skip_serializing_if = "Option::is_none")]
    pub deactvtn_dt: Option<IsoDateTime>,
    #[serde(rename = "KeyChckVal", skip_serializing_if = "Option::is_none")]
    pub key_chck_val: Option<Max35Binary>,
    #[serde(rename = "PblcKeyVal", skip_serializing_if = "Option::is_none")]
    pub pblc_key_val: Option<PublicRsaKey1>,
    #[serde(rename = "KeyChcVal", skip_serializing_if = "Option::is_none")]
    pub key_chc_val: Option<KeyChoiceValue2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Signer3 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "SgnrId", skip_serializing_if = "Option::is_none")]
    pub sgnr_id: Option<Recipient5Choice>,
    #[validate]
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: AlgorithmIdentification16,
    #[validate]
    #[serde(rename = "SgntrAlgo")]
    pub sgntr_algo: AlgorithmIdentification17,
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
#[derive(
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
pub struct PostalAddress17 {
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[validate]
    #[serde(rename = "TwnNm")]
    pub twn_nm: Max35Text,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "CtrySubDvsn", default)]
    pub ctry_sub_dvsn: Vec<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[serde(rename = "GLctn", skip_serializing_if = "Option::is_none")]
    pub g_lctn: Option<GeographicLocation1Choice>,
}
#[derive(
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
pub enum Algorithm15Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
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
pub struct AtmKeyDownloadResponseV03 {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header31,
    #[serde(
        rename = "PrtctdATMKeyDwnldRspn",
        skip_serializing_if = "Option::is_none"
    )]
    pub prtctd_atm_key_dwnld_rspn: Option<ContentInformationType10>,
    #[serde(rename = "ATMKeyDwnldRspn", skip_serializing_if = "Option::is_none")]
    pub atm_key_dwnld_rspn: Option<AtmKeyDownloadResponse4>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TerminalHosting1 {
    #[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
    pub ctgy: Option<TransactionEnvironment3Code>,
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
pub struct AtmCommandParameters1ChoiceEnum {
    #[serde(rename = "XpctdMsgFctn", skip_serializing_if = "Option::is_none")]
    pub xpctd_msg_fctn: Option<MessageFunction8Code>,
    #[serde(rename = "ATMReqrdGblSts", skip_serializing_if = "Option::is_none")]
    pub atm_reqrd_gbl_sts: Option<AtmStatus1Code>,
    #[serde(rename = "ReqrdCfgtnParam", skip_serializing_if = "Option::is_none")]
    pub reqrd_cfgtn_param: Option<AtmConfigurationParameter1>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter6 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat1Code>,
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
pub struct AtmSecurityConfiguration2 {
    #[serde(rename = "MaxSmmtrcKey", skip_serializing_if = "Option::is_none")]
    pub max_smmtrc_key: Option<Number>,
    #[serde(rename = "MaxAsmmtrcKey", skip_serializing_if = "Option::is_none")]
    pub max_asmmtrc_key: Option<Number>,
    #[serde(rename = "MaxRSAKeyLngth", skip_serializing_if = "Option::is_none")]
    pub max_rsa_key_lngth: Option<Number>,
    #[serde(rename = "MaxRootKeyLngth", skip_serializing_if = "Option::is_none")]
    pub max_root_key_lngth: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification14 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm15Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter6>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PinFormat4Code {
    #[serde(rename = "ANSI")]
    Ansi,
    #[serde(rename = "BNCM")]
    Bncm,
    #[serde(rename = "BKSY")]
    Bksy,
    #[serde(rename = "DBLD")]
    Dbld,
    #[serde(rename = "DBLC")]
    Dblc,
    #[serde(rename = "ECI2")]
    Eci2,
    #[serde(rename = "ECI3")]
    Eci3,
    #[serde(rename = "EMVS")]
    Emvs,
    #[serde(rename = "IBM3")]
    Ibm3,
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
    #[serde(rename = "VIS2")]
    Vis2,
    #[serde(rename = "VIS3")]
    Vis3,
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
pub struct ContentInformationType14 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "SgndData")]
    pub sgnd_data: SignedData4,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Header31 {
    #[validate]
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: AtmMessageFunction2,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max3NumericText,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: Max35Text,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<Max35Text>,
    #[serde(rename = "PrcStat", skip_serializing_if = "Option::is_none")]
    pub prc_stat: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KeyTransport4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient5Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification11,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max5000Binary,
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
pub struct ContentInformationType13 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "AuthntcdData", skip_serializing_if = "Option::is_none")]
    pub authntcd_data: Option<AuthenticatedData4>,
    #[serde(rename = "SgndData", skip_serializing_if = "Option::is_none")]
    pub sgnd_data: Option<SignedData4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeographicLocation1ChoiceEnum {
    #[serde(rename = "UTMCordints", skip_serializing_if = "Option::is_none")]
    pub utm_cordints: Option<UtmCoordinates1>,
    #[serde(rename = "GeogcCordints", skip_serializing_if = "Option::is_none")]
    pub geogc_cordints: Option<GeographicCoordinates1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeographicLocation1Choice {
    #[serde(flatten)]
    pub value: GeographicLocation1ChoiceEnum,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PublicRsaKey1 {
    #[validate]
    #[serde(rename = "Mdlus")]
    pub mdlus: Max5000Binary,
    #[validate]
    #[serde(rename = "Expnt")]
    pub expnt: Max5000Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmSecurityContext3 {
    #[serde(rename = "CurSctySchme")]
    pub cur_scty_schme: AtmSecurityScheme3Code,
    #[serde(rename = "DvcPrprty", skip_serializing_if = "Option::is_none")]
    pub dvc_prprty: Option<AtmEquipment3>,
    #[serde(rename = "CurCfgtn", skip_serializing_if = "Option::is_none")]
    pub cur_cfgtn: Option<AtmSecurityConfiguration1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm14Code {
    #[serde(rename = "ERS2")]
    Ers2,
    #[serde(rename = "ERS1")]
    Ers1,
    #[serde(rename = "RPSS")]
    Rpss,
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
pub struct EncryptedContent3 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "CnttNcrptnAlgo")]
    pub cntt_ncrptn_algo: AlgorithmIdentification14,
    #[validate]
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: Max100KBinary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvelopedData4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient4Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent3>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutomatedTellerMachine3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35Text>,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<PostalAddress17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification11 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm7Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType10 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "EnvlpdData")]
    pub envlpd_data: EnvelopedData4,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AuthenticatedData4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient4Choice>,
    #[validate]
    #[serde(rename = "MACAlgo")]
    pub mac_algo: AlgorithmIdentification15,
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
pub struct AtmSecurityConfiguration4 {
    #[serde(rename = "MaxCerts", skip_serializing_if = "Option::is_none")]
    pub max_certs: Option<Number>,
    #[serde(rename = "MaxSgntrs", skip_serializing_if = "Option::is_none")]
    pub max_sgntrs: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DgtlSgntrAlgo", default)]
    pub dgtl_sgntr_algo: Vec<Algorithm14Code>,
}
#[derive(
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
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
pub enum Algorithm8Code {
    #[serde(rename = "MGF1")]
    Mgf1,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageProtection1Code {
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "MACB")]
    Macb,
    #[serde(rename = "MACM")]
    Macm,
    #[serde(rename = "UNPR")]
    Unpr,
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
pub struct AtmSecurityConfiguration1 {
    #[serde(rename = "Keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<AtmSecurityConfiguration2>,
    #[serde(rename = "Ncrptn", skip_serializing_if = "Option::is_none")]
    pub ncrptn: Option<AtmSecurityConfiguration3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MACAlgo", default)]
    pub mac_algo: Vec<Algorithm12Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DgstAlgo", default)]
    pub dgst_algo: Vec<Algorithm11Code>,
    #[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub dgtl_sgntr: Option<AtmSecurityConfiguration4>,
    #[serde(rename = "PIN", skip_serializing_if = "Option::is_none")]
    pub pin: Option<AtmSecurityConfiguration5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgPrtcn", default)]
    pub msg_prtcn: Vec<MessageProtection1Code>,
}
#[derive(
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
pub struct AtmConfigurationParameter1 {
    #[serde(rename = "Tp")]
    pub tp: DataSetCategory7Code,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptionFormat1Code {
    #[serde(rename = "TR31")]
    Tr31,
    #[serde(rename = "TR34")]
    Tr34,
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
pub struct KeyChoiceValue2Enum {
    #[serde(rename = "NcrptdKeyVal", skip_serializing_if = "Option::is_none")]
    pub ncrptd_key_val: Option<ContentInformationType10>,
    #[serde(rename = "TRRltdData", skip_serializing_if = "Option::is_none")]
    pub tr_rltd_data: Option<TrRelatedData2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KeyChoiceValue2 {
    #[serde(flatten)]
    pub value: KeyChoiceValue2Enum,
}
#[derive(
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
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber1>,
}
#[derive(
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
pub struct SecurityParameters10 {
    #[serde(rename = "HstChllng", skip_serializing_if = "Option::is_none")]
    pub hst_chllng: Option<Max140Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Key", default)]
    pub key: Vec<CryptographicKey12>,
    #[serde(rename = "SgntrChc", skip_serializing_if = "Option::is_none")]
    pub sgntr_chc: Option<AtmSignature2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification15 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm12Code,
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
pub struct AtmCommand13 {
    #[serde(rename = "Tp")]
    pub tp: AtmCommand6Code,
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
pub struct AtmEnvironment7 {
    #[serde(rename = "Acqrr", skip_serializing_if = "Option::is_none")]
    pub acqrr: Option<Acquirer7>,
    #[serde(rename = "ATMMgr", skip_serializing_if = "Option::is_none")]
    pub atm_mgr: Option<Acquirer8>,
    #[serde(rename = "HstgNtty", skip_serializing_if = "Option::is_none")]
    pub hstg_ntty: Option<TerminalHosting1>,
    #[validate]
    #[serde(rename = "ATM")]
    pub atm: AutomatedTellerMachine3,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm12Code {
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
pub struct Acquirer7 {
    #[serde(rename = "AcqrgInstn", skip_serializing_if = "Option::is_none")]
    pub acqrg_instn: Option<Max35Text>,
    #[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
    pub brnch: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Kek4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier2,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification13,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max500Binary,
}
#[derive(
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
pub struct AlgorithmIdentification13 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm13Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification16 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm11Code,
}
#[derive(
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
pub struct Acquirer8 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "ApplVrsn", skip_serializing_if = "Option::is_none")]
    pub appl_vrsn: Option<Max35Text>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Tr34Command1Code {
    #[serde(rename = "BIND")]
    Bind,
    #[serde(rename = "HILR")]
    Hilr,
    #[serde(rename = "HILU")]
    Hilu,
    #[serde(rename = "RBND")]
    Rbnd,
    #[serde(rename = "UBND")]
    Ubnd,
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
pub struct AlgorithmIdentification17 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm14Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UtmCoordinates1 {
    #[validate]
    #[serde(rename = "UTMZone")]
    pub utm_zone: Max16Text,
    #[validate]
    #[serde(rename = "UTMEstwrd")]
    pub utm_estwrd: Number,
    #[validate]
    #[serde(rename = "UTMNrthwrd")]
    pub utm_nrthwrd: Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrRelatedData2 {
    #[serde(rename = "TR34Cmd", skip_serializing_if = "Option::is_none")]
    pub tr_34_cmd: Option<Tr34Command1Code>,
    #[serde(rename = "TRBlck", skip_serializing_if = "Option::is_none")]
    pub tr_blck: Option<Max100KBinary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter4 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat1Code>,
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm11Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification12>,
}
#[derive(
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
pub struct SignedData4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "DgstAlgo", default)]
    pub dgst_algo: Vec<AlgorithmIdentification16>,
    #[validate]
    #[serde(rename = "NcpsltdCntt")]
    pub ncpsltd_cntt: EncapsulatedContent3,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cert", default)]
    pub cert: Vec<Max5000Binary>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Sgnr", default)]
    pub sgnr: Vec<Signer3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmSignature2ChoiceEnum {
    #[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub dgtl_sgntr: Option<ContentInformationType14>,
    #[serde(rename = "TRRltdData", skip_serializing_if = "Option::is_none")]
    pub tr_rltd_data: Option<TrRelatedData2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmSignature2Choice {
    #[serde(flatten)]
    pub value: AtmSignature2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionEnvironment3Code {
    #[serde(rename = "BRCH")]
    Brch,
    #[serde(rename = "MERC")]
    Merc,
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
pub struct AtmKeyDownloadResponse4 {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: AtmEnvironment7,
    #[validate]
    #[serde(rename = "ATMSctyCntxt")]
    pub atm_scty_cntxt: AtmSecurityContext3,
    #[serde(rename = "ATMChllng", skip_serializing_if = "Option::is_none")]
    pub atm_chllng: Option<Max140Binary>,
    #[validate]
    #[serde(rename = "HstSctyParams")]
    pub hst_scty_params: SecurityParameters10,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmd", default)]
    pub cmd: Vec<AtmCommand13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmSecurityConfiguration3 {
    #[serde(rename = "AsmmtrcNcrptn", skip_serializing_if = "Option::is_none")]
    pub asmmtrc_ncrptn: Option<TrueFalseIndicator>,
    #[serde(rename = "AsmmtrcKeyStdId", skip_serializing_if = "Option::is_none")]
    pub asmmtrc_key_std_id: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AsmmtrcNcrptnAlgo", default)]
    pub asmmtrc_ncrptn_algo: Vec<Algorithm7Code>,
    #[serde(rename = "SmmtrcTrnsprtKey", skip_serializing_if = "Option::is_none")]
    pub smmtrc_trnsprt_key: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SmmtrcTrnsprtKeyAlgo", default)]
    pub smmtrc_trnsprt_key_algo: Vec<Algorithm13Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SmmtrcNcrptnAlgo", default)]
    pub smmtrc_ncrptn_algo: Vec<Algorithm15Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NcrptnFrmt", default)]
    pub ncrptn_frmt: Vec<EncryptionFormat1Code>,
}
#[derive(
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
pub struct Recipient4ChoiceEnum {
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek4>,
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Recipient4Choice {
    #[serde(flatten)]
    pub value: Recipient4ChoiceEnum,
}
