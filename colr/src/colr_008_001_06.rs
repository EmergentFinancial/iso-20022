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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:colr.008.001.06".to_string()
}

#[derive(
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
pub struct CollateralProposalResponseV06<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[validate]
    #[serde(rename = "Oblgtn")]
    pub oblgtn: Obligation9,
    #[serde(rename = "PrpslRspn")]
    pub prpsl_rspn: CollateralProposalResponse4Choice,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralProposalResponse1Code {
    #[serde(rename = "INPR")]
    Inpr,
    #[serde(rename = "COPR")]
    Copr,
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
pub struct CashCollateralResponse3 {
    #[serde(rename = "RspnTp")]
    pub rspn_tp: Status4Code,
    #[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
    pub coll_id: Option<Max35Text>,
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<AccountIdentification4Choice>,
    #[serde(rename = "RjctnRsn", skip_serializing_if = "Option::is_none")]
    pub rjctn_rsn: Option<RejectionReason68Code>,
    #[serde(rename = "RjctnInf", skip_serializing_if = "Option::is_none")]
    pub rjctn_inf: Option<Max35Text>,
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
pub struct AccountSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalAccountIdentification1Code>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralProposalResponseType4 {
    #[validate]
    #[serde(rename = "CollPrpslId")]
    pub coll_prpsl_id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: CollateralProposalResponse1Code,
    #[validate]
    #[serde(rename = "Rspn")]
    pub rspn: CollateralResponse3,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason68Code {
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "EVNM")]
    Evnm,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "ICOL")]
    Icol,
    #[serde(rename = "CONL")]
    Conl,
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "INID")]
    Inid,
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
pub struct SecuritiesCollateralResponse2 {
    #[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
    pub coll_id: Option<Max35Text>,
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "RspnTp")]
    pub rspn_tp: Status4Code,
    #[serde(rename = "RjctnRsn", skip_serializing_if = "Option::is_none")]
    pub rjctn_rsn: Option<RejectionReason68Code>,
    #[serde(rename = "RjctnInf", skip_serializing_if = "Option::is_none")]
    pub rjctn_inf: Option<Max35Text>,
}
#[derive(
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
pub struct CollateralProposalResponse4 {
    #[validate]
    #[serde(rename = "VartnMrgn")]
    pub vartn_mrgn: CollateralProposalResponseType4,
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<CollateralProposalResponseType4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralResponse3 {
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesCollRspn", default)]
    pub scties_coll_rspn: Vec<SecuritiesCollateralResponse2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshCollRspn", default)]
    pub csh_coll_rspn: Vec<CashCollateralResponse3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrCollRspn", default)]
    pub othr_coll_rspn: Vec<OtherCollateralResponse3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralProposalResponse4ChoiceEnum {
    #[serde(rename = "SgrtdIndpdntAmt", skip_serializing_if = "Option::is_none")]
    pub sgrtd_indpdnt_amt: Option<CollateralProposalResponseType4>,
    #[serde(rename = "CollPrpsl", skip_serializing_if = "Option::is_none")]
    pub coll_prpsl: Option<CollateralProposalResponse4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralProposalResponse4Choice {
    #[serde(flatten)]
    pub value: CollateralProposalResponse4ChoiceEnum,
}
#[derive(
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
pub struct OtherCollateralResponse3 {
    #[serde(rename = "RspnTp")]
    pub rspn_tp: Status4Code,
    #[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
    pub coll_id: Option<Max35Text>,
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "RjctnRsn", skip_serializing_if = "Option::is_none")]
    pub rjctn_rsn: Option<RejectionReason68Code>,
    #[serde(rename = "RjctnInf", skip_serializing_if = "Option::is_none")]
    pub rjctn_inf: Option<Max35Text>,
}
#[derive(
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
pub struct PartyIdentification178ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CollPrpslRspn")]
    pub coll_prpsl_rspn: CollateralProposalResponseV06<A>,
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
pub struct NameAndAddress6 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress2,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Status4Code {
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "PACK")]
    Pack,
    #[default]
    Unknown,
}
