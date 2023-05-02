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
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IN[a-zA-Z0-9]{11,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"GR[0-9]{7,7}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.011.001.09".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransferInstructionStatusReportV09 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_ref: Option<AdditionalReference10>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<References64Choice>,
    #[validate]
    #[serde(rename = "StsRpt")]
    pub sts_rpt: TransferStatusAndReason8,
    #[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
    pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
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
pub struct Unit11 {
    #[validate]
    #[serde(rename = "UnitsNb")]
    pub units_nb: DecimalNumber,
    #[serde(rename = "OrdrDt", skip_serializing_if = "Option::is_none")]
    pub ordr_dt: Option<IsoDate>,
    #[serde(rename = "AcqstnDt", skip_serializing_if = "Option::is_none")]
    pub acqstn_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<Max35Text>,
    #[serde(rename = "Grp1Or2Units", skip_serializing_if = "Option::is_none")]
    pub grp_1_or_2_units: Option<UkTaxGroupUnit1Code>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<UnitPrice23>,
    #[serde(rename = "TxOvrhd", skip_serializing_if = "Option::is_none")]
    pub tx_ovrhd: Option<TotalFeesAndTaxes41>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmt", default)]
    pub othr_amt: Vec<OtherAmount1>,
}
#[derive(
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
pub struct TransferStatusAndReason8 {
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TrfRef")]
    pub trf_ref: Max35Text,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<AdditionalReference10>,
    #[serde(rename = "CxlRef", skip_serializing_if = "Option::is_none")]
    pub cxl_ref: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TrfEvtTp", default)]
    pub trf_evt_tp: Vec<TransferStatusType3Choice>,
    #[serde(rename = "TrfSts")]
    pub trf_sts: TransferStatus5Choice,
    #[serde(rename = "Instrm", skip_serializing_if = "Option::is_none")]
    pub instrm: Option<FinancialInstrument63Choice>,
    #[serde(rename = "InvstmtAcctDtls", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_dtls: Option<Account33>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrmyInf", default)]
    pub intrmy_inf: Vec<Intermediary48>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<IsoDate>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<IsoDate>,
    #[serde(rename = "SndOutDt", skip_serializing_if = "Option::is_none")]
    pub snd_out_dt: Option<IsoDate>,
    #[serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none")]
    pub csh_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub ttl_units_nb: Option<DecimalNumber>,
    #[serde(rename = "AvrgPric", skip_serializing_if = "Option::is_none")]
    pub avrg_pric: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UnitsDtls", default)]
    pub units_dtls: Vec<Unit11>,
    #[serde(rename = "Convs", skip_serializing_if = "Option::is_none")]
    pub convs: Option<Conversion2>,
    #[serde(rename = "TtlTrfVal", skip_serializing_if = "Option::is_none")]
    pub ttl_trf_val: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtDtls", default)]
    pub pmt_dtls: Vec<PaymentInstrument18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BnftCrstllstnEvt", default)]
    pub bnft_crstllstn_evt: Vec<BenefitCrystallisationEvent2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DrwdwnTrch", default)]
    pub drwdwn_trch: Vec<Drawdown2>,
    #[serde(rename = "OthrDrwdwnInf", skip_serializing_if = "Option::is_none")]
    pub othr_drwdwn_inf: Option<Drawdown3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "QryRspn", default)]
    pub qry_rspn: Vec<Max350Text>,
    #[serde(rename = "StsInitr", skip_serializing_if = "Option::is_none")]
    pub sts_initr: Option<PartyIdentification139>,
    #[serde(rename = "StsIssr", skip_serializing_if = "Option::is_none")]
    pub sts_issr: Option<PartyIdentification139>,
    #[serde(rename = "StsRcpt", skip_serializing_if = "Option::is_none")]
    pub sts_rcpt: Option<PartyIdentification139>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TotalFeesAndTaxes41 {
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
    pub indv_fee: Vec<Fee5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvTax", default)]
    pub indv_tax: Vec<Tax35>,
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
pub struct BeneficiaryType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BeneficiaryType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryType1Choice {
    #[serde(flatten)]
    pub value: BeneficiaryType1ChoiceEnum,
}
#[derive(
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
pub struct DrawdownType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DrawdownType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DrawdownType2Choice {
    #[serde(flatten)]
    pub value: DrawdownType2ChoiceEnum,
}
#[derive(
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
pub struct ClearingSystemMemberIdentification2ChoiceEnum {
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "GRHEBIC", skip_serializing_if = "Option::is_none")]
    pub grhebic: Option<HellenicBankIdentificationCodeIdentifier>,
    #[serde(rename = "INIFSC", skip_serializing_if = "Option::is_none")]
    pub inifsc: Option<IndianFinancialSystemCodeIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "PLKNR", skip_serializing_if = "Option::is_none")]
    pub plknr: Option<PolishNationalClearingCodeIdentifier>,
    #[serde(rename = "OthrClrCdId", skip_serializing_if = "Option::is_none")]
    pub othr_clr_cd_id: Option<Max35Text>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
}
#[derive(
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
pub struct FinancialInstrumentIdentification2 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification17 {
    #[serde(rename = "Pty")]
    pub pty: FinancialInstitutionIdentification10Choice,
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
pub struct Unit13 {
    #[validate]
    #[serde(rename = "UnitsNb")]
    pub units_nb: DecimalNumber,
    #[serde(rename = "OrdrDt", skip_serializing_if = "Option::is_none")]
    pub ordr_dt: Option<IsoDate>,
    #[serde(rename = "AcqstnDt", skip_serializing_if = "Option::is_none")]
    pub acqstn_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<Max35Text>,
    #[serde(rename = "Grp1Or2Units", skip_serializing_if = "Option::is_none")]
    pub grp_1_or_2_units: Option<UkTaxGroupUnit1Code>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
}
#[derive(
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
#[derive(
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
pub struct Account34 {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "AcctSvcr")]
    pub acct_svcr: PartyIdentification132,
    #[serde(rename = "RegnAdr", skip_serializing_if = "Option::is_none")]
    pub regn_adr: Option<PostalAddress1>,
}
#[derive(
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
pub struct OtherAmount1 {
    #[serde(rename = "Tp")]
    pub tp: OtherAmountType1Choice,
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
pub struct References64ChoiceEnum {
    #[serde(rename = "OthrRef", skip_serializing_if = "Option::is_none")]
    pub othr_ref: Option<AdditionalReference10>,
    #[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ref: Option<AdditionalReference10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References64Choice {
    #[serde(flatten)]
    pub value: References64ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UkTaxGroupUnit1Code {
    #[serde(rename = "GRP1")]
    Grp1,
    #[serde(rename = "GRP2")]
    Grp2,
    #[default]
    Unknown,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "TrfInstrStsRpt")]
    pub trf_instr_sts_rpt: TransferInstructionStatusReportV09,
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
pub struct PriceValue1 {
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
pub struct CancelledStatus13ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancelledStatusReason3Code>,
    #[serde(rename = "XtndedRsn", skip_serializing_if = "Option::is_none")]
    pub xtnded_rsn: Option<Extended350Code>,
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledStatus13Choice {
    #[serde(flatten)]
    pub value: CancelledStatus13ChoiceEnum,
}
#[derive(
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
pub struct Conversion2 {
    #[validate]
    #[serde(rename = "SrcScty")]
    pub src_scty: FinancialInstrumentIdentification1,
    #[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub ttl_units_nb: Option<DecimalNumber>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UnitsDtls", default)]
    pub units_dtls: Vec<Unit13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Drawdown2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max140Text>,
    #[serde(rename = "TrchTp")]
    pub trch_tp: DrawdownType2Choice,
    #[serde(rename = "AplblRules", skip_serializing_if = "Option::is_none")]
    pub aplbl_rules: Option<ApplicableRules1Choice>,
    #[serde(rename = "InvstrTaxRef", skip_serializing_if = "Option::is_none")]
    pub invstr_tax_ref: Option<TaxReference2>,
    #[serde(rename = "PctgOfTtlTrfVal", skip_serializing_if = "Option::is_none")]
    pub pctg_of_ttl_trf_val: Option<PercentageRate>,
    #[serde(rename = "TtlAmtNetDrwdwn", skip_serializing_if = "Option::is_none")]
    pub ttl_amt_net_drwdwn: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AddtlFndsDsgntd", skip_serializing_if = "Option::is_none")]
    pub addtl_fnds_dsgntd: Option<YesNoIndicator>,
    #[serde(
        rename = "PnsnCmcmntLumpSumRmng",
        skip_serializing_if = "Option::is_none"
    )]
    pub pnsn_cmcmnt_lump_sum_rmng: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(
        rename = "PnsnCmcmntLumpSumDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub pnsn_cmcmnt_lump_sum_dt: Option<IsoDate>,
    #[serde(
        rename = "MltplPnsnCmcmntLumpSums",
        skip_serializing_if = "Option::is_none"
    )]
    pub mltpl_pnsn_cmcmnt_lump_sums: Option<YesNoIndicator>,
    #[serde(rename = "LftmAllwnc", skip_serializing_if = "Option::is_none")]
    pub lftm_allwnc: Option<PercentageRate>,
    #[serde(rename = "RcptOfDrwdwnInd", skip_serializing_if = "Option::is_none")]
    pub rcpt_of_drwdwn_ind: Option<YesNoIndicator>,
    #[serde(rename = "BnfcryDtls", skip_serializing_if = "Option::is_none")]
    pub bnfcry_dtls: Option<BeneficiaryDrawdown1>,
    #[serde(rename = "CapdLmts", skip_serializing_if = "Option::is_none")]
    pub capd_lmts: Option<Capped1>,
    #[serde(
        rename = "FlxblDrwdwnTrggrdDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub flxbl_drwdwn_trggrd_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason3Code {
    #[serde(rename = "CNTA")]
    Cnta,
    #[serde(rename = "CNCL")]
    Cncl,
    #[serde(rename = "CNIN")]
    Cnin,
    #[default]
    Unknown,
}
#[derive(
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
pub struct TaxBasis1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis2Code>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransferStatus5ChoiceEnum {
    #[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
    pub umtchd: Option<TransferUnmatchedStatus4Choice>,
    #[serde(rename = "FaildSttlm", skip_serializing_if = "Option::is_none")]
    pub faild_sttlm: Option<FailedSettlementStatus2Choice>,
    #[serde(rename = "CxlPdg", skip_serializing_if = "Option::is_none")]
    pub cxl_pdg: Option<CancellationPendingStatus7Choice>,
    #[serde(rename = "Rvsd", skip_serializing_if = "Option::is_none")]
    pub rvsd: Option<ReversedStatus2Choice>,
    #[serde(rename = "PdgSttlm", skip_serializing_if = "Option::is_none")]
    pub pdg_sttlm: Option<PendingSettlementStatus3Choice>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancelledStatus13Choice>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<TransferInstructionStatus5>,
    #[serde(rename = "InRpr", skip_serializing_if = "Option::is_none")]
    pub in_rpr: Option<InRepairStatus4Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectionReason56>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransferStatus5Choice {
    #[serde(flatten)]
    pub value: TransferStatus5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
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
pub struct AccountIdentificationAndName6 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BeneficiaryType1Code {
    #[serde(rename = "DEPE")]
    Depe,
    #[serde(rename = "NOMI")]
    Nomi,
    #[serde(rename = "SUCC")]
    Succ,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ApplicableRules1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ApplicableRules1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ApplicableRules1Choice {
    #[serde(flatten)]
    pub value: ApplicableRules1ChoiceEnum,
}
#[derive(
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
pub struct EmploymentDetails1 {
    #[serde(rename = "TaxCd", skip_serializing_if = "Option::is_none")]
    pub tax_cd: Option<GenericIdentification36>,
    #[serde(rename = "OthrTaxCdInd", skip_serializing_if = "Option::is_none")]
    pub othr_tax_cd_ind: Option<YesNoIndicator>,
    #[serde(rename = "CmltvTaxInd", skip_serializing_if = "Option::is_none")]
    pub cmltv_tax_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrvsPay", skip_serializing_if = "Option::is_none")]
    pub prvs_pay: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrvsTax", skip_serializing_if = "Option::is_none")]
    pub prvs_tax: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<DateFormat42Choice>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<DateFormat42Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationPendingStatus7ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max350Text>,
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
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
pub struct CancellationPendingStatus7Choice {
    #[serde(flatten)]
    pub value: CancellationPendingStatus7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryDrawdown1 {
    #[serde(rename = "BnfcryTp", skip_serializing_if = "Option::is_none")]
    pub bnfcry_tp: Option<BeneficiaryType1Choice>,
    #[serde(rename = "DthUdrLmt", skip_serializing_if = "Option::is_none")]
    pub dth_udr_lmt: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct CreditTransfer9 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentification132>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<AccountIdentificationAndName6>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<FinancialInstitutionIdentification16>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<AccountIdentificationAndName6>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_1: Option<FinancialInstitutionIdentification16>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_1_acct: Option<AccountIdentificationAndName6>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_2: Option<FinancialInstitutionIdentification16>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_2_acct: Option<AccountIdentificationAndName6>,
    #[validate]
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: FinancialInstitutionIdentification16,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<AccountIdentificationAndName6>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentification132>,
    #[validate]
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: AccountIdentificationAndName6,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Fee5 {
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
    pub rcpt_id: Option<PartyIdentification139>,
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
pub struct ContactIdentification2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OtherAmountType1Code {
    #[serde(rename = "PINT")]
    Pint,
    #[serde(rename = "SINT")]
    Sint,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason56 {
    #[serde(rename = "Rsn")]
    pub rsn: RejectedReason45Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max350Text>,
}
#[derive(
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
pub struct CashAsset3 {
    #[serde(rename = "CshAsstTp")]
    pub csh_asst_tp: CashAssetType1Choice,
    #[serde(rename = "HldgCcy")]
    pub hldg_ccy: ActiveCurrencyCode,
    #[serde(rename = "TrfCcy", skip_serializing_if = "Option::is_none")]
    pub trf_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Drawdown3 {
    #[serde(rename = "MplymntDtls", skip_serializing_if = "Option::is_none")]
    pub mplymnt_dtls: Option<EmploymentDetails1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct FinancialInstitutionIdentification16 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<Max35Text>,
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
pub struct PartyIdentification139 {
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification125Choice,
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
pub struct TickerIdentifier {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashAssetType1Code {
    #[serde(rename = "CSH2")]
    Csh2,
    #[serde(rename = "CSH1")]
    Csh1,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransferInstructionStatus5 {
    #[serde(rename = "Sts")]
    pub sts: TransferStatus6Code,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max350Text>,
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
pub struct TaxableIncomePerShareCalculated2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxableIncomePerShareCalculated2Code>,
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
pub struct TaxableIncomePerShareCalculated2Choice {
    #[serde(flatten)]
    pub value: TaxableIncomePerShareCalculated2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tax35 {
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
    pub rcpt_id: Option<PartyIdentification139>,
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
pub struct WaivingInstruction2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WaivingInstruction1Code>,
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
pub struct AdditionalReference10 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification139>,
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
pub struct SecurityIdentification25ChoiceEnum {
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
}
#[derive(
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
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransferStatus6Code {
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "COSE")]
    Cose,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "DELY")]
    Dely,
    #[serde(rename = "MACH")]
    Mach,
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "STNP")]
    Stnp,
    #[serde(rename = "SETT")]
    Sett,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransferStatusType2Code {
    #[serde(rename = "S019")]
    S019,
    #[serde(rename = "BCEV")]
    Bcev,
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "PAYA")]
    Paya,
    #[serde(rename = "S012")]
    S012,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "S005")]
    S005,
    #[serde(rename = "S001")]
    S001,
    #[serde(rename = "CONV")]
    Conv,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Intermediary48 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification132,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account34>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role8Choice>,
    #[serde(rename = "CtctPrsn", skip_serializing_if = "Option::is_none")]
    pub ctct_prsn: Option<ContactIdentification2>,
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
pub struct Cheque12 {
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
    #[serde(rename = "PyeeId", skip_serializing_if = "Option::is_none")]
    pub pyee_id: Option<PartyIdentification139>,
    #[serde(rename = "DrweeId", skip_serializing_if = "Option::is_none")]
    pub drwee_id: Option<FinancialInstitutionIdentification17>,
    #[serde(rename = "DrwrId", skip_serializing_if = "Option::is_none")]
    pub drwr_id: Option<PartyIdentification139>,
}
#[derive(
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
pub struct PartyIdentification132 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2Choice>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct Account33 {
    #[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
    pub ownr_id: Option<PartyIdentification132>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<PartyIdentification132>,
    #[serde(rename = "SubAcctDtls", skip_serializing_if = "Option::is_none")]
    pub sub_acct_dtls: Option<SubAccount5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Capped1 {
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "IncmLmtCurPrd", skip_serializing_if = "Option::is_none")]
    pub incm_lmt_cur_prd: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "IncmCurPrd", skip_serializing_if = "Option::is_none")]
    pub incm_cur_prd: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "IncmLmtNxtPrd", skip_serializing_if = "Option::is_none")]
    pub incm_lmt_nxt_prd: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ApplicableRules1Code {
    #[serde(rename = "NPRE")]
    Npre,
    #[serde(rename = "YPRE")]
    Ypre,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAssetType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashAssetType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAssetType1Choice {
    #[serde(flatten)]
    pub value: CashAssetType1ChoiceEnum,
}
#[derive(
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
pub struct ChargeBasis2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis5Code>,
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
pub struct ChargeBasis2Choice {
    #[serde(flatten)]
    pub value: ChargeBasis2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DrawdownType2Code {
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CAPP")]
    Capp,
    #[serde(rename = "FLEX")]
    Flex,
    #[default]
    Unknown,
}
#[derive(
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
pub struct UkDomesticSortCodeIdentifier {
    #[validate(regex = "UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
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
pub struct BenefitCrystallisationEvent2 {
    #[serde(rename = "EvtTpNb", skip_serializing_if = "Option::is_none")]
    pub evt_tp_nb: Option<Max35Text>,
    #[serde(rename = "EvtTpNm", skip_serializing_if = "Option::is_none")]
    pub evt_tp_nm: Option<Max35Text>,
    #[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
    pub evt_dt: Option<IsoDate>,
    #[serde(rename = "CrstllstnAmt", skip_serializing_if = "Option::is_none")]
    pub crstllstn_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "PctgOfAllwnc", skip_serializing_if = "Option::is_none")]
    pub pctg_of_allwnc: Option<PercentageRate>,
    #[serde(rename = "LftmAllwncPrtcn", skip_serializing_if = "Option::is_none")]
    pub lftm_allwnc_prtcn: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct PartyIdentification125ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
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
pub struct PartyIdentification125Choice {
    #[serde(flatten)]
    pub value: PartyIdentification125ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PersonIdentificationType7Code {
    #[serde(rename = "ATIN")]
    Atin,
    #[serde(rename = "GTIN")]
    Gtin,
    #[serde(rename = "ITIN")]
    Itin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectedStatusReason13Code {
    #[serde(rename = "BLCA")]
    Blca,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "IAQD")]
    Iaqd,
    #[serde(rename = "ICTN")]
    Ictn,
    #[serde(rename = "CYPA")]
    Cypa,
    #[serde(rename = "TREF")]
    Tref,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "IDNA")]
    Idna,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "FTAX")]
    Ftax,
    #[serde(rename = "INID")]
    Inid,
    #[serde(rename = "INAC")]
    Inac,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "INPM")]
    Inpm,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "INUK")]
    Inuk,
    #[serde(rename = "LEGL")]
    Legl,
    #[serde(rename = "NSLA")]
    Nsla,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "PTNS")]
    Ptns,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "ISTP")]
    Istp,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ISAT")]
    Isat,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "ACLO")]
    Aclo,
    #[serde(rename = "NASS")]
    Nass,
    #[serde(rename = "NQTY")]
    Nqty,
    #[serde(rename = "BLTR")]
    Bltr,
    #[serde(rename = "COSE")]
    Cose,
    #[serde(rename = "ILLI")]
    Illi,
    #[serde(rename = "BMRV")]
    Bmrv,
    #[serde(rename = "DINV")]
    Dinv,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "IPAC")]
    Ipac,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "IVAG")]
    Ivag,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "URSC")]
    Ursc,
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
pub struct SpanishDomesticInterbankingIdentifier {
    #[validate(regex = "SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX")]
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
#[derive(
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
pub struct ClearingSystemMemberIdentification4ChoiceEnum {
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification4Choice {
    #[serde(flatten)]
    pub value: ClearingSystemMemberIdentification4ChoiceEnum,
}
#[derive(
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
pub struct FinancialInstitutionIdentification10ChoiceEnum {
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2Choice>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<Max35Text>,
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification10Choice {
    #[serde(flatten)]
    pub value: FinancialInstitutionIdentification10ChoiceEnum,
}
#[derive(
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
pub struct CanadianPaymentsArnIdentifier {
    #[validate(regex = "CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole8Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "UCL1")]
    Ucl1,
    #[serde(rename = "UCL2")]
    Ucl2,
    #[serde(rename = "REGI")]
    Regi,
    #[serde(rename = "CACO")]
    Caco,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "DATP")]
    Datp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReversedStatus2ChoiceEnum {
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReversedStatus2Choice {
    #[serde(flatten)]
    pub value: ReversedStatus2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReference2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxReferenceType1Choice>,
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransferUnmatchedReason3Code {
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "SAFE")]
    Safe,
    #[default]
    Unknown,
}
#[derive(
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
pub struct SubAccount5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Extended350Code {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OtherAsset2Code {
    #[serde(rename = "DIMA")]
    Dima,
    #[serde(rename = "EXIA")]
    Exia,
    #[serde(rename = "MOVE")]
    Move,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "TIPP")]
    Tipp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument25ChoiceEnum {
    #[serde(rename = "CdtTrfDtls", skip_serializing_if = "Option::is_none")]
    pub cdt_trf_dtls: Option<CreditTransfer9>,
    #[serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none")]
    pub chq_dtls: Option<Cheque12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument25Choice {
    #[serde(flatten)]
    pub value: PaymentInstrument25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReferenceType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PersonIdentificationType7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReferenceType1Choice {
    #[serde(flatten)]
    pub value: TaxReferenceType1ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "YrMnth", skip_serializing_if = "Option::is_none")]
    pub yr_mnth: Option<IsoYearMonth>,
    #[serde(rename = "YrMnthDay", skip_serializing_if = "Option::is_none")]
    pub yr_mnth_day: Option<IsoDate>,
}
#[derive(
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
pub struct TypeOfPrice46ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice10Code>,
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
pub struct PaymentInstrument18 {
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<IsoDate>,
    #[serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none")]
    pub csh_sttlm_dtls: Option<PaymentInstrument25Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedReason45ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectedStatusReason13Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedReason45Choice {
    #[serde(flatten)]
    pub value: RejectedReason45ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailedSettlementStatus2ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max350Text>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailedSettlementStatus2Choice {
    #[serde(flatten)]
    pub value: FailedSettlementStatus2ChoiceEnum,
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
pub struct TransferUnmatchedStatus4ChoiceEnum {
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
    #[serde(rename = "XtndedRsn", skip_serializing_if = "Option::is_none")]
    pub xtnded_rsn: Option<Extended350Code>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<TransferUnmatchedReason3Code>,
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
pub struct TransferUnmatchedStatus4Choice {
    #[serde(flatten)]
    pub value: TransferUnmatchedStatus4ChoiceEnum,
}
#[derive(
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
pub struct ClassificationType32ChoiceEnum {
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification36>,
}
#[derive(
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
pub struct MarketPracticeVersion1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoYearMonth>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
}
#[derive(
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
#[derive(
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
pub struct TransferStatusType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TransferStatusType2Code>,
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
pub struct TransferStatusType3Choice {
    #[serde(flatten)]
    pub value: TransferStatusType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument63ChoiceEnum {
    #[serde(rename = "CshAsst", skip_serializing_if = "Option::is_none")]
    pub csh_asst: Option<CashAsset3>,
    #[serde(rename = "OthrAsst", skip_serializing_if = "Option::is_none")]
    pub othr_asst: Option<OtherAsset2>,
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<FinancialInstrumentIdentification2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument63Choice {
    #[serde(flatten)]
    pub value: FinancialInstrument63ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentIdentification1 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
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
pub struct UnitPrice23 {
    #[serde(rename = "Tp")]
    pub tp: TypeOfPrice46Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PriceValue1,
    #[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
    pub pric_mtd: Option<PriceMethod1Code>,
    #[serde(rename = "AcrdIntrstNAV", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Number>,
    #[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
    pub taxbl_incm_per_shr: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Choice>,
}
#[derive(
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
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
    pub dmst: Option<CountryCode>,
}
#[derive(
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
pub struct AdditionalInformation15 {
    #[validate]
    #[serde(rename = "InfTp")]
    pub inf_tp: GenericIdentification36,
    #[validate]
    #[serde(rename = "InfVal")]
    pub inf_val: Max350Text,
}
#[derive(
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
pub struct OtherAmountType1ChoiceEnum {
    #[serde(rename = "PrtryCd", skip_serializing_if = "Option::is_none")]
    pub prtry_cd: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OtherAmountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmountType1Choice {
    #[serde(flatten)]
    pub value: OtherAmountType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAsset2 {
    #[serde(rename = "OthrAsstTp")]
    pub othr_asst_tp: OtherAsset2Choice,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct OtherAsset2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OtherAsset2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAsset2Choice {
    #[serde(flatten)]
    pub value: OtherAsset2ChoiceEnum,
}
#[derive(
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
pub struct GermanBankleitzahlIdentifier {
    #[validate(regex = "GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingSettlementStatusReason2Code {
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "MINF")]
    Minf,
    #[serde(rename = "ACOP")]
    Acop,
    #[serde(rename = "IINV")]
    Iinv,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "AINV")]
    Ainv,
    #[serde(rename = "WTRF")]
    Wtrf,
    #[serde(rename = "USUA")]
    Usua,
    #[serde(rename = "ASTA")]
    Asta,
    #[serde(rename = "AFST")]
    Afst,
    #[serde(rename = "STST")]
    Stst,
    #[serde(rename = "LPRO")]
    Lpro,
    #[serde(rename = "ADRQ")]
    Adrq,
    #[serde(rename = "ADS1")]
    Ads1,
    #[serde(rename = "ADS2")]
    Ads2,
    #[serde(rename = "DRJC")]
    Drjc,
    #[serde(rename = "CYIN")]
    Cyin,
    #[serde(rename = "CYDV")]
    Cydv,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "WCPA")]
    Wcpa,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "TAPR")]
    Tapr,
    #[serde(rename = "XCNF")]
    Xcnf,
    #[serde(rename = "ESCA")]
    Esca,
    #[serde(rename = "NRCP")]
    Nrcp,
    #[serde(rename = "FVER")]
    Fver,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Role8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole8Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Role8Choice {
    #[serde(flatten)]
    pub value: Role8ChoiceEnum,
}
#[derive(
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
pub struct PendingSettlementStatus3ChoiceEnum {
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingSettlementStatusReason2Code>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "XtndedRsn", skip_serializing_if = "Option::is_none")]
    pub xtnded_rsn: Option<Extended350Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingSettlementStatus3Choice {
    #[serde(flatten)]
    pub value: PendingSettlementStatus3ChoiceEnum,
}
#[derive(
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
pub struct InRepairStatus4ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "DataSrcSchme", skip_serializing_if = "Option::is_none")]
    pub data_src_schme: Option<GenericIdentification1>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InRepairStatus4Choice {
    #[serde(flatten)]
    pub value: InRepairStatus4ChoiceEnum,
}
#[derive(
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
pub struct ValorenIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
