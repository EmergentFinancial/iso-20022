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
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_30_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_350_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,350}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINZ_MAX_2048_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z!"%&\*;<> \.,\(\)\n\r/='\+:\?@#\{\-_]{1,2048}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_EXACT_2_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"XX|TS"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_UPPER_CASE_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_16_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINZ_MAX_8000_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z!"%&\*;<> \.,\(\)\n\r/='\+:\?@#\{\-_]{1,8000}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_140_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,140}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_35_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_2_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,34}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_31_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,31}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.031.002.12".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConsentTypeFormat5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConsentType1Code>,
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
pub struct ConsentTypeFormat5Choice {
    #[serde(flatten)]
    pub value: ConsentTypeFormat5ChoiceEnum,
}
#[derive(
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
pub enum RateStatus1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "INDI")]
    Indi,
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
pub struct RateTypeAndAmountAndStatus62 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType83Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndQuantityRatio5 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: RestrictedFinDecimalNumber,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPrice4 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType2Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: RestrictedFinActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat43ChoiceEnum {
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus60>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus2>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat43Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat43ChoiceEnum,
}
#[derive(
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
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Quantity4Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "ANYA")]
    Anya,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Quantity5Code {
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
pub struct AccountIdentification52ChoiceEnum {
    #[serde(
        rename = "AcctsListAndBalDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub accts_list_and_bal_dtls: Option<AccountAndBalance53>,
    #[serde(rename = "ForAllAccts", skip_serializing_if = "Option::is_none")]
    pub for_all_accts: Option<AccountIdentification10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification52Choice {
    #[serde(flatten)]
    pub value: AccountIdentification52ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat37ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
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
pub struct RateAndAmountFormat37Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat37ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProprietaryQuantity9 {
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: RestrictedFinDecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType7Code {
    #[serde(rename = "SCHD")]
    Schd,
    #[serde(rename = "USCD")]
    Uscd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RenounceableStatus1Code {
    #[serde(rename = "NREN")]
    Nren,
    #[serde(rename = "RENO")]
    Reno,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionFeatures11Code {
    #[serde(rename = "ASVO")]
    Asvo,
    #[serde(rename = "BOIS")]
    Bois,
    #[serde(rename = "COND")]
    Cond,
    #[serde(rename = "MAXC")]
    Maxc,
    #[serde(rename = "MAXS")]
    Maxs,
    #[serde(rename = "NOSE")]
    Nose,
    #[serde(rename = "OPLF")]
    Oplf,
    #[serde(rename = "CAOS")]
    Caos,
    #[serde(rename = "PINS")]
    Pins,
    #[serde(rename = "PROR")]
    Pror,
    #[serde(rename = "VVPR")]
    Vvpr,
    #[serde(rename = "QCAS")]
    Qcas,
    #[serde(rename = "SHAR")]
    Shar,
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
pub struct DateFormat45ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "NotSpcfdDt", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_dt: Option<DateType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat45Choice {
    #[serde(flatten)]
    pub value: DateFormat45ChoiceEnum,
}
#[derive(
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
pub struct ProcessingPosition10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingPosition3Code>,
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
pub struct ProcessingPosition10Choice {
    #[serde(flatten)]
    pub value: ProcessingPosition10ChoiceEnum,
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
pub struct DateCode26ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType7Code>,
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
pub struct DateCode26Choice {
    #[serde(flatten)]
    pub value: DateCode26ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption46ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionOption15Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption46Choice {
    #[serde(flatten)]
    pub value: CorporateActionOption46ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BeneficiaryCertificationType4Code {
    #[serde(rename = "ACCI")]
    Acci,
    #[serde(rename = "DOMI")]
    Domi,
    #[serde(rename = "NDOM")]
    Ndom,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NCOM")]
    Ncom,
    #[serde(rename = "QIBB")]
    Qibb,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "PAPW")]
    Papw,
    #[serde(rename = "PABD")]
    Pabd,
    #[serde(rename = "FRAC")]
    Frac,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ElectionMovementType2Code {
    #[serde(rename = "DRCT")]
    Drct,
    #[serde(rename = "SEQD")]
    Seqd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BidRangeType1Code {
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "MULT")]
    Mult,
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
pub struct AmountPrice5 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: RestrictedFinActiveCurrencyAnd13DecimalAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionChangeType1Code {
    #[serde(rename = "BERE")]
    Bere,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "DEPH")]
    Deph,
    #[serde(rename = "GPPH")]
    Gpph,
    #[serde(rename = "GTGP")]
    Gtgp,
    #[serde(rename = "GTPH")]
    Gtph,
    #[serde(rename = "NAME")]
    Name,
    #[serde(rename = "PHDE")]
    Phde,
    #[serde(rename = "REBE")]
    Rebe,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "DECI")]
    Deci,
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
pub struct CorporateActionEventStatus1 {
    #[serde(rename = "EvtCmpltnsSts")]
    pub evt_cmpltns_sts: EventCompletenessStatus1Code,
    #[serde(rename = "EvtConfSts")]
    pub evt_conf_sts: EventConfirmationStatus1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Payment2Code {
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
pub struct RestrictedFinActiveCurrencyAnd13DecimalAmountSimpleType {
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
pub struct DateFormat54ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCdAndTm", skip_serializing_if = "Option::is_none")]
    pub dt_cd_and_tm: Option<DateCodeAndTimeFormat4>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode22Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat54Choice {
    #[serde(flatten)]
    pub value: DateFormat54ChoiceEnum,
}
#[derive(
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
pub struct SafekeepingPlaceFormat32ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification85>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat32Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat32ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CertificationFormatType1Code {
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
pub struct LotteryTypeFormat5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LotteryType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LotteryTypeFormat5Choice {
    #[serde(flatten)]
    pub value: LotteryTypeFormat5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonEligibleProceedsIndicator4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NonEligibleProceedsIndicator1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonEligibleProceedsIndicator4Choice {
    #[serde(flatten)]
    pub value: NonEligibleProceedsIndicator4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionAvailabilityStatus4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionAvailabilityStatus1Code>,
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
pub struct OptionAvailabilityStatus4Choice {
    #[serde(flatten)]
    pub value: OptionAvailabilityStatus4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceRateType3Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "YIEL")]
    Yiel,
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
pub struct FinancialInstrumentQuantity44ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Quantity4Code>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity44Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity44ChoiceEnum,
}
#[derive(
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
pub struct SignedQuantityFormat13 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity36Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ShortLong1Code {
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "LONG")]
    Long,
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
pub struct CorporateActionNarrative48 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlTxt", default)]
    pub addtl_txt: Vec<UpdatedAdditionalInformation14>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrrtvVrsn", default)]
    pub nrrtv_vrsn: Vec<UpdatedAdditionalInformation14>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfConds", default)]
    pub inf_conds: Vec<UpdatedAdditionalInformation15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfToCmplyWth", default)]
    pub inf_to_cmply_wth: Vec<UpdatedAdditionalInformation15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyRstrctn", default)]
    pub scty_rstrctn: Vec<UpdatedAdditionalInformation15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxtnConds", default)]
    pub taxtn_conds: Vec<UpdatedAdditionalInformation15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dsclmr", default)]
    pub dsclmr: Vec<UpdatedAdditionalInformation15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwn", default)]
    pub certfctn_brkdwn: Vec<UpdatedAdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPricePerFinancialInstrumentQuantity11 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "FinInstrmQty")]
    pub fin_instrm_qty: FinancialInstrumentQuantity36Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat16ChoiceEnum {
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat13>,
    #[serde(rename = "FullPrdUnits", skip_serializing_if = "Option::is_none")]
    pub full_prd_units: Option<SignedQuantityFormat13>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat12>,
    #[serde(rename = "PartWayPrdUnits", skip_serializing_if = "Option::is_none")]
    pub part_way_prd_units: Option<SignedQuantityFormat13>,
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat16Choice {
    #[serde(flatten)]
    pub value: BalanceFormat16ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNotification002V12<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Pgntn", skip_serializing_if = "Option::is_none")]
    pub pgntn: Option<Pagination1>,
    #[validate]
    #[serde(rename = "NtfctnGnlInf")]
    pub ntfctn_gnl_inf: CorporateActionNotification6,
    #[serde(rename = "PrvsNtfctnId", skip_serializing_if = "Option::is_none")]
    pub prvs_ntfctn_id: Option<DocumentIdentification37>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<DocumentIdentification17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrDocId", default)]
    pub othr_doc_id: Vec<DocumentIdentification38>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtsLkg", default)]
    pub evts_lkg: Vec<CorporateActionEventReference4>,
    #[validate]
    #[serde(rename = "CorpActnGnlInf")]
    pub corp_actn_gnl_inf: CorporateActionGeneralInformation170,
    #[serde(rename = "AcctDtls")]
    pub acct_dtls: AccountIdentification52Choice,
    #[serde(rename = "IntrmdtScty", skip_serializing_if = "Option::is_none")]
    pub intrmdt_scty: Option<FinancialInstrumentAttributes117>,
    #[serde(rename = "CorpActnDtls", skip_serializing_if = "Option::is_none")]
    pub corp_actn_dtls: Option<CorporateAction67>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpActnOptnDtls", default)]
    pub corp_actn_optn_dtls: Vec<CorporateActionOption208>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative55>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IssrAgt", default)]
    pub issr_agt: Vec<PartyIdentification151Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PngAgt", default)]
    pub png_agt: Vec<PartyIdentification137Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SubPngAgt", default)]
    pub sub_png_agt: Vec<PartyIdentification137Choice>,
    #[serde(rename = "Regar", skip_serializing_if = "Option::is_none")]
    pub regar: Option<PartyIdentification137Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RsellngAgt", default)]
    pub rsellng_agt: Vec<PartyIdentification137Choice>,
    #[serde(rename = "PhysSctiesAgt", skip_serializing_if = "Option::is_none")]
    pub phys_scties_agt: Option<PartyIdentification137Choice>,
    #[serde(rename = "DrpAgt", skip_serializing_if = "Option::is_none")]
    pub drp_agt: Option<PartyIdentification137Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SlctnAgt", default)]
    pub slctn_agt: Vec<PartyIdentification137Choice>,
    #[serde(rename = "InfAgt", skip_serializing_if = "Option::is_none")]
    pub inf_agt: Option<PartyIdentification137Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification151Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Offerr", default)]
    pub offerr: Vec<PartyIdentification151Choice>,
    #[serde(rename = "TrfAgt", skip_serializing_if = "Option::is_none")]
    pub trf_agt: Option<PartyIdentification151Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionFrequencyType5Code {
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "REGR")]
    Regr,
    #[serde(rename = "SPEC")]
    Spec,
    #[serde(rename = "SPRE")]
    Spre,
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
pub struct FinancialInstrumentAttributes114 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification20,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification4Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat5Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType33Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle9Choice>,
    #[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
    pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cpn_dt: Option<IsoDate>,
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
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<RateFormat3Choice>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<RateFormat3Choice>,
    #[serde(rename = "PctgOfDebtClm", skip_serializing_if = "Option::is_none")]
    pub pctg_of_debt_clm: Option<RateFormat3Choice>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<RateFormat12Choice>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<RateFormat12Choice>,
    #[serde(rename = "WarrtParity", skip_serializing_if = "Option::is_none")]
    pub warrt_parity: Option<QuantityToQuantityRatio2>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity36Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode22ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode22Choice {
    #[serde(flatten)]
    pub value: DateCode22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity43ChoiceEnum {
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Quantity5Code>,
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
pub struct FinancialInstrumentQuantity43Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity43ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationFormat4ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification86>,
    #[serde(rename = "ShrtId", skip_serializing_if = "Option::is_none")]
    pub shrt_id: Option<Exact3UpperCaseAlphaNumericText>,
    #[serde(rename = "LngId", skip_serializing_if = "Option::is_none")]
    pub lng_id: Option<RestrictedFinxMax30Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationFormat4Choice {
    #[serde(flatten)]
    pub value: IdentificationFormat4ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetDividendRateType7Code {
    #[serde(rename = "CAPO")]
    Capo,
    #[serde(rename = "CDFI")]
    Cdfi,
    #[serde(rename = "FUPU")]
    Fupu,
    #[serde(rename = "FLFR")]
    Flfr,
    #[serde(rename = "INCO")]
    Inco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "SOIC")]
    Soic,
    #[serde(rename = "TXBL")]
    Txbl,
    #[serde(rename = "TXDF")]
    Txdf,
    #[serde(rename = "TXFR")]
    Txfr,
    #[serde(rename = "UNFR")]
    Unfr,
    #[serde(rename = "PAPU")]
    Papu,
    #[serde(rename = "REES")]
    Rees,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LotteryType1Code {
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "SUPP")]
    Supp,
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
pub struct DocumentIdentification38 {
    #[serde(rename = "Id")]
    pub id: DocumentIdentification4Choice,
    #[serde(rename = "DocNb", skip_serializing_if = "Option::is_none")]
    pub doc_nb: Option<DocumentNumber6Choice>,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition10Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionFeaturesFormat26ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionFeatures11Code>,
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
pub struct OptionFeaturesFormat26Choice {
    #[serde(flatten)]
    pub value: OptionFeaturesFormat26ChoiceEnum,
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
pub struct CorporateActionProcessingStatus6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStatus1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionProcessingStatus6Choice {
    #[serde(flatten)]
    pub value: CorporateActionProcessingStatus6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms28 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "RsltgAmt", skip_serializing_if = "Option::is_none")]
    pub rsltg_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat58ChoiceEnum {
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice4>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat58Choice {
    #[serde(flatten)]
    pub value: PriceFormat58ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType46ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WithholdingTaxRateType1Code>,
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
pub struct RateType46Choice {
    #[serde(flatten)]
    pub value: RateType46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus59 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType80Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat44ChoiceEnum {
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus2>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus62>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat44Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat44ChoiceEnum,
}
#[derive(
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
pub struct TemporaryFinancialInstrumentIndicator4ChoiceEnum {
    #[serde(rename = "TempInd", skip_serializing_if = "Option::is_none")]
    pub temp_ind: Option<YesNoIndicator>,
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
pub struct TemporaryFinancialInstrumentIndicator4Choice {
    #[serde(flatten)]
    pub value: TemporaryFinancialInstrumentIndicator4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdatedAdditionalInformation14 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: RestrictedFinxMax350Text,
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
pub struct TaxableIncomePerShareCalculatedFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionTaxableIncomePerShareCalculated1Code>,
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
pub struct TaxableIncomePerShareCalculatedFormat4Choice {
    #[serde(flatten)]
    pub value: TaxableIncomePerShareCalculatedFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat49ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode22Choice>,
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
pub struct DateFormat49Choice {
    #[serde(flatten)]
    pub value: DateFormat49ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionPrice81 {
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat57Choice>,
    #[serde(rename = "OverSbcptDpstPric", skip_serializing_if = "Option::is_none")]
    pub over_sbcpt_dpst_pric: Option<PriceFormat57Choice>,
    #[serde(rename = "MaxCshToInst", skip_serializing_if = "Option::is_none")]
    pub max_csh_to_inst: Option<PriceFormat62Choice>,
    #[serde(rename = "MinCshToInst", skip_serializing_if = "Option::is_none")]
    pub min_csh_to_inst: Option<PriceFormat62Choice>,
    #[serde(rename = "MinMltplCshToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_csh_to_inst: Option<PriceFormat62Choice>,
    #[serde(rename = "MaxPric", skip_serializing_if = "Option::is_none")]
    pub max_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "MinPric", skip_serializing_if = "Option::is_none")]
    pub min_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "FrstBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub frst_bid_incrmt_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "LastBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub last_bid_incrmt_pric: Option<PriceFormat59Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification4ChoiceEnum {
    #[serde(rename = "AcctOwnrDocId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_doc_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AcctSvcrDocId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_doc_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification4Choice {
    #[serde(flatten)]
    pub value: DocumentIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IntermediateSecuritiesDistributionTypeFormat18ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<IntermediateSecurityDistributionType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IntermediateSecuritiesDistributionTypeFormat18Choice {
    #[serde(flatten)]
    pub value: IntermediateSecuritiesDistributionTypeFormat18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType47ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DividendRateType1Code>,
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
pub struct RateType47Choice {
    #[serde(flatten)]
    pub value: RateType47ChoiceEnum,
}
#[derive(
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventCompletenessStatus1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "INCO")]
    Inco,
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
pub struct Quantity80ChoiceEnum {
    #[serde(rename = "QtyChc", skip_serializing_if = "Option::is_none")]
    pub qty_chc: Option<Quantity57Choice>,
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity80Choice {
    #[serde(flatten)]
    pub value: Quantity80ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentAttributes117 {
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification20,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<RestrictedFinDecimalNumber>,
    #[serde(
        rename = "RnncblEntitlmntStsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub rnncbl_entitlmnt_sts_tp: Option<RenounceableEntitlementStatusTypeFormat4Choice>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType32Choice>,
    #[serde(
        rename = "IntrmdtSctiesToUndrlygRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_to_undrlyg_ratio: Option<QuantityToQuantityRatio2>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<AmountPrice4>,
    #[serde(rename = "XpryDt")]
    pub xpry_dt: DateFormat41Choice,
    #[serde(rename = "PstngDt")]
    pub pstng_dt: DateFormat41Choice,
    #[serde(rename = "TradgPrd", skip_serializing_if = "Option::is_none")]
    pub tradg_prd: Option<Period11>,
    #[serde(rename = "UinstdBal", skip_serializing_if = "Option::is_none")]
    pub uinstd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "InstdBal", skip_serializing_if = "Option::is_none")]
    pub instd_bal: Option<BalanceFormat14Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption208 {
    #[validate]
    #[serde(rename = "OptnNb")]
    pub optn_nb: Exact3NumericText,
    #[serde(rename = "OptnTp")]
    pub optn_tp: CorporateActionOption46Choice,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType31Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferTypeFormat13Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnFeatrs", default)]
    pub optn_featrs: Vec<OptionFeaturesFormat26Choice>,
    #[serde(rename = "OptnAvlbtySts", skip_serializing_if = "Option::is_none")]
    pub optn_avlbty_sts: Option<OptionAvailabilityStatus4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwnTp", default)]
    pub certfctn_brkdwn_tp: Vec<BeneficiaryCertificationType12Choice>,
    #[serde(rename = "BidRgTp", skip_serializing_if = "Option::is_none")]
    pub bid_rg_tp: Option<BidRangeType1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NonDmclCtry", default)]
    pub non_dmcl_ctry: Vec<CountryCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VldDmclCtry", default)]
    pub vld_dmcl_ctry: Vec<CountryCode>,
    #[serde(rename = "CcyOptn", skip_serializing_if = "Option::is_none")]
    pub ccy_optn: Option<ActiveCurrencyCode>,
    #[serde(rename = "DfltPrcgOrStgInstr")]
    pub dflt_prcg_or_stg_instr: DefaultProcessingOrStandingInstruction1Choice,
    #[serde(rename = "ChrgsApldInd", skip_serializing_if = "Option::is_none")]
    pub chrgs_apld_ind: Option<YesNoIndicator>,
    #[serde(rename = "CertfctnBrkdwnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_brkdwn_ind: Option<YesNoIndicator>,
    #[serde(rename = "WdrwlAllwdInd", skip_serializing_if = "Option::is_none")]
    pub wdrwl_allwd_ind: Option<YesNoIndicator>,
    #[serde(rename = "ChngAllwdInd", skip_serializing_if = "Option::is_none")]
    pub chng_allwd_ind: Option<YesNoIndicator>,
    #[serde(rename = "ApldOptnInd", skip_serializing_if = "Option::is_none")]
    pub apld_optn_ind: Option<YesNoIndicator>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification20>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate82>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod12>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate111>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice81>,
    #[serde(rename = "SctiesQty", skip_serializing_if = "Option::is_none")]
    pub scties_qty: Option<SecuritiesOption84>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmntDtls", default)]
    pub scties_mvmnt_dtls: Vec<SecuritiesOption85>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmntDtls", default)]
    pub csh_mvmnt_dtls: Vec<CashOption82>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative48>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityDate18 {
    #[serde(rename = "PmtDt")]
    pub pmt_dt: DateFormat49Choice,
    #[serde(rename = "AvlblDt", skip_serializing_if = "Option::is_none")]
    pub avlbl_dt: Option<DateFormat49Choice>,
    #[serde(rename = "DvddRnkgDt", skip_serializing_if = "Option::is_none")]
    pub dvdd_rnkg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat49Choice>,
    #[serde(rename = "PrpssDt", skip_serializing_if = "Option::is_none")]
    pub prpss_dt: Option<DateFormat49Choice>,
    #[serde(rename = "LastTradgDt", skip_serializing_if = "Option::is_none")]
    pub last_tradg_dt: Option<DateFormat49Choice>,
}
#[derive(
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
pub struct PriceFormat59ChoiceEnum {
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice5>,
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat59Choice {
    #[serde(flatten)]
    pub value: PriceFormat59ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesOption84 {
    #[serde(rename = "MaxQtyToInst", skip_serializing_if = "Option::is_none")]
    pub max_qty_to_inst: Option<FinancialInstrumentQuantity44Choice>,
    #[serde(rename = "MinQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_qty_to_inst: Option<FinancialInstrumentQuantity44Choice>,
    #[serde(rename = "MinMltplQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_qty_to_inst: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "NewBrdLotQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_qty: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "NewDnmtnQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_qty: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "FrntEndOddLotQty", skip_serializing_if = "Option::is_none")]
    pub frnt_end_odd_lot_qty: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "BckEndOddLotQty", skip_serializing_if = "Option::is_none")]
    pub bck_end_odd_lot_qty: Option<FinancialInstrumentQuantity43Choice>,
}
#[derive(
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
pub struct CashOption82 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "CtrctlPmtInd", skip_serializing_if = "Option::is_none")]
    pub ctrctl_pmt_ind: Option<Payment2Code>,
    #[serde(rename = "NonElgblPrcdsInd", skip_serializing_if = "Option::is_none")]
    pub non_elgbl_prcds_ind: Option<NonEligibleProceedsIndicator4Choice>,
    #[serde(
        rename = "IssrOfferrTaxbltyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub issr_offerr_taxblty_ind: Option<IssuerOfferorTaxabilityIndicator1Choice>,
    #[serde(rename = "IncmTp", skip_serializing_if = "Option::is_none")]
    pub incm_tp: Option<GenericIdentification47>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrIncmTp", default)]
    pub othr_incm_tp: Vec<GenericIdentification47>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XmptnTp", default)]
    pub xmptn_tp: Vec<GenericIdentification47>,
    #[serde(rename = "CtryOfIncmSrc", skip_serializing_if = "Option::is_none")]
    pub ctry_of_incm_src: Option<CountryCode>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<CashAccountIdentification6Choice>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<CorporateActionAmounts59>,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: CorporateActionDate72,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms28>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<Rate38>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<PriceDetails29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionQuantity14 {
    #[serde(rename = "MaxQty", skip_serializing_if = "Option::is_none")]
    pub max_qty: Option<FinancialInstrumentQuantity44Choice>,
    #[serde(rename = "MinQtySght", skip_serializing_if = "Option::is_none")]
    pub min_qty_sght: Option<FinancialInstrumentQuantity44Choice>,
    #[serde(rename = "NewBrdLotQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_qty: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "NewDnmtnQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_qty: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "BaseDnmtn", skip_serializing_if = "Option::is_none")]
    pub base_dnmtn: Option<FinancialInstrumentQuantity43Choice>,
    #[serde(rename = "IncrmtlDnmtn", skip_serializing_if = "Option::is_none")]
    pub incrmtl_dnmtn: Option<FinancialInstrumentQuantity43Choice>,
}
#[derive(
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
pub enum DateType9Code {
    #[serde(rename = "PWAL")]
    Pwal,
    #[serde(rename = "MKDT")]
    Mkdt,
    #[serde(rename = "MEET")]
    Meet,
    #[serde(rename = "PAYD")]
    Payd,
    #[serde(rename = "RDTE")]
    Rdte,
    #[serde(rename = "RDDT")]
    Rddt,
    #[serde(rename = "NARR")]
    Narr,
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
pub struct RateFormat12ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<BaseOne14Rate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateFormat12Choice {
    #[serde(flatten)]
    pub value: RateFormat12ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType7Code {
    #[serde(rename = "ONGO")]
    Ongo,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionOption15Code {
    #[serde(rename = "ABST")]
    Abst,
    #[serde(rename = "BSPL")]
    Bspl,
    #[serde(rename = "BUYA")]
    Buya,
    #[serde(rename = "CASE")]
    Case,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CEXC")]
    Cexc,
    #[serde(rename = "CONN")]
    Conn,
    #[serde(rename = "CONY")]
    Cony,
    #[serde(rename = "CTEN")]
    Cten,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "MPUT")]
    Mput,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "NOQU")]
    Noqu,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "QINV")]
    Qinv,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SLLE")]
    Slle,
    #[serde(rename = "PRUN")]
    Prun,
    #[serde(rename = "BOBD")]
    Bobd,
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
pub struct CorporateActionNotification6 {
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: CorporateActionNotificationType1Code,
    #[serde(rename = "PrcgSts")]
    pub prcg_sts: CorporateActionProcessingStatus6Choice,
    #[serde(rename = "ElgblBalInd", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal_ind: Option<YesNoIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FractionDispositionType9Code {
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "RDDN")]
    Rddn,
    #[serde(rename = "STAN")]
    Stan,
    #[serde(rename = "RDUP")]
    Rdup,
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
pub struct PriceFormat70ChoiceEnum {
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice5>,
    #[serde(
        rename = "AmtPricPerFinInstrmQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub amt_pric_per_fin_instrm_qty: Option<AmountPricePerFinancialInstrumentQuantity11>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType8Code>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "AmtPricPerAmt", skip_serializing_if = "Option::is_none")]
    pub amt_pric_per_amt: Option<AmountPricePerAmount3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat70Choice {
    #[serde(flatten)]
    pub value: PriceFormat70ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType81ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NetDividendRateType6Code>,
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
pub struct RateType81Choice {
    #[serde(flatten)]
    pub value: RateType81ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionRate110 {
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<RateAndAmountFormat46Choice>,
    #[serde(rename = "PctgSght", skip_serializing_if = "Option::is_none")]
    pub pctg_sght: Option<RateFormat7Choice>,
    #[serde(rename = "RltdIndx", skip_serializing_if = "Option::is_none")]
    pub rltd_indx: Option<RateFormat3Choice>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<RateFormat3Choice>,
    #[serde(rename = "BidIntrvl", skip_serializing_if = "Option::is_none")]
    pub bid_intrvl: Option<RateAndAmountFormat50Choice>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<RateFormat12Choice>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<RateFormat12Choice>,
    #[serde(
        rename = "RinvstmtDscntRateToMkt",
        skip_serializing_if = "Option::is_none"
    )]
    pub rinvstmt_dscnt_rate_to_mkt: Option<RateFormat3Choice>,
    #[serde(rename = "IntrstShrtfll", skip_serializing_if = "Option::is_none")]
    pub intrst_shrtfll: Option<RateAndAmountFormat43Choice>,
    #[serde(rename = "RealsdLoss", skip_serializing_if = "Option::is_none")]
    pub realsd_loss: Option<RateAndAmountFormat43Choice>,
    #[serde(rename = "DclrdRate", skip_serializing_if = "Option::is_none")]
    pub dclrd_rate: Option<RateAndAmountFormat43Choice>,
    #[serde(rename = "IndxFctr", skip_serializing_if = "Option::is_none")]
    pub indx_fctr: Option<RateAndAmountFormat37Choice>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType8Code {
    #[serde(rename = "TBSP")]
    Tbsp,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "NILP")]
    Nilp,
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
pub struct FractionDispositionType31ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType8Code>,
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
pub struct FractionDispositionType31Choice {
    #[serde(flatten)]
    pub value: FractionDispositionType31ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<RestrictedFinImpliedCurrencyAndAmount>,
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
}
#[derive(
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
pub struct RateTypeAndAmountAndStatus32 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType45Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(
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
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
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
pub struct RenounceableEntitlementStatusTypeFormat4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RenounceableStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RenounceableEntitlementStatusTypeFormat4Choice {
    #[serde(flatten)]
    pub value: RenounceableEntitlementStatusTypeFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdatedAdditionalInformation6 {
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: RestrictedFinxMax350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat41ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode22Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat41Choice {
    #[serde(flatten)]
    pub value: DateFormat41ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeemedRateType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeemedRateType1Code>,
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
pub struct DeemedRateType2Choice {
    #[serde(flatten)]
    pub value: DeemedRateType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinzMax2048Text {
    #[validate(
        length(min = 1, max = 2048,),
        regex = "RESTRICTED_FINZ_MAX_2048_TEXT_REGEX"
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
pub struct DecimalNumber {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IntermediateSecurityDistributionType5Code {
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "MRGR")]
    Mrgr,
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
pub struct SafekeepingPlaceTypeAndText9 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace2Code,
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
pub struct UpdatedAdditionalInformation15 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<RestrictedFinxMax350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPrice80 {
    #[serde(rename = "MaxPric", skip_serializing_if = "Option::is_none")]
    pub max_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "MinPric", skip_serializing_if = "Option::is_none")]
    pub min_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "FrstBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub frst_bid_incrmt_pric: Option<PriceFormat59Choice>,
    #[serde(rename = "LastBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub last_bid_incrmt_pric: Option<PriceFormat59Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionInformationType1Code {
    #[serde(rename = "CONF")]
    Conf,
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
pub struct DateCode33ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType9Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode33Choice {
    #[serde(flatten)]
    pub value: DateCode33ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerOfferorTaxabilityIndicator1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<IssuerTaxability2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerOfferorTaxabilityIndicator1Choice {
    #[serde(flatten)]
    pub value: IssuerOfferorTaxabilityIndicator1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryCertificationType12ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BeneficiaryCertificationType4Code>,
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
pub struct BeneficiaryCertificationType12Choice {
    #[serde(flatten)]
    pub value: BeneficiaryCertificationType12ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionRate95 {
    #[serde(
        rename = "AddtlQtyForSbcbdRsltntScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_sbcbd_rsltnt_scties: Option<RatioFormat23Choice>,
    #[serde(
        rename = "AddtlQtyForExstgScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_exstg_scties: Option<RatioFormat23Choice>,
    #[serde(rename = "NewToOd", skip_serializing_if = "Option::is_none")]
    pub new_to_od: Option<RatioFormat24Choice>,
    #[serde(rename = "TrfrmatnRate", skip_serializing_if = "Option::is_none")]
    pub trfrmatn_rate: Option<PercentageRate>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat46Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<RateFormat3Choice>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<RateFormat3Choice>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat21Choice>,
    #[serde(rename = "FinTxTaxRate", skip_serializing_if = "Option::is_none")]
    pub fin_tx_tax_rate: Option<RateFormat3Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionAvailabilityStatus1Code {
    #[serde(rename = "INTV")]
    Intv,
    #[serde(rename = "CANC")]
    Canc,
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
pub struct DocumentIdentification17 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax16Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode19ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType8Code>,
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
pub struct DateCode19Choice {
    #[serde(flatten)]
    pub value: DateCode19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat62ChoiceEnum {
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice7>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat62Choice {
    #[serde(flatten)]
    pub value: PriceFormat62ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNarrative55 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlTxt", default)]
    pub addtl_txt: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrrtvVrsn", default)]
    pub nrrtv_vrsn: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfConds", default)]
    pub inf_conds: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfToCmplyWth", default)]
    pub inf_to_cmply_wth: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxtnConds", default)]
    pub taxtn_conds: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dsclmr", default)]
    pub dsclmr: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PtyCtctNrrtv", default)]
    pub pty_ctct_nrrtv: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RegnDtls", default)]
    pub regn_dtls: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BsktOrIndxInf", default)]
    pub bskt_or_indx_inf: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwn", default)]
    pub certfctn_brkdwn: Vec<UpdatedAdditionalInformation16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "URLAdr", default)]
    pub url_adr: Vec<UpdatedUrLlnformation5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrcgTxtForNxtIntrmy", default)]
    pub prcg_txt_for_nxt_intrmy: Vec<UpdatedAdditionalInformation16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventProcessingType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventProcessingType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventProcessingType3Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingType3ChoiceEnum,
}
#[derive(
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
pub struct RateStatus4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateStatus4Choice {
    #[serde(flatten)]
    pub value: RateStatus4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType5Code {
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
pub struct RateFormat21ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateFormat21Choice {
    #[serde(flatten)]
    pub value: RateFormat21ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CapitalGainFormat4ChoiceEnum {
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
pub struct CapitalGainFormat4Choice {
    #[serde(flatten)]
    pub value: CapitalGainFormat4ChoiceEnum,
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
pub struct SignedQuantityFormat12 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "QtyChc")]
    pub qty_chc: Quantity53Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPeriod15 {
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<Period6Choice>,
    #[serde(rename = "IntrstPrd", skip_serializing_if = "Option::is_none")]
    pub intrst_prd: Option<Period6Choice>,
    #[serde(rename = "CmplsryPurchsPrd", skip_serializing_if = "Option::is_none")]
    pub cmplsry_purchs_prd: Option<Period6Choice>,
    #[serde(rename = "ClmPrd", skip_serializing_if = "Option::is_none")]
    pub clm_prd: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForBookNtryTrf",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_book_ntry_trf: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForDpstAtAgt",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_dpst_at_agt: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForDpst",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_dpst: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForPldg",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_pldg: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForSgrtn",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_sgrtn: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForWdrwlAtAgt",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_wdrwl_at_agt: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForWdrwlInNmneeNm",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_wdrwl_in_nmnee_nm: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForWdrwlInStrtNm",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_wdrwl_in_strt_nm: Option<Period6Choice>,
    #[serde(rename = "BookClsrPrd", skip_serializing_if = "Option::is_none")]
    pub book_clsr_prd: Option<Period6Choice>,
    #[serde(
        rename = "CoDpstriesSspnsnPrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_dpstries_sspnsn_prd: Option<Period6Choice>,
    #[serde(rename = "SpltPrd", skip_serializing_if = "Option::is_none")]
    pub splt_prd: Option<Period6Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndicativeOrMarketPrice11ChoiceEnum {
    #[serde(rename = "IndctvPric", skip_serializing_if = "Option::is_none")]
    pub indctv_pric: Option<PriceFormat57Choice>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<PriceFormat57Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndicativeOrMarketPrice11Choice {
    #[serde(flatten)]
    pub value: IndicativeOrMarketPrice11ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConsentType1Code {
    #[serde(rename = "CTRM")]
    Ctrm,
    #[serde(rename = "DUPY")]
    Dupy,
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
pub struct UpdatedAdditionalInformation16 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<RestrictedFinzMax8000Text>,
}
#[derive(
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
pub struct RateAndAmountFormat50ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<RestrictedFinDecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat50Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat50ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalAndCurrentQuantities7 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: RestrictedFinImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: RestrictedFinImpliedCurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventProcessingType1Code {
    #[serde(rename = "GENL")]
    Genl,
    #[serde(rename = "DISN")]
    Disn,
    #[serde(rename = "REOR")]
    Reor,
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
pub struct CorporateActionEventReference4 {
    #[serde(rename = "EvtId")]
    pub evt_id: CorporateActionEventReference4Choice,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition10Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IssuerTaxability2Code {
    #[serde(rename = "TXBL")]
    Txbl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetDividendRateType6Code {
    #[serde(rename = "CAPO")]
    Capo,
    #[serde(rename = "CDFI")]
    Cdfi,
    #[serde(rename = "FLFR")]
    Flfr,
    #[serde(rename = "INCO")]
    Inco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "REES")]
    Rees,
    #[serde(rename = "SOIC")]
    Soic,
    #[serde(rename = "TXBL")]
    Txbl,
    #[serde(rename = "TXDF")]
    Txdf,
    #[serde(rename = "TXFR")]
    Txfr,
    #[serde(rename = "UNFR")]
    Unfr,
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
pub struct CorporateActionAmounts59 {
    #[serde(rename = "GrssCshAmt", skip_serializing_if = "Option::is_none")]
    pub grss_csh_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "NetCshAmt", skip_serializing_if = "Option::is_none")]
    pub net_csh_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "SlctnFees", skip_serializing_if = "Option::is_none")]
    pub slctn_fees: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "CshInLieuOfShr", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "CptlGn", skip_serializing_if = "Option::is_none")]
    pub cptl_gn: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "IntrstAmt", skip_serializing_if = "Option::is_none")]
    pub intrst_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "IndmntyAmt", skip_serializing_if = "Option::is_none")]
    pub indmnty_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ManfctrdDvddPmtAmt", skip_serializing_if = "Option::is_none")]
    pub manfctrd_dvdd_pmt_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "RinvstmtAmt", skip_serializing_if = "Option::is_none")]
    pub rinvstmt_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "FullyFrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub fully_frnkd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "UfrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub ufrnkd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "SndryOrOthrAmt", skip_serializing_if = "Option::is_none")]
    pub sndry_or_othr_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TaxFreeAmt", skip_serializing_if = "Option::is_none")]
    pub tax_free_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TaxDfrrdAmt", skip_serializing_if = "Option::is_none")]
    pub tax_dfrrd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ValAddedTaxAmt", skip_serializing_if = "Option::is_none")]
    pub val_added_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "StmpDtyAmt", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TaxRclmAmt", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TaxCdtAmt", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "AddtlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub addtl_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "WhldgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub whldg_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ScndLvlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub scnd_lvl_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "FsclStmpAmt", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "PngAgtComssnAmt", skip_serializing_if = "Option::is_none")]
    pub png_agt_comssn_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "LclBrkrComssnAmt", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "RgltryFeesAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_fees_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ShppgFeesAmt", skip_serializing_if = "Option::is_none")]
    pub shppg_fees_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "ChrgsAmt", skip_serializing_if = "Option::is_none")]
    pub chrgs_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "EntitldAmt", skip_serializing_if = "Option::is_none")]
    pub entitld_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "PrncplOrCrps", skip_serializing_if = "Option::is_none")]
    pub prncpl_or_crps: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "RedPrmAmt", skip_serializing_if = "Option::is_none")]
    pub red_prm_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "IncmPrtn", skip_serializing_if = "Option::is_none")]
    pub incm_prtn: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "StockXchgTax", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "EUTaxRtntnAmt", skip_serializing_if = "Option::is_none")]
    pub eu_tax_rtntn_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "EqulstnAmt", skip_serializing_if = "Option::is_none")]
    pub equlstn_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "FATCATaxAmt", skip_serializing_if = "Option::is_none")]
    pub fatca_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "NRATaxAmt", skip_serializing_if = "Option::is_none")]
    pub nra_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "BckUpWhldgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub bck_up_whldg_tax_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TaxOnIncmAmt", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "TxTax", skip_serializing_if = "Option::is_none")]
    pub tx_tax: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "DmdAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "FrgnIncmAmt", skip_serializing_if = "Option::is_none")]
    pub frgn_incm_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "DmdDvddAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_dvdd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "DmdFndAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_fnd_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "DmdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_intrst_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
    #[serde(rename = "DmdRyltsAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_rylts_amt: Option<RestrictedFinActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EventSequenceTypeFormat2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventSequenceType1Code>,
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
pub struct EventSequenceTypeFormat2Choice {
    #[serde(flatten)]
    pub value: EventSequenceTypeFormat2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FractionDispositionType8Code {
    #[serde(rename = "BUYU")]
    Buyu,
    #[serde(rename = "CINL")]
    Cinl,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "RDDN")]
    Rddn,
    #[serde(rename = "STAN")]
    Stan,
    #[serde(rename = "RDUP")]
    Rdup,
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
pub struct AccountIdentification10 {
    #[serde(rename = "IdCd")]
    pub id_cd: SafekeepingAccountIdentification1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat43ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode19Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat43Choice {
    #[serde(flatten)]
    pub value: DateFormat43ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat42ChoiceEnum {
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus61>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus2>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat42Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat42ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType10Code {
    #[serde(rename = "ANYA")]
    Anya,
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
pub struct FractionDispositionType32ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType9Code>,
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
pub struct FractionDispositionType32Choice {
    #[serde(flatten)]
    pub value: FractionDispositionType32ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate74 {
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<DateFormat49Choice>,
    #[serde(rename = "CertfctnDdln", skip_serializing_if = "Option::is_none")]
    pub certfctn_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "CrtApprvlDt", skip_serializing_if = "Option::is_none")]
    pub crt_apprvl_dt: Option<DateFormat49Choice>,
    #[serde(rename = "EarlyClsgDt", skip_serializing_if = "Option::is_none")]
    pub early_clsg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<DateFormat49Choice>,
    #[serde(rename = "EqulstnDt", skip_serializing_if = "Option::is_none")]
    pub equlstn_dt: Option<DateFormat49Choice>,
    #[serde(rename = "FrthrDtldAnncmntDt", skip_serializing_if = "Option::is_none")]
    pub frthr_dtld_anncmnt_dt: Option<DateFormat49Choice>,
    #[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
    pub fxg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "LtryDt", skip_serializing_if = "Option::is_none")]
    pub ltry_dt: Option<DateFormat49Choice>,
    #[serde(rename = "NewMtrtyDt", skip_serializing_if = "Option::is_none")]
    pub new_mtrty_dt: Option<DateFormat49Choice>,
    #[serde(rename = "MtgDt", skip_serializing_if = "Option::is_none")]
    pub mtg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "MrgnFxgDt", skip_serializing_if = "Option::is_none")]
    pub mrgn_fxg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "PrratnDt", skip_serializing_if = "Option::is_none")]
    pub prratn_dt: Option<DateFormat49Choice>,
    #[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
    pub rcrd_dt: Option<DateFormat49Choice>,
    #[serde(rename = "RegnDdln", skip_serializing_if = "Option::is_none")]
    pub regn_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "RsltsPblctnDt", skip_serializing_if = "Option::is_none")]
    pub rslts_pblctn_dt: Option<DateFormat49Choice>,
    #[serde(rename = "DdlnToSplt", skip_serializing_if = "Option::is_none")]
    pub ddln_to_splt: Option<DateFormat49Choice>,
    #[serde(
        rename = "DdlnForTaxBrkdwnInstr",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddln_for_tax_brkdwn_instr: Option<DateFormat49Choice>,
    #[serde(rename = "TradgSspdDt", skip_serializing_if = "Option::is_none")]
    pub tradg_sspd_dt: Option<DateFormat49Choice>,
    #[serde(rename = "UcondlDt", skip_serializing_if = "Option::is_none")]
    pub ucondl_dt: Option<DateFormat49Choice>,
    #[serde(rename = "WhlyUcondlDt", skip_serializing_if = "Option::is_none")]
    pub whly_ucondl_dt: Option<DateFormat49Choice>,
    #[serde(rename = "ExDvddDt", skip_serializing_if = "Option::is_none")]
    pub ex_dvdd_dt: Option<DateFormat49Choice>,
    #[serde(
        rename = "OffclAnncmntPblctnDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub offcl_anncmnt_pblctn_dt: Option<DateFormat49Choice>,
    #[serde(rename = "SpclExDt", skip_serializing_if = "Option::is_none")]
    pub spcl_ex_dt: Option<DateFormat49Choice>,
    #[serde(rename = "GrntedPrtcptnDt", skip_serializing_if = "Option::is_none")]
    pub grnted_prtcptn_dt: Option<DateFormat49Choice>,
    #[serde(
        rename = "ElctnToCtrPtyMktDdln",
        skip_serializing_if = "Option::is_none"
    )]
    pub elctn_to_ctr_pty_mkt_ddln: Option<DateFormat49Choice>,
    #[serde(
        rename = "ElctnToCtrPtyRspnDdln",
        skip_serializing_if = "Option::is_none"
    )]
    pub elctn_to_ctr_pty_rspn_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "LpsdDt", skip_serializing_if = "Option::is_none")]
    pub lpsd_dt: Option<DateFormat49Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat49Choice>,
    #[serde(rename = "ThrdPtyDdln", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "EarlyThrdPtyDdln", skip_serializing_if = "Option::is_none")]
    pub early_thrd_pty_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "MktClmTrckgEndDt", skip_serializing_if = "Option::is_none")]
    pub mkt_clm_trckg_end_dt: Option<DateFormat49Choice>,
    #[serde(rename = "LeadPlntffDdln", skip_serializing_if = "Option::is_none")]
    pub lead_plntff_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "FilgDt", skip_serializing_if = "Option::is_none")]
    pub filg_dt: Option<DateFormat41Choice>,
    #[serde(rename = "HrgDt", skip_serializing_if = "Option::is_none")]
    pub hrg_dt: Option<DateFormat41Choice>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountPriceType1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PLOT")]
    Plot,
    #[serde(rename = "PREM")]
    Prem,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WithholdingTaxRateType1Code {
    #[serde(rename = "BWIT")]
    Bwit,
    #[serde(rename = "FTCA")]
    Ftca,
    #[serde(rename = "NRAT")]
    Nrat,
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
pub struct DocumentNumber6ChoiceEnum {
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification86>,
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
pub enum NonEligibleProceedsIndicator1Code {
    #[serde(rename = "NELC")]
    Nelc,
    #[serde(rename = "ACLI")]
    Acli,
    #[serde(rename = "ONEL")]
    Onel,
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
pub struct GenericIdentification85 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification47,
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
pub struct DateFormat59ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode33Choice>,
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
pub struct DateFormat59Choice {
    #[serde(flatten)]
    pub value: DateFormat59ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus33 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType47Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventType31Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "ATTI")]
    Atti,
    #[serde(rename = "BRUP")]
    Brup,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "CAPD")]
    Capd,
    #[serde(rename = "CAPG")]
    Capg,
    #[serde(rename = "CAPI")]
    Capi,
    #[serde(rename = "DRCA")]
    Drca,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "CHAN")]
    Chan,
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "CLSA")]
    Clsa,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "CREV")]
    Crev,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "DETI")]
    Deti,
    #[serde(rename = "DSCL")]
    Dscl,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "MCAL")]
    Mcal,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "PPMT")]
    Ppmt,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "RHDI")]
    Rhdi,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "EXTM")]
    Extm,
    #[serde(rename = "MRGR")]
    Mrgr,
    #[serde(rename = "NOOF")]
    Noof,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "ODLT")]
    Odlt,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PARI")]
    Pari,
    #[serde(rename = "PCAL")]
    Pcal,
    #[serde(rename = "PRED")]
    Pred,
    #[serde(rename = "PINK")]
    Pink,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PDEF")]
    Pdef,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "BPUT")]
    Bput,
    #[serde(rename = "REDO")]
    Redo,
    #[serde(rename = "REMK")]
    Remk,
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "SPLR")]
    Splr,
    #[serde(rename = "RHTS")]
    Rhts,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "SHPR")]
    Shpr,
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "TREC")]
    Trec,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "DLST")]
    Dlst,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "EXWA")]
    Exwa,
    #[serde(rename = "WTRC")]
    Wtrc,
    #[serde(rename = "WRTH")]
    Wrth,
    #[serde(rename = "ACCU")]
    Accu,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "TNDP")]
    Tndp,
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
pub struct Quantity57ChoiceEnum {
    #[serde(rename = "SgndQty", skip_serializing_if = "Option::is_none")]
    pub sgnd_qty: Option<SignedQuantityFormat13>,
    #[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity57Choice {
    #[serde(flatten)]
    pub value: Quantity57ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TotalEligibleBalanceFormat11 {
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<Quantity80Choice>,
    #[serde(rename = "FullPrdUnits", skip_serializing_if = "Option::is_none")]
    pub full_prd_units: Option<SignedQuantityFormat13>,
    #[serde(rename = "PartWayPrdUnits", skip_serializing_if = "Option::is_none")]
    pub part_way_prd_units: Option<SignedQuantityFormat13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateFormat7ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateFormat7Choice {
    #[serde(flatten)]
    pub value: RateFormat7ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionNarrative54 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Offerr", default)]
    pub offerr: Vec<UpdatedAdditionalInformation6>,
    #[serde(rename = "NewCpnyNm", skip_serializing_if = "Option::is_none")]
    pub new_cpny_nm: Option<UpdatedAdditionalInformation6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "URLAdr", default)]
    pub url_adr: Vec<UpdatedUrLlnformation5>,
    #[serde(rename = "EvtPrcgWebSiteAdr", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_web_site_adr: Option<RestrictedFinzMax2048Text>,
}
#[derive(
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
pub struct PriceFormat57ChoiceEnum {
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice5>,
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat57Choice {
    #[serde(flatten)]
    pub value: PriceFormat57ChoiceEnum,
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
pub struct ProprietaryQuantity10 {
    #[serde(rename = "ShrtLngPos", skip_serializing_if = "Option::is_none")]
    pub shrt_lng_pos: Option<ShortLong1Code>,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: RestrictedFinDecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
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
pub struct RateFormat3ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateFormat3Choice {
    #[serde(flatten)]
    pub value: RateFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPricePerAmount3 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
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
pub struct CorporateActionMandatoryVoluntary4Choice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingAccountIdentification1Code {
    #[serde(rename = "GENR")]
    Genr,
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
pub struct RateAndAmountFormat46ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat46Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionRate111 {
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat46Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat41Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat43Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat10Choice>,
    #[serde(
        rename = "MaxAllwdOvrsbcptRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_allwd_ovrsbcpt_rate: Option<RateFormat3Choice>,
    #[serde(rename = "PrratnRate", skip_serializing_if = "Option::is_none")]
    pub prratn_rate: Option<RateFormat3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat47Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat47Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxblIncmPerDvddShr", default)]
    pub taxbl_incm_per_dvdd_shr: Vec<RateTypeAndAmountAndStatus33>,
    #[serde(rename = "IssrDclrdXchgRate", skip_serializing_if = "Option::is_none")]
    pub issr_dclrd_xchg_rate: Option<ForeignExchangeTerms19>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateAndAmountFormat46Choice>,
    #[serde(rename = "BidIntrvl", skip_serializing_if = "Option::is_none")]
    pub bid_intrvl: Option<RateAndAmountFormat38Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate82 {
    #[serde(rename = "EarlyRspnDdln", skip_serializing_if = "Option::is_none")]
    pub early_rspn_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "CoverXprtnDdln", skip_serializing_if = "Option::is_none")]
    pub cover_xprtn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "PrtctDdln", skip_serializing_if = "Option::is_none")]
    pub prtct_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat49Choice>,
    #[serde(rename = "RspnDdln", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln: Option<DateFormat54Choice>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<DateFormat49Choice>,
    #[serde(rename = "SbcptCostDbtDt", skip_serializing_if = "Option::is_none")]
    pub sbcpt_cost_dbt_dt: Option<DateFormat49Choice>,
    #[serde(rename = "DpstryCoverXprtnDt", skip_serializing_if = "Option::is_none")]
    pub dpstry_cover_xprtn_dt: Option<DateFormat49Choice>,
    #[serde(rename = "StockLndgDdln", skip_serializing_if = "Option::is_none")]
    pub stock_lndg_ddln: Option<DateFormat49Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrrwrStockLndgDdln", default)]
    pub brrwr_stock_lndg_ddln: Vec<BorrowerLendingDeadline6>,
    #[serde(
        rename = "EndOfSctiesBlckgPrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_of_scties_blckg_prd: Option<DateFormat59Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType13Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "NILP")]
    Nilp,
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
pub struct AdditionalBusinessProcessFormat22ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AdditionalBusinessProcess9Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalBusinessProcessFormat22Choice {
    #[serde(flatten)]
    pub value: AdditionalBusinessProcessFormat22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification37 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax16Text,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition10Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateValueType7Code {
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
pub struct CorporateActionEventType89ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType31Code>,
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
pub struct CorporateActionEventType89Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventType89ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InformationTypeFormat5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionInformationType1Code>,
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
pub struct InformationTypeFormat5Choice {
    #[serde(flatten)]
    pub value: InformationTypeFormat5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat14ChoiceEnum {
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat13>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat12>,
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat14Choice {
    #[serde(flatten)]
    pub value: BalanceFormat14ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus61 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType82Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionMandatoryVoluntary1Code {
    #[serde(rename = "MAND")]
    Mand,
    #[serde(rename = "CHOS")]
    Chos,
    #[serde(rename = "VOLU")]
    Volu,
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
pub struct RateTypeAndAmountAndStatus60 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType81Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesOption85 {
    #[validate]
    #[serde(rename = "SctyDtls")]
    pub scty_dtls: FinancialInstrumentAttributes115,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "TempFinInstrmInd", skip_serializing_if = "Option::is_none")]
    pub temp_fin_instrm_ind: Option<TemporaryFinancialInstrumentIndicator4Choice>,
    #[serde(rename = "NonElgblPrcdsInd", skip_serializing_if = "Option::is_none")]
    pub non_elgbl_prcds_ind: Option<NonEligibleProceedsIndicator4Choice>,
    #[serde(
        rename = "IssrOfferrTaxbltyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub issr_offerr_taxblty_ind: Option<IssuerOfferorTaxabilityIndicator1Choice>,
    #[serde(rename = "NewSctiesIssncInd", skip_serializing_if = "Option::is_none")]
    pub new_scties_issnc_ind: Option<NewSecuritiesIssuanceType5Code>,
    #[serde(rename = "IncmTp", skip_serializing_if = "Option::is_none")]
    pub incm_tp: Option<GenericIdentification47>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrIncmTp", default)]
    pub othr_incm_tp: Vec<GenericIdentification47>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XmptnTp", default)]
    pub xmptn_tp: Vec<GenericIdentification47>,
    #[serde(rename = "EntitldQty", skip_serializing_if = "Option::is_none")]
    pub entitld_qty: Option<Quantity54Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat39Choice>,
    #[serde(rename = "CtryOfIncmSrc", skip_serializing_if = "Option::is_none")]
    pub ctry_of_incm_src: Option<CountryCode>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType31Choice>,
    #[serde(rename = "CcyOptn", skip_serializing_if = "Option::is_none")]
    pub ccy_optn: Option<ActiveCurrencyCode>,
    #[serde(rename = "TradgPrd", skip_serializing_if = "Option::is_none")]
    pub tradg_prd: Option<Period6Choice>,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: SecurityDate18,
    #[serde(rename = "RateDtls", skip_serializing_if = "Option::is_none")]
    pub rate_dtls: Option<CorporateActionRate95>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice79>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionGeneralInformation170 {
    #[validate]
    #[serde(rename = "CorpActnEvtId")]
    pub corp_actn_evt_id: RestrictedFinxMax16Text,
    #[serde(rename = "OffclCorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub offcl_corp_actn_evt_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "ClssActnNb", skip_serializing_if = "Option::is_none")]
    pub clss_actn_nb: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "EvtPrcgTp", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_tp: Option<CorporateActionEventProcessingType3Choice>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType89Choice,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary4Choice,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: FinancialInstrumentAttributes114,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdatedUrLlnformation5 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "URLAdr")]
    pub url_adr: RestrictedFinzMax2048Text,
}
#[derive(
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
pub struct RestrictedFinExact2Text {
    #[validate(regex = "RESTRICTED_FIN_EXACT_2_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionType3Code {
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "ONGO")]
    Ongo,
    #[serde(rename = "ROLL")]
    Roll,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeemedRateType1Code {
    #[serde(rename = "DEDI")]
    Dedi,
    #[serde(rename = "DEFP")]
    Defp,
    #[serde(rename = "DEIT")]
    Deit,
    #[serde(rename = "DERY")]
    Dery,
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
pub struct Iso2ALanguageCode {
    #[validate(regex = "ISO_2_A_LANGUAGE_CODE_REGEX")]
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
pub struct RateAndAmountFormat38ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat38Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat38ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType8Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "ONGO")]
    Ongo,
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
pub struct DateCodeAndTimeFormat4 {
    #[serde(rename = "DtCd")]
    pub dt_cd: DateCode26Choice,
    #[validate]
    #[serde(rename = "Tm")]
    pub tm: IsoTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact3UpperCaseAlphaNumericText {
    #[validate(regex = "EXACT_3_UPPER_CASE_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct RateTypeAndPercentageRate11 {
    #[serde(rename = "RateTp")]
    pub rate_tp: DeemedRateType2Choice,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SolicitationFeeRateFormat10ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio5>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SolicitationFeeRateFormat10Choice {
    #[serde(flatten)]
    pub value: SolicitationFeeRateFormat10ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPrice7 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType3Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: RestrictedFinActiveCurrencyAnd13DecimalAmount,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CorpActnNtfctn")]
    pub corp_actn_ntfctn: CorporateActionNotification002V12<A>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinzMax8000Text {
    #[validate(
        length(min = 1, max = 8000,),
        regex = "RESTRICTED_FINZ_MAX_8000_TEXT_REGEX"
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
pub struct RateType80ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GrossDividendRateType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType80Choice {
    #[serde(flatten)]
    pub value: RateType80ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OfferType4Code {
    #[serde(rename = "SQUE")]
    Sque,
    #[serde(rename = "ERUN")]
    Erun,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "FCFS")]
    Fcfs,
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "NDIS")]
    Ndis,
    #[serde(rename = "DISS")]
    Diss,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountPriceType2Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GrossDividendRateType7Code {
    #[serde(rename = "CAPO")]
    Capo,
    #[serde(rename = "CDFI")]
    Cdfi,
    #[serde(rename = "FUPU")]
    Fupu,
    #[serde(rename = "FLFR")]
    Flfr,
    #[serde(rename = "INCO")]
    Inco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LTCG")]
    Ltcg,
    #[serde(rename = "PAPU")]
    Papu,
    #[serde(rename = "REES")]
    Rees,
    #[serde(rename = "STCG")]
    Stcg,
    #[serde(rename = "SOIC")]
    Soic,
    #[serde(rename = "TXBL")]
    Txbl,
    #[serde(rename = "TXDF")]
    Txdf,
    #[serde(rename = "TXFR")]
    Txfr,
    #[serde(rename = "UNFR")]
    Unfr,
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
pub struct ForeignExchangeTerms19 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
}
#[derive(
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
pub struct RatioFormat24ChoiceEnum {
    #[serde(rename = "QtyToAmt", skip_serializing_if = "Option::is_none")]
    pub qty_to_amt: Option<AmountAndQuantityRatio5>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio2>,
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio3>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat24Choice {
    #[serde(flatten)]
    pub value: RatioFormat24ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat23ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio2>,
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat23Choice {
    #[serde(flatten)]
    pub value: RatioFormat23ChoiceEnum,
}
#[derive(
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
pub struct PercentagePrice1 {
    #[serde(rename = "PctgPricTp")]
    pub pctg_pric_tp: PriceRateType3Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: PercentageRate,
}
#[derive(
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
pub struct CorporateAction67 {
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate74>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod15>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate110>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice80>,
    #[serde(rename = "SctiesQty", skip_serializing_if = "Option::is_none")]
    pub scties_qty: Option<CorporateActionQuantity14>,
    #[serde(rename = "IntrstAcrdNbOfDays", skip_serializing_if = "Option::is_none")]
    pub intrst_acrd_nb_of_days: Option<Max3Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CpnNb", default)]
    pub cpn_nb: Vec<IdentificationFormat4Choice>,
    #[serde(rename = "CertfctnBrkdwnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_brkdwn_ind: Option<YesNoIndicator>,
    #[serde(rename = "ChrgsApldInd", skip_serializing_if = "Option::is_none")]
    pub chrgs_apld_ind: Option<YesNoIndicator>,
    #[serde(rename = "RstrctnInd", skip_serializing_if = "Option::is_none")]
    pub rstrctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "AcrdIntrstInd", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_ind: Option<YesNoIndicator>,
    #[serde(
        rename = "LttrOfGrntedDlvryInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub lttr_of_grnted_dlvry_ind: Option<YesNoIndicator>,
    #[serde(
        rename = "ShrhldrRghtsDrctvInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub shrhldr_rghts_drctv_ind: Option<YesNoIndicator>,
    #[serde(rename = "DvddTp", skip_serializing_if = "Option::is_none")]
    pub dvdd_tp: Option<DividendTypeFormat10Choice>,
    #[serde(rename = "EvtSeqTp", skip_serializing_if = "Option::is_none")]
    pub evt_seq_tp: Option<EventSequenceTypeFormat2Choice>,
    #[serde(rename = "OcrncTp", skip_serializing_if = "Option::is_none")]
    pub ocrnc_tp: Option<DistributionTypeFormat8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferTypeFormat13Choice>,
    #[serde(
        rename = "RnncblEntitlmntStsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub rnncbl_entitlmnt_sts_tp: Option<RenounceableEntitlementStatusTypeFormat4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtStag", default)]
    pub evt_stag: Vec<CorporateActionEventStageFormat20Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlBizPrcInd", default)]
    pub addtl_biz_prc_ind: Vec<AdditionalBusinessProcessFormat22Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ChngTp", default)]
    pub chng_tp: Vec<CorporateActionChangeTypeFormat8Choice>,
    #[serde(
        rename = "IntrmdtSctiesDstrbtnTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_dstrbtn_tp: Option<IntermediateSecuritiesDistributionTypeFormat18Choice>,
    #[serde(rename = "CptlGnInOutInd", skip_serializing_if = "Option::is_none")]
    pub cptl_gn_in_out_ind: Option<CapitalGainFormat4Choice>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculatedFormat4Choice>,
    #[serde(rename = "ElctnTp", skip_serializing_if = "Option::is_none")]
    pub elctn_tp: Option<ElectionTypeFormat4Choice>,
    #[serde(rename = "LtryTp", skip_serializing_if = "Option::is_none")]
    pub ltry_tp: Option<LotteryTypeFormat5Choice>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<CertificationTypeFormat4Choice>,
    #[serde(rename = "CnsntTp", skip_serializing_if = "Option::is_none")]
    pub cnsnt_tp: Option<ConsentTypeFormat5Choice>,
    #[serde(rename = "InfTp", skip_serializing_if = "Option::is_none")]
    pub inf_tp: Option<InformationTypeFormat5Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxOnNonDstrbtdPrcdsInd", default)]
    pub tax_on_non_dstrbtd_prcds_ind: Vec<GenericIdentification47>,
    #[serde(rename = "NewPlcOfIncorprtn", skip_serializing_if = "Option::is_none")]
    pub new_plc_of_incorprtn: Option<RestrictedFinxMax350Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative54>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate72 {
    #[serde(rename = "PmtDt")]
    pub pmt_dt: DateFormat49Choice,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateFormat55Choice>,
    #[serde(rename = "FXRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fx_rate_fxg_dt: Option<DateFormat49Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat49Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DistributionTypeFormat8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DistributionType3Code>,
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
pub struct DistributionTypeFormat8Choice {
    #[serde(flatten)]
    pub value: DistributionTypeFormat8ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GrossDividendRateType6Code {
    #[serde(rename = "CAPO")]
    Capo,
    #[serde(rename = "FLFR")]
    Flfr,
    #[serde(rename = "INCO")]
    Inco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LTCG")]
    Ltcg,
    #[serde(rename = "REES")]
    Rees,
    #[serde(rename = "STCG")]
    Stcg,
    #[serde(rename = "SOIC")]
    Soic,
    #[serde(rename = "TXBL")]
    Txbl,
    #[serde(rename = "TXDF")]
    Txdf,
    #[serde(rename = "TXFR")]
    Txfr,
    #[serde(rename = "UNFR")]
    Unfr,
    #[serde(rename = "CDFI")]
    Cdfi,
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
pub struct GrossDividendRateFormat41ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus2>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus59>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat41Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat41ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndRateStatus2 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts")]
    pub rate_sts: RateStatus1Code,
}
#[derive(
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
pub struct Period6ChoiceEnum {
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Period11>,
    #[serde(rename = "PrdCd", skip_serializing_if = "Option::is_none")]
    pub prd_cd: Option<DateType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period6Choice {
    #[serde(flatten)]
    pub value: Period6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType83ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NetDividendRateType7Code>,
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
pub struct RateType83Choice {
    #[serde(flatten)]
    pub value: RateType83ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period11 {
    #[serde(rename = "StartDt")]
    pub start_dt: DateFormat45Choice,
    #[serde(rename = "EndDt")]
    pub end_dt: DateFormat45Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalBusinessProcess9Code {
    #[serde(rename = "ACLA")]
    Acla,
    #[serde(rename = "ATXF")]
    Atxf,
    #[serde(rename = "CNTR")]
    Cntr,
    #[serde(rename = "NAMC")]
    Namc,
    #[serde(rename = "NPLE")]
    Nple,
    #[serde(rename = "SCHM")]
    Schm,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "PPUT")]
    Pput,
    #[serde(rename = "FPRE")]
    Fpre,
    #[serde(rename = "PPRE")]
    Ppre,
    #[serde(rename = "REAC")]
    Reac,
    #[serde(rename = "INCP")]
    Incp,
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
pub struct BaseOne14Rate {
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
pub struct ElectionTypeFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ElectionMovementType2Code>,
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
pub struct ElectionTypeFormat4Choice {
    #[serde(flatten)]
    pub value: ElectionTypeFormat4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType10Code {
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
pub struct CorporateActionEventStageFormat20ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStage3Code>,
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
pub struct CorporateActionEventStageFormat20Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventStageFormat20ChoiceEnum,
}
#[derive(
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
pub struct FinancialInstrumentAttributes115 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification20,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification4Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat5Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType33Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle9Choice>,
    #[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
    pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cpn_dt: Option<IsoDate>,
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
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<RateFormat12Choice>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<RateFormat12Choice>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<RateFormat3Choice>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<RateFormat3Choice>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "MinQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_qty_to_inst: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "MinMltplQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_qty_to_inst: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity36Choice>,
    #[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
    pub isse_pric: Option<PriceFormat57Choice>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DividendRateType1Code {
    #[serde(rename = "TXBL")]
    Txbl,
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
pub struct InterestComputationMethodFormat5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod2Code>,
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
pub struct RateAndAmountFormat47ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat47Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat47ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText15>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
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
pub struct RateType82ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GrossDividendRateType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType82Choice {
    #[serde(flatten)]
    pub value: RateType82ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "RestrictedFinActiveCurrencyAnd13DecimalAmount")]
    pub value: RestrictedFinActiveCurrencyAnd13DecimalAmountSimpleType,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionNotificationType1Code {
    #[serde(rename = "NEWM")]
    Newm,
    #[serde(rename = "REPL")]
    Repl,
    #[serde(rename = "RMDR")]
    Rmdr,
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct DefaultProcessingOrStandingInstruction1ChoiceEnum {
    #[serde(rename = "StgInstrInd", skip_serializing_if = "Option::is_none")]
    pub stg_instr_ind: Option<YesNoIndicator>,
    #[serde(rename = "DfltOptnInd", skip_serializing_if = "Option::is_none")]
    pub dflt_optn_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DefaultProcessingOrStandingInstruction1Choice {
    #[serde(flatten)]
    pub value: DefaultProcessingOrStandingInstruction1ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct DateFormat55ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode27Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat55Choice {
    #[serde(flatten)]
    pub value: DateFormat55ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionBalanceDetails46 {
    #[serde(rename = "TtlElgblBal", skip_serializing_if = "Option::is_none")]
    pub ttl_elgbl_bal: Option<TotalEligibleBalanceFormat11>,
    #[serde(rename = "BlckdBal", skip_serializing_if = "Option::is_none")]
    pub blckd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "BrrwdBal", skip_serializing_if = "Option::is_none")]
    pub brrwd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "CollInBal", skip_serializing_if = "Option::is_none")]
    pub coll_in_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "CollOutBal", skip_serializing_if = "Option::is_none")]
    pub coll_out_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "OnLnBal", skip_serializing_if = "Option::is_none")]
    pub on_ln_bal: Option<BalanceFormat14Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgDlvryBal", default)]
    pub pdg_dlvry_bal: Vec<BalanceFormat16Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgRctBal", default)]
    pub pdg_rct_bal: Vec<BalanceFormat16Choice>,
    #[serde(rename = "OutForRegnBal", skip_serializing_if = "Option::is_none")]
    pub out_for_regn_bal: Option<BalanceFormat14Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmPosBal", default)]
    pub sttlm_pos_bal: Vec<BalanceFormat16Choice>,
    #[serde(rename = "StrtPosBal", skip_serializing_if = "Option::is_none")]
    pub strt_pos_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "TradDtPosBal", skip_serializing_if = "Option::is_none")]
    pub trad_dt_pos_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "InTrnsShipmntBal", skip_serializing_if = "Option::is_none")]
    pub in_trns_shipmnt_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "RegdBal", skip_serializing_if = "Option::is_none")]
    pub regd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "OblgtdBal", skip_serializing_if = "Option::is_none")]
    pub oblgtd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "UinstdBal", skip_serializing_if = "Option::is_none")]
    pub uinstd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "InstdBal", skip_serializing_if = "Option::is_none")]
    pub instd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "AfctdBal", skip_serializing_if = "Option::is_none")]
    pub afctd_bal: Option<BalanceFormat14Choice>,
    #[serde(rename = "UafctdBal", skip_serializing_if = "Option::is_none")]
    pub uafctd_bal: Option<BalanceFormat14Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificationTypeFormat4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CertificationFormatType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificationTypeFormat4Choice {
    #[serde(flatten)]
    pub value: CertificationTypeFormat4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventConfirmationStatus1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "UCON")]
    Ucon,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventStage3Code {
    #[serde(rename = "APPD")]
    Appd,
    #[serde(rename = "CLDE")]
    Clde,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "PWAL")]
    Pwal,
    #[serde(rename = "RESC")]
    Resc,
    #[serde(rename = "SUAP")]
    Suap,
    #[serde(rename = "UNAC")]
    Unac,
    #[serde(rename = "WHOU")]
    Whou,
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
pub struct BorrowerLendingDeadline6 {
    #[serde(rename = "StockLndgDdln")]
    pub stock_lndg_ddln: DateFormat49Choice,
    #[serde(rename = "Brrwr")]
    pub brrwr: PartyIdentification136Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountToAmountRatio3 {
    #[validate]
    #[serde(rename = "Amt1")]
    pub amt_1: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Amt2")]
    pub amt_2: RestrictedFinActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode27ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType1Code>,
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
pub struct DateCode27Choice {
    #[serde(flatten)]
    pub value: DateCode27ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DividendTypeFormat10ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionFrequencyType5Code>,
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
pub struct DividendTypeFormat10Choice {
    #[serde(flatten)]
    pub value: DividendTypeFormat10ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NewSecuritiesIssuanceType5Code {
    #[serde(rename = "DEFE")]
    Defe,
    #[serde(rename = "EXIS")]
    Exis,
    #[serde(rename = "NEIS")]
    Neis,
    #[serde(rename = "NDEF")]
    Ndef,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "NREF")]
    Nref,
    #[serde(rename = "REFU")]
    Refu,
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
pub struct RateAndAmountFormat43ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat43Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat43ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OfferTypeFormat13ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OfferType4Code>,
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
pub struct OfferTypeFormat13Choice {
    #[serde(flatten)]
    pub value: OfferTypeFormat13ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus54 {
    #[serde(rename = "RateTp")]
    pub rate_tp: DeemedRateType2Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification151ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
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
pub struct PartyIdentification151Choice {
    #[serde(flatten)]
    pub value: PartyIdentification151ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat53ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus54>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat53Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat53ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndPercentageRate9 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType46Choice,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountAndBalance53 {
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<RestrictedFinxMax35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<RestrictedFinxMax140Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification136Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat32Choice>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<CorporateActionBalanceDetails46>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionTaxableIncomePerShareCalculated1Code {
    #[serde(rename = "TDIY")]
    Tdiy,
    #[serde(rename = "TDIN")]
    Tdin,
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
pub struct RateType45ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType45Choice {
    #[serde(flatten)]
    pub value: RateType45ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventReference4ChoiceEnum {
    #[serde(
        rename = "LkdOffclCorpActnEvtId",
        skip_serializing_if = "Option::is_none"
    )]
    pub lkd_offcl_corp_actn_evt_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "LkdCorpActnId", skip_serializing_if = "Option::is_none")]
    pub lkd_corp_actn_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventReference4Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventReference4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceDetails29 {
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat59Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat70Choice>,
}
#[derive(
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
pub struct RateAndAmountFormat48ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat48Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat48ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
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
pub struct QuantityToQuantityRatio2 {
    #[validate]
    #[serde(rename = "Qty1")]
    pub qty_1: RestrictedFinDecimalNumber,
    #[validate]
    #[serde(rename = "Qty2")]
    pub qty_2: RestrictedFinDecimalNumber,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountPriceType3Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "PLOT")]
    Plot,
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
pub struct BidRangeType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BidRangeType1Code>,
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
pub struct BidRangeType1Choice {
    #[serde(flatten)]
    pub value: BidRangeType1ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionChangeTypeFormat8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionChangeType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionChangeTypeFormat8Choice {
    #[serde(flatten)]
    pub value: CorporateActionChangeTypeFormat8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPeriod12 {
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<Period6Choice>,
    #[serde(rename = "ParllTradgPrd", skip_serializing_if = "Option::is_none")]
    pub parll_tradg_prd: Option<Period6Choice>,
    #[serde(rename = "ActnPrd", skip_serializing_if = "Option::is_none")]
    pub actn_prd: Option<Period6Choice>,
    #[serde(rename = "RvcbltyPrd", skip_serializing_if = "Option::is_none")]
    pub rvcblty_prd: Option<Period6Choice>,
    #[serde(rename = "PrvlgSspnsnPrd", skip_serializing_if = "Option::is_none")]
    pub prvlg_sspnsn_prd: Option<Period6Choice>,
    #[serde(rename = "AcctSvcrRvcbltyPrd", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_rvcblty_prd: Option<Period6Choice>,
    #[serde(
        rename = "DpstrySspnsnPrdForWdrwl",
        skip_serializing_if = "Option::is_none"
    )]
    pub dpstry_sspnsn_prd_for_wdrwl: Option<Period6Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateUsedForPaymentFormat10ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus32>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RestrictedFinActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateUsedForPaymentFormat10Choice {
    #[serde(flatten)]
    pub value: InterestRateUsedForPaymentFormat10ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventSequenceType1Code {
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "INTE")]
    Inte,
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
pub struct CorporateActionPrice79 {
    #[serde(rename = "IndctvOrMktPric", skip_serializing_if = "Option::is_none")]
    pub indctv_or_mkt_pric: Option<IndicativeOrMarketPrice11Choice>,
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat57Choice>,
    #[serde(rename = "CshValForTax", skip_serializing_if = "Option::is_none")]
    pub csh_val_for_tax: Option<PriceFormat58Choice>,
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat59Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat70Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity53ChoiceEnum {
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity9>,
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
pub struct Quantity53Choice {
    #[serde(flatten)]
    pub value: Quantity53ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Rate38 {
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat46Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat42Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat10Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat47Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat47Choice>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat46Choice>,
    #[serde(rename = "EarlySlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub early_slctn_fee_rate: Option<SolicitationFeeRateFormat10Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<RateFormat3Choice>,
    #[serde(rename = "ThrdPtyIncntivRate", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_incntiv_rate: Option<RateFormat21Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat44Choice>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<RateFormat3Choice>,
    #[serde(rename = "SlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub slctn_fee_rate: Option<SolicitationFeeRateFormat10Choice>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat21Choice>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateAndAmountFormat46Choice>,
    #[serde(rename = "TaxOnPrfts", skip_serializing_if = "Option::is_none")]
    pub tax_on_prfts: Option<RateFormat3Choice>,
    #[serde(rename = "TaxRclmRate", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_rate: Option<RateFormat3Choice>,
    #[serde(rename = "EqulstnRate", skip_serializing_if = "Option::is_none")]
    pub equlstn_rate: Option<RateAndAmountFormat48Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DmdRate", default)]
    pub dmd_rate: Vec<RateAndAmountFormat53Choice>,
}
#[derive(
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
