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
    static ref DUNS_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ID_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[\i-[:]][\c-[:]]*"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[B-DF-HJ-NP-TV-XZ0-9]{7,7}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EANGLN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{13,13}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IDREF_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[\i-[:]][\c-[:]]*"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.053.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatusReason4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FinancingStatusReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatusReason4Choice {
    #[serde(flatten)]
    pub value: StatusReason4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GovernanceRules2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: ID,
    #[serde(rename = "RuleId")]
    pub rule_id: GovernanceIdentification1Choice,
    #[serde(rename = "AplblLaw", skip_serializing_if = "Option::is_none")]
    pub aplbl_law: Option<Location1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Jursdctn", default)]
    pub jursdctn: Vec<Location1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DunsIdentifier {
    #[validate(regex = "DUNS_IDENTIFIER_REGEX")]
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
pub struct ReconciliationList1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Dt")]
    pub dt: IsoDate,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdDoc", default)]
    pub rltd_doc: Vec<IDREF>,
    #[validate]
    #[serde(rename = "Rcpt")]
    pub rcpt: IDREF,
    #[validate]
    #[serde(rename = "Advsr")]
    pub advsr: IDREF,
    #[validate]
    #[serde(rename = "Params")]
    pub params: FinancialItemParameters1,
    #[validate]
    #[serde(rename = "PmtRef")]
    pub pmt_ref: PaymentIdentification1,
    #[validate]
    #[serde(rename = "PmtMeans")]
    pub pmt_means: PaymentMeans1,
    #[validate]
    #[serde(rename = "PmtDt")]
    pub pmt_dt: IsoDate,
    #[validate]
    #[serde(rename = "PmtTerms")]
    pub pmt_terms: PaymentTerms6,
    #[validate]
    #[serde(rename = "PmtAmt")]
    pub pmt_amt: CurrencyAndAmount,
    #[validate(length(min = 1,))]
    #[serde(rename = "Itm", default)]
    pub itm: Vec<FinancialItem1<A>>,
    #[validate]
    #[serde(rename = "ItmCnt")]
    pub itm_cnt: Max15NumericText,
    #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdDoc", default)]
    pub assoctd_doc: Vec<IDREF>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max2000Text>,
    #[serde(rename = "VldtnStsInf", skip_serializing_if = "Option::is_none")]
    pub vldtn_sts_inf: Option<ValidationStatusInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalClearingSystemIdentification1Code {
    #[validate(length(min = 1, max = 5,))]
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
pub struct FinancialIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
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
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: FinancialIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancingAllowedSummary1 {
    #[validate]
    #[serde(rename = "FincdItmNb")]
    pub fincd_itm_nb: Number,
    #[validate]
    #[serde(rename = "TtlAccptdItmsAmt")]
    pub ttl_accptd_itms_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "ApldPctg", skip_serializing_if = "Option::is_none")]
    pub apld_pctg: Option<PercentageRate>,
    #[validate]
    #[serde(rename = "TtlFincdAmt")]
    pub ttl_fincd_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "FincgDtDtls", skip_serializing_if = "Option::is_none")]
    pub fincg_dt_dtls: Option<FinancingDateDetails1>,
    #[serde(rename = "CdtAcct", skip_serializing_if = "Option::is_none")]
    pub cdt_acct: Option<CashAccount7>,
    #[serde(rename = "FincgAcct", skip_serializing_if = "Option::is_none")]
    pub fincg_acct: Option<CashAccount7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CopyDuplicate1Code {
    #[serde(rename = "CODU")]
    Codu,
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "DUPL")]
    Dupl,
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
pub struct PersonIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPersonIdentification1Code>,
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
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: PersonIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification3ChoiceEnum {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "PrtryAcct", skip_serializing_if = "Option::is_none")]
    pub prtry_acct: Option<SimpleIdentificationInformation2>,
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification3Choice {
    #[serde(flatten)]
    pub value: AccountIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalTradeMarket1Code {
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
pub struct PostalAddress6 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
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
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[validate(length(min = 0, max = 7,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority3Code {
    #[serde(rename = "URGT")]
    Urgt,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
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
pub struct OrganisationIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalOrganisationIdentification1Code>,
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
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification2 {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<BicIdentifier>,
    #[serde(rename = "IBEI", skip_serializing_if = "Option::is_none")]
    pub ibei: Option<IbeiIdentifier>,
    #[serde(rename = "BEI", skip_serializing_if = "Option::is_none")]
    pub bei: Option<BeiIdentifier>,
    #[serde(rename = "EANGLN", skip_serializing_if = "Option::is_none")]
    pub eangln: Option<EanglnIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "DUNS", skip_serializing_if = "Option::is_none")]
    pub duns: Option<DunsIdentifier>,
    #[serde(rename = "BkPtyId", skip_serializing_if = "Option::is_none")]
    pub bk_pty_id: Option<Max35Text>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party9ChoiceEnum {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<PartyIdentification42>,
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    pub fi_id: Option<BranchAndFinancialInstitutionIdentification5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party9Choice {
    #[serde(flatten)]
    pub value: Party9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party8ChoiceEnum {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification6>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party8Choice {
    #[serde(flatten)]
    pub value: Party8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Adjustment5 {
    #[serde(rename = "Drctn")]
    pub drctn: AdjustmentDirection1Code,
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
pub struct FinancingDateDetails1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "BookDt", default)]
    pub book_dt: Vec<IsoDate>,
    #[validate]
    #[serde(rename = "CdtDt")]
    pub cdt_dt: IsoDate,
    #[serde(rename = "DbtDt", skip_serializing_if = "Option::is_none")]
    pub dbt_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericPersonIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
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
pub struct EncapsulatedBusinessMessage1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Hdr", skip_serializing_if = "Option::is_none")]
    pub hdr: Option<BusinessApplicationHeader1<A>>,
    #[serde(rename = "Prfx", skip_serializing_if = "Option::is_none")]
    pub prfx: Option<ID>,
    #[validate]
    #[serde(rename = "Prtl")]
    pub prtl: YesNoIndicator,
    #[validate]
    #[serde(rename = "Msg")]
    pub msg: StrictPayload<B>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessLetter1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "ApplCntxt", skip_serializing_if = "Option::is_none")]
    pub appl_cntxt: Option<Max35Text>,
    #[validate]
    #[serde(rename = "LttrIdr")]
    pub lttr_idr: QualifiedDocumentInformation1,
    #[validate]
    #[serde(rename = "Dt")]
    pub dt: IsoDate,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdLttr", default)]
    pub rltd_lttr: Vec<QualifiedDocumentInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdMsg", default)]
    pub rltd_msg: Vec<QualifiedDocumentInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CnttIdr", default)]
    pub cntt_idr: Vec<Max35Text>,
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority3Code>,
    #[validate]
    #[serde(rename = "Orgtr")]
    pub orgtr: QualifiedPartyIdentification1,
    #[validate(length(min = 1,))]
    #[serde(rename = "PmryRcpt", default)]
    pub pmry_rcpt: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Sndr", default)]
    pub sndr: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AuthstnUsr", default)]
    pub authstn_usr: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RspnRcpt", default)]
    pub rspn_rcpt: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CpyRcpt", default)]
    pub cpy_rcpt: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrPty", default)]
    pub othr_pty: Vec<QualifiedPartyIdentification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdDoc", default)]
    pub assoctd_doc: Vec<QualifiedDocumentInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GovngCtrct", default)]
    pub govng_ctrct: Vec<QualifiedDocumentInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LglCntxt", default)]
    pub lgl_cntxt: Vec<GovernanceRules2>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max2000Text>,
    #[serde(rename = "Ntce", skip_serializing_if = "Option::is_none")]
    pub ntce: Option<Max350Text>,
    #[serde(rename = "VldtnStsInf", skip_serializing_if = "Option::is_none")]
    pub vldtn_sts_inf: Option<ValidationStatusInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DgtlSgntr", default)]
    pub dgtl_sgntr: Vec<QualifiedPartyAndXmlSignature1<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDocumentType1Code {
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
pub struct FinancialItem1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "ItmCntxt")]
    pub itm_cntxt: FinancialItemParameters1,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinDocRef", default)]
    pub fin_doc_ref: Vec<IDREF>,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[validate]
    #[serde(rename = "TtlAmt")]
    pub ttl_amt: InvoiceTotals1,
    #[serde(rename = "DueAmt", skip_serializing_if = "Option::is_none")]
    pub due_amt: Option<ActiveCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InstlmtInf", default)]
    pub instlmt_inf: Vec<Instalment2>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max2000Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdDoc", default)]
    pub assoctd_doc: Vec<IDREF>,
    #[serde(rename = "VldtnStsInf", skip_serializing_if = "Option::is_none")]
    pub vldtn_sts_inf: Option<ValidationStatusInformation1>,
    #[serde(rename = "FincgSts", skip_serializing_if = "Option::is_none")]
    pub fincg_sts: Option<FinancingInformationAndStatus1>,
    #[serde(rename = "PrtryDtls", skip_serializing_if = "Option::is_none")]
    pub prtry_dtls: Option<SupplementaryData1<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QualifiedDocumentInformation1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: ID,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<IDREF>,
    #[serde(rename = "ItmListIdr", skip_serializing_if = "Option::is_none")]
    pub itm_list_idr: Option<Max35Text>,
    #[serde(rename = "ItmIdr", skip_serializing_if = "Option::is_none")]
    pub itm_idr: Option<Max35Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max6Text>,
    #[validate]
    #[serde(rename = "ElctrncOrgnl")]
    pub elctrnc_orgnl: YesNoIndicator,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "Dgst", default)]
    pub dgst: Vec<AlgorithmAndDigest1>,
    #[serde(rename = "DocTp", skip_serializing_if = "Option::is_none")]
    pub doc_tp: Option<ExternalDocumentType1Code>,
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<Max2048Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AttchdFile", default)]
    pub attchd_file: Vec<BinaryFile1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentification5 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericPersonIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SingleQualifiedPartyIdentification1 {
    #[validate]
    #[serde(rename = "BasePty")]
    pub base_pty: TradeParty1,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "RltvIdr", default)]
    pub rltv_idr: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxExemptionReasonFormatChoiceEnum {
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<Max140Text>,
    #[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
    pub strd: Option<TaxExemptReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxExemptionReasonFormatChoice {
    #[serde(flatten)]
    pub value: TaxExemptionReasonFormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification2ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
    pub bic_or_bei: Option<AnyBicIdentifier>,
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
pub struct PartyIdentification2Choice {
    #[serde(flatten)]
    pub value: PartyIdentification2ChoiceEnum,
}
#[derive(
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
pub struct ID {
    #[validate(regex = "ID_REGEX")]
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
pub struct Contacts3 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max2048Text>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Max35Text>,
    #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<Max35Text>,
    #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<Max35Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalServiceLevel1Code {
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
pub struct CashAccount7 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification3Choice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<CurrencyCode>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashAccountType4Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "TAXE")]
    Taxe,
    #[serde(rename = "CISH")]
    Cish,
    #[serde(rename = "TRAS")]
    Tras,
    #[serde(rename = "SACC")]
    Sacc,
    #[serde(rename = "CACC")]
    Cacc,
    #[serde(rename = "SVGS")]
    Svgs,
    #[serde(rename = "ONDP")]
    Ondp,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "NREX")]
    Nrex,
    #[serde(rename = "MOMA")]
    Moma,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "SLRY")]
    Slry,
    #[serde(rename = "ODFT")]
    Odft,
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
pub struct OriginalInvoiceInformation1 {
    #[validate]
    #[serde(rename = "DocNb")]
    pub doc_nb: Max35Text,
    #[validate]
    #[serde(rename = "TtlInvcAmt")]
    pub ttl_invc_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
    #[validate]
    #[serde(rename = "PmtDueDt")]
    pub pmt_due_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPersonIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GovernanceIdentification1Code {
    #[serde(rename = "ISPR")]
    Ispr,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "UCPR")]
    Ucpr,
    #[serde(rename = "URDG")]
    Urdg,
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
pub struct ExternalCategoryPurpose1Code {
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
pub struct UnicodeChartsCode {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentMethod4Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "DD")]
    Dd,
    #[serde(rename = "TRA")]
    Tra,
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
pub struct BranchAndFinancialInstitutionIdentification4 {
    #[validate]
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification7,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification7 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeiIdentifier {
    #[validate(regex = "BEI_IDENTIFIER_REGEX")]
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
pub struct InvoiceTotals1 {
    #[validate]
    #[serde(rename = "TtlTaxblAmt")]
    pub ttl_taxbl_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "TtlTaxAmt")]
    pub ttl_tax_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none")]
    pub adjstmnt: Option<Adjustment5>,
    #[validate]
    #[serde(rename = "TtlInvcAmt")]
    pub ttl_invc_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "PmtDueDt")]
    pub pmt_due_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvoiceFinancingDetails1 {
    #[validate]
    #[serde(rename = "OrgnlInvcInf")]
    pub orgnl_invc_inf: OriginalInvoiceInformation1,
    #[serde(rename = "Spplr", skip_serializing_if = "Option::is_none")]
    pub spplr: Option<PartyIdentification8>,
    #[validate]
    #[serde(rename = "InvcFincgRslt")]
    pub invc_fincg_rslt: FinancingResult1,
    #[validate(length(min = 0,))]
    #[serde(rename = "InstlmtFincgInf", default)]
    pub instlmt_fincg_inf: Vec<InstalmentFinancingInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentification3Enum {
    #[serde(rename = "CstmrNb", skip_serializing_if = "Option::is_none")]
    pub cstmr_nb: Option<Max35Text>,
    #[serde(rename = "MplyrIdNb", skip_serializing_if = "Option::is_none")]
    pub mplyr_id_nb: Option<Max35Text>,
    #[serde(rename = "AlnRegnNb", skip_serializing_if = "Option::is_none")]
    pub aln_regn_nb: Option<Max35Text>,
    #[serde(rename = "DrvrsLicNb", skip_serializing_if = "Option::is_none")]
    pub drvrs_lic_nb: Option<Max35Text>,
    #[serde(rename = "PsptNb", skip_serializing_if = "Option::is_none")]
    pub pspt_nb: Option<Max35Text>,
    #[serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none")]
    pub scl_scty_nb: Option<Max35Text>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<GenericIdentification4>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "IdntyCardNb", skip_serializing_if = "Option::is_none")]
    pub idnty_card_nb: Option<Max35Text>,
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentification3 {
    #[serde(flatten)]
    pub value: PersonIdentification3Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericFinancialIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
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
pub struct IbeiIdentifier {
    #[validate(regex = "IBEI_IDENTIFIER_REGEX")]
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
pub struct EanglnIdentifier {
    #[validate(regex = "EANGLN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TechnicalValidationStatus1Code {
    #[serde(rename = "RCCF")]
    Rccf,
    #[serde(rename = "RCER")]
    Rcer,
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
pub struct ClearingSystemIdentification2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalClearingSystemIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemIdentification2Choice {
    #[serde(flatten)]
    pub value: ClearingSystemIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Location1 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<CountrySubdivision1Choice>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "Txt", default)]
    pub txt: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GovernanceIdentification1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GovernanceIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GovernanceIdentification1Choice {
    #[serde(flatten)]
    pub value: GovernanceIdentification1ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentTime1Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "EMTD")]
    Emtd,
    #[serde(rename = "EPRD")]
    Eprd,
    #[serde(rename = "PRMD")]
    Prmd,
    #[serde(rename = "IREC")]
    Irec,
    #[serde(rename = "PRMR")]
    Prmr,
    #[serde(rename = "EPRR")]
    Eprr,
    #[serde(rename = "EMTR")]
    Emtr,
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
pub struct CountrySubdivision1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max35Text>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountrySubdivision1Choice {
    #[serde(flatten)]
    pub value: CountrySubdivision1ChoiceEnum,
}
#[derive(
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
pub struct GenericOrganisationIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
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
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
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
pub struct InstalmentFinancingInformation1 {
    #[validate]
    #[serde(rename = "InstlmtSeqId")]
    pub instlmt_seq_id: Max70Text,
    #[validate]
    #[serde(rename = "InstlmtTtlAmt")]
    pub instlmt_ttl_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "InstlmtFincgRslt")]
    pub instlmt_fincg_rslt: FinancingResult1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeMarket1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification20>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalTradeMarket1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeMarket1Choice {
    #[serde(flatten)]
    pub value: TradeMarket1ChoiceEnum,
}
#[derive(
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
pub struct FinancialInstitutionIdentification7 {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<BicIdentifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentPeriod1 {
    #[serde(rename = "Cd")]
    pub cd: PaymentTime1Code,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "InvcPmtRcncltnAdvc")]
    pub invc_pmt_rcncltn_advc: InvoicePaymentReconciliationAdviceV01<A, B, C, D>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
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
pub struct PaymentIdentification1 {
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Max35Text,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancingStatusReason1Code {
    #[serde(rename = "CA01")]
    Ca01,
    #[serde(rename = "CA02")]
    Ca02,
    #[serde(rename = "AC01")]
    Ac01,
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "AC06")]
    Ac06,
    #[serde(rename = "BE08")]
    Be08,
    #[serde(rename = "BE09")]
    Be09,
    #[serde(rename = "BE10")]
    Be10,
    #[serde(rename = "BE11")]
    Be11,
    #[serde(rename = "DT02")]
    Dt02,
    #[serde(rename = "ID01")]
    Id01,
    #[serde(rename = "ID02")]
    Id02,
    #[serde(rename = "ID03")]
    Id03,
    #[serde(rename = "MI01")]
    Mi01,
    #[serde(rename = "NA01")]
    Na01,
    #[serde(rename = "CA03")]
    Ca03,
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
pub struct Party2ChoiceEnum {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification2>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party2Choice {
    #[serde(flatten)]
    pub value: Party2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm5Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification6 {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification1>,
}
#[derive(
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
pub struct TaxParty3 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<Max35Text>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxXmptnRsn", default)]
    pub tax_xmptn_rsn: Vec<TaxExemptionReasonFormatChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max2000Text {
    #[validate(length(min = 1, max = 2000,))]
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
pub struct FinancingResult1 {
    #[serde(rename = "FincgReqSts")]
    pub fincg_req_sts: RequestStatus1Code,
    #[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
    pub sts_rsn: Option<StatusReason4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlStsRsnInf", default)]
    pub addtl_sts_rsn_inf: Vec<Max105Text>,
    #[serde(rename = "FincdAmt", skip_serializing_if = "Option::is_none")]
    pub fincd_amt: Option<FinancingRateOrAmountChoice>,
}
#[derive(
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
pub struct ContactDetails2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max2048Text>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalFinancialInstitutionIdentification1Code {
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
pub struct StrictPayload<
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
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
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
pub struct CashAccount16 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2>,
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
pub struct SimpleIdentificationInformation2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancingRateOrAmountChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct FinancingRateOrAmountChoice {
    #[serde(flatten)]
    pub value: FinancingRateOrAmountChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoNormalisedDateTime {
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
pub struct PartyIdentification45 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party8Choice>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtctDtls", default)]
    pub ctct_dtls: Vec<Contacts3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancingInformationAndStatus1 {
    #[validate]
    #[serde(rename = "FincgAllwdSummry")]
    pub fincg_allwd_summry: FinancingAllowedSummary1,
    #[validate(length(min = 1,))]
    #[serde(rename = "InvcFincgDtls", default)]
    pub invc_fincg_dtls: Vec<InvoiceFinancingDetails1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessApplicationHeader1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    pub fr: Party9Choice,
    #[serde(rename = "To")]
    pub to: Party9Choice,
    #[validate]
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: Max35Text,
    #[validate]
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: Max35Text,
    #[serde(rename = "BizSvc", skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CreDt")]
    pub cre_dt: IsoNormalisedDateTime,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<SignatureEnvelope<A>>,
}
#[derive(
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
pub struct Max105Text {
    #[validate(length(min = 1, max = 105,))]
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
pub struct Party10ChoiceEnum {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification7>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party10Choice {
    #[serde(flatten)]
    pub value: Party10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDocumentPurpose1Code {
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
pub struct UpicIdentifier {
    #[validate(regex = "UPIC_IDENTIFIER_REGEX")]
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct BbanIdentifier {
    #[validate(regex = "BBAN_IDENTIFIER_REGEX")]
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
pub struct ExternalOrganisationIdentification1Code {
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
pub struct Instalment2 {
    #[validate]
    #[serde(rename = "SeqId")]
    pub seq_id: Max70Text,
    #[validate]
    #[serde(rename = "PmtDueDt")]
    pub pmt_due_dt: IsoDate,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "PmtInstrm", skip_serializing_if = "Option::is_none")]
    pub pmt_instrm: Option<PaymentMeans1>,
}
#[derive(
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
pub struct QualifiedPartyIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: ID,
    #[validate(length(min = 1,))]
    #[serde(rename = "Pty", default)]
    pub pty: Vec<SingleQualifiedPartyIdentification1>,
    #[serde(rename = "ShrtId", skip_serializing_if = "Option::is_none")]
    pub shrt_id: Option<PartyIdentification2Choice>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<GenericIdentification1>,
    #[serde(rename = "RoleDesc", skip_serializing_if = "Option::is_none")]
    pub role_desc: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPlaceOfBirth {
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
pub struct GenericIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdTp")]
    pub id_tp: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RequestStatus1Code {
    #[serde(rename = "FNCD")]
    Fncd,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "NTFD")]
    Ntfd,
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
pub struct ActiveCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct CategoryPurpose1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalCategoryPurpose1Code>,
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
pub struct CategoryPurpose1Choice {
    #[serde(flatten)]
    pub value: CategoryPurpose1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification42 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party10Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<ContactDetails2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmAndDigest1 {
    #[serde(rename = "DgstAlgo")]
    pub dgst_algo: Algorithm5Code,
    #[validate]
    #[serde(rename = "Dgst")]
    pub dgst: Max140Text,
}
#[derive(
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
pub struct ValidationStatusInformation1 {
    #[serde(rename = "Sts")]
    pub sts: TechnicalValidationStatus1Code,
    #[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
    pub sts_rsn: Option<StatusReason4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlStsRsnInf", default)]
    pub addtl_sts_rsn_inf: Vec<Max105Text>,
}
#[derive(
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
pub struct QualifiedPartyAndXmlSignature1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<IDREF>,
    #[validate]
    #[serde(rename = "Sgntr")]
    pub sgntr: SignatureEnvelope<A>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalLocalInstrument1Code {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct LegalOrganisation1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
}
#[derive(
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
pub struct PaymentTerms6 {
    #[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
    pub due_dt: Option<IsoDate>,
    #[serde(rename = "PmtPrd", skip_serializing_if = "Option::is_none")]
    pub pmt_prd: Option<PaymentPeriod1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Desc", default)]
    pub desc: Vec<Max140Text>,
    #[serde(rename = "PrtlPmtPct", skip_serializing_if = "Option::is_none")]
    pub prtl_pmt_pct: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DrctDbtMndtId", default)]
    pub drct_dbt_mndt_id: Vec<Max35Text>,
    #[serde(rename = "BsisAmt", skip_serializing_if = "Option::is_none")]
    pub bsis_amt: Option<CurrencyAndAmount>,
    #[serde(rename = "DscntAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_amt: Option<CurrencyAndAmount>,
    #[serde(rename = "DscntPctRate", skip_serializing_if = "Option::is_none")]
    pub dscnt_pct_rate: Option<PercentageRate>,
    #[serde(rename = "PnltyAmt", skip_serializing_if = "Option::is_none")]
    pub pnlty_amt: Option<CurrencyAndAmount>,
    #[serde(rename = "PnltyPctRate", skip_serializing_if = "Option::is_none")]
    pub pnlty_pct_rate: Option<PercentageRate>,
}
#[derive(
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
pub struct BusinessMessagePriorityCode {
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
pub struct PaymentTypeInformation19 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<ServiceLevel8Choice>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,
}
#[derive(
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
pub struct SignatureEnvelope<
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
pub struct PaymentMeans1 {
    #[validate]
    #[serde(rename = "PmtTp")]
    pub pmt_tp: PaymentTypeInformation19,
    #[serde(rename = "PmtMtdCd")]
    pub pmt_mtd_cd: PaymentMethod4Code,
    #[validate]
    #[serde(rename = "PyeeCdtrAcct")]
    pub pyee_cdtr_acct: CashAccount16,
    #[validate]
    #[serde(rename = "PyeeFI")]
    pub pyee_fi: BranchAndFinancialInstitutionIdentification4,
    #[serde(rename = "PyerDbtrAcct", skip_serializing_if = "Option::is_none")]
    pub pyer_dbtr_acct: Option<CashAccount16>,
    #[serde(rename = "PyerFI", skip_serializing_if = "Option::is_none")]
    pub pyer_fi: Option<BranchAndFinancialInstitutionIdentification4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalInstrument2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalLocalInstrument1Code>,
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
pub struct LocalInstrument2Choice {
    #[serde(flatten)]
    pub value: LocalInstrument2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BranchAndFinancialInstitutionIdentification5 {
    #[validate]
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification8,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialItemParameters1 {
    #[validate]
    #[serde(rename = "Idr")]
    pub idr: Max35Text,
    #[validate]
    #[serde(rename = "IsseDt")]
    pub isse_dt: IsoDate,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdItm", default)]
    pub rltd_itm: Vec<IDREF>,
    #[serde(rename = "DocPurp", skip_serializing_if = "Option::is_none")]
    pub doc_purp: Option<ExternalDocumentPurpose1Code>,
    #[serde(rename = "LangCd", skip_serializing_if = "Option::is_none")]
    pub lang_cd: Option<LanguageCode>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<IDREF>,
    #[serde(rename = "Rcpt", skip_serializing_if = "Option::is_none")]
    pub rcpt: Option<IDREF>,
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<IDREF>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<IDREF>,
    #[serde(rename = "SellrFinAgt", skip_serializing_if = "Option::is_none")]
    pub sellr_fin_agt: Option<IDREF>,
    #[serde(rename = "BuyrFinAgt", skip_serializing_if = "Option::is_none")]
    pub buyr_fin_agt: Option<IDREF>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GovngCtrct", default)]
    pub govng_ctrct: Vec<IDREF>,
    #[serde(rename = "LglCntxt", skip_serializing_if = "Option::is_none")]
    pub lgl_cntxt: Option<IDREF>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<CurrencyCode>,
    #[serde(rename = "DbtAcct", skip_serializing_if = "Option::is_none")]
    pub dbt_acct: Option<AccountIdentification4Choice>,
    #[serde(rename = "CdtAcct", skip_serializing_if = "Option::is_none")]
    pub cdt_acct: Option<AccountIdentification4Choice>,
    #[serde(rename = "TradMkt", skip_serializing_if = "Option::is_none")]
    pub trad_mkt: Option<TradeMarket1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification8 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party2Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
}
#[derive(
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
pub struct FinancialInstitutionIdentification8 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiIdentifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IDREF {
    #[validate(regex = "IDREF_REGEX")]
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
pub struct TradeParty1 {
    #[validate]
    #[serde(rename = "PtyId")]
    pub pty_id: PartyIdentification45,
    #[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
    pub lgl_org: Option<LegalOrganisation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxPty", default)]
    pub tax_pty: Vec<TaxParty3>,
}
#[derive(
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
pub struct CashAccountType2Enum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashAccountType4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountType2 {
    #[serde(flatten)]
    pub value: CashAccountType2Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ServiceLevel8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalServiceLevel1Code>,
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
pub struct ServiceLevel8Choice {
    #[serde(flatten)]
    pub value: ServiceLevel8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvoicePaymentReconciliationAdviceV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: BusinessLetter1<A>,
    #[validate(length(min = 1,))]
    #[serde(rename = "RcncltnList", default)]
    pub rcncltn_list: Vec<ReconciliationList1<B>>,
    #[serde(rename = "RcncltnCnt", skip_serializing_if = "Option::is_none")]
    pub rcncltn_cnt: Option<Max15NumericText>,
    #[serde(rename = "ItmCnt", skip_serializing_if = "Option::is_none")]
    pub itm_cnt: Option<Max15NumericText>,
    #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AttchdMsg", default)]
    pub attchd_msg: Vec<EncapsulatedBusinessMessage1<C, D>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BinaryFile1 {
    #[serde(rename = "MIMETp", skip_serializing_if = "Option::is_none")]
    pub mime_tp: Option<Max35Text>,
    #[serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none")]
    pub ncodg_tp: Option<Max35Text>,
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<Max35Text>,
    #[serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none")]
    pub incl_binry_objct: Option<Max100KBinary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct BranchData2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
}
#[derive(
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
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
}
#[derive(
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
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[validate]
    #[serde(rename = "MmbId")]
    pub mmb_id: Max35Text,
}
#[derive(
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
