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
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.036.001.13".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Rate35 {
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat37Choice>,
    #[serde(rename = "EarlySlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub early_slctn_fee_rate: Option<SolicitationFeeRateFormat8Choice>,
    #[serde(rename = "ThrdPtyIncntivRate", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_incntiv_rate: Option<RateAndAmountFormat39Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat7Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat40Choice>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<PercentageRate>,
    #[serde(rename = "SlctnFeeRate", skip_serializing_if = "Option::is_none")]
    pub slctn_fee_rate: Option<SolicitationFeeRateFormat8Choice>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat22Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat40Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat40Choice>,
    #[serde(rename = "TaxOnIncm", skip_serializing_if = "Option::is_none")]
    pub tax_on_incm: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "TaxOnPrfts", skip_serializing_if = "Option::is_none")]
    pub tax_on_prfts: Option<PercentageRate>,
    #[serde(rename = "TaxRclmRate", skip_serializing_if = "Option::is_none")]
    pub tax_rclm_rate: Option<PercentageRate>,
    #[serde(rename = "EqulstnRate", skip_serializing_if = "Option::is_none")]
    pub equlstn_rate: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DmdRate", default)]
    pub dmd_rate: Vec<RateAndAmountFormat52Choice>,
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct GenericIdentification78 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification30,
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
pub struct MarketIdentification1ChoiceEnum {
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
pub struct RateAndAmountFormat40ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat40Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat40ChoiceEnum,
}
#[derive(
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
pub struct AccountAndBalance49 {
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification127Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat28Choice>,
    #[validate]
    #[serde(rename = "Bal")]
    pub bal: CorporateActionBalanceDetails41,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate65 {
    #[serde(rename = "PstngDt")]
    pub pstng_dt: DateAndDateTime2Choice,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "FXRateFxgDt", skip_serializing_if = "Option::is_none")]
    pub fx_rate_fxg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateAndDateTime2Choice>,
}
#[derive(
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
pub struct DocumentIdentification31 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct CorporateActionPrice61 {
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat50Choice>,
    #[serde(rename = "OverSbcptDpstPric", skip_serializing_if = "Option::is_none")]
    pub over_sbcpt_dpst_pric: Option<PriceFormat50Choice>,
}
#[derive(
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
pub struct AmountAndQuantityRatio4 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NewSecuritiesIssuanceType6Code {
    #[serde(rename = "DEFE")]
    Defe,
    #[serde(rename = "NDEF")]
    Ndef,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "NREF")]
    Nref,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionFeatures6Code {
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
pub struct TaxVoucher4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "BrgnDt", skip_serializing_if = "Option::is_none")]
    pub brgn_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "BrgnSttlmDt", skip_serializing_if = "Option::is_none")]
    pub brgn_sttlm_dt: Option<DateAndDateTime2Choice>,
}
#[derive(
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
#[derive(
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
pub struct PriceFormat64ChoiceEnum {
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
    #[serde(
        rename = "AmtPricPerFinInstrmQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub amt_pric_per_fin_instrm_qty: Option<AmountPricePerFinancialInstrumentQuantity10>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<DecimalNumber>,
    #[serde(rename = "AmtPricPerAmt", skip_serializing_if = "Option::is_none")]
    pub amt_pric_per_amt: Option<AmountPricePerAmount2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat64Choice {
    #[serde(flatten)]
    pub value: PriceFormat64ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionDate59 {
    #[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
    pub rcrd_dt: Option<DateFormat43Choice>,
    #[serde(rename = "ExDvddDt", skip_serializing_if = "Option::is_none")]
    pub ex_dvdd_dt: Option<DateFormat43Choice>,
}
#[derive(
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
    #[serde(
        rename = "LkdOffclCorpActnEvtId",
        skip_serializing_if = "Option::is_none"
    )]
    pub lkd_offcl_corp_actn_evt_id: Option<Max35Text>,
    #[serde(rename = "LkdCorpActnId", skip_serializing_if = "Option::is_none")]
    pub lkd_corp_actn_id: Option<Max35Text>,
}
#[derive(
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
pub struct CorporateActionOption195 {
    #[serde(rename = "OptnNb")]
    pub optn_nb: OptionNumber1Choice,
    #[serde(rename = "OptnTp")]
    pub optn_tp: CorporateActionOption33Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnFeatrs", default)]
    pub optn_featrs: Vec<OptionFeaturesFormat18Choice>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType27Choice>,
    #[serde(rename = "CcyOptn", skip_serializing_if = "Option::is_none")]
    pub ccy_optn: Option<ActiveCurrencyCode>,
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate79>,
    #[serde(rename = "PrdDtls", skip_serializing_if = "Option::is_none")]
    pub prd_dtls: Option<CorporateActionPeriod13>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<CorporateActionRate107>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice61>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<MarketIdentification84>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmntDtls", default)]
    pub scties_mvmnt_dtls: Vec<SecuritiesOption78>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmntDtls", default)]
    pub csh_mvmnt_dtls: Vec<CashOption79>,
}
#[derive(
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
pub struct DocumentIdentification9 {
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
pub struct RateAndAmountFormat52ChoiceEnum {
    #[serde(rename = "RateTpAndRate", skip_serializing_if = "Option::is_none")]
    pub rate_tp_and_rate: Option<RateTypeAndPercentageRate10>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus37>,
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
pub struct RateAndAmountFormat52Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat52ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateUsedForPaymentFormat7ChoiceEnum {
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus24>,
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
pub struct InterestRateUsedForPaymentFormat7Choice {
    #[serde(flatten)]
    pub value: InterestRateUsedForPaymentFormat7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionNumber1Code {
    #[serde(rename = "UNSO")]
    Unso,
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
pub struct CorporateActionGeneralInformation162 {
    #[validate]
    #[serde(rename = "CorpActnEvtId")]
    pub corp_actn_evt_id: Max35Text,
    #[serde(rename = "OffclCorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub offcl_corp_actn_evt_id: Option<Max35Text>,
    #[serde(rename = "ClssActnNb", skip_serializing_if = "Option::is_none")]
    pub clss_actn_nb: Option<Max35Text>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType87Choice,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "FrctnlQty", skip_serializing_if = "Option::is_none")]
    pub frctnl_qty: Option<FinancialInstrumentQuantity33Choice>,
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
pub struct PartyIdentification143 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification122Choice,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnId", default)]
    pub altrn_id: Vec<AlternatePartyIdentification7>,
}
#[derive(
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
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventStage4Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "RESC")]
    Resc,
    #[default]
    Unknown,
}
#[derive(
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
pub struct SafekeepingPlaceTypeAndText8 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
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
pub struct PartyIdentificationAndAccount204 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnId", default)]
    pub altrn_id: Vec<AlternatePartyIdentification7>,
}
#[derive(
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
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct SolicitationFeeRateFormat8ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "AmtToQty", skip_serializing_if = "Option::is_none")]
    pub amt_to_qty: Option<AmountAndQuantityRatio4>,
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
pub struct SolicitationFeeRateFormat8Choice {
    #[serde(flatten)]
    pub value: SolicitationFeeRateFormat8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionNarrative31 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlTxt", default)]
    pub addtl_txt: Vec<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NrrtvVrsn", default)]
    pub nrrtv_vrsn: Vec<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PtyCtctNrrtv", default)]
    pub pty_ctct_nrrtv: Vec<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxtnConds", default)]
    pub taxtn_conds: Vec<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateAction62 {
    #[serde(rename = "DtDtls", skip_serializing_if = "Option::is_none")]
    pub dt_dtls: Option<CorporateActionDate59>,
    #[serde(rename = "EvtStag", skip_serializing_if = "Option::is_none")]
    pub evt_stag: Option<CorporateActionEventStageFormat14Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlBizPrcInd", default)]
    pub addtl_biz_prc_ind: Vec<AdditionalBusinessProcessFormat19Choice>,
    #[serde(
        rename = "IntrmdtSctiesDstrbtnTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrmdt_scties_dstrbtn_tp: Option<IntermediateSecuritiesDistributionTypeFormat15Choice>,
    #[serde(rename = "LtryTp", skip_serializing_if = "Option::is_none")]
    pub ltry_tp: Option<LotteryTypeFormat4Choice>,
}
#[derive(
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
pub struct SettlementParties102 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification143>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount204>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount204>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount204>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPeriod13 {
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<Period11>,
    #[serde(rename = "ActnPrd", skip_serializing_if = "Option::is_none")]
    pub actn_prd: Option<Period11>,
    #[serde(rename = "ParllTradgPrd", skip_serializing_if = "Option::is_none")]
    pub parll_tradg_prd: Option<Period11>,
}
#[derive(
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
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RateTypeAndAmountAndStatus55 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType76Choice,
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
pub struct OptionFeaturesFormat18ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionFeatures6Code>,
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
pub struct OptionFeaturesFormat18Choice {
    #[serde(flatten)]
    pub value: OptionFeaturesFormat18ChoiceEnum,
}
#[derive(
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
pub struct GrossDividendRateFormat35ChoiceEnum {
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus55>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat35Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat35ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentificationAndAccount162 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
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
pub struct CashParties34 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentificationAndAccount162>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<PartyIdentificationAndAccount172>,
    #[serde(rename = "MktClmCtrPty", skip_serializing_if = "Option::is_none")]
    pub mkt_clm_ctr_pty: Option<PartyIdentificationAndAccount162>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionRate90 {
    #[serde(
        rename = "AddtlQtyForSbcbdRsltntScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_sbcbd_rsltnt_scties: Option<RatioFormat20Choice>,
    #[serde(
        rename = "AddtlQtyForExstgScties",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_qty_for_exstg_scties: Option<RatioFormat20Choice>,
    #[serde(rename = "NewToOd", skip_serializing_if = "Option::is_none")]
    pub new_to_od: Option<RatioFormat19Choice>,
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<RateAndAmountFormat39Choice>,
    #[serde(rename = "FsclStmp", skip_serializing_if = "Option::is_none")]
    pub fscl_stmp: Option<PercentageRate>,
    #[serde(rename = "AplblRate", skip_serializing_if = "Option::is_none")]
    pub aplbl_rate: Option<PercentageRate>,
    #[serde(rename = "TaxCdtRate", skip_serializing_if = "Option::is_none")]
    pub tax_cdt_rate: Option<RateFormat22Choice>,
    #[serde(rename = "FinTxTaxRate", skip_serializing_if = "Option::is_none")]
    pub fin_tx_tax_rate: Option<PercentageRate>,
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
#[derive(
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
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText6>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
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
pub struct RateFormat22ChoiceEnum {
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
pub struct RateFormat22Choice {
    #[serde(flatten)]
    pub value: RateFormat22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesOption78 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "TempFinInstrmInd", skip_serializing_if = "Option::is_none")]
    pub temp_fin_instrm_ind: Option<TemporaryFinancialInstrumentIndicator3Choice>,
    #[serde(rename = "NewSctiesIssncInd", skip_serializing_if = "Option::is_none")]
    pub new_scties_issnc_ind: Option<NewSecuritiesIssuanceType6Code>,
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
    #[serde(rename = "PstngQty")]
    pub pstng_qty: Quantity51Choice,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat29Choice>,
    #[serde(rename = "FrctnDspstn", skip_serializing_if = "Option::is_none")]
    pub frctn_dspstn: Option<FractionDispositionType27Choice>,
    #[serde(rename = "CcyOptn", skip_serializing_if = "Option::is_none")]
    pub ccy_optn: Option<ActiveCurrencyCode>,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: SecurityDate15,
    #[serde(rename = "RateDtls", skip_serializing_if = "Option::is_none")]
    pub rate_dtls: Option<CorporateActionRate90>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<CorporateActionPrice74>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties102>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties102>,
}
#[derive(
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
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
}
#[derive(
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
pub struct CorporateActionEventStageFormat14ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventStage4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventStageFormat14Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventStageFormat14ChoiceEnum,
}
#[derive(
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
pub struct TransactionIdentification15 {
    #[validate]
    #[serde(rename = "MktInfrstrctrTxId")]
    pub mkt_infrstrctr_tx_id: Max35Text,
}
#[derive(
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
pub struct DateCode19ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct DateCode19Choice {
    #[serde(flatten)]
    pub value: DateCode19ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventType30Code {
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
    #[serde(rename = "MTNG")]
    Mtng,
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
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct CorporateActionDate79 {
    #[serde(rename = "RspnDdln", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "SbcptCostDbtDt", skip_serializing_if = "Option::is_none")]
    pub sbcpt_cost_dbt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<DateFormat43Choice>,
    #[serde(rename = "CoverXprtnDdln", skip_serializing_if = "Option::is_none")]
    pub cover_xprtn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "PrtctDdln", skip_serializing_if = "Option::is_none")]
    pub prtct_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "TradgDt", skip_serializing_if = "Option::is_none")]
    pub tradg_dt: Option<DateFormat43Choice>,
}
#[derive(
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
pub struct DocumentIdentification32 {
    #[serde(rename = "Id")]
    pub id: DocumentIdentification3Choice,
    #[serde(rename = "DocNb", skip_serializing_if = "Option::is_none")]
    pub doc_nb: Option<DocumentNumber5Choice>,
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
pub struct PriceFormat50ChoiceEnum {
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
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
pub struct PriceFormat50Choice {
    #[serde(flatten)]
    pub value: PriceFormat50ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification36>,
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
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
pub struct PriceFormat51ChoiceEnum {
    #[serde(rename = "IndxPts", skip_serializing_if = "Option::is_none")]
    pub indx_pts: Option<DecimalNumber>,
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
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
pub struct PriceFormat51Choice {
    #[serde(flatten)]
    pub value: PriceFormat51ChoiceEnum,
}
#[derive(
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
pub struct NetDividendRateFormat40ChoiceEnum {
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
pub struct NetDividendRateFormat40Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat40ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionAmounts56 {
    #[validate]
    #[serde(rename = "PstngAmt")]
    pub pstng_amt: ActiveCurrencyAndAmount,
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
    #[serde(rename = "MktClmAmt", skip_serializing_if = "Option::is_none")]
    pub mkt_clm_amt: Option<ActiveCurrencyAndAmount>,
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
    #[serde(rename = "CshAmtBrghtFwd", skip_serializing_if = "Option::is_none")]
    pub csh_amt_brght_fwd: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshAmtCrrdFwd", skip_serializing_if = "Option::is_none")]
    pub csh_amt_crrd_fwd: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NtnlDvddPyblAmt", skip_serializing_if = "Option::is_none")]
    pub ntnl_dvdd_pybl_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NtnlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ntnl_tax_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TaxArrearsAmt", skip_serializing_if = "Option::is_none")]
    pub tax_arrears_amt: Option<ActiveCurrencyAndAmount>,
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
    #[serde(rename = "AdjstdSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub adjstd_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RfnddSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub rfndd_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
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
pub struct DeemedRateType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeemedRateType1Code>,
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
pub struct MarketType8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType2Code>,
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
pub struct MarketType8Choice {
    #[serde(flatten)]
    pub value: MarketType8ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateStatus1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "INDI")]
    Indi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionOption12Code {
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
    #[serde(rename = "MKDW")]
    Mkdw,
    #[serde(rename = "MKUP")]
    Mkup,
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
    #[default]
    Unknown,
}
#[derive(
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
pub struct SecurityDate15 {
    #[serde(rename = "PstngDt")]
    pub pstng_dt: DateAndDateTime2Choice,
    #[serde(rename = "AvlblDt", skip_serializing_if = "Option::is_none")]
    pub avlbl_dt: Option<DateFormat43Choice>,
    #[serde(rename = "PrpssDt", skip_serializing_if = "Option::is_none")]
    pub prpss_dt: Option<DateFormat43Choice>,
    #[serde(rename = "DvddRnkgDt", skip_serializing_if = "Option::is_none")]
    pub dvdd_rnkg_dt: Option<DateFormat43Choice>,
    #[serde(rename = "EarlstPmtDt", skip_serializing_if = "Option::is_none")]
    pub earlst_pmt_dt: Option<DateFormat43Choice>,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat43Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Account8ChoiceEnum {
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification5Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Account8Choice {
    #[serde(flatten)]
    pub value: Account8ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct PartyIdentification133ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
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
pub struct PartyIdentification133Choice {
    #[serde(flatten)]
    pub value: PartyIdentification133ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
}
#[derive(
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
pub struct PriceDetails26 {
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat51Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat64Choice>,
}
#[derive(
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
pub struct BalanceFormat11ChoiceEnum {
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat11>,
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
pub struct MarketIdentification84 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification1Choice>,
    #[serde(rename = "Tp")]
    pub tp: MarketType8Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IssuerTaxability2Code {
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
pub struct AmountPricePerFinancialInstrumentQuantity10 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "FinInstrmQty")]
    pub fin_instrm_qty: FinancialInstrumentQuantity33Choice,
}
#[derive(
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
pub struct IdentificationSource3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
}
#[derive(
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
pub struct IntermediateSecuritiesDistributionTypeFormat15ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<IntermediateSecurityDistributionType5Code>,
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
pub struct IndicativeOrMarketPrice8ChoiceEnum {
    #[serde(rename = "IndctvPric", skip_serializing_if = "Option::is_none")]
    pub indctv_pric: Option<PriceFormat50Choice>,
    #[serde(rename = "MktPric", skip_serializing_if = "Option::is_none")]
    pub mkt_pric: Option<PriceFormat50Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndicativeOrMarketPrice8Choice {
    #[serde(flatten)]
    pub value: IndicativeOrMarketPrice8ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Payment1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "CONT")]
    Cont,
    #[default]
    Unknown,
}
#[derive(
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
pub struct CorporateActionRate107 {
    #[validate(length(min = 0,))]
    #[serde(rename = "GrssDvddRate", default)]
    pub grss_dvdd_rate: Vec<GrossDividendRateFormat35Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetDvddRate", default)]
    pub net_dvdd_rate: Vec<NetDividendRateFormat37Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRateUsdForPmt", default)]
    pub intrst_rate_usd_for_pmt: Vec<InterestRateUsedForPaymentFormat7Choice>,
    #[serde(
        rename = "MaxAllwdOvrsbcptRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_allwd_ovrsbcpt_rate: Option<PercentageRate>,
    #[serde(rename = "PrratnRate", skip_serializing_if = "Option::is_none")]
    pub prratn_rate: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRate", default)]
    pub whldg_tax_rate: Vec<RateAndAmountFormat40Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndLvlTax", default)]
    pub scnd_lvl_tax: Vec<RateAndAmountFormat40Choice>,
    #[serde(rename = "AddtlTax", skip_serializing_if = "Option::is_none")]
    pub addtl_tax: Option<RateAndAmountFormat39Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxblIncmPerDvddShr", default)]
    pub taxbl_incm_per_dvdd_shr: Vec<RateTypeAndAmountAndStatus26>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashOption79 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "CtrctlPmtInd", skip_serializing_if = "Option::is_none")]
    pub ctrctl_pmt_ind: Option<Payment1Code>,
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
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account8Choice>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties34>,
    #[validate]
    #[serde(rename = "AmtDtls")]
    pub amt_dtls: CorporateActionAmounts56,
    #[validate]
    #[serde(rename = "DtDtls")]
    pub dt_dtls: CorporateActionDate65,
    #[validate(length(min = 0,))]
    #[serde(rename = "FXDtls", default)]
    pub fx_dtls: Vec<ForeignExchangeTerms23>,
    #[serde(rename = "TaxVchrDtls", skip_serializing_if = "Option::is_none")]
    pub tax_vchr_dtls: Option<TaxVoucher4>,
    #[serde(rename = "RateAndAmtDtls", skip_serializing_if = "Option::is_none")]
    pub rate_and_amt_dtls: Option<Rate35>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<PriceDetails26>,
}
#[derive(
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
pub struct CorporateActionEventType87ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType30Code>,
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
pub struct CorporateActionEventType87Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventType87ChoiceEnum,
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
pub struct NetDividendRateFormat37ChoiceEnum {
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus56>,
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
pub struct NetDividendRateFormat37Choice {
    #[serde(flatten)]
    pub value: NetDividendRateFormat37ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount172 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification133Choice,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
}
#[derive(
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
pub struct ProcessingPosition7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessingPosition3Code>,
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
pub struct CorporateActionMovementConfirmationV13<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Pgntn", skip_serializing_if = "Option::is_none")]
    pub pgntn: Option<Pagination1>,
    #[serde(rename = "NtfctnId", skip_serializing_if = "Option::is_none")]
    pub ntfctn_id: Option<DocumentIdentification31>,
    #[serde(rename = "MvmntPrlimryAdvcId", skip_serializing_if = "Option::is_none")]
    pub mvmnt_prlimry_advc_id: Option<DocumentIdentification31>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<DocumentIdentification9>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrDocId", default)]
    pub othr_doc_id: Vec<DocumentIdentification32>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtsLkg", default)]
    pub evts_lkg: Vec<CorporateActionEventReference3>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<TransactionIdentification15>,
    #[validate]
    #[serde(rename = "CorpActnGnlInf")]
    pub corp_actn_gnl_inf: CorporateActionGeneralInformation162,
    #[validate]
    #[serde(rename = "AcctDtls")]
    pub acct_dtls: AccountAndBalance49,
    #[serde(rename = "CorpActnDtls", skip_serializing_if = "Option::is_none")]
    pub corp_actn_dtls: Option<CorporateAction62>,
    #[validate]
    #[serde(rename = "CorpActnConfDtls")]
    pub corp_actn_conf_dtls: CorporateActionOption195,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionNarrative31>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IssrAgt", default)]
    pub issr_agt: Vec<PartyIdentification120Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PngAgt", default)]
    pub png_agt: Vec<PartyIdentification120Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SubPngAgt", default)]
    pub sub_png_agt: Vec<PartyIdentification120Choice>,
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
pub struct PartyIdentification122ChoiceEnum {
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
pub struct PartyIdentification122Choice {
    #[serde(flatten)]
    pub value: PartyIdentification122ChoiceEnum,
}
#[derive(
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
pub struct AmountToAmountRatio2 {
    #[validate]
    #[serde(rename = "Amt1")]
    pub amt_1: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "Amt2")]
    pub amt_2: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat37ChoiceEnum {
    #[serde(
        rename = "RateTpAndAmtAndRateSts",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_tp_and_amt_and_rate_sts: Option<RateTypeAndAmountAndStatus57>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "AmtAndRateSts", skip_serializing_if = "Option::is_none")]
    pub amt_and_rate_sts: Option<AmountAndRateStatus1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GrossDividendRateFormat37Choice {
    #[serde(flatten)]
    pub value: GrossDividendRateFormat37ChoiceEnum,
}
#[derive(
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
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat19ChoiceEnum {
    #[serde(rename = "QtyToQty", skip_serializing_if = "Option::is_none")]
    pub qty_to_qty: Option<QuantityToQuantityRatio1>,
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio2>,
    #[serde(rename = "QtyToAmt", skip_serializing_if = "Option::is_none")]
    pub qty_to_amt: Option<AmountAndQuantityRatio4>,
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
pub struct RatioFormat19Choice {
    #[serde(flatten)]
    pub value: RatioFormat19ChoiceEnum,
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
pub struct CorporateActionBalanceDetails41 {
    #[serde(rename = "ConfdBal")]
    pub confd_bal: BalanceFormat11Choice,
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
    #[serde(rename = "AfctdBal", skip_serializing_if = "Option::is_none")]
    pub afctd_bal: Option<BalanceFormat11Choice>,
    #[serde(rename = "UafctdBal", skip_serializing_if = "Option::is_none")]
    pub uafctd_bal: Option<BalanceFormat11Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RatioFormat20ChoiceEnum {
    #[serde(rename = "AmtToAmt", skip_serializing_if = "Option::is_none")]
    pub amt_to_amt: Option<AmountToAmountRatio2>,
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
pub struct RatioFormat20Choice {
    #[serde(flatten)]
    pub value: RatioFormat20ChoiceEnum,
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
pub struct OptionNumber1ChoiceEnum {
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Exact3NumericText>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionNumber1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionNumber1Choice {
    #[serde(flatten)]
    pub value: OptionNumber1ChoiceEnum,
}
#[derive(
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
pub struct RateTypeAndAmountAndStatus57 {
    #[serde(rename = "RateTp")]
    pub rate_tp: RateType78Choice,
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
pub struct SafekeepingPlaceFormat29ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText8>,
}
#[derive(
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
pub struct FinancialInstrumentQuantity33ChoiceEnum {
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
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
#[derive(
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
pub struct FractionDispositionType27ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FractionDispositionType11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FractionDispositionType27Choice {
    #[serde(flatten)]
    pub value: FractionDispositionType27ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct AdditionalBusinessProcessFormat19ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AdditionalBusinessProcess11Code>,
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
pub struct AdditionalBusinessProcessFormat19Choice {
    #[serde(flatten)]
    pub value: AdditionalBusinessProcessFormat19ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FractionDispositionType11Code {
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
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionPrice74 {
    #[serde(rename = "CshInLieuOfShrPric", skip_serializing_if = "Option::is_none")]
    pub csh_in_lieu_of_shr_pric: Option<PriceFormat50Choice>,
    #[serde(rename = "IndctvOrMktPric", skip_serializing_if = "Option::is_none")]
    pub indctv_or_mkt_pric: Option<IndicativeOrMarketPrice8Choice>,
    #[serde(rename = "CshValForTax", skip_serializing_if = "Option::is_none")]
    pub csh_val_for_tax: Option<AmountPrice2>,
    #[serde(
        rename = "GncCshPricPdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_pd_per_pdct: Option<PriceFormat51Choice>,
    #[serde(
        rename = "GncCshPricRcvdPerPdct",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnc_csh_pric_rcvd_per_pdct: Option<PriceFormat64Choice>,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WithholdingTaxRateType1Code>,
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
pub struct BalanceFormat12ChoiceEnum {
    #[serde(rename = "FullPrdUnits", skip_serializing_if = "Option::is_none")]
    pub full_prd_units: Option<SignedQuantityFormat10>,
    #[serde(rename = "PartWayPrdUnits", skip_serializing_if = "Option::is_none")]
    pub part_way_prd_units: Option<SignedQuantityFormat10>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat11>,
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat10>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CorpActnMvmntConf")]
    pub corp_actn_mvmnt_conf: CorporateActionMovementConfirmationV13<A>,
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
pub struct RateType78ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
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
pub struct RateType78Choice {
    #[serde(flatten)]
    pub value: RateType78ChoiceEnum,
}
#[derive(
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
pub struct CorporateActionOption33ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionOption12Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption33Choice {
    #[serde(flatten)]
    pub value: CorporateActionOption33ChoiceEnum,
}
#[derive(
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
pub struct RateType76ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GrossDividendRateType6Code>,
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
pub struct RateType76Choice {
    #[serde(flatten)]
    pub value: RateType76ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalBusinessProcess11Code {
    #[serde(rename = "CLAI")]
    Clai,
    #[serde(rename = "TAXR")]
    Taxr,
    #[serde(rename = "ACLA")]
    Acla,
    #[serde(rename = "ATXF")]
    Atxf,
    #[serde(rename = "CNTR")]
    Cntr,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "NAMC")]
    Namc,
    #[serde(rename = "NPLE")]
    Nple,
    #[serde(rename = "SCHM")]
    Schm,
    #[serde(rename = "PPUT")]
    Pput,
    #[serde(rename = "PPRE")]
    Ppre,
    #[serde(rename = "FPRE")]
    Fpre,
    #[serde(rename = "INCP")]
    Incp,
    #[default]
    Unknown,
}
