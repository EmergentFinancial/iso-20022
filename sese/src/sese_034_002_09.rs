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
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_16_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FIN_EXACT_2_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"XX|TS"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_30_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_210_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,210}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_140_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,140}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_350_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,350}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_31_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,31}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_35_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_52_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,52}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_70_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,70}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_8_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_24_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.034.002.09".to_string()
}

#[derive(
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
pub struct PendingProcessingStatus14ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingProcessingReason11>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus14Choice {
    #[serde(flatten)]
    pub value: PendingProcessingStatus14ChoiceEnum,
}
#[derive(
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
pub struct RateType67ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateType1Code>,
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
pub struct RateType67Choice {
    #[serde(flatten)]
    pub value: RateType67ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText15>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification85>,
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
pub struct PendingReason38ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason1Code>,
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
pub struct PendingReason38Choice {
    #[serde(flatten)]
    pub value: PendingReason38ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate32ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<SettlementDateCode11Choice>,
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
pub struct SettlementDate32Choice {
    #[serde(flatten)]
    pub value: SettlementDate32ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus27ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason15>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus27Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus27ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDetails173 {
    #[serde(rename = "HldInd", skip_serializing_if = "Option::is_none")]
    pub hld_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition22Choice>,
    #[serde(rename = "SttlgCpcty", skip_serializing_if = "Option::is_none")]
    pub sttlg_cpcty: Option<SettlingCapacity8Choice>,
    #[serde(rename = "StmpDtyTaxBsis", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_tax_bsis: Option<GenericIdentification47>,
    #[serde(rename = "SctiesRTGS", skip_serializing_if = "Option::is_none")]
    pub scties_rtgs: Option<SecuritiesRtgs5Choice>,
    #[serde(rename = "BnfclOwnrsh", skip_serializing_if = "Option::is_none")]
    pub bnfcl_ownrsh: Option<BeneficialOwnership5Choice>,
    #[serde(rename = "CshClrSys", skip_serializing_if = "Option::is_none")]
    pub csh_clr_sys: Option<CashSettlementSystem5Choice>,
    #[serde(rename = "TaxCpcty", skip_serializing_if = "Option::is_none")]
    pub tax_cpcty: Option<TaxCapacityParty5Choice>,
    #[serde(rename = "MktClntSd", skip_serializing_if = "Option::is_none")]
    pub mkt_clnt_sd: Option<MarketClientSide7Choice>,
    #[serde(rename = "BlckTrad", skip_serializing_if = "Option::is_none")]
    pub blck_trad: Option<BlockTrade5Choice>,
    #[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctns: Option<Restriction6Choice>,
    #[serde(rename = "SttlmSysMtd", skip_serializing_if = "Option::is_none")]
    pub sttlm_sys_mtd: Option<SettlementSystemMethod5Choice>,
    #[serde(rename = "NetgElgblty", skip_serializing_if = "Option::is_none")]
    pub netg_elgblty: Option<NettingEligibility5Choice>,
    #[serde(rename = "CCPElgblty", skip_serializing_if = "Option::is_none")]
    pub ccp_elgblty: Option<CentralCounterPartyEligibility5Choice>,
    #[serde(rename = "PrtlSttlmInd", skip_serializing_if = "Option::is_none")]
    pub prtl_sttlm_ind: Option<SettlementTransactionCondition5Code>,
    #[serde(rename = "ElgblForColl", skip_serializing_if = "Option::is_none")]
    pub elgbl_for_coll: Option<YesNoIndicator>,
}
#[derive(
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
pub struct SecuritiesRtgs5ChoiceEnum {
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
pub struct SecuritiesRtgs5Choice {
    #[serde(flatten)]
    pub value: SecuritiesRtgs5ChoiceEnum,
}
#[derive(
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
pub struct DateCode32ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType5Code>,
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
pub struct DateCode32Choice {
    #[serde(flatten)]
    pub value: DateCode32ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<RestrictedFinDecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<RestrictedFinImpliedCurrencyAndAmount>,
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
pub struct AcknowledgedAcceptedStatus25ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason13>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus25Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus25ChoiceEnum,
}
#[derive(
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
pub struct MarketType16ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
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
pub struct Rate2 {
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
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
pub struct PendingProcessingReason11 {
    #[serde(rename = "Cd")]
    pub cd: PendingProcessingReason13Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct SecuritiesFinancingTransactionDetails53 {
    #[serde(rename = "SctiesFincgTradId", skip_serializing_if = "Option::is_none")]
    pub scties_fincg_trad_id: Option<RestrictedFinxMax52Text>,
    #[serde(rename = "ClsgLegId", skip_serializing_if = "Option::is_none")]
    pub clsg_leg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_evt_id: Option<RestrictedFinxMax16Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrCollTxId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_coll_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "ClntTrptyCollTxId", skip_serializing_if = "Option::is_none")]
    pub clnt_trpty_coll_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount30>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafeKeepingPlace4>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification2>,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification20,
    #[serde(rename = "SttlmQty")]
    pub sttlm_qty: Quantity54Choice,
    #[serde(rename = "OpngSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub opng_sttlm_amt: Option<AmountAndDirection67>,
    #[serde(rename = "TermntnTxAmt", skip_serializing_if = "Option::is_none")]
    pub termntn_tx_amt: Option<AmountAndDirection59>,
    #[serde(rename = "OpngSttlmDt")]
    pub opng_sttlm_dt: SettlementDate32Choice,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<TerminationDate7Choice>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate9Choice>,
    #[serde(rename = "XpctdSttlmDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_sttlm_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_val_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "LateDlvryDt", skip_serializing_if = "Option::is_none")]
    pub late_dlvry_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "RateChngDt", skip_serializing_if = "Option::is_none")]
    pub rate_chng_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "SctiesFincgTxTp")]
    pub scties_fincg_tx_tp: SecuritiesFinancingTransactionType2Code,
    #[serde(rename = "SctiesMvmntTp")]
    pub scties_mvmnt_tp: ReceiveDelivery1Code,
    #[serde(rename = "Pmt")]
    pub pmt: DeliveryReceiptType2Code,
    #[serde(rename = "SttlmParams", skip_serializing_if = "Option::is_none")]
    pub sttlm_params: Option<SettlementDetails173>,
    #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<RateType67Choice>,
    #[serde(rename = "VarblRateSpprt", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_spprt: Option<RateName2>,
    #[serde(rename = "RpRate", skip_serializing_if = "Option::is_none")]
    pub rp_rate: Option<Rate2>,
    #[serde(rename = "StockLnMrgn", skip_serializing_if = "Option::is_none")]
    pub stock_ln_mrgn: Option<Rate2>,
    #[serde(rename = "SctiesHrcut", skip_serializing_if = "Option::is_none")]
    pub scties_hrcut: Option<Rate2>,
    #[serde(rename = "PricgRate", skip_serializing_if = "Option::is_none")]
    pub pricg_rate: Option<RateOrName2Choice>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<Rate2>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties107>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties107>,
    #[serde(rename = "Invstr", skip_serializing_if = "Option::is_none")]
    pub invstr: Option<PartyIdentification170>,
    #[serde(
        rename = "SttlmInstrPrcgAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub sttlm_instr_prcg_addtl_dtls: Option<RestrictedFinxMax350Text>,
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
pub struct PendingReason21 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason38Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct RejectionReason46ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason70Code>,
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
pub struct RejectionReason46Choice {
    #[serde(flatten)]
    pub value: RejectionReason46ChoiceEnum,
}
#[derive(
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
pub struct BlockTrade5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BlockTrade1Code>,
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
pub struct BlockTrade5Choice {
    #[serde(flatten)]
    pub value: BlockTrade5ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeniedReason3Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DCAL")]
    Dcal,
    #[serde(rename = "DFOR")]
    Dfor,
    #[serde(rename = "LATE")]
    Late,
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason9Code {
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "CANS")]
    Cans,
    #[serde(rename = "CSUB")]
    Csub,
    #[serde(rename = "CXLR")]
    Cxlr,
    #[serde(rename = "CANT")]
    Cant,
    #[serde(rename = "CANZ")]
    Canz,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "SCEX")]
    Scex,
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
pub struct PendingReason20 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason37Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct DeniedReason18ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeniedReason3Code>,
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
pub struct DeniedReason18Choice {
    #[serde(flatten)]
    pub value: DeniedReason18ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress12>,
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
#[derive(
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
pub struct FailingReason10 {
    #[serde(rename = "Cd")]
    pub cd: FailingReason11Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementSystemMethod5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementSystemMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementSystemMethod5Choice {
    #[serde(flatten)]
    pub value: SettlementSystemMethod5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType4Code {
    #[serde(rename = "OPEN")]
    Open,
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
pub enum PendingProcessingReason1Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "CERT")]
    Cert,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PartyIdentificationAndAccount213 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification137Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification9>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount37>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace3Code {
    #[serde(rename = "SHHE")]
    Shhe,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType5Code {
    #[serde(rename = "OPEN")]
    Open,
    #[default]
    Unknown,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
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
pub struct MatchingStatus29ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
    pub umtchd: Option<UnmatchedStatus19Choice>,
    #[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
    pub mtchd: Option<ProprietaryReason5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus29Choice {
    #[serde(flatten)]
    pub value: MatchingStatus29ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoCallAcknowledgementReason2Code {
    #[serde(rename = "CALD")]
    Cald,
    #[serde(rename = "CALP")]
    Calp,
    #[serde(rename = "ADEA")]
    Adea,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionStatus40ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionReason64>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionStatus40Choice {
    #[serde(flatten)]
    pub value: RejectionStatus40ChoiceEnum,
}
#[derive(
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
pub struct AcknowledgementReason18ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepoCallAcknowledgementReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason18Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason18ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason1Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "TAMM")]
    Tamm,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BlockTrade1Code {
    #[serde(rename = "BLPA")]
    Blpa,
    #[serde(rename = "BLCH")]
    Blch,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedReason13 {
    #[serde(rename = "Cd")]
    pub cd: DeniedReason18Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct SettlementDateCode11ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType4Code>,
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
pub struct SettlementDateCode11Choice {
    #[serde(flatten)]
    pub value: SettlementDateCode11ChoiceEnum,
}
#[derive(
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
pub struct SettlementStatus21ChoiceEnum {
    #[serde(rename = "Flng", skip_serializing_if = "Option::is_none")]
    pub flng: Option<FailingStatus12Choice>,
    #[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
    pub pdg: Option<PendingStatus47Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStatus21Choice {
    #[serde(flatten)]
    pub value: SettlementStatus21ChoiceEnum,
}
#[derive(
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
pub enum AcknowledgementReason5Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "RQWV")]
    Rqwv,
    #[serde(rename = "LATE")]
    Late,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection59 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: RestrictedFinActiveOrHistoricCurrencyAndAmount,
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
pub struct ProprietaryReason5 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<GenericIdentification47>,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason25ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason9Code>,
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
pub struct CancellationReason25Choice {
    #[serde(flatten)]
    pub value: CancellationReason25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason11 {
    #[serde(rename = "Cd")]
    pub cd: RepairReason13Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
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
pub struct RestrictedFinxMax31Text {
    #[validate(
        length(min = 1, max = 31,),
        regex = "RESTRICTED_FINX_MAX_31_TEXT_REGEX"
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
#[derive(
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
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashSettlementSystem2Code>,
}
#[derive(
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
#[derive(
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
pub struct ProcessingStatus90ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "ModReqd", skip_serializing_if = "Option::is_none")]
    pub mod_reqd: Option<ProprietaryReason5>,
    #[serde(rename = "Rpr", skip_serializing_if = "Option::is_none")]
    pub rpr: Option<RepairStatus15Choice>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus25Choice>,
    #[serde(rename = "CxlReqd", skip_serializing_if = "Option::is_none")]
    pub cxl_reqd: Option<ProprietaryReason5>,
    #[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg: Option<PendingProcessingStatus14Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectionStatus40Choice>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancellationStatus18Choice>,
    #[serde(rename = "PdgCxl", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl: Option<PendingStatus46Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus90Choice {
    #[serde(flatten)]
    pub value: ProcessingStatus90ChoiceEnum,
}
#[derive(
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
pub struct RateOrName2ChoiceEnum {
    #[serde(rename = "RateNm", skip_serializing_if = "Option::is_none")]
    pub rate_nm: Option<RateName2>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrName2Choice {
    #[serde(flatten)]
    pub value: RateOrName2ChoiceEnum,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "SctiesFincgStsAdvc")]
    pub scties_fincg_sts_advc: SecuritiesFinancingStatusAdvice002V09<A>,
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
pub struct FailingReason11ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FailingReason2Code>,
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
pub struct FailingReason11Choice {
    #[serde(flatten)]
    pub value: FailingReason11ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition6Code {
    #[serde(rename = "ASGN")]
    Asgn,
    #[serde(rename = "BUTC")]
    Butc,
    #[serde(rename = "CLEN")]
    Clen,
    #[serde(rename = "DIRT")]
    Dirt,
    #[serde(rename = "DLWM")]
    Dlwm,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "FRCL")]
    Frcl,
    #[serde(rename = "KNOC")]
    Knoc,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "RESI")]
    Resi,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "SPDL")]
    Spdl,
    #[serde(rename = "SPST")]
    Spst,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "PENS")]
    Pens,
    #[serde(rename = "UNEX")]
    Unex,
    #[serde(rename = "TRIP")]
    Trip,
    #[serde(rename = "NOMC")]
    Nomc,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "RHYP")]
    Rhyp,
    #[serde(rename = "ADEA")]
    Adea,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus12ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<FailingReason10>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus12Choice {
    #[serde(flatten)]
    pub value: FailingStatus12ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FailingReason2Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CANR")]
    Canr,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "OBJT")]
    Objt,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "STCD")]
    Stcd,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "MLAT")]
    Mlat,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "INBC")]
    Inbc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason13ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepairReason5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason13Choice {
    #[serde(flatten)]
    pub value: RepairReason13ChoiceEnum,
}
#[derive(
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
pub struct RepairStatus15ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RepairReason11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairStatus15Choice {
    #[serde(flatten)]
    pub value: RepairStatus15ChoiceEnum,
}
#[derive(
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
pub struct PendingStatus47ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason21>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus47Choice {
    #[serde(flatten)]
    pub value: PendingStatus47ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason25ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnmatchedReason13Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason25Choice {
    #[serde(flatten)]
    pub value: UnmatchedReason25ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason70Code {
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "PLIS")]
    Plis,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType1Code {
    #[serde(rename = "FIXE")]
    Fixe,
    #[serde(rename = "FORF")]
    Forf,
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
pub struct CancellationReason15 {
    #[serde(rename = "Cd")]
    pub cd: CancellationReason25Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
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
pub struct RepoCallRequestStatus9ChoiceEnum {
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus27Choice>,
    #[serde(rename = "Dnd", skip_serializing_if = "Option::is_none")]
    pub dnd: Option<DeniedStatus18Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepoCallRequestStatus9Choice {
    #[serde(flatten)]
    pub value: RepoCallRequestStatus9ChoiceEnum,
}
#[derive(
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
pub struct DeniedStatus18ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<DeniedReason13>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedStatus18Choice {
    #[serde(flatten)]
    pub value: DeniedStatus18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition22ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition6Code>,
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
pub struct SettlementTransactionCondition22Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition22ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnmatchedReason13Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "FORF")]
    Forf,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "DTRA")]
    Dtra,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "FRAP")]
    Frap,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLIS")]
    Plis,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "REGD")]
    Regd,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "DMCT")]
    Dmct,
    #[serde(rename = "DCMX")]
    Dcmx,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepairReason5Code {
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "REPA")]
    Repa,
    #[serde(rename = "CADE")]
    Cade,
    #[serde(rename = "RERT")]
    Rert,
    #[serde(rename = "RSPR")]
    Rspr,
    #[serde(rename = "VASU")]
    Vasu,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REPP")]
    Repp,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "FORF")]
    Forf,
    #[default]
    Unknown,
}
#[derive(
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
pub struct AcknowledgementReason15 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason18Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct RestrictedFinxMax8Text {
    #[validate(length(min = 1, max = 8,), regex = "RESTRICTED_FINX_MAX_8_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
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
pub struct Restriction6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OwnershipLegalRestrictions1Code>,
}
#[derive(
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
pub struct RejectionReason64 {
    #[serde(rename = "Cd")]
    pub cd: RejectionReason46Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
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
pub struct MarketClientSide7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketClientSide1Code>,
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
pub struct AcknowledgementReason16ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason16Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason16ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason13ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingProcessingReason1Code>,
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
pub struct PendingProcessingReason13Choice {
    #[serde(flatten)]
    pub value: PendingProcessingReason13ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus46ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason20>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus46Choice {
    #[serde(flatten)]
    pub value: PendingStatus46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax24Text {
    #[validate(
        length(min = 1, max = 24,),
        regex = "RESTRICTED_FINX_MAX_24_TEXT_REGEX"
    )]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementSystemMethod1Code {
    #[serde(rename = "NSET")]
    Nset,
    #[serde(rename = "YSET")]
    Yset,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ProprietaryStatusAndReason7 {
    #[validate]
    #[serde(rename = "PrtrySts")]
    pub prtry_sts: GenericIdentification47,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtryRsn", default)]
    pub prtry_rsn: Vec<ProprietaryReason5>,
}
#[derive(
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
pub enum SettlementTransactionCondition5Code {
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NPAR")]
    Npar,
    #[serde(rename = "PARC")]
    Parc,
    #[serde(rename = "PARQ")]
    Parq,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesFinancingTransactionType2Code {
    #[serde(rename = "REPU")]
    Repu,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SECB")]
    Secb,
    #[serde(rename = "SECL")]
    Secl,
    #[serde(rename = "BSBK")]
    Bsbk,
    #[serde(rename = "SBBK")]
    Sbbk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionIdentifications35 {
    #[validate]
    #[serde(rename = "AcctOwnrTxId")]
    pub acct_ownr_tx_id: RestrictedFinxMax16Text,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedStatus19ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<UnmatchedReason18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedStatus19Choice {
    #[serde(flatten)]
    pub value: UnmatchedStatus19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason18 {
    #[serde(rename = "Cd")]
    pub cd: UnmatchedReason25Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
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
pub struct AcknowledgementReason13 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason16Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
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
pub struct PendingReason37ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason37Choice {
    #[serde(flatten)]
    pub value: PendingReason37ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationStatus18ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancellationReason15>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationStatus18Choice {
    #[serde(flatten)]
    pub value: CancellationStatus18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount37 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification47>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesFinancingStatusAdvice002V09<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentifications35,
    #[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
    pub prcg_sts: Option<ProcessingStatus90Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus29Choice>,
    #[serde(rename = "IfrrdMtchgSts", skip_serializing_if = "Option::is_none")]
    pub ifrrd_mtchg_sts: Option<MatchingStatus29Choice>,
    #[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
    pub sttlm_sts: Option<SettlementStatus21Choice>,
    #[serde(rename = "RepoCallReqSts", skip_serializing_if = "Option::is_none")]
    pub repo_call_req_sts: Option<RepoCallRequestStatus9Choice>,
    #[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
    pub tx_dtls: Option<SecuritiesFinancingTransactionDetails53>,
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
pub struct SettlementParties107 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification162>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount213>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount213>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount213>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount213>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount213>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TerminationDate7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateCode32Choice>,
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
pub struct TerminationDate7Choice {
    #[serde(flatten)]
    pub value: TerminationDate7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateName2 {
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<RestrictedFinxMax8Text>,
    #[validate]
    #[serde(rename = "RateNm")]
    pub rate_nm: RestrictedFinxMax24Text,
}
