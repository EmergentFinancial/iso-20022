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
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.036.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTradeConfirmationRequestCancellationRequestV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header23,
    #[serde(rename = "CxlReqId", skip_serializing_if = "Option::is_none")]
    pub cxl_req_id: Option<MessageIdentification1>,
    #[validate]
    #[serde(rename = "TradgSdId")]
    pub tradg_sd_id: TradePartyIdentification7,
    #[validate]
    #[serde(rename = "CtrPtyRoleId")]
    pub ctr_pty_role_id: TradePartyIdentification7,
    #[validate]
    #[serde(rename = "TradId")]
    pub trad_id: Max35Text,
    #[serde(rename = "UndrlygPdctTp")]
    pub undrlyg_pdct_tp: UnderlyingProductIdentifier1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IdentificationType1Code {
    #[serde(rename = "BASC")]
    Basc,
    #[serde(rename = "BICO")]
    Bico,
    #[serde(rename = "CFET")]
    Cfet,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty3Code {
    #[serde(rename = "MAKE")]
    Make,
    #[serde(rename = "TAKE")]
    Take,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification19ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification19Choice {
    #[serde(flatten)]
    pub value: PartyIdentification19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SimpleIdentificationInformation4 {
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
pub struct PartyIdentification90 {
    #[serde(rename = "IdTp")]
    pub id_tp: PartyIdentificationType1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyIdentificationType1Code {
    #[serde(rename = "FXID")]
    Fxid,
    #[serde(rename = "FXSN")]
    Fxsn,
    #[serde(rename = "INGN")]
    Ingn,
    #[serde(rename = "IICS")]
    Iics,
    #[serde(rename = "IGBT")]
    Igbt,
    #[serde(rename = "MAMA")]
    Mama,
    #[serde(rename = "MEOC")]
    Meoc,
    #[serde(rename = "METY")]
    Mety,
    #[serde(rename = "NOMM")]
    Nomm,
    #[serde(rename = "OSCO")]
    Osco,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "PONU")]
    Ponu,
    #[serde(rename = "POAD")]
    Poad,
    #[serde(rename = "RMID")]
    Rmid,
    #[serde(rename = "SLCN")]
    Slcn,
    #[serde(rename = "SLNF")]
    Slnf,
    #[serde(rename = "TACN")]
    Tacn,
    #[serde(rename = "TRCO")]
    Trco,
    #[serde(rename = "TANA")]
    Tana,
    #[serde(rename = "USIT")]
    Usit,
    #[serde(rename = "USNA")]
    Usna,
    #[serde(rename = "AUIT")]
    Auit,
    #[serde(rename = "BRID")]
    Brid,
    #[serde(rename = "CLIN")]
    Clin,
    #[serde(rename = "CMID")]
    Cmid,
    #[serde(rename = "COIN")]
    Coin,
    #[serde(rename = "CMOT")]
    Cmot,
    #[serde(rename = "CONU")]
    Conu,
    #[serde(rename = "CMIN")]
    Cmin,
    #[serde(rename = "DECN")]
    Decn,
    #[serde(rename = "DEPA")]
    Depa,
    #[serde(rename = "ELCO")]
    Elco,
    #[serde(rename = "EXVE")]
    Exve,
    #[serde(rename = "FICO")]
    Fico,
    #[serde(rename = "FIID")]
    Fiid,
    #[serde(rename = "FLCN")]
    Flcn,
    #[serde(rename = "FLNF")]
    Flnf,
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
pub struct PartyIdentification44 {
    #[validate]
    #[serde(rename = "AnyBIC")]
    pub any_bic: AnyBicIdentifier,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
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
pub struct NameAndAddress8 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress1 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
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
pub struct PartyIdentificationAndAccount119 {
    #[validate(length(min = 1,))]
    #[serde(rename = "PtyId", default)]
    pub pty_id: Vec<PartyIdentification90>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AcctId", default)]
    pub acct_id: Vec<AccountIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Header23 {
    #[validate]
    #[serde(rename = "FrmtVrsn")]
    pub frmt_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max3NumericText,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification32,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification32>,
    #[validate]
    #[serde(rename = "MsgSeqNb")]
    pub msg_seq_nb: Number,
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
pub struct AccountIdentification30 {
    #[serde(rename = "AcctTp")]
    pub acct_tp: AccountInformationType1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AccountIdentification26,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingProductIdentifier1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "NDFO")]
    Ndfo,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountInformationType1Code {
    #[serde(rename = "IBND")]
    Ibnd,
    #[serde(rename = "IBCC")]
    Ibcc,
    #[serde(rename = "IBDC")]
    Ibdc,
    #[serde(rename = "BIBC")]
    Bibc,
    #[serde(rename = "BIBD")]
    Bibd,
    #[serde(rename = "BINC")]
    Binc,
    #[serde(rename = "BIND")]
    Bind,
    #[serde(rename = "BICC")]
    Bicc,
    #[serde(rename = "BIDC")]
    Bidc,
    #[serde(rename = "CMSA")]
    Cmsa,
    #[serde(rename = "CBBC")]
    Cbbc,
    #[serde(rename = "CBBD")]
    Cbbd,
    #[serde(rename = "CBNC")]
    Cbnc,
    #[serde(rename = "CBND")]
    Cbnd,
    #[serde(rename = "CBCC")]
    Cbcc,
    #[serde(rename = "CBDC")]
    Cbdc,
    #[serde(rename = "CUAC")]
    Cuac,
    #[serde(rename = "DEAC")]
    Deac,
    #[serde(rename = "FCAA")]
    Fcaa,
    #[serde(rename = "FCAN")]
    Fcan,
    #[serde(rename = "FCBN")]
    Fcbn,
    #[serde(rename = "IBBC")]
    Ibbc,
    #[serde(rename = "IBBD")]
    Ibbd,
    #[serde(rename = "IBNC")]
    Ibnc,
    #[serde(rename = "MCAA")]
    Mcaa,
    #[serde(rename = "MCAN")]
    Mcan,
    #[serde(rename = "MCIC")]
    Mcic,
    #[serde(rename = "MCIN")]
    Mcin,
    #[serde(rename = "MSAA")]
    Msaa,
    #[serde(rename = "MSBN")]
    Msbn,
    #[serde(rename = "MCAD")]
    Mcad,
    #[serde(rename = "NODC")]
    Nodc,
    #[serde(rename = "SCAC")]
    Scac,
    #[serde(rename = "SCAA")]
    Scaa,
    #[serde(rename = "OMSA")]
    Omsa,
    #[serde(rename = "NOCC")]
    Nocc,
    #[serde(rename = "MSBS")]
    Msbs,
    #[serde(rename = "MSAN")]
    Msan,
    #[serde(rename = "SCAN")]
    Scan,
    #[serde(rename = "SCIC")]
    Scic,
    #[serde(rename = "SCIN")]
    Scin,
    #[serde(rename = "SOCA")]
    Soca,
    #[serde(rename = "SSCA")]
    Ssca,
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
pub struct FundIdentification3 {
    #[validate]
    #[serde(rename = "FndId")]
    pub fnd_id: Max35Text,
    #[serde(rename = "AcctIdWthCtdn", skip_serializing_if = "Option::is_none")]
    pub acct_id_wth_ctdn: Option<Max35Text>,
    #[serde(rename = "CtdnId", skip_serializing_if = "Option::is_none")]
    pub ctdn_id: Option<PartyIdentification19Choice>,
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
pub struct AccountIdentification26 {
    #[validate]
    #[serde(rename = "Prtry")]
    pub prtry: SimpleIdentificationInformation4,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty1Code {
    #[serde(rename = "SLLR")]
    Sllr,
    #[serde(rename = "BYER")]
    Byer,
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
pub struct TradePartyIdentification7 {
    #[serde(rename = "FndInf", skip_serializing_if = "Option::is_none")]
    pub fnd_inf: Option<FundIdentification3>,
    #[serde(rename = "BuyrOrSellrInd")]
    pub buyr_or_sellr_ind: OptionParty1Code,
    #[serde(rename = "InitrInd")]
    pub initr_ind: OptionParty3Code,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: PartyIdentification78,
    #[validate]
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: PartyIdentificationAndAccount119,
}
#[derive(
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
    #[serde(rename = "FXTradConfReqCxlReq")]
    pub fx_trad_conf_req_cxl_req: ForeignExchangeTradeConfirmationRequestCancellationRequestV01<A>,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
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
pub struct AnyBicIdentifier {
    #[validate(regex = "ANY_BIC_IDENTIFIER_REGEX")]
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
pub struct PartyIdentification78 {
    #[serde(rename = "PtySrc", skip_serializing_if = "Option::is_none")]
    pub pty_src: Option<IdentificationType1Code>,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: Max35Text,
}
#[derive(
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
