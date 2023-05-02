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
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_UNDERSCORE_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1}[a-zA-Z0-9_]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.022.001.03".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentScheduleType2Code {
    #[serde(rename = "CNTR")]
    Cntr,
    #[serde(rename = "ESTM")]
    Estm,
    #[serde(rename = "BOTH")]
    Both,
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
pub struct RegisteredContractAmendment1 {
    #[validate]
    #[serde(rename = "AmdmntDt")]
    pub amdmnt_dt: IsoDate,
    #[validate]
    #[serde(rename = "Doc")]
    pub doc: DocumentIdentification28,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "AmdmntRsn", skip_serializing_if = "Option::is_none")]
    pub amdmnt_rsn: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
}
#[derive(
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
#[derive(
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
pub struct ExternalContractClosureReason1Code {
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
pub struct UnderlyingContract3ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Ln", skip_serializing_if = "Option::is_none")]
    pub ln: Option<LoanContract3<A>>,
    #[serde(rename = "Trad", skip_serializing_if = "Option::is_none")]
    pub trad: Option<TradeContract3<B>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnderlyingContract3Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: UnderlyingContract3ChoiceEnum<A, B>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate2ChoiceEnum {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<PercentageRate>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<FloatingInterestRate4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate2Choice {
    #[serde(flatten)]
    pub value: InterestRate2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
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
pub struct PersonIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPersonIdentification1Code>,
}
#[derive(
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
pub struct ExchangeRate1 {
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<ExchangeRateType1Code>,
    #[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCertificateRecord2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CertRcrdId")]
    pub cert_rcrd_id: Max35Text,
    #[serde(rename = "DocSubmitgPrcdr", skip_serializing_if = "Option::is_none")]
    pub doc_submitg_prcdr: Option<Exact1NumericText>,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: TransactionCertificate5,
    #[serde(rename = "Ctrct", skip_serializing_if = "Option::is_none")]
    pub ctrct: Option<TransactionCertificateContract2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Attchmnt", default)]
    pub attchmnt: Vec<DocumentGeneralInformation5<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification35 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "DtOfIsse", skip_serializing_if = "Option::is_none")]
    pub dt_of_isse: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashCollateral5 {
    #[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
    pub coll_id: Option<Max35Text>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<AccountIdentification4Choice>,
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "DpstAmt", skip_serializing_if = "Option::is_none")]
    pub dpst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DpstTp", skip_serializing_if = "Option::is_none")]
    pub dpst_tp: Option<DepositType1Code>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[validate]
    #[serde(rename = "CollVal")]
    pub coll_val: ActiveCurrencyAndAmount,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
}
#[derive(
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
pub struct TransactionCertificateContract2 {
    #[serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none")]
    pub ctrct_ref: Option<ContractRegistrationReference2Choice>,
    #[serde(rename = "TxAmtInCtrctCcy", skip_serializing_if = "Option::is_none")]
    pub tx_amt_in_ctrct_ccy: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpctdShipmntDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "XpctdAdvncPmtRtrDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_advnc_pmt_rtr_dt: Option<IsoDate>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName2Code {
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "BBSW")]
    Bbsw,
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
pub struct DocumentEntryAmendment1 {
    #[validate]
    #[serde(rename = "CrrctgNtryNb")]
    pub crrctg_ntry_nb: Number,
    #[validate]
    #[serde(rename = "OrgnlDoc")]
    pub orgnl_doc: DocumentIdentification28,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentSchedule1 {
    #[serde(rename = "PmtSchdlId", skip_serializing_if = "Option::is_none")]
    pub pmt_schdl_id: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_dt: Option<IsoDate>,
    #[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
    pub due_dt: Option<IsoDate>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractBalanceType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalContractBalanceType1Code>,
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
pub struct ContractBalanceType1Choice {
    #[serde(flatten)]
    pub value: ContractBalanceType1ChoiceEnum,
}
#[derive(
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
pub struct GenericOrganisationIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExchangeRateType1Code {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "AGRD")]
    Agrd,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact1NumericText {
    #[validate(regex = "EXACT_1_NUMERIC_TEXT_REGEX")]
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
pub struct ContractRegistrationStatementV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    H: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    I: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    J: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: CurrencyControlHeader6,
    #[validate(length(min = 1,))]
    #[serde(rename = "Stmt", default)]
    pub stmt: Vec<ContractRegistrationStatement3<A, B, C, D, E, F, G, H, I>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<J>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractBalance1 {
    #[serde(rename = "Tp")]
    pub tp: ContractBalanceType1Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebit3Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegisteredContract12<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "OrgnlCtrctRegnReq", skip_serializing_if = "Option::is_none")]
    pub orgnl_ctrct_regn_req: Option<Max35Text>,
    #[validate]
    #[serde(rename = "IssrFI")]
    pub issr_fi: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "Ctrct")]
    pub ctrct: UnderlyingContract3Choice<A, B>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrctBal", default)]
    pub ctrct_bal: Vec<ContractBalance1>,
    #[serde(rename = "PmtSchdlTp", skip_serializing_if = "Option::is_none")]
    pub pmt_schdl_tp: Option<PaymentScheduleType2Choice>,
    #[validate]
    #[serde(rename = "RegdCtrctId")]
    pub regd_ctrct_id: DocumentIdentification29,
    #[serde(rename = "PrvsRegdCtrctId", skip_serializing_if = "Option::is_none")]
    pub prvs_regd_ctrct_id: Option<DocumentIdentification22>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RegdCtrctJrnl", default)]
    pub regd_ctrct_jrnl: Vec<RegisteredContractJournal2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Amdmnt", default)]
    pub amdmnt: Vec<RegisteredContractAmendment1>,
    #[validate]
    #[serde(rename = "Submissn")]
    pub submissn: RegisteredContractCommunication1,
    #[validate]
    #[serde(rename = "Dlvry")]
    pub dlvry: RegisteredContractCommunication1,
    #[serde(rename = "LnPrncplAmt", skip_serializing_if = "Option::is_none")]
    pub ln_prncpl_amt: Option<ActiveCurrencyAndAmount>,
    #[validate]
    #[serde(rename = "EstmtdDtInd")]
    pub estmtd_dt_ind: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "IntrCpnyLn")]
    pub intr_cpny_ln: TrueFalseIndicator,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentSchedule2ChoiceEnum {
    #[serde(rename = "ShipmntDtRg", skip_serializing_if = "Option::is_none")]
    pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
    #[serde(rename = "ShipmntSubSchdl", skip_serializing_if = "Option::is_none")]
    pub shipmnt_sub_schdl: Option<ShipmentDateRange2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentSchedule2Choice {
    #[serde(flatten)]
    pub value: ShipmentSchedule2ChoiceEnum,
}
#[derive(
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
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateBasis1Code {
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
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
pub struct PartyIdentification135 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice>,
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
pub struct FloatingInterestRate4 {
    #[serde(rename = "RefRate")]
    pub ref_rate: BenchmarkCurveName4Choice,
    #[validate]
    #[serde(rename = "Term")]
    pub term: InterestRateContractTerm1,
    #[validate]
    #[serde(rename = "BsisPtSprd")]
    pub bsis_pt_sprd: Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractCollateral1 {
    #[validate]
    #[serde(rename = "TtlAmt")]
    pub ttl_amt: ActiveCurrencyAndAmount,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollDesc", default)]
    pub coll_desc: Vec<CashCollateral5>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificateReference2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: CertificateIdentification1,
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
pub struct DatePeriod3 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<IsoDate>,
}
#[derive(
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
pub struct TrueFalseIndicator {
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
pub struct Party38ChoiceEnum {
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification13>,
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party38Choice {
    #[serde(flatten)]
    pub value: Party38ChoiceEnum,
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
pub struct SyndicatedLoan2 {
    #[validate]
    #[serde(rename = "Brrwr")]
    pub brrwr: TradeParty5,
    #[serde(rename = "Lndr", skip_serializing_if = "Option::is_none")]
    pub lndr: Option<TradeParty5>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Shr", skip_serializing_if = "Option::is_none")]
    pub shr: Option<PercentageRate>,
    #[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
    pub xchg_rate_inf: Option<ExchangeRate1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalContractBalanceType1Code {
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
pub struct TradeParty5 {
    #[validate]
    #[serde(rename = "PtyId")]
    pub pty_id: PartyIdentification135,
    #[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
    pub lgl_org: Option<LegalOrganisation2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxPty", default)]
    pub tax_pty: Vec<TaxParty4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LoanContract3<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CtrctDocId")]
    pub ctrct_doc_id: DocumentIdentification22,
    #[serde(rename = "LnTpId", skip_serializing_if = "Option::is_none")]
    pub ln_tp_id: Option<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Buyr", default)]
    pub buyr: Vec<TradeParty5>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Sellr", default)]
    pub sellr: Vec<TradeParty5>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
    pub prlngtn_flg: Option<TrueFalseIndicator>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "SpclConds", skip_serializing_if = "Option::is_none")]
    pub spcl_conds: Option<SpecialCondition1>,
    #[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
    pub drtn_cd: Option<Exact1NumericText>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<InterestRate2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Trch", default)]
    pub trch: Vec<LoanContractTranche1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtSchdl", default)]
    pub pmt_schdl: Vec<PaymentSchedule1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstSchdl", default)]
    pub intrst_schdl: Vec<InterestPaymentSchedule1>,
    #[validate]
    #[serde(rename = "IntraCpnyLn")]
    pub intra_cpny_ln: TrueFalseIndicator,
    #[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
    pub coll: Option<ContractCollateral1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SndctdLn", default)]
    pub sndctd_ln: Vec<SyndicatedLoan2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Attchmnt", default)]
    pub attchmnt: Vec<DocumentGeneralInformation5<A>>,
}
#[derive(
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
pub struct Exact4AlphaNumericUnderscoreText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_UNDERSCORE_TEXT_REGEX")]
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
pub struct Max25Text {
    #[validate(length(min = 1, max = 25,))]
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
pub struct LoanContractTranche1 {
    #[validate]
    #[serde(rename = "TrchNb")]
    pub trch_nb: Number,
    #[validate]
    #[serde(rename = "XpctdDt")]
    pub xpctd_dt: IsoDate,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
    pub due_dt: Option<IsoDate>,
    #[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
    pub drtn_cd: Option<Exact1NumericText>,
    #[serde(rename = "LastTrchInd", skip_serializing_if = "Option::is_none")]
    pub last_trch_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification22 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "DtOfIsse", skip_serializing_if = "Option::is_none")]
    pub dt_of_isse: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestPaymentSchedule1 {
    #[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
    pub intrst_schdl_id: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_dt: Option<IsoDate>,
    #[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
    pub due_dt: Option<IsoDate>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max1025Text>,
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
pub struct ShipmentDateRange2 {
    #[validate]
    #[serde(rename = "SubQtyVal")]
    pub sub_qty_val: DecimalNumber,
    #[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub earlst_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub latst_shipmnt_dt: Option<IsoDate>,
}
#[derive(
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
pub enum DepositType1Code {
    #[serde(rename = "FITE")]
    Fite,
    #[serde(rename = "CALL")]
    Call,
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
pub struct LegalOrganisation2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "EstblishmtDt", skip_serializing_if = "Option::is_none")]
    pub estblishmt_dt: Option<IsoDate>,
    #[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
    pub regn_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentDateRange1 {
    #[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub earlst_shipmnt_dt: Option<IsoDate>,
    #[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
    pub latst_shipmnt_dt: Option<IsoDate>,
}
#[derive(
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
pub struct ContractRegistrationStatement3<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    H: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    I: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "StmtId", skip_serializing_if = "Option::is_none")]
    pub stmt_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "RptgPty")]
    pub rptg_pty: TradeParty5,
    #[validate]
    #[serde(rename = "RegnAgt")]
    pub regn_agt: BranchAndFinancialInstitutionIdentification6,
    #[validate]
    #[serde(rename = "RptgPrd")]
    pub rptg_prd: ReportingPeriod4,
    #[validate]
    #[serde(rename = "RegdCtrct")]
    pub regd_ctrct: RegisteredContract12<A, B>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxJrnl", default)]
    pub tx_jrnl: Vec<TransactionCertificate4<C, D>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpprtgDocJrnl", default)]
    pub spprtg_doc_jrnl: Vec<SupportingDocument3<E, F>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSpprtgDocJrnl", default)]
    pub addtl_spprtg_doc_jrnl: Vec<SupportingDocument3<G, H>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RgltryRuleVldtn", default)]
    pub rgltry_rule_vldtn: Vec<GenericValidationRuleIdentification1>,
    #[validate]
    #[serde(rename = "TtlCtrctTrnvrSum")]
    pub ttl_ctrct_trnvr_sum: ActiveCurrencyAndAmount,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<I>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentGeneralInformation5<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DocTp")]
    pub doc_tp: ExternalDocumentType1Code,
    #[validate]
    #[serde(rename = "DocNb")]
    pub doc_nb: Max35Text,
    #[serde(rename = "DocNm", skip_serializing_if = "Option::is_none")]
    pub doc_nm: Option<Max140Text>,
    #[serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none")]
    pub sndr_rcvr_seq_id: Option<Max140Text>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoDate>,
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<Max256Text>,
    #[serde(rename = "LkFileHash", skip_serializing_if = "Option::is_none")]
    pub lk_file_hash: Option<SignatureEnvelopeReference<A>>,
    #[validate]
    #[serde(rename = "AttchdBinryFile")]
    pub attchd_binry_file: BinaryFile1,
}
#[derive(
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
pub struct Exact5NumericText {
    #[validate(regex = "EXACT_5_NUMERIC_TEXT_REGEX")]
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
pub struct CertificateIdentification1 {
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<Max35Text>,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<Max35Text>,
    #[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<Max35Text>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<Max35Text>,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<Max35Text>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryReference1>,
}
#[derive(
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
pub struct IsoTime {
    #[serde(rename = "$value")]
    pub value: ::chrono::naive::NaiveTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupportingDocumentEntry2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "NtryId")]
    pub ntry_id: Max35Text,
    #[validate]
    #[serde(rename = "OrgnlDoc")]
    pub orgnl_doc: DocumentIdentification22,
    #[validate]
    #[serde(rename = "DocTp")]
    pub doc_tp: Exact4AlphaNumericUnderscoreText,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlAmtAftrShipmnt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt_aftr_shipmnt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlAmtInCtrctCcy", skip_serializing_if = "Option::is_none")]
    pub ttl_amt_in_ctrct_ccy: Option<ActiveCurrencyAndAmount>,
    #[serde(
        rename = "TtlAmtAftrShipmntInCtrctCcy",
        skip_serializing_if = "Option::is_none"
    )]
    pub ttl_amt_aftr_shipmnt_in_ctrct_ccy: Option<ActiveCurrencyAndAmount>,
    #[validate]
    #[serde(rename = "ShipmntAttrbts")]
    pub shipmnt_attrbts: ShipmentAttribute2,
    #[serde(rename = "NtryAmdmntId", skip_serializing_if = "Option::is_none")]
    pub ntry_amdmnt_id: Option<DocumentEntryAmendment1>,
    #[serde(rename = "MtrtyData", skip_serializing_if = "Option::is_none")]
    pub mtrty_data: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Attchmnt", default)]
    pub attchmnt: Vec<DocumentGeneralInformation5<A>>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    H: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    I: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    J: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CtrctRegnStmt")]
    pub ctrct_regn_stmt: ContractRegistrationStatementV03<A, B, C, D, E, F, G, H, I, J>,
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
pub struct DocumentAmendment1 {
    #[validate]
    #[serde(rename = "CrrctnId")]
    pub crrctn_id: Number,
    #[serde(rename = "OrgnlDocId", skip_serializing_if = "Option::is_none")]
    pub orgnl_doc_id: Option<Max35Text>,
}
#[derive(
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
pub struct BenchmarkCurveName4ChoiceEnum {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName4Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName4ChoiceEnum,
}
#[derive(
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
pub struct SpecialCondition1 {
    #[validate]
    #[serde(rename = "IncmgAmt")]
    pub incmg_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OutgngAmt")]
    pub outgng_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "IncmgAmtToOthrAcct", skip_serializing_if = "Option::is_none")]
    pub incmg_amt_to_othr_acct: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PmtFrOthrAcct", skip_serializing_if = "Option::is_none")]
    pub pmt_fr_othr_acct: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupportingDocument3<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "SpprtgDocId")]
    pub spprtg_doc_id: Max35Text,
    #[serde(rename = "OrgnlReqId", skip_serializing_if = "Option::is_none")]
    pub orgnl_req_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Cert")]
    pub cert: DocumentIdentification28,
    #[validate]
    #[serde(rename = "AcctOwnr")]
    pub acct_ownr: PartyIdentification135,
    #[validate]
    #[serde(rename = "AcctSvcr")]
    pub acct_svcr: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "Amdmnt", skip_serializing_if = "Option::is_none")]
    pub amdmnt: Option<DocumentAmendment1>,
    #[serde(rename = "CtrctRef")]
    pub ctrct_ref: ContractRegistrationReference2Choice,
    #[validate(length(min = 1,))]
    #[serde(rename = "Ntry", default)]
    pub ntry: Vec<SupportingDocumentEntry2<A>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebit3Code {
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
pub struct TransactionCertificate5 {
    #[validate]
    #[serde(rename = "RfrdDoc")]
    pub rfrd_doc: CertificateReference2,
    #[validate]
    #[serde(rename = "TxDt")]
    pub tx_dt: IsoDate,
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<Exact1NumericText>,
    #[validate]
    #[serde(rename = "LclInstrm")]
    pub lcl_instrm: Exact5NumericText,
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
pub struct ValidationRuleSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalValidationRuleIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValidationRuleSchemeName1Choice {
    #[serde(flatten)]
    pub value: ValidationRuleSchemeName1ChoiceEnum,
}
#[derive(
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
pub struct DocumentIdentification29 {
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
pub struct DocumentIdentification28 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
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
pub struct CurrencyControlHeader6 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "NbOfItms")]
    pub nb_of_itms: Max15NumericText,
    #[validate]
    #[serde(rename = "RcvgPty")]
    pub rcvg_pty: PartyIdentification135,
    #[validate]
    #[serde(rename = "RegnAgt")]
    pub regn_agt: BranchAndFinancialInstitutionIdentification6,
}
#[derive(
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
pub enum QueryType3Code {
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "CHNG")]
    Chng,
    #[serde(rename = "MODF")]
    Modf,
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
pub struct ContractRegistrationReference2ChoiceEnum {
    #[serde(rename = "RegdCtrctId", skip_serializing_if = "Option::is_none")]
    pub regd_ctrct_id: Option<Max35Text>,
    #[serde(rename = "Ctrct", skip_serializing_if = "Option::is_none")]
    pub ctrct: Option<DocumentIdentification35>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractRegistrationReference2Choice {
    #[serde(flatten)]
    pub value: ContractRegistrationReference2ChoiceEnum,
}
#[derive(
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
pub struct TimePeriod2 {
    #[validate]
    #[serde(rename = "FrTm")]
    pub fr_tm: IsoTime,
    #[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
    pub to_tm: Option<IsoTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCertificate4<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[validate]
    #[serde(rename = "Cert")]
    pub cert: DocumentIdentification28,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<CashAccount40>,
    #[serde(rename = "BkAcctDmcltnCtry", skip_serializing_if = "Option::is_none")]
    pub bk_acct_dmcltn_ctry: Option<CountryCode>,
    #[serde(rename = "Amdmnt", skip_serializing_if = "Option::is_none")]
    pub amdmnt: Option<DocumentAmendment1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "CertRcrd", default)]
    pub cert_rcrd: Vec<TransactionCertificateRecord2<A>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegisteredContractJournal2 {
    #[validate]
    #[serde(rename = "RegnAgt")]
    pub regn_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "UnqId", skip_serializing_if = "Option::is_none")]
    pub unq_id: Option<DocumentIdentification28>,
    #[validate]
    #[serde(rename = "ClsrDt")]
    pub clsr_dt: IsoDate,
    #[serde(rename = "ClsrRsn")]
    pub clsr_rsn: ContractClosureReason1Choice,
}
#[derive(
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
#[derive(
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
pub struct DateAndPlaceOfBirth1 {
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
pub struct TradeContract3<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "CtrctDocId", skip_serializing_if = "Option::is_none")]
    pub ctrct_doc_id: Option<DocumentIdentification22>,
    #[serde(rename = "TradTpId", skip_serializing_if = "Option::is_none")]
    pub trad_tp_id: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Buyr", default)]
    pub buyr: Vec<TradeParty5>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Sellr", default)]
    pub sellr: Vec<TradeParty5>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
    pub prlngtn_flg: Option<TrueFalseIndicator>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
    pub xchg_rate_inf: Option<ExchangeRate1>,
    #[serde(rename = "PmtSchdl", skip_serializing_if = "Option::is_none")]
    pub pmt_schdl: Option<InterestPaymentDateRange1>,
    #[serde(rename = "ShipmntSchdl", skip_serializing_if = "Option::is_none")]
    pub shipmnt_schdl: Option<ShipmentSchedule2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Attchmnt", default)]
    pub attchmnt: Vec<DocumentGeneralInformation5<A>>,
}
#[derive(
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
#[derive(
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
pub struct PaymentScheduleType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PaymentScheduleType2Code>,
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
pub struct PaymentScheduleType2Choice {
    #[serde(flatten)]
    pub value: PaymentScheduleType2ChoiceEnum,
}
#[derive(
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
pub struct GenericValidationRuleIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
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
pub struct ExternalShipmentCondition1Code {
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
pub struct RegisteredContractCommunication1 {
    #[serde(rename = "Mtd")]
    pub mtd: CommunicationMethod4Code,
    #[validate]
    #[serde(rename = "Dt")]
    pub dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CommunicationMethod4Code {
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "ONLI")]
    Onli,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "SWMT")]
    Swmt,
    #[serde(rename = "SWMX")]
    Swmx,
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
pub struct ExternalValidationRuleIdentification1Code {
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
pub struct ProprietaryReference1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentCondition1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalShipmentCondition1Code>,
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
pub struct ShipmentCondition1Choice {
    #[serde(flatten)]
    pub value: ShipmentCondition1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateContractTerm1 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SignatureEnvelopeReference<
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
pub struct TaxExemptionReasonFormat1ChoiceEnum {
    #[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
    pub strd: Option<TaxExemptReason1Code>,
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxExemptionReasonFormat1Choice {
    #[serde(flatten)]
    pub value: TaxExemptionReasonFormat1ChoiceEnum,
}
#[derive(
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
pub struct TaxParty4 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<Max35Text>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxXmptnRsn", default)]
    pub tax_xmptn_rsn: Vec<TaxExemptionReasonFormat1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max1025Text {
    #[validate(length(min = 1, max = 1025,))]
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
pub struct InterestPaymentDateRange1 {
    #[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
    pub intrst_schdl_id: Option<Max35Text>,
    #[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_dt: Option<IsoDate>,
    #[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
    pub due_dt: Option<IsoDate>,
}
#[derive(
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
pub struct ReportingPeriod4 {
    #[validate]
    #[serde(rename = "FrToDt")]
    pub fr_to_dt: DatePeriod3,
    #[validate]
    #[serde(rename = "FrToTm")]
    pub fr_to_tm: TimePeriod2,
    #[serde(rename = "Tp")]
    pub tp: QueryType3Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShipmentAttribute2 {
    #[serde(rename = "Conds", skip_serializing_if = "Option::is_none")]
    pub conds: Option<ShipmentCondition1Choice>,
    #[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_dt: Option<IsoDate>,
    #[serde(rename = "CtryOfCntrPty", skip_serializing_if = "Option::is_none")]
    pub ctry_of_cntr_pty: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractClosureReason1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalContractClosureReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractClosureReason1Choice {
    #[serde(flatten)]
    pub value: ContractClosureReason1ChoiceEnum,
}
