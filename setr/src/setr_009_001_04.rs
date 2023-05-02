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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:setr.009.001.04".to_string()
}

#[derive(
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
pub struct FundOrderType4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundOrderType8Code>,
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
pub struct FundOrderType4Choice {
    #[serde(flatten)]
    pub value: FundOrderType4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditTransfer8 {
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentification113>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<AccountIdentificationAndName5>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<FinancialInstitutionIdentification10>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<AccountIdentificationAndName5>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_1: Option<FinancialInstitutionIdentification10>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_1_acct: Option<AccountIdentificationAndName5>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_2: Option<FinancialInstitutionIdentification10>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_2_acct: Option<AccountIdentificationAndName5>,
    #[validate]
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: FinancialInstitutionIdentification10,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<AccountIdentificationAndName5>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentification113>,
    #[validate]
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: AccountIdentificationAndName5,
}
#[derive(
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
pub struct UkDomesticSortCodeIdentifier {
    #[validate(regex = "UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WaivingInstruction1Code {
    #[serde(rename = "WICA")]
    Wica,
    #[serde(rename = "WIUN")]
    Wiun,
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
pub struct DeliveringPartiesAndAccount16 {
    #[serde(rename = "DlvrrsCtdnDtls", skip_serializing_if = "Option::is_none")]
    pub dlvrrs_ctdn_dtls: Option<PartyIdentificationAndAccount147>,
    #[serde(rename = "DlvrrsIntrmy1Dtls", skip_serializing_if = "Option::is_none")]
    pub dlvrrs_intrmy_1_dtls: Option<PartyIdentificationAndAccount147>,
    #[serde(rename = "DlvrrsIntrmy2Dtls", skip_serializing_if = "Option::is_none")]
    pub dlvrrs_intrmy_2_dtls: Option<PartyIdentificationAndAccount147>,
    #[validate]
    #[serde(rename = "DlvrgAgtDtls")]
    pub dlvrg_agt_dtls: PartyIdentificationAndAccount147,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WertpapierIdentifier {
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxableIncomePerShareCalculated2Code {
    #[serde(rename = "TSIY")]
    Tsiy,
    #[serde(rename = "TSIN")]
    Tsin,
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
pub struct TickerIdentifier {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BestExecution1Code {
    #[serde(rename = "BTEX")]
    Btex,
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
pub struct SubAccount6 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionChannel2Code {
    #[serde(rename = "FIAD")]
    Fiad,
    #[serde(rename = "HOBA")]
    Hoba,
    #[serde(rename = "BRAN")]
    Bran,
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
pub struct CanadianPaymentsArnIdentifier {
    #[validate(regex = "CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX")]
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
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
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
pub struct TotalFeesAndTaxes40 {
    #[serde(rename = "TtlOvrhdApld", skip_serializing_if = "Option::is_none")]
    pub ttl_ovrhd_apld: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlFees", skip_serializing_if = "Option::is_none")]
    pub ttl_fees: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlTaxs", skip_serializing_if = "Option::is_none")]
    pub ttl_taxs: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ComrclAgrmtRef", skip_serializing_if = "Option::is_none")]
    pub comrcl_agrmt_ref: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvFee", default)]
    pub indv_fee: Vec<Fee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvTax", default)]
    pub indv_tax: Vec<Tax31>,
}
#[derive(
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
pub struct Extension1 {
    #[validate]
    #[serde(rename = "PlcAndNm")]
    pub plc_and_nm: Max350Text,
    #[validate]
    #[serde(rename = "Txt")]
    pub txt: Max350Text,
}
#[derive(
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
pub struct Cheque9 {
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "PyeeId")]
    pub pyee_id: PartyIdentification113,
    #[serde(rename = "DrweeId", skip_serializing_if = "Option::is_none")]
    pub drwee_id: Option<FinancialInstitutionIdentification10>,
    #[serde(rename = "DrwrId", skip_serializing_if = "Option::is_none")]
    pub drwr_id: Option<PartyIdentification113>,
}
#[derive(
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
pub struct OtherIdentification3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PartyIdentificationType7Code>,
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
pub struct OtherIdentification3Choice {
    #[serde(flatten)]
    pub value: OtherIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceValue1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResidentialStatus1Code {
    #[serde(rename = "RESI")]
    Resi,
    #[serde(rename = "PRES")]
    Pres,
    #[serde(rename = "NRES")]
    Nres,
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
pub struct TaxCalculationInformation10 {
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<TaxBasis1Choice>,
    #[validate]
    #[serde(rename = "TaxblAmt")]
    pub taxbl_amt: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValorenIdentifier {
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
pub struct CustomerConductClassification1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConductClassification1Code>,
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
pub struct CustomerConductClassification1Choice {
    #[serde(flatten)]
    pub value: CustomerConductClassification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundSettlementParameters12 {
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "SttlmPlc")]
    pub sttlm_plc: PartyIdentification113,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat8Choice>,
    #[serde(rename = "SctiesSttlmSysId", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_sys_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition30Choice>,
    #[serde(rename = "RcvgSdDtls", skip_serializing_if = "Option::is_none")]
    pub rcvg_sd_dtls: Option<ReceivingPartiesAndAccount16>,
    #[validate]
    #[serde(rename = "DlvrgSdDtls")]
    pub dlvrg_sd_dtls: DeliveringPartiesAndAccount16,
}
#[derive(
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
pub struct EuCapitalGain3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EuCapitalGain2Code>,
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
pub struct EuCapitalGain3Choice {
    #[serde(flatten)]
    pub value: EuCapitalGain3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardType1Code {
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
pub struct ItalianDomesticIdentifier {
    #[validate(regex = "ITALIAN_DOMESTIC_IDENTIFIER_REGEX")]
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
pub struct ExtensiveBranchNetworkIdentifier {
    #[validate(regex = "EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX")]
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
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
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
pub struct ProfitAndLoss2ChoiceEnum {
    #[serde(rename = "Prft", skip_serializing_if = "Option::is_none")]
    pub prft: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Loss", skip_serializing_if = "Option::is_none")]
    pub loss: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProfitAndLoss2Choice {
    #[serde(flatten)]
    pub value: ProfitAndLoss2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition30ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition11Code>,
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
pub struct SettlementTransactionCondition30Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition30ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SubscriptionBulkExecution4 {
    #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<YesNoIndicator>,
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification1Choice>,
    #[serde(rename = "OrdrDtTm", skip_serializing_if = "Option::is_none")]
    pub ordr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "RcvdDtTm", skip_serializing_if = "Option::is_none")]
    pub rcvd_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ReqdFutrTradDt", skip_serializing_if = "Option::is_none")]
    pub reqd_futr_trad_dt: Option<IsoDate>,
    #[serde(rename = "CxlRght", skip_serializing_if = "Option::is_none")]
    pub cxl_rght: Option<CancellationRight1Choice>,
    #[validate]
    #[serde(rename = "FinInstrmDtls")]
    pub fin_instrm_dtls: FinancialInstrument57,
    #[validate(length(min = 1,))]
    #[serde(rename = "IndvExctnDtls", default)]
    pub indv_exctn_dtls: Vec<SubscriptionExecution12>,
    #[serde(rename = "ReqdSttlmCcy", skip_serializing_if = "Option::is_none")]
    pub reqd_sttlm_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
    pub reqd_nav_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "TtlSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_sttlm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "BlkCshSttlmDtls", skip_serializing_if = "Option::is_none")]
    pub blk_csh_sttlm_dtls: Option<PaymentTransaction70>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SwissSicIdentifier {
    #[validate(regex = "SWISS_SIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConductClassification1Code {
    #[serde(rename = "NSTA")]
    Nsta,
    #[serde(rename = "RCLT")]
    Rclt,
    #[serde(rename = "STAN")]
    Stan,
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
pub struct AccountIdentificationAndName5 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
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
pub struct PartyIdentification113 {
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification90Choice,
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
pub struct UnitPrice22 {
    #[serde(rename = "Tp")]
    pub tp: TypeOfPrice46Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PriceValue1,
    #[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
    pub pric_mtd: Option<PriceMethod1Code>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Number>,
    #[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
    pub taxbl_incm_per_shr: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Choice>,
    #[serde(rename = "PricDiffRsn", skip_serializing_if = "Option::is_none")]
    pub pric_diff_rsn: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicfiIdentifier {
    #[validate(regex = "BICFI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FormOfSecurity1Code {
    #[serde(rename = "BEAR")]
    Bear,
    #[serde(rename = "REGD")]
    Regd,
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
pub struct FinancialInstitutionIdentification8ChoiceEnum {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<Max35Text>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentificationChoice>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification8Choice {
    #[serde(flatten)]
    pub value: FinancialInstitutionIdentification8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderWaiverReason3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OrderWaiverReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderWaiverReason3Choice {
    #[serde(flatten)]
    pub value: OrderWaiverReason3ChoiceEnum,
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
pub struct Series1 {
    #[serde(rename = "SrsDt", skip_serializing_if = "Option::is_none")]
    pub srs_dt: Option<DateFormat42Choice>,
    #[serde(rename = "SrsNm", skip_serializing_if = "Option::is_none")]
    pub srs_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification10 {
    #[serde(rename = "Pty")]
    pub pty: FinancialInstitutionIdentification8Choice,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition5Code {
    #[serde(rename = "XCPN")]
    Xcpn,
    #[serde(rename = "CCPN")]
    Ccpn,
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole2Code {
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "REGI")]
    Regi,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "UCL1")]
    Ucl1,
    #[serde(rename = "UCL2")]
    Ucl2,
    #[serde(rename = "TRAN")]
    Tran,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity8Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "PRIN")]
    Prin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionPolicy1Code {
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "ACCU")]
    Accu,
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
pub struct PaymentInstrument20ChoiceEnum {
    #[serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none")]
    pub pmt_card_dtls: Option<PaymentCard25>,
    #[serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none")]
    pub chq_dtls: Option<Cheque9>,
    #[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
    pub csh_acct_dtls: Option<InvestmentAccount60>,
    #[serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none")]
    pub bkrs_drft_dtls: Option<Cheque9>,
    #[serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none")]
    pub drct_dbt_dtls: Option<DirectDebitMandate6>,
    #[serde(rename = "CdtTrfDtls", skip_serializing_if = "Option::is_none")]
    pub cdt_trf_dtls: Option<CreditTransfer8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument20Choice {
    #[serde(flatten)]
    pub value: PaymentInstrument20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SedolIdentifier {
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
pub struct IndividualPerson31 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
    pub birth_dt: Option<IsoDate>,
    #[serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none")]
    pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
    #[serde(
        rename = "BnfcryCertfctnCmpltn",
        skip_serializing_if = "Option::is_none"
    )]
    pub bnfcry_certfctn_cmpltn: Option<BeneficiaryCertificationCompletion1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<GenericIdentification164>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeOrCommissionDiscount1 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<WaivingInstruction2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RicIdentifier {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundOrderType5Code {
    #[serde(rename = "NSPN")]
    Nspn,
    #[serde(rename = "NCPN")]
    Ncpn,
    #[serde(rename = "SWSP")]
    Swsp,
    #[serde(rename = "CWSP")]
    Cwsp,
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
pub struct SafekeepingPlaceTypeAndText6 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace2Code,
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
pub struct SignatureType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SignatureType2Code>,
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
pub struct SignatureType1Choice {
    #[serde(flatten)]
    pub value: SignatureType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SubscriptionExecution12 {
    #[validate]
    #[serde(rename = "OrdrRef")]
    pub ordr_ref: Max35Text,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "DealRef")]
    pub deal_ref: Max35Text,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "OrdrTp", default)]
    pub ordr_tp: Vec<FundOrderType4Choice>,
    #[validate]
    #[serde(rename = "InvstmtAcctDtls")]
    pub invstmt_acct_dtls: InvestmentAccount58,
    #[validate(length(min = 0,))]
    #[serde(rename = "BnfcryDtls", default)]
    pub bnfcry_dtls: Vec<IndividualPerson31>,
    #[validate]
    #[serde(rename = "UnitsNb")]
    pub units_nb: DecimalNumber,
    #[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
    pub rndg: Option<RoundingDirection2Code>,
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrssAmt", skip_serializing_if = "Option::is_none")]
    pub grss_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TradDtTm")]
    pub trad_dt_tm: DateAndDateTimeChoice,
    #[validate]
    #[serde(rename = "DealgPricDtls")]
    pub dealg_pric_dtls: UnitPrice22,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "InftvPricDtls", default)]
    pub inftv_pric_dtls: Vec<UnitPrice22>,
    #[validate]
    #[serde(rename = "SttlmAmt")]
    pub sttlm_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none")]
    pub csh_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "SttlmMtd", skip_serializing_if = "Option::is_none")]
    pub sttlm_mtd: Option<DeliveryReceiptType2Code>,
    #[validate]
    #[serde(rename = "PrtlyExctdInd")]
    pub prtly_exctd_ind: YesNoIndicator,
    #[serde(rename = "BestExctn", skip_serializing_if = "Option::is_none")]
    pub best_exctn: Option<BestExecution1Code>,
    #[validate]
    #[serde(rename = "CumDvddInd")]
    pub cum_dvdd_ind: YesNoIndicator,
    #[serde(rename = "IntrmPrftAmt", skip_serializing_if = "Option::is_none")]
    pub intrm_prft_amt: Option<ProfitAndLoss2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FXDtls", default)]
    pub fx_dtls: Vec<ForeignExchangeTerms33>,
    #[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
    pub incm_pref: Option<IncomePreference1Code>,
    #[serde(rename = "LttrInttRef", skip_serializing_if = "Option::is_none")]
    pub lttr_intt_ref: Option<Max35Text>,
    #[serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none")]
    pub acmltn_rght_ref: Option<Max35Text>,
    #[serde(rename = "TxOvrhd", skip_serializing_if = "Option::is_none")]
    pub tx_ovrhd: Option<TotalFeesAndTaxes40>,
    #[serde(rename = "InftvTaxDtls", skip_serializing_if = "Option::is_none")]
    pub inftv_tax_dtls: Option<InformativeTax1>,
    #[serde(rename = "SttlmAndCtdyDtls", skip_serializing_if = "Option::is_none")]
    pub sttlm_and_ctdy_dtls: Option<FundSettlementParameters12>,
    #[validate]
    #[serde(rename = "PhysDlvryInd")]
    pub phys_dlvry_ind: YesNoIndicator,
    #[serde(rename = "PhysDlvryDtls", skip_serializing_if = "Option::is_none")]
    pub phys_dlvry_dtls: Option<DeliveryParameters3>,
    #[validate(length(min = 0, max = 4,))]
    #[serde(rename = "StffClntBrkdwn", default)]
    pub stff_clnt_brkdwn: Vec<InvestmentFundsOrderBreakdown2>,
    #[serde(rename = "Rfnd", skip_serializing_if = "Option::is_none")]
    pub rfnd: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SbcptIntrst", skip_serializing_if = "Option::is_none")]
    pub sbcpt_intrst: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none")]
    pub csh_sttlm_dtls: Option<PaymentTransaction70>,
    #[serde(rename = "NonStdSttlmInf", skip_serializing_if = "Option::is_none")]
    pub non_std_sttlm_inf: Option<Max350Text>,
    #[serde(rename = "PrtlSttlmOfUnits", skip_serializing_if = "Option::is_none")]
    pub prtl_sttlm_of_units: Option<PercentageRate>,
    #[serde(rename = "FinAdvc", skip_serializing_if = "Option::is_none")]
    pub fin_advc: Option<FinancialAdvice1Code>,
    #[serde(rename = "NgtdTrad", skip_serializing_if = "Option::is_none")]
    pub ngtd_trad: Option<NegotiatedTrade1Code>,
    #[serde(rename = "LateRpt", skip_serializing_if = "Option::is_none")]
    pub late_rpt: Option<LateReport1Code>,
    #[serde(rename = "PrtlSttlmOfCsh", skip_serializing_if = "Option::is_none")]
    pub prtl_sttlm_of_csh: Option<PercentageRate>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "RltdPtyDtls", default)]
    pub rltd_pty_dtls: Vec<Intermediary39>,
    #[serde(rename = "Equlstn", skip_serializing_if = "Option::is_none")]
    pub equlstn: Option<Equalisation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SrcOfCsh", default)]
    pub src_of_csh: Vec<SourceOfCash1Choice>,
    #[serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none")]
    pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
    #[serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none")]
    pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
    #[serde(rename = "SgntrTp", skip_serializing_if = "Option::is_none")]
    pub sgntr_tp: Option<SignatureType1Choice>,
    #[serde(rename = "OrdrWvrDtls", skip_serializing_if = "Option::is_none")]
    pub ordr_wvr_dtls: Option<OrderWaiver1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tax31 {
    #[serde(rename = "Tp")]
    pub tp: TaxType3Choice,
    #[validate]
    #[serde(rename = "ApldAmt")]
    pub apld_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "ApldRate", skip_serializing_if = "Option::is_none")]
    pub apld_rate: Option<PercentageRate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification113>,
    #[serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none")]
    pub tax_clctn_dtls: Option<TaxCalculationInformation10>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IncomePreference1Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "DRIP")]
    Drip,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SignatureType2Code {
    #[serde(rename = "DIGI")]
    Digi,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ORIG")]
    Orig,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxationBasis5Code {
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "NEAM")]
    Neam,
    #[serde(rename = "NAVP")]
    Navp,
    #[serde(rename = "PERU")]
    Peru,
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
pub struct GermanBankleitzahlIdentifier {
    #[validate(regex = "GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderOriginatorEligibility1Code {
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PROF")]
    Prof,
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
pub struct SwissBcIdentifier {
    #[validate(regex = "SWISS_BC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice10Code {
    #[serde(rename = "BIDE")]
    Bide,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "NAVL")]
    Navl,
    #[serde(rename = "CREA")]
    Crea,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "SWNG")]
    Swng,
    #[serde(rename = "MIDD")]
    Midd,
    #[serde(rename = "RINV")]
    Rinv,
    #[serde(rename = "SWIC")]
    Swic,
    #[serde(rename = "DDVR")]
    Ddvr,
    #[serde(rename = "ACTU")]
    Actu,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RoundingDirection2Code {
    #[serde(rename = "RDUP")]
    Rdup,
    #[serde(rename = "RDWN")]
    Rdwn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialAdvice1Code {
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "NREC")]
    Nrec,
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
pub struct EuDividendStatusType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EuDividendStatus1Code>,
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
pub struct EuDividendStatusType2Choice {
    #[serde(flatten)]
    pub value: EuDividendStatusType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxType17Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType3Choice {
    #[serde(flatten)]
    pub value: TaxType3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundCashAccount2Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CPFO")]
    Cpfo,
    #[serde(rename = "CPFS")]
    Cpfs,
    #[serde(rename = "SRSA")]
    Srsa,
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
pub struct ReceivingPartiesAndAccount16 {
    #[serde(rename = "RcvrsCtdnDtls", skip_serializing_if = "Option::is_none")]
    pub rcvrs_ctdn_dtls: Option<PartyIdentificationAndAccount147>,
    #[serde(rename = "RcvrsIntrmy1Dtls", skip_serializing_if = "Option::is_none")]
    pub rcvrs_intrmy_1_dtls: Option<PartyIdentificationAndAccount147>,
    #[serde(rename = "RcvrsIntrmy2Dtls", skip_serializing_if = "Option::is_none")]
    pub rcvrs_intrmy_2_dtls: Option<PartyIdentificationAndAccount147>,
    #[validate]
    #[serde(rename = "RcvgAgtDtls")]
    pub rcvg_agt_dtls: PartyIdentificationAndAccount147,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SmallNetworkIdentifier {
    #[validate(regex = "SMALL_NETWORK_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxationBasis2Code {
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PERU")]
    Peru,
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
pub struct BranchData {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CusipIdentifier {
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
pub struct Intermediary39 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification113,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account22>,
    #[serde(rename = "OrdrOrgtrElgblty", skip_serializing_if = "Option::is_none")]
    pub ordr_orgtr_elgblty: Option<OrderOriginatorEligibility1Code>,
    #[serde(rename = "TradgPtyCpcty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty_cpcty: Option<TradingCapacity8Code>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<InvestmentFundRole2Choice>,
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
pub struct PartyIdentification90ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification90Choice {
    #[serde(flatten)]
    pub value: PartyIdentification90ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BeneficiaryCertificationCompletion1Code {
    #[serde(rename = "NCER")]
    Ncer,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "PHYS")]
    Phys,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AustrianBankleitzahlIdentifier {
    #[validate(regex = "AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
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
pub struct ExemptionReason1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxExemptReason1Code>,
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
pub struct ExemptionReason1Choice {
    #[serde(flatten)]
    pub value: ExemptionReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeTransactionCondition5Code>,
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
pub struct TradeTransactionCondition8Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderWaiver1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "OrdrWvrRsn", default)]
    pub ordr_wvr_rsn: Vec<OrderWaiverReason3Choice>,
    #[serde(rename = "InfVal", skip_serializing_if = "Option::is_none")]
    pub inf_val: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConsolidatedTapeAssociationIdentifier {
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
pub struct ChipsParticipantIdentifier {
    #[validate(regex = "CHIPS_PARTICIPANT_IDENTIFIER_REGEX")]
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
pub struct OrderBreakdownType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundOrderType5Code>,
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
pub struct OrderBreakdownType1Choice {
    #[serde(flatten)]
    pub value: OrderBreakdownType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyIdentificationType7Code {
    #[serde(rename = "ATIN")]
    Atin,
    #[serde(rename = "IDCD")]
    Idcd,
    #[serde(rename = "NRIN")]
    Nrin,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "POCD")]
    Pocd,
    #[serde(rename = "SOCS")]
    Socs,
    #[serde(rename = "SRSA")]
    Srsa,
    #[serde(rename = "GUNL")]
    Gunl,
    #[serde(rename = "GTIN")]
    Gtin,
    #[serde(rename = "ITIN")]
    Itin,
    #[serde(rename = "CPFA")]
    Cpfa,
    #[serde(rename = "AREG")]
    Areg,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "EMID")]
    Emid,
    #[serde(rename = "NINV")]
    Ninv,
    #[serde(rename = "INCL")]
    Incl,
    #[serde(rename = "GIIN")]
    Giin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundFee1Code {
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "BRKF")]
    Brkf,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CDPL")]
    Cdpl,
    #[serde(rename = "CDSC")]
    Cdsc,
    #[serde(rename = "CBCH")]
    Cbch,
    #[serde(rename = "DLEV")]
    Dlev,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "ADDF")]
    Addf,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "SHIP")]
    Ship,
    #[serde(rename = "SWIT")]
    Swit,
    #[serde(rename = "UCIC")]
    Ucic,
    #[serde(rename = "REGF")]
    Regf,
    #[serde(rename = "PENA")]
    Pena,
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
pub struct InvestmentAccount60 {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InvestmentAccountType1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentCard25 {
    #[serde(rename = "Tp")]
    pub tp: CardType1Code,
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[validate]
    #[serde(rename = "HldrNm")]
    pub hldr_nm: Max35Text,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoYearMonth>,
    #[validate]
    #[serde(rename = "XpryDt")]
    pub xpry_dt: IsoYearMonth,
    #[serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none")]
    pub card_issr_nm: Option<Max35Text>,
    #[serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none")]
    pub card_issr_id: Option<PartyIdentification113>,
    #[serde(rename = "SctyCd", skip_serializing_if = "Option::is_none")]
    pub scty_cd: Option<Max35Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max3Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NegotiatedTrade1Code {
    #[serde(rename = "NEGO")]
    Nego,
    #[serde(rename = "NNGO")]
    Nngo,
    #[serde(rename = "UNKW")]
    Unkw,
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
pub struct SicovamIdentifier {
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
pub struct TypeOfPrice46ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice46Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WaivingInstruction2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WaivingInstruction1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WaivingInstruction2Choice {
    #[serde(flatten)]
    pub value: WaivingInstruction2ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SourceOfCash1Code {
    #[serde(rename = "ALMY")]
    Almy,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "EMIN")]
    Emin,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "INHE")]
    Inhe,
    #[serde(rename = "INLQ")]
    Inlq,
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REPY")]
    Repy,
    #[serde(rename = "SEAQ")]
    Seaq,
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "SVGS")]
    Svgs,
    #[serde(rename = "SELF")]
    XSelf,
    #[serde(rename = "WINS")]
    Wins,
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
pub struct QuickIdentifier {
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
pub struct SouthAfricanNccIdentifier {
    #[validate(regex = "SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX")]
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
pub struct HongKongBankIdentifier {
    #[validate(regex = "HONG_KONG_BANK_IDENTIFIER_REGEX")]
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
pub struct IdentificationSource1ChoiceEnum {
    #[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
    pub dmst: Option<CountryCode>,
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
pub struct IdentificationSource1Choice {
    #[serde(flatten)]
    pub value: IdentificationSource1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress4 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NewZealandNccIdentifier {
    #[validate(regex = "NEW_ZEALAND_NCC_IDENTIFIER_REGEX")]
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
pub struct DeliveryParameters3 {
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: NameAndAddress4,
    #[serde(rename = "IssdCertNb", skip_serializing_if = "Option::is_none")]
    pub issd_cert_nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SpanishDomesticInterbankingIdentifier {
    #[validate(regex = "SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX")]
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
pub struct ChipsUniversalIdentifier {
    #[validate(regex = "CHIPS_UNIVERSAL_IDENTIFIER_REGEX")]
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
pub struct SubscriptionBulkOrderConfirmationV04 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
    pub pool_ref: Option<AdditionalReference9>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrvsRef", default)]
    pub prvs_ref: Vec<AdditionalReference8>,
    #[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ref: Option<AdditionalReference8>,
    #[validate]
    #[serde(rename = "BlkExctnDtls")]
    pub blk_exctn_dtls: SubscriptionBulkExecution4,
    #[serde(rename = "CpyDtls", skip_serializing_if = "Option::is_none")]
    pub cpy_dtls: Option<CopyInformation4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Xtnsn", default)]
    pub xtnsn: Vec<Extension1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxBasis1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxBasis1Choice {
    #[serde(flatten)]
    pub value: TaxBasis1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryAndResidentialStatusType2 {
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[serde(rename = "ResdtlSts")]
    pub resdtl_sts: ResidentialStatus1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LateReport1Code {
    #[serde(rename = "LAT1")]
    Lat1,
    #[serde(rename = "LAT2")]
    Lat2,
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
pub struct EuroclearClearstreamIdentifier {
    #[validate(length(min = 1, max = 12,))]
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
pub struct GenericIdentification78 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification30,
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
pub struct IsoYearMonth {
    #[validate(regex = "ISO_YEAR_MONTH_REGEX")]
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
pub struct GenericIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
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
pub struct PlaceOfTradeIdentification1ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<AnyBicIdentifier>,
    #[serde(rename = "Xchg", skip_serializing_if = "Option::is_none")]
    pub xchg: Option<MicIdentifier>,
    #[serde(rename = "OverTheCntr", skip_serializing_if = "Option::is_none")]
    pub over_the_cntr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOfTradeIdentification1Choice {
    #[serde(flatten)]
    pub value: PlaceOfTradeIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortugueseNccIdentifier {
    #[validate(regex = "PORTUGUESE_NCC_IDENTIFIER_REGEX")]
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
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat42ChoiceEnum {
    #[serde(rename = "YrMnthDay", skip_serializing_if = "Option::is_none")]
    pub yr_mnth_day: Option<IsoDate>,
    #[serde(rename = "YrMnth", skip_serializing_if = "Option::is_none")]
    pub yr_mnth: Option<IsoYearMonth>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat42Choice {
    #[serde(flatten)]
    pub value: DateFormat42ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryReceiptType2Code {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "APMT")]
    Apmt,
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
pub struct CancellationRight1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancellationRight1Code>,
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
pub struct CancellationRight1Choice {
    #[serde(flatten)]
    pub value: CancellationRight1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxExemptReason1Code {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MASA")]
    Masa,
    #[serde(rename = "MISA")]
    Misa,
    #[serde(rename = "SISA")]
    Sisa,
    #[serde(rename = "IISA")]
    Iisa,
    #[serde(rename = "CUYP")]
    Cuyp,
    #[serde(rename = "PRYP")]
    Pryp,
    #[serde(rename = "ASTR")]
    Astr,
    #[serde(rename = "EMPY")]
    Empy,
    #[serde(rename = "EMCY")]
    Emcy,
    #[serde(rename = "EPRY")]
    Epry,
    #[serde(rename = "ECYE")]
    Ecye,
    #[serde(rename = "NFPI")]
    Nfpi,
    #[serde(rename = "NFQP")]
    Nfqp,
    #[serde(rename = "DECP")]
    Decp,
    #[serde(rename = "IRAC")]
    Irac,
    #[serde(rename = "IRAR")]
    Irar,
    #[serde(rename = "KEOG")]
    Keog,
    #[serde(rename = "PFSP")]
    Pfsp,
    #[serde(rename = "401K")]
    X401K,
    #[serde(rename = "SIRA")]
    Sira,
    #[serde(rename = "403B")]
    X403B,
    #[serde(rename = "457X")]
    X457X,
    #[serde(rename = "RIRA")]
    Rira,
    #[serde(rename = "RIAN")]
    Rian,
    #[serde(rename = "RCRF")]
    Rcrf,
    #[serde(rename = "RCIP")]
    Rcip,
    #[serde(rename = "EIFP")]
    Eifp,
    #[serde(rename = "EIOP")]
    Eiop,
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
pub struct InvestmentFundsOrderBreakdown2 {
    #[serde(rename = "OrdrBrkdwnTp")]
    pub ordr_brkdwn_tp: OrderBreakdownType1Choice,
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
pub struct Tax32 {
    #[serde(rename = "Tp")]
    pub tp: TaxType3Choice,
    #[serde(rename = "InftvAmt", skip_serializing_if = "Option::is_none")]
    pub inftv_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "InftvRate", skip_serializing_if = "Option::is_none")]
    pub inftv_rate: Option<PercentageRate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[validate]
    #[serde(rename = "XmptnInd")]
    pub xmptn_ind: YesNoIndicator,
    #[serde(rename = "XmptnRsn", skip_serializing_if = "Option::is_none")]
    pub xmptn_rsn: Option<ExemptionReason1Choice>,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification113>,
    #[serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none")]
    pub tax_clctn_dtls: Option<TaxCalculationInformation10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument57 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
    pub splmtry_id: Option<Max35Text>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
    pub scties_form: Option<FormOfSecurity1Code>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
    pub pdct_grp: Option<Max140Text>,
    #[serde(rename = "SrsId", skip_serializing_if = "Option::is_none")]
    pub srs_id: Option<Series1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceMethod1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "HIST")]
    Hist,
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
pub struct DutchIdentifier {
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
pub struct TransactionChannelType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TransactionChannel2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionChannelType1Choice {
    #[serde(flatten)]
    pub value: TransactionChannelType1ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderWaiverReason1Code {
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "BMIN")]
    Bmin,
    #[serde(rename = "CUTO")]
    Cuto,
    #[serde(rename = "COMW")]
    Comw,
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
pub struct AdditionalReference8 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification113>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(
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
pub struct Account22 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification113>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IrishNscIdentifier {
    #[validate(regex = "IRISH_NSC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace2Code {
    #[serde(rename = "SHHE")]
    Shhe,
    #[serde(rename = "ALLP")]
    Allp,
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
pub struct SecurityIdentification25ChoiceEnum {
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification25Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Equalisation1 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
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
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundOrderType8Code {
    #[serde(rename = "BEDB")]
    Bedb,
    #[serde(rename = "INVP")]
    Invp,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "RGSV")]
    Rgsv,
    #[serde(rename = "RGSU")]
    Rgsu,
    #[serde(rename = "RDIV")]
    Rdiv,
    #[serde(rename = "STAF")]
    Staf,
    #[serde(rename = "WIDP")]
    Widp,
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
pub struct Bloomberg2Identifier {
    #[validate(regex = "BLOOMBERG_2_IDENTIFIER_REGEX")]
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
pub struct BelgianIdentifier {
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
pub struct InvestmentAccount58 {
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OwnrId", default)]
    pub ownr_id: Vec<PartyIdentification113>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification113>,
    #[serde(rename = "OrdrOrgtrElgblty", skip_serializing_if = "Option::is_none")]
    pub ordr_orgtr_elgblty: Option<OrderOriginatorEligibility1Code>,
    #[serde(rename = "SubAcctDtls", skip_serializing_if = "Option::is_none")]
    pub sub_acct_dtls: Option<SubAccount6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalReference9 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification113>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccountType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundCashAccount2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccountType1Choice {
    #[serde(flatten)]
    pub value: InvestmentAccountType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat8ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText6>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndAnyBicIdentifier1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat8Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SourceOfCash1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SourceOfCash1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SourceOfCash1Choice {
    #[serde(flatten)]
    pub value: SourceOfCash1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentFundRole2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentFundRole2Choice {
    #[serde(flatten)]
    pub value: InvestmentFundRole2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxType17Code {
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "NATI")]
    Nati,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "KAPA")]
    Kapa,
    #[serde(rename = "NKAP")]
    Nkap,
    #[serde(rename = "INPO")]
    Inpo,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "WTAX")]
    Wtax,
    #[serde(rename = "INHT")]
    Inht,
    #[serde(rename = "SOSU")]
    Sosu,
    #[serde(rename = "CTAX")]
    Ctax,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "EUTR")]
    Eutr,
    #[serde(rename = "AKT1")]
    Akt1,
    #[serde(rename = "AKT2")]
    Akt2,
    #[serde(rename = "ZWIS")]
    Zwis,
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
pub struct PaymentTransaction70 {
    #[serde(rename = "PmtInstrm")]
    pub pmt_instrm: PaymentInstrument20Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition11Code {
    #[serde(rename = "NOMC")]
    Nomc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EuDividendStatus1Code {
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "DIVO")]
    Divo,
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
pub struct GenericIdentification164 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdTp")]
    pub id_tp: OtherIdentification3Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancellationRight1Code {
    #[serde(rename = "VALI")]
    Vali,
    #[serde(rename = "NOXO")]
    Noxo,
    #[serde(rename = "NOWA")]
    Nowa,
    #[serde(rename = "NOIN")]
    Noin,
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
pub struct DirectDebitMandate6 {
    #[validate]
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: AccountIdentificationAndName5,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentification113>,
    #[serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none")]
    pub dbtr_tax_id_nb: Option<Max35Text>,
    #[serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub dbtr_ntl_regn_nb: Option<Max35Text>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentification113>,
    #[validate]
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: FinancialInstitutionIdentification10,
    #[serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_brnch: Option<BranchData>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<FinancialInstitutionIdentification10>,
    #[serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_brnch: Option<BranchData>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FedwireRoutingNumberIdentifier {
    #[validate(regex = "FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX")]
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
pub struct PartyIdentificationAndAccount147 {
    #[validate]
    #[serde(rename = "PtyId")]
    pub pty_id: PartyIdentification113,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RussianCentralBankIdentificationCodeIdentifier {
    #[validate(regex = "RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX")]
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
pub struct ForeignExchangeTerms33 {
    #[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
    pub to_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
    pub fr_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<IsoDateTime>,
    #[serde(rename = "QtgInstn", skip_serializing_if = "Option::is_none")]
    pub qtg_instn: Option<PartyIdentification113>,
}
#[derive(
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
pub enum EuCapitalGain2Code {
    #[serde(rename = "EUSI")]
    Eusi,
    #[serde(rename = "EUSO")]
    Euso,
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
pub struct CopyInformation4 {
    #[validate]
    #[serde(rename = "CpyInd")]
    pub cpy_ind: YesNoIndicator,
    #[serde(rename = "OrgnlRcvr", skip_serializing_if = "Option::is_none")]
    pub orgnl_rcvr: Option<AnyBicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InformativeTax1 {
    #[serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none")]
    pub taxbl_incm_per_dvdd: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none")]
    pub eu_cptl_gn: Option<EuCapitalGain3Choice>,
    #[serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none")]
    pub eu_dvdd_sts: Option<EuDividendStatusType2Choice>,
    #[serde(rename = "PctgOfDebtClm", skip_serializing_if = "Option::is_none")]
    pub pctg_of_debt_clm: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvTax", default)]
    pub indv_tax: Vec<Tax32>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Fee2 {
    #[serde(rename = "Tp")]
    pub tp: ChargeType5Choice,
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<ChargeBasis2Choice>,
    #[serde(rename = "StdAmt", skip_serializing_if = "Option::is_none")]
    pub std_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StdRate", skip_serializing_if = "Option::is_none")]
    pub std_rate: Option<PercentageRate>,
    #[serde(rename = "DscntDtls", skip_serializing_if = "Option::is_none")]
    pub dscnt_dtls: Option<ChargeOrCommissionDiscount1>,
    #[serde(rename = "ApldAmt", skip_serializing_if = "Option::is_none")]
    pub apld_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ApldRate", skip_serializing_if = "Option::is_none")]
    pub apld_rate: Option<PercentageRate>,
    #[serde(rename = "NonStdSLARef", skip_serializing_if = "Option::is_none")]
    pub non_std_sla_ref: Option<Max35Text>,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification113>,
    #[validate]
    #[serde(rename = "InftvInd")]
    pub inftv_ind: YesNoIndicator,
}
#[derive(
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
    #[serde(rename = "SbcptBlkOrdrConf")]
    pub sbcpt_blk_ordr_conf: SubscriptionBulkOrderConfirmationV04,
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
pub struct ChargeType5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundFee1Code>,
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
pub struct ChargeType5Choice {
    #[serde(flatten)]
    pub value: ChargeType5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeBasis2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeBasis2Choice {
    #[serde(flatten)]
    pub value: ChargeBasis2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentificationChoiceEnum {
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentificationChoice {
    #[serde(flatten)]
    pub value: ClearingSystemMemberIdentificationChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxableIncomePerShareCalculated2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxableIncomePerShareCalculated2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxableIncomePerShareCalculated2Choice {
    #[serde(flatten)]
    pub value: TaxableIncomePerShareCalculated2ChoiceEnum,
}
