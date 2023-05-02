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
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_UPPER_CASE_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.031.001.12".to_string()
}

#[derive(
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
pub struct CorporateActionEventStageFormat13ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStage3Code>,
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
pub struct CorporateActionEventStageFormat13Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventStageFormat13ChoiceEnum,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType26ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType26Choice {
    #[serde(flatten)]
    pub value: FractionDispositionType26ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
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
pub struct IdentificationSource3Choice {
    #[serde(flatten)]
    pub value: IdentificationSource3ChoiceEnum,
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
pub struct CorporateActionNarrative50 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Offerr", default)]
    pub offerr: Vec<UpdatedAdditionalInformation3>,
    #[serde(rename = "NewCpnyNm", skip_serializing_if = "Option::is_none")]
    pub new_cpny_nm: Option<UpdatedAdditionalInformation3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "URLAdr", default)]
    pub url_adr: Vec<UpdatedUrLlnformation4>,
    #[serde(rename = "EvtPrcgWebSiteAdr", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_web_site_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SignedQuantityFormat10 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity33Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxableIncomePerShareCalculatedFormat3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionTaxableIncomePerShareCalculated1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxableIncomePerShareCalculatedFormat3Choice {
    #[serde(flatten)]
    pub value: TaxableIncomePerShareCalculatedFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct InterestComputationMethodFormat4Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethodFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesOption77 {
    #[validate]
    #[serde(rename = "SctyDtls")]
    pub scty_dtls: FinancialInstrumentAttributes107,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "TempFinInstrmInd", skip_serializing_if = "Option::is_none")]
    pub temp_fin_instrm_ind: Option<TemporaryFinancialInstrumentIndicator3Choice>,
    #[serde(rename = "NonElgblPrcdsInd", skip_serializing_if = "Option::is_none")]
    pub non_elgbl_prcds_ind: Option<NonEligibleProceedsIndicator3Choice>,
    #[serde(
        rename = "IssrOfferrTaxbltyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub issr_offerr_taxblty_ind: Option<IssuerOfferorTaxabilityIndicator1Choice>,
    #[serde(rename = "NewSctiesIssncInd", skip_serializing_if = "Option::is_none")]
    pub new_scties_issnc_ind: Option<NewSecuritiesIssuanceType5Code>,
    #[serde(rename = "IncmTp", skip_serializing_if = "Option::is_none")]
    pub incm_tp: Option<GenericIdentification30>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrIncmTp", default)]
    pub othr_incm_tp: Vec<GenericIdentification30>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XmptnTp", default)]
    pub xmptn_tp: Vec<GenericIdentification30>,
    #[serde(rename = "EntitldQty", skip_serializing_if = "Option::is_none")]
    pub entitld_qty: Option<Quantity51Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat29Choice>,
    #[serde(rename = "CtryOfIncmSrc", skip_serializing_if = "Option::is_none")]
    pub ctry_of_incm_src: Option<CountryCode>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType26Choice>,
    #[serde(rename = "CcyOptn", skip_serializing_if = "Option::is_none")]
    pub ccy_optn: Option<ActiveCurrencyCode>,
    #[serde(rename = "TradgPrd", skip_serializing_if = "Option::is_none")]
    pub tradg_prd: Option<Period6Choice>,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: SecurityDate16,
    #[serde(rename = "RateDtls", skip_serializing_if = "Option::is_none")]
    pub rate_dtls: Option<CorporateActionRate89>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice75>,
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
pub struct RateFormat3ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType5Code>,
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
pub struct Quantity48ChoiceEnum {
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity8>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity33Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity48Choice {
    #[serde(flatten)]
    pub value: Quantity48ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat61ChoiceEnum {
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat61Choice {
    #[serde(flatten)]
    pub value: PriceFormat61ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionRate105 {
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<RateAndAmountFormat37Choice>,
    #[serde(rename = "PctgSght", skip_serializing_if = "Option::is_none")]
    pub pctg_sght: Option<RateFormat7Choice>,
    #[serde(rename = "RltdIndx", skip_serializing_if = "Option::is_none")]
    pub rltd_indx: Option<RateFormat3Choice>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<RateFormat3Choice>,
    #[serde(rename = "BidIntrvl", skip_serializing_if = "Option::is_none")]
    pub bid_intrvl: Option<RateAndAmountFormat38Choice>,
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
    pub intrst_shrtfll: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "RealsdLoss", skip_serializing_if = "Option::is_none")]
    pub realsd_loss: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "DclrdRate", skip_serializing_if = "Option::is_none")]
    pub dclrd_rate: Option<RateAndAmountFormat39Choice>,
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
pub struct CashOption77 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "CtrctlPmtInd", skip_serializing_if = "Option::is_none")]
    pub ctrctl_pmt_ind: Option<Payment2Code>,
    #[serde(rename = "NonElgblPrcdsInd", skip_serializing_if = "Option::is_none")]
    pub non_elgbl_prcds_ind: Option<NonEligibleProceedsIndicator3Choice>,
    #[serde(
        rename = "IssrOfferrTaxbltyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub issr_offerr_taxblty_ind: Option<IssuerOfferorTaxabilityIndicator1Choice>,
    #[serde(rename = "IncmTp", skip_serializing_if = "Option::is_none")]
    pub incm_tp: Option<GenericIdentification30>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrIncmTp", default)]
    pub othr_incm_tp: Vec<GenericIdentification30>,
    #[validate(length(min = 0,))]
    #[serde(rename = "XmptnTp", default)]
    pub xmptn_tp: Vec<GenericIdentification30>,
    #[serde(rename = "CtryOfIncmSrc", skip_serializing_if = "Option::is_none")]
    pub ctry_of_incm_src: Option<CountryCode>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<CorporateActionAmounts54>,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: CorporateActionDate62,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<ForeignExchangeTerms24>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<Rate36>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<PriceDetails27>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OriginalAndCurrentQuantities1 {
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionProcessingStatus5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct CorporateActionProcessingStatus5Choice {
    #[serde(flatten)]
    pub value: CorporateActionProcessingStatus5ChoiceEnum,
}
#[derive(
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
pub enum RateType7Code {
    #[serde(rename = "SCHD")]
    Schd,
    #[serde(rename = "USCD")]
    Uscd,
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
pub struct Max8000Text {
    #[validate(length(min = 1, max = 8000,))]
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
pub struct CorporateAction60 {
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate61>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod15>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate105>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice72>,
    #[serde(rename = "SctiesQty", skip_serializing_if = "Option::is_none")]
    pub scties_qty: Option<CorporateActionQuantity11>,
    #[serde(rename = "IntrstAcrdNbOfDays", skip_serializing_if = "Option::is_none")]
    pub intrst_acrd_nb_of_days: Option<Max3Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CpnNb", default)]
    pub cpn_nb: Vec<IdentificationFormat3Choice>,
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
    pub dvdd_tp: Option<DividendTypeFormat9Choice>,
    #[serde(rename = "EvtSeqTp", skip_serializing_if = "Option::is_none")]
    pub evt_seq_tp: Option<EventSequenceTypeFormat1Choice>,
    #[serde(rename = "OcrncTp", skip_serializing_if = "Option::is_none")]
    pub ocrnc_tp: Option<DistributionTypeFormat7Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferTypeFormat12Choice>,
    #[serde(
        rename = "RnncblEntitlmntStsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub rnncbl_entitlmnt_sts_tp: Option<RenounceableEntitlementStatusTypeFormat3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtStag", default)]
    pub evt_stag: Vec<CorporateActionEventStageFormat13Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlBizPrcInd", default)]
    pub addtl_biz_prc_ind: Vec<AdditionalBusinessProcessFormat17Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ChngTp", default)]
    pub chng_tp: Vec<CorporateActionChangeTypeFormat5Choice>,
    #[serde(
        rename = "IntrmdtSctiesDstrbtnTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_dstrbtn_tp: Option<IntermediateSecuritiesDistributionTypeFormat15Choice>,
    #[serde(rename = "CptlGnInOutInd", skip_serializing_if = "Option::is_none")]
    pub cptl_gn_in_out_ind: Option<CapitalGainFormat3Choice>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculatedFormat3Choice>,
    #[serde(rename = "ElctnTp", skip_serializing_if = "Option::is_none")]
    pub elctn_tp: Option<ElectionTypeFormat3Choice>,
    #[serde(rename = "LtryTp", skip_serializing_if = "Option::is_none")]
    pub ltry_tp: Option<LotteryTypeFormat4Choice>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<CertificationTypeFormat3Choice>,
    #[serde(rename = "CnsntTp", skip_serializing_if = "Option::is_none")]
    pub cnsnt_tp: Option<ConsentTypeFormat4Choice>,
    #[serde(rename = "InfTp", skip_serializing_if = "Option::is_none")]
    pub inf_tp: Option<InformationTypeFormat4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxOnNonDstrbtdPrcdsInd", default)]
    pub tax_on_non_dstrbtd_prcds_ind: Vec<GenericIdentification30>,
    #[serde(rename = "NewPlcOfIncorprtn", skip_serializing_if = "Option::is_none")]
    pub new_plc_of_incorprtn: Option<Max350Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative50>,
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
pub struct PriceFormat65ChoiceEnum {
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
    #[serde(
        rename = "AmtPricPerFinInstrmQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub amt_pric_per_fin_instrm_qty: Option<AmountPricePerFinancialInstrumentQuantity10>,
    #[serde(rename = "AmtPricPerAmt", skip_serializing_if = "Option::is_none")]
    pub amt_pric_per_amt: Option<AmountPricePerAmount2>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<DecimalNumber>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat65Choice {
    #[serde(flatten)]
    pub value: PriceFormat65ChoiceEnum,
}
#[derive(
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
pub struct Iso2ALanguageCode {
    #[validate(regex = "ISO_2_A_LANGUAGE_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct AmountToAmountRatio2 {
    #[validate]
    #[serde(rename = "Amt1")]
    pub amt_1: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Amt2")]
    pub amt_2: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionInformationType1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[default]
    Unknown,
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
pub struct CorporateActionEventStatus1 {
    #[serde(rename = "EvtCmpltnsSts")]
    pub evt_cmpltns_sts: EventCompletenessStatus1Code,
    #[serde(rename = "EvtConfSts")]
    pub evt_conf_sts: EventConfirmationStatus1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IssuerTaxability2Code {
    #[serde(rename = "TXBL")]
    Txbl,
    #[default]
    Unknown,
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
#[derive(
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
pub struct BidRangeType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BidRangeType1Code>,
}
#[derive(
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
pub struct CorporateActionOption193 {
    #[validate]
    #[serde(rename = "OptnNb")]
    pub optn_nb: Exact3NumericText,
    #[serde(rename = "OptnTp")]
    pub optn_tp: CorporateActionOption37Choice,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType26Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferTypeFormat12Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnFeatrs", default)]
    pub optn_featrs: Vec<OptionFeaturesFormat24Choice>,
    #[serde(rename = "OptnAvlbtySts", skip_serializing_if = "Option::is_none")]
    pub optn_avlbty_sts: Option<OptionAvailabilityStatus3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwnTp", default)]
    pub certfctn_brkdwn_tp: Vec<BeneficiaryCertificationType9Choice>,
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
    pub fin_instrm_id: Option<SecurityIdentification19>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate77>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod12>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate104>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice73>,
    #[serde(rename = "SctiesQty", skip_serializing_if = "Option::is_none")]
    pub scties_qty: Option<SecuritiesOption81>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmntDtls", default)]
    pub scties_mvmnt_dtls: Vec<SecuritiesOption77>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmntDtls", default)]
    pub csh_mvmnt_dtls: Vec<CashOption77>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative45>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionFeaturesFormat24ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionFeatures11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionFeaturesFormat24Choice {
    #[serde(flatten)]
    pub value: OptionFeaturesFormat24ChoiceEnum,
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
pub struct SafekeepingPlaceFormat28ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText6>,
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
pub struct SafekeepingPlaceFormat28Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat28ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption37ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct CorporateActionOption37Choice {
    #[serde(flatten)]
    pub value: CorporateActionOption37ChoiceEnum,
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
pub struct CorporateActionRate89 {
    #[serde(
        rename = "AddtlQtyForSbcbdRsltntScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_sbcbd_rsltnt_scties: Option<RatioFormat17Choice>,
    #[serde(
        rename = "AddtlQtyForExstgScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_exstg_scties: Option<RatioFormat17Choice>,
    #[serde(rename = "NewToOd", skip_serializing_if = "Option::is_none")]
    pub new_to_od: Option<RatioFormat18Choice>,
    #[serde(rename = "TrfrmatnRate", skip_serializing_if = "Option::is_none")]
    pub trfrmatn_rate: Option<PercentageRate>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat37Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<RateFormat3Choice>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<RateFormat3Choice>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat20Choice>,
    #[serde(rename = "FinTxTaxRate", skip_serializing_if = "Option::is_none")]
    pub fin_tx_tax_rate: Option<RateFormat3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DistributionTypeFormat7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DistributionType3Code>,
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
pub struct DistributionTypeFormat7Choice {
    #[serde(flatten)]
    pub value: DistributionTypeFormat7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ElectionTypeFormat3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ElectionMovementType2Code>,
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
pub struct ElectionTypeFormat3Choice {
    #[serde(flatten)]
    pub value: ElectionTypeFormat3ChoiceEnum,
}
#[derive(
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
pub struct Period11 {
    #[serde(rename = "StartDt")]
    pub start_dt: DateFormat45Choice,
    #[serde(rename = "EndDt")]
    pub end_dt: DateFormat45Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndRateStatus1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
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
pub struct CorporateActionBalanceDetails43 {
    #[serde(rename = "TtlElgblBal", skip_serializing_if = "Option::is_none")]
    pub ttl_elgbl_bal: Option<TotalEligibleBalanceFormat10>,
    #[serde(rename = "BlckdBal", skip_serializing_if = "Option::is_none")]
    pub blckd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "BrrwdBal", skip_serializing_if = "Option::is_none")]
    pub brrwd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "CollInBal", skip_serializing_if = "Option::is_none")]
    pub coll_in_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "CollOutBal", skip_serializing_if = "Option::is_none")]
    pub coll_out_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "OnLnBal", skip_serializing_if = "Option::is_none")]
    pub on_ln_bal: Option<BalanceFormat11Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgDlvryBal", default)]
    pub pdg_dlvry_bal: Vec<BalanceFormat12Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgRctBal", default)]
    pub pdg_rct_bal: Vec<BalanceFormat12Choice>,
    #[serde(rename = "OutForRegnBal", skip_serializing_if = "Option::is_none")]
    pub out_for_regn_bal: Option<BalanceFormat11Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmPosBal", default)]
    pub sttlm_pos_bal: Vec<BalanceFormat12Choice>,
    #[serde(rename = "StrtPosBal", skip_serializing_if = "Option::is_none")]
    pub strt_pos_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "TradDtPosBal", skip_serializing_if = "Option::is_none")]
    pub trad_dt_pos_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "InTrnsShipmntBal", skip_serializing_if = "Option::is_none")]
    pub in_trns_shipmnt_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "RegdBal", skip_serializing_if = "Option::is_none")]
    pub regd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "OblgtdBal", skip_serializing_if = "Option::is_none")]
    pub oblgtd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "UinstdBal", skip_serializing_if = "Option::is_none")]
    pub uinstd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "InstdBal", skip_serializing_if = "Option::is_none")]
    pub instd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "AfctdBal", skip_serializing_if = "Option::is_none")]
    pub afctd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "UafctdBal", skip_serializing_if = "Option::is_none")]
    pub uafctd_bal: Option<BalanceFormat11Choice>,
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
pub struct PriceFormat45ChoiceEnum {
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat45Choice {
    #[serde(flatten)]
    pub value: PriceFormat45ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPricePerAmount2 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCodeAndTimeFormat3 {
    #[serde(rename = "DtCd")]
    pub dt_cd: DateCode21Choice,
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
pub struct ActiveCurrencyCode {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
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
pub struct RateAndAmountFormat39ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat39Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat39ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CapitalGainFormat3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EuCapitalGain2Code>,
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
pub struct CapitalGainFormat3Choice {
    #[serde(flatten)]
    pub value: CapitalGainFormat3ChoiceEnum,
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
pub struct RateAndAmountFormat51ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus37>,
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate10>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat51Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat51ChoiceEnum,
}
#[derive(
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
pub struct FinancialInstrumentAttributes107 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification3Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat4Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle8Choice>,
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
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "MinQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_qty_to_inst: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "MinMltplQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_qty_to_inst: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
    pub isse_pric: Option<PriceFormat45Choice>,
}
#[derive(
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
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<DecimalNumber>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OfferTypeFormat12ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OfferType4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OfferTypeFormat12Choice {
    #[serde(flatten)]
    pub value: OfferTypeFormat12ChoiceEnum,
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
pub struct OptionStyle8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionStyle2Code>,
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
pub struct OptionStyle8Choice {
    #[serde(flatten)]
    pub value: OptionStyle8ChoiceEnum,
}
#[derive(
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
pub struct RateTypeAndAmountAndStatus55 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType76Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
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
pub struct RateFormat20ChoiceEnum {
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
pub struct RateFormat20Choice {
    #[serde(flatten)]
    pub value: RateFormat20ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<IssuerTaxability2Code>,
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
pub struct CorporateActionRate104 {
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat37Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat36Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat38Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat8Choice>,
    #[serde(
        rename = "MaxAllwdOvrsbcptRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_allwd_ovrsbcpt_rate: Option<RateFormat3Choice>,
    #[serde(rename = "PrratnRate", skip_serializing_if = "Option::is_none")]
    pub prratn_rate: Option<RateFormat3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat41Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat41Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxblIncmPerDvddShr", default)]
    pub taxbl_incm_per_dvdd_shr: Vec<RateTypeAndAmountAndStatus26>,
    #[serde(rename = "IssrDclrdXchgRate", skip_serializing_if = "Option::is_none")]
    pub issr_dclrd_xchg_rate: Option<ForeignExchangeTerms19>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateAndAmountFormat37Choice>,
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
pub struct Period6ChoiceEnum {
    #[serde(rename = "PrdCd", skip_serializing_if = "Option::is_none")]
    pub prd_cd: Option<DateType8Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Period11>,
}
#[derive(
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
pub struct UpdatedAdditionalInformation12 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<Max140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Max350Text,
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
pub struct AmountPrice6 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType3Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountPriceType2Code {
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
pub struct SecuritiesOption81 {
    #[serde(rename = "MaxQtyToInst", skip_serializing_if = "Option::is_none")]
    pub max_qty_to_inst: Option<FinancialInstrumentQuantity34Choice>,
    #[serde(rename = "MinQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_qty_to_inst: Option<FinancialInstrumentQuantity34Choice>,
    #[serde(rename = "MinMltplQtyToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_qty_to_inst: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "NewBrdLotQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_qty: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "NewDnmtnQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_qty: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "FrntEndOddLotQty", skip_serializing_if = "Option::is_none")]
    pub frnt_end_odd_lot_qty: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "BckEndOddLotQty", skip_serializing_if = "Option::is_none")]
    pub bck_end_odd_lot_qty: Option<FinancialInstrumentQuantity35Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification32 {
    #[serde(rename = "Id")]
    pub id: DocumentIdentification3Choice,
    #[serde(rename = "DocNb", skip_serializing_if = "Option::is_none")]
    pub doc_nb: Option<DocumentNumber5Choice>,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition7Choice>,
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
pub struct ProprietaryQuantity7 {
    #[serde(rename = "ShrtLngPos", skip_serializing_if = "Option::is_none")]
    pub shrt_lng_pos: Option<ShortLong1Code>,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
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
pub struct FinancialInstrumentAttributes108 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification3Choice>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat4Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle8Choice>,
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
    pub warrt_parity: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity33Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max34Text>,
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
pub struct CashAccountIdentification5Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification5ChoiceEnum,
}
#[derive(
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
pub struct RateTypeAndAmountAndStatus37 {
    #[serde(rename = "RateTp")]
    pub rate_tp: DeemedRateType1Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
}
#[derive(
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
pub struct OptionAvailabilityStatus3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionAvailabilityStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionAvailabilityStatus3Choice {
    #[serde(flatten)]
    pub value: OptionAvailabilityStatus3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeemedRateType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeemedRateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeemedRateType1Choice {
    #[serde(flatten)]
    pub value: DeemedRateType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClassificationType32ChoiceEnum {
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification36>,
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
pub struct ClassificationType32Choice {
    #[serde(flatten)]
    pub value: ClassificationType32ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity35ChoiceEnum {
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Quantity5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity35Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity35ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification120ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
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
pub struct PartyIdentification120Choice {
    #[serde(flatten)]
    pub value: PartyIdentification120ChoiceEnum,
}
#[derive(
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
pub struct TemporaryFinancialInstrumentIndicator3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "TempInd", skip_serializing_if = "Option::is_none")]
    pub temp_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TemporaryFinancialInstrumentIndicator3Choice {
    #[serde(flatten)]
    pub value: TemporaryFinancialInstrumentIndicator3ChoiceEnum,
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
pub struct CorporateActionEventType84ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType31Code>,
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
pub struct CorporateActionEventType84Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventType84ChoiceEnum,
}
#[derive(
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
pub struct SignedQuantityFormat11 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "QtyChc")]
    pub qty_chc: Quantity48Choice,
}
#[derive(
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
pub struct CorporateActionGeneralInformation165 {
    #[validate]
    #[serde(rename = "CorpActnEvtId")]
    pub corp_actn_evt_id: Max35Text,
    #[serde(rename = "OffclCorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub offcl_corp_actn_evt_id: Option<Max35Text>,
    #[serde(rename = "ClssActnNb", skip_serializing_if = "Option::is_none")]
    pub clss_actn_nb: Option<Max35Text>,
    #[serde(rename = "EvtPrcgTp", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_tp: Option<CorporateActionEventProcessingType2Choice>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType84Choice,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary3Choice,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: FinancialInstrumentAttributes108,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DividendTypeFormat9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionFrequencyType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DividendTypeFormat9Choice {
    #[serde(flatten)]
    pub value: DividendTypeFormat9ChoiceEnum,
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
pub struct PartyIdentification127ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
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
pub struct PartyIdentification127Choice {
    #[serde(flatten)]
    pub value: PartyIdentification127ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionDate61 {
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "CertfctnDdln", skip_serializing_if = "Option::is_none")]
    pub certfctn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "CrtApprvlDt", skip_serializing_if = "Option::is_none")]
    pub crt_apprvl_dt: Option<DateFormat43Choice>,
    #[serde(rename = "EarlyClsgDt", skip_serializing_if = "Option::is_none")]
    pub early_clsg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<DateFormat43Choice>,
    #[serde(rename = "EqulstnDt", skip_serializing_if = "Option::is_none")]
    pub equlstn_dt: Option<DateFormat43Choice>,
    #[serde(rename = "FrthrDtldAnncmntDt", skip_serializing_if = "Option::is_none")]
    pub frthr_dtld_anncmnt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
    pub fxg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "LtryDt", skip_serializing_if = "Option::is_none")]
    pub ltry_dt: Option<DateFormat43Choice>,
    #[serde(rename = "NewMtrtyDt", skip_serializing_if = "Option::is_none")]
    pub new_mtrty_dt: Option<DateFormat43Choice>,
    #[serde(rename = "MtgDt", skip_serializing_if = "Option::is_none")]
    pub mtg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "MrgnFxgDt", skip_serializing_if = "Option::is_none")]
    pub mrgn_fxg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "PrratnDt", skip_serializing_if = "Option::is_none")]
    pub prratn_dt: Option<DateFormat43Choice>,
    #[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
    pub rcrd_dt: Option<DateFormat43Choice>,
    #[serde(rename = "RegnDdln", skip_serializing_if = "Option::is_none")]
    pub regn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "RsltsPblctnDt", skip_serializing_if = "Option::is_none")]
    pub rslts_pblctn_dt: Option<DateFormat43Choice>,
    #[serde(rename = "DdlnToSplt", skip_serializing_if = "Option::is_none")]
    pub ddln_to_splt: Option<DateFormat43Choice>,
    #[serde(
        rename = "DdlnForTaxBrkdwnInstr",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddln_for_tax_brkdwn_instr: Option<DateFormat43Choice>,
    #[serde(rename = "TradgSspdDt", skip_serializing_if = "Option::is_none")]
    pub tradg_sspd_dt: Option<DateFormat43Choice>,
    #[serde(rename = "UcondlDt", skip_serializing_if = "Option::is_none")]
    pub ucondl_dt: Option<DateFormat43Choice>,
    #[serde(rename = "WhlyUcondlDt", skip_serializing_if = "Option::is_none")]
    pub whly_ucondl_dt: Option<DateFormat43Choice>,
    #[serde(rename = "ExDvddDt", skip_serializing_if = "Option::is_none")]
    pub ex_dvdd_dt: Option<DateFormat43Choice>,
    #[serde(
        rename = "OffclAnncmntPblctnDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub offcl_anncmnt_pblctn_dt: Option<DateFormat43Choice>,
    #[serde(rename = "SpclExDt", skip_serializing_if = "Option::is_none")]
    pub spcl_ex_dt: Option<DateFormat43Choice>,
    #[serde(rename = "GrntedPrtcptnDt", skip_serializing_if = "Option::is_none")]
    pub grnted_prtcptn_dt: Option<DateFormat43Choice>,
    #[serde(
        rename = "ElctnToCtrPtyMktDdln",
        skip_serializing_if = "Option::is_none"
    )]
    pub elctn_to_ctr_pty_mkt_ddln: Option<DateFormat43Choice>,
    #[serde(
        rename = "ElctnToCtrPtyRspnDdln",
        skip_serializing_if = "Option::is_none"
    )]
    pub elctn_to_ctr_pty_rspn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "LpsdDt", skip_serializing_if = "Option::is_none")]
    pub lpsd_dt: Option<DateFormat43Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "ThrdPtyDdln", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "EarlyThrdPtyDdln", skip_serializing_if = "Option::is_none")]
    pub early_thrd_pty_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "MktClmTrckgEndDt", skip_serializing_if = "Option::is_none")]
    pub mkt_clm_trckg_end_dt: Option<DateFormat43Choice>,
    #[serde(rename = "LeadPlntffDdln", skip_serializing_if = "Option::is_none")]
    pub lead_plntff_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "FilgDt", skip_serializing_if = "Option::is_none")]
    pub filg_dt: Option<DateFormat30Choice>,
    #[serde(rename = "HrgDt", skip_serializing_if = "Option::is_none")]
    pub hrg_dt: Option<DateFormat30Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat42ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat42Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat42ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus56 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType77Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat36ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus55>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat36Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat36ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus58 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType79Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification47ChoiceEnum {
    #[serde(
        rename = "AcctsListAndBalDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub accts_list_and_bal_dtls: Option<AccountAndBalance47>,
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
pub struct AccountIdentification47Choice {
    #[serde(flatten)]
    pub value: AccountIdentification47ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat44ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode19Choice>,
    #[serde(rename = "DtCdAndTm", skip_serializing_if = "Option::is_none")]
    pub dt_cd_and_tm: Option<DateCodeAndTimeFormat3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat44Choice {
    #[serde(flatten)]
    pub value: DateFormat44ChoiceEnum,
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
pub struct OriginalAndCurrentQuantities6 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
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
pub struct Rate36 {
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat37Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat38Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat41Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat41Choice>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat37Choice>,
    #[serde(rename = "EarlySlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub early_slctn_fee_rate: Option<SolicitationFeeRateFormat7Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<RateFormat3Choice>,
    #[serde(rename = "ThrdPtyIncntivRate", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_incntiv_rate: Option<RateFormat20Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat39Choice>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<RateFormat3Choice>,
    #[serde(rename = "SlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub slctn_fee_rate: Option<SolicitationFeeRateFormat7Choice>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat20Choice>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateAndAmountFormat37Choice>,
    #[serde(rename = "TaxOnPrfts", skip_serializing_if = "Option::is_none")]
    pub tax_on_prfts: Option<RateFormat3Choice>,
    #[serde(rename = "TaxRclmRate", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_rate: Option<RateFormat3Choice>,
    #[serde(rename = "EqulstnRate", skip_serializing_if = "Option::is_none")]
    pub equlstn_rate: Option<RateAndAmountFormat42Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DmdRate", default)]
    pub dmd_rate: Vec<RateAndAmountFormat51Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat38ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus57>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat38Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat38ChoiceEnum,
}
#[derive(
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
pub struct IdentificationFormat3ChoiceEnum {
    #[serde(rename = "LngId", skip_serializing_if = "Option::is_none")]
    pub lng_id: Option<Max30Text>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "ShrtId", skip_serializing_if = "Option::is_none")]
    pub shrt_id: Option<Exact3UpperCaseAlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationFormat3Choice {
    #[serde(flatten)]
    pub value: IdentificationFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat46ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode20Choice>,
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
pub struct DateFormat46Choice {
    #[serde(flatten)]
    pub value: DateFormat46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNotificationV12<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Pgntn", skip_serializing_if = "Option::is_none")]
    pub pgntn: Option<Pagination1>,
    #[validate]
    #[serde(rename = "NtfctnGnlInf")]
    pub ntfctn_gnl_inf: CorporateActionNotification5,
    #[serde(rename = "PrvsNtfctnId", skip_serializing_if = "Option::is_none")]
    pub prvs_ntfctn_id: Option<DocumentIdentification31>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<DocumentIdentification9>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrDocId", default)]
    pub othr_doc_id: Vec<DocumentIdentification32>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtsLkg", default)]
    pub evts_lkg: Vec<CorporateActionEventReference3>,
    #[validate]
    #[serde(rename = "CorpActnGnlInf")]
    pub corp_actn_gnl_inf: CorporateActionGeneralInformation165,
    #[serde(rename = "AcctDtls")]
    pub acct_dtls: AccountIdentification47Choice,
    #[serde(rename = "IntrmdtScty", skip_serializing_if = "Option::is_none")]
    pub intrmdt_scty: Option<FinancialInstrumentAttributes110>,
    #[serde(rename = "CorpActnDtls", skip_serializing_if = "Option::is_none")]
    pub corp_actn_dtls: Option<CorporateAction60>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpActnOptnDtls", default)]
    pub corp_actn_optn_dtls: Vec<CorporateActionOption193>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative51>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IssrAgt", default)]
    pub issr_agt: Vec<PartyIdentification129Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PngAgt", default)]
    pub png_agt: Vec<PartyIdentification120Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SubPngAgt", default)]
    pub sub_png_agt: Vec<PartyIdentification120Choice>,
    #[serde(rename = "Regar", skip_serializing_if = "Option::is_none")]
    pub regar: Option<PartyIdentification120Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RsellngAgt", default)]
    pub rsellng_agt: Vec<PartyIdentification120Choice>,
    #[serde(rename = "PhysSctiesAgt", skip_serializing_if = "Option::is_none")]
    pub phys_scties_agt: Option<PartyIdentification120Choice>,
    #[serde(rename = "DrpAgt", skip_serializing_if = "Option::is_none")]
    pub drp_agt: Option<PartyIdentification120Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SlctnAgt", default)]
    pub slctn_agt: Vec<PartyIdentification120Choice>,
    #[serde(rename = "InfAgt", skip_serializing_if = "Option::is_none")]
    pub inf_agt: Option<PartyIdentification120Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification129Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Offerr", default)]
    pub offerr: Vec<PartyIdentification129Choice>,
    #[serde(rename = "TrfAgt", skip_serializing_if = "Option::is_none")]
    pub trf_agt: Option<PartyIdentification129Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType7Code {
    #[serde(rename = "ONGO")]
    Ongo,
    #[default]
    Unknown,
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
pub struct InterestRateUsedForPaymentFormat8ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType13Code>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus24>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateUsedForPaymentFormat8Choice {
    #[serde(flatten)]
    pub value: InterestRateUsedForPaymentFormat8ChoiceEnum,
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
pub struct AmountPrice3 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
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
pub struct FinancialInstrumentAttributes110 {
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification19,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<DecimalNumber>,
    #[serde(
        rename = "RnncblEntitlmntStsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub rnncbl_entitlmnt_sts_tp: Option<RenounceableEntitlementStatusTypeFormat3Choice>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType25Choice>,
    #[serde(
        rename = "IntrmdtSctiesToUndrlygRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_to_undrlyg_ratio: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<AmountPrice2>,
    #[serde(rename = "XpryDt")]
    pub xpry_dt: DateFormat30Choice,
    #[serde(rename = "PstngDt")]
    pub pstng_dt: DateFormat30Choice,
    #[serde(rename = "TradgPrd", skip_serializing_if = "Option::is_none")]
    pub tradg_prd: Option<Period11>,
    #[serde(rename = "UinstdBal", skip_serializing_if = "Option::is_none")]
    pub uinstd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "InstdBal", skip_serializing_if = "Option::is_none")]
    pub instd_bal: Option<BalanceFormat11Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndAmountAndStatus57 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType78Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
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
pub struct InformationTypeFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionInformationType1Code>,
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
pub struct InformationTypeFormat4Choice {
    #[serde(flatten)]
    pub value: InformationTypeFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPricePerFinancialInstrumentQuantity10 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "FinInstrmQty")]
    pub fin_instrm_qty: FinancialInstrumentQuantity33Choice,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Quantity4Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "ANYA")]
    Anya,
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
pub struct RateAndAmountFormat37ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
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
pub struct CorporateActionPrice72 {
    #[serde(rename = "MaxPric", skip_serializing_if = "Option::is_none")]
    pub max_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "MinPric", skip_serializing_if = "Option::is_none")]
    pub min_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "FrstBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub frst_bid_incrmt_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "LastBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub last_bid_incrmt_pric: Option<PriceFormat44Choice>,
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
pub struct CorporateActionChangeTypeFormat5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct CorporateActionChangeTypeFormat5Choice {
    #[serde(flatten)]
    pub value: CorporateActionChangeTypeFormat5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificationTypeFormat3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CertificationFormatType1Code>,
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
pub struct CertificationTypeFormat3Choice {
    #[serde(flatten)]
    pub value: CertificationTypeFormat3ChoiceEnum,
}
#[derive(
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
pub struct RateType36ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DividendRateType1Code>,
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
pub struct RateType36Choice {
    #[serde(flatten)]
    pub value: RateType36ChoiceEnum,
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
pub struct RateTypeAndAmountAndStatus24 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType33Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
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
pub struct CorporateActionEventProcessingType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct CorporateActionEventProcessingType2Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType33ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RateType33Choice {
    #[serde(flatten)]
    pub value: RateType33ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RenounceableEntitlementStatusTypeFormat3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RenounceableEntitlementStatusTypeFormat3Choice {
    #[serde(flatten)]
    pub value: RenounceableEntitlementStatusTypeFormat3ChoiceEnum,
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
pub struct CorporateActionPrice75 {
    #[serde(rename = "IndctvOrMktPric", skip_serializing_if = "Option::is_none")]
    pub indctv_or_mkt_pric: Option<IndicativeOrMarketPrice7Choice>,
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat45Choice>,
    #[serde(rename = "CshValForTax", skip_serializing_if = "Option::is_none")]
    pub csh_val_for_tax: Option<PriceFormat46Choice>,
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat44Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat65Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode20ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType1Code>,
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
pub struct DateCode20Choice {
    #[serde(flatten)]
    pub value: DateCode20ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct QuantityToQuantityRatio1 {
    #[validate]
    #[serde(rename = "Qty1")]
    pub qty_1: DecimalNumber,
    #[validate]
    #[serde(rename = "Qty2")]
    pub qty_2: DecimalNumber,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType42ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WithholdingTaxRateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType42Choice {
    #[serde(flatten)]
    pub value: RateType42ChoiceEnum,
}
#[derive(
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
pub struct DateFormat45ChoiceEnum {
    #[serde(rename = "NotSpcfdDt", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_dt: Option<DateType8Code>,
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
pub struct DateFormat45Choice {
    #[serde(flatten)]
    pub value: DateFormat45ChoiceEnum,
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
pub struct Max30Text {
    #[validate(length(max = 30,))]
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
pub struct CorporateActionNarrative51 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlTxt", default)]
    pub addtl_txt: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrrtvVrsn", default)]
    pub nrrtv_vrsn: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfConds", default)]
    pub inf_conds: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfToCmplyWth", default)]
    pub inf_to_cmply_wth: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxtnConds", default)]
    pub taxtn_conds: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dsclmr", default)]
    pub dsclmr: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PtyCtctNrrtv", default)]
    pub pty_ctct_nrrtv: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RegnDtls", default)]
    pub regn_dtls: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BsktOrIndxInf", default)]
    pub bskt_or_indx_inf: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwn", default)]
    pub certfctn_brkdwn: Vec<UpdatedAdditionalInformation13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "URLAdr", default)]
    pub url_adr: Vec<UpdatedUrLlnformation4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrcgTxtForNxtIntrmy", default)]
    pub prcg_txt_for_nxt_intrmy: Vec<UpdatedAdditionalInformation13>,
}
#[derive(
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
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType5Code>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<BaseOne14Rate>,
}
#[derive(
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
pub struct AmountAndQuantityRatio4 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
}
#[derive(
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
pub struct RateTypeAndAmountAndStatus26 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType36Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "RateSts", skip_serializing_if = "Option::is_none")]
    pub rate_sts: Option<RateStatus3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat11ChoiceEnum {
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat11>,
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat11Choice {
    #[serde(flatten)]
    pub value: BalanceFormat11ChoiceEnum,
}
#[derive(
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
pub struct BalanceFormat12ChoiceEnum {
    #[serde(rename = "PartWayPrdUnits", skip_serializing_if = "Option::is_none")]
    pub part_way_prd_units: Option<SignedQuantityFormat10>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat11>,
    #[serde(rename = "FullPrdUnits", skip_serializing_if = "Option::is_none")]
    pub full_prd_units: Option<SignedQuantityFormat10>,
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat12Choice {
    #[serde(flatten)]
    pub value: BalanceFormat12ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType9Code>,
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
pub struct DateCode33Choice {
    #[serde(flatten)]
    pub value: DateCode33ChoiceEnum,
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
pub struct CorporateActionNarrative45 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlTxt", default)]
    pub addtl_txt: Vec<UpdatedAdditionalInformation12>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrrtvVrsn", default)]
    pub nrrtv_vrsn: Vec<UpdatedAdditionalInformation12>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfConds", default)]
    pub inf_conds: Vec<UpdatedAdditionalInformation11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InfToCmplyWth", default)]
    pub inf_to_cmply_wth: Vec<UpdatedAdditionalInformation11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyRstrctn", default)]
    pub scty_rstrctn: Vec<UpdatedAdditionalInformation11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxtnConds", default)]
    pub taxtn_conds: Vec<UpdatedAdditionalInformation11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dsclmr", default)]
    pub dsclmr: Vec<UpdatedAdditionalInformation11>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertfctnBrkdwn", default)]
    pub certfctn_brkdwn: Vec<UpdatedAdditionalInformation11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalBusinessProcessFormat17ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AdditionalBusinessProcess9Code>,
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
pub struct AdditionalBusinessProcessFormat17Choice {
    #[serde(flatten)]
    pub value: AdditionalBusinessProcessFormat17ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "DfltOptnInd", skip_serializing_if = "Option::is_none")]
    pub dflt_optn_ind: Option<YesNoIndicator>,
    #[serde(rename = "StgInstrInd", skip_serializing_if = "Option::is_none")]
    pub stg_instr_ind: Option<YesNoIndicator>,
}
#[derive(
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
pub struct FinancialInstrumentQuantity33ChoiceEnum {
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
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
pub struct FinancialInstrumentQuantity33Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity33ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat44ChoiceEnum {
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
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
pub struct PriceFormat44Choice {
    #[serde(flatten)]
    pub value: PriceFormat44ChoiceEnum,
}
#[derive(
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
    pub corp_actn_ntfctn: CorporateActionNotificationV12<A>,
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
pub struct UpdatedAdditionalInformation13 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<Max140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max8000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification3ChoiceEnum {
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification3Choice {
    #[serde(flatten)]
    pub value: MarketIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber5ChoiceEnum {
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification36>,
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
pub struct DocumentNumber5Choice {
    #[serde(flatten)]
    pub value: DocumentNumber5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SolicitationFeeRateFormat7ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio4>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
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
pub struct SolicitationFeeRateFormat7Choice {
    #[serde(flatten)]
    pub value: SolicitationFeeRateFormat7ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionMandatoryVoluntary3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
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
pub struct CorporateActionMandatoryVoluntary3Choice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat18ChoiceEnum {
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio2>,
    #[serde(rename = "QtyToAmt", skip_serializing_if = "Option::is_none")]
    pub qty_to_amt: Option<AmountAndQuantityRatio4>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat18Choice {
    #[serde(flatten)]
    pub value: RatioFormat18ChoiceEnum,
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
pub struct ProprietaryQuantity8 {
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
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
pub struct IndicativeOrMarketPrice7ChoiceEnum {
    #[serde(rename = "IndctvPric", skip_serializing_if = "Option::is_none")]
    pub indctv_pric: Option<PriceFormat45Choice>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<PriceFormat45Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndicativeOrMarketPrice7Choice {
    #[serde(flatten)]
    pub value: IndicativeOrMarketPrice7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat30ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
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
pub struct DateFormat30Choice {
    #[serde(flatten)]
    pub value: DateFormat30ChoiceEnum,
}
#[derive(
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
pub struct RateTypeAndPercentageRate10 {
    #[serde(rename = "RateTp")]
    pub rate_tp: DeemedRateType1Choice,
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
pub struct CorporateActionEventReference3 {
    #[serde(rename = "EvtId")]
    pub evt_id: CorporateActionEventReference3Choice,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition7Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct ProcessingPosition7Choice {
    #[serde(flatten)]
    pub value: ProcessingPosition7ChoiceEnum,
}
#[derive(
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
pub struct RateTypeAndPercentageRate8 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType42Choice,
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
pub struct AmountPrice2 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType2Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPrice73 {
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat45Choice>,
    #[serde(rename = "OverSbcptDpstPric", skip_serializing_if = "Option::is_none")]
    pub over_sbcpt_dpst_pric: Option<PriceFormat45Choice>,
    #[serde(rename = "MaxCshToInst", skip_serializing_if = "Option::is_none")]
    pub max_csh_to_inst: Option<PriceFormat61Choice>,
    #[serde(rename = "MinCshToInst", skip_serializing_if = "Option::is_none")]
    pub min_csh_to_inst: Option<PriceFormat61Choice>,
    #[serde(rename = "MinMltplCshToInst", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_csh_to_inst: Option<PriceFormat61Choice>,
    #[serde(rename = "MaxPric", skip_serializing_if = "Option::is_none")]
    pub max_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "MinPric", skip_serializing_if = "Option::is_none")]
    pub min_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "FrstBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub frst_bid_incrmt_pric: Option<PriceFormat44Choice>,
    #[serde(rename = "LastBidIncrmtPric", skip_serializing_if = "Option::is_none")]
    pub last_bid_incrmt_pric: Option<PriceFormat44Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity34ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Quantity4Code>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
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
pub struct FinancialInstrumentQuantity34Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity34ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionAmounts54 {
    #[serde(rename = "GrssCshAmt", skip_serializing_if = "Option::is_none")]
    pub grss_csh_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NetCshAmt", skip_serializing_if = "Option::is_none")]
    pub net_csh_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SlctnFees", skip_serializing_if = "Option::is_none")]
    pub slctn_fees: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshInLieuOfShr", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CptlGn", skip_serializing_if = "Option::is_none")]
    pub cptl_gn: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrstAmt", skip_serializing_if = "Option::is_none")]
    pub intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IndmntyAmt", skip_serializing_if = "Option::is_none")]
    pub indmnty_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ManfctrdDvddPmtAmt", skip_serializing_if = "Option::is_none")]
    pub manfctrd_dvdd_pmt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RinvstmtAmt", skip_serializing_if = "Option::is_none")]
    pub rinvstmt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FullyFrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub fully_frnkd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "UfrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub ufrnkd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SndryOrOthrAmt", skip_serializing_if = "Option::is_none")]
    pub sndry_or_othr_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxFreeAmt", skip_serializing_if = "Option::is_none")]
    pub tax_free_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxDfrrdAmt", skip_serializing_if = "Option::is_none")]
    pub tax_dfrrd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ValAddedTaxAmt", skip_serializing_if = "Option::is_none")]
    pub val_added_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StmpDtyAmt", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxRclmAmt", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxCdtAmt", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AddtlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub addtl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "WhldgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub whldg_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ScndLvlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub scnd_lvl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FsclStmpAmt", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PngAgtComssnAmt", skip_serializing_if = "Option::is_none")]
    pub png_agt_comssn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "LclBrkrComssnAmt", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RgltryFeesAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_fees_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ShppgFeesAmt", skip_serializing_if = "Option::is_none")]
    pub shppg_fees_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ChrgsAmt", skip_serializing_if = "Option::is_none")]
    pub chrgs_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EntitldAmt", skip_serializing_if = "Option::is_none")]
    pub entitld_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrncplOrCrps", skip_serializing_if = "Option::is_none")]
    pub prncpl_or_crps: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RedPrmAmt", skip_serializing_if = "Option::is_none")]
    pub red_prm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IncmPrtn", skip_serializing_if = "Option::is_none")]
    pub incm_prtn: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StockXchgTax", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EUTaxRtntnAmt", skip_serializing_if = "Option::is_none")]
    pub eu_tax_rtntn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EqulstnAmt", skip_serializing_if = "Option::is_none")]
    pub equlstn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FATCATaxAmt", skip_serializing_if = "Option::is_none")]
    pub fatca_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NRATaxAmt", skip_serializing_if = "Option::is_none")]
    pub nra_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "BckUpWhldgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub bck_up_whldg_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxOnIncmAmt", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TxTax", skip_serializing_if = "Option::is_none")]
    pub tx_tax: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DmdAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FrgnIncmAmt", skip_serializing_if = "Option::is_none")]
    pub frgn_incm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DmdDvddAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_dvdd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DmdFndAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_fnd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DmdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DmdRyltsAmt", skip_serializing_if = "Option::is_none")]
    pub dmd_rylts_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountAndBalance47 {
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification127Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat28Choice>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<CorporateActionBalanceDetails43>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode21ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType7Code>,
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
pub struct DateCode21Choice {
    #[serde(flatten)]
    pub value: DateCode21ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionQuantity11 {
    #[serde(rename = "MaxQty", skip_serializing_if = "Option::is_none")]
    pub max_qty: Option<FinancialInstrumentQuantity34Choice>,
    #[serde(rename = "MinQtySght", skip_serializing_if = "Option::is_none")]
    pub min_qty_sght: Option<FinancialInstrumentQuantity34Choice>,
    #[serde(rename = "NewBrdLotQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_qty: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "NewDnmtnQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_qty: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "BaseDnmtn", skip_serializing_if = "Option::is_none")]
    pub base_dnmtn: Option<FinancialInstrumentQuantity35Choice>,
    #[serde(rename = "IncrmtlDnmtn", skip_serializing_if = "Option::is_none")]
    pub incrmtl_dnmtn: Option<FinancialInstrumentQuantity35Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateStatus3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RateStatus3Choice {
    #[serde(flatten)]
    pub value: RateStatus3ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate62 {
    #[serde(rename = "PmtDt")]
    pub pmt_dt: DateFormat43Choice,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateFormat46Choice>,
    #[serde(rename = "FXRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fx_rate_fxg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat43Choice>,
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
pub struct Quantity51ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "OrgnlAndCurFace", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face: Option<OriginalAndCurrentQuantities1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity51Choice {
    #[serde(flatten)]
    pub value: Quantity51ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat17ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat17Choice {
    #[serde(flatten)]
    pub value: RatioFormat17ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndText8 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
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
pub struct NonEligibleProceedsIndicator3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NonEligibleProceedsIndicator1Code>,
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
pub struct NonEligibleProceedsIndicator3Choice {
    #[serde(flatten)]
    pub value: NonEligibleProceedsIndicator3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BorrowerLendingDeadline5 {
    #[serde(rename = "StockLndgDdln")]
    pub stock_lndg_ddln: DateFormat43Choice,
    #[serde(rename = "Brrwr")]
    pub brrwr: PartyIdentification127Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType79ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NetDividendRateType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType79Choice {
    #[serde(flatten)]
    pub value: RateType79ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdatedAdditionalInformation11 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<Max140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdatedUrLlnformation4 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<Max140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "URLAdr")]
    pub url_adr: Max2048Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventReference3ChoiceEnum {
    #[serde(rename = "LkdCorpActnId", skip_serializing_if = "Option::is_none")]
    pub lkd_corp_actn_id: Option<Max35Text>,
    #[serde(
        rename = "LkdOffclCorpActnEvtId",
        skip_serializing_if = "Option::is_none"
    )]
    pub lkd_offcl_corp_actn_evt_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventReference3Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventReference3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity49ChoiceEnum {
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity7>,
    #[serde(rename = "QtyChc", skip_serializing_if = "Option::is_none")]
    pub qty_chc: Option<Quantity50Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity49Choice {
    #[serde(flatten)]
    pub value: Quantity49ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionNotification5 {
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: CorporateActionNotificationType1Code,
    #[serde(rename = "PrcgSts")]
    pub prcg_sts: CorporateActionProcessingStatus5Choice,
    #[serde(rename = "ElgblBalInd", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TotalEligibleBalanceFormat10 {
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<Quantity49Choice>,
    #[serde(rename = "FullPrdUnits", skip_serializing_if = "Option::is_none")]
    pub full_prd_units: Option<SignedQuantityFormat10>,
    #[serde(rename = "PartWayPrdUnits", skip_serializing_if = "Option::is_none")]
    pub part_way_prd_units: Option<SignedQuantityFormat10>,
}
#[derive(
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
pub struct SafekeepingPlaceFormat29ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText8>,
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
pub struct SafekeepingPlaceFormat29Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat29ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat41ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate8>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat41Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat41ChoiceEnum,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
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
pub struct ForeignExchangeTerms24 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "RsltgAmt", skip_serializing_if = "Option::is_none")]
    pub rsltg_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification9 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct DateFormat59ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode33Choice>,
}
#[derive(
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
pub struct CorporateActionDate77 {
    #[serde(rename = "EarlyRspnDdln", skip_serializing_if = "Option::is_none")]
    pub early_rspn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "CoverXprtnDdln", skip_serializing_if = "Option::is_none")]
    pub cover_xprtn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "PrtctDdln", skip_serializing_if = "Option::is_none")]
    pub prtct_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "RspnDdln", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln: Option<DateFormat44Choice>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<DateFormat43Choice>,
    #[serde(rename = "SbcptCostDbtDt", skip_serializing_if = "Option::is_none")]
    pub sbcpt_cost_dbt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "DpstryCoverXprtnDt", skip_serializing_if = "Option::is_none")]
    pub dpstry_cover_xprtn_dt: Option<DateFormat43Choice>,
    #[serde(rename = "StockLndgDdln", skip_serializing_if = "Option::is_none")]
    pub stock_lndg_ddln: Option<DateFormat43Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrrwrStockLndgDdln", default)]
    pub brrwr_stock_lndg_ddln: Vec<BorrowerLendingDeadline5>,
    #[serde(
        rename = "EndOfSctiesBlckgPrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_of_scties_blckg_prd: Option<DateFormat59Choice>,
}
#[derive(
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
pub struct NetDividendRateFormat39ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus58>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat39Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat39ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType78ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GrossDividendRateType7Code>,
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
pub struct RateType78Choice {
    #[serde(flatten)]
    pub value: RateType78ChoiceEnum,
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
pub struct PartyIdentification129ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct PartyIdentification129Choice {
    #[serde(flatten)]
    pub value: PartyIdentification129ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IntermediateSecuritiesDistributionTypeFormat15ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct IntermediateSecuritiesDistributionTypeFormat15Choice {
    #[serde(flatten)]
    pub value: IntermediateSecuritiesDistributionTypeFormat15ChoiceEnum,
}
#[derive(
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
pub enum SafekeepingAccountIdentification1Code {
    #[serde(rename = "GENR")]
    Genr,
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
pub struct DocumentIdentification31 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition7Choice>,
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
pub struct ConsentTypeFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConsentType1Code>,
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
pub struct ConsentTypeFormat4Choice {
    #[serde(flatten)]
    pub value: ConsentTypeFormat4ChoiceEnum,
}
#[derive(
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
pub struct PriceDetails27 {
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat44Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat65Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType76ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RateType76Choice {
    #[serde(flatten)]
    pub value: RateType76ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat46ChoiceEnum {
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat46Choice {
    #[serde(flatten)]
    pub value: PriceFormat46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat38ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType7Code>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus56>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateFormat38Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat38ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity50ChoiceEnum {
    #[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities6>,
    #[serde(rename = "SgndQty", skip_serializing_if = "Option::is_none")]
    pub sgnd_qty: Option<SignedQuantityFormat10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity50Choice {
    #[serde(flatten)]
    pub value: Quantity50ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification3ChoiceEnum {
    #[serde(rename = "AcctSvcrDocId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_doc_id: Option<Max35Text>,
    #[serde(rename = "AcctOwnrDocId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_doc_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification3Choice {
    #[serde(flatten)]
    pub value: DocumentIdentification3ChoiceEnum,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EventSequenceTypeFormat1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventSequenceType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EventSequenceTypeFormat1Choice {
    #[serde(flatten)]
    pub value: EventSequenceTypeFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType25ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType9Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType25Choice {
    #[serde(flatten)]
    pub value: FractionDispositionType25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType77ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NetDividendRateType6Code>,
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
pub struct RateType77Choice {
    #[serde(flatten)]
    pub value: RateType77ChoiceEnum,
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
pub struct LotteryTypeFormat4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LotteryType1Code>,
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
pub struct LotteryTypeFormat4Choice {
    #[serde(flatten)]
    pub value: LotteryTypeFormat4ChoiceEnum,
}
#[derive(
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
pub struct BeneficiaryCertificationType9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BeneficiaryCertificationType4Code>,
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
pub struct BeneficiaryCertificationType9Choice {
    #[serde(flatten)]
    pub value: BeneficiaryCertificationType9ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LotteryType1Code {
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "SUPP")]
    Supp,
    #[default]
    Unknown,
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
pub struct SecurityDate16 {
    #[serde(rename = "PmtDt")]
    pub pmt_dt: DateFormat43Choice,
    #[serde(rename = "AvlblDt", skip_serializing_if = "Option::is_none")]
    pub avlbl_dt: Option<DateFormat43Choice>,
    #[serde(rename = "DvddRnkgDt", skip_serializing_if = "Option::is_none")]
    pub dvdd_rnkg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "PrpssDt", skip_serializing_if = "Option::is_none")]
    pub prpss_dt: Option<DateFormat43Choice>,
    #[serde(rename = "LastTradgDt", skip_serializing_if = "Option::is_none")]
    pub last_tradg_dt: Option<DateFormat43Choice>,
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
pub struct UpdatedAdditionalInformation3 {
    #[serde(rename = "UpdDesc", skip_serializing_if = "Option::is_none")]
    pub upd_desc: Option<Max140Text>,
    #[serde(rename = "UpdDt", skip_serializing_if = "Option::is_none")]
    pub upd_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
