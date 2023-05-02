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
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:reda.004.001.06".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExtendedParty13 {
    #[validate]
    #[serde(rename = "PtyRole")]
    pub pty_role: GenericIdentification36,
    #[validate]
    #[serde(rename = "OthrPtyDtls")]
    pub othr_pty_dtls: ContactAttributes5,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundReferenceDataReport4 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<MarketPracticeVersion1>,
    #[serde(rename = "AuthrsdPrxy", skip_serializing_if = "Option::is_none")]
    pub authrsd_prxy: Option<ContactAttributes6>,
    #[validate]
    #[serde(rename = "GnlRefDt")]
    pub gnl_ref_dt: IsoDate,
    #[serde(rename = "TrgtMktInd", skip_serializing_if = "Option::is_none")]
    pub trgt_mkt_ind: Option<YesNoIndicator>,
    #[serde(rename = "ExAnteInd", skip_serializing_if = "Option::is_none")]
    pub ex_ante_ind: Option<YesNoIndicator>,
    #[serde(rename = "ExPstInd", skip_serializing_if = "Option::is_none")]
    pub ex_pst_ind: Option<YesNoIndicator>,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification41,
    #[serde(rename = "FndPties", skip_serializing_if = "Option::is_none")]
    pub fnd_pties: Option<FundParties1>,
    #[serde(rename = "MainFndOrdrDsk", skip_serializing_if = "Option::is_none")]
    pub main_fnd_ordr_dsk: Option<OrderDesk1>,
    #[serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none")]
    pub fnd_mgmt_cpny: Option<ContactAttributes5>,
    #[serde(rename = "FndDtls", skip_serializing_if = "Option::is_none")]
    pub fnd_dtls: Option<FinancialInstrument96>,
    #[serde(rename = "ValtnDealgChrtcs", skip_serializing_if = "Option::is_none")]
    pub valtn_dealg_chrtcs: Option<ValuationDealingProcessingCharacteristics3>,
    #[serde(rename = "InvstmtRstrctns", skip_serializing_if = "Option::is_none")]
    pub invstmt_rstrctns: Option<InvestmentRestrictions3>,
    #[serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics8>,
    #[serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub red_prcg_chrtcs: Option<ProcessingCharacteristics7>,
    #[serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PlanChrtcs", default)]
    pub plan_chrtcs: Vec<InvestmentPlanCharacteristics1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtInstrm", default)]
    pub pmt_instrm: Vec<PaymentInstrument16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshSttlmDtls", default)]
    pub csh_sttlm_dtls: Vec<CashAccount205>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LclMktAnx", default)]
    pub lcl_mkt_anx: Vec<LocalMarketAnnex5>,
    #[serde(rename = "TrgtMkt", skip_serializing_if = "Option::is_none")]
    pub trgt_mkt: Option<TargetMarket3>,
    #[serde(rename = "DstrbtnStrtgy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_strtgy: Option<DistributionStrategy1>,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "CostsAndChrgs", default)]
    pub costs_and_chrgs: Vec<CostsAndCharges2>,
    #[serde(
        rename = "AddtlPdctInfGrmnMkt",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_pdct_inf_grmn_mkt: Option<AdditionalProductInformation1>,
    #[serde(
        rename = "AddtlPdctInfFrnchMkt",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_pdct_inf_frnch_mkt: Option<AdditionalProductInformation2>,
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
pub struct AccountIdentificationAndName7 {
    #[serde(rename = "Id")]
    pub id: CashAccountIdentification8Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotionalOrUnitBased1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NotionalOrUnitBased1Code>,
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
pub struct NotionalOrUnitBased1Choice {
    #[serde(flatten)]
    pub value: NotionalOrUnitBased1ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum HoldingTransferable1Code {
    #[serde(rename = "TRAL")]
    Tral,
    #[serde(rename = "TRNA")]
    Trna,
    #[serde(rename = "RFOD")]
    Rfod,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RiskTolerance1 {
    #[serde(
        rename = "RskTlrncePRIIPSMthdlgy",
        skip_serializing_if = "Option::is_none"
    )]
    pub rsk_tlrnce_priips_mthdlgy: Option<Max1Number>,
    #[serde(
        rename = "RskTlrnceUCITSMthdlgy",
        skip_serializing_if = "Option::is_none"
    )]
    pub rsk_tlrnce_ucits_mthdlgy: Option<Max1Number>,
    #[serde(rename = "RskTlrnceIntl", skip_serializing_if = "Option::is_none")]
    pub rsk_tlrnce_intl: Option<RiskLevel1Code>,
    #[serde(
        rename = "RskTlrnceForNonPRIIPSAndNonUCITSES",
        skip_serializing_if = "Option::is_none"
    )]
    pub rsk_tlrnce_for_non_priips_and_non_ucitses: Option<Max1Number>,
    #[serde(
        rename = "NotForInvstrsWthTheLwstRskTlrnceDE",
        skip_serializing_if = "Option::is_none"
    )]
    pub not_for_invstrs_wth_the_lwst_rsk_tlrnce_de: Option<TargetMarket2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherTargetMarketRiskTolerance1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContactAttributes6 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress1>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
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
pub struct FinancialInstrument96 {
    #[serde(rename = "PhysBrScties", skip_serializing_if = "Option::is_none")]
    pub phys_br_scties: Option<YesNoIndicator>,
    #[serde(rename = "DmtrlsdBrScties", skip_serializing_if = "Option::is_none")]
    pub dmtrlsd_br_scties: Option<YesNoIndicator>,
    #[serde(rename = "PhysRegdScties", skip_serializing_if = "Option::is_none")]
    pub phys_regd_scties: Option<YesNoIndicator>,
    #[serde(rename = "DmtrlsdRegdScties", skip_serializing_if = "Option::is_none")]
    pub dmtrlsd_regd_scties: Option<YesNoIndicator>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[serde(rename = "DvddPlcy", skip_serializing_if = "Option::is_none")]
    pub dvdd_plcy: Option<DividendPolicy1Code>,
    #[serde(rename = "DvddFrqcy", skip_serializing_if = "Option::is_none")]
    pub dvdd_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "RinvstmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub rinvstmt_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "FrntEndLd", skip_serializing_if = "Option::is_none")]
    pub frnt_end_ld: Option<YesNoIndicator>,
    #[serde(rename = "BckEndLd", skip_serializing_if = "Option::is_none")]
    pub bck_end_ld: Option<YesNoIndicator>,
    #[serde(rename = "SwtchFee", skip_serializing_if = "Option::is_none")]
    pub swtch_fee: Option<YesNoIndicator>,
    #[serde(rename = "EUSvgsDrctv", skip_serializing_if = "Option::is_none")]
    pub eu_svgs_drctv: Option<EuSavingsDirective1Code>,
    #[serde(rename = "LnchDt", skip_serializing_if = "Option::is_none")]
    pub lnch_dt: Option<IsoDate>,
    #[serde(rename = "FndEndDt", skip_serializing_if = "Option::is_none")]
    pub fnd_end_dt: Option<IsoDate>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDate>,
    #[serde(rename = "InitlOfferEndDt", skip_serializing_if = "Option::is_none")]
    pub initl_offer_end_dt: Option<IsoDate>,
    #[serde(rename = "SspnsnStartDt", skip_serializing_if = "Option::is_none")]
    pub sspnsn_start_dt: Option<IsoDate>,
    #[serde(rename = "SspnsnEndDt", skip_serializing_if = "Option::is_none")]
    pub sspnsn_end_dt: Option<IsoDate>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "MayBeTermntdEarly", skip_serializing_if = "Option::is_none")]
    pub may_be_termntd_early: Option<TargetMarket1Code>,
    #[serde(rename = "ClsdEndFnd", skip_serializing_if = "Option::is_none")]
    pub clsd_end_fnd: Option<YesNoIndicator>,
    #[serde(rename = "Equlstn", skip_serializing_if = "Option::is_none")]
    pub equlstn: Option<YesNoIndicator>,
    #[serde(rename = "TaxEffcntPdctElgbl", skip_serializing_if = "Option::is_none")]
    pub tax_effcnt_pdct_elgbl: Option<YesNoIndicator>,
    #[serde(rename = "Authrsd", skip_serializing_if = "Option::is_none")]
    pub authrsd: Option<YesNoIndicator>,
    #[serde(rename = "RDRCmplnt", skip_serializing_if = "Option::is_none")]
    pub rdr_cmplnt: Option<YesNoIndicator>,
    #[serde(rename = "MgmtFeeSrc", skip_serializing_if = "Option::is_none")]
    pub mgmt_fee_src: Option<AnnualChargePaymentType1Code>,
    #[serde(rename = "PrfrmncFee", skip_serializing_if = "Option::is_none")]
    pub prfrmnc_fee: Option<YesNoIndicator>,
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
pub struct ProcessingCharacteristics5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "DealgCcyAccptd", default)]
    pub dealg_ccy_accptd: Vec<ActiveCurrencyCode>,
    #[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
    pub red_authstn: Option<Forms1>,
    #[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
    pub amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
    pub units_ind: Option<YesNoIndicator>,
    #[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
    pub rndg: Option<RoundingDirection2Code>,
    #[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
    pub pctg_ind: Option<YesNoIndicator>,
    #[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
    pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
    #[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy_desc: Option<Max350Text>,
    #[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm: Option<IsoTime>,
    #[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm_frame: Option<TimeFrame4>,
    #[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm: Option<IsoTime>,
    #[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm_frame: Option<TimeFrame5>,
    #[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
    pub ltd_prd: Option<Max350Text>,
    #[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
    pub sttlm_cycl: Option<TimeFrame8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SignatureType1Code {
    #[serde(rename = "ORIG")]
    Orig,
    #[serde(rename = "DIGI")]
    Digi,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "NONE")]
    None,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame5 {
    #[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
    pub othr_tm_frame_desc: Option<Max350Text>,
    #[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
    pub t_plus: Option<Number>,
    #[serde(
        rename = "NonWorkgDayAdjstmnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
    #[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
    pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}
#[derive(
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
pub struct CashAccountIdentification8ChoiceEnum {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification8Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification8ChoiceEnum,
}
#[derive(
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
pub struct Period15 {
    #[validate]
    #[serde(rename = "StartDt")]
    pub start_dt: IsoDate,
    #[validate]
    #[serde(rename = "EndDt")]
    pub end_dt: IsoDate,
}
#[derive(
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
pub enum ReferToFundOrderDesk1Code {
    #[serde(rename = "RFOD")]
    Rfod,
    #[default]
    Unknown,
}
#[derive(
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
pub struct TargetMarket3 {
    #[serde(rename = "RefDt", skip_serializing_if = "Option::is_none")]
    pub ref_dt: Option<IsoDate>,
    #[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
    pub invstr_tp: Option<InvestorType2>,
    #[serde(rename = "KnwldgAndOrExprnc", skip_serializing_if = "Option::is_none")]
    pub knwldg_and_or_exprnc: Option<InvestorKnowledge1>,
    #[serde(rename = "AbltyToBearLosses", skip_serializing_if = "Option::is_none")]
    pub ablty_to_bear_losses: Option<LossBearing2>,
    #[serde(rename = "RskTlrnce", skip_serializing_if = "Option::is_none")]
    pub rsk_tlrnce: Option<RiskTolerance1>,
    #[serde(
        rename = "ClntObjctvsAndNeeds",
        skip_serializing_if = "Option::is_none"
    )]
    pub clnt_objctvs_and_needs: Option<InvestorRequirements3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherTargetMarket1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundPaymentType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundPaymentType1Code>,
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
pub struct FundPaymentType1Choice {
    #[serde(flatten)]
    pub value: FundPaymentType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LossBearing2 {
    #[serde(rename = "NoCptlLoss", skip_serializing_if = "Option::is_none")]
    pub no_cptl_loss: Option<TargetMarket1Code>,
    #[serde(rename = "LtdCptlLoss", skip_serializing_if = "Option::is_none")]
    pub ltd_cptl_loss: Option<TargetMarket1Code>,
    #[serde(rename = "LtdCptlLossLvl", skip_serializing_if = "Option::is_none")]
    pub ltd_cptl_loss_lvl: Option<PercentageRate>,
    #[serde(rename = "NoCptlGrnt", skip_serializing_if = "Option::is_none")]
    pub no_cptl_grnt: Option<TargetMarket1Code>,
    #[serde(rename = "LossByndCptl", skip_serializing_if = "Option::is_none")]
    pub loss_bynd_cptl: Option<TargetMarket1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherTargetMarketLossBearing1>,
}
#[derive(
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
pub struct OtherTargetMarket1 {
    #[validate]
    #[serde(rename = "TrgtMktTp")]
    pub trgt_mkt_tp: Max350Text,
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
pub struct TimeFrame4 {
    #[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
    pub othr_tm_frame_desc: Option<Max350Text>,
    #[serde(rename = "TMns", skip_serializing_if = "Option::is_none")]
    pub t_mns: Option<Number>,
    #[serde(
        rename = "NonWorkgDayAdjstmnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
    #[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
    pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}
#[derive(
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
pub struct DistributionStrategy1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestorType3Code>,
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
pub struct DistributionStrategy1Choice {
    #[serde(flatten)]
    pub value: DistributionStrategy1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalMarketAnnex5 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Ctry", default)]
    pub ctry: Vec<CountryCode>,
    #[validate]
    #[serde(rename = "LclOrdrDsk")]
    pub lcl_ordr_dsk: OrderDesk1,
    #[serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics8>,
    #[serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub red_prcg_chrtcs: Option<ProcessingCharacteristics5>,
    #[serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none")]
    pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshSttlmDtls", default)]
    pub csh_sttlm_dtls: Vec<CashAccount205>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GovernanceProcessType1Code {
    #[serde(rename = "BMIF")]
    Bmif,
    #[serde(rename = "NINF")]
    Ninf,
    #[serde(rename = "CMIF")]
    Cmif,
    #[serde(rename = "AMIF")]
    Amif,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentFundPlanType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundPlanType1Code>,
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
pub struct InvestmentFundPlanType1Choice {
    #[serde(flatten)]
    pub value: InvestmentFundPlanType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IntendedOrActual2Code {
    #[serde(rename = "ANTE")]
    Ante,
    #[serde(rename = "POST")]
    Post,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TargetMarket1Code {
    #[serde(rename = "YSCO")]
    Ysco,
    #[serde(rename = "NEUT")]
    Neut,
    #[serde(rename = "NSCO")]
    Nsco,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentNeed2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentNeed2Code>,
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
pub struct InvestmentNeed2Choice {
    #[serde(flatten)]
    pub value: InvestmentNeed2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmfDoctrine1Code {
    #[serde(rename = "AMF1")]
    Amf1,
    #[serde(rename = "AMF3")]
    Amf3,
    #[serde(rename = "AMF2")]
    Amf2,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherTargetMarketInvestorKnowledge1 {
    #[serde(rename = "InvstrKnwldgTp", skip_serializing_if = "Option::is_none")]
    pub invstr_knwldg_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<TargetMarket1Choice>,
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
pub struct UnitsOrAmount1ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct UnitsOrAmount1Choice {
    #[serde(flatten)]
    pub value: UnitsOrAmount1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorType3Code {
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PRF2")]
    Prf2,
    #[serde(rename = "NEI1")]
    Nei1,
    #[serde(rename = "BOT2")]
    Bot2,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ProcessingCharacteristics7 {
    #[validate(length(min = 0,))]
    #[serde(rename = "DealgCcyAccptd", default)]
    pub dealg_ccy_accptd: Vec<ActiveCurrencyCode>,
    #[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
    pub red_authstn: Option<Forms1>,
    #[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
    pub amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
    pub units_ind: Option<YesNoIndicator>,
    #[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
    pub rndg: Option<RoundingDirection2Code>,
    #[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
    pub pctg_ind: Option<YesNoIndicator>,
    #[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
    pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
    #[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy_desc: Option<Max350Text>,
    #[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm: Option<IsoTime>,
    #[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm_frame: Option<TimeFrame4>,
    #[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm: Option<IsoTime>,
    #[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm_frame: Option<TimeFrame6>,
    #[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
    pub ltd_prd: Option<Max350Text>,
    #[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
    pub sttlm_cycl: Option<TimeFrame8Choice>,
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
pub struct GenericIdentification3 {
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
pub struct ChargeType8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundMiFidFee2Code>,
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
pub struct ChargeType8Choice {
    #[serde(flatten)]
    pub value: ChargeType8ChoiceEnum,
}
#[derive(
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
pub struct TimeFrame7 {
    #[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
    pub othr_tm_frame_desc: Option<Max350Text>,
    #[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
    pub t_plus: Option<Number>,
    #[serde(
        rename = "NonWorkgDayAdjstmnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
    #[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
    pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}
#[derive(
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
pub enum EventFrequency8Code {
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FOMN")]
    Fomn,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "TYEA")]
    Tyea,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ONDE")]
    Onde,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "TWMN")]
    Twmn,
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
pub struct PartyIdentification125ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalProductInformation1 {
    #[serde(rename = "ESGCtgyGrmnFndMkt", skip_serializing_if = "Option::is_none")]
    pub esg_ctgy_grmn_fnd_mkt: Option<EsgCategoryGermanFundMarket1Code>,
    #[serde(
        rename = "ESGCtgyGrmnStrdSctiesMkt",
        skip_serializing_if = "Option::is_none"
    )]
    pub esg_ctgy_grmn_strd_scties_mkt: Option<EsgCategoryGermanStructuredSecuritiesMarket1Code>,
    #[serde(rename = "ESGFcs", skip_serializing_if = "Option::is_none")]
    pub esg_fcs: Option<EsgFocus1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ESGLablOrStd", default)]
    pub esg_labl_or_std: Vec<EsgLabelOrStandard1Code>,
}
#[derive(
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
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalAccountIdentification1Code>,
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
pub struct CashAccount205 {
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "PmryAcct", skip_serializing_if = "Option::is_none")]
    pub pmry_acct: Option<CashAccount206>,
    #[serde(rename = "ScndryAcct", skip_serializing_if = "Option::is_none")]
    pub scndry_acct: Option<CashAccount206>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContactAttributes5 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress1>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EsgLabelOrStandard1Code {
    #[serde(rename = "E005")]
    E005,
    #[serde(rename = "C003")]
    C003,
    #[serde(rename = "B002")]
    B002,
    #[serde(rename = "L012")]
    L012,
    #[serde(rename = "D004")]
    D004,
    #[serde(rename = "O015")]
    O015,
    #[serde(rename = "A001")]
    A001,
    #[serde(rename = "F006")]
    F006,
    #[serde(rename = "K011")]
    K011,
    #[serde(rename = "J010")]
    J010,
    #[serde(rename = "H008")]
    H008,
    #[serde(rename = "I009")]
    I009,
    #[serde(rename = "G007")]
    G007,
    #[serde(rename = "N014")]
    N014,
    #[serde(rename = "M013")]
    M013,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundPaymentType1Code {
    #[serde(rename = "DRAF")]
    Draf,
    #[serde(rename = "CACC")]
    Cacc,
    #[serde(rename = "CHEQ")]
    Cheq,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DDEB")]
    Ddeb,
    #[serde(rename = "CARD")]
    Card,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorKnowledge1 {
    #[serde(rename = "BsicInvstr", skip_serializing_if = "Option::is_none")]
    pub bsic_invstr: Option<TargetMarket1Code>,
    #[serde(rename = "InfrmdInvstr", skip_serializing_if = "Option::is_none")]
    pub infrmd_invstr: Option<TargetMarket1Code>,
    #[serde(rename = "AdvncdInvstr", skip_serializing_if = "Option::is_none")]
    pub advncd_invstr: Option<TargetMarket1Code>,
    #[serde(rename = "ExprtInvstrDE", skip_serializing_if = "Option::is_none")]
    pub exprt_invstr_de: Option<TargetMarket1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherTargetMarketInvestorKnowledge1>,
}
#[derive(
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
pub struct AdditionalInformation15 {
    #[validate]
    #[serde(rename = "InfTp")]
    pub inf_tp: GenericIdentification36,
    #[validate]
    #[serde(rename = "InfVal")]
    pub inf_val: Max350Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorType2Code {
    #[serde(rename = "BOT3")]
    Bot3,
    #[serde(rename = "EPRO")]
    Epro,
    #[serde(rename = "PRF2")]
    Prf2,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExPostCostCalculationBasis1Code {
    #[serde(rename = "FIXB")]
    Fixb,
    #[serde(rename = "ROLL")]
    Roll,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorType4Code {
    #[serde(rename = "BOT3")]
    Bot3,
    #[serde(rename = "NPRF")]
    Nprf,
    #[serde(rename = "PRF3")]
    Prf3,
    #[serde(rename = "PRF4")]
    Prf4,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundPlanType1Code {
    #[serde(rename = "INVP")]
    Invp,
    #[serde(rename = "SWIP")]
    Swip,
    #[serde(rename = "WTHP")]
    Wthp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalProductInformation2 {
    #[serde(rename = "AMFDctrn", skip_serializing_if = "Option::is_none")]
    pub amf_dctrn: Option<AmfDoctrine1Code>,
}
#[derive(
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
pub struct TimeFrame6 {
    #[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
    pub othr_tm_frame_desc: Option<Max350Text>,
    #[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
    pub t_plus: Option<Number>,
    #[serde(
        rename = "NonWorkgDayAdjstmnt",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
    #[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
    pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentRestrictions3 {
    #[serde(rename = "MinInitlSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub min_initl_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MinInitlSbcptUnits", skip_serializing_if = "Option::is_none")]
    pub min_initl_sbcpt_units: Option<Number>,
    #[serde(rename = "MinSbsqntSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub min_sbsqnt_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(
        rename = "MinSbsqntSbcptUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_sbsqnt_sbcpt_units: Option<Number>,
    #[serde(rename = "MaxRedAmt", skip_serializing_if = "Option::is_none")]
    pub max_red_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MaxRedUnits", skip_serializing_if = "Option::is_none")]
    pub max_red_units: Option<DecimalNumber>,
    #[serde(rename = "MinRedPctg", skip_serializing_if = "Option::is_none")]
    pub min_red_pctg: Option<DecimalNumber>,
    #[serde(rename = "OthrRedRstrctns", skip_serializing_if = "Option::is_none")]
    pub othr_red_rstrctns: Option<Max350Text>,
    #[serde(rename = "MinSwtchSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MinSwtchSbcptUnits", skip_serializing_if = "Option::is_none")]
    pub min_swtch_sbcpt_units: Option<DecimalNumber>,
    #[serde(rename = "MaxSwtchRedAmt", skip_serializing_if = "Option::is_none")]
    pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MaxSwtchRedUnits", skip_serializing_if = "Option::is_none")]
    pub max_swtch_red_units: Option<DecimalNumber>,
    #[serde(rename = "OthrSwtchRstrctns", skip_serializing_if = "Option::is_none")]
    pub othr_swtch_rstrctns: Option<Max350Text>,
    #[serde(rename = "MinHldgAmt", skip_serializing_if = "Option::is_none")]
    pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MinHldgUnits", skip_serializing_if = "Option::is_none")]
    pub min_hldg_units: Option<DecimalNumber>,
    #[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
    pub min_hldg_prd: Option<Max70Text>,
    #[serde(rename = "HldgTrfbl", skip_serializing_if = "Option::is_none")]
    pub hldg_trfbl: Option<HoldingTransferable1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QuotationType1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "PRCT")]
    Prct,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UtcOffset1 {
    #[validate]
    #[serde(rename = "Sgn")]
    pub sgn: PlusOrMinusIndicator,
    #[validate]
    #[serde(rename = "NbOfHrs")]
    pub nb_of_hrs: IsoTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TargetMarket1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TargetMarket1Code>,
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
pub struct TargetMarket1Choice {
    #[serde(flatten)]
    pub value: TargetMarket1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundMiFidFee2Code {
    #[serde(rename = "BORF")]
    Borf,
    #[serde(rename = "DIS2")]
    Dis2,
    #[serde(rename = "FES3")]
    Fes3,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "FES2")]
    Fes2,
    #[serde(rename = "GOC1")]
    Goc1,
    #[serde(rename = "GOCS")]
    Gocs,
    #[serde(rename = "INCF")]
    Incf,
    #[serde(rename = "INCS")]
    Incs,
    #[serde(rename = "MNF1")]
    Mnf1,
    #[serde(rename = "MANS")]
    Mans,
    #[serde(rename = "NET2")]
    Net2,
    #[serde(rename = "NESF")]
    Nesf,
    #[serde(rename = "NETO")]
    Neto,
    #[serde(rename = "NRAM")]
    Nram,
    #[serde(rename = "OOEA")]
    Ooea,
    #[serde(rename = "OOSF")]
    Oosf,
    #[serde(rename = "OOSS")]
    Ooss,
    #[serde(rename = "BENS")]
    Bens,
    #[serde(rename = "ENAC")]
    Enac,
    #[serde(rename = "ENFX")]
    Enfx,
    #[serde(rename = "EXAC")]
    Exac,
    #[serde(rename = "ENBX")]
    Enbx,
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "PENO")]
    Peno,
    #[serde(rename = "OTES")]
    Otes,
    #[serde(rename = "OCAS")]
    Ocas,
    #[serde(rename = "RPSS")]
    Rpss,
    #[serde(rename = "TRS1")]
    Trs1,
    #[default]
    Unknown,
}
#[derive(
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
pub struct FundOrderType5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundOrderType10Code>,
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
pub struct FundOrderType5Choice {
    #[serde(flatten)]
    pub value: FundOrderType5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherTargetMarketRiskTolerance1 {
    #[serde(rename = "RskTlrnceTp", skip_serializing_if = "Option::is_none")]
    pub rsk_tlrnce_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<TargetMarket1Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionPolicy1Code {
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "ACCU")]
    Accu,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TimeFrame2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame9Choice {
    #[serde(flatten)]
    pub value: TimeFrame9ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency5Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "TWMN")]
    Twmn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductStructure1Code {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "NUMM")]
    Numm,
    #[serde(rename = "UCMM")]
    Ucmm,
    #[serde(rename = "EXTC")]
    Extc,
    #[serde(rename = "UCIT")]
    Ucit,
    #[serde(rename = "SSEC")]
    Ssec,
    #[serde(rename = "SFUN")]
    Sfun,
    #[serde(rename = "NUCI")]
    Nuci,
    #[default]
    Unknown,
}
#[derive(
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
pub struct IndividualCostOrCharge2 {
    #[serde(rename = "CostTp")]
    pub cost_tp: ChargeType8Choice,
    #[serde(rename = "ExAnteOrExPst")]
    pub ex_ante_or_ex_pst: IntendedOrActual2Code,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "RefPrd", skip_serializing_if = "Option::is_none")]
    pub ref_prd: Option<Period15>,
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
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RiskLevel1Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "LOWW")]
    Loww,
    #[serde(rename = "MEDM")]
    Medm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EsgCategoryGermanFundMarket1Code {
    #[serde(rename = "NEUT")]
    Neut,
    #[serde(rename = "IMPF")]
    Impf,
    #[serde(rename = "ESGF")]
    Esgf,
    #[serde(rename = "BASF")]
    Basf,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ExPostCostCalculationBasis1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExPostCostCalculationBasis1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExPostCostCalculationBasis1Choice {
    #[serde(flatten)]
    pub value: ExPostCostCalculationBasis1ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TargetMarket2Code {
    #[serde(rename = "NEUT")]
    Neut,
    #[serde(rename = "YSCO")]
    Ysco,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundOrderType10Code {
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "RDIV")]
    Rdiv,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "RGSV")]
    Rgsv,
    #[serde(rename = "WIDP")]
    Widp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorType2 {
    #[serde(rename = "InvstrTpRtl", skip_serializing_if = "Option::is_none")]
    pub invstr_tp_rtl: Option<TargetMarket1Code>,
    #[serde(rename = "InvstrTpPrfssnl", skip_serializing_if = "Option::is_none")]
    pub invstr_tp_prfssnl: Option<TargetMarket5Choice>,
    #[serde(
        rename = "InvstrTpElgblCtrPty",
        skip_serializing_if = "Option::is_none"
    )]
    pub invstr_tp_elgbl_ctr_pty: Option<TargetMarket3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherTargetMarketInvestor1>,
}
#[derive(
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DividendPolicy1Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "UNIT")]
    Unit,
    #[serde(rename = "BOTH")]
    Both,
    #[default]
    Unknown,
}
#[derive(
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
pub struct CashAccount206 {
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: AccountIdentificationAndName7,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "AcctTpDesc", skip_serializing_if = "Option::is_none")]
    pub acct_tp_desc: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeHorizon2ChoiceEnum {
    #[serde(rename = "TmFrame", skip_serializing_if = "Option::is_none")]
    pub tm_frame: Option<TimeFrame9Choice>,
    #[serde(rename = "NbOfYrs", skip_serializing_if = "Option::is_none")]
    pub nb_of_yrs: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeHorizon2Choice {
    #[serde(flatten)]
    pub value: TimeHorizon2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuotationType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<QuotationType1Code>,
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
pub struct QuotationType1Choice {
    #[serde(flatten)]
    pub value: QuotationType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotionalOrUnitBased1Code {
    #[serde(rename = "UNIT")]
    Unit,
    #[serde(rename = "NOTI")]
    Noti,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductStructure1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProductStructure1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProductStructure1Choice {
    #[serde(flatten)]
    pub value: ProductStructure1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TargetMarket5ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InvestorType4Code>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<TargetMarket1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TargetMarket5Choice {
    #[serde(flatten)]
    pub value: TargetMarket5ChoiceEnum,
}
#[derive(
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
pub struct FundParties1 {
    #[serde(rename = "Guarntr", skip_serializing_if = "Option::is_none")]
    pub guarntr: Option<ContactAttributes5>,
    #[serde(rename = "Audtr", skip_serializing_if = "Option::is_none")]
    pub audtr: Option<ContactAttributes5>,
    #[serde(rename = "Trstee", skip_serializing_if = "Option::is_none")]
    pub trstee: Option<ContactAttributes5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrPty", default)]
    pub othr_pty: Vec<ExtendedParty13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame8ChoiceEnum {
    #[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
    pub t_plus: Option<Number>,
    #[serde(rename = "RPlus", skip_serializing_if = "Option::is_none")]
    pub r_plus: Option<Number>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame8Choice {
    #[serde(flatten)]
    pub value: TimeFrame8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherInvestmentNeed1 {
    #[serde(
        rename = "ClntObjctvsAndNeedsTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub clnt_objctvs_and_needs_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<TargetMarket1Choice>,
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
pub struct OrderDesk1 {
    #[serde(rename = "OrdrDsk", skip_serializing_if = "Option::is_none")]
    pub ordr_dsk: Option<ContactAttributes5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClsrDts", default)]
    pub clsr_dts: Vec<IsoDate>,
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
pub struct DistributionStrategy1 {
    #[serde(rename = "ExctnOnly", skip_serializing_if = "Option::is_none")]
    pub exctn_only: Option<DistributionStrategy1Choice>,
    #[serde(
        rename = "ExctnWthApprprtnssTstOrNonAdvsdSvcs",
        skip_serializing_if = "Option::is_none"
    )]
    pub exctn_wth_apprprtnss_tst_or_non_advsd_svcs: Option<DistributionStrategy1Choice>,
    #[serde(rename = "InvstmtAdvc", skip_serializing_if = "Option::is_none")]
    pub invstmt_advc: Option<DistributionStrategy1Choice>,
    #[serde(rename = "PrtflMgmt", skip_serializing_if = "Option::is_none")]
    pub prtfl_mgmt: Option<DistributionStrategy1Choice>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OtherDistributionStrategy1>,
}
#[derive(
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
pub struct PaymentInstrument16 {
    #[serde(rename = "OrdrTp")]
    pub ordr_tp: FundOrderType5Choice,
    #[serde(rename = "InstrmTp")]
    pub instrm_tp: FundPaymentType1Choice,
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
pub struct GovernanceProcess1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GovernanceProcessType1Code>,
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
pub struct GovernanceProcess1Choice {
    #[serde(flatten)]
    pub value: GovernanceProcess1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification41 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: SecurityIdentification40,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "UmbrllNm", skip_serializing_if = "Option::is_none")]
    pub umbrll_nm: Option<Max35Text>,
    #[serde(rename = "NewUmbrll", skip_serializing_if = "Option::is_none")]
    pub new_umbrll: Option<YesNoIndicator>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
    #[serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none")]
    pub base_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none")]
    pub ctry_of_dmcl: Option<CountryCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RegdDstrbtnCtry", default)]
    pub regd_dstrbtn_ctry: Vec<CountryCode>,
    #[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
    pub pdct_tp: Option<ProductStructure1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<ContactAttributes5>,
    #[serde(rename = "IssrPdctGovncPrc", skip_serializing_if = "Option::is_none")]
    pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
    #[serde(rename = "PdctCtgy", skip_serializing_if = "Option::is_none")]
    pub pdct_ctgy: Option<Max140Text>,
    #[serde(rename = "PdctCtgyDE", skip_serializing_if = "Option::is_none")]
    pub pdct_ctgy_de: Option<Max140Text>,
    #[serde(rename = "NtnlOrUnitBased", skip_serializing_if = "Option::is_none")]
    pub ntnl_or_unit_based: Option<NotionalOrUnitBased1Choice>,
    #[serde(rename = "QtnTp", skip_serializing_if = "Option::is_none")]
    pub qtn_tp: Option<QuotationType1Choice>,
    #[serde(rename = "LvrgdOrCntgntLblty", skip_serializing_if = "Option::is_none")]
    pub lvrgd_or_cntgnt_lblty: Option<YesNoIndicator>,
    #[serde(rename = "NoRtrcssnInd", skip_serializing_if = "Option::is_none")]
    pub no_rtrcssn_ind: Option<YesNoIndicator>,
    #[serde(rename = "ExPstCostClctnBsis", skip_serializing_if = "Option::is_none")]
    pub ex_pst_cost_clctn_bsis: Option<ExPostCostCalculationBasis1Choice>,
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
pub struct Forms1 {
    #[validate]
    #[serde(rename = "ApplForm")]
    pub appl_form: YesNoIndicator,
    #[serde(rename = "SgntrTp")]
    pub sgntr_tp: SignatureType1Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SustainabilityPreferences1Code {
    #[serde(rename = "CESG")]
    Cesg,
    #[serde(rename = "NEUT")]
    Neut,
    #[serde(rename = "OSUS")]
    Osus,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EsgCategoryGermanStructuredSecuritiesMarket1Code {
    #[serde(rename = "NEUT")]
    Neut,
    #[serde(rename = "IMPS")]
    Imps,
    #[serde(rename = "ESGS")]
    Esgs,
    #[serde(rename = "BASS")]
    Bass,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AnnualChargePaymentType1Code {
    #[serde(rename = "CAPL")]
    Capl,
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "FndRefDataRpt")]
    pub fnd_ref_data_rpt: FundReferenceDataReportV06,
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
pub struct CostsAndCharges2 {
    #[serde(rename = "ExAnteRefDt", skip_serializing_if = "Option::is_none")]
    pub ex_ante_ref_dt: Option<IsoDate>,
    #[validate(length(min = 1,))]
    #[serde(rename = "IndvCostOrChrg", default)]
    pub indv_cost_or_chrg: Vec<IndividualCostOrCharge2>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BusinessDayConvention1Code {
    #[serde(rename = "FWNG")]
    Fwng,
    #[serde(rename = "PREC")]
    Prec,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherDistributionStrategy1 {
    #[serde(rename = "DstrbtnStrtgyTp", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_strtgy_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<DistributionStrategy1Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EuSavingsDirective1Code {
    #[serde(rename = "EUSI")]
    Eusi,
    #[serde(rename = "EUSO")]
    Euso,
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
pub struct MainFundOrderDeskLocation1 {
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "TmZoneOffSet")]
    pub tm_zone_off_set: UtcOffset1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RoundingDirection2Code {
    #[serde(rename = "RDUP")]
    Rdup,
    #[serde(rename = "RDWN")]
    Rdwn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentNeed2Code {
    #[serde(rename = "NSPE")]
    Nspe,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "ISLB")]
    Islb,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification40 {
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherTargetMarketInvestor1 {
    #[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
    pub invstr_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<TargetMarket3Choice>,
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
pub struct TargetMarket3ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InvestorType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<TargetMarket1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TargetMarket3Choice {
    #[serde(flatten)]
    pub value: TargetMarket3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValuationDealingProcessingCharacteristics3 {
    #[serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none")]
    pub valtn_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "ValtnFrqcyDesc", skip_serializing_if = "Option::is_none")]
    pub valtn_frqcy_desc: Option<Max350Text>,
    #[serde(rename = "ValtnTm", skip_serializing_if = "Option::is_none")]
    pub valtn_tm: Option<IsoTime>,
    #[serde(rename = "DcmlstnUnits", skip_serializing_if = "Option::is_none")]
    pub dcmlstn_units: Option<Number>,
    #[serde(rename = "DcmlstnPric", skip_serializing_if = "Option::is_none")]
    pub dcmlstn_pric: Option<Number>,
    #[serde(rename = "DualFndInd", skip_serializing_if = "Option::is_none")]
    pub dual_fnd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
    pub pric_mtd: Option<PriceMethod1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PricCcy", default)]
    pub pric_ccy: Vec<ActiveCurrencyCode>,
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
pub struct ProcessingCharacteristics6 {
    #[validate(length(min = 0,))]
    #[serde(rename = "DealgCcyAccptd", default)]
    pub dealg_ccy_accptd: Vec<ActiveCurrencyCode>,
    #[serde(rename = "SwtchAuthstn", skip_serializing_if = "Option::is_none")]
    pub swtch_authstn: Option<Forms1>,
    #[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
    pub amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
    pub units_ind: Option<YesNoIndicator>,
    #[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
    pub rndg: Option<RoundingDirection2Code>,
    #[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
    pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
    #[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy_desc: Option<Max350Text>,
    #[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm: Option<IsoTime>,
    #[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm_frame: Option<TimeFrame4>,
    #[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm: Option<IsoTime>,
    #[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm_frame: Option<TimeFrame5>,
    #[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
    pub ltd_prd: Option<Max350Text>,
    #[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
    pub sttlm_cycl: Option<TimeFrame8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TargetMarket3Code {
    #[serde(rename = "YSCO")]
    Ysco,
    #[serde(rename = "NSCO")]
    Nsco,
    #[default]
    Unknown,
}
#[derive(
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
pub struct InvestorRequirements3 {
    #[serde(rename = "RtrPrflPrsrvtn", skip_serializing_if = "Option::is_none")]
    pub rtr_prfl_prsrvtn: Option<TargetMarket1Code>,
    #[serde(rename = "RtrPrflGrwth", skip_serializing_if = "Option::is_none")]
    pub rtr_prfl_grwth: Option<TargetMarket1Code>,
    #[serde(rename = "RtrPrflIncm", skip_serializing_if = "Option::is_none")]
    pub rtr_prfl_incm: Option<TargetMarket1Code>,
    #[serde(rename = "RtrPrflHdgg", skip_serializing_if = "Option::is_none")]
    pub rtr_prfl_hdgg: Option<TargetMarket1Code>,
    #[serde(rename = "OptnOrLvrgdRtrPrfl", skip_serializing_if = "Option::is_none")]
    pub optn_or_lvrgd_rtr_prfl: Option<TargetMarket1Code>,
    #[serde(rename = "RtrPrflPnsnSchmeDE", skip_serializing_if = "Option::is_none")]
    pub rtr_prfl_pnsn_schme_de: Option<TargetMarket1Code>,
    #[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
    pub min_hldg_prd: Option<TimeHorizon2Choice>,
    #[serde(rename = "ESGPrefs", skip_serializing_if = "Option::is_none")]
    pub esg_prefs: Option<TargetMarket2Code>,
    #[serde(rename = "SstnbltyPrefs", skip_serializing_if = "Option::is_none")]
    pub sstnblty_prefs: Option<SustainabilityPreferences1Code>,
    #[serde(
        rename = "OthrSpcfcInvstmtNeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_spcfc_invstmt_need: Option<InvestmentNeed2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherInvestmentNeed1>,
}
#[derive(
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
pub struct OtherTargetMarketLossBearing1 {
    #[serde(
        rename = "AbltyToBearLossesTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub ablty_to_bear_losses_tp: Option<Max35Text>,
    #[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
    pub trgt: Option<TargetMarket1Choice>,
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
pub struct SecurityClassificationType2ChoiceEnum {
    #[serde(rename = "CFI", skip_serializing_if = "Option::is_none")]
    pub cfi: Option<CfiOct2015Identifier>,
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityClassificationType2Choice {
    #[serde(flatten)]
    pub value: SecurityClassificationType2ChoiceEnum,
}
#[derive(
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
pub struct TimeFrame7ChoiceEnum {
    #[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
    pub t_plus: Option<Number>,
    #[serde(rename = "Prepmt", skip_serializing_if = "Option::is_none")]
    pub prepmt: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeFrame7Choice {
    #[serde(flatten)]
    pub value: TimeFrame7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max1Number {
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
pub struct Frequency20ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency20Choice {
    #[serde(flatten)]
    pub value: Frequency20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundReferenceDataReportV06 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrvsRef", default)]
    pub prvs_ref: Vec<AdditionalReference10>,
    #[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ref: Option<AdditionalReference10>,
    #[serde(rename = "FndRefDataRptId", skip_serializing_if = "Option::is_none")]
    pub fnd_ref_data_rpt_id: Option<Max35Text>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rpt", default)]
    pub rpt: Vec<FundReferenceDataReport4>,
}
#[derive(
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
pub struct ProcessingCharacteristics8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "DealgCcyAccptd", default)]
    pub dealg_ccy_accptd: Vec<ActiveCurrencyCode>,
    #[serde(rename = "InitlInvstmtAppl", skip_serializing_if = "Option::is_none")]
    pub initl_invstmt_appl: Option<Forms1>,
    #[serde(rename = "SbsqntInvstmtAppl", skip_serializing_if = "Option::is_none")]
    pub sbsqnt_invstmt_appl: Option<Forms1>,
    #[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
    pub amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
    pub units_ind: Option<YesNoIndicator>,
    #[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
    pub rndg: Option<RoundingDirection2Code>,
    #[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
    pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
    #[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy: Option<EventFrequency5Code>,
    #[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
    pub dealg_frqcy_desc: Option<Max350Text>,
    #[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm: Option<IsoTime>,
    #[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
    pub dealg_cut_off_tm_frame: Option<TimeFrame4>,
    #[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm: Option<IsoTime>,
    #[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
    pub deal_conf_tm_frame: Option<TimeFrame7>,
    #[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
    pub ltd_prd: Option<Max350Text>,
    #[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
    pub sttlm_cycl: Option<TimeFrame7Choice>,
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
pub struct InvestmentPlanCharacteristics1 {
    #[serde(rename = "PlanTp")]
    pub plan_tp: InvestmentFundPlanType1Choice,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<Frequency20Choice>,
    #[serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_instlmts: Option<Number>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<UnitsOrAmount1Choice>,
    #[serde(rename = "PlanConttn", skip_serializing_if = "Option::is_none")]
    pub plan_conttn: Option<YesNoIndicator>,
    #[serde(rename = "AddtlSbcpt", skip_serializing_if = "Option::is_none")]
    pub addtl_sbcpt: Option<YesNoIndicator>,
    #[serde(rename = "AddtlSbcptFctn", skip_serializing_if = "Option::is_none")]
    pub addtl_sbcpt_fctn: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EsgFocus1Code {
    #[serde(rename = "ENVR")]
    Envr,
    #[serde(rename = "GOVR")]
    Govr,
    #[serde(rename = "SOCL")]
    Socl,
    #[default]
    Unknown,
}
#[derive(
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
pub enum TimeFrame2Code {
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "MEDM")]
    Medm,
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "VSHT")]
    Vsht,
    #[default]
    Unknown,
}
