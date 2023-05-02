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
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:secl.002.001.03".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Clearing4 {
    #[serde(rename = "SttlmNetgElgblCd")]
    pub sttlm_netg_elgbl_cd: NettingEligible1Code,
    #[serde(rename = "ClrSgmt", skip_serializing_if = "Option::is_none")]
    pub clr_sgmt: Option<PartyIdentification35Choice>,
    #[serde(rename = "GrntedTrad", skip_serializing_if = "Option::is_none")]
    pub grnted_trad: Option<YesNoIndicator>,
    #[serde(rename = "NonGrntedTrad", skip_serializing_if = "Option::is_none")]
    pub non_grnted_trad: Option<NonGuaranteedTrade3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternatePartyIdentification5 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType40Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingAccountType1Code {
    #[serde(rename = "HOUS")]
    Hous,
    #[serde(rename = "CLIE")]
    Clie,
    #[serde(rename = "LIPR")]
    Lipr,
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
pub struct AlternatePartyIdentification4 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType6Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount102 {
    #[serde(rename = "PtyId")]
    pub pty_id: PartyIdentification33Choice,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTimeChoice>,
    #[serde(rename = "SubAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<SubAccount4>,
    #[serde(rename = "CtctPrsn", skip_serializing_if = "Option::is_none")]
    pub ctct_prsn: Option<ContactIdentification2>,
}
#[derive(
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
pub struct IdentificationSource3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource3Choice {
    #[serde(flatten)]
    pub value: IdentificationSource3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification83ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification83Choice {
    #[serde(flatten)]
    pub value: PartyIdentification83ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType5Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType9Choice {
    #[serde(flatten)]
    pub value: MarketType9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification20>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode3Choice {
    #[serde(flatten)]
    pub value: DateCode3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat7ChoiceEnum {
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndAnyBicIdentifier1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText1>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification58>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat7Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAndAmount")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
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
pub struct GenericIdentification58 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification40,
}
#[derive(
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
pub struct SubAccount4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: AccountIdentification26,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms17 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[validate]
    #[serde(rename = "RsltgAmt")]
    pub rsltg_amt: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification34ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification34Choice {
    #[serde(flatten)]
    pub value: PartyIdentification34ChoiceEnum,
}
#[derive(
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
pub struct IdentificationType40ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType40Choice {
    #[serde(flatten)]
    pub value: IdentificationType40ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix1Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MADM")]
    Madm,
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
pub struct NonGuaranteedTrade3 {
    #[serde(rename = "TradCtrPtyMmbId")]
    pub trad_ctr_pty_mmb_id: PartyIdentification35Choice,
    #[serde(rename = "TradCtrPtyClrMmbId")]
    pub trad_ctr_pty_clr_mmb_id: PartyIdentification35Choice,
    #[serde(rename = "DlvrgPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_pties: Option<DeliveringPartiesAndAccount11>,
    #[serde(rename = "RcvgPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_pties: Option<ReceivingPartiesAndAccount11>,
}
#[derive(
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
pub struct GenericIdentification20 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
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
pub struct GenericIdentification30 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketType5Code {
    #[serde(rename = "OTCO")]
    Otco,
    #[serde(rename = "EXCH")]
    Exch,
    #[default]
    Unknown,
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
pub struct MarketIdentification85 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification1Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType9Choice,
}
#[derive(
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
pub struct AmountAndDirection27 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress13 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount19 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
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
pub struct Settlement1 {
    #[validate]
    #[serde(rename = "SttlmAmt")]
    pub sttlm_amt: AmountAndDirection27,
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification34Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount18 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: ClearingAccountType1Code,
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
pub struct GenericIdentification29 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradePosting1Code {
    #[serde(rename = "GROS")]
    Gros,
    #[serde(rename = "NETT")]
    Nett,
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
pub struct SafekeepingPlaceTypeAndAnyBicIdentifier1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AnyBicIdentifier,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NettingEligible1Code {
    #[serde(rename = "GROS")]
    Gros,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "AGFS")]
    Agfs,
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
pub struct Price4 {
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmountChoice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PriceValueType7Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ICSD")]
    Icsd,
    #[serde(rename = "NCSD")]
    Ncsd,
    #[serde(rename = "SHHE")]
    Shhe,
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
pub struct SafekeepingPlaceTypeAndText1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
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
pub struct NameAndAddress6 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1ChoiceEnum {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1Choice {
    #[serde(flatten)]
    pub value: MarketIdentification1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Side1Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWOS")]
    Twos,
    #[serde(rename = "BUMI")]
    Bumi,
    #[serde(rename = "SEPL")]
    Sepl,
    #[serde(rename = "SESH")]
    Sesh,
    #[serde(rename = "SSEX")]
    Ssex,
    #[serde(rename = "CROS")]
    Cros,
    #[serde(rename = "CRSH")]
    Crsh,
    #[serde(rename = "CSHE")]
    Cshe,
    #[serde(rename = "DEFI")]
    Defi,
    #[serde(rename = "OPPO")]
    Oppo,
    #[serde(rename = "UNDI")]
    Undi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Status5Code {
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "PDNG")]
    Pdng,
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
pub struct TradeLeg8 {
    #[validate]
    #[serde(rename = "TradLegId")]
    pub trad_leg_id: Max35Text,
    #[serde(rename = "TradId", skip_serializing_if = "Option::is_none")]
    pub trad_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TradExctnId")]
    pub trad_exctn_id: Max35Text,
    #[serde(rename = "OrdrId", skip_serializing_if = "Option::is_none")]
    pub ordr_id: Option<Max35Text>,
    #[serde(rename = "AllcnId", skip_serializing_if = "Option::is_none")]
    pub allcn_id: Option<Max35Text>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<Status5Code>,
    #[validate]
    #[serde(rename = "TradDt")]
    pub trad_dt: IsoDateTime,
    #[serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none")]
    pub tx_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<DateFormat15Choice>,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification14,
    #[serde(rename = "TradgCcy", skip_serializing_if = "Option::is_none")]
    pub tradg_ccy: Option<CurrencyCode>,
    #[serde(rename = "BuySellInd")]
    pub buy_sell_ind: Side1Code,
    #[serde(rename = "TradQty")]
    pub trad_qty: FinancialInstrumentQuantity1Choice,
    #[validate]
    #[serde(rename = "DealPric")]
    pub deal_pric: Price4,
    #[serde(rename = "GrssAmt", skip_serializing_if = "Option::is_none")]
    pub grss_amt: Option<AmountAndDirection21>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection21>,
    #[validate]
    #[serde(rename = "PlcOfTrad")]
    pub plc_of_trad: MarketIdentification84,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification85>,
    #[serde(rename = "TradTp")]
    pub trad_tp: TradeType1Code,
    #[serde(rename = "DerivRltdTrad", skip_serializing_if = "Option::is_none")]
    pub deriv_rltd_trad: Option<YesNoIndicator>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<PartyIdentificationAndAccount100>,
    #[serde(rename = "TradgPty")]
    pub tradg_pty: PartyIdentification35Choice,
    #[serde(rename = "TradRegnOrgn", skip_serializing_if = "Option::is_none")]
    pub trad_regn_orgn: Option<Max35Text>,
    #[serde(rename = "TradgPtyAcct", skip_serializing_if = "Option::is_none")]
    pub tradg_pty_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "TradgCpcty")]
    pub tradg_cpcty: TradingCapacity5Code,
    #[serde(rename = "TradPstngCd", skip_serializing_if = "Option::is_none")]
    pub trad_pstng_cd: Option<TradePosting1Code>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat7Choice>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeLegNotificationCancellationV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "ClrMmb")]
    pub clr_mmb: PartyIdentification35Choice,
    #[validate]
    #[serde(rename = "ClrAcct")]
    pub clr_acct: SecuritiesAccount18,
    #[serde(rename = "DlvryAcct", skip_serializing_if = "Option::is_none")]
    pub dlvry_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "NonClrMmb", skip_serializing_if = "Option::is_none")]
    pub non_clr_mmb: Option<PartyIdentificationAndAccount31>,
    #[serde(rename = "ClrDtls", skip_serializing_if = "Option::is_none")]
    pub clr_dtls: Option<Clearing4>,
    #[validate]
    #[serde(rename = "TradLegDtls")]
    pub trad_leg_dtls: TradeLeg8,
    #[validate]
    #[serde(rename = "SttlmDtls")]
    pub sttlm_dtls: Settlement1,
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
pub struct PriceRateOrAmountChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmountChoice {
    #[serde(flatten)]
    pub value: PriceRateOrAmountChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType6Choice {
    #[serde(flatten)]
    pub value: IdentificationType6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification2Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "FIIN")]
    Fiin,
    #[serde(rename = "TXID")]
    Txid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
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
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace3Code {
    #[serde(rename = "SHHE")]
    Shhe,
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
pub struct DateFormat15ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode3Choice>,
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
pub struct DateFormat15Choice {
    #[serde(flatten)]
    pub value: DateFormat15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReceivingPartiesAndAccount11 {
    #[serde(rename = "Dpstry")]
    pub dpstry: PartyIdentification34Choice,
    #[validate]
    #[serde(rename = "Pty1")]
    pub pty_1: PartyIdentificationAndAccount102,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount102>,
    #[serde(rename = "SctiesSttlmSys", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_sys: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification14 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification1Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "FIIN")]
    Fiin,
    #[serde(rename = "TXID")]
    Txid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity5Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "RISP")]
    Risp,
    #[serde(rename = "AGEN")]
    Agen,
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
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TradLegNtfctnCxl")]
    pub trad_leg_ntfctn_cxl: TradeLegNotificationCancellationV03<A>,
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
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType7Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "YIEL")]
    Yiel,
    #[serde(rename = "SPRE")]
    Spre,
    #[serde(rename = "PEUN")]
    Peun,
    #[serde(rename = "ABSO")]
    Abso,
    #[serde(rename = "TEDP")]
    Tedp,
    #[serde(rename = "TEDY")]
    Tedy,
    #[serde(rename = "FICT")]
    Fict,
    #[serde(rename = "VACT")]
    Vact,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "ACTU")]
    Actu,
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
pub struct NameAndAddress5 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType8Choice {
    #[serde(flatten)]
    pub value: MarketType8ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketType2Code {
    #[serde(rename = "PRIM")]
    Prim,
    #[serde(rename = "SECM")]
    Secm,
    #[serde(rename = "OTCO")]
    Otco,
    #[serde(rename = "VARI")]
    Vari,
    #[serde(rename = "EXCH")]
    Exch,
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
pub struct PartyIdentification33ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification33Choice {
    #[serde(flatten)]
    pub value: PartyIdentification33ChoiceEnum,
}
#[derive(
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
pub struct MarketIdentification84 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification1Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType8Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType1Code {
    #[serde(rename = "UKWN")]
    Ukwn,
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
pub struct AmountAndDirection21 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification35ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification35Choice {
    #[serde(flatten)]
    pub value: PartyIdentification35ChoiceEnum,
}
#[derive(
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
pub struct DeliveringPartiesAndAccount11 {
    #[serde(rename = "Dpstry")]
    pub dpstry: PartyIdentification34Choice,
    #[validate]
    #[serde(rename = "Pty1")]
    pub pty_1: PartyIdentificationAndAccount102,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount102>,
    #[serde(rename = "SctiesSttlmSys", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_sys: Option<Max35Text>,
}
#[derive(
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
pub enum TradeType1Code {
    #[serde(rename = "OOBK")]
    Oobk,
    #[serde(rename = "OFBK")]
    Ofbk,
    #[serde(rename = "BKTR")]
    Bktr,
    #[serde(rename = "COTR")]
    Cotr,
    #[serde(rename = "GUTR")]
    Gutr,
    #[serde(rename = "LKTR")]
    Lktr,
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
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
pub struct IsinIdentifier {
    #[validate(regex = "ISIN_IDENTIFIER_REGEX")]
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
pub struct ContactIdentification2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity1ChoiceEnum {
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity1Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity1ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentificationAndAccount100 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification83Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
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
pub struct PartyIdentificationAndAccount31 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification33Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification4>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
    #[serde(rename = "ClrAcct", skip_serializing_if = "Option::is_none")]
    pub clr_acct: Option<SecuritiesAccount18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation1 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeChoiceEnum {
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
pub struct DateAndDateTimeChoice {
    #[serde(flatten)]
    pub value: DateAndDateTimeChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification40 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
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
pub struct PostalAddress8 {
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
pub struct OtherIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}
