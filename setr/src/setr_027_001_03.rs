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
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_NON_FI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_20022_MESSAGE_IDENTIFICATION_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{4}\.[0-9]{3}\.[0-9]{3}\.[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{1,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:setr.027.001.03".to_string()
}

#[derive(
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
pub struct GenericIdentification20 {
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
pub struct OtherPrices2 {
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<Price4>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<Price4>,
    #[serde(rename = "MktBrkrComssn", skip_serializing_if = "Option::is_none")]
    pub mkt_brkr_comssn: Option<Price4>,
    #[serde(rename = "MrkdUp", skip_serializing_if = "Option::is_none")]
    pub mrkd_up: Option<Price4>,
    #[serde(rename = "MrkdDwn", skip_serializing_if = "Option::is_none")]
    pub mrkd_dwn: Option<Price4>,
    #[serde(rename = "NetDscld", skip_serializing_if = "Option::is_none")]
    pub net_dscld: Option<Price4>,
    #[serde(rename = "NetUdscld", skip_serializing_if = "Option::is_none")]
    pub net_udscld: Option<Price4>,
    #[serde(rename = "NtnlGrss", skip_serializing_if = "Option::is_none")]
    pub ntnl_grss: Option<Price4>,
    #[serde(rename = "BchmkWghtdAvrg", skip_serializing_if = "Option::is_none")]
    pub bchmk_wghtd_avrg: Option<Price4>,
    #[serde(rename = "AllMktsWghtdAvrg", skip_serializing_if = "Option::is_none")]
    pub all_mkts_wghtd_avrg: Option<Price4>,
    #[serde(rename = "Bchmk", skip_serializing_if = "Option::is_none")]
    pub bchmk: Option<Price4>,
    #[serde(rename = "OthrPric", skip_serializing_if = "Option::is_none")]
    pub othr_pric: Option<Price4>,
    #[serde(rename = "IndxPric", skip_serializing_if = "Option::is_none")]
    pub indx_pric: Option<Price4>,
    #[serde(rename = "RptdPric", skip_serializing_if = "Option::is_none")]
    pub rptd_pric: Option<Price4>,
    #[serde(rename = "RefPric", skip_serializing_if = "Option::is_none")]
    pub ref_pric: Option<PriceInformation11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity18ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity18Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity18ChoiceEnum,
}
#[derive(
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
pub struct SettlementDetails43 {
    #[serde(rename = "SttlmTxTp")]
    pub sttlm_tx_tp: SettlementTransactionType1Choice,
    #[serde(rename = "HldInd", skip_serializing_if = "Option::is_none")]
    pub hld_ind: Option<YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<PriorityNumeric3Choice>,
    #[serde(rename = "SttlmInstrGnrtn", skip_serializing_if = "Option::is_none")]
    pub sttlm_instr_gnrtn: Option<SettlementInstructionGeneration1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition11Choice>,
    #[serde(rename = "PrtlSttlmInd", skip_serializing_if = "Option::is_none")]
    pub prtl_sttlm_ind: Option<YesNoIndicator>,
    #[serde(rename = "BnfclOwnrsh", skip_serializing_if = "Option::is_none")]
    pub bnfcl_ownrsh: Option<BeneficialOwnership3Choice>,
    #[serde(rename = "BlckTrad", skip_serializing_if = "Option::is_none")]
    pub blck_trad: Option<BlockTrade3Choice>,
    #[serde(rename = "CCPElgblty", skip_serializing_if = "Option::is_none")]
    pub ccp_elgblty: Option<CentralCounterPartyEligibility3Choice>,
    #[serde(rename = "CshClrSys", skip_serializing_if = "Option::is_none")]
    pub csh_clr_sys: Option<CashSettlementSystem3Choice>,
    #[serde(rename = "XpsrTp", skip_serializing_if = "Option::is_none")]
    pub xpsr_tp: Option<ExposureType9Choice>,
    #[serde(rename = "FxStgInstr", skip_serializing_if = "Option::is_none")]
    pub fx_stg_instr: Option<FxStandingInstruction3Choice>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "MktClntSd", skip_serializing_if = "Option::is_none")]
    pub mkt_clnt_sd: Option<MarketClientSide3Choice>,
    #[serde(rename = "NetgElgblty", skip_serializing_if = "Option::is_none")]
    pub netg_elgblty: Option<NettingEligibility3Choice>,
    #[serde(rename = "Regn", skip_serializing_if = "Option::is_none")]
    pub regn: Option<Registration6Choice>,
    #[serde(rename = "RpTp", skip_serializing_if = "Option::is_none")]
    pub rp_tp: Option<RepurchaseType11Choice>,
    #[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctns: Option<Restriction3Choice>,
    #[serde(rename = "SctiesRTGS", skip_serializing_if = "Option::is_none")]
    pub scties_rtgs: Option<SecuritiesRtgs3Choice>,
    #[serde(rename = "SttlgCpcty", skip_serializing_if = "Option::is_none")]
    pub sttlg_cpcty: Option<SettlingCapacity3Choice>,
    #[serde(rename = "SttlmSysMtd", skip_serializing_if = "Option::is_none")]
    pub sttlm_sys_mtd: Option<SettlementSystemMethod3Choice>,
    #[serde(rename = "TaxCpcty", skip_serializing_if = "Option::is_none")]
    pub tax_cpcty: Option<TaxCapacityParty3Choice>,
    #[serde(rename = "StmpDtyInd", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_ind: Option<YesNoIndicator>,
    #[serde(rename = "StmpDtyTaxBsis", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_tax_bsis: Option<GenericIdentification38>,
    #[serde(rename = "Trckg", skip_serializing_if = "Option::is_none")]
    pub trckg: Option<Tracking3Choice>,
    #[serde(rename = "AutomtcBrrwg", skip_serializing_if = "Option::is_none")]
    pub automtc_brrwg: Option<AutomaticBorrowing5Choice>,
    #[serde(rename = "LttrOfGrnt", skip_serializing_if = "Option::is_none")]
    pub lttr_of_grnt: Option<LetterOfGuarantee3Choice>,
    #[serde(rename = "RtrLeg", skip_serializing_if = "Option::is_none")]
    pub rtr_leg: Option<YesNoIndicator>,
    #[serde(rename = "ModCxlAllwd", skip_serializing_if = "Option::is_none")]
    pub mod_cxl_allwd: Option<ModificationCancellationAllowed3Choice>,
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
pub struct PartyIdentificationAndAccount77 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
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
pub struct CashSettlementSystem3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashSettlementSystem2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashSettlementSystem3Choice {
    #[serde(flatten)]
    pub value: CashSettlementSystem3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Restriction3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OwnershipLegalRestrictions1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Restriction3Choice {
    #[serde(flatten)]
    pub value: Restriction3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reversible1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Reversible1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reversible1Choice {
    #[serde(flatten)]
    pub value: Reversible1ChoiceEnum,
}
#[derive(
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
pub struct SimpleIdentificationInformation2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
}
#[derive(
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
    #[serde(rename = "SctiesTradConf")]
    pub scties_trad_conf: SecuritiesTradeConfirmationV03<A>,
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
pub struct PostalAddress8 {
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
pub enum Registration1Code {
    #[serde(rename = "NREG")]
    Nreg,
    #[serde(rename = "YREG")]
    Yreg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDate5Code {
    #[serde(rename = "REGU")]
    Regu,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "NXTD")]
    Nxtd,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "TTWO")]
    Ttwo,
    #[serde(rename = "TTRE")]
    Ttre,
    #[serde(rename = "TFOR")]
    Tfor,
    #[serde(rename = "TFIV")]
    Tfiv,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "WDIS")]
    Wdis,
    #[serde(rename = "WHID")]
    Whid,
    #[serde(rename = "TBAT")]
    Tbat,
    #[serde(rename = "WISS")]
    Wiss,
    #[default]
    Unknown,
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
pub struct OriginalAndCurrentQuantities1 {
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition7Code {
    #[serde(rename = "ASGN")]
    Asgn,
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
    #[serde(rename = "CSDP")]
    Csdp,
    #[serde(rename = "SPCS")]
    Spcs,
    #[serde(rename = "SPDL")]
    Spdl,
    #[serde(rename = "SPST")]
    Spst,
    #[serde(rename = "UNEX")]
    Unex,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tracking3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct Tracking3Choice {
    #[serde(flatten)]
    pub value: Tracking3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashParties18 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentificationAndAccount80>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<PartyIdentificationAndAccount80>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentificationAndAccount80>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<PartyIdentificationAndAccount80>,
    #[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
    pub intrmy: Option<PartyIdentificationAndAccount80>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementInstructionGeneration1Code {
    #[serde(rename = "GENS")]
    Gens,
    #[serde(rename = "NOGE")]
    Noge,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max34Text>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification2Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDate5Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDateCode5Choice {
    #[serde(flatten)]
    pub value: SettlementDateCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationReference8ChoiceEnum {
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "BlckId", skip_serializing_if = "Option::is_none")]
    pub blck_id: Option<Max35Text>,
    #[serde(rename = "IndvAllcnId", skip_serializing_if = "Option::is_none")]
    pub indv_allcn_id: Option<Max35Text>,
    #[serde(rename = "IndxId", skip_serializing_if = "Option::is_none")]
    pub indx_id: Option<Max35Text>,
    #[serde(rename = "CmplcId", skip_serializing_if = "Option::is_none")]
    pub cmplc_id: Option<Max35Text>,
    #[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<Max35Text>,
    #[serde(rename = "InstgPtyTxId", skip_serializing_if = "Option::is_none")]
    pub instg_pty_tx_id: Option<Max35Text>,
    #[serde(rename = "CollTxId", skip_serializing_if = "Option::is_none")]
    pub coll_tx_id: Option<Max35Text>,
    #[serde(rename = "ExctgPtyTxId", skip_serializing_if = "Option::is_none")]
    pub exctg_pty_tx_id: Option<Max35Text>,
    #[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
    pub cmon_id: Option<Max35Text>,
    #[serde(rename = "ClntOrdrLkId", skip_serializing_if = "Option::is_none")]
    pub clnt_ordr_lk_id: Option<Max35Text>,
    #[serde(rename = "ScndryAllcnId", skip_serializing_if = "Option::is_none")]
    pub scndry_allcn_id: Option<Max35Text>,
    #[serde(rename = "AllcnId", skip_serializing_if = "Option::is_none")]
    pub allcn_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationReference8Choice {
    #[serde(flatten)]
    pub value: IdentificationReference8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrName1ChoiceEnum {
    #[serde(rename = "RateNm", skip_serializing_if = "Option::is_none")]
    pub rate_nm: Option<RateName1>,
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
pub struct RateOrName1Choice {
    #[serde(flatten)]
    pub value: RateOrName1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency1Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxCapacityParty3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxLiability1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxCapacityParty3Choice {
    #[serde(flatten)]
    pub value: TaxCapacityParty3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification2Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
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
pub struct MarketIdentification80 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification3Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<MarketType12Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TwoLegTransactionDetails1 {
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate1Choice>,
    #[serde(rename = "OpngLegId", skip_serializing_if = "Option::is_none")]
    pub opng_leg_id: Option<Max35Text>,
    #[serde(rename = "ClsgLegId", skip_serializing_if = "Option::is_none")]
    pub clsg_leg_id: Option<Max35Text>,
    #[serde(rename = "GrssTradAmt", skip_serializing_if = "Option::is_none")]
    pub grss_trad_amt: Option<AmountAndDirection29>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmts", default)]
    pub othr_amts: Vec<OtherAmounts16>,
    #[serde(rename = "ScndLegNrrtv", skip_serializing_if = "Option::is_none")]
    pub scnd_leg_nrrtv: Option<Max140Text>,
    #[serde(rename = "EndPric", skip_serializing_if = "Option::is_none")]
    pub end_pric: Option<Price4>,
    #[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
    pub clsg_dt: Option<ClosingDate1Choice>,
    #[serde(rename = "ClsgSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub clsg_sttlm_amt: Option<AmountAndDirection5>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<TradeDate4Choice>,
    #[serde(rename = "TwoLegTxTp", skip_serializing_if = "Option::is_none")]
    pub two_leg_tx_tp: Option<TwoLegTransactionType1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate8ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementDateCode5Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDate8Choice {
    #[serde(flatten)]
    pub value: SettlementDate8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification70ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
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
pub struct PartyIdentification70Choice {
    #[serde(flatten)]
    pub value: PartyIdentification70ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification7 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max8Text,
    #[validate]
    #[serde(rename = "Inf")]
    pub inf: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification35ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification35Choice {
    #[serde(flatten)]
    pub value: PartyIdentification35ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SpreadRate1 {
    #[validate]
    #[serde(rename = "Sgn")]
    pub sgn: PlusOrMinusIndicator,
    #[serde(rename = "RateOrAmt")]
    pub rate_or_amt: AmountOrRate1Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingSide1Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORW")]
    Borw,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodDetails1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LegalFramework1Code {
    #[serde(rename = "FRAN")]
    Fran,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStandingInstructionDatabase3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementStandingInstructionDatabase1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStandingInstructionDatabase3Choice {
    #[serde(flatten)]
    pub value: SettlementStandingInstructionDatabase3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Appearance1Code {
    #[serde(rename = "DELI")]
    Deli,
    #[serde(rename = "NDEL")]
    Ndel,
    #[serde(rename = "LIMI")]
    Limi,
    #[serde(rename = "BENT")]
    Bent,
    #[serde(rename = "DFBE")]
    Dfbe,
    #[serde(rename = "DLBE")]
    Dlbe,
    #[serde(rename = "TMPG")]
    Tmpg,
    #[serde(rename = "GLOB")]
    Glob,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Reporting5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct Reporting5Choice {
    #[serde(flatten)]
    pub value: Reporting5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClosingType1Code {
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "FLEX")]
    Flex,
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
pub struct Order17 {
    #[serde(rename = "BizPrcTp", skip_serializing_if = "Option::is_none")]
    pub biz_prc_tp: Option<BusinessProcessType1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OrdrId", default)]
    pub ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntOrdrId", default)]
    pub clnt_ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndryClntOrdrId", default)]
    pub scndry_clnt_ordr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ListId", default)]
    pub list_id: Vec<Max35Text>,
    #[serde(rename = "Sd")]
    pub sd: Side3Code,
    #[serde(rename = "Pmt", skip_serializing_if = "Option::is_none")]
    pub pmt: Option<DeliveryReceiptType2Code>,
    #[serde(rename = "TradTxTp", skip_serializing_if = "Option::is_none")]
    pub trad_tx_tp: Option<TradeType3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition4Choice>,
    #[serde(rename = "PreAdvc", skip_serializing_if = "Option::is_none")]
    pub pre_advc: Option<YesNoIndicator>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<MarketIdentification79>,
    #[serde(rename = "OrdrBookgDt", skip_serializing_if = "Option::is_none")]
    pub ordr_bookg_dt: Option<IsoDate>,
    #[serde(rename = "TradOrgtnDt", skip_serializing_if = "Option::is_none")]
    pub trad_orgtn_dt: Option<IsoDateTime>,
    #[serde(rename = "TradDt")]
    pub trad_dt: TradeDate4Choice,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<TradeDate4Choice>,
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: SettlementDate8Choice,
    #[serde(rename = "NAVDt", skip_serializing_if = "Option::is_none")]
    pub nav_dt: Option<DateAndDateTime1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtlFillDtls", default)]
    pub prtl_fill_dtls: Vec<PartialFill2>,
    #[serde(rename = "ConfQty")]
    pub conf_qty: Quantity6Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "QtyBrkdwn", default)]
    pub qty_brkdwn: Vec<QuantityBreakdown11>,
    #[serde(rename = "GrssTradAmt", skip_serializing_if = "Option::is_none")]
    pub grss_trad_amt: Option<AmountAndDirection29>,
    #[validate]
    #[serde(rename = "DealPric")]
    pub deal_pric: Price4,
    #[serde(rename = "TpOfPric", skip_serializing_if = "Option::is_none")]
    pub tp_of_pric: Option<TypeOfPrice10Choice>,
    #[serde(rename = "CshMrgn", skip_serializing_if = "Option::is_none")]
    pub csh_mrgn: Option<CashMarginOrder1Code>,
    #[serde(rename = "Comssn", skip_serializing_if = "Option::is_none")]
    pub comssn: Option<Commission16>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "GvUpNbOfDays", skip_serializing_if = "Option::is_none")]
    pub gv_up_nb_of_days: Option<Max3Number>,
    #[serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none")]
    pub intrst_tp: Option<InterestType2Code>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection29>,
    #[serde(rename = "AcrdIntrstPctg", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_pctg: Option<PercentageRate>,
    #[serde(rename = "TradRgltryCondsTp", skip_serializing_if = "Option::is_none")]
    pub trad_rgltry_conds_tp: Option<TradeRegulatoryConditions1Code>,
    #[serde(rename = "CcyToBuyOrSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy_or_sell: Option<CurrencyToBuyOrSell1Choice>,
    #[serde(rename = "OrdrOrgtrElgblty", skip_serializing_if = "Option::is_none")]
    pub ordr_orgtr_elgblty: Option<Eligibility1Code>,
    #[serde(rename = "PosFct", skip_serializing_if = "Option::is_none")]
    pub pos_fct: Option<PositionEffect2Code>,
    #[serde(rename = "DerivCvrd", skip_serializing_if = "Option::is_none")]
    pub deriv_cvrd: Option<YesNoIndicator>,
    #[serde(rename = "ChrgTaxBsisTp", skip_serializing_if = "Option::is_none")]
    pub chrg_tax_bsis_tp: Option<ChargeTaxBasisType1Choice>,
    #[serde(rename = "CptlGnTp", skip_serializing_if = "Option::is_none")]
    pub cptl_gn_tp: Option<EuCapitalGainType2Choice>,
    #[serde(rename = "MtchSts", skip_serializing_if = "Option::is_none")]
    pub mtch_sts: Option<MatchingStatus8Choice>,
    #[serde(rename = "CallInTp", skip_serializing_if = "Option::is_none")]
    pub call_in_tp: Option<CallIn1Code>,
    #[serde(rename = "YldTp", skip_serializing_if = "Option::is_none")]
    pub yld_tp: Option<YieldCalculation2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rptg", default)]
    pub rptg: Vec<Reporting5Choice>,
    #[serde(
        rename = "AddtlPhysOrRegnDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_phys_or_regn_dtls: Option<RegistrationParameters3>,
    #[serde(
        rename = "AddtlTradInstrPrcgInf",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_trad_instr_prcg_inf: Option<Max350Text>,
}
#[derive(
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
pub struct ExposureType9ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExposureType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureType9Choice {
    #[serde(flatten)]
    pub value: ExposureType9ChoiceEnum,
}
#[derive(
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
pub struct SecurityIdentification14 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
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
pub struct TotalNumber1 {
    #[validate]
    #[serde(rename = "CurInstrNb")]
    pub cur_instr_nb: Exact3NumericText,
    #[validate]
    #[serde(rename = "TtlOfLkdInstrs")]
    pub ttl_of_lkd_instrs: Exact3NumericText,
}
#[derive(
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
pub struct FormOfSecurity4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FormOfSecurity1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FormOfSecurity4Choice {
    #[serde(flatten)]
    pub value: FormOfSecurity4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesRtgs3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct SecuritiesRtgs3Choice {
    #[serde(flatten)]
    pub value: SecuritiesRtgs3ChoiceEnum,
}
#[derive(
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
pub struct AmountAndDirection28 {
    #[serde(rename = "AcrdIntrstInd", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_ind: Option<YesNoIndicator>,
    #[serde(rename = "StmpDtyInd", skip_serializing_if = "Option::is_none")]
    pub stmp_dty_ind: Option<YesNoIndicator>,
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
    pub fx_dtls: Option<ForeignExchangeTerms18>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateAndDateTime1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketClientSideCode {
    #[serde(rename = "MAKT")]
    Makt,
    #[serde(rename = "CLNT")]
    Clnt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodChoiceEnum {
    #[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
    pub dt_tm_rg: Option<DateTimePeriodDetails>,
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriodChoice {
    #[serde(flatten)]
    pub value: DateTimePeriodChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketClientSide3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketClientSideCode>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketClientSide3Choice {
    #[serde(flatten)]
    pub value: MarketClientSide3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operation1Code {
    #[serde(rename = "TILL")]
    Till,
    #[serde(rename = "ORRR")]
    Orrr,
    #[serde(rename = "ANDD")]
    Andd,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalTradeTransactionCondition1Code {
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
pub struct AutomaticBorrowing5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AutoBorrowing1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutomaticBorrowing5Choice {
    #[serde(flatten)]
    pub value: AutomaticBorrowing5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationPartyDetails1 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification7>,
    #[serde(rename = "NbId", skip_serializing_if = "Option::is_none")]
    pub nb_id: Option<Max3NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number1Choice {
    #[serde(flatten)]
    pub value: Number1ChoiceEnum,
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
pub struct UnderlyingRatio1 {
    #[serde(rename = "UndrlygQtyDnmtr")]
    pub undrlyg_qty_dnmtr: FinancialInstrumentQuantity1Choice,
    #[serde(rename = "UndrlygQtyNmrtr")]
    pub undrlyg_qty_nmrtr: FinancialInstrumentQuantity1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdFinInstrmId", default)]
    pub rltd_fin_instrm_id: Vec<SecurityIdentification14>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldCalculation2 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
    #[serde(rename = "ClctnTp")]
    pub clctn_tp: CalculationType1Code,
    #[serde(rename = "RedPric", skip_serializing_if = "Option::is_none")]
    pub red_pric: Option<Price4>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "ValPrd", skip_serializing_if = "Option::is_none")]
    pub val_prd: Option<DateTimePeriodChoice>,
    #[serde(rename = "ClctnDt", skip_serializing_if = "Option::is_none")]
    pub clctn_dt: Option<IsoDate>,
}
#[derive(
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
pub struct TradeType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeType3Choice {
    #[serde(flatten)]
    pub value: TradeType3ChoiceEnum,
}
#[derive(
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
pub struct PartialFill2 {
    #[serde(rename = "ConfQty")]
    pub conf_qty: Quantity6Choice,
    #[validate]
    #[serde(rename = "DealPric")]
    pub deal_pric: Price4,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<TradeDate4Choice>,
    #[serde(rename = "PlcOfTrad", skip_serializing_if = "Option::is_none")]
    pub plc_of_trad: Option<MarketIdentification80>,
    #[serde(rename = "OrgnlOrdrdQty")]
    pub orgnl_ordrd_qty: QuantityOrAmount1Choice,
    #[serde(rename = "PrevslyExctdQty")]
    pub prevsly_exctd_qty: QuantityOrAmount1Choice,
    #[serde(rename = "RmngQty")]
    pub rmng_qty: QuantityOrAmount1Choice,
    #[serde(rename = "MtchIncrmtQty", skip_serializing_if = "Option::is_none")]
    pub mtch_incrmt_qty: Option<QuantityOrAmount1Choice>,
}
#[derive(
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
pub struct AmountOrRate2ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct AmountOrRate2Choice {
    #[serde(flatten)]
    pub value: AmountOrRate2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
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
pub struct BorrowingReason1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BorrowingReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BorrowingReason1Choice {
    #[serde(flatten)]
    pub value: BorrowingReason1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FutureAndOptionContractType1Code {
    #[serde(rename = "ORDY")]
    Ordy,
    #[serde(rename = "INDX")]
    Indx,
    #[serde(rename = "EXFU")]
    Exfu,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlingCapacity1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "SAGE")]
    Sage,
    #[serde(rename = "SPRI")]
    Spri,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity4Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "CPRN")]
    Cprn,
    #[serde(rename = "RISP")]
    Risp,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "CAGN")]
    Cagn,
    #[serde(rename = "OAGN")]
    Oagn,
    #[serde(rename = "PRAG")]
    Prag,
    #[serde(rename = "BAGN")]
    Bagn,
    #[serde(rename = "INFI")]
    Infi,
    #[serde(rename = "MKTM")]
    Mktm,
    #[serde(rename = "MLTF")]
    Mltf,
    #[serde(rename = "RMKT")]
    Rmkt,
    #[serde(rename = "SINT")]
    Sint,
    #[serde(rename = "TAGT")]
    Tagt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity6Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "BAGN")]
    Bagn,
    #[serde(rename = "CAGN")]
    Cagn,
    #[serde(rename = "CPRN")]
    Cprn,
    #[serde(rename = "OAGN")]
    Oagn,
    #[serde(rename = "PRAG")]
    Prag,
    #[serde(rename = "PRIN")]
    Prin,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesCertificate3 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
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
pub struct IdentificationType40ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType40Choice {
    #[serde(flatten)]
    pub value: IdentificationType40ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice10ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice10Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice10ChoiceEnum,
}
#[derive(
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
pub struct TransactiontIdentification4 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount87 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification70Choice,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification6>,
}
#[derive(
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
pub struct CurrencyCode {
    #[validate(regex = "CURRENCY_CODE_REGEX")]
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
pub struct ConfirmationPartyDetails2 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(rename = "InvstrCpcty", skip_serializing_if = "Option::is_none")]
    pub invstr_cpcty: Option<InvestorCapacity3Choice>,
    #[serde(rename = "TradgPtyCpcty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty_cpcty: Option<TradingPartyCapacity1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection29 {
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
    pub fx_dtls: Option<ForeignExchangeTerms18>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType7Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "YIEL")]
    Yiel,
    #[serde(rename = "SPRE")]
    Spre,
    #[serde(rename = "PEUN")]
    Peun,
    #[serde(rename = "ABSO")]
    Abso,
    #[serde(rename = "TEDP")]
    Tedp,
    #[serde(rename = "TEDY")]
    Tedy,
    #[serde(rename = "FICT")]
    Fict,
    #[serde(rename = "VACT")]
    Vact,
    #[serde(rename = "PRCT")]
    Prct,
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
pub struct Rate2 {
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionType7Code {
    #[serde(rename = "BSBK")]
    Bsbk,
    #[serde(rename = "COLI")]
    Coli,
    #[serde(rename = "COLO")]
    Colo,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "FCTA")]
    Fcta,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "MKDW")]
    Mkdw,
    #[serde(rename = "MKUP")]
    Mkup,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "NSYN")]
    Nsyn,
    #[serde(rename = "OWNE")]
    Owne,
    #[serde(rename = "OWNI")]
    Owni,
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PORT")]
    Port,
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "REDI")]
    Redi,
    #[serde(rename = "RELE")]
    Rele,
    #[serde(rename = "REPU")]
    Repu,
    #[serde(rename = "RODE")]
    Rode,
    #[serde(rename = "RPTO")]
    Rpto,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SBBK")]
    Sbbk,
    #[serde(rename = "SECB")]
    Secb,
    #[serde(rename = "SECL")]
    Secl,
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
    #[serde(rename = "GUAR")]
    Guar,
    #[serde(rename = "OFIT")]
    Ofit,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ForeignExchangeTerms18 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[validate]
    #[serde(rename = "ConvtdAmt")]
    pub convtd_amt: ActiveCurrencyAndAmount,
}
#[derive(
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
pub enum CallIn1Code {
    #[serde(rename = "CFAV")]
    Cfav,
    #[serde(rename = "CFST")]
    Cfst,
    #[serde(rename = "CFCC")]
    Cfcc,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification37 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingAccountType1Code {
    #[serde(rename = "HOUS")]
    Hous,
    #[serde(rename = "CLIE")]
    Clie,
    #[serde(rename = "LIPR")]
    Lipr,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestType2Code {
    #[serde(rename = "CINT")]
    Cint,
    #[serde(rename = "XINT")]
    Xint,
    #[default]
    Unknown,
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
pub struct PartyIdentification32ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification32Choice {
    #[serde(flatten)]
    pub value: PartyIdentification32ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicNonFiIdentifier {
    #[validate(regex = "BIC_NON_FI_IDENTIFIER_REGEX")]
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
pub struct MarketType11ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MarketType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketType11Choice {
    #[serde(flatten)]
    pub value: MarketType11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrRate1ChoiceEnum {
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
pub struct AmountOrRate1Choice {
    #[serde(flatten)]
    pub value: AmountOrRate1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NettingEligibility3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct NettingEligibility3Choice {
    #[serde(flatten)]
    pub value: NettingEligibility3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CalculationType1Code {
    #[serde(rename = "AFTX")]
    Aftx,
    #[serde(rename = "ANNU")]
    Annu,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "AVMA")]
    Avma,
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "YTNC")]
    Ytnc,
    #[serde(rename = "CHCL")]
    Chcl,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "CUYI")]
    Cuyi,
    #[serde(rename = "TRGR")]
    Trgr,
    #[serde(rename = "GVEQ")]
    Gveq,
    #[serde(rename = "FLAS")]
    Flas,
    #[serde(rename = "NVFL")]
    Nvfl,
    #[serde(rename = "LSCL")]
    Lscl,
    #[serde(rename = "LSMT")]
    Lsmt,
    #[serde(rename = "LSQR")]
    Lsqr,
    #[serde(rename = "LSYR")]
    Lsyr,
    #[serde(rename = "LGAL")]
    Lgal,
    #[serde(rename = "MARK")]
    Mark,
    #[serde(rename = "YTMA")]
    Ytma,
    #[serde(rename = "NXRF")]
    Nxrf,
    #[serde(rename = "PNAV")]
    Pnav,
    #[serde(rename = "NXPT")]
    Nxpt,
    #[serde(rename = "PRCL")]
    Prcl,
    #[serde(rename = "PRYL")]
    Pryl,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "SHLF")]
    Shlf,
    #[serde(rename = "SPLL")]
    Spll,
    #[serde(rename = "TXQV")]
    Txqv,
    #[serde(rename = "TTDT")]
    Ttdt,
    #[serde(rename = "TRYL")]
    Tryl,
    #[serde(rename = "WRST")]
    Wrst,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegistrationParameters3 {
    #[serde(rename = "CertfctnId", skip_serializing_if = "Option::is_none")]
    pub certfctn_id: Option<Max35Text>,
    #[serde(rename = "CertfctnDtTm", skip_serializing_if = "Option::is_none")]
    pub certfctn_dt_tm: Option<DateAndDateTime1Choice>,
    #[serde(rename = "RegarAcct", skip_serializing_if = "Option::is_none")]
    pub regar_acct: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<SecuritiesCertificate3>,
}
#[derive(
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
pub struct PartyIdentificationAndAccount79 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification32Choice>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType41ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType41Choice {
    #[serde(flatten)]
    pub value: IdentificationType41ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeRegulatoryConditions1Code {
    #[serde(rename = "SOLI")]
    Soli,
    #[serde(rename = "USOL")]
    Usol,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity1ChoiceEnum {
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
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
pub struct FinancialInstrumentQuantity1Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType3Code {
    #[serde(rename = "CCIR")]
    Ccir,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CRDS")]
    Crds,
    #[serde(rename = "CRPR")]
    Crpr,
    #[serde(rename = "CRSP")]
    Crsp,
    #[serde(rename = "CRTL")]
    Crtl,
    #[serde(rename = "EQPT")]
    Eqpt,
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXPT")]
    Expt,
    #[serde(rename = "EXTD")]
    Extd,
    #[serde(rename = "FIXI")]
    Fixi,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FORX")]
    Forx,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "OTCD")]
    Otcd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCRP")]
    Scrp,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "SLOA")]
    Sloa,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "TBAS")]
    Tbas,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Term1 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operator1Code,
    #[serde(rename = "Val")]
    pub val: RateOrAbsoluteValue1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation5 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Clearing3 {
    #[validate(length(min = 1,))]
    #[serde(rename = "ClrMmb", default)]
    pub clr_mmb: Vec<PartyIdentificationAndAccount78>,
    #[serde(rename = "ClrSgmt", skip_serializing_if = "Option::is_none")]
    pub clr_sgmt: Option<PartyIdentification35Choice>,
}
#[derive(
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
    #[serde(rename = "CcyToBuy", skip_serializing_if = "Option::is_none")]
    pub ccy_to_buy: Option<ActiveCurrencyCode>,
    #[serde(rename = "CcyToSell", skip_serializing_if = "Option::is_none")]
    pub ccy_to_sell: Option<ActiveCurrencyCode>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition2Code {
    #[serde(rename = "SPCC")]
    Spcc,
    #[serde(rename = "SECN")]
    Secn,
    #[serde(rename = "SEBN")]
    Sebn,
    #[serde(rename = "SCBN")]
    Scbn,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SERT")]
    Sert,
    #[serde(rename = "SCCR")]
    Sccr,
    #[serde(rename = "SECR")]
    Secr,
    #[serde(rename = "CAST")]
    Cast,
    #[serde(rename = "SPPR")]
    Sppr,
    #[serde(rename = "SPCU")]
    Spcu,
    #[serde(rename = "SPEX")]
    Spex,
    #[serde(rename = "GTDL")]
    Gtdl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeTaxBasisType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ChargeTaxBasis1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeTaxBasisType1Choice {
    #[serde(flatten)]
    pub value: ChargeTaxBasisType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification55 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification68Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuCapitalGainType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none")]
    pub eu_cptl_gn: Option<EuCapitalGain2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuCapitalGainType2Choice {
    #[serde(flatten)]
    pub value: EuCapitalGainType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification68ChoiceEnum {
    #[serde(rename = "BIC", skip_serializing_if = "Option::is_none")]
    pub bic: Option<AnyBicIdentifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
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
pub struct PartyIdentification68Choice {
    #[serde(flatten)]
    pub value: PartyIdentification68ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethodFormat3Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethodFormat3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionType7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionType1Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionType1ChoiceEnum,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashMarginOrder1Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "MRGO")]
    Mrgo,
    #[serde(rename = "MRGC")]
    Mrgc,
    #[default]
    Unknown,
}
#[derive(
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
pub struct QuantityOrAmount1ChoiceEnum {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantityChoice>,
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
pub struct QuantityOrAmount1Choice {
    #[serde(flatten)]
    pub value: QuantityOrAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MatchingStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus8Choice {
    #[serde(flatten)]
    pub value: MatchingStatus8ChoiceEnum,
}
#[derive(
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
pub struct InterestComputationMethod2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestComputationMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestComputationMethod2Choice {
    #[serde(flatten)]
    pub value: InterestComputationMethod2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Side3Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "CROS")]
    Cros,
    #[serde(rename = "CRSH")]
    Crsh,
    #[serde(rename = "CSHE")]
    Cshe,
    #[serde(rename = "DEFI")]
    Defi,
    #[serde(rename = "OPPO")]
    Oppo,
    #[serde(rename = "UNDI")]
    Undi,
    #[serde(rename = "TWOS")]
    Twos,
    #[serde(rename = "BUMI")]
    Bumi,
    #[serde(rename = "SEPL")]
    Sepl,
    #[serde(rename = "SESH")]
    Sesh,
    #[serde(rename = "SSEX")]
    Ssex,
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORW")]
    Borw,
    #[serde(rename = "OPEX")]
    Opex,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount20 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: ClearingAccountType1Code,
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
pub struct Date2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Date2Choice {
    #[serde(flatten)]
    pub value: Date2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingCapacity6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity2Choice {
    #[serde(flatten)]
    pub value: TradingPartyCapacity2ChoiceEnum,
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod1Code {
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
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketIdentification79 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MarketIdentification3Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<MarketType11Choice>,
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
pub struct RateOrAbsoluteValue1ChoiceEnum {
    #[serde(rename = "RateVal", skip_serializing_if = "Option::is_none")]
    pub rate_val: Option<PercentageRate>,
    #[serde(rename = "AbsVal", skip_serializing_if = "Option::is_none")]
    pub abs_val: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrAbsoluteValue1Choice {
    #[serde(flatten)]
    pub value: RateOrAbsoluteValue1ChoiceEnum,
}
#[derive(
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
pub struct DateTimePeriodDetails {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentStipulations2 {
    #[serde(rename = "Geogcs", skip_serializing_if = "Option::is_none")]
    pub geogcs: Option<Max35Text>,
    #[serde(rename = "YldRg", skip_serializing_if = "Option::is_none")]
    pub yld_rg: Option<AmountOrPercentageRange>,
    #[serde(rename = "Ratg", skip_serializing_if = "Option::is_none")]
    pub ratg: Option<Rating1>,
    #[serde(rename = "CpnRg", skip_serializing_if = "Option::is_none")]
    pub cpn_rg: Option<AmountOrPercentageRange>,
    #[serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none")]
    pub amtsbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max256Text>,
    #[serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none")]
    pub altrntv_min_tax_ind: Option<YesNoIndicator>,
    #[serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none")]
    pub auto_rinvstmt: Option<PercentageRate>,
    #[serde(rename = "TxConds", skip_serializing_if = "Option::is_none")]
    pub tx_conds: Option<TradeTransactionCondition2Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<CurrencyCode>,
    #[serde(rename = "CstmDt", skip_serializing_if = "Option::is_none")]
    pub cstm_dt: Option<DateTimePeriodDetails1>,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
    #[serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none")]
    pub insrd_ind: Option<YesNoIndicator>,
    #[serde(rename = "LookBck", skip_serializing_if = "Option::is_none")]
    pub look_bck: Option<Number>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoYearMonth>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoYearMonth>,
    #[serde(rename = "IssrId", skip_serializing_if = "Option::is_none")]
    pub issr_id: Option<BicNonFiIdentifier>,
    #[serde(rename = "IsseSz", skip_serializing_if = "Option::is_none")]
    pub isse_sz: Option<Number>,
    #[serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none")]
    pub min_dnmtn: Option<FinancialInstrumentQuantityChoice>,
    #[serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none")]
    pub max_sbstitn: Option<Number>,
    #[serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none")]
    pub min_incrmt: Option<FinancialInstrumentQuantityChoice>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency1Code>,
    #[serde(rename = "MinQty", skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<FinancialInstrumentQuantityChoice>,
    #[serde(rename = "Pdctn", skip_serializing_if = "Option::is_none")]
    pub pdctn: Option<Max35Text>,
    #[serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none")]
    pub rstrctd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none")]
    pub pric_frqcy: Option<Frequency1Code>,
    #[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
    pub sctr: Option<Max35Text>,
    #[serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none")]
    pub sbstitn_frqcy: Option<Frequency1Code>,
    #[serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none")]
    pub sbstitn_lft: Option<Number>,
    #[serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none")]
    pub whl_pool_ind: Option<YesNoIndicator>,
    #[serde(rename = "PricSrc", skip_serializing_if = "Option::is_none")]
    pub pric_src: Option<Max35Text>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDateTime>,
    #[serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none")]
    pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none")]
    pub over_alltmt_rate: Option<PercentageRate>,
    #[serde(rename = "PricRg", skip_serializing_if = "Option::is_none")]
    pub pric_rg: Option<AmountOrPercentageRange>,
    #[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none")]
    pub convtbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
    pub putbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none")]
    pub pre_fndd_ind: Option<YesNoIndicator>,
    #[serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none")]
    pub escrwd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none")]
    pub perptl_ind: Option<YesNoIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesAccountPurposeType1Code {
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "ABRD")]
    Abrd,
    #[serde(rename = "CEND")]
    Cend,
    #[serde(rename = "DVPA")]
    Dvpa,
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
pub struct RatingValueIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AutoBorrowing1Code {
    #[serde(rename = "LAMI")]
    Lami,
    #[serde(rename = "NBOR")]
    Nbor,
    #[serde(rename = "YBOR")]
    Ybor,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepurchaseType7Code {
    #[serde(rename = "CADJ")]
    Cadj,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "RATE")]
    Rate,
    #[serde(rename = "ROLP")]
    Rolp,
    #[serde(rename = "TOPU")]
    Topu,
    #[serde(rename = "WTHD")]
    Wthd,
    #[serde(rename = "PADJ")]
    Padj,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentageRange {
    #[serde(rename = "Opr", skip_serializing_if = "Option::is_none")]
    pub opr: Option<Operation1Code>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "Term", default)]
    pub term: Vec<Term1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficialOwnership3ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficialOwnership3Choice {
    #[serde(flatten)]
    pub value: BeneficialOwnership3ChoiceEnum,
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
#[derive(
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
pub struct PurposeCode5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesAccountPurposeType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PurposeCode5Choice {
    #[serde(flatten)]
    pub value: PurposeCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LendingTransactionMethod1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LendingTransactionMethod1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LendingTransactionMethod1Choice {
    #[serde(flatten)]
    pub value: LendingTransactionMethod1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepurchaseType11ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepurchaseType7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepurchaseType11Choice {
    #[serde(flatten)]
    pub value: RepurchaseType11ChoiceEnum,
}
#[derive(
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
pub struct FutureOrOptionDetails1 {
    #[serde(rename = "FutrAndOptnCtrctTp", skip_serializing_if = "Option::is_none")]
    pub futr_and_optn_ctrct_tp: Option<FutureAndOptionContractType1Code>,
    #[serde(rename = "LastDlvryDt", skip_serializing_if = "Option::is_none")]
    pub last_dlvry_dt: Option<IsoDateTime>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "FutrDt", skip_serializing_if = "Option::is_none")]
    pub futr_dt: Option<IsoDateTime>,
    #[serde(rename = "MinSz", skip_serializing_if = "Option::is_none")]
    pub min_sz: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<IsoDateTime>,
    #[serde(rename = "Apprnc", skip_serializing_if = "Option::is_none")]
    pub apprnc: Option<Appearance1Code>,
    #[serde(rename = "StrpblInd", skip_serializing_if = "Option::is_none")]
    pub strpbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PosLmt", skip_serializing_if = "Option::is_none")]
    pub pos_lmt: Option<Number>,
    #[serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none")]
    pub near_term_pos_lmt: Option<Number>,
    #[serde(
        rename = "MinTradgPricgIncrmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_tradg_pricg_incrmt: Option<Number>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max256Text>,
    #[serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none")]
    pub ctrct_sttlm_mnth: Option<IsoYearMonth>,
    #[serde(rename = "FrstDealgDt", skip_serializing_if = "Option::is_none")]
    pub frst_dealg_dt: Option<DateAndDateTime1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ratio", default)]
    pub ratio: Vec<UnderlyingRatio1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ratg", default)]
    pub ratg: Vec<Rating1>,
    #[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
    pub isse_pric: Option<Price4>,
    #[serde(rename = "OptnRghts", skip_serializing_if = "Option::is_none")]
    pub optn_rghts: Option<OptionRight1Choice>,
    #[serde(rename = "LastTx", skip_serializing_if = "Option::is_none")]
    pub last_tx: Option<YesNoIndicator>,
    #[serde(rename = "SprdTx", skip_serializing_if = "Option::is_none")]
    pub sprd_tx: Option<YesNoIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BorrowingReason1Code {
    #[serde(rename = "SFCT")]
    Sfct,
    #[serde(rename = "TTTP")]
    Tttp,
    #[serde(rename = "MMPP")]
    Mmpp,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Number21ChoiceEnum {
    #[serde(rename = "NbId", skip_serializing_if = "Option::is_none")]
    pub nb_id: Option<Max4NumericText>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number21Choice {
    #[serde(flatten)]
    pub value: Number21ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceInformation11 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: Price4,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "PricClctnPrd", skip_serializing_if = "Option::is_none")]
    pub pric_clctn_prd: Option<DateTimePeriodChoice>,
    #[serde(rename = "SrcOfPric", skip_serializing_if = "Option::is_none")]
    pub src_of_pric: Option<MarketIdentification79>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationPartyDetails3 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount3>,
    #[serde(rename = "CshDtls", skip_serializing_if = "Option::is_none")]
    pub csh_dtls: Option<AccountIdentification3Choice>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(rename = "PtyCpcty", skip_serializing_if = "Option::is_none")]
    pub pty_cpcty: Option<TradingPartyCapacity2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification29 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
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
pub struct FinancialInstrumentAttributes44 {
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MarketIdentification3Choice>,
    #[serde(rename = "Ratg", skip_serializing_if = "Option::is_none")]
    pub ratg: Option<Rating1>,
    #[serde(rename = "CertNb", skip_serializing_if = "Option::is_none")]
    pub cert_nb: Option<Max35Text>,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethodFormat3Choice>,
    #[serde(rename = "RegnForm", skip_serializing_if = "Option::is_none")]
    pub regn_form: Option<FormOfSecurity4Choice>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency7Choice>,
    #[serde(rename = "VarblRateChngFrqcy", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_chng_frqcy: Option<Frequency7Choice>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType30Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle6Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType4Choice>,
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
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
    pub putbl_dt: Option<IsoDate>,
    #[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
    pub dtd_dt: Option<IsoDate>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr_dt: Option<IsoDate>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<BaseOneRate>,
    #[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
    pub cur_fctr: Option<BaseOneRate>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<BaseOneRate>,
    #[serde(rename = "EndFctr", skip_serializing_if = "Option::is_none")]
    pub end_fctr: Option<BaseOneRate>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<PercentageRate>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<PercentageRate>,
    #[serde(rename = "IndxRateBsis", skip_serializing_if = "Option::is_none")]
    pub indx_rate_bsis: Option<PercentageRate>,
    #[serde(rename = "PctgOfDebtClms", skip_serializing_if = "Option::is_none")]
    pub pctg_of_debt_clms: Option<PercentageRate>,
    #[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
    pub cpn_attchd_nb: Option<Number1Choice>,
    #[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
    pub pool_nb: Option<GenericIdentification37>,
    #[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
    pub vrsn_nb: Option<Number1Choice>,
    #[serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none")]
    pub convtbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_ind: Option<YesNoIndicator>,
    #[serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none")]
    pub cvrd_ind: Option<YesNoIndicator>,
    #[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
    pub putbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "WarrtAttchdOnDlvry", skip_serializing_if = "Option::is_none")]
    pub warrt_attchd_on_dlvry: Option<YesNoIndicator>,
    #[serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none")]
    pub odd_cpn_ind: Option<YesNoIndicator>,
    #[serde(rename = "RedYldImpct", skip_serializing_if = "Option::is_none")]
    pub red_yld_impct: Option<YesNoIndicator>,
    #[serde(rename = "YldVar", skip_serializing_if = "Option::is_none")]
    pub yld_var: Option<YesNoIndicator>,
    #[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
    pub exrc_pric: Option<Price4>,
    #[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
    pub sbcpt_pric: Option<Price4>,
    #[serde(rename = "ConvsPric", skip_serializing_if = "Option::is_none")]
    pub convs_pric: Option<Price4>,
    #[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
    pub taxbl_incm_per_shr: Option<Price4>,
    #[serde(rename = "MinNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_nmnl_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none")]
    pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "MinExrcblMltplQty", skip_serializing_if = "Option::is_none")]
    pub min_exrcbl_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<FinancialInstrumentQuantity18Choice>,
    #[serde(
        rename = "FinInstrmAttrAddtlDtls",
        skip_serializing_if = "Option::is_none"
    )]
    pub fin_instrm_attr_addtl_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BbanIdentifier {
    #[validate(regex = "BBAN_IDENTIFIER_REGEX")]
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
pub struct DateAndDateTime1ChoiceEnum {
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
pub struct DateAndDateTime1Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operator1Code {
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SMEQ")]
    Smeq,
    #[serde(rename = "GREA")]
    Grea,
    #[serde(rename = "GREQ")]
    Greq,
    #[serde(rename = "EQAL")]
    Eqal,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommissionType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CommissionType9Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommissionType2Choice {
    #[serde(flatten)]
    pub value: CommissionType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionType4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct OptionType4Choice {
    #[serde(flatten)]
    pub value: OptionType4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification3ChoiceEnum {
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "PrtryAcct", skip_serializing_if = "Option::is_none")]
    pub prtry_acct: Option<SimpleIdentificationInformation2>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification3Choice {
    #[serde(flatten)]
    pub value: AccountIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmountChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmountChoice {
    #[serde(flatten)]
    pub value: PriceRateOrAmountChoiceEnum,
}
#[derive(
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
pub struct PartyIdentificationAndAccount78 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
    pub sd: Option<ClearingSide1Code>,
    #[serde(rename = "ClrAcct", skip_serializing_if = "Option::is_none")]
    pub clr_acct: Option<SecuritiesAccount20>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle4Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "BERM")]
    Berm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LendingTransactionMethod1Code {
    #[serde(rename = "ODTR")]
    Odtr,
    #[serde(rename = "EXTR")]
    Extr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate1ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTimeChoice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<TradeDateCode1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate1Choice {
    #[serde(flatten)]
    pub value: TradeDate1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed3ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationCancellationAllowed3Choice {
    #[serde(flatten)]
    pub value: ModificationCancellationAllowed3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionStyle4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle6Choice {
    #[serde(flatten)]
    pub value: OptionStyle6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity6ChoiceEnum {
    #[serde(rename = "OrgnlAndCurFace", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face: Option<OriginalAndCurrentQuantities1>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity6Choice {
    #[serde(flatten)]
    pub value: Quantity6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount80 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "ComssnAcct", skip_serializing_if = "Option::is_none")]
    pub comssn_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "TaxAcct", skip_serializing_if = "Option::is_none")]
    pub tax_acct: Option<CashAccountIdentification5Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateCode1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification20>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateCode1Choice {
    #[serde(flatten)]
    pub value: TradeDateCode1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CollateralType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralType1Choice {
    #[serde(flatten)]
    pub value: CollateralType1ChoiceEnum,
}
#[derive(
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
pub struct TradingPartyCapacity1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingCapacity4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingPartyCapacity1Choice {
    #[serde(flatten)]
    pub value: TradingPartyCapacity1ChoiceEnum,
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
pub struct Linkages15 {
    #[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
    pub msg_nb: Option<DocumentNumber4Choice>,
    #[serde(rename = "Ref")]
    pub r#ref: IdentificationReference8Choice,
}
#[derive(
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
pub struct AmountAndDirection5 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebitCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesFinancing10 {
    #[serde(rename = "RateChngDt", skip_serializing_if = "Option::is_none")]
    pub rate_chng_dt: Option<IsoDateTime>,
    #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<RateType19Choice>,
    #[serde(rename = "Rvaltn", skip_serializing_if = "Option::is_none")]
    pub rvaltn: Option<Revaluation2Choice>,
    #[serde(rename = "LglFrmwk", skip_serializing_if = "Option::is_none")]
    pub lgl_frmwk: Option<LegalFramework1Code>,
    #[serde(rename = "IntrstCmptnMtd", skip_serializing_if = "Option::is_none")]
    pub intrst_cmptn_mtd: Option<InterestComputationMethod2Choice>,
    #[serde(rename = "VarblRateSpprt", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_spprt: Option<RateName1>,
    #[serde(rename = "RpRate", skip_serializing_if = "Option::is_none")]
    pub rp_rate: Option<Rate2>,
    #[serde(rename = "StockLnMrgn", skip_serializing_if = "Option::is_none")]
    pub stock_ln_mrgn: Option<Rate2>,
    #[serde(rename = "SctiesHrcut", skip_serializing_if = "Option::is_none")]
    pub scties_hrcut: Option<Rate2>,
    #[serde(rename = "PricgRate", skip_serializing_if = "Option::is_none")]
    pub pricg_rate: Option<RateOrName1Choice>,
    #[serde(rename = "SprdRate", skip_serializing_if = "Option::is_none")]
    pub sprd_rate: Option<SpreadRate1>,
    #[serde(rename = "CllblTradInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_trad_ind: Option<YesNoIndicator>,
    #[serde(rename = "TxCallDely", skip_serializing_if = "Option::is_none")]
    pub tx_call_dely: Option<Max3NumericText>,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<AmountAndDirection5>,
    #[serde(rename = "AcrdIntrstPctg", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_pctg: Option<PercentageRate>,
    #[serde(rename = "FrftAmt", skip_serializing_if = "Option::is_none")]
    pub frft_amt: Option<AmountAndDirection5>,
    #[serde(rename = "PrmAmt", skip_serializing_if = "Option::is_none")]
    pub prm_amt: Option<AmountAndDirection5>,
    #[serde(
        rename = "ClsgAmtPerPcsOfColl",
        skip_serializing_if = "Option::is_none"
    )]
    pub clsg_amt_per_pcs_of_coll: Option<AmountAndDirection5>,
    #[serde(rename = "TtlNbOfCollInstrs", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_coll_instrs: Option<Max3NumericText>,
    #[serde(rename = "FincgAgrmt", skip_serializing_if = "Option::is_none")]
    pub fincg_agrmt: Option<Agreement3>,
    #[serde(rename = "LndgTxMtd", skip_serializing_if = "Option::is_none")]
    pub lndg_tx_mtd: Option<LendingTransactionMethod1Choice>,
    #[serde(rename = "LndgWthColl", skip_serializing_if = "Option::is_none")]
    pub lndg_wth_coll: Option<YesNoIndicator>,
    #[serde(rename = "BrrwgRsn", skip_serializing_if = "Option::is_none")]
    pub brrwg_rsn: Option<BorrowingReason1Choice>,
    #[serde(rename = "CollTp", skip_serializing_if = "Option::is_none")]
    pub coll_tp: Option<CollateralType1Choice>,
    #[serde(rename = "CtrctTermsModChngd", skip_serializing_if = "Option::is_none")]
    pub ctrct_terms_mod_chngd: Option<YesNoIndicator>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<Rate2>,
    #[serde(rename = "BrrwgRate", skip_serializing_if = "Option::is_none")]
    pub brrwg_rate: Option<Rate2>,
    #[serde(rename = "StdCollRatio", skip_serializing_if = "Option::is_none")]
    pub std_coll_ratio: Option<Rate2>,
    #[serde(rename = "DvddRatio", skip_serializing_if = "Option::is_none")]
    pub dvdd_ratio: Option<Rate2>,
    #[serde(rename = "NbOfDaysLndgBrrwg", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_lndg_brrwg: Option<Number21Choice>,
    #[serde(rename = "StdCollAmt", skip_serializing_if = "Option::is_none")]
    pub std_coll_amt: Option<AmountAndDirection5>,
    #[serde(rename = "AcrdIntrstTax", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_tax: Option<YesNoIndicator>,
    #[serde(rename = "EndNbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub end_nb_of_days_acrd: Option<Max3Number>,
    #[serde(rename = "EndFctr", skip_serializing_if = "Option::is_none")]
    pub end_fctr: Option<BaseOneRate>,
    #[serde(rename = "SctiesLndgTp", skip_serializing_if = "Option::is_none")]
    pub scties_lndg_tp: Option<SecuritiesLendingType1Choice>,
    #[serde(rename = "Rvsbl", skip_serializing_if = "Option::is_none")]
    pub rvsbl: Option<Reversible1Choice>,
    #[serde(rename = "MinDtForCallBck", skip_serializing_if = "Option::is_none")]
    pub min_dt_for_call_bck: Option<IsoDate>,
    #[serde(rename = "RollOver", skip_serializing_if = "Option::is_none")]
    pub roll_over: Option<YesNoIndicator>,
    #[serde(rename = "PrdcPmt", skip_serializing_if = "Option::is_none")]
    pub prdc_pmt: Option<YesNoIndicator>,
    #[serde(rename = "ExCpn", skip_serializing_if = "Option::is_none")]
    pub ex_cpn: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationParties2 {
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Brrwr", skip_serializing_if = "Option::is_none")]
    pub brrwr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "Lndr", skip_serializing_if = "Option::is_none")]
    pub lndr: Option<ConfirmationPartyDetails2>,
    #[serde(rename = "BrkrOfCdt", skip_serializing_if = "Option::is_none")]
    pub brkr_of_cdt: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "IntrdcgFirm", skip_serializing_if = "Option::is_none")]
    pub intrdcg_firm: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "StepInFirm", skip_serializing_if = "Option::is_none")]
    pub step_in_firm: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "StepOutFirm", skip_serializing_if = "Option::is_none")]
    pub step_out_firm: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "ClrFirm", skip_serializing_if = "Option::is_none")]
    pub clr_firm: Option<ConfirmationPartyDetails5>,
    #[serde(rename = "ExctgBrkr", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr: Option<ConfirmationPartyDetails5>,
    #[serde(rename = "CMUPty", skip_serializing_if = "Option::is_none")]
    pub cmu_pty: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "CMUCtrPty", skip_serializing_if = "Option::is_none")]
    pub cmu_ctr_pty: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "AffrmgPty", skip_serializing_if = "Option::is_none")]
    pub affrmg_pty: Option<ConfirmationPartyDetails1>,
    #[serde(rename = "TradBnfcryPty", skip_serializing_if = "Option::is_none")]
    pub trad_bnfcry_pty: Option<ConfirmationPartyDetails3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification54 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: AnyBicIdentifier,
    #[validate]
    #[serde(rename = "PrtryId")]
    pub prtry_id: GenericIdentification29,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties23 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification55>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount34>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternatePartyIdentification6 {
    #[serde(rename = "TpOfId")]
    pub tp_of_id: IdentificationType41Choice,
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
pub struct Revaluation2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct Revaluation2Choice {
    #[serde(flatten)]
    pub value: Revaluation2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementSystemMethod3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementSystemMethod1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementSystemMethod3Choice {
    #[serde(flatten)]
    pub value: SettlementSystemMethod3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingDateCode1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradingDate1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingDateCode1Choice {
    #[serde(flatten)]
    pub value: TradingDateCode1ChoiceEnum,
}
#[derive(
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
pub enum CommissionType9Code {
    #[serde(rename = "CLDI")]
    Cldi,
    #[serde(rename = "STEP")]
    Step,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "PERN")]
    Pern,
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PERU")]
    Peru,
    #[serde(rename = "PWCD")]
    Pwcd,
    #[serde(rename = "PWEU")]
    Pweu,
    #[serde(rename = "BRKR")]
    Brkr,
    #[serde(rename = "DFDP")]
    Dfdp,
    #[serde(rename = "PBOC")]
    Pboc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryType2Code {
    #[serde(rename = "APMT")]
    Apmt,
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "TRIP")]
    Trip,
    #[serde(rename = "HOIC")]
    Hoic,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification38 {
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
pub struct SettlementTransactionCondition11ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition11Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition11ChoiceEnum,
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
pub struct BlockTrade3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BlockTrade1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockTrade3Choice {
    #[serde(flatten)]
    pub value: BlockTrade3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CentralCounterPartyEligibility3ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CentralCounterPartyEligibility3Choice {
    #[serde(flatten)]
    pub value: CentralCounterPartyEligibility3ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentificationAndAccount34 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityBreakdown11 {
    #[serde(rename = "LotNb", skip_serializing_if = "Option::is_none")]
    pub lot_nb: Option<GenericIdentification37>,
    #[serde(rename = "LotQty", skip_serializing_if = "Option::is_none")]
    pub lot_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "LotDtTm", skip_serializing_if = "Option::is_none")]
    pub lot_dt_tm: Option<DateAndDateTime1Choice>,
    #[serde(rename = "LotPric", skip_serializing_if = "Option::is_none")]
    pub lot_pric: Option<Price4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalTradeTransactionCondition1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition4Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingDate1Code {
    #[serde(rename = "VARI")]
    Vari,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice3Code {
    #[serde(rename = "AVER")]
    Aver,
    #[serde(rename = "AVOV")]
    Avov,
    #[serde(rename = "GREX")]
    Grex,
    #[serde(rename = "NET2")]
    Net2,
    #[serde(rename = "NET1")]
    Net1,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "RDAV")]
    Rdav,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInstructionGeneration1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementInstructionGeneration1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInstructionGeneration1Choice {
    #[serde(flatten)]
    pub value: SettlementInstructionGeneration1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriorityNumeric3ChoiceEnum {
    #[serde(rename = "Nmrc", skip_serializing_if = "Option::is_none")]
    pub nmrc: Option<Exact4NumericText>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriorityNumeric3Choice {
    #[serde(flatten)]
    pub value: PriorityNumeric3ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PositionEffect2Code {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "ROLL")]
    Roll,
    #[serde(rename = "FIFO")]
    Fifo,
    #[serde(rename = "CLOA")]
    Cloa,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmounts16 {
    #[serde(rename = "ChrgsFees", skip_serializing_if = "Option::is_none")]
    pub chrgs_fees: Option<AmountAndDirection29>,
    #[serde(rename = "CtryNtlFdrlTax", skip_serializing_if = "Option::is_none")]
    pub ctry_ntl_fdrl_tax: Option<AmountAndDirection29>,
    #[serde(rename = "ExctgBrkrAmt", skip_serializing_if = "Option::is_none")]
    pub exctg_brkr_amt: Option<AmountAndDirection29>,
    #[serde(rename = "IsseDscntAllwnc", skip_serializing_if = "Option::is_none")]
    pub isse_dscnt_allwnc: Option<AmountAndDirection29>,
    #[serde(rename = "PmtLevyTax", skip_serializing_if = "Option::is_none")]
    pub pmt_levy_tax: Option<AmountAndDirection29>,
    #[serde(rename = "LclTax", skip_serializing_if = "Option::is_none")]
    pub lcl_tax: Option<AmountAndDirection29>,
    #[serde(rename = "LclBrkrComssn", skip_serializing_if = "Option::is_none")]
    pub lcl_brkr_comssn: Option<AmountAndDirection29>,
    #[serde(rename = "Mrgn", skip_serializing_if = "Option::is_none")]
    pub mrgn: Option<AmountAndDirection29>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AmountAndDirection29>,
    #[serde(rename = "RgltryAmt", skip_serializing_if = "Option::is_none")]
    pub rgltry_amt: Option<AmountAndDirection29>,
    #[serde(rename = "SpclCncssn", skip_serializing_if = "Option::is_none")]
    pub spcl_cncssn: Option<AmountAndDirection29>,
    #[serde(rename = "StmpDty", skip_serializing_if = "Option::is_none")]
    pub stmp_dty: Option<AmountAndDirection29>,
    #[serde(rename = "StockXchgTax", skip_serializing_if = "Option::is_none")]
    pub stock_xchg_tax: Option<AmountAndDirection29>,
    #[serde(rename = "TrfTax", skip_serializing_if = "Option::is_none")]
    pub trf_tax: Option<AmountAndDirection29>,
    #[serde(rename = "TxTax", skip_serializing_if = "Option::is_none")]
    pub tx_tax: Option<AmountAndDirection29>,
    #[serde(rename = "ValAddedTax", skip_serializing_if = "Option::is_none")]
    pub val_added_tax: Option<AmountAndDirection29>,
    #[serde(rename = "WhldgTax", skip_serializing_if = "Option::is_none")]
    pub whldg_tax: Option<AmountAndDirection29>,
    #[serde(rename = "NetGnLoss", skip_serializing_if = "Option::is_none")]
    pub net_gn_loss: Option<AmountAndDirection29>,
    #[serde(rename = "CsmptnTax", skip_serializing_if = "Option::is_none")]
    pub csmptn_tax: Option<AmountAndDirection29>,
    #[serde(rename = "MtchgConfFee", skip_serializing_if = "Option::is_none")]
    pub mtchg_conf_fee: Option<AmountAndDirection29>,
    #[serde(rename = "ConvtdAmt", skip_serializing_if = "Option::is_none")]
    pub convtd_amt: Option<AmountAndDirection29>,
    #[serde(rename = "OrgnlCcyAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_ccy_amt: Option<AmountAndDirection29>,
    #[serde(rename = "BookVal", skip_serializing_if = "Option::is_none")]
    pub book_val: Option<AmountAndDirection29>,
    #[serde(rename = "AcrdCptlstnAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_cptlstn_amt: Option<AmountAndDirection29>,
    #[serde(rename = "LclTaxCtrySpcfc1", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc_1: Option<AmountAndDirection29>,
    #[serde(rename = "LclTaxCtrySpcfc2", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc_2: Option<AmountAndDirection29>,
    #[serde(rename = "LclTaxCtrySpcfc3", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc_3: Option<AmountAndDirection29>,
    #[serde(rename = "LclTaxCtrySpcfc4", skip_serializing_if = "Option::is_none")]
    pub lcl_tax_ctry_spcfc_4: Option<AmountAndDirection29>,
    #[serde(rename = "ShrdBrkrgAmt", skip_serializing_if = "Option::is_none")]
    pub shrd_brkrg_amt: Option<AmountAndDirection29>,
    #[serde(rename = "MktMmbFeeAmt", skip_serializing_if = "Option::is_none")]
    pub mkt_mmb_fee_amt: Option<AmountAndDirection29>,
    #[serde(rename = "RmnrtnAmtReq", skip_serializing_if = "Option::is_none")]
    pub rmnrtn_amt_req: Option<YesNoIndicator>,
    #[serde(rename = "RmnrtnAmt", skip_serializing_if = "Option::is_none")]
    pub rmnrtn_amt: Option<AmountAndDirection29>,
    #[serde(rename = "BrrwgIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub brrwg_intrst_amt: Option<AmountAndDirection29>,
    #[serde(rename = "BrrwgFee", skip_serializing_if = "Option::is_none")]
    pub brrwg_fee: Option<AmountAndDirection29>,
    #[serde(rename = "NetMktVal", skip_serializing_if = "Option::is_none")]
    pub net_mkt_val: Option<AmountAndDirection29>,
    #[serde(rename = "RmngFaceVal", skip_serializing_if = "Option::is_none")]
    pub rmng_face_val: Option<AmountAndDirection29>,
    #[serde(rename = "RmngBookVal", skip_serializing_if = "Option::is_none")]
    pub rmng_book_val: Option<AmountAndDirection29>,
    #[serde(rename = "ClrBrkrComssn", skip_serializing_if = "Option::is_none")]
    pub clr_brkr_comssn: Option<AmountAndDirection29>,
    #[serde(rename = "DiffInPric", skip_serializing_if = "Option::is_none")]
    pub diff_in_pric: Option<AmountAndDirection29>,
    #[serde(rename = "OddLotFee", skip_serializing_if = "Option::is_none")]
    pub odd_lot_fee: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency7Choice {
    #[serde(flatten)]
    pub value: Frequency7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount83 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification70Choice,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
    pub csh_acct: Option<CashAccountIdentification2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Commission16 {
    #[serde(rename = "Tp")]
    pub tp: CommissionType2Choice,
    #[serde(rename = "Comssn")]
    pub comssn: AmountOrRate2Choice,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification54>,
    #[serde(rename = "ClctnDt", skip_serializing_if = "Option::is_none")]
    pub clctn_dt: Option<IsoDate>,
    #[serde(rename = "TtlComssn", skip_serializing_if = "Option::is_none")]
    pub ttl_comssn: Option<AmountAndDirection29>,
    #[serde(rename = "TtlVATAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_vat_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "VATRate", skip_serializing_if = "Option::is_none")]
    pub vat_rate: Option<BaseOneRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorCapacity3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Eligibility1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorCapacity3Choice {
    #[serde(flatten)]
    pub value: InvestorCapacity3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BusinessProcessType1Code {
    #[serde(rename = "ISUP")]
    Isup,
    #[serde(rename = "NISP")]
    Nisp,
    #[serde(rename = "PRAC")]
    Prac,
    #[serde(rename = "RSAL")]
    Rsal,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "THRU")]
    Thru,
    #[serde(rename = "IDEL")]
    Idel,
    #[serde(rename = "DPLX")]
    Dplx,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherParties18 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Invstr", default)]
    pub invstr: Vec<PartyIdentificationAndAccount79>,
    #[serde(rename = "StockXchg", skip_serializing_if = "Option::is_none")]
    pub stock_xchg: Option<PartyIdentificationAndAccount87>,
    #[serde(rename = "TradRgltr", skip_serializing_if = "Option::is_none")]
    pub trad_rgltr: Option<PartyIdentificationAndAccount87>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<PartyIdentificationAndAccount83>,
    #[serde(rename = "QlfdFrgnIntrmy", skip_serializing_if = "Option::is_none")]
    pub qlfd_frgn_intrmy: Option<PartyIdentificationAndAccount77>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionRight1Code {
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "ASGN")]
    Asgn,
    #[serde(rename = "RENO")]
    Reno,
    #[serde(rename = "EXPI")]
    Expi,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StandingSettlementInstruction9 {
    #[serde(rename = "SttlmStgInstrDB")]
    pub sttlm_stg_instr_db: SettlementStandingInstructionDatabase3Choice,
    #[serde(rename = "Vndr", skip_serializing_if = "Option::is_none")]
    pub vndr: Option<PartyIdentification32Choice>,
    #[serde(
        rename = "OthrDlvrgSttlmPties",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_dlvrg_sttlm_pties: Option<SettlementParties23>,
    #[serde(rename = "OthrRcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub othr_rcvg_sttlm_pties: Option<SettlementParties23>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price4 {
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmountChoice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PriceValueType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber4ChoiceEnum {
    #[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
    pub lng_nb: Option<Iso20022MessageIdentificationText>,
    #[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
    pub shrt_nb: Option<Exact3NumericText>,
    #[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
    pub prtry_nb: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentNumber4Choice {
    #[serde(flatten)]
    pub value: DocumentNumber4ChoiceEnum,
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
pub struct NumberCount1ChoiceEnum {
    #[serde(rename = "CurInstrNb", skip_serializing_if = "Option::is_none")]
    pub cur_instr_nb: Option<Exact3NumericText>,
    #[serde(rename = "TtlNb", skip_serializing_if = "Option::is_none")]
    pub ttl_nb: Option<TotalNumber1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberCount1Choice {
    #[serde(flatten)]
    pub value: NumberCount1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Reversible1Code {
    #[serde(rename = "REVL")]
    Revl,
    #[serde(rename = "FIXD")]
    Fixd,
    #[serde(rename = "CABK")]
    Cabk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LetterOfGuarantee3ChoiceEnum {
    #[serde(rename = "Ind", skip_serializing_if = "Option::is_none")]
    pub ind: Option<YesNoIndicator>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LetterOfGuarantee3Choice {
    #[serde(flatten)]
    pub value: LetterOfGuarantee3ChoiceEnum,
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
pub struct PartyTextInformation1 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration6ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Registration1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Registration6Choice {
    #[serde(flatten)]
    pub value: Registration6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TwoLegTransactionType1ChoiceEnum {
    #[serde(rename = "FutrOrOptnDtls", skip_serializing_if = "Option::is_none")]
    pub futr_or_optn_dtls: Option<FutureOrOptionDetails1>,
    #[serde(rename = "SctiesFincgDtls", skip_serializing_if = "Option::is_none")]
    pub scties_fincg_dtls: Option<SecuritiesFinancing10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TwoLegTransactionType1Choice {
    #[serde(flatten)]
    pub value: TwoLegTransactionType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConfirmationPartyDetails5 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification32Choice,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification5>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation5>,
    #[serde(
        rename = "InvstrPrtcnAssoctnMmbsh",
        skip_serializing_if = "Option::is_none"
    )]
    pub invstr_prtcn_assoctn_mmbsh: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation2 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate4ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<TradingDateCode1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDate4Choice {
    #[serde(flatten)]
    pub value: TradeDate4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CfiIdentifier {
    #[validate(regex = "CFI_IDENTIFIER_REGEX")]
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
pub struct OptionRight1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionRight1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionRight1Choice {
    #[serde(flatten)]
    pub value: OptionRight1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure1Code {
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "ACRE")]
    Acre,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FxStandingInstruction3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct FxStandingInstruction3Choice {
    #[serde(flatten)]
    pub value: FxStandingInstruction3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantityChoiceEnum {
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
pub struct FinancialInstrumentQuantityChoice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantityChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType19ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType19Choice {
    #[serde(flatten)]
    pub value: RateType19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegulatoryStipulations1 {
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate(length(min = 1,))]
    #[serde(rename = "Stiptns", default)]
    pub stiptns: Vec<Max350Text>,
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
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeTaxBasis1Code {
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
pub struct MarketType12ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
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
pub struct MarketType12Choice {
    #[serde(flatten)]
    pub value: MarketType12ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClosingDate1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Date2Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTimeChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClosingDate1Choice {
    #[serde(flatten)]
    pub value: ClosingDate1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Rating1 {
    #[validate]
    #[serde(rename = "RatgSchme")]
    pub ratg_schme: Max35Text,
    #[validate]
    #[serde(rename = "ValDt")]
    pub val_dt: IsoDateTime,
    #[validate]
    #[serde(rename = "ValId")]
    pub val_id: RatingValueIdentifier,
}
#[derive(
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
pub struct SecuritiesLendingType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesLendingType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesLendingType1Choice {
    #[serde(flatten)]
    pub value: SecuritiesLendingType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MarketType6Code {
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
pub struct Exact4NumericText {
    #[validate(regex = "EXACT_4_NUMERIC_TEXT_REGEX")]
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
pub struct SecuritiesAccount3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PurposeCode5Choice>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
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
pub struct AlternatePartyIdentification5 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType40Choice,
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
pub struct Max4NumericText {
    #[validate(regex = "MAX_4_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralType3Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "STCF")]
    Stcf,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "GBBK")]
    Gbbk,
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
pub struct UpicIdentifier {
    #[validate(regex = "UPIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeType3Code {
    #[serde(rename = "BSKT")]
    Bskt,
    #[serde(rename = "INDX")]
    Indx,
    #[serde(rename = "IPOO")]
    Ipoo,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "PRAL")]
    Pral,
    #[serde(rename = "PROG")]
    Prog,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "BRBR")]
    Brbr,
    #[serde(rename = "RISK")]
    Risk,
    #[serde(rename = "VWAP")]
    Vwap,
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "GUAR")]
    Guar,
    #[serde(rename = "EMTR")]
    Emtr,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "BOST")]
    Bost,
    #[serde(rename = "BOEN")]
    Boen,
    #[serde(rename = "LABO")]
    Labo,
    #[serde(rename = "BORE")]
    Bore,
    #[serde(rename = "OFIT")]
    Ofit,
    #[serde(rename = "BOSU")]
    Bosu,
    #[serde(rename = "FBBT")]
    Fbbt,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "FUOP")]
    Fuop,
    #[serde(rename = "FUTR")]
    Futr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Agreement3 {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDateTime>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<CurrencyCode>,
    #[serde(rename = "ClsgTp", skip_serializing_if = "Option::is_none")]
    pub clsg_tp: Option<ClosingType1Code>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDateTime>,
    #[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
    pub dlvry_tp: Option<DeliveryType2Code>,
    #[serde(rename = "MrgnRatio", skip_serializing_if = "Option::is_none")]
    pub mrgn_ratio: Option<PercentageRate>,
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
pub struct ClassificationType30ChoiceEnum {
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiIdentifier>,
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
pub struct ClassificationType30Choice {
    #[serde(flatten)]
    pub value: ClassificationType30ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress13 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress8>,
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
pub struct SecuritiesTradeConfirmationV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Id")]
    pub id: TransactiontIdentification4,
    #[serde(rename = "NbCnt", skip_serializing_if = "Option::is_none")]
    pub nb_cnt: Option<NumberCount1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Refs", default)]
    pub refs: Vec<Linkages15>,
    #[validate]
    #[serde(rename = "TradDtls")]
    pub trad_dtls: Order17,
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification14,
    #[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_attrbts: Option<FinancialInstrumentAttributes44>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygFinInstrm", default)]
    pub undrlyg_fin_instrm: Vec<UnderlyingFinancialInstrument2>,
    #[serde(rename = "Stiptns", skip_serializing_if = "Option::is_none")]
    pub stiptns: Option<FinancialInstrumentStipulations2>,
    #[validate(length(min = 1,))]
    #[serde(rename = "ConfPties", default)]
    pub conf_pties: Vec<ConfirmationParties2>,
    #[serde(rename = "SttlmParams", skip_serializing_if = "Option::is_none")]
    pub sttlm_params: Option<SettlementDetails43>,
    #[serde(rename = "StgSttlmInstr", skip_serializing_if = "Option::is_none")]
    pub stg_sttlm_instr: Option<StandingSettlementInstruction9>,
    #[serde(rename = "DlvrgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sttlm_pties: Option<SettlementParties23>,
    #[serde(rename = "RcvgSttlmPties", skip_serializing_if = "Option::is_none")]
    pub rcvg_sttlm_pties: Option<SettlementParties23>,
    #[serde(rename = "CshPties", skip_serializing_if = "Option::is_none")]
    pub csh_pties: Option<CashParties18>,
    #[serde(rename = "ClrDtls", skip_serializing_if = "Option::is_none")]
    pub clr_dtls: Option<Clearing3>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<AmountAndDirection28>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmts", default)]
    pub othr_amts: Vec<OtherAmounts16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrPrics", default)]
    pub othr_prics: Vec<OtherPrices2>,
    #[serde(rename = "OthrBizPties", skip_serializing_if = "Option::is_none")]
    pub othr_biz_pties: Option<OtherParties18>,
    #[serde(rename = "TwoLegTxDtls", skip_serializing_if = "Option::is_none")]
    pub two_leg_tx_dtls: Option<TwoLegTransactionDetails1>,
    #[serde(rename = "RgltryStiptns", skip_serializing_if = "Option::is_none")]
    pub rgltry_stiptns: Option<RegulatoryStipulations1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesLendingType1Code {
    #[serde(rename = "NWRG")]
    Nwrg,
    #[serde(rename = "RENW")]
    Renw,
    #[serde(rename = "CABK")]
    Cabk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlingCapacity1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlingCapacity3Choice {
    #[serde(flatten)]
    pub value: SettlingCapacity3ChoiceEnum,
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
pub struct UnderlyingFinancialInstrument2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: SecurityIdentification14,
    #[serde(rename = "Attrbts", skip_serializing_if = "Option::is_none")]
    pub attrbts: Option<FinancialInstrumentAttributes44>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessProcessType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification38>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BusinessProcessType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessProcessType1Choice {
    #[serde(flatten)]
    pub value: BusinessProcessType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateName1 {
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max8Text>,
    #[validate]
    #[serde(rename = "RateNm")]
    pub rate_nm: Max35Text,
}
