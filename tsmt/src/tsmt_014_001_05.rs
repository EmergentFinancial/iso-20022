// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
// This software is dual-licensed under the MIT License OR the Apache License, Version 2.0.
//
// MIT License
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the “Software”),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_7_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{7}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.014.001.05".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LineItem15 {
    #[validate]
    #[serde(rename = "PurchsOrdrRef")]
    pub purchs_ordr_ref: DocumentIdentification7,
    #[validate]
    #[serde(rename = "FnlSubmissn")]
    pub fnl_submissn: YesNoIndicator,
    #[validate(length(min = 1,))]
    #[serde(rename = "ComrclLineItms", default)]
    pub comrcl_line_itms: Vec<LineItemDetails14>,
    #[validate]
    #[serde(rename = "LineItmsTtlAmt")]
    pub line_itms_ttl_amt: CurrencyAndAmount,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment6>,
    #[serde(rename = "FrghtChrgs", skip_serializing_if = "Option::is_none")]
    pub frght_chrgs: Option<Charge25>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax22>,
    #[validate]
    #[serde(rename = "TtlNetAmt")]
    pub ttl_net_amt: CurrencyAndAmount,
    #[validate(length(min = 0,))]
    #[serde(rename = "BuyrDfndInf", default)]
    pub buyr_dfnd_inf: Vec<UserDefinedInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SellrDfndInf", default)]
    pub sellr_dfnd_inf: Vec<UserDefinedInformation1>,
    #[serde(rename = "Incotrms", skip_serializing_if = "Option::is_none")]
    pub incotrms: Option<Incoterms4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeCertificateType1Code {
    #[serde(rename = "ANLY")]
    Anly,
    #[serde(rename = "QUAL")]
    Qual,
    #[serde(rename = "QUAN")]
    Quan,
    #[serde(rename = "WEIG")]
    Weig,
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "HEAL")]
    Heal,
    #[serde(rename = "PHYT")]
    Phyt,
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
pub struct DataSetSubmissionV05 {
    #[validate]
    #[serde(rename = "SubmissnId")]
    pub submissn_id: MessageIdentification1,
    #[validate(length(min = 1,))]
    #[serde(rename = "RltdTxRefs", default)]
    pub rltd_tx_refs: Vec<DataSetSubmissionReferences3>,
    #[validate]
    #[serde(rename = "CmonSubmissnRef")]
    pub cmon_submissn_ref: SimpleIdentificationInformation,
    #[validate]
    #[serde(rename = "Instr")]
    pub instr: InstructionType3,
    #[validate]
    #[serde(rename = "BuyrBk")]
    pub buyr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "SellrBk")]
    pub sellr_bk: BicIdentification1,
    #[serde(rename = "ComrclDataSet", skip_serializing_if = "Option::is_none")]
    pub comrcl_data_set: Option<CommercialDataSet5>,
    #[serde(rename = "TrnsprtDataSet", skip_serializing_if = "Option::is_none")]
    pub trnsprt_data_set: Option<TransportDataSet5>,
    #[serde(rename = "InsrncDataSet", skip_serializing_if = "Option::is_none")]
    pub insrnc_data_set: Option<InsuranceDataSet1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertDataSet", default)]
    pub cert_data_set: Vec<CertificateDataSet2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrCertDataSet", default)]
    pub othr_cert_data_set: Vec<OtherCertificateDataSet2>,
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
pub struct ProductCharacteristics1 {
    #[serde(rename = "Tp")]
    pub tp: ProductCharacteristics1Code,
    #[validate]
    #[serde(rename = "Chrtcs")]
    pub chrtcs: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificateDataSet2 {
    #[validate]
    #[serde(rename = "DataSetId")]
    pub data_set_id: DocumentIdentification1,
    #[serde(rename = "CertTp")]
    pub cert_tp: TradeCertificateType1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "LineItm", default)]
    pub line_itm: Vec<LineItemAndPoIdentification1>,
    #[serde(rename = "CertfdChrtcs")]
    pub certfd_chrtcs: CertifiedCharacteristics2Choice,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
    #[serde(rename = "PlcOfIsse", skip_serializing_if = "Option::is_none")]
    pub plc_of_isse: Option<PostalAddress5>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: PartyIdentification26,
    #[serde(rename = "InspctnDt", skip_serializing_if = "Option::is_none")]
    pub inspctn_dt: Option<DatePeriodDetails>,
    #[serde(rename = "AuthrsdInspctrInd", skip_serializing_if = "Option::is_none")]
    pub authrsd_inspctr_ind: Option<YesNoIndicator>,
    #[validate]
    #[serde(rename = "CertId")]
    pub cert_id: Max35Text,
    #[serde(rename = "Trnsprt", skip_serializing_if = "Option::is_none")]
    pub trnsprt: Option<SingleTransport3>,
    #[serde(rename = "GoodsDesc", skip_serializing_if = "Option::is_none")]
    pub goods_desc: Option<Max70Text>,
    #[serde(rename = "Consgnr", skip_serializing_if = "Option::is_none")]
    pub consgnr: Option<PartyIdentification26>,
    #[serde(rename = "Consgn", skip_serializing_if = "Option::is_none")]
    pub consgn: Option<PartyIdentification26>,
    #[serde(rename = "Manfctr", skip_serializing_if = "Option::is_none")]
    pub manfctr: Option<PartyIdentification26>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max350Text>,
}
#[derive(
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
pub struct PartyIdentification29ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<PartyIdentification26>,
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<BicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification29Choice {
    #[serde(flatten)]
    pub value: PartyIdentification29ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MultimodalTransport3 {
    #[validate]
    #[serde(rename = "TakngInChrg")]
    pub takng_in_chrg: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfFnlDstn")]
    pub plc_of_fnl_dstn: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargesType1ChoiceEnum {
    #[serde(rename = "OthrChrgsTp", skip_serializing_if = "Option::is_none")]
    pub othr_chrgs_tp: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ChargeType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargesType1Choice {
    #[serde(flatten)]
    pub value: ChargesType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTerms3 {
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<FinancialInstitutionIdentification4Choice>,
    #[validate]
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: CashAccount24,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportByRail2 {
    #[validate]
    #[serde(rename = "PlcOfRct")]
    pub plc_of_rct: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfDlvry")]
    pub plc_of_dlvry: Max35Text,
    #[serde(rename = "RailCrrierNm", skip_serializing_if = "Option::is_none")]
    pub rail_crrier_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitPrice18 {
    #[serde(rename = "UnitPric")]
    pub unit_pric: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
    #[serde(rename = "Fctr", skip_serializing_if = "Option::is_none")]
    pub fctr: Option<Max15NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
    #[validate]
    #[serde(rename = "Submitr")]
    pub submitr: BicIdentification1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InsuranceClauses1Code {
    #[serde(rename = "ICCA")]
    Icca,
    #[serde(rename = "ICCB")]
    Iccb,
    #[serde(rename = "ICCC")]
    Iccc,
    #[serde(rename = "ICAI")]
    Icai,
    #[serde(rename = "IWCC")]
    Iwcc,
    #[serde(rename = "ISCC")]
    Iscc,
    #[serde(rename = "IREC")]
    Irec,
    #[serde(rename = "ICLC")]
    Iclc,
    #[serde(rename = "ISMC")]
    Ismc,
    #[serde(rename = "CMCC")]
    Cmcc,
    #[serde(rename = "IRCE")]
    Irce,
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
pub struct TransportByRoad4 {
    #[validate]
    #[serde(rename = "PlcOfRct")]
    pub plc_of_rct: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfDlvry")]
    pub plc_of_dlvry: Max35Text,
    #[serde(rename = "RoadCrrierNm", skip_serializing_if = "Option::is_none")]
    pub road_crrier_nm: Option<Max70Text>,
    #[serde(rename = "RoadCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub road_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DataSetSubmissionReferences3 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[validate]
    #[serde(rename = "PurchsOrdrRef")]
    pub purchs_ordr_ref: DocumentIdentification7,
    #[serde(rename = "SubmitrTxRef", skip_serializing_if = "Option::is_none")]
    pub submitr_tx_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "ForcdMtch")]
    pub forcd_mtch: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Adjustment6 {
    #[serde(rename = "Tp")]
    pub tp: AdjustmentType1Choice,
    #[serde(rename = "Drctn")]
    pub drctn: AdjustmentDirection1Code,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SingleTransport3 {
    #[serde(rename = "TrnsprtByAir", skip_serializing_if = "Option::is_none")]
    pub trnsprt_by_air: Option<TransportByAir2>,
    #[serde(rename = "TrnsprtBySea", skip_serializing_if = "Option::is_none")]
    pub trnsprt_by_sea: Option<TransportBySea4>,
    #[serde(rename = "TrnsprtByRoad", skip_serializing_if = "Option::is_none")]
    pub trnsprt_by_road: Option<TransportByRoad2>,
    #[serde(rename = "TrnsprtByRail", skip_serializing_if = "Option::is_none")]
    pub trnsprt_by_rail: Option<TransportByRail2>,
}
#[derive(
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
pub struct LineItemDetails14 {
    #[validate]
    #[serde(rename = "LineItmId")]
    pub line_itm_id: Max70Text,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: Quantity9,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<UnitPrice18>,
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
    #[serde(rename = "PdctOrgn", skip_serializing_if = "Option::is_none")]
    pub pdct_orgn: Option<CountryCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Adjstmnt", default)]
    pub adjstmnt: Vec<Adjustment6>,
    #[serde(rename = "FrghtChrgs", skip_serializing_if = "Option::is_none")]
    pub frght_chrgs: Option<Charge25>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tax", default)]
    pub tax: Vec<Tax22>,
    #[validate]
    #[serde(rename = "TtlAmt")]
    pub ttl_amt: CurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InsuranceDataSet1 {
    #[validate]
    #[serde(rename = "DataSetId")]
    pub data_set_id: DocumentIdentification1,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: PartyIdentification26,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<IsoDate>,
    #[serde(rename = "PlcOfIsse", skip_serializing_if = "Option::is_none")]
    pub plc_of_isse: Option<PostalAddress5>,
    #[validate]
    #[serde(rename = "InsrncDocId")]
    pub insrnc_doc_id: Max35Text,
    #[serde(rename = "Trnsprt", skip_serializing_if = "Option::is_none")]
    pub trnsprt: Option<SingleTransport3>,
    #[validate]
    #[serde(rename = "InsrdAmt")]
    pub insrd_amt: CurrencyAndAmount,
    #[serde(rename = "InsrdGoodsDesc", skip_serializing_if = "Option::is_none")]
    pub insrd_goods_desc: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InsrncConds", default)]
    pub insrnc_conds: Vec<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InsrncClauses", default)]
    pub insrnc_clauses: Vec<InsuranceClauses1Code>,
    #[serde(rename = "Assrd")]
    pub assrd: PartyIdentification29Choice,
    #[validate]
    #[serde(rename = "ClmsPyblAt")]
    pub clms_pybl_at: PostalAddress5,
    #[serde(rename = "ClmsPyblIn", skip_serializing_if = "Option::is_none")]
    pub clms_pybl_in: Option<CurrencyCode>,
}
#[derive(
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
pub struct PaymentTerms4 {
    #[serde(rename = "PmtTerms")]
    pub pmt_terms: PaymentCodeOrOther1Choice,
    #[serde(rename = "AmtOrPctg")]
    pub amt_or_pctg: AmountOrPercentage2Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportByAir2 {
    #[serde(rename = "DprtureAirprt")]
    pub dprture_airprt: AirportName1Choice,
    #[serde(rename = "DstnAirprt")]
    pub dstn_airprt: AirportName1Choice,
    #[serde(rename = "AirCrrierNm", skip_serializing_if = "Option::is_none")]
    pub air_crrier_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportByAir4 {
    #[serde(rename = "DprtureAirprt")]
    pub dprture_airprt: AirportName1Choice,
    #[serde(rename = "DstnAirprt")]
    pub dstn_airprt: AirportName1Choice,
    #[serde(rename = "FlghtNb", skip_serializing_if = "Option::is_none")]
    pub flght_nb: Option<Max35Text>,
    #[serde(rename = "AirCrrierNm", skip_serializing_if = "Option::is_none")]
    pub air_crrier_nm: Option<Max70Text>,
    #[serde(rename = "AirCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub air_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
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
pub struct AirportName1ChoiceEnum {
    #[serde(rename = "AirprtCd", skip_serializing_if = "Option::is_none")]
    pub airprt_cd: Option<Max6Text>,
    #[serde(rename = "OthrAirprtDesc", skip_serializing_if = "Option::is_none")]
    pub othr_airprt_desc: Option<AirportDescription1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AirportName1Choice {
    #[serde(flatten)]
    pub value: AirportName1ChoiceEnum,
}
#[derive(
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
pub struct TransportMeans6 {
    #[validate]
    #[serde(rename = "IndvTrnsprt")]
    pub indv_trnsprt: SingleTransport8,
    #[serde(rename = "MltmdlTrnsprt", skip_serializing_if = "Option::is_none")]
    pub mltmdl_trnsprt: Option<MultimodalTransport3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iban2007Identifier {
    #[validate(regex = "IBAN_2007_IDENTIFIER_REGEX")]
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
pub struct ExternalAccountIdentification1Code {
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
pub struct InvoiceIdentification1 {
    #[validate]
    #[serde(rename = "InvcNb")]
    pub invc_nb: Max35Text,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
}
#[derive(
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
pub struct TaxType2ChoiceEnum {
    #[serde(rename = "OthrTaxTp", skip_serializing_if = "Option::is_none")]
    pub othr_tax_tp: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxType9Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType2Choice {
    #[serde(flatten)]
    pub value: TaxType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportByRoad2 {
    #[validate]
    #[serde(rename = "PlcOfRct")]
    pub plc_of_rct: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfDlvry")]
    pub plc_of_dlvry: Max35Text,
    #[serde(rename = "RoadCrrierNm", skip_serializing_if = "Option::is_none")]
    pub road_crrier_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdjustmentType1ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<AdjustmentType2Code>,
    #[serde(rename = "OthrAdjstmntTp", skip_serializing_if = "Option::is_none")]
    pub othr_adjstmnt_tp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdjustmentType1Choice {
    #[serde(flatten)]
    pub value: AdjustmentType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommercialDataSet5 {
    #[validate]
    #[serde(rename = "DataSetId")]
    pub data_set_id: DocumentIdentification1,
    #[validate]
    #[serde(rename = "ComrclDocRef")]
    pub comrcl_doc_ref: InvoiceIdentification1,
    #[validate]
    #[serde(rename = "Buyr")]
    pub buyr: PartyIdentification26,
    #[validate]
    #[serde(rename = "Sellr")]
    pub sellr: PartyIdentification26,
    #[serde(rename = "BllTo", skip_serializing_if = "Option::is_none")]
    pub bll_to: Option<PartyIdentification26>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Goods", default)]
    pub goods: Vec<LineItem15>,
    #[validate(length(min = 1,))]
    #[serde(rename = "PmtTerms", default)]
    pub pmt_terms: Vec<PaymentTerms4>,
    #[validate]
    #[serde(rename = "SttlmTerms")]
    pub sttlm_terms: SettlementTerms3,
}
#[derive(
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
pub struct Incoterms4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalIncoterms1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Incoterms4Choice {
    #[serde(flatten)]
    pub value: Incoterms4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccount24 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
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
pub struct AccountSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalAccountIdentification1Code>,
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
pub struct AccountSchemeName1Choice {
    #[serde(flatten)]
    pub value: AccountSchemeName1ChoiceEnum,
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
#[derive(
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
pub struct ChargesDetails4 {
    #[serde(rename = "ChrgsTp")]
    pub chrgs_tp: ChargesType1Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
}
#[derive(
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
pub struct LineItemAndPoIdentification1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "LineItmId", default)]
    pub line_itm_id: Vec<Max70Text>,
    #[validate]
    #[serde(rename = "PurchsOrdrRef")]
    pub purchs_ordr_ref: DocumentIdentification7,
}
#[derive(
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
pub struct InstructionType3 {
    #[serde(rename = "Tp")]
    pub tp: InstructionType3Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstructionType3Code {
    #[serde(rename = "MTCH")]
    Mtch,
    #[serde(rename = "PMTC")]
    Pmtc,
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
pub struct SimpleIdentificationInformation {
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
pub struct UserDefinedInformation1 {
    #[validate]
    #[serde(rename = "Labl")]
    pub labl: Max35Text,
    #[validate]
    #[serde(rename = "Inf")]
    pub inf: Max140Text,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Incoterms4 {
    #[serde(rename = "IncotrmsCd")]
    pub incotrms_cd: Incoterms4Choice,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<Max70Text>,
}
#[derive(
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
    #[serde(rename = "DataSetSubmissn")]
    pub data_set_submissn: DataSetSubmissionV05,
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
pub struct Quantity9 {
    #[serde(rename = "UnitOfMeasr")]
    pub unit_of_measr: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: DecimalNumber,
    #[serde(rename = "Fctr", skip_serializing_if = "Option::is_none")]
    pub fctr: Option<Max15NumericText>,
}
#[derive(
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
pub struct PaymentPeriod3 {
    #[serde(rename = "Cd")]
    pub cd: PaymentTime3Code,
    #[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
    pub nb_of_days: Option<Number>,
}
#[derive(
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
pub struct UnitOfMeasure3ChoiceEnum {
    #[serde(rename = "UnitOfMeasrCd", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr_cd: Option<UnitOfMeasure4Code>,
    #[serde(rename = "OthrUnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub othr_unit_of_measr: Option<Max35Text>,
}
#[derive(
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
pub struct Tax22 {
    #[serde(rename = "Tp")]
    pub tp: TaxType2Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeType8Code {
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "STDE")]
    Stde,
    #[serde(rename = "STOR")]
    Stor,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PICK")]
    Pick,
    #[serde(rename = "DNGR")]
    Dngr,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "COLF")]
    Colf,
    #[serde(rename = "CHOR")]
    Chor,
    #[serde(rename = "CHDE")]
    Chde,
    #[serde(rename = "AIRF")]
    Airf,
    #[serde(rename = "TRPT")]
    Trpt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FreightCharges1Code {
    #[serde(rename = "CLCT")]
    Clct,
    #[serde(rename = "PRPD")]
    Prpd,
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
pub struct TransportedGoods1 {
    #[validate]
    #[serde(rename = "PurchsOrdrRef")]
    pub purchs_ordr_ref: DocumentIdentification7,
    #[serde(rename = "GoodsDesc", skip_serializing_if = "Option::is_none")]
    pub goods_desc: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BuyrDfndInf", default)]
    pub buyr_dfnd_inf: Vec<UserDefinedInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SellrDfndInf", default)]
    pub sellr_dfnd_inf: Vec<UserDefinedInformation1>,
}
#[derive(
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
pub struct DatePeriodDetails {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericAccountIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice>,
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
pub struct NameAndAddress6 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress2,
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
pub struct ActiveCurrencyCode {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Max34Text {
    #[validate(length(min = 1, max = 34,))]
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
pub struct ShipmentDate1ChoiceEnum {
    #[serde(rename = "PropsdShipmntDt", skip_serializing_if = "Option::is_none")]
    pub propsd_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "ActlShipmntDt", skip_serializing_if = "Option::is_none")]
    pub actl_shipmnt_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentDate1Choice {
    #[serde(flatten)]
    pub value: ShipmentDate1ChoiceEnum,
}
#[derive(
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
pub struct TransportByRail4 {
    #[validate]
    #[serde(rename = "PlcOfRct")]
    pub plc_of_rct: Max35Text,
    #[validate]
    #[serde(rename = "PlcOfDlvry")]
    pub plc_of_dlvry: Max35Text,
    #[serde(rename = "RailCrrierNm", skip_serializing_if = "Option::is_none")]
    pub rail_crrier_nm: Option<Max70Text>,
    #[serde(rename = "RailCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub rail_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification4ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<BicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification4Choice {
    #[serde(flatten)]
    pub value: FinancialInstitutionIdentification4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentTime3Code {
    #[serde(rename = "EMTD")]
    Emtd,
    #[serde(rename = "EMTR")]
    Emtr,
    #[serde(rename = "EPBE")]
    Epbe,
    #[serde(rename = "EPRD")]
    Eprd,
    #[serde(rename = "PRMD")]
    Prmd,
    #[serde(rename = "PRMR")]
    Prmr,
    #[serde(rename = "EPIN")]
    Epin,
    #[serde(rename = "EPAM")]
    Epam,
    #[serde(rename = "EPPO")]
    Eppo,
    #[serde(rename = "EPRR")]
    Eprr,
    #[serde(rename = "EPSD")]
    Epsd,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "IREC")]
    Irec,
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
pub struct SingleTransport8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByAir", default)]
    pub trnsprt_by_air: Vec<TransportByAir4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtBySea", default)]
    pub trnsprt_by_sea: Vec<TransportBySea5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByRoad", default)]
    pub trnsprt_by_road: Vec<TransportByRoad4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrnsprtByRail", default)]
    pub trnsprt_by_rail: Vec<TransportByRail4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "DtOfIsse")]
    pub dt_of_isse: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertifiedCharacteristics2ChoiceEnum {
    #[serde(rename = "Orgn", skip_serializing_if = "Option::is_none")]
    pub orgn: Option<CountryCode>,
    #[serde(rename = "Wght", skip_serializing_if = "Option::is_none")]
    pub wght: Option<Quantity9>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<Quantity9>,
    #[serde(rename = "PhytosntryIndctn", skip_serializing_if = "Option::is_none")]
    pub phytosntry_indctn: Option<YesNoIndicator>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<Max70Text>,
    #[serde(rename = "Anlys", skip_serializing_if = "Option::is_none")]
    pub anlys: Option<Max70Text>,
    #[serde(rename = "HlthIndctn", skip_serializing_if = "Option::is_none")]
    pub hlth_indctn: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertifiedCharacteristics2Choice {
    #[serde(flatten)]
    pub value: CertifiedCharacteristics2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportDetails4 {
    #[validate(length(min = 1,))]
    #[serde(rename = "TrnsprtDocRef", default)]
    pub trnsprt_doc_ref: Vec<DocumentIdentification7>,
    #[validate(length(min = 1,))]
    #[serde(rename = "TrnsprtdGoods", default)]
    pub trnsprtd_goods: Vec<TransportedGoods1>,
    #[serde(rename = "Consgnmt", skip_serializing_if = "Option::is_none")]
    pub consgnmt: Option<Consignment3>,
    #[validate]
    #[serde(rename = "RtgSummry")]
    pub rtg_summry: TransportMeans6,
    #[serde(rename = "ShipmntDt")]
    pub shipmnt_dt: ShipmentDate1Choice,
    #[serde(rename = "FrghtChrgs", skip_serializing_if = "Option::is_none")]
    pub frght_chrgs: Option<Charge25>,
    #[serde(rename = "Incotrms", skip_serializing_if = "Option::is_none")]
    pub incotrms: Option<Incoterms4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdjustmentDirection1Code {
    #[serde(rename = "ADDD")]
    Addd,
    #[serde(rename = "SUBS")]
    Subs,
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
pub struct Quantity10 {
    #[serde(rename = "UnitOfMeasr")]
    pub unit_of_measr: UnitOfMeasure3Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: DecimalNumber,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdjustmentType2Code {
    #[serde(rename = "REBA")]
    Reba,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "SURC")]
    Surc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxType9Code {
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "NATI")]
    Nati,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "VATA")]
    Vata,
    #[serde(rename = "CUST")]
    Cust,
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
pub struct TransportBySea4 {
    #[validate]
    #[serde(rename = "PortOfLoadng")]
    pub port_of_loadng: Max35Text,
    #[validate]
    #[serde(rename = "PortOfDschrge")]
    pub port_of_dschrge: Max35Text,
    #[serde(rename = "VsslNm", skip_serializing_if = "Option::is_none")]
    pub vssl_nm: Option<Max35Text>,
    #[serde(rename = "SeaCrrierNm", skip_serializing_if = "Option::is_none")]
    pub sea_crrier_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalIncoterms1Code {
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
pub struct TransportBySea5 {
    #[validate]
    #[serde(rename = "PortOfLoadng")]
    pub port_of_loadng: Max35Text,
    #[validate]
    #[serde(rename = "PortOfDschrge")]
    pub port_of_dschrge: Max35Text,
    #[serde(rename = "VsslNm", skip_serializing_if = "Option::is_none")]
    pub vssl_nm: Option<Max70Text>,
    #[serde(rename = "SeaCrrierNm", skip_serializing_if = "Option::is_none")]
    pub sea_crrier_nm: Option<Max70Text>,
    #[serde(rename = "SeaCrrierCtry", skip_serializing_if = "Option::is_none")]
    pub sea_crrier_ctry: Option<CountryCode>,
    #[serde(rename = "CrrierAgtNm", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_nm: Option<Max70Text>,
    #[serde(rename = "CrrierAgtCtry", skip_serializing_if = "Option::is_none")]
    pub crrier_agt_ctry: Option<CountryCode>,
    #[serde(rename = "MstrNm", skip_serializing_if = "Option::is_none")]
    pub mstr_nm: Option<Max70Text>,
    #[serde(rename = "ChrtrrNm", skip_serializing_if = "Option::is_none")]
    pub chrtrr_nm: Option<Max70Text>,
    #[serde(rename = "OwnrNm", skip_serializing_if = "Option::is_none")]
    pub ownr_nm: Option<Max70Text>,
    #[serde(rename = "IMONb", skip_serializing_if = "Option::is_none")]
    pub imo_nb: Option<Exact7NumericText>,
    #[serde(rename = "VygNb", skip_serializing_if = "Option::is_none")]
    pub vyg_nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Consignment3 {
    #[serde(rename = "TtlQty", skip_serializing_if = "Option::is_none")]
    pub ttl_qty: Option<Quantity10>,
    #[serde(rename = "TtlVol", skip_serializing_if = "Option::is_none")]
    pub ttl_vol: Option<Quantity10>,
    #[serde(rename = "TtlWght", skip_serializing_if = "Option::is_none")]
    pub ttl_wght: Option<Quantity10>,
}
#[derive(
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
pub struct AccountIdentification4ChoiceEnum {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4Choice {
    #[serde(flatten)]
    pub value: AccountIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct ExternalCashAccountType1Code {
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
pub struct AirportDescription1 {
    #[validate]
    #[serde(rename = "Twn")]
    pub twn: Max35Text,
    #[serde(rename = "AirprtNm", skip_serializing_if = "Option::is_none")]
    pub airprt_nm: Option<Max35Text>,
}
#[derive(
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
pub struct OtherCertificateDataSet2 {
    #[validate]
    #[serde(rename = "DataSetId")]
    pub data_set_id: DocumentIdentification1,
    #[validate]
    #[serde(rename = "CertId")]
    pub cert_id: Max35Text,
    #[validate]
    #[serde(rename = "CertTp")]
    pub cert_tp: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: PartyIdentification26,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertInf", default)]
    pub cert_inf: Vec<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalCashAccountType1Code>,
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
pub struct CashAccountType2Choice {
    #[serde(flatten)]
    pub value: CashAccountType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentage2ChoiceEnum {
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentage2Choice {
    #[serde(flatten)]
    pub value: AmountOrPercentage2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact7NumericText {
    #[validate(regex = "EXACT_7_NUMERIC_TEXT_REGEX")]
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
pub struct Charge25 {
    #[serde(rename = "Tp")]
    pub tp: FreightCharges1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "Chrgs", default)]
    pub chrgs: Vec<ChargesDetails4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransportDataSet5 {
    #[validate]
    #[serde(rename = "DataSetId")]
    pub data_set_id: DocumentIdentification1,
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<PartyIdentification26>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<PartyIdentification26>,
    #[validate]
    #[serde(rename = "Consgnr")]
    pub consgnr: PartyIdentification26,
    #[serde(rename = "Consgn", skip_serializing_if = "Option::is_none")]
    pub consgn: Option<PartyIdentification26>,
    #[serde(rename = "ShipTo", skip_serializing_if = "Option::is_none")]
    pub ship_to: Option<PartyIdentification26>,
    #[validate]
    #[serde(rename = "TrnsprtInf")]
    pub trnsprt_inf: TransportDetails4,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCodeOrOther1ChoiceEnum {
    #[serde(rename = "PmtDueDt", skip_serializing_if = "Option::is_none")]
    pub pmt_due_dt: Option<IsoDate>,
    #[serde(rename = "PmtCd", skip_serializing_if = "Option::is_none")]
    pub pmt_cd: Option<PaymentPeriod3>,
    #[serde(rename = "OthrPmtTerms", skip_serializing_if = "Option::is_none")]
    pub othr_pmt_terms: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCodeOrOther1Choice {
    #[serde(flatten)]
    pub value: PaymentCodeOrOther1ChoiceEnum,
}
#[derive(
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
