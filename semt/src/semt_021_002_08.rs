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
    static ref RESTRICTED_FINX_MAX_70_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,70}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_140_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,140}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_31_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,31}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_EXACT_2_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"XX|TS"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_35_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_30_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:semt.021.002.08".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber14 {
    #[serde(rename = "Nb")]
    pub nb: DocumentNumber6Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStage4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStage2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStage4Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventStage4ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RegistrationProcessingStatus1Code {
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "REJT")]
    Rejt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesStatementType1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ACCT")]
    Acct,
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
pub struct StatementBasis9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<StatementBasis1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatementBasis9Choice {
    #[serde(flatten)]
    pub value: StatementBasis9ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnmatchedReason14Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "EXEC")]
    Exec,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "MCAN")]
    Mcan,
    #[serde(rename = "MIME")]
    Mime,
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "DTRA")]
    Dtra,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "FRAP")]
    Frap,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLIS")]
    Plis,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "REGD")]
    Regd,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "DEAL")]
    Deal,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "UNBR")]
    Unbr,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "DCMX")]
    Dcmx,
    #[serde(rename = "DMCT")]
    Dmct,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancellationProcessingStatus1Code {
    #[serde(rename = "CAND")]
    Cand,
    #[serde(rename = "CANP")]
    Canp,
    #[serde(rename = "DEND")]
    Dend,
    #[serde(rename = "EXCH")]
    Exch,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PARF")]
    Parf,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "REPR")]
    Repr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepairReason6Code {
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "BUSE")]
    Buse,
    #[serde(rename = "COMC")]
    Comc,
    #[serde(rename = "FEEE")]
    Feee,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "NRGM")]
    Nrgm,
    #[serde(rename = "NRGN")]
    Nrgn,
    #[serde(rename = "ULNK")]
    Ulnk,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeniedReason7Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DCAL")]
    Dcal,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "DFOR")]
    Dfor,
    #[serde(rename = "DREP")]
    Drep,
    #[serde(rename = "DSET")]
    Dset,
    #[serde(rename = "IPNC")]
    Ipnc,
    #[serde(rename = "LATE")]
    Late,
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
pub struct CorporateActionEventProcessingStatus4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventProcessingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventProcessingStatus4Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingStatus4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax70Text {
    #[validate(
        length(min = 1, max = 70,),
        regex = "RESTRICTED_FINX_MAX_70_TEXT_REGEX"
    )]
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
pub struct SecuritiesAccount30 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification47>,
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
pub struct GeneratedReasons6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GeneratedReason3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneratedReasons6Choice {
    #[serde(flatten)]
    pub value: GeneratedReasons6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification86 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax30Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AllocationStatus1Code {
    #[serde(rename = "AOLF")]
    Aolf,
    #[serde(rename = "AOLP")]
    Aolp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstructionProcessingStatus1Code {
    #[serde(rename = "CAN1")]
    Can1,
    #[serde(rename = "CAN2")]
    Can2,
    #[serde(rename = "CAN3")]
    Can3,
    #[serde(rename = "CAND")]
    Cand,
    #[serde(rename = "CANO")]
    Cano,
    #[serde(rename = "CANP")]
    Canp,
    #[serde(rename = "CGEN")]
    Cgen,
    #[serde(rename = "COSE")]
    Cose,
    #[serde(rename = "CPRC")]
    Cprc,
    #[serde(rename = "DFLA")]
    Dfla,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "DONF")]
    Donf,
    #[serde(rename = "EXCH")]
    Exch,
    #[serde(rename = "EXSE")]
    Exse,
    #[serde(rename = "FORC")]
    Forc,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "NOTC")]
    Notc,
    #[serde(rename = "OPOD")]
    Opod,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PAFI")]
    Pafi,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "PPRC")]
    Pprc,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "REPR")]
    Repr,
    #[serde(rename = "SESE")]
    Sese,
    #[serde(rename = "STIN")]
    Stin,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "UNDE")]
    Unde,
    #[serde(rename = "MPRC")]
    Mprc,
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
pub struct SecuritiesStatementQuery002V08<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "StmtReqd")]
    pub stmt_reqd: DocumentNumber14,
    #[serde(rename = "StmtGnlDtls", skip_serializing_if = "Option::is_none")]
    pub stmt_gnl_dtls: Option<Statement84>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlQryParams", default)]
    pub addtl_qry_params: Vec<AdditionalQueryParameters14>,
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
pub struct UnmatchedReason29ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnmatchedReason14Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason29Choice {
    #[serde(flatten)]
    pub value: UnmatchedReason29ChoiceEnum,
}
#[derive(
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
pub struct DeniedReason23ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeniedReason7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedReason23Choice {
    #[serde(flatten)]
    pub value: DeniedReason23ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber6ChoiceEnum {
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification86>,
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber6Choice {
    #[serde(flatten)]
    pub value: DocumentNumber6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason7Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DQUA")]
    Dqua,
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
pub struct Period2 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResponseStatus1Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "SUBR")]
    Subr,
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
pub struct Period7ChoiceEnum {
    #[serde(rename = "FrDtTmToDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm_to_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt_to_dt: Option<Period2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period7Choice {
    #[serde(flatten)]
    pub value: Period7ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification136ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification136Choice {
    #[serde(flatten)]
    pub value: PartyIdentification136ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason8Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "TAMM")]
    Tamm,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "MCER")]
    Mcer,
    #[serde(rename = "NPAY")]
    Npay,
    #[serde(rename = "NSEC")]
    Nsec,
    #[serde(rename = "PENR")]
    Penr,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "VLDA")]
    Vlda,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IPNC")]
    Ipnc,
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
pub struct IdentificationSource4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<RestrictedFinExact2Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource4Choice {
    #[serde(flatten)]
    pub value: IdentificationSource4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason18ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepairReason6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason18Choice {
    #[serde(flatten)]
    pub value: RepairReason18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResponseStatus7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ResponseStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResponseStatus7Choice {
    #[serde(flatten)]
    pub value: ResponseStatus7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingCancellationReasons5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingCancellationReasons5Choice {
    #[serde(flatten)]
    pub value: PendingCancellationReasons5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification47 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason15ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FailingReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason15Choice {
    #[serde(flatten)]
    pub value: FailingReason15ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventProcessingStatus1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "PEND")]
    Pend,
    #[serde(rename = "RECD")]
    Recd,
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
pub struct DateTimePeriod1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoCallAcknowledgementReason2Code {
    #[serde(rename = "CALD")]
    Cald,
    #[serde(rename = "CALP")]
    Calp,
    #[serde(rename = "ADEA")]
    Adea,
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
pub struct RepoCallRequestStatus10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepoCallRequestStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepoCallRequestStatus10Choice {
    #[serde(flatten)]
    pub value: RepoCallRequestStatus10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason47ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason8Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason47Choice {
    #[serde(flatten)]
    pub value: PendingReason47ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdateType16ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<StatementUpdateType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdateType16Choice {
    #[serde(flatten)]
    pub value: UpdateType16ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency26ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency26Choice {
    #[serde(flatten)]
    pub value: Frequency26ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason13ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingProcessingReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason13Choice {
    #[serde(flatten)]
    pub value: PendingProcessingReason13ChoiceEnum,
}
#[derive(
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
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
pub struct StatementType6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesStatementType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatementType6Choice {
    #[serde(flatten)]
    pub value: StatementType6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementConditionModificationStatus4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementConditionModificationStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementConditionModificationStatus4Choice {
    #[serde(flatten)]
    pub value: SettlementConditionModificationStatus4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegistrationProcessingStatus4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RegistrationProcessingStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegistrationProcessingStatus4Choice {
    #[serde(flatten)]
    pub value: RegistrationProcessingStatus4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason76Code {
    #[serde(rename = "ULNK")]
    Ulnk,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DADR")]
    Dadr,
    #[serde(rename = "INIR")]
    Inir,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INTV")]
    Intv,
    #[serde(rename = "INVA")]
    Inva,
    #[serde(rename = "REFT")]
    Reft,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "NMTY")]
    Nmty,
    #[serde(rename = "OPNM")]
    Opnm,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "OPTY")]
    Opty,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "EVNM")]
    Evnm,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "NRGM")]
    Nrgm,
    #[serde(rename = "MLEG")]
    Mleg,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "OWNT")]
    Ownt,
    #[serde(rename = "NRGN")]
    Nrgn,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "REQW")]
    Reqw,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "RREA")]
    Rrea,
    #[serde(rename = "REQM")]
    Reqm,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "ICOL")]
    Icol,
    #[serde(rename = "BPAR")]
    Bpar,
    #[serde(rename = "BREF")]
    Bref,
    #[serde(rename = "BUSE")]
    Buse,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "COMC")]
    Comc,
    #[serde(rename = "CONL")]
    Conl,
    #[serde(rename = "CPTY")]
    Cpty,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "DISE")]
    Dise,
    #[serde(rename = "DORD")]
    Dord,
    #[serde(rename = "FEEE")]
    Feee,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "PLIS")]
    Plis,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReplacementProcessingStatus1Code {
    #[serde(rename = "DEND")]
    Dend,
    #[serde(rename = "EXCH")]
    Exch,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "PEND")]
    Pend,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "REPL")]
    Repl,
    #[serde(rename = "REPR")]
    Repr,
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
pub struct CancellationReason30ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason12Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason30Choice {
    #[serde(flatten)]
    pub value: CancellationReason30ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MatchingStatus1Code {
    #[serde(rename = "MACH")]
    Mach,
    #[serde(rename = "NMAT")]
    Nmat,
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
pub struct RestrictedFinxMax140Text {
    #[validate(
        length(min = 1, max = 140,),
        regex = "RESTRICTED_FINX_MAX_140_TEXT_REGEX"
    )]
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
pub struct SecurityIdentification20 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification2>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<RestrictedFinxMax140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Statement84 {
    #[serde(rename = "StmtDtOrPrd", skip_serializing_if = "Option::is_none")]
    pub stmt_dt_or_prd: Option<DateAndPeriod3Choice>,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<Frequency26Choice>,
    #[serde(rename = "UpdTp", skip_serializing_if = "Option::is_none")]
    pub upd_tp: Option<UpdateType16Choice>,
    #[serde(rename = "StmtBsis", skip_serializing_if = "Option::is_none")]
    pub stmt_bsis: Option<StatementBasis9Choice>,
    #[serde(rename = "StmtTp", skip_serializing_if = "Option::is_none")]
    pub stmt_tp: Option<StatementType6Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus28ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MatchingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus28Choice {
    #[serde(flatten)]
    pub value: MatchingStatus28ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax31Text {
    #[validate(
        length(min = 1, max = 31,),
        regex = "RESTRICTED_FINX_MAX_31_TEXT_REGEX"
    )]
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
pub struct Iso20022MessageIdentificationText {
    #[validate(regex = "ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoCallRequestStatus1Code {
    #[serde(rename = "CACK")]
    Cack,
    #[serde(rename = "DEND")]
    Dend,
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
pub struct AcknowledgementReason18ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepoCallAcknowledgementReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason18Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinExact2Text {
    #[validate(regex = "RESTRICTED_FIN_EXACT_2_TEXT_REGEX")]
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
pub struct Reason20ChoiceEnum {
    #[serde(rename = "FlngRsn", skip_serializing_if = "Option::is_none")]
    pub flng_rsn: Option<FailingReason15Choice>,
    #[serde(rename = "PdgPrcgRsn", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg_rsn: Option<PendingProcessingReason13Choice>,
    #[serde(rename = "RprRsn", skip_serializing_if = "Option::is_none")]
    pub rpr_rsn: Option<RepairReason18Choice>,
    #[serde(rename = "PdgCxlRsn", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl_rsn: Option<PendingCancellationReasons5Choice>,
    #[serde(rename = "RepoCallAckRsn", skip_serializing_if = "Option::is_none")]
    pub repo_call_ack_rsn: Option<AcknowledgementReason18Choice>,
    #[serde(rename = "PdgRsn", skip_serializing_if = "Option::is_none")]
    pub pdg_rsn: Option<PendingReason47Choice>,
    #[serde(rename = "GnrtdRsn", skip_serializing_if = "Option::is_none")]
    pub gnrtd_rsn: Option<GeneratedReasons6Choice>,
    #[serde(rename = "PdgModRsn", skip_serializing_if = "Option::is_none")]
    pub pdg_mod_rsn: Option<PendingReason37Choice>,
    #[serde(rename = "DndRsn", skip_serializing_if = "Option::is_none")]
    pub dnd_rsn: Option<DeniedReason23Choice>,
    #[serde(rename = "RjctnRsn", skip_serializing_if = "Option::is_none")]
    pub rjctn_rsn: Option<RejectionReason51Choice>,
    #[serde(rename = "UmtchdRsn", skip_serializing_if = "Option::is_none")]
    pub umtchd_rsn: Option<UnmatchedReason29Choice>,
    #[serde(rename = "AckdAccptdRsn", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd_rsn: Option<AcknowledgementReason16Choice>,
    #[serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none")]
    pub cxl_rsn: Option<CancellationReason30Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reason20Choice {
    #[serde(flatten)]
    pub value: Reason20ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventStage2Code {
    #[serde(rename = "PWAL")]
    Pwal,
    #[serde(rename = "SUAP")]
    Suap,
    #[serde(rename = "APPD")]
    Appd,
    #[serde(rename = "UNAC")]
    Unac,
    #[serde(rename = "WHOU")]
    Whou,
    #[serde(rename = "CLDE")]
    Clde,
    #[serde(rename = "LAPS")]
    Laps,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StatementUpdateType1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "DELT")]
    Delt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason12Code {
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "CANS")]
    Cans,
    #[serde(rename = "CSUB")]
    Csub,
    #[serde(rename = "CXLR")]
    Cxlr,
    #[serde(rename = "CANT")]
    Cant,
    #[serde(rename = "CANZ")]
    Canz,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "SCEX")]
    Scex,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CANO")]
    Cano,
    #[serde(rename = "CREG")]
    Creg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AcknowledgementReason5Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "RQWV")]
    Rqwv,
    #[serde(rename = "LATE")]
    Late,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GeneratedReason3Code {
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "CLAI")]
    Clai,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "RODE")]
    Rode,
    #[serde(rename = "SPLI")]
    Spli,
    #[serde(rename = "THRD")]
    Thrd,
    #[serde(rename = "TRAN")]
    Tran,
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
pub struct PendingReason37ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason37Choice {
    #[serde(flatten)]
    pub value: PendingReason37ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StatementBasis1Code {
    #[serde(rename = "CONT")]
    Cont,
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "TRAD")]
    Trad,
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
pub struct SettlementStatus25ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesSettlementStatus2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStatus25Choice {
    #[serde(flatten)]
    pub value: SettlementStatus25ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency4Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "WEEK")]
    Week,
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
pub struct AcknowledgementReason16ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason16Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason16ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FailingReason1Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CANR")]
    Canr,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "OBJT")]
    Objt,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "STCD")]
    Stcd,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "MLAT")]
    Mlat,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
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
pub struct AdditionalQueryParameters14 {
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<Status22Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<Reason20Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmId", default)]
    pub fin_instrm_id: Vec<SecurityIdentification20>,
}
#[derive(
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
pub struct RestrictedFinxMax35Text {
    #[validate(
        length(min = 1, max = 35,),
        regex = "RESTRICTED_FINX_MAX_35_TEXT_REGEX"
    )]
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
pub struct OtherIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax31Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource4Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification84 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax34Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Status22ChoiceEnum {
    #[serde(rename = "CorpActnEvtPrcgSts", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_prcg_sts: Option<CorporateActionEventProcessingStatus4Choice>,
    #[serde(rename = "IfrrdMtchgSts", skip_serializing_if = "Option::is_none")]
    pub ifrrd_mtchg_sts: Option<MatchingStatus28Choice>,
    #[serde(rename = "RspnSts", skip_serializing_if = "Option::is_none")]
    pub rspn_sts: Option<ResponseStatus7Choice>,
    #[serde(rename = "AllcnSts", skip_serializing_if = "Option::is_none")]
    pub allcn_sts: Option<AllocationSatus4Choice>,
    #[serde(rename = "InstrPrcgSts", skip_serializing_if = "Option::is_none")]
    pub instr_prcg_sts: Option<InstructionProcessingStatus26Choice>,
    #[serde(rename = "CxlPrcgSts", skip_serializing_if = "Option::is_none")]
    pub cxl_prcg_sts: Option<CancellationProcessingStatus8Choice>,
    #[serde(rename = "RplcmntPrcgSts", skip_serializing_if = "Option::is_none")]
    pub rplcmnt_prcg_sts: Option<ReplacementProcessingStatus9Choice>,
    #[serde(rename = "RepoCallReqSts", skip_serializing_if = "Option::is_none")]
    pub repo_call_req_sts: Option<RepoCallRequestStatus10Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus28Choice>,
    #[serde(rename = "RegnPrcgSts", skip_serializing_if = "Option::is_none")]
    pub regn_prcg_sts: Option<RegistrationProcessingStatus4Choice>,
    #[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
    pub sttlm_sts: Option<SettlementStatus25Choice>,
    #[serde(rename = "SttlmCondModSts", skip_serializing_if = "Option::is_none")]
    pub sttlm_cond_mod_sts: Option<SettlementConditionModificationStatus4Choice>,
    #[serde(rename = "CorpActnEvtStag", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_stag: Option<CorporateActionEventStage4Choice>,
    #[serde(rename = "AffirmSts", skip_serializing_if = "Option::is_none")]
    pub affirm_sts: Option<AffirmationStatus9Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Status22Choice {
    #[serde(flatten)]
    pub value: Status22ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification156 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification136Choice,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "SctiesStmtQry")]
    pub scties_stmt_qry: SecuritiesStatementQuery002V08<A>,
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
pub struct RestrictedFinxMax34Text {
    #[validate(
        length(min = 1, max = 34,),
        regex = "RESTRICTED_FINX_MAX_34_TEXT_REGEX"
    )]
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
pub struct RejectionReason51ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason76Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason51Choice {
    #[serde(flatten)]
    pub value: RejectionReason51ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AffirmationStatus1Code {
    #[serde(rename = "AFFI")]
    Affi,
    #[serde(rename = "NAFI")]
    Nafi,
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
pub struct BlockChainAddressWallet7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification47>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<RestrictedFinxMax70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPeriod3ChoiceEnum {
    #[serde(rename = "StmtDt", skip_serializing_if = "Option::is_none")]
    pub stmt_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "StmtPrd", skip_serializing_if = "Option::is_none")]
    pub stmt_prd: Option<Period7Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPeriod3Choice {
    #[serde(flatten)]
    pub value: DateAndPeriod3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingProcessingReason1Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "CERT")]
    Cert,
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
pub struct AffirmationStatus9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AffirmationStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AffirmationStatus9Choice {
    #[serde(flatten)]
    pub value: AffirmationStatus9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AllocationSatus4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AllocationStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AllocationSatus4Choice {
    #[serde(flatten)]
    pub value: AllocationSatus4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReplacementProcessingStatus9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ReplacementProcessingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReplacementProcessingStatus9Choice {
    #[serde(flatten)]
    pub value: ReplacementProcessingStatus9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax30Text {
    #[validate(
        length(min = 1, max = 30,),
        regex = "RESTRICTED_FINX_MAX_30_TEXT_REGEX"
    )]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesSettlementStatus2Code {
    #[serde(rename = "PEND")]
    Pend,
    #[serde(rename = "PENF")]
    Penf,
    #[serde(rename = "USET")]
    Uset,
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "PAIN")]
    Pain,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementConditionModificationStatus1Code {
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "MODP")]
    Modp,
    #[serde(rename = "DEND")]
    Dend,
    #[serde(rename = "MODC")]
    Modc,
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
pub struct InstructionProcessingStatus26ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InstructionProcessingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionProcessingStatus26Choice {
    #[serde(flatten)]
    pub value: InstructionProcessingStatus26ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationProcessingStatus8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancellationProcessingStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationProcessingStatus8Choice {
    #[serde(flatten)]
    pub value: CancellationProcessingStatus8ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
}
#[derive(
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
