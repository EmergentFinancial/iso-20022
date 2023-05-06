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
    static ref BEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IN[a-zA-Z0-9]{11,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[B-DF-HJ-NP-TV-XZ0-9]{7,7}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref DUNS_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EANGLN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{13,13}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"GR[0-9]{7,7}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsin.002.001.01".to_string()
}

#[derive(
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
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
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
pub struct AccountIdentification3ChoiceEnum {
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "PrtryAcct", skip_serializing_if = "Option::is_none")]
    pub prtry_acct: Option<SimpleIdentificationInformation2>,
}
#[derive(
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
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndianFinancialSystemCodeIdentifier {
    #[validate(regex = "INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX")]
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
pub struct PersonIdentification3Enum {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
    #[serde(rename = "AlnRegnNb", skip_serializing_if = "Option::is_none")]
    pub aln_regn_nb: Option<Max35Text>,
    #[serde(rename = "IdntyCardNb", skip_serializing_if = "Option::is_none")]
    pub idnty_card_nb: Option<Max35Text>,
    #[serde(rename = "MplyrIdNb", skip_serializing_if = "Option::is_none")]
    pub mplyr_id_nb: Option<Max35Text>,
    #[serde(rename = "DrvrsLicNb", skip_serializing_if = "Option::is_none")]
    pub drvrs_lic_nb: Option<Max35Text>,
    #[serde(rename = "CstmrNb", skip_serializing_if = "Option::is_none")]
    pub cstmr_nb: Option<Max35Text>,
    #[serde(rename = "PsptNb", skip_serializing_if = "Option::is_none")]
    pub pspt_nb: Option<Max35Text>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none")]
    pub scl_scty_nb: Option<Max35Text>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<GenericIdentification4>,
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
pub struct SwissSicIdentifier {
    #[validate(regex = "SWISS_SIC_IDENTIFIER_REGEX")]
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
#[derive(
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
pub struct PartyIdentificationAndAccount6 {
    #[validate]
    #[serde(rename = "PtyId")]
    pub pty_id: PartyIdentification25,
    #[serde(rename = "CdtAcct", skip_serializing_if = "Option::is_none")]
    pub cdt_acct: Option<CashAccount7>,
    #[serde(rename = "FincgAcct", skip_serializing_if = "Option::is_none")]
    pub fincg_acct: Option<CashAccount7>,
}
#[derive(
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
pub struct OriginalRequestInformation1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[serde(rename = "FincgRqstr", skip_serializing_if = "Option::is_none")]
    pub fincg_rqstr: Option<PartyIdentificationAndAccount6>,
    #[serde(rename = "IntrmyAgt", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt: Option<FinancialInstitutionIdentification6>,
    #[serde(rename = "FrstAgt", skip_serializing_if = "Option::is_none")]
    pub frst_agt: Option<FinancialInstitutionIdentification6>,
    #[validate]
    #[serde(rename = "VldtnStsInf")]
    pub vldtn_sts_inf: ValidationStatusInformation1,
    #[serde(rename = "CxlStsInf", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_inf: Option<CancellationStatusInformation1>,
}
#[derive(
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
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "ActiveCurrencyAndAmount")]
    pub value: ActiveCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
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
pub struct PartyIdentification25 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
    #[serde(rename = "BEI", skip_serializing_if = "Option::is_none")]
    pub bei: Option<BeiIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification2ChoiceEnum {
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
    #[serde(rename = "OthrClrCdId", skip_serializing_if = "Option::is_none")]
    pub othr_clr_cd_id: Option<Max35Text>,
    #[serde(rename = "INIFSC", skip_serializing_if = "Option::is_none")]
    pub inifsc: Option<IndianFinancialSystemCodeIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "GRHEBIC", skip_serializing_if = "Option::is_none")]
    pub grhebic: Option<HellenicBankIdentificationCodeIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "PLKNR", skip_serializing_if = "Option::is_none")]
    pub plknr: Option<PolishNationalClearingCodeIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification2Choice {
    #[serde(flatten)]
    pub value: ClearingSystemMemberIdentification2ChoiceEnum,
}
#[derive(
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
pub struct SouthAfricanNccIdentifier {
    #[validate(regex = "SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX")]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancellationStatus4Code {
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "REJT")]
    Rejt,
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
pub struct SmallNetworkIdentifier {
    #[validate(regex = "SMALL_NETWORK_IDENTIFIER_REGEX")]
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
pub struct EanglnIdentifier {
    #[validate(regex = "EANGLN_IDENTIFIER_REGEX")]
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
pub struct InvoiceFinancingRequestStatusV01 {
    #[validate]
    #[serde(rename = "StsId")]
    pub sts_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "OrgnlReqInfAndSts")]
    pub orgnl_req_inf_and_sts: OriginalRequestInformation1,
    #[serde(rename = "FincgInfAndSts", skip_serializing_if = "Option::is_none")]
    pub fincg_inf_and_sts: Option<FinancingInformationAndStatus1>,
}
#[derive(
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
pub struct CashAccountType2Enum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashAccountType4Code>,
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
pub struct UkDomesticSortCodeIdentifier {
    #[validate(regex = "UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct CancellationStatusInformation1 {
    #[serde(rename = "Sts")]
    pub sts: CancellationStatus4Code,
    #[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
    pub sts_rsn: Option<StatusReason4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlStsRsnInf", default)]
    pub addtl_sts_rsn_inf: Vec<Max105Text>,
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
pub struct SwissBcIdentifier {
    #[validate(regex = "SWISS_BC_IDENTIFIER_REGEX")]
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "InvcFincgReqSts")]
    pub invc_fincg_req_sts: InvoiceFinancingRequestStatusV01,
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
pub struct PolishNationalClearingCodeIdentifier {
    #[validate(regex = "POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX")]
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
pub struct FinancialInstitutionIdentification6 {
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2Choice>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
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
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct HellenicBankIdentificationCodeIdentifier {
    #[validate(regex = "HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX")]
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
pub struct Party2ChoiceEnum {
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification3>,
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification2>,
}
#[derive(
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
#[derive(
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
