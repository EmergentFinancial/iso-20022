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
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:colr.019.001.01".to_string()
}

#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod2Code>,
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
pub struct CollateralParameters10 {
    #[serde(rename = "CollInstrTp")]
    pub coll_instr_tp: CollateralTransactionType1Choice,
    #[serde(rename = "XpsrTp")]
    pub xpsr_tp: ExposureType23Choice,
    #[serde(rename = "CollSd")]
    pub coll_sd: CollateralRole1Code,
    #[serde(rename = "ValSghtMrgnRate", skip_serializing_if = "Option::is_none")]
    pub val_sght_mrgn_rate: Option<RateOrType1Choice>,
    #[serde(rename = "TrfTitl", skip_serializing_if = "Option::is_none")]
    pub trf_titl: Option<YesNoIndicator>,
    #[serde(rename = "SttlmPrc", skip_serializing_if = "Option::is_none")]
    pub sttlm_prc: Option<GenericIdentification30>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<GenericIdentification30>,
    #[serde(rename = "AutomtcAllcn", skip_serializing_if = "Option::is_none")]
    pub automtc_allcn: Option<YesNoIndicator>,
    #[serde(rename = "FaildSttlmSlvtn", skip_serializing_if = "Option::is_none")]
    pub faild_sttlm_slvtn: Option<YesNoIndicator>,
    #[serde(
        rename = "MainTradgAcctCollstn",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_tradg_acct_collstn: Option<YesNoIndicator>,
    #[serde(
        rename = "BsktIdAndElgbltySetPrfl",
        skip_serializing_if = "Option::is_none"
    )]
    pub bskt_id_and_elgblty_set_prfl: Option<BasketIdentificationAndEligibilitySetProfile1>,
    #[serde(rename = "RspnSts", skip_serializing_if = "Option::is_none")]
    pub rspn_sts: Option<ResponseStatus9Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation24>,
}
#[derive(
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
pub struct TransactionIdentifications45 {
    #[validate]
    #[serde(rename = "ClntCollInstrId")]
    pub clnt_coll_instr_id: Max35Text,
    #[serde(rename = "ClntCollTxId", skip_serializing_if = "Option::is_none")]
    pub clnt_coll_tx_id: Option<Max35Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrCollInstrId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_coll_instr_id: Option<Max35Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrCollTxId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_coll_tx_id: Option<Max35Text>,
    #[serde(rename = "CmonTxId", skip_serializing_if = "Option::is_none")]
    pub cmon_tx_id: Option<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1ChoiceEnum {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[serde(rename = "MktIdrCd", skip_serializing_if = "Option::is_none")]
    pub mkt_idr_cd: Option<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification1Choice {
    #[serde(flatten)]
    pub value: MarketIdentification1ChoiceEnum,
}
#[derive(
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
pub struct AmountAndDirection49 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
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
pub struct AdditionalInformation24 {
    #[serde(rename = "CollInstr", skip_serializing_if = "Option::is_none")]
    pub coll_instr: Option<Max350Text>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Max350Text>,
}
#[derive(
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
pub struct ResponseStatus9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ResponseStatus2Code>,
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
pub struct ResponseStatus9Choice {
    #[serde(flatten)]
    pub value: ResponseStatus9ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DealTransactionDetails5 {
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<PlaceOfTradeIdentification1>,
    #[serde(rename = "CncntrtnLmt", skip_serializing_if = "Option::is_none")]
    pub cncntrtn_lmt: Option<YesNoIndicator>,
    #[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
    pub min_ntce_prd: Option<Exact3NumericText>,
    #[serde(rename = "ClsgDt")]
    pub clsg_dt: ClosingDate4Choice,
    #[serde(rename = "DealDtlsAmt", skip_serializing_if = "Option::is_none")]
    pub deal_dtls_amt: Option<CollateralAmount18>,
    #[serde(rename = "PricgRateAndIndx", skip_serializing_if = "Option::is_none")]
    pub pricg_rate_and_indx: Option<RateOrName4Choice>,
    #[serde(
        rename = "OvrnghtFrqcyRateFxg",
        skip_serializing_if = "Option::is_none"
    )]
    pub ovrnght_frqcy_rate_fxg: Option<FrequencyRateFixing1Choice>,
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<PercentageRate>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat4Choice>,
    #[serde(rename = "Pmt", skip_serializing_if = "Option::is_none")]
    pub pmt: Option<DeliveryReceiptType2Code>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType6Choice>,
    #[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
    pub termntn_optn: Option<RepoTerminationOption1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherParties38 {
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification136>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Invstr", default)]
    pub invstr: Vec<PartyIdentification149>,
}
#[derive(
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
pub enum FrequencyRateFixing1Code {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "PRDC")]
    Prdc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CalculationMethod1Code {
    #[serde(rename = "SIMP")]
    Simp,
    #[serde(rename = "COMP")]
    Comp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType14Code {
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CBCO")]
    Cbco,
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
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXTD")]
    Extd,
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
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "UDMS")]
    Udms,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "SHSL")]
    Shsl,
    #[serde(rename = "MGLD")]
    Mgld,
    #[default]
    Unknown,
}
#[derive(
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
pub struct MarketIdentification84 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification1Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType8Choice,
}
#[derive(
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
pub struct CollateralAmount18 {
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<AmountAndDirection49>,
    #[serde(rename = "Termntn", skip_serializing_if = "Option::is_none")]
    pub termntn: Option<AmountAndDirection49>,
    #[serde(rename = "Acrd", skip_serializing_if = "Option::is_none")]
    pub acrd: Option<AmountAndDirection49>,
    #[serde(rename = "CmpndSmplAcrlClctn", skip_serializing_if = "Option::is_none")]
    pub cmpnd_smpl_acrl_clctn: Option<CalculationMethod1Code>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency38Choice>,
    #[serde(rename = "IntrstPmtDely", skip_serializing_if = "Option::is_none")]
    pub intrst_pmt_dely: Option<Max3NumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxAmtBrkdwn", default)]
    pub tx_amt_brkdwn: Vec<CollateralTransactionAmountBreakdown2>,
    #[serde(rename = "ValSght", skip_serializing_if = "Option::is_none")]
    pub val_sght: Option<AmountAndDirection49>,
}
#[derive(
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
pub struct PartyIdentification134ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
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
pub struct PartyIdentification134Choice {
    #[serde(flatten)]
    pub value: PartyIdentification134ChoiceEnum,
}
#[derive(
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
pub struct Frequency38ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency12Code>,
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
pub struct Frequency38Choice {
    #[serde(flatten)]
    pub value: Frequency38ChoiceEnum,
}
#[derive(
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
pub struct TripartyCollateralTransactionInstructionV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TxInstrId")]
    pub tx_instr_id: TransactionIdentifications45,
    #[validate(length(min = 0,))]
    #[serde(rename = "Lnkgs", default)]
    pub lnkgs: Vec<Linkages58>,
    #[validate]
    #[serde(rename = "Pgntn")]
    pub pgntn: Pagination1,
    #[validate]
    #[serde(rename = "GnlParams")]
    pub gnl_params: CollateralParameters10,
    #[validate]
    #[serde(rename = "CollPties")]
    pub coll_pties: CollateralParties10,
    #[validate]
    #[serde(rename = "DealTxDtls")]
    pub deal_tx_dtls: DealTransactionDetails5,
    #[validate]
    #[serde(rename = "DealTxDt")]
    pub deal_tx_dt: CollateralDate2,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmnt", default)]
    pub scties_mvmnt: Vec<SecuritiesMovement9>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmnt", default)]
    pub csh_mvmnt: Vec<CashMovement8>,
    #[serde(rename = "OthrPties", skip_serializing_if = "Option::is_none")]
    pub othr_pties: Option<OtherParties38>,
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
pub struct BlockChainAddressWallet3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
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
pub struct RateOrType1ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification1>,
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
pub struct RateOrType1Choice {
    #[serde(flatten)]
    pub value: RateOrType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrName4ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "RateIndxDtls", skip_serializing_if = "Option::is_none")]
    pub rate_indx_dtls: Option<RateTypeAndLookback2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrName4Choice {
    #[serde(flatten)]
    pub value: RateOrName4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification149 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification134Choice,
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
pub struct BenchmarkCurveName13ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BenchmarkCurveName7Code>,
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
pub struct BenchmarkCurveName13Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName13ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResponseStatus2Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "DKNY")]
    Dkny,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct MarketType8Choice {
    #[serde(flatten)]
    pub value: MarketType8ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestRateIndexTenor2Code {
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "FOMN")]
    Fomn,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "TOWK")]
    Towk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FrequencyRateFixing1ChoiceEnum {
    #[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
    pub nb_of_days: Option<Max3NumericText>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FrequencyRateFixing1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FrequencyRateFixing1Choice {
    #[serde(flatten)]
    pub value: FrequencyRateFixing1ChoiceEnum,
}
#[derive(
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
pub struct GenericIdentification178 {
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName7Code {
    #[serde(rename = "BBSW")]
    Bbsw,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "BCOL")]
    Bcol,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "CORA")]
    Cora,
    #[serde(rename = "CZNA")]
    Czna,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "ESTR")]
    Estr,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "EFFR")]
    Effr,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "HKIO")]
    Hkio,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "ETIO")]
    Etio,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "BJUO")]
    Bjuo,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "OBFR")]
    Obfr,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "RCTR")]
    Rctr,
    #[serde(rename = "SOFR")]
    Sofr,
    #[serde(rename = "SONA")]
    Sona,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TOAR")]
    Toar,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "WIBO")]
    Wibo,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
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
pub enum OptionType1Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralTransactionType1Code {
    #[serde(rename = "AADJ")]
    Aadj,
    #[serde(rename = "CDTA")]
    Cdta,
    #[serde(rename = "CADJ")]
    Cadj,
    #[serde(rename = "DADJ")]
    Dadj,
    #[serde(rename = "DBVT")]
    Dbvt,
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "MADJ")]
    Madj,
    #[serde(rename = "PADJ")]
    Padj,
    #[serde(rename = "RATA")]
    Rata,
    #[serde(rename = "TERM")]
    Term,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period2 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralEntryType1Code {
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
pub struct AlternatePartyIdentification7 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType42Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralParties10 {
    #[validate]
    #[serde(rename = "PtyA")]
    pub pty_a: PartyIdentificationAndAccount202,
    #[serde(rename = "ClntPtyA", skip_serializing_if = "Option::is_none")]
    pub clnt_pty_a: Option<PartyIdentificationAndAccount202>,
    #[validate]
    #[serde(rename = "PtyB")]
    pub pty_b: PartyIdentificationAndAccount203,
    #[serde(rename = "ClntPtyB", skip_serializing_if = "Option::is_none")]
    pub clnt_pty_b: Option<PartyIdentificationAndAccount203>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<PartyIdentification136>,
    #[serde(rename = "CollAcct", skip_serializing_if = "Option::is_none")]
    pub coll_acct: Option<SecuritiesAccount19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOfTradeIdentification1 {
    #[serde(rename = "MktTpAndId", skip_serializing_if = "Option::is_none")]
    pub mkt_tp_and_id: Option<MarketIdentification84>,
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
pub struct FinancialInstrumentQuantity33ChoiceEnum {
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
}
#[derive(
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
pub struct ClosingDate4ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Date3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClosingDate4Choice {
    #[serde(flatten)]
    pub value: ClosingDate4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralTransactionAmountBreakdown2 {
    #[validate]
    #[serde(rename = "LotNb")]
    pub lot_nb: GenericIdentification178,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Period4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Period4ChoiceEnum {
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<IsoDate>,
    #[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt_to_dt: Option<Period2>,
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
pub struct Period4Choice {
    #[serde(flatten)]
    pub value: Period4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralDate2 {
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<IsoDate>,
    #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<IsoDate>,
}
#[derive(
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
pub struct SecuritiesAccount19 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType2Code {
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
pub struct IdentificationType42ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
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
pub struct IdentificationType42Choice {
    #[serde(flatten)]
    pub value: IdentificationType42ChoiceEnum,
}
#[derive(
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
pub struct OtherIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoTerminationOption1Code {
    #[serde(rename = "EGRN")]
    Egrn,
    #[serde(rename = "ETSB")]
    Etsb,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity7Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "PRIN")]
    Prin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralRole1Code {
    #[serde(rename = "GIVE")]
    Give,
    #[serde(rename = "TAKE")]
    Take,
    #[default]
    Unknown,
}
#[derive(
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
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max34Text>,
}
#[derive(
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
pub struct Linkages58 {
    #[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
    pub msg_nb: Option<DocumentNumber5Choice>,
    #[serde(rename = "Refs")]
    pub refs: References70Choice,
}
#[derive(
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
pub struct CrystallisationDay1 {
    #[validate]
    #[serde(rename = "Day")]
    pub day: YesNoIndicator,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Max3NumericText>,
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
pub struct GenericIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
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
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
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
pub struct ExposureType23ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExposureType14Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureType23Choice {
    #[serde(flatten)]
    pub value: ExposureType23ChoiceEnum,
}
#[derive(
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
pub struct TradingPartyCapacity5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingCapacity7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity5Choice {
    #[serde(flatten)]
    pub value: TradingPartyCapacity5ChoiceEnum,
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
pub struct PartyIdentificationAndAccount202 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification136>,
    #[serde(rename = "PtyCpcty", skip_serializing_if = "Option::is_none")]
    pub pty_cpcty: Option<TradingPartyCapacity5Choice>,
}
#[derive(
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
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification36>,
}
#[derive(
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
pub struct Date3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Date3Choice {
    #[serde(flatten)]
    pub value: Date3ChoiceEnum,
}
#[derive(
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
pub struct CollateralTransactionType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CollateralTransactionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralTransactionType1Choice {
    #[serde(flatten)]
    pub value: CollateralTransactionType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateTypeAndLookback2 {
    #[serde(rename = "Tp")]
    pub tp: BenchmarkCurveName13Choice,
    #[serde(rename = "LookBckDays", skip_serializing_if = "Option::is_none")]
    pub look_bck_days: Option<Max3NumericText>,
    #[serde(rename = "CrstllstnDt", skip_serializing_if = "Option::is_none")]
    pub crstllstn_dt: Option<CrystallisationDay1>,
    #[serde(rename = "Tnr", skip_serializing_if = "Option::is_none")]
    pub tnr: Option<InterestRateIndexTenor2Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesMovement9 {
    #[serde(rename = "SctiesMvmntTp")]
    pub scties_mvmnt_tp: CollateralEntryType1Code,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity33Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[validate]
    #[serde(rename = "CollMvmnt")]
    pub coll_mvmnt: YesNoIndicator,
    #[serde(rename = "ClntSctiesMvmntId", skip_serializing_if = "Option::is_none")]
    pub clnt_scties_mvmnt_id: Option<Max35Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrSctiesMvmntId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_scties_mvmnt_id: Option<Max35Text>,
}
#[derive(
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
pub struct OptionType6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct OptionType6Choice {
    #[serde(flatten)]
    pub value: OptionType6ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "TrptyCollTxInstr")]
    pub trpty_coll_tx_instr: TripartyCollateralTransactionInstructionV01<A>,
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
pub struct PartyIdentification120ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
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
pub struct PartyIdentification120Choice {
    #[serde(flatten)]
    pub value: PartyIdentification120ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency12Code {
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "TWMN")]
    Twmn,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "ONDE")]
    Onde,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification136 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
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
pub struct CashMovement8 {
    #[serde(rename = "CshMvmnt")]
    pub csh_mvmnt: CollateralEntryType1Code,
    #[validate]
    #[serde(rename = "CshAmt")]
    pub csh_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[validate]
    #[serde(rename = "CollMvmnt")]
    pub coll_mvmnt: YesNoIndicator,
    #[serde(rename = "ClntCshMvmntId", skip_serializing_if = "Option::is_none")]
    pub clnt_csh_mvmnt_id: Option<Max35Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrCshMvmntId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_csh_mvmnt_id: Option<Max35Text>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BasketIdentificationAndEligibilitySetProfile1 {
    #[serde(rename = "PrfrntlBsktIdNb", skip_serializing_if = "Option::is_none")]
    pub prfrntl_bskt_id_nb: Option<GenericIdentification1>,
    #[serde(rename = "FllbckStartgBsktId", skip_serializing_if = "Option::is_none")]
    pub fllbck_startg_bskt_id: Option<GenericIdentification1>,
    #[serde(rename = "ExclsnBsktId", skip_serializing_if = "Option::is_none")]
    pub exclsn_bskt_id: Option<GenericIdentification1>,
    #[serde(rename = "ElgbltySetPrfl", skip_serializing_if = "Option::is_none")]
    pub elgblty_set_prfl: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References70ChoiceEnum {
    #[serde(rename = "ClntCollInstrId", skip_serializing_if = "Option::is_none")]
    pub clnt_coll_instr_id: Option<Max35Text>,
    #[serde(
        rename = "TrptyAgtSvcPrvdrCollInstrId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trpty_agt_svc_prvdr_coll_instr_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References70Choice {
    #[serde(flatten)]
    pub value: References70ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount203 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[serde(rename = "PtyCpcty", skip_serializing_if = "Option::is_none")]
    pub pty_cpcty: Option<TradingPartyCapacity5Choice>,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
