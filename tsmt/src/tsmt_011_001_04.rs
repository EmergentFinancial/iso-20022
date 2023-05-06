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
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.011.001.04".to_string()
}

#[derive(
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
#[derive(
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "BaselnRpt")]
    pub baseln_rpt: BaselineReportV04,
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
pub struct BaselineReportV04 {
    #[validate]
    #[serde(rename = "RptId")]
    pub rpt_id: MessageIdentification1,
    #[serde(rename = "RltdMsgRef", skip_serializing_if = "Option::is_none")]
    pub rltd_msg_ref: Option<MessageIdentification1>,
    #[validate]
    #[serde(rename = "RptTp")]
    pub rpt_tp: ReportType2,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: SimpleIdentificationInformation,
    #[validate]
    #[serde(rename = "EstblishdBaselnId")]
    pub estblishd_baseln_id: DocumentIdentification6,
    #[validate]
    #[serde(rename = "TxSts")]
    pub tx_sts: TransactionStatus4,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "UsrTxRef", default)]
    pub usr_tx_ref: Vec<DocumentIdentification5>,
    #[validate]
    #[serde(rename = "Buyr")]
    pub buyr: PartyIdentification26,
    #[validate]
    #[serde(rename = "Sellr")]
    pub sellr: PartyIdentification26,
    #[validate]
    #[serde(rename = "BuyrBk")]
    pub buyr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "SellrBk")]
    pub sellr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "RptdLineItm")]
    pub rptd_line_itm: LineItem14,
    #[serde(rename = "ReqForActn", skip_serializing_if = "Option::is_none")]
    pub req_for_actn: Option<PendingActivity2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductIdentifier2 {
    #[serde(rename = "Tp")]
    pub tp: ProductIdentifier2Code,
    #[validate]
    #[serde(rename = "Idr")]
    pub idr: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionStatus4 {
    #[serde(rename = "Sts")]
    pub sts: BaselineStatus3Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicIdentification1 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: BicIdentifier,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCharacteristics1 {
    #[serde(rename = "Tp")]
    pub tp: ProductCharacteristics1Code,
    #[validate]
    #[serde(rename = "Chrtcs")]
    pub chrtcs: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure4Code {
    #[serde(rename = "KGM")]
    Kgm,
    #[serde(rename = "EA")]
    Ea,
    #[serde(rename = "LTN")]
    Ltn,
    #[serde(rename = "MTR")]
    Mtr,
    #[serde(rename = "INH")]
    Inh,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "GLI")]
    Gli,
    #[serde(rename = "GRM")]
    Grm,
    #[serde(rename = "CMT")]
    Cmt,
    #[serde(rename = "MTK")]
    Mtk,
    #[serde(rename = "FOT")]
    Fot,
    #[serde(rename = "1A")]
    X1A,
    #[serde(rename = "INK")]
    Ink,
    #[serde(rename = "FTK")]
    Ftk,
    #[serde(rename = "MIK")]
    Mik,
    #[serde(rename = "ONZ")]
    Onz,
    #[serde(rename = "PTI")]
    Pti,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "QTI")]
    Qti,
    #[serde(rename = "QT")]
    Qt,
    #[serde(rename = "GLL")]
    Gll,
    #[serde(rename = "MMT")]
    Mmt,
    #[serde(rename = "KTM")]
    Ktm,
    #[serde(rename = "YDK")]
    Ydk,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "CMK")]
    Cmk,
    #[serde(rename = "KMK")]
    Kmk,
    #[serde(rename = "MMQ")]
    Mmq,
    #[serde(rename = "CLT")]
    Clt,
    #[serde(rename = "LTR")]
    Ltr,
    #[serde(rename = "LBR")]
    Lbr,
    #[serde(rename = "STN")]
    Stn,
    #[serde(rename = "BLL")]
    Bll,
    #[serde(rename = "BX")]
    Bx,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "CT")]
    Ct,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "INQ")]
    Inq,
    #[serde(rename = "MTQ")]
    Mtq,
    #[serde(rename = "OZI")]
    Ozi,
    #[serde(rename = "OZA")]
    Oza,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "TNE")]
    Tne,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductCategory1Code {
    #[serde(rename = "HRTR")]
    Hrtr,
    #[serde(rename = "QOTA")]
    Qota,
    #[serde(rename = "PRGP")]
    Prgp,
    #[serde(rename = "LOBU")]
    Lobu,
    #[serde(rename = "GNDR")]
    Gndr,
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
pub struct ReportType2 {
    #[serde(rename = "Tp")]
    pub tp: ReportType2Code,
}
#[derive(
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
pub struct LineItem14 {
    #[validate(length(min = 1,))]
    #[serde(rename = "LineItmDtls", default)]
    pub line_itm_dtls: Vec<LineItemDetails12>,
    #[validate]
    #[serde(rename = "OrdrdLineItmsTtlAmt")]
    pub ordrd_line_itms_ttl_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "AccptdLineItmsTtlAmt")]
    pub accptd_line_itms_ttl_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "OutsdngLineItmsTtlAmt")]
    pub outsdng_line_itms_ttl_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "PdgLineItmsTtlAmt")]
    pub pdg_line_itms_ttl_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "OrdrdTtlNetAmt")]
    pub ordrd_ttl_net_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "AccptdTtlNetAmt")]
    pub accptd_ttl_net_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "OutsdngTtlNetAmt")]
    pub outsdng_ttl_net_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "PdgTtlNetAmt")]
    pub pdg_ttl_net_amt: CurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCategory1 {
    #[serde(rename = "Tp")]
    pub tp: ProductCategory1Code,
    #[validate]
    #[serde(rename = "Ctgy")]
    pub ctgy: Max35Text,
}
#[derive(
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
pub struct DocumentIdentification6 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
    #[serde(rename = "AmdmntSeqNb", skip_serializing_if = "Option::is_none")]
    pub amdmnt_seq_nb: Option<Max3NumericText>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportType2Code {
    #[serde(rename = "PREC")]
    Prec,
    #[serde(rename = "CURR")]
    Curr,
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
pub struct PercentageTolerance1 {
    #[validate]
    #[serde(rename = "PlusPct")]
    pub plus_pct: PercentageRate,
    #[validate]
    #[serde(rename = "MnsPct")]
    pub mns_pct: PercentageRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification26 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
    #[validate]
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: PostalAddress5,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductCharacteristics1Code {
    #[serde(rename = "BISP")]
    Bisp,
    #[serde(rename = "CHNR")]
    Chnr,
    #[serde(rename = "CLOR")]
    Clor,
    #[serde(rename = "EDSP")]
    Edsp,
    #[serde(rename = "ENNR")]
    Ennr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "ORCR")]
    Orcr,
    #[serde(rename = "PCTV")]
    Pctv,
    #[serde(rename = "SISP")]
    Sisp,
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "SZRG")]
    Szrg,
    #[serde(rename = "SPRM")]
    Sprm,
    #[serde(rename = "STOR")]
    Stor,
    #[serde(rename = "VINR")]
    Vinr,
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
pub struct PendingActivity2 {
    #[serde(rename = "Tp")]
    pub tp: Action2Code,
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
pub struct UnitOfMeasure3ChoiceEnum {
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
    #[serde(rename = "UnitOfMeasrCd", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr_cd: Option<UnitOfMeasure4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure3Choice {
    #[serde(flatten)]
    pub value: UnitOfMeasure3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCharacteristics1ChoiceEnum {
    #[serde(rename = "OthrPdctChrtcs", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_chrtcs: Option<GenericIdentification4>,
    #[serde(rename = "StrdPdctChrtcs", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_chrtcs: Option<ProductCharacteristics1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCharacteristics1Choice {
    #[serde(flatten)]
    pub value: ProductCharacteristics1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicIdentifier {
    #[validate(regex = "BIC_IDENTIFIER_REGEX")]
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
pub struct SimpleIdentificationInformation {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Action2Code {
    #[serde(rename = "SBTW")]
    Sbtw,
    #[serde(rename = "RSTW")]
    Rstw,
    #[serde(rename = "RSBS")]
    Rsbs,
    #[serde(rename = "ARDM")]
    Ardm,
    #[serde(rename = "ARCS")]
    Arcs,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "UPDT")]
    Updt,
    #[serde(rename = "SBDS")]
    Sbds,
    #[serde(rename = "ARBA")]
    Arba,
    #[serde(rename = "ARRO")]
    Arro,
    #[serde(rename = "CINR")]
    Cinr,
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
pub struct ProductIdentifier2ChoiceEnum {
    #[serde(rename = "StrdPdctIdr", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_idr: Option<ProductIdentifier2>,
    #[serde(rename = "OthrPdctIdr", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_idr: Option<GenericIdentification4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductIdentifier2Choice {
    #[serde(flatten)]
    pub value: ProductIdentifier2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LineItemDetails12 {
    #[validate]
    #[serde(rename = "LineItmId")]
    pub line_itm_id: Max70Text,
    #[serde(rename = "PdctNm", skip_serializing_if = "Option::is_none")]
    pub pdct_nm: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctIdr", default)]
    pub pdct_idr: Vec<ProductIdentifier2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctChrtcs", default)]
    pub pdct_chrtcs: Vec<ProductCharacteristics1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdctCtgy", default)]
    pub pdct_ctgy: Vec<ProductCategory1Choice>,
    #[validate]
    #[serde(rename = "OrdrdQty")]
    pub ordrd_qty: Quantity9,
    #[validate]
    #[serde(rename = "AccptdQty")]
    pub accptd_qty: Quantity9,
    #[validate]
    #[serde(rename = "OutsdngQty")]
    pub outsdng_qty: Quantity9,
    #[validate]
    #[serde(rename = "PdgQty")]
    pub pdg_qty: Quantity9,
    #[serde(rename = "QtyTlrnce", skip_serializing_if = "Option::is_none")]
    pub qty_tlrnce: Option<PercentageTolerance1>,
    #[validate]
    #[serde(rename = "OrdrdAmt")]
    pub ordrd_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "AccptdAmt")]
    pub accptd_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "OutsdngAmt")]
    pub outsdng_amt: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "PdgAmt")]
    pub pdg_amt: CurrencyAndAmount,
    #[serde(rename = "PricTlrnce", skip_serializing_if = "Option::is_none")]
    pub pric_tlrnce: Option<PercentageTolerance1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdIssr")]
    pub id_issr: BicIdentification1,
}
#[derive(
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
pub struct PostalAddress5 {
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "PstCdId", skip_serializing_if = "Option::is_none")]
    pub pst_cd_id: Option<Max16Text>,
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
pub struct ProductCategory1ChoiceEnum {
    #[serde(rename = "StrdPdctCtgy", skip_serializing_if = "Option::is_none")]
    pub strd_pdct_ctgy: Option<ProductCategory1>,
    #[serde(rename = "OthrPdctCtgy", skip_serializing_if = "Option::is_none")]
    pub othr_pdct_ctgy: Option<GenericIdentification4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductCategory1Choice {
    #[serde(flatten)]
    pub value: ProductCategory1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductIdentifier2Code {
    #[serde(rename = "BINR")]
    Binr,
    #[serde(rename = "COMD")]
    Comd,
    #[serde(rename = "EANC")]
    Eanc,
    #[serde(rename = "HRTR")]
    Hrtr,
    #[serde(rename = "MANI")]
    Mani,
    #[serde(rename = "MODL")]
    Modl,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "QOTA")]
    Qota,
    #[serde(rename = "STYL")]
    Styl,
    #[serde(rename = "SUPI")]
    Supi,
    #[serde(rename = "UPCC")]
    Upcc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BaselineStatus3Code {
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "CLSD")]
    Clsd,
    #[serde(rename = "PMTC")]
    Pmtc,
    #[serde(rename = "ESTD")]
    Estd,
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "AMRQ")]
    Amrq,
    #[serde(rename = "RARQ")]
    Rarq,
    #[serde(rename = "CLRQ")]
    Clrq,
    #[serde(rename = "SCRQ")]
    Scrq,
    #[serde(rename = "SERQ")]
    Serq,
    #[serde(rename = "DARQ")]
    Darq,
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
pub struct Quantity9 {
    #[serde(rename = "UnitOfMeasr")]
    pub unit_of_measr: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: DecimalNumber,
    #[serde(rename = "Fctr", skip_serializing_if = "Option::is_none")]
    pub fctr: Option<Max15NumericText>,
}
