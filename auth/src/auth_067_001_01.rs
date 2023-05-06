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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.067.001.01".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType16Code {
    #[serde(rename = "FXCR")]
    Fxcr,
    #[serde(rename = "FXEM")]
    Fxem,
    #[serde(rename = "FXMJ")]
    Fxmj,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "FOIL")]
    Foil,
    #[serde(rename = "GOIL")]
    Goil,
    #[serde(rename = "GSLN")]
    Gsln,
    #[serde(rename = "GASP")]
    Gasp,
    #[serde(rename = "HEAT")]
    Heat,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "JTFL")]
    Jtfl,
    #[serde(rename = "KERO")]
    Kero,
    #[serde(rename = "LAMP")]
    Lamp,
    #[serde(rename = "LEAD")]
    Lead,
    #[serde(rename = "LLSO")]
    Llso,
    #[serde(rename = "LNGG")]
    Lngg,
    #[serde(rename = "CORN")]
    Corn,
    #[serde(rename = "MARS")]
    Mars,
    #[serde(rename = "MWHT")]
    Mwht,
    #[serde(rename = "MOLY")]
    Moly,
    #[serde(rename = "NAPH")]
    Naph,
    #[serde(rename = "NBPG")]
    Nbpg,
    #[serde(rename = "NASC")]
    Nasc,
    #[serde(rename = "NCGG")]
    Ncgg,
    #[serde(rename = "NGLO")]
    Nglo,
    #[serde(rename = "NICK")]
    Nick,
    #[serde(rename = "OFFP")]
    Offp,
    #[serde(rename = "ALUM")]
    Alum,
    #[serde(rename = "ALUA")]
    Alua,
    #[serde(rename = "BAKK")]
    Bakk,
    #[serde(rename = "BSLD")]
    Bsld,
    #[serde(rename = "BDSL")]
    Bdsl,
    #[serde(rename = "BRNT")]
    Brnt,
    #[serde(rename = "BRNX")]
    Brnx,
    #[serde(rename = "CNDA")]
    Cnda,
    #[serde(rename = "CERE")]
    Cere,
    #[serde(rename = "CBLT")]
    Cblt,
    #[serde(rename = "CCOA")]
    Ccoa,
    #[serde(rename = "COND")]
    Cond,
    #[serde(rename = "CSHP")]
    Cshp,
    #[serde(rename = "COPR")]
    Copr,
    #[serde(rename = "DSEL")]
    Dsel,
    #[serde(rename = "DBCR")]
    Dbcr,
    #[serde(rename = "DUBA")]
    Duba,
    #[serde(rename = "ERUE")]
    Erue,
    #[serde(rename = "ESPO")]
    Espo,
    #[serde(rename = "ETHA")]
    Etha,
    #[serde(rename = "EUAE")]
    Euae,
    #[serde(rename = "EUAA")]
    Euaa,
    #[serde(rename = "FWHT")]
    Fwht,
    #[serde(rename = "FITR")]
    Fitr,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PLDM")]
    Pldm,
    #[serde(rename = "PKLD")]
    Pkld,
    #[serde(rename = "PTNM")]
    Ptnm,
    #[serde(rename = "POTA")]
    Pota,
    #[serde(rename = "RPSD")]
    Rpsd,
    #[serde(rename = "BRWN")]
    Brwn,
    #[serde(rename = "RICE")]
    Rice,
    #[serde(rename = "ROBU")]
    Robu,
    #[serde(rename = "SLVR")]
    Slvr,
    #[serde(rename = "SOYB")]
    Soyb,
    #[serde(rename = "STEL")]
    Stel,
    #[serde(rename = "TNKR")]
    Tnkr,
    #[serde(rename = "TAPI")]
    Tapi,
    #[serde(rename = "TINN")]
    Tinn,
    #[serde(rename = "TTFG")]
    Ttfg,
    #[serde(rename = "URAL")]
    Ural,
    #[serde(rename = "WHSG")]
    Whsg,
    #[serde(rename = "WTIO")]
    Wtio,
    #[serde(rename = "ZINC")]
    Zinc,
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
    #[validate]
    #[serde(rename = "CCPCollRpt")]
    pub ccp_coll_rpt: CcpCollateralReportV01<A>,
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
pub struct ActiveCurrencyAnd24Amount {
    #[serde(rename = "ActiveCurrencyAnd24Amount")]
    pub value: ActiveCurrencyAnd24AmountSimpleType,
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
pub struct AssetClassDetailedSubProductType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AssetClassDetailedSubProductType16Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassDetailedSubProductType1Choice {
    #[serde(flatten)]
    pub value: AssetClassDetailedSubProductType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralAccount4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification165,
    #[validate(length(min = 1,))]
    #[serde(rename = "AsstHldg", default)]
    pub asst_hldg: Vec<AssetHolding1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Guarantee1 {
    #[serde(rename = "Prvdr")]
    pub prvdr: PartyIdentification118Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}
#[derive(
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
pub struct AssetHolding1 {
    #[validate]
    #[serde(rename = "PstHrcutVal")]
    pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
    #[serde(rename = "AsstTp")]
    pub asst_tp: AssetHolding1Choice,
    #[serde(rename = "CollRqrmnt")]
    pub coll_rqrmnt: CollateralAccountType3Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductType7Code {
    #[serde(rename = "SVGN")]
    Svgn,
    #[serde(rename = "EQUI")]
    Equi,
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
pub struct GenericIdentification165 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<SchemeIdentificationType1Code>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralAccountType3Code {
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
pub struct SecurityIdentificationAndAmount1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: IsinOct2015Identifier,
    #[validate]
    #[serde(rename = "MktVal")]
    pub mkt_val: ActiveCurrencyAnd24Amount,
    #[serde(rename = "FinInstrmTp")]
    pub fin_instrm_tp: ProductType7Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetHolding1ChoiceEnum {
    #[serde(rename = "Gold", skip_serializing_if = "Option::is_none")]
    pub gold: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
    pub csh: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<SecurityIdentificationAndAmount1>,
    #[serde(rename = "Trpty", skip_serializing_if = "Option::is_none")]
    pub trpty: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Grnt", skip_serializing_if = "Option::is_none")]
    pub grnt: Option<Guarantee1>,
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<Commodity2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetHolding1Choice {
    #[serde(flatten)]
    pub value: AssetHolding1ChoiceEnum,
}
#[derive(
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
pub struct Commodity2 {
    #[validate]
    #[serde(rename = "MktVal")]
    pub mkt_val: ActiveCurrencyAnd24Amount,
    #[serde(rename = "CmmdtyTp")]
    pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}
#[derive(
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
pub struct LeiIdentifier {
    #[validate(regex = "LEI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SchemeIdentificationType1Code {
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "POSI")]
    Posi,
    #[serde(rename = "CLIM")]
    Clim,
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
pub struct CcpCollateralReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 1,))]
    #[serde(rename = "CollAcctOwnr", default)]
    pub coll_acct_ownr: Vec<CollateralAccount4>,
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
pub struct GenericIdentification168 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
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
pub struct PartyIdentification118ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification168>,
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
pub struct PartyIdentification118Choice {
    #[serde(flatten)]
    pub value: PartyIdentification118ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
    #[serde(rename = "$text")]
    pub value: String,
}
