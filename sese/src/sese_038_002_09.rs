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
    static ref RESTRICTED_FINX_MAX_30_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_31_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,31}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_210_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,210}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_350_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,350}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_35_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_16_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_70_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,70}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_2_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,34}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_MAX_30_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([^/]+/)+([^/]+)|([^/]*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_52_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,52}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_MAX_8_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([^/]+/)+([^/]+)|([^/]*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_EXACT_2_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"XX|TS"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_140_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,140}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.038.002.09".to_string()
}

#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CfiOct2015Identifier {
    #[validate(regex = "CFI_OCT_2015_IDENTIFIER_REGEX")]
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
pub struct CurrencyToBuyOrSell1ChoiceEnum {
    #[serde(rename = "CcyToSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_sell: Option<ActiveCurrencyCode>,
    #[serde(rename = "CcyToBuy", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyToBuyOrSell1Choice {
    #[serde(flatten)]
    pub value: CurrencyToBuyOrSell1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed5Choice {
    #[serde(flatten)]
    pub value: ModificationCancellationAllowed5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeliveryReturn4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeliveryReturn1Code>,
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
pub struct DeliveryReturn4Choice {
    #[serde(flatten)]
    pub value: DeliveryReturn4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NettingEligibility5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct NettingEligibility5Choice {
    #[serde(flatten)]
    pub value: NettingEligibility5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat5Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethodFormat5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashParties38 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentificationAndAccount177>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<PartyIdentificationAndAccount178>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentificationAndAccount177>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<PartyIdentificationAndAccount178>,
    #[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
    pub intrmy: Option<PartyIdentificationAndAccount178>,
}
#[derive(
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
pub struct PartyIdentification147ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification147Choice {
    #[serde(flatten)]
    pub value: PartyIdentification147ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification176ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
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
pub struct PartyIdentification176Choice {
    #[serde(flatten)]
    pub value: PartyIdentification176ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number23ChoiceEnum {
    #[serde(rename = "Shrt", skip_serializing_if = "Option::is_none")]
    pub shrt: Option<Exact3NumericText>,
    #[serde(rename = "Lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<GenericIdentification18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number23Choice {
    #[serde(flatten)]
    pub value: Number23ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FormOfSecurity7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FormOfSecurity1Code>,
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
pub struct FormOfSecurity7Choice {
    #[serde(flatten)]
    pub value: FormOfSecurity7ChoiceEnum,
}
#[derive(
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
pub struct References58ChoiceEnum {
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none")]
    pub intra_pos_mvmnt_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
    pub othr_tx_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References58Choice {
    #[serde(flatten)]
    pub value: References58ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType12Code {
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CCPC")]
    Ccpc,
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
    #[serde(rename = "EXTD")]
    Extd,
    #[serde(rename = "EQUS")]
    Equs,
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
    #[serde(rename = "REPO")]
    Repo,
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
    #[serde(rename = "SHSL")]
    Shsl,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "UDMS")]
    Udms,
    #[serde(rename = "TRCP")]
    Trcp,
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
pub struct RegistrationParameters7 {
    #[serde(rename = "CertfctnId", skip_serializing_if = "Option::is_none")]
    pub certfctn_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "CertfctnDtTm", skip_serializing_if = "Option::is_none")]
    pub certfctn_dt_tm: Option<DateAndDateTime2Choice>,
    #[serde(rename = "RegarAcct", skip_serializing_if = "Option::is_none")]
    pub regar_acct: Option<RestrictedFinxMax35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<SecuritiesCertificate5>,
}
#[derive(
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
pub struct AffirmationStatus9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AffirmationStatus1Code>,
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
pub struct AffirmationStatus9Choice {
    #[serde(flatten)]
    pub value: AffirmationStatus9ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Reporting2Code {
    #[serde(rename = "STEX")]
    Stex,
    #[serde(rename = "REGU")]
    Regu,
    #[serde(rename = "DEFR")]
    Defr,
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
pub struct RestrictedFinxMax210Text {
    #[validate(
        length(min = 1, max = 210,),
        regex = "RESTRICTED_FINX_MAX_210_TEXT_REGEX"
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
pub struct SafeKeepingPlace4 {
    #[serde(rename = "SfkpgPlcFrmt", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc_frmt: Option<SafekeepingPlaceFormat39Choice>,
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
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OriginatorRole2Code {
    #[serde(rename = "SINT")]
    Sint,
    #[serde(rename = "MLTF")]
    Mltf,
    #[serde(rename = "RMKT")]
    Rmkt,
    #[serde(rename = "MKTM")]
    Mktm,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "TAGT")]
    Tagt,
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
pub struct SafekeepingPlaceFormat39ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification85>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText15>,
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
pub struct SafekeepingPlaceFormat39Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat39ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms23 {
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
pub struct PartyIdentificationAndAccount206 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceType5ChoiceEnum {
    #[serde(rename = "Mkt", skip_serializing_if = "Option::is_none")]
    pub mkt: Option<Price3>,
    #[serde(rename = "Indctv", skip_serializing_if = "Option::is_none")]
    pub indctv: Option<Price3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceType5Choice {
    #[serde(flatten)]
    pub value: PriceType5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndIdentification1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AnyBicDec2014Identifier,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesSettlementTransactionDetails54<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(
        rename = "SttlmTpAndAddtlParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_tp_and_addtl_params: Option<SettlementTypeAndAdditionalParameters17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Lnkgs", default)]
    pub lnkgs: Vec<Linkages49>,
    #[serde(rename = "TradDtls", skip_serializing_if = "Option::is_none")]
    pub trad_dtls: Option<SecuritiesTradeDetails132>,
    #[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_attrbts: Option<FinancialInstrumentAttributes122>,
    #[serde(rename = "QtyAndAcctDtls", skip_serializing_if = "Option::is_none")]
    pub qty_and_acct_dtls: Option<QuantityAndAccount111>,
    #[serde(rename = "SttlmParams", skip_serializing_if = "Option::is_none")]
    pub sttlm_params: Option<SettlementDetails208>,
    #[serde(rename = "StgSttlmInstrDtls", skip_serializing_if = "Option::is_none")]
    pub stg_sttlm_instr_dtls: Option<StandingSettlementInstruction19>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties119>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties119>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties38>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection101>,
    #[serde(rename = "OthrAmts", skip_serializing_if = "Option::is_none")]
    pub othr_amts: Option<OtherAmounts43>,
    #[serde(rename = "OthrBizPties", skip_serializing_if = "Option::is_none")]
    pub othr_biz_pties: Option<OtherParties44>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters7>,
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
pub struct SettlementTypeAndIdentification22 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: RestrictedFinxMax16Text,
    #[serde(rename = "SctiesMvmntTp")]
    pub scties_mvmnt_tp: ReceiveDelivery1Code,
    #[serde(rename = "Pmt")]
    pub pmt: DeliveryReceiptType2Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StandingSettlementInstruction19 {
    #[serde(rename = "SttlmStgInstrDB")]
    pub sttlm_stg_instr_db: SettlementStandingInstructionDatabase5Choice,
    #[serde(rename = "CtrPty")]
    pub ctr_pty: Counterparty16Choice,
    #[serde(rename = "Vndr", skip_serializing_if = "Option::is_none")]
    pub vndr: Option<PartyIdentification157>,
    #[serde(
        rename = "OthrDlvrgSttlmPties",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_dlvrg_sttlm_pties: Option<SettlementParties105>,
    #[serde(rename = "OthrRcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub othr_rcvg_sttlm_pties: Option<SettlementParties105>,
}
#[derive(
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
pub struct TaxCapacityParty5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxLiability1Code>,
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
pub struct TaxCapacityParty5Choice {
    #[serde(flatten)]
    pub value: TaxCapacityParty5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionType1Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition10Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ASGN")]
    Asgn,
    #[serde(rename = "BUTC")]
    Butc,
    #[serde(rename = "CLEN")]
    Clen,
    #[serde(rename = "DLWM")]
    Dlwm,
    #[serde(rename = "DIRT")]
    Dirt,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "FRCL")]
    Frcl,
    #[serde(rename = "KNOC")]
    Knoc,
    #[serde(rename = "NOMC")]
    Nomc,
    #[serde(rename = "NACT")]
    Nact,
    #[serde(rename = "PENS")]
    Pens,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "RHYP")]
    Rhyp,
    #[serde(rename = "RPTO")]
    Rpto,
    #[serde(rename = "RESI")]
    Resi,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "SPDL")]
    Spdl,
    #[serde(rename = "SPST")]
    Spst,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "TRIP")]
    Trip,
    #[serde(rename = "UNEX")]
    Unex,
    #[default]
    Unknown,
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
pub struct AmountAndDirection58 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<RestrictedFinActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms27>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount209 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<RestrictedFinxMax35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndText15 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<RestrictedFinxMax30Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionDetails160 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification20,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate9Choice>,
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: SettlementDate20Choice,
    #[serde(rename = "SttlmQty")]
    pub sttlm_qty: Quantity54Choice,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection67>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties109>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties109>,
    #[serde(rename = "Invstr", skip_serializing_if = "Option::is_none")]
    pub invstr: Option<PartyIdentification170>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification18 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax30Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max4AlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeOriginator4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OriginatorRole2Code>,
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
pub struct TradeOriginator4Choice {
    #[serde(flatten)]
    pub value: TradeOriginator4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount208 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification137Choice>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<CountryCode>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<RestrictedFinxMax35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(
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
pub struct SecuritiesTradeDetails133 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TradId", default)]
    pub trad_id: Vec<RestrictedFinxMax52Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollTxId", default)]
    pub coll_tx_id: Vec<RestrictedFinxMax16Text>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification2>,
    #[serde(rename = "PlcOfClr", skip_serializing_if = "Option::is_none")]
    pub plc_of_clr: Option<PlaceOfClearingIdentification2>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate9Choice>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<SettlementDate20Choice>,
    #[serde(rename = "LateDlvryDt", skip_serializing_if = "Option::is_none")]
    pub late_dlvry_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price11>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "OpngClsg", skip_serializing_if = "Option::is_none")]
    pub opng_clsg: Option<OpeningClosing4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rptg", default)]
    pub rptg: Vec<Reporting9Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition6Choice>,
    #[serde(rename = "InvstrCpcty", skip_serializing_if = "Option::is_none")]
    pub invstr_cpcty: Option<InvestorCapacity5Choice>,
    #[serde(rename = "TradOrgtrRole", skip_serializing_if = "Option::is_none")]
    pub trad_orgtr_role: Option<TradeOriginator4Choice>,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice32Choice>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus28Choice>,
    #[serde(rename = "AffirmSts", skip_serializing_if = "Option::is_none")]
    pub affirm_sts: Option<AffirmationStatus9Choice>,
    #[serde(rename = "FxAddtlDtls", skip_serializing_if = "Option::is_none")]
    pub fx_addtl_dtls: Option<RestrictedFinxMax350Text>,
    #[serde(
        rename = "SttlmInstrPrcgAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_instr_prcg_addtl_dtls: Option<RestrictedFinxMax350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax350Text {
    #[validate(
        length(min = 1, max = 350,),
        regex = "RESTRICTED_FINX_MAX_350_TEXT_REGEX"
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
pub struct SettlementParties105 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification162>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount206>,
}
#[derive(
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
pub struct PlaceOfTradeIdentification2 {
    #[serde(rename = "MktTpAndId", skip_serializing_if = "Option::is_none")]
    pub mkt_tp_and_id: Option<MarketIdentification90>,
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
pub struct SecuritiesTransactionType60ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesTransactionType28Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionType60Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionType60ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification4ChoiceEnum {
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<RestrictedFinxMax30Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification4Choice {
    #[serde(flatten)]
    pub value: MarketIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlingCapacity2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity8Choice {
    #[serde(flatten)]
    pub value: SettlingCapacity8ChoiceEnum,
}
#[derive(
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
> {
    #[validate]
    #[serde(rename = "SctiesSttlmTxModReq")]
    pub scties_sttlm_tx_mod_req: SecuritiesSettlementTransactionModificationRequest002V09<A, B, C>,
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
pub struct FxStandingInstruction5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct FxStandingInstruction5Choice {
    #[serde(flatten)]
    pub value: FxStandingInstruction5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate9ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<TradeDateCode4Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate9Choice {
    #[serde(flatten)]
    pub value: TradeDate9ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice14Code {
    #[serde(rename = "AVER")]
    Aver,
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
pub struct Exact3NumericText {
    #[validate(regex = "EXACT_3_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OpeningClosing1Code {
    #[serde(rename = "CLOP")]
    Clop,
    #[serde(rename = "OPEP")]
    Opep,
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
pub struct Reporting9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Reporting2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reporting9Choice {
    #[serde(flatten)]
    pub value: Reporting9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "RestrictedFinActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: RestrictedFinActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
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
pub struct PartyIdentificationAndAccount177 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "ComssnAcct", skip_serializing_if = "Option::is_none")]
    pub comssn_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation3 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<RestrictedFinxMax350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<RestrictedFinxMax350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityAndAccount112 {
    #[serde(rename = "SttlmQty", skip_serializing_if = "Option::is_none")]
    pub sttlm_qty: Option<Quantity54Choice>,
    #[serde(rename = "DnmtnChc", skip_serializing_if = "Option::is_none")]
    pub dnmtn_chc: Option<RestrictedFinxMax210Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafeKeepingPlace4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "QtyBrkdwn", default)]
    pub qty_brkdwn: Vec<QuantityBreakdown69>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection44 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms23>,
}
#[derive(
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
pub enum EventFrequency3Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType12Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "NEGA")]
    Nega,
    #[default]
    Unknown,
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
pub struct Quantity54ChoiceEnum {
    #[serde(rename = "OrgnlAndCurFace", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face: Option<OriginalAndCurrentQuantities4>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity36Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity54Choice {
    #[serde(flatten)]
    pub value: Quantity54ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesPaymentStatus6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesPaymentStatus1Code>,
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
pub struct SecuritiesPaymentStatus6Choice {
    #[serde(flatten)]
    pub value: SecuritiesPaymentStatus6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesSettlementTransactionModificationRequest002V09<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "ModfdTxDtls")]
    pub modfd_tx_dtls: TransactionDetails159,
    #[validate(length(min = 1, max = 3,))]
    #[serde(rename = "UpdTp", default)]
    pub upd_tp: Vec<UpdateType38Choice<A, B, C>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition4Code {
    #[serde(rename = "CBNS")]
    Cbns,
    #[serde(rename = "XBNS")]
    Xbns,
    #[serde(rename = "CCPN")]
    Ccpn,
    #[serde(rename = "XCPN")]
    Xcpn,
    #[serde(rename = "CDIV")]
    Cdiv,
    #[serde(rename = "XDIV")]
    Xdiv,
    #[serde(rename = "CRTS")]
    Crts,
    #[serde(rename = "XRTS")]
    Xrts,
    #[serde(rename = "CWAR")]
    Cwar,
    #[serde(rename = "XWAR")]
    Xwar,
    #[serde(rename = "SPCU")]
    Spcu,
    #[serde(rename = "SPEX")]
    Spex,
    #[serde(rename = "GTDL")]
    Gtdl,
    #[serde(rename = "BCRO")]
    Bcro,
    #[serde(rename = "BCRP")]
    Bcrp,
    #[serde(rename = "BCFD")]
    Bcfd,
    #[serde(rename = "BCBL")]
    Bcbl,
    #[serde(rename = "BCBN")]
    Bcbn,
    #[serde(rename = "MAPR")]
    Mapr,
    #[serde(rename = "NEGO")]
    Nego,
    #[serde(rename = "NMPR")]
    Nmpr,
    #[serde(rename = "BCPD")]
    Bcpd,
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
pub struct Max30DecimalNumber {
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
pub struct AmountAndDirection67 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<RestrictedFinActiveOrHistoricCurrencyAndAmount>,
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
pub struct ExposureType24ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExposureType12Code>,
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
pub struct ExposureType24Choice {
    #[serde(flatten)]
    pub value: ExposureType24ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MatchingStatus1Code>,
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
pub struct PartyIdentification191 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification145Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashSettlementSystem2Code {
    #[serde(rename = "GROS")]
    Gros,
    #[serde(rename = "NETS")]
    Nets,
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
pub struct QuantityBreakdown69 {
    #[serde(rename = "LotNb", skip_serializing_if = "Option::is_none")]
    pub lot_nb: Option<GenericIdentification39>,
    #[serde(rename = "LotQty", skip_serializing_if = "Option::is_none")]
    pub lot_qty: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "LotDtTm", skip_serializing_if = "Option::is_none")]
    pub lot_dt_tm: Option<DateAndDateTime2Choice>,
    #[serde(rename = "LotPric", skip_serializing_if = "Option::is_none")]
    pub lot_pric: Option<Price3>,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice32Choice>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesSettlementTransactionDetails55<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(
        rename = "SttlmTpAndAddtlParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_tp_and_addtl_params: Option<SettlementTypeAndAdditionalParameters18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Lnkgs", default)]
    pub lnkgs: Vec<Linkages70>,
    #[serde(rename = "TradDtls", skip_serializing_if = "Option::is_none")]
    pub trad_dtls: Option<SecuritiesTradeDetails133>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification20>,
    #[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_attrbts: Option<FinancialInstrumentAttributes122>,
    #[serde(rename = "QtyAndAcctDtls", skip_serializing_if = "Option::is_none")]
    pub qty_and_acct_dtls: Option<QuantityAndAccount112>,
    #[serde(rename = "SttlmParams", skip_serializing_if = "Option::is_none")]
    pub sttlm_params: Option<SettlementDetails208>,
    #[serde(rename = "StgSttlmInstrDtls", skip_serializing_if = "Option::is_none")]
    pub stg_sttlm_instr_dtls: Option<StandingSettlementInstruction19>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties105>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties105>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties38>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection101>,
    #[serde(rename = "OthrAmts", skip_serializing_if = "Option::is_none")]
    pub othr_amts: Option<OtherAmounts43>,
    #[serde(rename = "OthrBizPties", skip_serializing_if = "Option::is_none")]
    pub othr_biz_pties: Option<OtherParties44>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters7>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryReturn1Code {
    #[serde(rename = "UNRE")]
    Unre,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DUEB")]
    Dueb,
    #[serde(rename = "PARD")]
    Pard,
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
pub struct BeneficialOwnership5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct BeneficialOwnership5Choice {
    #[serde(flatten)]
    pub value: BeneficialOwnership5ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification157 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
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
pub struct RestrictedFinActiveOrHistoricCurrencyAndAmountSimpleType {
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
pub struct PartyIdentification137ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification137Choice {
    #[serde(flatten)]
    pub value: PartyIdentification137ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount178 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification147Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "ComssnAcct", skip_serializing_if = "Option::is_none")]
    pub comssn_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price3 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax16Text {
    #[validate(
        length(min = 1, max = 16,),
        regex = "RESTRICTED_FINX_MAX_16_TEXT_REGEX"
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
pub struct CentralCounterPartyEligibility5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct CentralCounterPartyEligibility5Choice {
    #[serde(flatten)]
    pub value: CentralCounterPartyEligibility5ChoiceEnum,
}
#[derive(
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
pub struct ClassificationType33ChoiceEnum {
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification86>,
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClassificationType33Choice {
    #[serde(flatten)]
    pub value: ClassificationType33ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesPaymentStatus1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NILL")]
    Nill,
    #[serde(rename = "PART")]
    Part,
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
pub struct ForeignExchangeTerms27 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[validate]
    #[serde(rename = "RsltgAmt")]
    pub rsltg_amt: RestrictedFinActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Linkages49 {
    #[serde(rename = "Ref")]
    pub r#ref: References58Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDate4Code {
    #[serde(rename = "WISS")]
    Wiss,
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
pub struct TransactionDetails159 {
    #[validate]
    #[serde(rename = "AcctOwnrTxId")]
    pub acct_ownr_tx_id: SettlementTypeAndIdentification22,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none")]
    pub prcr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
    pub othr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
    pub tx_dtls: Option<TransactionDetails160>,
}
#[derive(
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
pub struct PartyIdentification162 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification145Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlingCapacity2Code {
    #[serde(rename = "SAGE")]
    Sage,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "SPRI")]
    Spri,
    #[serde(rename = "RISP")]
    Risp,
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
pub struct RestrictedFinx2Max34Text {
    #[validate(
        length(min = 1, max = 34,),
        regex = "RESTRICTED_FINX_2_MAX_34_TEXT_REGEX"
    )]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxLiability1Code {
    #[serde(rename = "PRIN")]
    Prin,
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
pub struct CashSettlementSystem5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashSettlementSystem2Code>,
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
pub struct CashSettlementSystem5Choice {
    #[serde(flatten)]
    pub value: CashSettlementSystem5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternatePartyIdentification9 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType44Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: RestrictedFinxMax30Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration11ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Registration1Code>,
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
pub struct Registration11Choice {
    #[serde(flatten)]
    pub value: Registration11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionStyle2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle9Choice {
    #[serde(flatten)]
    pub value: OptionStyle9ChoiceEnum,
}
#[derive(
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
pub struct OptionType7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionType7Choice {
    #[serde(flatten)]
    pub value: OptionType7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalAndCurrentQuantities4 {
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: RestrictedFinImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: RestrictedFinImpliedCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDate4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode9Choice {
    #[serde(flatten)]
    pub value: SettlementDateCode9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency27ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency3Code>,
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
pub struct Frequency27Choice {
    #[serde(flatten)]
    pub value: Frequency27ChoiceEnum,
}
#[derive(
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
pub struct AmountAndDirection101 {
    #[serde(rename = "AcrdIntrstInd", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_ind: Option<YesNoIndicator>,
    #[serde(rename = "StmpDtyInd", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_ind: Option<YesNoIndicator>,
    #[serde(rename = "BrkrgAmtInd", skip_serializing_if = "Option::is_none")]
    pub brkrg_amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "RsrchFeeInd", skip_serializing_if = "Option::is_none")]
    pub rsrch_fee_ind: Option<YesNoIndicator>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(
        rename = "OrgnlCcyAndOrdrdAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_ccy_and_ordrd_amt: Option<RestrictedFinActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms27>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinMax30Text {
    #[validate(length(min = 1, max = 30,), regex = "RESTRICTED_FIN_MAX_30_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessingPosition3Code {
    #[serde(rename = "AFTE")]
    Afte,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "BEFO")]
    Befo,
    #[serde(rename = "INFO")]
    Info,
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
pub struct BicfiDec2014Identifier {
    #[validate(regex = "BICFI_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Eligibility1Code {
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
pub enum MarketClientSide1Code {
    #[serde(rename = "CLNT")]
    Clnt,
    #[serde(rename = "MAKT")]
    Makt,
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
pub struct MarketClientSide7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketClientSide1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketClientSide7Choice {
    #[serde(flatten)]
    pub value: MarketClientSide7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification90 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification2Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType16Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReceiveDelivery1Code {
    #[serde(rename = "DELI")]
    Deli,
    #[serde(rename = "RECE")]
    Rece,
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
pub struct Restriction6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OwnershipLegalRestrictions1Code>,
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
pub struct Restriction6Choice {
    #[serde(flatten)]
    pub value: Restriction6ChoiceEnum,
}
#[derive(
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
pub struct SecuritiesSettlementTransactionDetails53<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(
        rename = "SttlmTpAndAddtlParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_tp_and_addtl_params: Option<SettlementTypeAndAdditionalParameters17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Lnkgs", default)]
    pub lnkgs: Vec<Linkages70>,
    #[serde(rename = "TradDtls", skip_serializing_if = "Option::is_none")]
    pub trad_dtls: Option<SecuritiesTradeDetails132>,
    #[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_attrbts: Option<FinancialInstrumentAttributes122>,
    #[serde(rename = "QtyAndAcctDtls", skip_serializing_if = "Option::is_none")]
    pub qty_and_acct_dtls: Option<QuantityAndAccount111>,
    #[serde(rename = "SttlmParams", skip_serializing_if = "Option::is_none")]
    pub sttlm_params: Option<SettlementDetails208>,
    #[serde(rename = "StgSttlmInstrDtls", skip_serializing_if = "Option::is_none")]
    pub stg_sttlm_instr_dtls: Option<StandingSettlementInstruction19>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties119>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties119>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties38>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection101>,
    #[serde(rename = "OthrAmts", skip_serializing_if = "Option::is_none")]
    pub othr_amts: Option<OtherAmounts43>,
    #[serde(rename = "OthrBizPties", skip_serializing_if = "Option::is_none")]
    pub othr_biz_pties: Option<OtherParties44>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters7>,
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
pub struct DocumentNumber6ChoiceEnum {
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification86>,
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinActiveCurrencyAndAmount {
    #[serde(rename = "RestrictedFinActiveCurrencyAndAmount")]
    pub value: RestrictedFinActiveCurrencyAndAmountSimpleType,
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
pub struct FinancialInstrumentAttributes122 {
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification4Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat5Choice>,
    #[serde(rename = "RegnForm", skip_serializing_if = "Option::is_none")]
    pub regn_form: Option<FormOfSecurity7Choice>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency27Choice>,
    #[serde(rename = "PmtSts", skip_serializing_if = "Option::is_none")]
    pub pmt_sts: Option<SecuritiesPaymentStatus6Choice>,
    #[serde(rename = "VarblRateChngFrqcy", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_chng_frqcy: Option<Frequency27Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType33Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle9Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType7Choice>,
    #[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
    pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "CpnDt", skip_serializing_if = "Option::is_none")]
    pub cpn_dt: Option<IsoDate>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[serde(rename = "FltgRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fltg_rate_fxg_dt: Option<IsoDate>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoDate>,
    #[serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cllbl_dt: Option<IsoDate>,
    #[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
    pub putbl_dt: Option<IsoDate>,
    #[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
    pub dtd_dt: Option<IsoDate>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<BaseOneRate>,
    #[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
    pub cur_fctr: Option<BaseOneRate>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<BaseOneRate>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<PercentageRate>,
    #[serde(rename = "YldToMtrtyRate", skip_serializing_if = "Option::is_none")]
    pub yld_to_mtrty_rate: Option<PercentageRate>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<PercentageRate>,
    #[serde(rename = "IndxRateBsis", skip_serializing_if = "Option::is_none")]
    pub indx_rate_bsis: Option<PercentageRate>,
    #[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
    pub cpn_attchd_nb: Option<Number23Choice>,
    #[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
    pub pool_nb: Option<GenericIdentification39>,
    #[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_ind: Option<YesNoIndicator>,
    #[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
    pub putbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "MktOrIndctvPric", skip_serializing_if = "Option::is_none")]
    pub mkt_or_indctv_pric: Option<PriceType5Choice>,
    #[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
    pub exrc_pric: Option<Price3>,
    #[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
    pub sbcpt_pric: Option<Price3>,
    #[serde(rename = "ConvsPric", skip_serializing_if = "Option::is_none")]
    pub convs_pric: Option<Price3>,
    #[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
    pub strk_pric: Option<Price3>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity36Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygFinInstrmId", default)]
    pub undrlyg_fin_instrm_id: Vec<SecurityIdentification20>,
    #[serde(
        rename = "FinInstrmAttrAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub fin_instrm_attr_addtl_dtls: Option<RestrictedFinxMax350Text>,
}
#[derive(
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
pub struct SettlementParties119 {
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount206>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<RestrictedFinx2Max34Text>,
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
pub struct CashAccountIdentification6Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType3Code {
    #[serde(rename = "VARI")]
    Vari,
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
pub struct PartyIdentification136ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTypeAndAdditionalParameters18 {
    #[serde(rename = "SctiesMvmntTp", skip_serializing_if = "Option::is_none")]
    pub scties_mvmnt_tp: Option<ReceiveDelivery1Code>,
    #[serde(rename = "Pmt", skip_serializing_if = "Option::is_none")]
    pub pmt: Option<DeliveryReceiptType2Code>,
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "RcncltnInd", skip_serializing_if = "Option::is_none")]
    pub rcncltn_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity36ChoiceEnum {
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<RestrictedFinImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity36Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity36ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDetails208 {
    #[serde(rename = "SctiesTxTp", skip_serializing_if = "Option::is_none")]
    pub scties_tx_tp: Option<SecuritiesTransactionType60Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition28Choice>,
    #[serde(rename = "BnfclOwnrsh", skip_serializing_if = "Option::is_none")]
    pub bnfcl_ownrsh: Option<BeneficialOwnership5Choice>,
    #[serde(rename = "CCPElgblty", skip_serializing_if = "Option::is_none")]
    pub ccp_elgblty: Option<CentralCounterPartyEligibility5Choice>,
    #[serde(rename = "DlvryRtrRsn", skip_serializing_if = "Option::is_none")]
    pub dlvry_rtr_rsn: Option<DeliveryReturn4Choice>,
    #[serde(rename = "CshClrSys", skip_serializing_if = "Option::is_none")]
    pub csh_clr_sys: Option<CashSettlementSystem5Choice>,
    #[serde(rename = "XpsrTp", skip_serializing_if = "Option::is_none")]
    pub xpsr_tp: Option<ExposureType24Choice>,
    #[serde(rename = "FxStgInstr", skip_serializing_if = "Option::is_none")]
    pub fx_stg_instr: Option<FxStandingInstruction5Choice>,
    #[serde(rename = "MktClntSd", skip_serializing_if = "Option::is_none")]
    pub mkt_clnt_sd: Option<MarketClientSide7Choice>,
    #[serde(rename = "NetgElgblty", skip_serializing_if = "Option::is_none")]
    pub netg_elgblty: Option<NettingEligibility5Choice>,
    #[serde(rename = "Regn", skip_serializing_if = "Option::is_none")]
    pub regn: Option<Registration11Choice>,
    #[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctns: Option<Restriction6Choice>,
    #[serde(rename = "SttlgCpcty", skip_serializing_if = "Option::is_none")]
    pub sttlg_cpcty: Option<SettlingCapacity8Choice>,
    #[serde(rename = "TaxCpcty", skip_serializing_if = "Option::is_none")]
    pub tax_cpcty: Option<TaxCapacityParty5Choice>,
    #[serde(rename = "StmpDtyTaxBsis", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_tax_bsis: Option<GenericIdentification47>,
    #[serde(rename = "Trckg", skip_serializing_if = "Option::is_none")]
    pub trckg: Option<Tracking5Choice>,
    #[serde(rename = "LttrOfGrnt", skip_serializing_if = "Option::is_none")]
    pub lttr_of_grnt: Option<LetterOfGuarantee5Choice>,
    #[serde(rename = "RtrLeg", skip_serializing_if = "Option::is_none")]
    pub rtr_leg: Option<YesNoIndicator>,
    #[serde(rename = "ModCxlAllwd", skip_serializing_if = "Option::is_none")]
    pub mod_cxl_allwd: Option<ModificationCancellationAllowed5Choice>,
    #[serde(rename = "ElgblForColl", skip_serializing_if = "Option::is_none")]
    pub elgbl_for_coll: Option<YesNoIndicator>,
    #[serde(rename = "SctiesSubBalTp", skip_serializing_if = "Option::is_none")]
    pub scties_sub_bal_tp: Option<GenericIdentification47>,
    #[serde(rename = "CshSubBalTp", skip_serializing_if = "Option::is_none")]
    pub csh_sub_bal_tp: Option<GenericIdentification47>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementStandingInstructionDatabase1Code {
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "BRKR")]
    Brkr,
    #[serde(rename = "VEND")]
    Vend,
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
pub struct TypeOfPrice32ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice14Code>,
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
pub struct TypeOfPrice32Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice32ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateCode4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType3Code>,
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
pub struct TradeDateCode4Choice {
    #[serde(flatten)]
    pub value: TradeDateCode4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod2Code {
    #[serde(rename = "A001")]
    A001,
    #[serde(rename = "A002")]
    A002,
    #[serde(rename = "A003")]
    A003,
    #[serde(rename = "A004")]
    A004,
    #[serde(rename = "A005")]
    A005,
    #[serde(rename = "A006")]
    A006,
    #[serde(rename = "A007")]
    A007,
    #[serde(rename = "A008")]
    A008,
    #[serde(rename = "A009")]
    A009,
    #[serde(rename = "A010")]
    A010,
    #[serde(rename = "A011")]
    A011,
    #[serde(rename = "A012")]
    A012,
    #[serde(rename = "A013")]
    A013,
    #[serde(rename = "A014")]
    A014,
    #[serde(rename = "NARR")]
    Narr,
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
pub struct PairedOrTurnedQuantity6ChoiceEnum {
    #[serde(rename = "PairdOffQty", skip_serializing_if = "Option::is_none")]
    pub paird_off_qty: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "TrndQty", skip_serializing_if = "Option::is_none")]
    pub trnd_qty: Option<FinancialInstrumentQuantity36Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PairedOrTurnedQuantity6Choice {
    #[serde(flatten)]
    pub value: PairedOrTurnedQuantity6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties109 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification191>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount215>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount215>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount215>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount215>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount215>,
}
#[derive(
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
pub struct InvestorCapacity5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Eligibility1Code>,
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
pub struct InvestorCapacity5Choice {
    #[serde(flatten)]
    pub value: InvestorCapacity5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LetterOfGuarantee5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct LetterOfGuarantee5Choice {
    #[serde(flatten)]
    pub value: LetterOfGuarantee5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OwnershipLegalRestrictions1Code {
    #[serde(rename = "A144")]
    A144,
    #[serde(rename = "NRST")]
    Nrst,
    #[serde(rename = "RSTR")]
    Rstr,
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
pub struct PriceRateOrAmount1ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveOrHistoricCurrencyAnd13DecimalAmount>,
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
pub struct PriceRateOrAmount1Choice {
    #[serde(flatten)]
    pub value: PriceRateOrAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition10ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingPosition3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition10Choice {
    #[serde(flatten)]
    pub value: ProcessingPosition10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTradeDetails132 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TradId", default)]
    pub trad_id: Vec<RestrictedFinxMax52Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollTxId", default)]
    pub coll_tx_id: Vec<RestrictedFinxMax16Text>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification2>,
    #[serde(rename = "PlcOfClr", skip_serializing_if = "Option::is_none")]
    pub plc_of_clr: Option<PlaceOfClearingIdentification2>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate9Choice>,
    #[serde(rename = "LateDlvryDt", skip_serializing_if = "Option::is_none")]
    pub late_dlvry_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price11>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "OpngClsg", skip_serializing_if = "Option::is_none")]
    pub opng_clsg: Option<OpeningClosing4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rptg", default)]
    pub rptg: Vec<Reporting9Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition6Choice>,
    #[serde(rename = "InvstrCpcty", skip_serializing_if = "Option::is_none")]
    pub invstr_cpcty: Option<InvestorCapacity5Choice>,
    #[serde(rename = "TradOrgtrRole", skip_serializing_if = "Option::is_none")]
    pub trad_orgtr_role: Option<TradeOriginator4Choice>,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice32Choice>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus28Choice>,
    #[serde(rename = "AffirmSts", skip_serializing_if = "Option::is_none")]
    pub affirm_sts: Option<AffirmationStatus9Choice>,
    #[serde(rename = "FxAddtlDtls", skip_serializing_if = "Option::is_none")]
    pub fx_addtl_dtls: Option<RestrictedFinxMax350Text>,
    #[serde(
        rename = "SttlmInstrPrcgAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_instr_prcg_addtl_dtls: Option<RestrictedFinxMax350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification39 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinMax30Text,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<RestrictedFinMax8Text>,
}
#[derive(
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
pub struct MarketIdentification2ChoiceEnum {
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<RestrictedFinxMax30Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification2Choice {
    #[serde(flatten)]
    pub value: MarketIdentification2ChoiceEnum,
}
#[derive(
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
pub struct OtherParties44 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Invstr", default)]
    pub invstr: Vec<PartyIdentificationAndAccount208>,
    #[serde(rename = "QlfdFrgnIntrmy", skip_serializing_if = "Option::is_none")]
    pub qlfd_frgn_intrmy: Option<PartyIdentificationAndAccount209>,
    #[serde(rename = "StockXchg", skip_serializing_if = "Option::is_none")]
    pub stock_xchg: Option<PartyIdentificationAndAccount181>,
    #[serde(rename = "TradRgltr", skip_serializing_if = "Option::is_none")]
    pub trad_rgltr: Option<PartyIdentificationAndAccount181>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<PartyIdentificationAndAccount209>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<PartyIdentificationAndAccount209>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification145ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification145Choice {
    #[serde(flatten)]
    pub value: PartyIdentification145ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct Linkages70 {
    #[serde(rename = "PrcgPos", skip_serializing_if = "Option::is_none")]
    pub prcg_pos: Option<ProcessingPosition10Choice>,
    #[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
    pub msg_nb: Option<DocumentNumber6Choice>,
    #[serde(rename = "Ref")]
    pub r#ref: References58Choice,
    #[serde(rename = "LkdQty", skip_serializing_if = "Option::is_none")]
    pub lkd_qty: Option<PairedOrTurnedQuantity6Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType1Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
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
pub struct RestrictedFinDecimalNumber {
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
pub struct RestrictedFinImpliedCurrencyAndAmount {
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
pub struct TradeTransactionCondition6ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeTransactionCondition4Code>,
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
pub struct TradeTransactionCondition6Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition6ChoiceEnum,
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
pub struct YieldedOrValueType1ChoiceEnum {
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType1Code>,
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType1Choice {
    #[serde(flatten)]
    pub value: YieldedOrValueType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "RestrictedFinActiveOrHistoricCurrencyAndAmount")]
    pub value: RestrictedFinActiveOrHistoricCurrencyAndAmountSimpleType,
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
pub struct YieldedOrValueType2ChoiceEnum {
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType12Code>,
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType2Choice {
    #[serde(flatten)]
    pub value: YieldedOrValueType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpeningClosing4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OpeningClosing1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpeningClosing4Choice {
    #[serde(flatten)]
    pub value: OpeningClosing4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStandingInstructionDatabase5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementStandingInstructionDatabase1Code>,
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
pub struct SettlementStandingInstructionDatabase5Choice {
    #[serde(flatten)]
    pub value: SettlementStandingInstructionDatabase5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesTransactionType28Code {
    #[serde(rename = "BSBK")]
    Bsbk,
    #[serde(rename = "COLI")]
    Coli,
    #[serde(rename = "COLO")]
    Colo,
    #[serde(rename = "MKDW")]
    Mkdw,
    #[serde(rename = "MKUP")]
    Mkup,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "NSYN")]
    Nsyn,
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PORT")]
    Port,
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REPU")]
    Repu,
    #[serde(rename = "RODE")]
    Rode,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SECB")]
    Secb,
    #[serde(rename = "SECL")]
    Secl,
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "SYND")]
    Synd,
    #[serde(rename = "TBAC")]
    Tbac,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "TRPO")]
    Trpo,
    #[serde(rename = "TRVO")]
    Trvo,
    #[serde(rename = "TURN")]
    Turn,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CNCB")]
    Cncb,
    #[serde(rename = "OWNE")]
    Owne,
    #[serde(rename = "FCTA")]
    Fcta,
    #[serde(rename = "OWNI")]
    Owni,
    #[serde(rename = "RELE")]
    Rele,
    #[serde(rename = "SBRE")]
    Sbre,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CLAI")]
    Clai,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "SWIF")]
    Swif,
    #[serde(rename = "SWIT")]
    Swit,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "ETFT")]
    Etft,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "SLRE")]
    Slre,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "SBBK")]
    Sbbk,
    #[serde(rename = "REDI")]
    Redi,
    #[serde(rename = "INTT")]
    Intt,
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
pub struct RestrictedFinxMax52Text {
    #[validate(
        length(min = 1, max = 52,),
        regex = "RESTRICTED_FINX_MAX_52_TEXT_REGEX"
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
pub struct Price11 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType2Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount1Choice,
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Registration1Code {
    #[serde(rename = "NREG")]
    Nreg,
    #[serde(rename = "YREG")]
    Yreg,
    #[default]
    Unknown,
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
pub struct RestrictedFinActiveCurrencyAndAmountSimpleType {
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
pub struct RestrictedFinMax8Text {
    #[validate(length(min = 1, max = 8,), regex = "RESTRICTED_FIN_MAX_8_TEXT_REGEX")]
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
pub struct MarketType16ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType2Code>,
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
pub struct MarketType16Choice {
    #[serde(flatten)]
    pub value: MarketType16ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount181 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation4 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<RestrictedFinxMax350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<RestrictedFinxMax140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount215 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOfClearingIdentification2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AnyBicDec2014Identifier>,
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
pub struct ActiveCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct SettlementDate20ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<SettlementDateCode9Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate20Choice {
    #[serde(flatten)]
    pub value: SettlementDate20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmounts43 {
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection58>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<AmountAndDirection58>,
    #[serde(rename = "CtryNtlFdrlTax", skip_serializing_if = "Option::is_none")]
    pub ctry_ntl_fdrl_tax: Option<AmountAndDirection58>,
    #[serde(rename = "TradAmt", skip_serializing_if = "Option::is_none")]
    pub trad_amt: Option<AmountAndDirection58>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<AmountAndDirection58>,
    #[serde(rename = "IsseDscntAllwnc", skip_serializing_if = "Option::is_none")]
    pub isse_dscnt_allwnc: Option<AmountAndDirection58>,
    #[serde(rename = "PmtLevyTax", skip_serializing_if = "Option::is_none")]
    pub pmt_levy_tax: Option<AmountAndDirection58>,
    #[serde(rename = "LclTax", skip_serializing_if = "Option::is_none")]
    pub lcl_tax: Option<AmountAndDirection58>,
    #[serde(rename = "LclTaxCtrySpcfc", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc: Option<AmountAndDirection58>,
    #[serde(rename = "LclBrkrComssn", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn: Option<AmountAndDirection58>,
    #[serde(rename = "Mrgn", skip_serializing_if = "Option::is_none")]
    pub mrgn: Option<AmountAndDirection58>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AmountAndDirection58>,
    #[serde(rename = "RgltryAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_amt: Option<AmountAndDirection58>,
    #[serde(rename = "ShppgAmt", skip_serializing_if = "Option::is_none")]
    pub shppg_amt: Option<AmountAndDirection58>,
    #[serde(rename = "SpclCncssn", skip_serializing_if = "Option::is_none")]
    pub spcl_cncssn: Option<AmountAndDirection58>,
    #[serde(rename = "StmpDty", skip_serializing_if = "Option::is_none")]
    pub stmp_dty: Option<AmountAndDirection58>,
    #[serde(rename = "StockXchgTax", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax: Option<AmountAndDirection58>,
    #[serde(rename = "TrfTax", skip_serializing_if = "Option::is_none")]
    pub trf_tax: Option<AmountAndDirection58>,
    #[serde(rename = "TxTax", skip_serializing_if = "Option::is_none")]
    pub tx_tax: Option<AmountAndDirection58>,
    #[serde(rename = "ValAddedTax", skip_serializing_if = "Option::is_none")]
    pub val_added_tax: Option<AmountAndDirection58>,
    #[serde(rename = "WhldgTax", skip_serializing_if = "Option::is_none")]
    pub whldg_tax: Option<AmountAndDirection58>,
    #[serde(rename = "NetGnLoss", skip_serializing_if = "Option::is_none")]
    pub net_gn_loss: Option<AmountAndDirection58>,
    #[serde(rename = "CsmptnTax", skip_serializing_if = "Option::is_none")]
    pub csmptn_tax: Option<AmountAndDirection58>,
    #[serde(rename = "AcrdCptlstnAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_cptlstn_amt: Option<AmountAndDirection58>,
    #[serde(rename = "RsrchFee", skip_serializing_if = "Option::is_none")]
    pub rsrch_fee: Option<AmountAndDirection44>,
}
#[derive(
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
pub struct PartyIdentification170 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification176Choice,
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
pub struct SettlementTransactionCondition28ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition10Code>,
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
pub struct SettlementTransactionCondition28Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition28ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle2Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
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
pub struct Counterparty16ChoiceEnum {
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<PartyIdentificationAndAccount206>,
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<PartyIdentificationAndAccount206>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Counterparty16Choice {
    #[serde(flatten)]
    pub value: Counterparty16ChoiceEnum,
}
#[derive(
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
pub struct QuantityAndAccount111 {
    #[serde(rename = "DnmtnChc", skip_serializing_if = "Option::is_none")]
    pub dnmtn_chc: Option<RestrictedFinxMax210Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafeKeepingPlace4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "QtyBrkdwn", default)]
    pub qty_brkdwn: Vec<QuantityBreakdown69>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdateType38ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Deltn", skip_serializing_if = "Option::is_none")]
    pub deltn: Option<SecuritiesSettlementTransactionDetails54<A>>,
    #[serde(rename = "Mod", skip_serializing_if = "Option::is_none")]
    pub r#mod: Option<SecuritiesSettlementTransactionDetails55<B>>,
    #[serde(rename = "Addtn", skip_serializing_if = "Option::is_none")]
    pub addtn: Option<SecuritiesSettlementTransactionDetails53<C>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdateType38Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: UpdateType38ChoiceEnum<A, B, C>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tracking5ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
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
pub struct Tracking5Choice {
    #[serde(flatten)]
    pub value: Tracking5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max3Number {
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
pub struct SettlementTypeAndAdditionalParameters17 {
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "RcncltnInd", skip_serializing_if = "Option::is_none")]
    pub rcncltn_ind: Option<YesNoIndicator>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesCertificate5 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: RestrictedFinxMax30Text,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max4AlphaNumericText>,
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
pub struct NameAndAddress12 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: RestrictedFinxMax140Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType44ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType44Choice {
    #[serde(flatten)]
    pub value: IdentificationType44ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification85 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification47,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<RestrictedFinxMax30Text>,
}
