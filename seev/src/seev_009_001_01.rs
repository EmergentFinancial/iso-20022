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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.009.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DistributionType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DistributionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DistributionType1FormatChoice {
    #[serde(flatten)]
    pub value: DistributionType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption1 {
    #[validate]
    #[serde(rename = "OptnNb")]
    pub optn_nb: Exact3NumericText,
    #[serde(rename = "OptnTp")]
    pub optn_tp: CorporateActionOption1FormatChoice,
    #[serde(rename = "OptnAvlbtySts")]
    pub optn_avlbty_sts: CorporateActionEventStatus2FormatChoice,
    #[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<BeneficiaryCertificationType1FormatChoice>,
    #[serde(rename = "AssntdLineSctyId", skip_serializing_if = "Option::is_none")]
    pub assntd_line_scty_id: Option<SecurityIdentification7>,
    #[serde(rename = "AgtSctiesAcctId", skip_serializing_if = "Option::is_none")]
    pub agt_scties_acct_id: Option<Max35Text>,
    #[serde(rename = "AgtCshAcctId", skip_serializing_if = "Option::is_none")]
    pub agt_csh_acct_id: Option<AccountIdentification2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferType1FormatChoice>,
    #[serde(
        rename = "IntrmdtSctiesDstrbtnTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_dstrbtn_tp: Option<IntermediateSecurityDistributionType1FormatChoice>,
    #[validate]
    #[serde(rename = "WdrwlAllwdInd")]
    pub wdrwl_allwd_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "ChngAllwdInd")]
    pub chng_allwd_ind: YesNoIndicator,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate4>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate2>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice1>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmntDtls", default)]
    pub scties_mvmnt_dtls: Vec<SecurityOption1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmntDtls", default)]
    pub csh_mvmnt_dtls: Vec<CashOption1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpActnOthrAgtDtls", default)]
    pub corp_actn_othr_agt_dtls: Vec<CorporateActionAgent1>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType1FormatChoice>,
    #[serde(rename = "RedChrgsApldInd", skip_serializing_if = "Option::is_none")]
    pub red_chrgs_apld_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnFeatrs", default)]
    pub optn_featrs: Vec<OptionFeatures1FormatChoice>,
    #[serde(rename = "CorpActnAddtlInf", skip_serializing_if = "Option::is_none")]
    pub corp_actn_addtl_inf: Option<CorporateActionNarrative1>,
}
#[derive(
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
pub struct PriceFormat4ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountPrice1>,
    #[serde(rename = "NotSpcfd", skip_serializing_if = "Option::is_none")]
    pub not_spcfd: Option<PriceValueType5FormatChoice>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PriceRate1>,
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
pub struct PriceFormat4Choice {
    #[serde(flatten)]
    pub value: PriceFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPriceType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AmountPriceType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountPriceType1FormatChoice {
    #[serde(flatten)]
    pub value: AmountPriceType1FormatChoiceEnum,
}
#[derive(
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
pub struct CorporateActionInformation2 {
    #[serde(rename = "AgtId")]
    pub agt_id: PartyIdentification2Choice,
    #[validate]
    #[serde(rename = "IssrCorpActnId")]
    pub issr_corp_actn_id: Max35Text,
    #[serde(rename = "CorpActnPrcgId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_prcg_id: Option<Max35Text>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType2FormatChoice,
    #[serde(rename = "EvtPrcgTp", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_tp: Option<CorporateActionEventProcessingType1FormatChoice>,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary1FormatChoice,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: FinancialInstrumentDescription3,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrUndrlygScty", default)]
    pub othr_undrlyg_scty: Vec<FinancialInstrumentDescription3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification2ChoiceEnum {
    #[serde(rename = "SctiesAcctId", skip_serializing_if = "Option::is_none")]
    pub scties_acct_id: Option<Max35Text>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification2Choice {
    #[serde(flatten)]
    pub value: AccountIdentification2ChoiceEnum,
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
pub struct CorporateActionPeriod1 {
    #[serde(rename = "ActnPrd", skip_serializing_if = "Option::is_none")]
    pub actn_prd: Option<Period1>,
    #[serde(rename = "CmplsryPurchsPrd", skip_serializing_if = "Option::is_none")]
    pub cmplsry_purchs_prd: Option<Period1>,
    #[serde(rename = "IntrstPrd", skip_serializing_if = "Option::is_none")]
    pub intrst_prd: Option<Period1>,
    #[serde(rename = "BlckgPrd", skip_serializing_if = "Option::is_none")]
    pub blckg_prd: Option<Period1>,
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<Period1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate5 {
    #[serde(rename = "FXRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fx_rate_fxg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateFormat4Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat4Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat4Choice>,
}
#[derive(
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
    #[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
    pub bic_or_bei: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
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
pub struct RatioFormat2ChoiceEnum {
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio1>,
    #[serde(rename = "QtyToAmt", skip_serializing_if = "Option::is_none")]
    pub qty_to_amt: Option<AmountAndQuantityRatio1>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio1>,
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat2Choice {
    #[serde(flatten)]
    pub value: RatioFormat2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ElectionMovementType1Code {
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "DRCT")]
    Drct,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType6Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "TBSP")]
    Tbsp,
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
pub struct ProcessingStatus1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus1FormatChoice {
    #[serde(flatten)]
    pub value: ProcessingStatus1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionOptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionOption1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionAmounts1 {
    #[serde(rename = "GrssCshAmt", skip_serializing_if = "Option::is_none")]
    pub grss_csh_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NetCshAmt", skip_serializing_if = "Option::is_none")]
    pub net_csh_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IsseDscntAmt", skip_serializing_if = "Option::is_none")]
    pub isse_dscnt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SlctnFees", skip_serializing_if = "Option::is_none")]
    pub slctn_fees: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshInLieuOfShr", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CptlGn", skip_serializing_if = "Option::is_none")]
    pub cptl_gn: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrstAmt", skip_serializing_if = "Option::is_none")]
    pub intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IndmntyAmt", skip_serializing_if = "Option::is_none")]
    pub indmnty_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RedPrmAmt", skip_serializing_if = "Option::is_none")]
    pub red_prm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ManfctrdDvddAmt", skip_serializing_if = "Option::is_none")]
    pub manfctrd_dvdd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrncplOrCrps", skip_serializing_if = "Option::is_none")]
    pub prncpl_or_crps: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RinvstmtAmt", skip_serializing_if = "Option::is_none")]
    pub rinvstmt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MktClmAmt", skip_serializing_if = "Option::is_none")]
    pub mkt_clm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FullyFrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub fully_frnkd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "UfrnkdAmt", skip_serializing_if = "Option::is_none")]
    pub ufrnkd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SndryOrOthrAmt", skip_serializing_if = "Option::is_none")]
    pub sndry_or_othr_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SpclCncssnAmt", skip_serializing_if = "Option::is_none")]
    pub spcl_cncssn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EntitldAmt", skip_serializing_if = "Option::is_none")]
    pub entitld_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshIncntiv", skip_serializing_if = "Option::is_none")]
    pub csh_incntiv: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AddtlSbcptCost", skip_serializing_if = "Option::is_none")]
    pub addtl_sbcpt_cost: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxFreeAmt", skip_serializing_if = "Option::is_none")]
    pub tax_free_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxDfrrdAmt", skip_serializing_if = "Option::is_none")]
    pub tax_dfrrd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrmnLclTax1Amt", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_1_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrmnLclTax2Amt", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_2_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrmnLclTax3Amt", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_3_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrmnLclTax4Amt", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_4_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StockXchgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TrfTaxAmt", skip_serializing_if = "Option::is_none")]
    pub trf_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TxTaxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ValAddedTaxAmt", skip_serializing_if = "Option::is_none")]
    pub val_added_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EURtntnTaxAmt", skip_serializing_if = "Option::is_none")]
    pub eu_rtntn_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "LclTaxAmt", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PmtLevyTaxAmt", skip_serializing_if = "Option::is_none")]
    pub pmt_levy_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CtryNtlFdrlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ctry_ntl_fdrl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StmpDtyAmt", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxRclmAmt", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxCdtAmt", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "WhldgOfFrgnTaxAmt", skip_serializing_if = "Option::is_none")]
    pub whldg_of_frgn_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "WhldgOfLclTaxAmt", skip_serializing_if = "Option::is_none")]
    pub whldg_of_lcl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AddtlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub addtl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "WhldgTaxAmt", skip_serializing_if = "Option::is_none")]
    pub whldg_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FsclStmpAmt", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PngAgtComssnAmt", skip_serializing_if = "Option::is_none")]
    pub png_agt_comssn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "LclBrkrComssnAmt", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PstgFeeAmt", skip_serializing_if = "Option::is_none")]
    pub pstg_fee_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RgltryFeesAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_fees_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ShppgFeesAmt", skip_serializing_if = "Option::is_none")]
    pub shppg_fees_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ChrgsAmt", skip_serializing_if = "Option::is_none")]
    pub chrgs_amt: Option<ActiveCurrencyAndAmount>,
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
pub struct CorporateActionEventStage1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStage1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStage1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventStage1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FractionDispositionType1Code {
    #[serde(rename = "BUYU")]
    Buyu,
    #[serde(rename = "CINL")]
    Cinl,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "RDDN")]
    Rddn,
    #[serde(rename = "RDUP")]
    Rdup,
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
pub struct AmountPrice1 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1FormatChoice,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessingStatus1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "PREC")]
    Prec,
    #[serde(rename = "PREU")]
    Preu,
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
pub struct CorporateActionDate3 {
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat4Choice>,
    #[serde(rename = "AvlblDt", skip_serializing_if = "Option::is_none")]
    pub avlbl_dt: Option<DateFormat4Choice>,
    #[serde(rename = "DvddRnkgDt", skip_serializing_if = "Option::is_none")]
    pub dvdd_rnkg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "PrpssDt", skip_serializing_if = "Option::is_none")]
    pub prpss_dt: Option<DateFormat4Choice>,
    #[serde(rename = "FrstDealgDt", skip_serializing_if = "Option::is_none")]
    pub frst_dealg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat4ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTimeChoice>,
    #[serde(rename = "NotSpcfdDt", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_dt: Option<DateType6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat4Choice {
    #[serde(flatten)]
    pub value: DateFormat4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShareRanking1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ShareRanking1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShareRanking1FormatChoice {
    #[serde(flatten)]
    pub value: ShareRanking1FormatChoiceEnum,
}
#[derive(
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
pub struct CorporateActionPeriod2 {
    #[serde(rename = "AssntdLinePrd", skip_serializing_if = "Option::is_none")]
    pub assntd_line_prd: Option<Period1>,
    #[serde(rename = "ActnPrd", skip_serializing_if = "Option::is_none")]
    pub actn_prd: Option<Period1>,
    #[serde(rename = "PrvlgSspnsnPrd", skip_serializing_if = "Option::is_none")]
    pub prvlg_sspnsn_prd: Option<Period1>,
    #[serde(rename = "ParllTradgPrd", skip_serializing_if = "Option::is_none")]
    pub parll_tradg_prd: Option<Period1>,
    #[serde(rename = "SellThruIssrPrd", skip_serializing_if = "Option::is_none")]
    pub sell_thru_issr_prd: Option<Period1>,
    #[serde(rename = "RvcbltyPrd", skip_serializing_if = "Option::is_none")]
    pub rvcblty_prd: Option<Period1>,
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<Period1>,
}
#[derive(
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
pub struct CorporateActionEventProcessingType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
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
pub struct CorporateActionEventProcessingType1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingType1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessingPosition2Code {
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
pub struct RateFormat1ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
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
pub struct RateFormat1Choice {
    #[serde(flatten)]
    pub value: RateFormat1ChoiceEnum,
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
pub struct CorporateActionRate2 {
    #[serde(rename = "WhldgTax", skip_serializing_if = "Option::is_none")]
    pub whldg_tax: Option<RateFormat1Choice>,
    #[serde(rename = "WhldgOfFrgnTax", skip_serializing_if = "Option::is_none")]
    pub whldg_of_frgn_tax: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "WhldgOfLclTax", skip_serializing_if = "Option::is_none")]
    pub whldg_of_lcl_tax: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "GrmnLclTax1", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_1: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "GrmnLclTax2", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_2: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "GrmnLclTax3", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_3: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "GrmnLclTax4", skip_serializing_if = "Option::is_none")]
    pub grmn_lcl_tax_4: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateFormat1Choice>,
    #[serde(rename = "TaxOnPrft", skip_serializing_if = "Option::is_none")]
    pub tax_on_prft: Option<RateFormat1Choice>,
    #[serde(rename = "TaxRclm", skip_serializing_if = "Option::is_none")]
    pub tax_rclm: Option<RateFormat1Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<RateFormat1Choice>,
    #[serde(rename = "Prratn", skip_serializing_if = "Option::is_none")]
    pub prratn: Option<RateFormat1Choice>,
    #[serde(rename = "NewToOd", skip_serializing_if = "Option::is_none")]
    pub new_to_od: Option<RatioFormat2Choice>,
    #[serde(
        rename = "NewSctiesToUndrlygScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_scties_to_undrlyg_scties: Option<RatioFormat2Choice>,
    #[serde(
        rename = "AddtlQtyForExstgScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_exstg_scties: Option<RatioFormat1Choice>,
    #[serde(
        rename = "AddtlQtyForSbcbdRsltntScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_sbcbd_rsltnt_scties: Option<RatioFormat1Choice>,
    #[serde(rename = "RltdTax", skip_serializing_if = "Option::is_none")]
    pub rltd_tax: Option<RelatedTaxType1>,
    #[serde(rename = "NonResdtRate", skip_serializing_if = "Option::is_none")]
    pub non_resdt_rate: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "IntrstForUsdPmt", skip_serializing_if = "Option::is_none")]
    pub intrst_for_usd_pmt: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "IndxFctr", skip_serializing_if = "Option::is_none")]
    pub indx_fctr: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "FullyFrnkd", skip_serializing_if = "Option::is_none")]
    pub fully_frnkd: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "GrssDvdd", skip_serializing_if = "Option::is_none")]
    pub grss_dvdd: Option<GrossDividendRate1Choice>,
    #[serde(rename = "NetDvdd", skip_serializing_if = "Option::is_none")]
    pub net_dvdd: Option<NetDividendRate1Choice>,
    #[serde(rename = "FnlDvdd", skip_serializing_if = "Option::is_none")]
    pub fnl_dvdd: Option<AmountAndRateFormat2Choice>,
    #[serde(rename = "PrvsnlDvdd", skip_serializing_if = "Option::is_none")]
    pub prvsnl_dvdd: Option<AmountAndRateFormat2Choice>,
    #[serde(rename = "CshIncntiv", skip_serializing_if = "Option::is_none")]
    pub csh_incntiv: Option<RateFormat1Choice>,
    #[serde(rename = "SlctnFee", skip_serializing_if = "Option::is_none")]
    pub slctn_fee: Option<RateFormat1Choice>,
    #[serde(rename = "MaxAllwdOvrsbcpt", skip_serializing_if = "Option::is_none")]
    pub max_allwd_ovrsbcpt: Option<RateFormat1Choice>,
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<ForeignExchangeTerms8>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<RateFormat1Choice>,
}
#[derive(
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
pub struct BeneficiaryCertificationType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BeneficiaryCertificationType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryCertificationType1FormatChoice {
    #[serde(flatten)]
    pub value: BeneficiaryCertificationType1FormatChoiceEnum,
}
#[derive(
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
pub struct AlternateSecurityIdentification3Enum {
    #[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
    pub dmst_id_src: Option<CountryCode>,
    #[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
    pub prtry_id_src: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternateSecurityIdentification3 {
    #[serde(flatten)]
    pub value: AlternateSecurityIdentification3Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat3ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountPrice1>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PriceRate1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat3Choice {
    #[serde(flatten)]
    pub value: PriceFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateValueType6FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateValueType6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateValueType6FormatChoice {
    #[serde(flatten)]
    pub value: RateValueType6FormatChoiceEnum,
}
#[derive(
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
    #[serde(rename = "AgtCANtfctnAdvc")]
    pub agt_ca_ntfctn_advc: AgentCaNotificationAdviceV01,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType5Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "OPEN")]
    Open,
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
pub struct UnitOrFaceAmount1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmount1Choice {
    #[serde(flatten)]
    pub value: UnitOrFaceAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContactPerson1 {
    #[validate]
    #[serde(rename = "CtctPrsn")]
    pub ctct_prsn: ContactIdentification4,
    #[serde(rename = "InstnId", skip_serializing_if = "Option::is_none")]
    pub instn_id: Option<PartyIdentification2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityOption1 {
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: FinancialInstrumentDescription3,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "SctiesQty", skip_serializing_if = "Option::is_none")]
    pub scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "MinExrcblSctiesQty", skip_serializing_if = "Option::is_none")]
    pub min_exrcbl_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(
        rename = "MinExrcblMltplSctiesQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_exrcbl_mltpl_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "NewDnmtnSctiesQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "NewBrdLotSctiesQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "ShrRnkg", skip_serializing_if = "Option::is_none")]
    pub shr_rnkg: Option<ShareRanking1FormatChoice>,
    #[serde(
        rename = "AddtlQtyForSbcbdRsltntScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_sbcbd_rsltnt_scties: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate3>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice4>,
    #[serde(rename = "TradgPrd", skip_serializing_if = "Option::is_none")]
    pub tradg_prd: Option<Period1>,
    #[serde(
        rename = "AddtlQtyForExstgScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_exstg_scties: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "TempFinInstrmInd", skip_serializing_if = "Option::is_none")]
    pub temp_fin_instrm_ind: Option<YesNoIndicator>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType1FormatChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionFrequencyType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionFrequencyType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionFrequencyType1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionFrequencyType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionFeatures1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionFeatures1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionFeatures1FormatChoice {
    #[serde(flatten)]
    pub value: OptionFeatures1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashOption1 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate5>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<CorporateActionAmounts1>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<ForeignExchangeTerms8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRate2 {
    #[serde(rename = "RateTp")]
    pub rate_tp: NetDividendRateType1FormatChoice,
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
pub struct ConversionType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConversionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConversionType1FormatChoice {
    #[serde(flatten)]
    pub value: ConversionType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms8 {
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AgentRole2Code {
    #[serde(rename = "SPAY")]
    Spay,
    #[serde(rename = "CODO")]
    Codo,
    #[serde(rename = "ISAG")]
    Isag,
    #[serde(rename = "REGR")]
    Regr,
    #[serde(rename = "PAYA")]
    Paya,
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
pub struct GrossDividendRate2 {
    #[serde(rename = "RateTp")]
    pub rate_tp: GrossDividendRateType1FormatChoice,
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
pub struct CorporateAction2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtStag", default)]
    pub evt_stag: Vec<CorporateActionEventStage1FormatChoice>,
    #[serde(rename = "DfltOptnTp", skip_serializing_if = "Option::is_none")]
    pub dflt_optn_tp: Option<CorporateActionOption1FormatChoice>,
    #[serde(rename = "DfltOptnNb", skip_serializing_if = "Option::is_none")]
    pub dflt_optn_nb: Option<Exact3NumericText>,
    #[serde(rename = "ClctnMtd", skip_serializing_if = "Option::is_none")]
    pub clctn_mtd: Option<CorporateActionCalculationMethod1FormatChoice>,
    #[serde(
        rename = "BckEndOddLotSctiesQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub bck_end_odd_lot_scties_qty: Option<UnitOrFaceAmountOrCode1Choice>,
    #[serde(
        rename = "FrntEndOddLotSctiesQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub frnt_end_odd_lot_scties_qty: Option<UnitOrFaceAmountOrCode1Choice>,
    #[serde(rename = "MinExrcblSctiesQty", skip_serializing_if = "Option::is_none")]
    pub min_exrcbl_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(
        rename = "MinExrcblMltplSctiesQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_exrcbl_mltpl_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "IncrmtlDnmtn", skip_serializing_if = "Option::is_none")]
    pub incrmtl_dnmtn: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "NewDnmtnSctiesQty", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "NewBrdLotSctiesQty", skip_serializing_if = "Option::is_none")]
    pub new_brd_lot_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "SctiesQtySght", skip_serializing_if = "Option::is_none")]
    pub scties_qty_sght: Option<UnitOrFaceAmountOrCode1Choice>,
    #[serde(rename = "BaseDnmtn", skip_serializing_if = "Option::is_none")]
    pub base_dnmtn: Option<UnitOrFaceAmount1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ChngTp", default)]
    pub chng_tp: Vec<CorporateActionChangeType1FormatChoice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OfferTp", default)]
    pub offer_tp: Vec<OfferType1FormatChoice>,
    #[serde(rename = "RstrctnInd", skip_serializing_if = "Option::is_none")]
    pub rstrctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrtlElctnInd", skip_serializing_if = "Option::is_none")]
    pub prtl_elctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "ElctnTp", skip_serializing_if = "Option::is_none")]
    pub elctn_tp: Option<ElectionMovementType1FormatChoice>,
    #[serde(rename = "LtryTp", skip_serializing_if = "Option::is_none")]
    pub ltry_tp: Option<LotteryType1FormatChoice>,
    #[serde(rename = "IncmTp", skip_serializing_if = "Option::is_none")]
    pub incm_tp: Option<GenericIdentification13>,
    #[serde(rename = "DvddTp", skip_serializing_if = "Option::is_none")]
    pub dvdd_tp: Option<CorporateActionFrequencyType1FormatChoice>,
    #[serde(
        rename = "IntrmdtSctiesDstrbtnTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_dstrbtn_tp: Option<IntermediateSecurityDistributionType1FormatChoice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CpnNb", default)]
    pub cpn_nb: Vec<Max3NumericText>,
    #[serde(rename = "IntrstAcrdNbOfDays", skip_serializing_if = "Option::is_none")]
    pub intrst_acrd_nb_of_days: Option<Number>,
    #[serde(rename = "NewDnmtnCcy", skip_serializing_if = "Option::is_none")]
    pub new_dnmtn_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PricDtls", default)]
    pub pric_dtls: Vec<CorporateActionPrice2>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod1>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate1>,
    #[serde(rename = "CorpActnAddtlInf", skip_serializing_if = "Option::is_none")]
    pub corp_actn_addtl_inf: Option<CorporateActionNarrative1>,
    #[serde(rename = "CertfctnReqrdInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_reqrd_ind: Option<YesNoIndicator>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<BeneficiaryCertificationType1FormatChoice>,
    #[serde(rename = "CptlGn", skip_serializing_if = "Option::is_none")]
    pub cptl_gn: Option<EuCapitalGain2Code>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Code>,
    #[serde(rename = "NewPlcOfIncorprtn", skip_serializing_if = "Option::is_none")]
    pub new_plc_of_incorprtn: Option<Max70Text>,
    #[serde(
        rename = "RnncblEntitlmntStsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub rnncbl_entitlmnt_sts_tp: Option<RenounceableStatus1FormatChoice>,
    #[serde(rename = "ConvsTp", skip_serializing_if = "Option::is_none")]
    pub convs_tp: Option<ConversionType1FormatChoice>,
    #[serde(rename = "RedChrgsApldInd", skip_serializing_if = "Option::is_none")]
    pub red_chrgs_apld_ind: Option<YesNoIndicator>,
    #[serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_tp: Option<DistributionType1FormatChoice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType6Code {
    #[serde(rename = "OPEN")]
    Open,
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
pub struct ElectionMovementType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ElectionMovementType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ElectionMovementType1FormatChoice {
    #[serde(flatten)]
    pub value: ElectionMovementType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate4 {
    #[serde(rename = "CpnClpngDt", skip_serializing_if = "Option::is_none")]
    pub cpn_clpng_dt: Option<DateFormat4Choice>,
    #[serde(rename = "CnsntXprtnDt", skip_serializing_if = "Option::is_none")]
    pub cnsnt_xprtn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "CnsntRcrdDt", skip_serializing_if = "Option::is_none")]
    pub cnsnt_rcrd_dt: Option<DateFormat4Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat4Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat4Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat4Choice>,
    #[serde(rename = "RspnDdln", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln: Option<DateFormat4Choice>,
    #[serde(rename = "DdlnToSplt", skip_serializing_if = "Option::is_none")]
    pub ddln_to_splt: Option<DateFormat4Choice>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<DateFormat4Choice>,
    #[serde(rename = "QtnSetngDt", skip_serializing_if = "Option::is_none")]
    pub qtn_setng_dt: Option<DateFormat4Choice>,
    #[serde(rename = "SbcptCostDbtDt", skip_serializing_if = "Option::is_none")]
    pub sbcpt_cost_dbt_dt: Option<DateFormat4Choice>,
}
#[derive(
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
pub struct RateType12FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateType12Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType12FormatChoice {
    #[serde(flatten)]
    pub value: RateType12FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Quantity1Code {
    #[serde(rename = "QALL")]
    Qall,
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
pub struct UnitOrFaceAmountOrCode1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Quantity1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmountOrCode1Choice {
    #[serde(flatten)]
    pub value: UnitOrFaceAmountOrCode1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgentRole1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AgentRole2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgentRole1FormatChoice {
    #[serde(flatten)]
    pub value: AgentRole1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification8 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
}
#[derive(
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
pub struct NetDividendRate1ChoiceEnum {
    #[serde(rename = "RateTpAmt", skip_serializing_if = "Option::is_none")]
    pub rate_tp_amt: Option<NetDividendRate2>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType6FormatChoice>,
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
pub struct NetDividendRate1Choice {
    #[serde(flatten)]
    pub value: NetDividendRate1ChoiceEnum,
}
#[derive(
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
pub struct PriceRate1 {
    #[serde(rename = "RateTp")]
    pub rate_tp: PriceRateType3FormatChoice,
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
pub struct CorporateActionDate2 {
    #[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
    pub rcrd_dt: Option<DateFormat4Choice>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<DateFormat4Choice>,
    #[serde(rename = "CoverXprtnDt", skip_serializing_if = "Option::is_none")]
    pub cover_xprtn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "EqulstnDt", skip_serializing_if = "Option::is_none")]
    pub equlstn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "MrgnFxgDt", skip_serializing_if = "Option::is_none")]
    pub mrgn_fxg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "LtryDt", skip_serializing_if = "Option::is_none")]
    pub ltry_dt: Option<DateFormat4Choice>,
    #[serde(rename = "PrtctDt", skip_serializing_if = "Option::is_none")]
    pub prtct_dt: Option<DateFormat4Choice>,
    #[serde(rename = "UcondlDt", skip_serializing_if = "Option::is_none")]
    pub ucondl_dt: Option<DateFormat4Choice>,
    #[serde(rename = "WhlyUcondlDt", skip_serializing_if = "Option::is_none")]
    pub whly_ucondl_dt: Option<DateFormat4Choice>,
    #[serde(rename = "RsltsPblctnDt", skip_serializing_if = "Option::is_none")]
    pub rslts_pblctn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "CrtApprvlDt", skip_serializing_if = "Option::is_none")]
    pub crt_apprvl_dt: Option<DateFormat4Choice>,
    #[serde(rename = "EarlyClsgDt", skip_serializing_if = "Option::is_none")]
    pub early_clsg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "ExDvddDt", skip_serializing_if = "Option::is_none")]
    pub ex_dvdd_dt: Option<DateFormat4Choice>,
    #[serde(rename = "IndxFxgDt", skip_serializing_if = "Option::is_none")]
    pub indx_fxg_dt: Option<DateFormat4Choice>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<DateFormat4Choice>,
    #[serde(rename = "TradgSspdDt", skip_serializing_if = "Option::is_none")]
    pub tradg_sspd_dt: Option<DateFormat4Choice>,
    #[serde(rename = "CertfctnDdln", skip_serializing_if = "Option::is_none")]
    pub certfctn_ddln: Option<DateFormat4Choice>,
    #[serde(rename = "RedDt", skip_serializing_if = "Option::is_none")]
    pub red_dt: Option<DateFormat4Choice>,
    #[serde(rename = "RegnDdln", skip_serializing_if = "Option::is_none")]
    pub regn_ddln: Option<DateFormat4Choice>,
    #[serde(rename = "PrratnDt", skip_serializing_if = "Option::is_none")]
    pub prratn_dt: Option<DateFormat4Choice>,
    #[serde(
        rename = "DdlnForTaxBrkdwnInstr",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddln_for_tax_brkdwn_instr: Option<DateFormat4Choice>,
    #[serde(rename = "LpsdDt", skip_serializing_if = "Option::is_none")]
    pub lpsd_dt: Option<DateFormat4Choice>,
    #[serde(rename = "GrntedPrtcptnDt", skip_serializing_if = "Option::is_none")]
    pub grnted_prtcptn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "ElctnToCtrPtyDdln", skip_serializing_if = "Option::is_none")]
    pub elctn_to_ctr_pty_ddln: Option<DateFormat4Choice>,
    #[serde(rename = "SpclExDt", skip_serializing_if = "Option::is_none")]
    pub spcl_ex_dt: Option<DateFormat4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat1ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(
        rename = "AmtPricPerFinInstrmQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub amt_pric_per_fin_instrm_qty: Option<AmountPricePerFinancialInstrumentQuantity1>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountPrice1>,
    #[serde(rename = "NotSpcfd", skip_serializing_if = "Option::is_none")]
    pub not_spcfd: Option<PriceValueType6FormatChoice>,
    #[serde(rename = "AmtPricPerAmt", skip_serializing_if = "Option::is_none")]
    pub amt_pric_per_amt: Option<AmountPricePerAmount1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat1Choice {
    #[serde(flatten)]
    pub value: PriceFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingPosition2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingPosition2FormatChoice {
    #[serde(flatten)]
    pub value: ProcessingPosition2FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RenounceableStatus1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RenounceableStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RenounceableStatus1FormatChoice {
    #[serde(flatten)]
    pub value: RenounceableStatus1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNarrative2 {
    #[serde(rename = "InfConds", skip_serializing_if = "Option::is_none")]
    pub inf_conds: Option<Max350Text>,
    #[serde(rename = "InfToCmplyWth", skip_serializing_if = "Option::is_none")]
    pub inf_to_cmply_wth: Option<Max350Text>,
    #[serde(rename = "TaxtnConds", skip_serializing_if = "Option::is_none")]
    pub taxtn_conds: Option<Max350Text>,
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
    #[serde(rename = "AddtlTxt", skip_serializing_if = "Option::is_none")]
    pub addtl_txt: Option<Max350Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionCalculationMethod1Code {
    #[serde(rename = "PROR")]
    Pror,
    #[serde(rename = "LOTT")]
    Lott,
    #[serde(rename = "NOMI")]
    Nomi,
    #[serde(rename = "NNOM")]
    Nnom,
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
#[derive(
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
pub struct PriceRateType3FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PriceRateType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateType3FormatChoice {
    #[serde(flatten)]
    pub value: PriceRateType3FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType1FormatChoice {
    #[serde(flatten)]
    pub value: FractionDispositionType1FormatChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionChangeType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionChangeType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionChangeType1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionChangeType1FormatChoiceEnum,
}
#[derive(
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
pub struct AgentCaNotificationAdviceV01 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: DocumentIdentification8,
    #[validate]
    #[serde(rename = "NtfctnTpAndLkg")]
    pub ntfctn_tp_and_lkg: LinkedCorporateAction1,
    #[validate]
    #[serde(rename = "NtfctnGnlInf")]
    pub ntfctn_gnl_inf: CorporateActionNotification1,
    #[validate]
    #[serde(rename = "CorpActnGnlInf")]
    pub corp_actn_gnl_inf: CorporateActionInformation2,
    #[validate]
    #[serde(rename = "CorpActnDtls")]
    pub corp_actn_dtls: CorporateAction2,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpActnOptnDtls", default)]
    pub corp_actn_optn_dtls: Vec<CorporateActionOption1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtctDtls", default)]
    pub ctct_dtls: Vec<ContactPerson1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndRateFormat2ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
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
pub struct AmountAndRateFormat2Choice {
    #[serde(flatten)]
    pub value: AmountAndRateFormat2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventStage1Code {
    #[serde(rename = "APPD")]
    Appd,
    #[serde(rename = "CLDE")]
    Clde,
    #[serde(rename = "PWAL")]
    Pwal,
    #[serde(rename = "SUAP")]
    Suap,
    #[serde(rename = "UNAC")]
    Unac,
    #[serde(rename = "WHOU")]
    Whou,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "RESC")]
    Resc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionOptionType1Code {
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
    #[serde(rename = "CTEN")]
    Cten,
    #[serde(rename = "CONN")]
    Conn,
    #[serde(rename = "CONY")]
    Cony,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "MPUT")]
    Mput,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SLLE")]
    Slle,
    #[serde(rename = "SPLI")]
    Spli,
    #[serde(rename = "NOQU")]
    Noqu,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "QINV")]
    Qinv,
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
pub struct AmountToAmountRatio1 {
    #[validate]
    #[serde(rename = "Amt1")]
    pub amt_1: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "Amt2")]
    pub amt_2: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventType2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventType2FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventType2FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NetDividendRateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetDividendRateType1FormatChoice {
    #[serde(flatten)]
    pub value: NetDividendRateType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GrossDividendRateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateType1FormatChoice {
    #[serde(flatten)]
    pub value: GrossDividendRateType1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OfferType1Code {
    #[serde(rename = "DISS")]
    Diss,
    #[serde(rename = "ERUN")]
    Erun,
    #[serde(rename = "FCFS")]
    Fcfs,
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "MINI")]
    Mini,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SQUE")]
    Sque,
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
pub struct AmountPricePerAmount1 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1FormatChoice,
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
pub struct CorporateActionPrice4 {
    #[serde(rename = "IndctvPric", skip_serializing_if = "Option::is_none")]
    pub indctv_pric: Option<PriceFormat2Choice>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<PriceFormat2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LotteryType1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
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
pub struct LotteryType1FormatChoice {
    #[serde(flatten)]
    pub value: LotteryType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndQuantityRatio1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
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
pub struct OfferType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OfferType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OfferType1FormatChoice {
    #[serde(flatten)]
    pub value: OfferType1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionFeatures1Code {
    #[serde(rename = "COND")]
    Cond,
    #[serde(rename = "MAXC")]
    Maxc,
    #[serde(rename = "MAXS")]
    Maxs,
    #[serde(rename = "OPLF")]
    Oplf,
    #[serde(rename = "PROR")]
    Pror,
    #[serde(rename = "QOVE")]
    Qove,
    #[serde(rename = "QREC")]
    Qrec,
    #[serde(rename = "VVPR")]
    Vvpr,
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
pub struct CorporateActionAgent1 {
    #[serde(rename = "AgtId")]
    pub agt_id: PartyIdentification2Choice,
    #[serde(rename = "AgtRole")]
    pub agt_role: AgentRole1FormatChoice,
    #[serde(rename = "CtctPrsn", skip_serializing_if = "Option::is_none")]
    pub ctct_prsn: Option<NameAndAddress5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period1 {
    #[serde(rename = "StartDt")]
    pub start_dt: DateFormat4Choice,
    #[serde(rename = "EndDt")]
    pub end_dt: DateFormat4Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateValueType2Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "NILP")]
    Nilp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GrossDividendRateType1Code {
    #[serde(rename = "CAPO")]
    Capo,
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
    #[serde(rename = "LTCG")]
    Ltcg,
    #[serde(rename = "STCG")]
    Stcg,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNarrative1 {
    #[serde(rename = "InfConds", skip_serializing_if = "Option::is_none")]
    pub inf_conds: Option<Max350Text>,
    #[serde(rename = "InfToCmplyWth", skip_serializing_if = "Option::is_none")]
    pub inf_to_cmply_wth: Option<Max350Text>,
    #[serde(rename = "TaxtnConds", skip_serializing_if = "Option::is_none")]
    pub taxtn_conds: Option<Max350Text>,
    #[serde(rename = "NewCpnyNm", skip_serializing_if = "Option::is_none")]
    pub new_cpny_nm: Option<Max350Text>,
    #[serde(rename = "Offerr", skip_serializing_if = "Option::is_none")]
    pub offerr: Option<PartyIdentification2Choice>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "AddtlTxt", skip_serializing_if = "Option::is_none")]
    pub addtl_txt: Option<Max350Text>,
}
#[derive(
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
pub struct FinancialInstrumentDescription3 {
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification7,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MicIdentifier>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<PartyIdentification2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionCalculationMethod1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionCalculationMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionCalculationMethod1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionCalculationMethod1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventType2Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "ATTI")]
    Atti,
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "BPUT")]
    Bput,
    #[serde(rename = "BRUP")]
    Brup,
    #[serde(rename = "CAPG")]
    Capg,
    #[serde(rename = "CAPI")]
    Capi,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "CHAN")]
    Chan,
    #[serde(rename = "CLSA")]
    Clsa,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "DETI")]
    Deti,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "DLST")]
    Dlst,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DSCL")]
    Dscl,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "EXTM")]
    Extm,
    #[serde(rename = "EXWA")]
    Exwa,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "MCAL")]
    Mcal,
    #[serde(rename = "MRGR")]
    Mrgr,
    #[serde(rename = "ODLT")]
    Odlt,
    #[serde(rename = "PARI")]
    Pari,
    #[serde(rename = "PCAL")]
    Pcal,
    #[serde(rename = "PDEF")]
    Pdef,
    #[serde(rename = "PINK")]
    Pink,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PPMT")]
    Ppmt,
    #[serde(rename = "PRED")]
    Pred,
    #[serde(rename = "PRII")]
    Prii,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REDO")]
    Redo,
    #[serde(rename = "REMK")]
    Remk,
    #[serde(rename = "RHDI")]
    Rhdi,
    #[serde(rename = "RHTS")]
    Rhts,
    #[serde(rename = "SHPR")]
    Shpr,
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "SPLR")]
    Splr,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "TREC")]
    Trec,
    #[serde(rename = "WRTH")]
    Wrth,
    #[serde(rename = "WTRC")]
    Wtrc,
    #[serde(rename = "OTHR")]
    Othr,
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventStatus2Code {
    #[serde(rename = "ACTI")]
    Acti,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INAC")]
    Inac,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateValueType6Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "OPEN")]
    Open,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IntermediateSecurityDistributionType1Code {
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "BONU")]
    Bonu,
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
pub struct CorporateActionNotification1 {
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<DateFormat4Choice>,
    #[serde(rename = "FrthrDtldAnncmntDt", skip_serializing_if = "Option::is_none")]
    pub frthr_dtld_anncmnt_dt: Option<DateFormat4Choice>,
    #[serde(
        rename = "OffclAnncmntPblctnDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub offcl_anncmnt_pblctn_dt: Option<DateFormat4Choice>,
    #[serde(rename = "PrcgSts")]
    pub prcg_sts: ProcessingStatus1FormatChoice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndRateFormat3ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType6FormatChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndRateFormat3Choice {
    #[serde(flatten)]
    pub value: AmountAndRateFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceValueType6FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PriceValueType6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceValueType6FormatChoice {
    #[serde(flatten)]
    pub value: PriceValueType6FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateValueType2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateValueType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateValueType2FormatChoice {
    #[serde(flatten)]
    pub value: RateValueType2FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPrice1 {
    #[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
    pub exrc_pric: Option<PriceFormat4Choice>,
    #[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
    pub isse_pric: Option<PriceFormat2Choice>,
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat2Choice>,
    #[serde(
        rename = "TaxblIncmPerDvddShr",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_dvdd_shr: Option<AmountPrice1>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat1Choice>,
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat2Choice>,
    #[serde(rename = "OverSbcptDpstPric", skip_serializing_if = "Option::is_none")]
    pub over_sbcpt_dpst_pric: Option<PriceFormat2Choice>,
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
pub struct RatioFormat1ChoiceEnum {
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio1>,
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat1Choice {
    #[serde(flatten)]
    pub value: RatioFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RelatedTaxType1 {
    #[serde(rename = "TaxTp")]
    pub tax_tp: TaxType3FormatChoice,
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
pub struct SecurityIdentification7Enum {
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<AlternateSecurityIdentification3>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification7 {
    #[serde(flatten)]
    pub value: SecurityIdentification7Enum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxType3Code {
    #[serde(rename = "LIDT")]
    Lidt,
    #[serde(rename = "WITF")]
    Witf,
    #[serde(rename = "WITL")]
    Witl,
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
pub struct ContactIdentification4 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max350Text>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Max35Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
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
pub struct AmountPricePerFinancialInstrumentQuantity1 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1FormatChoice,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "FinInstrmQty")]
    pub fin_instrm_qty: UnitOrFaceAmount1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IntermediateSecurityDistributionType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<IntermediateSecurityDistributionType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IntermediateSecurityDistributionType1FormatChoice {
    #[serde(flatten)]
    pub value: IntermediateSecurityDistributionType1FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BeneficiaryCertificationType1Code {
    #[serde(rename = "ACCI")]
    Acci,
    #[serde(rename = "DOMI")]
    Domi,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "QIBB")]
    Qibb,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "NCOM")]
    Ncom,
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
pub struct CorporateActionPrice2 {
    #[serde(rename = "MaxPric", skip_serializing_if = "Option::is_none")]
    pub max_pric: Option<PriceFormat3Choice>,
    #[serde(rename = "MinPric", skip_serializing_if = "Option::is_none")]
    pub min_pric: Option<PriceFormat3Choice>,
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
pub struct TaxType3FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType3FormatChoice {
    #[serde(flatten)]
    pub value: TaxType3FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionFrequencyType1Code {
    #[serde(rename = "FINL")]
    Finl,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "REGR")]
    Regr,
    #[serde(rename = "SPEC")]
    Spec,
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
pub struct PriceFormat2ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PriceRate1>,
    #[serde(rename = "NotSpcfd", skip_serializing_if = "Option::is_none")]
    pub not_spcfd: Option<PriceValueType5FormatChoice>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountPrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat2Choice {
    #[serde(flatten)]
    pub value: PriceFormat2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ShareRanking1Code {
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "PARI")]
    Pari,
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
pub struct CorporateActionRate1 {
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<RateAndAmountFormat1Choice>,
    #[serde(rename = "RltdIndx", skip_serializing_if = "Option::is_none")]
    pub rltd_indx: Option<RateFormat1Choice>,
    #[serde(rename = "PctgSght", skip_serializing_if = "Option::is_none")]
    pub pctg_sght: Option<RateFormat1Choice>,
    #[serde(rename = "RinvstmtDscntToMkt", skip_serializing_if = "Option::is_none")]
    pub rinvstmt_dscnt_to_mkt: Option<RateFormat1Choice>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<RateFormat1Choice>,
    #[serde(rename = "BidIntrvl", skip_serializing_if = "Option::is_none")]
    pub bid_intrvl: Option<AmountAndRateFormat3Choice>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<RateAndAmountFormat1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetDividendRateType1Code {
    #[serde(rename = "CAPO")]
    Capo,
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
pub struct RateAndAmountFormat1ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
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
pub struct RateAndAmountFormat1Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStatus2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStatus2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStatus2FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventStatus2FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConversionType1Code {
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
pub struct PriceValueType5FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PriceValueType5Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceValueType5FormatChoice {
    #[serde(flatten)]
    pub value: PriceValueType5FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType12Code {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "NILP")]
    Nilp,
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
pub struct GrossDividendRate1ChoiceEnum {
    #[serde(rename = "RateTpAmt", skip_serializing_if = "Option::is_none")]
    pub rate_tp_amt: Option<GrossDividendRate2>,
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateValueType2FormatChoice>,
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
pub struct GrossDividendRate1Choice {
    #[serde(flatten)]
    pub value: GrossDividendRate1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LinkedCorporateAction1 {
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: CorporateActionNotificationType1Code,
    #[serde(
        rename = "LkdAgtCANtfctnAdvcId",
        skip_serializing_if = "Option::is_none"
    )]
    pub lkd_agt_ca_ntfctn_advc_id: Option<DocumentIdentification8>,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition2FormatChoice>,
    #[serde(rename = "LkdIssrCorpActnId", skip_serializing_if = "Option::is_none")]
    pub lkd_issr_corp_actn_id: Option<Max35Text>,
    #[serde(rename = "LkdCorpActnPrcgId", skip_serializing_if = "Option::is_none")]
    pub lkd_corp_actn_prcg_id: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionType1Code {
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
pub struct GenericIdentification13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
}
