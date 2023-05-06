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
    static ref MAX_6_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{1,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_2_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RATE_SOURCE_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{3}[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.014.001.04".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralInformation5 {
    #[serde(rename = "BlckInd", skip_serializing_if = "Option::is_none")]
    pub blck_ind: Option<YesNoIndicator>,
    #[serde(rename = "RltdTradRef", skip_serializing_if = "Option::is_none")]
    pub rltd_trad_ref: Option<Max35Text>,
    #[serde(rename = "DealgMtd", skip_serializing_if = "Option::is_none")]
    pub dealg_mtd: Option<Trading1MethodCode>,
    #[serde(rename = "BrkrId", skip_serializing_if = "Option::is_none")]
    pub brkr_id: Option<PartyIdentification73Choice>,
    #[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_ref: Option<Max35Text>,
    #[serde(rename = "BrkrsComssn", skip_serializing_if = "Option::is_none")]
    pub brkrs_comssn: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SndrToRcvrInf", skip_serializing_if = "Option::is_none")]
    pub sndr_to_rcvr_inf: Option<Max210Text>,
    #[serde(rename = "DealgBrnchTradgSd", skip_serializing_if = "Option::is_none")]
    pub dealg_brnch_tradg_sd: Option<PartyIdentification73Choice>,
    #[serde(rename = "DealgBrnchCtrPtySd", skip_serializing_if = "Option::is_none")]
    pub dealg_brnch_ctr_pty_sd: Option<PartyIdentification73Choice>,
    #[serde(rename = "CtctInf", skip_serializing_if = "Option::is_none")]
    pub ctct_inf: Option<ContactInformation1>,
    #[serde(rename = "AgrmtDtls", skip_serializing_if = "Option::is_none")]
    pub agrmt_dtls: Option<AgreementConditions1>,
    #[serde(rename = "DefsYr", skip_serializing_if = "Option::is_none")]
    pub defs_yr: Option<IsoYear>,
    #[serde(rename = "BrkrsRef", skip_serializing_if = "Option::is_none")]
    pub brkrs_ref: Option<Max35Text>,
}
#[derive(
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
pub struct TradePartyIdentification6 {
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: PartyIdentification73Choice,
    #[serde(rename = "TradPty", skip_serializing_if = "Option::is_none")]
    pub trad_pty: Option<PartyIdentification73Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FndId", default)]
    pub fnd_id: Vec<FundIdentification4>,
}
#[derive(
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
pub struct Max6AlphaText {
    #[validate(regex = "MAX_6_ALPHA_TEXT_REGEX")]
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
pub struct PartyIdentification59 {
    #[serde(rename = "PtyNm", skip_serializing_if = "Option::is_none")]
    pub pty_nm: Option<Max34Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
    #[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
    pub acct_nb: Option<Max34Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Max105Text>,
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
}
#[derive(
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
pub enum SideIndicator1Code {
    #[serde(rename = "CCPL")]
    Ccpl,
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
pub struct TradingSideTransactionReporting1 {
    #[serde(rename = "RptgJursdctn", skip_serializing_if = "Option::is_none")]
    pub rptg_jursdctn: Option<Max35Text>,
    #[serde(rename = "RptgPty", skip_serializing_if = "Option::is_none")]
    pub rptg_pty: Option<PartyIdentification73Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradgSdUnqTxIdr", default)]
    pub tradg_sd_unq_tx_idr: Vec<UniqueTransactionIdentifier2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonDeliverableForwardConditions1 {
    #[validate]
    #[serde(rename = "OpngInd")]
    pub opng_ind: YesNoIndicator,
    #[serde(rename = "OpngFxgConds")]
    pub opng_fxg_conds: NdfOpeningFixing1Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralisationIndicator1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "ONEW")]
    Onew,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "UNCO")]
    Unco,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max105Text {
    #[validate(length(min = 1, max = 105,))]
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
pub struct RegulatoryReporting6 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TradgSdTxRptg", default)]
    pub tradg_sd_tx_rptg: Vec<TradingSideTransactionReporting1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrPtySdTxRptg", default)]
    pub ctr_pty_sd_tx_rptg: Vec<CounterpartySideTransactionReporting1>,
    #[serde(rename = "CntrlCtrPtyClrHs", skip_serializing_if = "Option::is_none")]
    pub cntrl_ctr_pty_clr_hs: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrBrkr", skip_serializing_if = "Option::is_none")]
    pub clr_brkr: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrXcptnPty", skip_serializing_if = "Option::is_none")]
    pub clr_xcptn_pty: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrBrkrId", skip_serializing_if = "Option::is_none")]
    pub clr_brkr_id: Option<ClearingBrokerIdentification1>,
    #[serde(rename = "ClrThrshldInd", skip_serializing_if = "Option::is_none")]
    pub clr_thrshld_ind: Option<YesNoIndicator>,
    #[serde(rename = "ClrdPdctId", skip_serializing_if = "Option::is_none")]
    pub clrd_pdct_id: Option<Max35Text>,
    #[serde(rename = "UndrlygPdctIdr", skip_serializing_if = "Option::is_none")]
    pub undrlyg_pdct_idr: Option<UnderlyingProductIdentifier1Code>,
    #[serde(rename = "AllcnInd", skip_serializing_if = "Option::is_none")]
    pub allcn_ind: Option<AllocationIndicator1Code>,
    #[serde(rename = "CollstnInd", skip_serializing_if = "Option::is_none")]
    pub collstn_ind: Option<CollateralisationIndicator1Code>,
    #[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
    pub exctn_vn: Option<Max35Text>,
    #[serde(rename = "ExctnTmstmp", skip_serializing_if = "Option::is_none")]
    pub exctn_tmstmp: Option<DateAndDateTimeChoice>,
    #[serde(rename = "NonStdFlg", skip_serializing_if = "Option::is_none")]
    pub non_std_flg: Option<YesNoIndicator>,
    #[serde(rename = "LkSwpId", skip_serializing_if = "Option::is_none")]
    pub lk_swp_id: Option<Exact42Text>,
    #[serde(
        rename = "FinNtrOfTheCtrPtyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub fin_ntr_of_the_ctr_pty_ind: Option<YesNoIndicator>,
    #[serde(rename = "CollPrtflInd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_ind: Option<YesNoIndicator>,
    #[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_cd: Option<Max10Text>,
    #[serde(rename = "PrtflCmprssnInd", skip_serializing_if = "Option::is_none")]
    pub prtfl_cmprssn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CorpSctrInd", skip_serializing_if = "Option::is_none")]
    pub corp_sctr_ind: Option<CorporateSectorIdentifier1Code>,
    #[serde(
        rename = "TradWthNonEEACtrPtyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub trad_wth_non_eea_ctr_pty_ind: Option<YesNoIndicator>,
    #[serde(rename = "NtrgrpTradInd", skip_serializing_if = "Option::is_none")]
    pub ntrgrp_trad_ind: Option<YesNoIndicator>,
    #[serde(
        rename = "ComrclOrTrsrFincgInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub comrcl_or_trsr_fincg_ind: Option<YesNoIndicator>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification19>,
    #[serde(rename = "ConfDtAndTmstmp", skip_serializing_if = "Option::is_none")]
    pub conf_dt_and_tmstmp: Option<IsoDateTime>,
    #[serde(rename = "ClrTmstmp", skip_serializing_if = "Option::is_none")]
    pub clr_tmstmp: Option<IsoTime>,
    #[serde(rename = "AddtlRptgInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rptg_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Trading1MethodCode {
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "BROK")]
    Brok,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartySideTransactionReporting1 {
    #[serde(rename = "RptgJursdctn", skip_serializing_if = "Option::is_none")]
    pub rptg_jursdctn: Option<Max35Text>,
    #[serde(rename = "RptgPty", skip_serializing_if = "Option::is_none")]
    pub rptg_pty: Option<PartyIdentification73Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrPtySdUnqTxIdr", default)]
    pub ctr_pty_sd_unq_tx_idr: Vec<UniqueTransactionIdentifier2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementParties29 {
    #[serde(rename = "DlvryAgt", skip_serializing_if = "Option::is_none")]
    pub dlvry_agt: Option<PartyIdentification73Choice>,
    #[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
    pub intrmy: Option<PartyIdentification73Choice>,
    #[serde(rename = "RcvgAgt")]
    pub rcvg_agt: PartyIdentification73Choice,
    #[serde(rename = "BnfcryInstn", skip_serializing_if = "Option::is_none")]
    pub bnfcry_instn: Option<PartyIdentification73Choice>,
}
#[derive(
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
    #[serde(rename = "FXTradInstr")]
    pub fx_trad_instr: ForeignExchangeTradeInstructionV04<A>,
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
pub struct AgreementConditions1 {
    #[validate]
    #[serde(rename = "AgrmtCd")]
    pub agrmt_cd: Max6AlphaText,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Exact4NumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OpeningConditions1 {
    #[serde(rename = "SttlmCcy")]
    pub sttlm_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "ValtnDt")]
    pub valtn_dt: IsoDate,
    #[validate(length(min = 1, max = 2,))]
    #[serde(rename = "SttlmRateSrc", default)]
    pub sttlm_rate_src: Vec<SettlementRateSource1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact2AlphaNumericText {
    #[validate(regex = "EXACT_2_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct ForeignExchangeTradeInstructionV04<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "TradInf")]
    pub trad_inf: TradeAgreement14,
    #[validate]
    #[serde(rename = "TradgSdId")]
    pub tradg_sd_id: TradePartyIdentification6,
    #[validate]
    #[serde(rename = "CtrPtySdId")]
    pub ctr_pty_sd_id: TradePartyIdentification6,
    #[validate]
    #[serde(rename = "TradAmts")]
    pub trad_amts: AmountsAndValueDate1,
    #[validate]
    #[serde(rename = "AgrdRate")]
    pub agrd_rate: AgreedRate3,
    #[serde(rename = "NDFConds", skip_serializing_if = "Option::is_none")]
    pub ndf_conds: Option<NonDeliverableForwardConditions1>,
    #[serde(rename = "TradgSdSttlmInstrs", skip_serializing_if = "Option::is_none")]
    pub tradg_sd_sttlm_instrs: Option<SettlementParties29>,
    #[serde(
        rename = "CtrPtySdSttlmInstrs",
        skip_serializing_if = "Option::is_none"
    )]
    pub ctr_pty_sd_sttlm_instrs: Option<SettlementParties29>,
    #[serde(rename = "OptnlGnlInf", skip_serializing_if = "Option::is_none")]
    pub optnl_gnl_inf: Option<GeneralInformation5>,
    #[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
    pub rgltry_rptg: Option<RegulatoryReporting6>,
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
pub struct FundIdentification4 {
    #[validate]
    #[serde(rename = "FndId")]
    pub fnd_id: PartyIdentification60,
    #[serde(rename = "AcctIdWthCtdn", skip_serializing_if = "Option::is_none")]
    pub acct_id_wth_ctdn: Option<Max35Text>,
    #[serde(rename = "CtdnId", skip_serializing_if = "Option::is_none")]
    pub ctdn_id: Option<PartyIdentification73Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoYear {
    #[validate(regex = "ISO_YEAR_REGEX")]
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
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
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
pub struct PartyIdentification44 {
    #[validate]
    #[serde(rename = "AnyBIC")]
    pub any_bic: AnyBicIdentifier,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(
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
pub enum UnderlyingProductIdentifier1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "NDFO")]
    Ndfo,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Max210Text {
    #[validate(length(min = 1, max = 210,))]
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
pub struct AgreedRate3 {
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "QtdCcy", skip_serializing_if = "Option::is_none")]
    pub qtd_ccy: Option<ActiveCurrencyCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateSectorIdentifier1Code {
    #[serde(rename = "L")]
    L,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "O")]
    O,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "U")]
    U,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ClearingSystemIdentification2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalClearingSystemIdentification1Code>,
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
pub struct AmountsAndValueDate1 {
    #[validate]
    #[serde(rename = "TradgSdBuyAmt")]
    pub tradg_sd_buy_amt: ActiveOrHistoricCurrencyAndAmount,
    #[validate]
    #[serde(rename = "TradgSdSellAmt")]
    pub tradg_sd_sell_amt: ActiveOrHistoricCurrencyAndAmount,
    #[validate]
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AllocationIndicator1Code {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "UNAL")]
    Unal,
    #[default]
    Unknown,
}
#[derive(
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
pub struct NdfOpeningFixing1ChoiceEnum {
    #[serde(rename = "OpngConfRef", skip_serializing_if = "Option::is_none")]
    pub opng_conf_ref: Option<Max35Text>,
    #[serde(rename = "OpngConds", skip_serializing_if = "Option::is_none")]
    pub opng_conds: Option<OpeningConditions1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NdfOpeningFixing1Choice {
    #[serde(flatten)]
    pub value: NdfOpeningFixing1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification73ChoiceEnum {
    #[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
    pub pty_id: Option<PartyIdentification59>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress8>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification73Choice {
    #[serde(flatten)]
    pub value: PartyIdentification73ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementRateSource1 {
    #[validate]
    #[serde(rename = "RateSrc")]
    pub rate_src: RateSourceText,
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<Exact4NumericText>,
    #[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
    pub ctry_cd: Option<CountryCode>,
    #[serde(rename = "LctnCd", skip_serializing_if = "Option::is_none")]
    pub lctn_cd: Option<Exact2AlphaNumericText>,
}
#[derive(
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
pub struct RateSourceText {
    #[validate(regex = "RATE_SOURCE_TEXT_REGEX")]
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
pub struct PartyIdentification60 {
    #[validate]
    #[serde(rename = "FndId")]
    pub fnd_id: Max35Text,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress8>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
}
#[derive(
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
pub struct UniqueTransactionIdentifier2 {
    #[validate]
    #[serde(rename = "UnqTxIdr")]
    pub unq_tx_idr: Max52Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrrUnqTxIdr", default)]
    pub prr_unq_tx_idr: Vec<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress8 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(
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
pub struct ClearingBrokerIdentification1 {
    #[serde(rename = "SdInd")]
    pub sd_ind: SideIndicator1Code,
    #[validate]
    #[serde(rename = "ClrBrkrId")]
    pub clr_brkr_id: Max35Text,
}
#[derive(
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
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
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
pub struct TradeAgreement14 {
    #[validate]
    #[serde(rename = "TradDt")]
    pub trad_dt: IsoDate,
    #[validate]
    #[serde(rename = "OrgtrRef")]
    pub orgtr_ref: Max35Text,
    #[serde(rename = "CmonRef", skip_serializing_if = "Option::is_none")]
    pub cmon_ref: Option<Max35Text>,
    #[serde(rename = "OprTp", skip_serializing_if = "Option::is_none")]
    pub opr_tp: Option<Max4Text>,
    #[serde(rename = "OprScp", skip_serializing_if = "Option::is_none")]
    pub opr_scp: Option<Max4Text>,
    #[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
    pub pdct_tp: Option<Max35Text>,
    #[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
    pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
    #[serde(rename = "PmtVrssPmtInd", skip_serializing_if = "Option::is_none")]
    pub pmt_vrss_pmt_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact42Text {
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
pub struct ContactInformation1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "TelNb", skip_serializing_if = "Option::is_none")]
    pub tel_nb: Option<PhoneNumber>,
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
