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
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.086.001.04".to_string()
}

#[derive(
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
pub struct ExternalBillingBalanceType1Code {
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
pub struct TaxReason1 {
    #[validate]
    #[serde(rename = "Cd")]
    pub cd: Max10Text,
    #[validate]
    #[serde(rename = "Expltn")]
    pub expltn: Max105Text,
}
#[derive(
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
pub struct BillingServiceCommonIdentification1 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max6Text,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max8Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingServicesTax1 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max40Text>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: DecimalNumber,
    #[validate]
    #[serde(rename = "HstAmt")]
    pub hst_amt: AmountAndDirection34,
    #[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
    pub pricg_amt: Option<AmountAndDirection34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalBillingCompensationType1Code {
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
pub struct BillingTaxIdentification2 {
    #[serde(rename = "VATRegnNb", skip_serializing_if = "Option::is_none")]
    pub vat_regn_nb: Option<Max35Text>,
    #[serde(rename = "TaxRegnNb", skip_serializing_if = "Option::is_none")]
    pub tax_regn_nb: Option<Max35Text>,
    #[serde(rename = "TaxCtct", skip_serializing_if = "Option::is_none")]
    pub tax_ctct: Option<Contact4>,
}
#[derive(
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
pub struct ReportHeader6 {
    #[validate]
    #[serde(rename = "RptId")]
    pub rpt_id: Max35Text,
    #[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
    pub msg_pgntn: Option<Pagination1>,
}
#[derive(
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
pub struct Party43ChoiceEnum {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification29>,
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    pub fi_id: Option<FinancialInstitutionIdentification19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party43Choice {
    #[serde(flatten)]
    pub value: Party43ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ServiceTaxDesignation1 {
    #[serde(rename = "Cd")]
    pub cd: ServiceTaxDesignation1Code,
    #[serde(rename = "Rgn", skip_serializing_if = "Option::is_none")]
    pub rgn: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRsn", default)]
    pub tax_rsn: Vec<TaxReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingService2 {
    #[validate]
    #[serde(rename = "SvcDtl")]
    pub svc_dtl: BillingServiceParameters3,
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<BillingPrice1>,
    #[serde(rename = "PmtMtd")]
    pub pmt_mtd: ServicePaymentMethod1Code,
    #[validate]
    #[serde(rename = "OrgnlChrgPric")]
    pub orgnl_chrg_pric: AmountAndDirection34,
    #[serde(rename = "OrgnlChrgSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_chrg_sttlm_amt: Option<AmountAndDirection34>,
    #[serde(rename = "BalReqrdAcctAmt", skip_serializing_if = "Option::is_none")]
    pub bal_reqrd_acct_amt: Option<AmountAndDirection34>,
    #[validate]
    #[serde(rename = "TaxDsgnt")]
    pub tax_dsgnt: ServiceTaxDesignation1,
    #[serde(rename = "TaxClctn", skip_serializing_if = "Option::is_none")]
    pub tax_clctn: Option<BillingMethod1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingMethod1 {
    #[validate]
    #[serde(rename = "SvcChrgHstAmt")]
    pub svc_chrg_hst_amt: AmountAndDirection34,
    #[validate]
    #[serde(rename = "SvcTax")]
    pub svc_tax: BillingServicesAmount1,
    #[validate]
    #[serde(rename = "TtlChrg")]
    pub ttl_chrg: BillingServicesAmount2,
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "TaxId", default)]
    pub tax_id: Vec<BillingServicesTax1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalBankTransactionFamily1Code {
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
pub struct BankTransactionCodeStructure4 {
    #[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
    pub domn: Option<BankTransactionCodeStructure5>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingMethod4 {
    #[validate(length(min = 1,))]
    #[serde(rename = "SvcDtl", default)]
    pub svc_dtl: Vec<BillingServiceParameters2>,
    #[validate]
    #[serde(rename = "TaxClctn")]
    pub tax_clctn: TaxCalculation1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingTaxCalculationMethod1Code {
    #[serde(rename = "NTAX")]
    Ntax,
    #[serde(rename = "MTDA")]
    Mtda,
    #[serde(rename = "MTDB")]
    Mtdb,
    #[serde(rename = "MTDC")]
    Mtdc,
    #[serde(rename = "MTDD")]
    Mtdd,
    #[serde(rename = "UDFD")]
    Udfd,
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
pub struct FinancialInstitutionIdentification19 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct AddressType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
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
pub struct AddressType3Choice {
    #[serde(flatten)]
    pub value: AddressType3ChoiceEnum,
}
#[derive(
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
pub enum PreferredContactMethod1Code {
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "CELL")]
    Cell,
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
pub struct BillingServiceAdjustment1 {
    #[serde(rename = "Tp")]
    pub tp: ServiceAdjustmentType1Code,
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max140Text,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: AmountAndDirection34,
    #[serde(rename = "BalReqrdAmt", skip_serializing_if = "Option::is_none")]
    pub bal_reqrd_amt: Option<AmountAndDirection34>,
    #[serde(rename = "ErrDt", skip_serializing_if = "Option::is_none")]
    pub err_dt: Option<IsoDate>,
    #[serde(rename = "AdjstmntId", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_id: Option<Max35Text>,
    #[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
    pub sub_svc: Option<BillingSubServiceIdentification1>,
    #[serde(rename = "PricChng", skip_serializing_if = "Option::is_none")]
    pub pric_chng: Option<AmountAndDirection34>,
    #[serde(rename = "OrgnlPric", skip_serializing_if = "Option::is_none")]
    pub orgnl_pric: Option<AmountAndDirection34>,
    #[serde(rename = "NewPric", skip_serializing_if = "Option::is_none")]
    pub new_pric: Option<AmountAndDirection34>,
    #[serde(rename = "VolChng", skip_serializing_if = "Option::is_none")]
    pub vol_chng: Option<DecimalNumber>,
    #[serde(rename = "OrgnlVol", skip_serializing_if = "Option::is_none")]
    pub orgnl_vol: Option<DecimalNumber>,
    #[serde(rename = "NewVol", skip_serializing_if = "Option::is_none")]
    pub new_vol: Option<DecimalNumber>,
    #[serde(rename = "OrgnlChrgAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_chrg_amt: Option<AmountAndDirection34>,
    #[serde(rename = "NewChrgAmt", skip_serializing_if = "Option::is_none")]
    pub new_chrg_amt: Option<AmountAndDirection34>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingCurrencyType1Code {
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "STLM")]
    Stlm,
    #[serde(rename = "PRCG")]
    Prcg,
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
pub struct BillingSubServiceQualifier1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BillingSubServiceQualifier1Code>,
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
pub struct BillingSubServiceQualifier1Choice {
    #[serde(flatten)]
    pub value: BillingSubServiceQualifier1ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification138 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "LglNm", skip_serializing_if = "Option::is_none")]
    pub lgl_nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Id")]
    pub id: Party43Choice,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlusOrMinusIndicator {
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
pub enum ServiceAdjustmentType1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "NCMP")]
    Ncmp,
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
pub struct BalanceAdjustment1 {
    #[serde(rename = "Tp")]
    pub tp: BalanceAdjustmentType1Code,
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max105Text,
    #[validate]
    #[serde(rename = "BalAmt")]
    pub bal_amt: AmountAndDirection34,
    #[serde(rename = "AvrgAmt", skip_serializing_if = "Option::is_none")]
    pub avrg_amt: Option<AmountAndDirection34>,
    #[serde(rename = "ErrDt", skip_serializing_if = "Option::is_none")]
    pub err_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "PstngDt")]
    pub pstng_dt: IsoDate,
    #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
    pub days: Option<DecimalNumber>,
    #[serde(rename = "EarngsAdjstmntAmt", skip_serializing_if = "Option::is_none")]
    pub earngs_adjstmnt_amt: Option<AmountAndDirection34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
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
pub struct ClearingSystemIdentification2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalClearingSystemIdentification1Code>,
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
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max2048Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingRate1 {
    #[serde(rename = "Id")]
    pub id: BillingRateIdentification1Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
    #[serde(rename = "DaysInPrd", skip_serializing_if = "Option::is_none")]
    pub days_in_prd: Option<Number>,
    #[serde(rename = "DaysInYr", skip_serializing_if = "Option::is_none")]
    pub days_in_yr: Option<Number>,
}
#[derive(
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
pub struct BillingRateIdentification1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalBillingRateIdentification1Code>,
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
pub struct BillingRateIdentification1Choice {
    #[serde(flatten)]
    pub value: BillingRateIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress24 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<Max35Text>,
    #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
    pub flr: Option<Max70Text>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<Max16Text>,
    #[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
    pub room: Option<Max70Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<Max35Text>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[validate(length(min = 0, max = 7,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingSubServiceQualifier1Code {
    #[serde(rename = "LBOX")]
    Lbox,
    #[serde(rename = "STOR")]
    Stor,
    #[serde(rename = "BILA")]
    Bila,
    #[serde(rename = "SEQN")]
    Seqn,
    #[serde(rename = "MACT")]
    Mact,
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
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountLevel1Code {
    #[serde(rename = "INTM")]
    Intm,
    #[serde(rename = "SMRY")]
    Smry,
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
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[validate]
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}
#[derive(
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
pub struct BillingStatement4 {
    #[validate]
    #[serde(rename = "StmtId")]
    pub stmt_id: Max35Text,
    #[validate]
    #[serde(rename = "FrToDt")]
    pub fr_to_dt: DatePeriod1,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[serde(rename = "Sts")]
    pub sts: BillingStatementStatus1Code,
    #[validate]
    #[serde(rename = "AcctChrtcs")]
    pub acct_chrtcs: CashAccountCharacteristics4,
    #[validate(length(min = 0,))]
    #[serde(rename = "RateData", default)]
    pub rate_data: Vec<BillingRate1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CcyXchg", default)]
    pub ccy_xchg: Vec<CurrencyExchange6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Bal", default)]
    pub bal: Vec<BillingBalance1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Compstn", default)]
    pub compstn: Vec<BillingCompensation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Svc", default)]
    pub svc: Vec<BillingService2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRgn", default)]
    pub tax_rgn: Vec<BillingTaxRegion2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BalAdjstmnt", default)]
    pub bal_adjstmnt: Vec<BalanceAdjustment1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvcAdjstmnt", default)]
    pub svc_adjstmnt: Vec<BillingServiceAdjustment1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalBankTransactionDomain1Code {
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
pub struct Max128Text {
    #[validate(length(min = 1, max = 128,))]
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
    #[serde(rename = "BkSvcsBllgStmt")]
    pub bk_svcs_bllg_stmt: BankServicesBillingStatementV04,
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
pub struct BillingMethod3 {
    #[validate]
    #[serde(rename = "SvcTaxPricAmt")]
    pub svc_tax_pric_amt: AmountAndDirection34,
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "TaxId", default)]
    pub tax_id: Vec<BillingServicesTax2>,
}
#[derive(
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
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingCurrencyType2Code {
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "STLM")]
    Stlm,
    #[serde(rename = "PRCG")]
    Prcg,
    #[serde(rename = "HOST")]
    Host,
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
pub struct ExternalBankTransactionSubFamily1Code {
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
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ServicePaymentMethod1Code {
    #[serde(rename = "BCMP")]
    Bcmp,
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PVCH")]
    Pvch,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "WVED")]
    Wved,
    #[serde(rename = "FREE")]
    Free,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingChargeMethod1Code {
    #[serde(rename = "UPRC")]
    Uprc,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "BCHG")]
    Bchg,
    #[serde(rename = "DPRC")]
    Dprc,
    #[serde(rename = "FCHG")]
    Fchg,
    #[serde(rename = "LPRC")]
    Lprc,
    #[serde(rename = "MCHG")]
    Mchg,
    #[serde(rename = "MXRD")]
    Mxrd,
    #[serde(rename = "TIR1")]
    Tir1,
    #[serde(rename = "TIR2")]
    Tir2,
    #[serde(rename = "TIR3")]
    Tir3,
    #[serde(rename = "TIR4")]
    Tir4,
    #[serde(rename = "TIR5")]
    Tir5,
    #[serde(rename = "TIR6")]
    Tir6,
    #[serde(rename = "TIR7")]
    Tir7,
    #[serde(rename = "TIR8")]
    Tir8,
    #[serde(rename = "TIR9")]
    Tir9,
    #[serde(rename = "TPRC")]
    Tprc,
    #[serde(rename = "ZPRC")]
    Zprc,
    #[serde(rename = "BBSE")]
    Bbse,
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
pub struct BillingServicesTax3 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max40Text>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: DecimalNumber,
    #[validate]
    #[serde(rename = "TtlTaxAmt")]
    pub ttl_tax_amt: AmountAndDirection34,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BillingStatementStatus1Code {
    #[serde(rename = "ORGN")]
    Orgn,
    #[serde(rename = "RPLC")]
    Rplc,
    #[serde(rename = "TEST")]
    Test,
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
pub struct BranchData3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
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
pub struct BillingServiceParameters2 {
    #[validate]
    #[serde(rename = "BkSvc")]
    pub bk_svc: BillingServiceIdentification2,
    #[serde(rename = "Vol", skip_serializing_if = "Option::is_none")]
    pub vol: Option<DecimalNumber>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<AmountAndDirection34>,
    #[validate]
    #[serde(rename = "SvcChrgAmt")]
    pub svc_chrg_amt: AmountAndDirection34,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingServiceIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
    pub sub_svc: Option<BillingSubServiceIdentification1>,
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max70Text,
    #[serde(rename = "CmonCd", skip_serializing_if = "Option::is_none")]
    pub cmon_cd: Option<BillingServiceCommonIdentification1>,
    #[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
    pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
    #[serde(rename = "SvcTp", skip_serializing_if = "Option::is_none")]
    pub svc_tp: Option<Max12Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BalanceAdjustmentType1Code {
    #[serde(rename = "LDGR")]
    Ldgr,
    #[serde(rename = "FLOT")]
    Flot,
    #[serde(rename = "CLLD")]
    Clld,
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
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct AmountAndDirection34 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[validate]
    #[serde(rename = "Sgn")]
    pub sgn: PlusOrMinusIndicator,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountLevel2Code {
    #[serde(rename = "INTM")]
    Intm,
    #[serde(rename = "SMRY")]
    Smry,
    #[serde(rename = "DETL")]
    Detl,
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
pub struct BankTransactionCodeStructure6 {
    #[serde(rename = "Cd")]
    pub cd: ExternalBankTransactionFamily1Code,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BankTransactionCodeStructure5 {
    #[serde(rename = "Cd")]
    pub cd: ExternalBankTransactionDomain1Code,
    #[serde(rename = "Fmly")]
    pub fmly: BankTransactionCodeStructure6,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BankServicesBillingStatementV04 {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: ReportHeader6,
    #[validate(length(min = 1,))]
    #[serde(rename = "BllgStmtGrp", default)]
    pub bllg_stmt_grp: Vec<StatementGroup4>,
}
#[derive(
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
pub struct BillingServicesAmount3 {
    #[validate]
    #[serde(rename = "SrcAmt")]
    pub src_amt: AmountAndDirection34,
    #[validate]
    #[serde(rename = "HstAmt")]
    pub hst_amt: AmountAndDirection34,
}
#[derive(
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
pub struct ParentCashAccount4 {
    #[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
    pub lvl: Option<AccountLevel1Code>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: CashAccount40,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProprietaryBankTransactionCodeStructure1 {
    #[validate]
    #[serde(rename = "Cd")]
    pub cd: Max35Text,
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
pub struct BillingServiceIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
    pub sub_svc: Option<BillingSubServiceIdentification1>,
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max70Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResidenceLocation1ChoiceEnum {
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Max35Text>,
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
pub struct ResidenceLocation1Choice {
    #[serde(flatten)]
    pub value: ResidenceLocation1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingBalanceType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalBillingBalanceType1Code>,
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
pub struct BillingBalanceType1Choice {
    #[serde(flatten)]
    pub value: BillingBalanceType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccount40 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountIdentification4Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProxyAccountType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalProxyAccountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProxyAccountType1Choice {
    #[serde(flatten)]
    pub value: ProxyAccountType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingMethod2 {
    #[validate]
    #[serde(rename = "SvcChrgHstAmt")]
    pub svc_chrg_hst_amt: AmountAndDirection34,
    #[validate]
    #[serde(rename = "SvcTax")]
    pub svc_tax: BillingServicesAmount1,
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "TaxId", default)]
    pub tax_id: Vec<BillingServicesTax1>,
}
#[derive(
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
pub struct OrganisationIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalOrganisationIdentification1Code>,
}
#[derive(
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
pub struct BillingTaxRegion2 {
    #[validate]
    #[serde(rename = "RgnNb")]
    pub rgn_nb: Max40Text,
    #[validate]
    #[serde(rename = "RgnNm")]
    pub rgn_nm: Max40Text,
    #[validate]
    #[serde(rename = "CstmrTaxId")]
    pub cstmr_tax_id: Max40Text,
    #[serde(rename = "PtDt", skip_serializing_if = "Option::is_none")]
    pub pt_dt: Option<IsoDate>,
    #[serde(rename = "SndgFI", skip_serializing_if = "Option::is_none")]
    pub sndg_fi: Option<BillingTaxIdentification2>,
    #[serde(rename = "InvcNb", skip_serializing_if = "Option::is_none")]
    pub invc_nb: Option<Max40Text>,
    #[serde(rename = "MtdC", skip_serializing_if = "Option::is_none")]
    pub mtd_c: Option<BillingMethod4>,
    #[validate]
    #[serde(rename = "SttlmAmt")]
    pub sttlm_amt: AmountAndDirection34,
    #[validate]
    #[serde(rename = "TaxDueToRgn")]
    pub tax_due_to_rgn: AmountAndDirection34,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Pagination1 {
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
#[derive(
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
pub struct AccountTax1 {
    #[serde(rename = "ClctnMtd")]
    pub clctn_mtd: BillingTaxCalculationMethod1Code,
    #[serde(rename = "Rgn", skip_serializing_if = "Option::is_none")]
    pub rgn: Option<Max40Text>,
    #[serde(rename = "NonResCtry", skip_serializing_if = "Option::is_none")]
    pub non_res_ctry: Option<ResidenceLocation1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatementGroup4 {
    #[validate]
    #[serde(rename = "GrpId")]
    pub grp_id: Max35Text,
    #[validate]
    #[serde(rename = "Sndr")]
    pub sndr: PartyIdentification138,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "SndrIndvCtct", default)]
    pub sndr_indv_ctct: Vec<Contact4>,
    #[validate]
    #[serde(rename = "Rcvr")]
    pub rcvr: PartyIdentification138,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "RcvrIndvCtct", default)]
    pub rcvr_indv_ctct: Vec<Contact4>,
    #[validate(length(min = 1,))]
    #[serde(rename = "BllgStmt", default)]
    pub bllg_stmt: Vec<BillingStatement4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max4Text {
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
pub struct BillingBalance1 {
    #[serde(rename = "Tp")]
    pub tp: BillingBalanceType1Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: AmountAndDirection34,
    #[serde(rename = "CcyTp", skip_serializing_if = "Option::is_none")]
    pub ccy_tp: Option<BillingCurrencyType1Code>,
}
#[derive(
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
pub struct CashAccountCharacteristics4 {
    #[serde(rename = "AcctLvl")]
    pub acct_lvl: AccountLevel2Code,
    #[validate]
    #[serde(rename = "CshAcct")]
    pub csh_acct: CashAccount40,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrntAcct", skip_serializing_if = "Option::is_none")]
    pub prnt_acct: Option<ParentCashAccount4>,
    #[serde(rename = "CompstnMtd")]
    pub compstn_mtd: CompensationMethod1Code,
    #[serde(rename = "DbtAcct", skip_serializing_if = "Option::is_none")]
    pub dbt_acct: Option<AccountIdentification4Choice>,
    #[serde(rename = "DelydDbtDt", skip_serializing_if = "Option::is_none")]
    pub delyd_dbt_dt: Option<IsoDate>,
    #[serde(rename = "SttlmAdvc", skip_serializing_if = "Option::is_none")]
    pub sttlm_advc: Option<Max105Text>,
    #[serde(rename = "AcctBalCcyCd")]
    pub acct_bal_ccy_cd: ActiveOrHistoricCurrencyCode,
    #[serde(rename = "SttlmCcyCd", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "HstCcyCd", skip_serializing_if = "Option::is_none")]
    pub hst_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<AccountTax1>,
    #[validate]
    #[serde(rename = "AcctSvcrCtct")]
    pub acct_svcr_ctct: Contact4,
}
#[derive(
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
pub struct Max40Text {
    #[validate(length(min = 1, max = 40,))]
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
pub struct BillingServicesAmount1 {
    #[validate]
    #[serde(rename = "HstAmt")]
    pub hst_amt: AmountAndDirection34,
    #[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
    pub pricg_amt: Option<AmountAndDirection34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingCompensationType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalBillingCompensationType1Code>,
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
pub struct BillingCompensationType1Choice {
    #[serde(flatten)]
    pub value: BillingCompensationType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingServicesAmount2 {
    #[validate]
    #[serde(rename = "HstAmt")]
    pub hst_amt: AmountAndDirection34,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection34>,
    #[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
    pub pricg_amt: Option<AmountAndDirection34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalBillingRateIdentification1Code {
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
pub struct DatePeriod1 {
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
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
pub struct Max8Text {
    #[validate(length(min = 1, max = 8,))]
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
pub struct Max20Text {
    #[validate(length(min = 1, max = 20,))]
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
pub struct Max12Text {
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
pub struct FinancialIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
}
#[derive(
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
pub struct BillingSubServiceIdentification1 {
    #[serde(rename = "Issr")]
    pub issr: BillingSubServiceQualifier1Choice,
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
pub struct BillingServiceParameters3 {
    #[validate]
    #[serde(rename = "BkSvc")]
    pub bk_svc: BillingServiceIdentification3,
    #[serde(rename = "Vol", skip_serializing_if = "Option::is_none")]
    pub vol: Option<DecimalNumber>,
}
#[derive(
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
pub struct BicfiDec2014Identifier {
    #[validate(regex = "BICFI_DEC_2014_IDENTIFIER_REGEX")]
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
pub struct BillingServicesTax2 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max40Text>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: DecimalNumber,
    #[validate]
    #[serde(rename = "PricgAmt")]
    pub pricg_amt: AmountAndDirection34,
}
#[derive(
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
pub struct BillingPrice1 {
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<AmountAndDirection34>,
    #[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
    pub mtd: Option<BillingChargeMethod1Code>,
    #[serde(rename = "Rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<Max20Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxCalculation1 {
    #[serde(rename = "HstCcy")]
    pub hst_ccy: ActiveOrHistoricCurrencyCode,
    #[validate(length(min = 1,))]
    #[serde(rename = "TaxblSvcChrgConvs", default)]
    pub taxbl_svc_chrg_convs: Vec<BillingServicesAmount3>,
    #[validate]
    #[serde(rename = "TtlTaxblSvcChrgHstAmt")]
    pub ttl_taxbl_svc_chrg_hst_amt: AmountAndDirection34,
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "TaxId", default)]
    pub tax_id: Vec<BillingServicesTax3>,
    #[validate]
    #[serde(rename = "TtlTax")]
    pub ttl_tax: AmountAndDirection34,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalProxyAccountType1Code {
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
pub struct BillingMethod1ChoiceEnum {
    #[serde(rename = "MtdB", skip_serializing_if = "Option::is_none")]
    pub mtd_b: Option<BillingMethod2>,
    #[serde(rename = "MtdA", skip_serializing_if = "Option::is_none")]
    pub mtd_a: Option<BillingMethod1>,
    #[serde(rename = "MtdD", skip_serializing_if = "Option::is_none")]
    pub mtd_d: Option<BillingMethod3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BillingMethod1Choice {
    #[serde(flatten)]
    pub value: BillingMethod1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyExchange6 {
    #[serde(rename = "SrcCcy")]
    pub src_ccy: ActiveOrHistoricCurrencyCode,
    #[serde(rename = "TrgtCcy")]
    pub trgt_ccy: ActiveOrHistoricCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max40Text>,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Cmnts", skip_serializing_if = "Option::is_none")]
    pub cmnts: Option<Max70Text>,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix2Code>,
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
    #[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<Max35Text>,
    #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<Max35Text>,
    #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<Max35Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherContact1>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherContact1 {
    #[validate]
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: Max4Text,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max128Text>,
}
#[derive(
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
pub struct BillingCompensation1 {
    #[serde(rename = "Tp")]
    pub tp: BillingCompensationType1Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: AmountAndDirection34,
    #[serde(rename = "CcyTp", skip_serializing_if = "Option::is_none")]
    pub ccy_tp: Option<BillingCurrencyType2Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CompensationMethod1Code {
    #[serde(rename = "NOCP")]
    Nocp,
    #[serde(rename = "DBTD")]
    Dbtd,
    #[serde(rename = "INVD")]
    Invd,
    #[serde(rename = "DDBT")]
    Ddbt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ServiceTaxDesignation1Code {
    #[serde(rename = "XMPT")]
    Xmpt,
    #[serde(rename = "ZERO")]
    Zero,
    #[serde(rename = "TAXE")]
    Taxe,
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
