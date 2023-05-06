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
    static ref ENTRY_TYPE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[BEOVW]{1,1}[0-9]{2,2}|DUM"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UUI_DV_4_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.005.001.09".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemIdentification3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalCashClearingSystem1Code>,
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
pub struct ClearingSystemIdentification3Choice {
    #[serde(flatten)]
    pub value: ClearingSystemIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ImpliedCurrencyAmountRange1ChoiceEnum {
    #[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
    pub fr_amt: Option<AmountRangeBoundary1>,
    #[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
    pub to_amt: Option<AmountRangeBoundary1>,
    #[serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none")]
    pub fr_to_amt: Option<FromToAmountRange1>,
    #[serde(rename = "EQAmt", skip_serializing_if = "Option::is_none")]
    pub eq_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none")]
    pub neq_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ImpliedCurrencyAmountRange1Choice {
    #[serde(flatten)]
    pub value: ImpliedCurrencyAmountRange1ChoiceEnum,
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
pub struct Priority1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Priority5Code>,
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
pub struct Priority1Choice {
    #[serde(flatten)]
    pub value: Priority1ChoiceEnum,
}
#[derive(
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
pub struct ExternalPersonIdentification1Code {
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EntryStatus1Code {
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "PDNG")]
    Pdng,
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
pub struct OtherContact1 {
    #[validate]
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: Max4Text,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max128Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PreferredContactMethod1Code {
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "CELL")]
    Cell,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentOrigin1ChoiceEnum {
    #[serde(rename = "FINMT", skip_serializing_if = "Option::is_none")]
    pub finmt: Option<Max3NumericText>,
    #[serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none")]
    pub xml_msg_nm: Option<Max35Text>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Instrm", skip_serializing_if = "Option::is_none")]
    pub instrm: Option<PaymentInstrument1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentOrigin1Choice {
    #[serde(flatten)]
    pub value: PaymentOrigin1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FromToAmountRange1 {
    #[validate]
    #[serde(rename = "FrAmt")]
    pub fr_amt: AmountRangeBoundary1,
    #[validate]
    #[serde(rename = "ToAmt")]
    pub to_amt: AmountRangeBoundary1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EntryTypeIdentifier {
    #[validate(regex = "ENTRY_TYPE_IDENTIFIER_REGEX")]
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
pub struct ActiveAmountRange3ChoiceEnum {
    #[serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none")]
    pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
    #[serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none")]
    pub ccy_and_amt_rg: Option<ActiveCurrencyAndAmountRange3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveAmountRange3Choice {
    #[serde(flatten)]
    pub value: ActiveAmountRange3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification135 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentStatusCodeSearch2ChoiceEnum {
    #[serde(rename = "PdgSts", skip_serializing_if = "Option::is_none")]
    pub pdg_sts: Option<PendingStatus4Code>,
    #[serde(rename = "FnlSts", skip_serializing_if = "Option::is_none")]
    pub fnl_sts: Option<FinalStatusCode>,
    #[serde(rename = "PdgAndFnlSts", skip_serializing_if = "Option::is_none")]
    pub pdg_and_fnl_sts: Option<CashPaymentStatus2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentStatusCodeSearch2Choice {
    #[serde(flatten)]
    pub value: PaymentStatusCodeSearch2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentReturnCriteria4 {
    #[serde(rename = "MsgIdInd", skip_serializing_if = "Option::is_none")]
    pub msg_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "ReqdExctnDtInd", skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt_ind: Option<RequestedIndicator>,
    #[serde(rename = "InstrInd", skip_serializing_if = "Option::is_none")]
    pub instr_ind: Option<RequestedIndicator>,
    #[serde(rename = "InstrStsRtrCrit", skip_serializing_if = "Option::is_none")]
    pub instr_sts_rtr_crit: Option<InstructionStatusReturnCriteria1>,
    #[serde(rename = "InstdAmtInd", skip_serializing_if = "Option::is_none")]
    pub instd_amt_ind: Option<RequestedIndicator>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<RequestedIndicator>,
    #[serde(rename = "IntrBkSttlmAmtInd", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_amt_ind: Option<RequestedIndicator>,
    #[serde(rename = "PrtyInd", skip_serializing_if = "Option::is_none")]
    pub prty_ind: Option<RequestedIndicator>,
    #[serde(rename = "PrcgVldtyTmInd", skip_serializing_if = "Option::is_none")]
    pub prcg_vldty_tm_ind: Option<RequestedIndicator>,
    #[serde(rename = "PurpInd", skip_serializing_if = "Option::is_none")]
    pub purp_ind: Option<RequestedIndicator>,
    #[serde(rename = "InstrCpyInd", skip_serializing_if = "Option::is_none")]
    pub instr_cpy_ind: Option<RequestedIndicator>,
    #[serde(rename = "PmtMTInd", skip_serializing_if = "Option::is_none")]
    pub pmt_mt_ind: Option<RequestedIndicator>,
    #[serde(rename = "PmtTpInd", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_ind: Option<RequestedIndicator>,
    #[serde(rename = "TxIdInd", skip_serializing_if = "Option::is_none")]
    pub tx_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "IntrBkSttlmDtInd", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt_ind: Option<RequestedIndicator>,
    #[serde(rename = "EndToEndIdInd", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "PmtMtdInd", skip_serializing_if = "Option::is_none")]
    pub pmt_mtd_ind: Option<RequestedIndicator>,
    #[serde(rename = "DbtrInd", skip_serializing_if = "Option::is_none")]
    pub dbtr_ind: Option<RequestedIndicator>,
    #[serde(rename = "DbtrAgtInd", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_ind: Option<RequestedIndicator>,
    #[serde(
        rename = "InstgRmbrsmntAgtInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub instg_rmbrsmnt_agt_ind: Option<RequestedIndicator>,
    #[serde(
        rename = "InstdRmbrsmntAgtInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub instd_rmbrsmnt_agt_ind: Option<RequestedIndicator>,
    #[serde(rename = "IntrmyInd", skip_serializing_if = "Option::is_none")]
    pub intrmy_ind: Option<RequestedIndicator>,
    #[serde(rename = "CdtrAgtInd", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_ind: Option<RequestedIndicator>,
    #[serde(rename = "CdtrInd", skip_serializing_if = "Option::is_none")]
    pub cdtr_ind: Option<RequestedIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max2048Text>,
    #[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<Max35Text>,
    #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<Max35Text>,
    #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<Max35Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherContact1>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinalStatusCode {
    #[serde(rename = "STLD")]
    Stld,
    #[serde(rename = "RJTD")]
    Rjtd,
    #[serde(rename = "CAND")]
    Cand,
    #[serde(rename = "FNLD")]
    Fnld,
    #[default]
    Unknown,
}
#[derive(
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
pub struct TransactionCriteria6ChoiceEnum {
    #[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
    pub new_crit: Option<TransactionCriteria9>,
    #[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
    pub qry_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCriteria6Choice {
    #[serde(flatten)]
    pub value: TransactionCriteria6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPlaceOfBirth1 {
    #[validate]
    #[serde(rename = "BirthDt")]
    pub birth_dt: IsoDate,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: Max35Text,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: CountryCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentType3Code {
    #[serde(rename = "CBS")]
    Cbs,
    #[serde(rename = "BCK")]
    Bck,
    #[serde(rename = "BAL")]
    Bal,
    #[serde(rename = "CLS")]
    Cls,
    #[serde(rename = "CTR")]
    Ctr,
    #[serde(rename = "CBH")]
    Cbh,
    #[serde(rename = "CBP")]
    Cbp,
    #[serde(rename = "DPG")]
    Dpg,
    #[serde(rename = "DPN")]
    Dpn,
    #[serde(rename = "EXP")]
    Exp,
    #[serde(rename = "TCH")]
    Tch,
    #[serde(rename = "LMT")]
    Lmt,
    #[serde(rename = "LIQ")]
    Liq,
    #[serde(rename = "DPP")]
    Dpp,
    #[serde(rename = "DPH")]
    Dph,
    #[serde(rename = "DPS")]
    Dps,
    #[serde(rename = "STF")]
    Stf,
    #[serde(rename = "TRP")]
    Trp,
    #[serde(rename = "TCS")]
    Tcs,
    #[serde(rename = "LOA")]
    Loa,
    #[serde(rename = "LOR")]
    Lor,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "OND")]
    Ond,
    #[serde(rename = "MGL")]
    Mgl,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericPersonIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QueueTransactionIdentification1 {
    #[validate]
    #[serde(rename = "QId")]
    pub q_id: Max16Text,
    #[validate]
    #[serde(rename = "PosInQ")]
    pub pos_in_q: Max16Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ShortPaymentIdentification2 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[validate]
    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: IsoDate,
    #[validate]
    #[serde(rename = "InstgAgt")]
    pub instg_agt: BranchAndFinancialInstitutionIdentification6,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "GetTx")]
    pub get_tx: GetTransactionV09<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority5Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "LOWW")]
    Loww,
    #[serde(rename = "NORM")]
    Norm,
    #[serde(rename = "URGT")]
    Urgt,
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
pub struct SystemSearch4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "SysId", default)]
    pub sys_id: Vec<ClearingSystemIdentification3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MmbId", default)]
    pub mmb_id: Vec<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<AccountIdentification4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportIndicator1Code {
    #[serde(rename = "STND")]
    Stnd,
    #[serde(rename = "PRPR")]
    Prpr,
    #[default]
    Unknown,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAmountRange1Choice,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Ccy")]
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
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[validate]
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalOrganisationIdentification1Code>,
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
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequestType4ChoiceEnum {
    #[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
    pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
    #[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
    pub enqry: Option<ExternalEnquiryRequestType1Code>,
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
pub struct RequestType4Choice {
    #[serde(flatten)]
    pub value: RequestType4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionStatusSearch5 {
    #[serde(rename = "PmtInstrSts", skip_serializing_if = "Option::is_none")]
    pub pmt_instr_sts: Option<PaymentStatusCodeSearch2Choice>,
    #[serde(rename = "PmtInstrStsDtTm", skip_serializing_if = "Option::is_none")]
    pub pmt_instr_sts_dt_tm: Option<DateTimePeriod1Choice>,
    #[serde(rename = "PrtryStsRsn", skip_serializing_if = "Option::is_none")]
    pub prtry_sts_rsn: Option<Max4AlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod1ChoiceEnum {
    #[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
    pub dt_tm_rg: Option<DateTimePeriod1>,
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
pub struct DateTimePeriod1Choice {
    #[serde(flatten)]
    pub value: DateTimePeriod1ChoiceEnum,
}
#[derive(
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
pub struct TransactionSearchCriteria9 {
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtTo", default)]
    pub pmt_to: Vec<SystemSearch4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtFr", default)]
    pub pmt_fr: Vec<SystemSearch4>,
    #[serde(rename = "PmtSch", skip_serializing_if = "Option::is_none")]
    pub pmt_sch: Option<PaymentSearch8>,
    #[serde(rename = "AcctNtrySch", skip_serializing_if = "Option::is_none")]
    pub acct_ntry_sch: Option<CashAccountEntrySearch7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[validate]
    #[serde(rename = "MmbId")]
    pub mmb_id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingStatus4Code {
    #[serde(rename = "ACPD")]
    Acpd,
    #[serde(rename = "VALD")]
    Vald,
    #[serde(rename = "MATD")]
    Matd,
    #[serde(rename = "AUTD")]
    Autd,
    #[serde(rename = "INVD")]
    Invd,
    #[serde(rename = "UMAC")]
    Umac,
    #[serde(rename = "STLE")]
    Stle,
    #[serde(rename = "STLM")]
    Stlm,
    #[serde(rename = "SSPD")]
    Sspd,
    #[serde(rename = "PCAN")]
    Pcan,
    #[serde(rename = "PSTL")]
    Pstl,
    #[serde(rename = "PFST")]
    Pfst,
    #[serde(rename = "SMLR")]
    Smlr,
    #[serde(rename = "RMLR")]
    Rmlr,
    #[serde(rename = "SRBL")]
    Srbl,
    #[serde(rename = "AVLB")]
    Avlb,
    #[serde(rename = "SRML")]
    Srml,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
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
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: FinancialIdentificationSchemeName1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCashClearingSystem1Code {
    #[validate(length(min = 1, max = 3,))]
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
pub struct AmountRangeBoundary1 {
    #[validate]
    #[serde(rename = "BdryAmt")]
    pub bdry_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "Incl")]
    pub incl: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionStatusReturnCriteria1 {
    #[validate]
    #[serde(rename = "PmtInstrStsInd")]
    pub pmt_instr_sts_ind: RequestedIndicator,
    #[serde(rename = "PmtInstrStsDtTmInd", skip_serializing_if = "Option::is_none")]
    pub pmt_instr_sts_dt_tm_ind: Option<RequestedIndicator>,
    #[serde(rename = "PmtInstrStsRsnInd", skip_serializing_if = "Option::is_none")]
    pub pmt_instr_sts_rsn_ind: Option<RequestedIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPaymentControlRequestType1Code {
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
pub struct Party40ChoiceEnum {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification135>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party40Choice {
    #[serde(flatten)]
    pub value: Party40ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeSearch3ChoiceEnum {
    #[serde(rename = "DtTmSch", skip_serializing_if = "Option::is_none")]
    pub dt_tm_sch: Option<DateTimePeriod1Choice>,
    #[serde(rename = "DtSch", skip_serializing_if = "Option::is_none")]
    pub dt_sch: Option<DatePeriodSearch1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeSearch3Choice {
    #[serde(flatten)]
    pub value: DateAndDateTimeSearch3ChoiceEnum,
}
#[derive(
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
pub struct Iban2007Identifier {
    #[validate(regex = "IBAN_2007_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashPaymentStatus2Code {
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "FINL")]
    Finl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max128Text {
    #[validate(length(min = 1, max = 128,))]
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
pub struct PaymentSearch8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgId", default)]
    pub msg_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ReqdExctnDt", default)]
    pub reqd_exctn_dt: Vec<DateAndDateTimeSearch3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtId", default)]
    pub pmt_id: Vec<PaymentIdentification6Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Sts", default)]
    pub sts: Vec<InstructionStatusSearch5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InstdAmt", default)]
    pub instd_amt: Vec<ActiveOrHistoricAmountRange2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InstdAmtCcy", default)]
    pub instd_amt_ccy: Vec<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrBkSttlmAmt", default)]
    pub intr_bk_sttlm_amt: Vec<ActiveAmountRange3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrBkSttlmAmtCcy", default)]
    pub intr_bk_sttlm_amt_ccy: Vec<ActiveCurrencyCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtMtd", default)]
    pub pmt_mtd: Vec<PaymentOrigin1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmtTp", default)]
    pub pmt_tp: Vec<PaymentType4Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Prty", default)]
    pub prty: Vec<Priority1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrcgVldtyTm", default)]
    pub prcg_vldty_tm: Vec<DateTimePeriod1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Instr", default)]
    pub instr: Vec<Instruction1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxId", default)]
    pub tx_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrBkSttlmDt", default)]
    pub intr_bk_sttlm_dt: Vec<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EndToEndId", default)]
    pub end_to_end_id: Vec<Max35Text>,
    #[serde(rename = "Pties", skip_serializing_if = "Option::is_none")]
    pub pties: Option<PaymentTransactionParty3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentTransactionParty3 {
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<Party40Choice>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt_3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<Party40Choice>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<Party40Choice>,
}
#[derive(
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
pub struct GenericPersonIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
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
pub struct DateTimePeriod1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Instruction1Code {
    #[serde(rename = "PBEN")]
    Pben,
    #[serde(rename = "TTIL")]
    Ttil,
    #[serde(rename = "TFRO")]
    Tfro,
    #[default]
    Unknown,
}
#[derive(
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
pub struct PaymentType4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PaymentType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentType4Choice {
    #[serde(flatten)]
    pub value: PaymentType4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SystemReturnCriteria2 {
    #[serde(rename = "SysIdInd", skip_serializing_if = "Option::is_none")]
    pub sys_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "MmbIdInd", skip_serializing_if = "Option::is_none")]
    pub mmb_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "CtryIdInd", skip_serializing_if = "Option::is_none")]
    pub ctry_id_ind: Option<RequestedIndicator>,
    #[serde(rename = "AcctIdInd", skip_serializing_if = "Option::is_none")]
    pub acct_id_ind: Option<RequestedIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionCriteria9 {
    #[serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none")]
    pub new_qry_nm: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SchCrit", default)]
    pub sch_crit: Vec<TransactionSearchCriteria9>,
    #[serde(rename = "StmtRpt", skip_serializing_if = "Option::is_none")]
    pub stmt_rpt: Option<ReportIndicator1Code>,
    #[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
    pub rtr_crit: Option<TransactionReturnCriteria5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress24 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<Max35Text>,
    #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
    pub flr: Option<Max70Text>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<Max16Text>,
    #[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
    pub room: Option<Max70Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<Max35Text>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<Max35Text>,
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
pub struct BranchData3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DatePeriod2 {
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
pub struct ExternalEnquiryRequestType1Code {
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
pub struct GenericFinancialIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
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
pub struct ActiveOrHistoricAmountRange2ChoiceEnum {
    #[serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none")]
    pub ccy_and_amt_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
    #[serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none")]
    pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricAmountRange2Choice {
    #[serde(flatten)]
    pub value: ActiveOrHistoricAmountRange2ChoiceEnum,
}
#[derive(
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
pub struct ActiveCurrencyAndAmountRange3 {
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAmountRange1Choice,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Ccy")]
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
pub struct CashAccountEntrySearch7 {
    #[validate(length(min = 0,))]
    #[serde(rename = "AcctId", default)]
    pub acct_id: Vec<AccountIdentificationSearchCriteria2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtryAmt", default)]
    pub ntry_amt: Vec<ActiveOrHistoricAmountRange2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtryAmtCcy", default)]
    pub ntry_amt_ccy: Vec<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtrySts", default)]
    pub ntry_sts: Vec<EntryStatus1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtryDt", default)]
    pub ntry_dt: Vec<DateAndDateTimeSearch3Choice>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification135>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentIdentification6ChoiceEnum {
    #[serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none")]
    pub shrt_biz_id: Option<ShortPaymentIdentification2>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<Max70Text>,
    #[serde(rename = "LngBizId", skip_serializing_if = "Option::is_none")]
    pub lng_biz_id: Option<LongPaymentIdentification2>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<Max35Text>,
    #[serde(rename = "QId", skip_serializing_if = "Option::is_none")]
    pub q_id: Option<QueueTransactionIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentIdentification6Choice {
    #[serde(flatten)]
    pub value: PaymentIdentification6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party38ChoiceEnum {
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification13>,
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party38Choice {
    #[serde(flatten)]
    pub value: Party38ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequestedIndicator {
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
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentInstrument1Code {
    #[serde(rename = "BDT")]
    Bdt,
    #[serde(rename = "BCT")]
    Bct,
    #[serde(rename = "CDT")]
    Cdt,
    #[serde(rename = "CCT")]
    Cct,
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "BKT")]
    Bkt,
    #[serde(rename = "DCP")]
    Dcp,
    #[serde(rename = "CCP")]
    Ccp,
    #[serde(rename = "RTI")]
    Rti,
    #[serde(rename = "CAN")]
    Can,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionReturnCriteria5 {
    #[serde(rename = "PmtToRtrCrit", skip_serializing_if = "Option::is_none")]
    pub pmt_to_rtr_crit: Option<SystemReturnCriteria2>,
    #[serde(rename = "PmtFrRtrCrit", skip_serializing_if = "Option::is_none")]
    pub pmt_fr_rtr_crit: Option<SystemReturnCriteria2>,
    #[serde(rename = "AcctCshNtryRtrCrit", skip_serializing_if = "Option::is_none")]
    pub acct_csh_ntry_rtr_crit: Option<AccountCashEntryReturnCriteria3>,
    #[serde(rename = "PmtRtrCrit", skip_serializing_if = "Option::is_none")]
    pub pmt_rtr_crit: Option<PaymentReturnCriteria4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalOrganisationIdentification1Code {
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
pub struct GenericOrganisationIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
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
pub struct AccountCashEntryReturnCriteria3 {
    #[serde(rename = "NtryRefInd", skip_serializing_if = "Option::is_none")]
    pub ntry_ref_ind: Option<RequestedIndicator>,
    #[serde(rename = "AcctTpInd", skip_serializing_if = "Option::is_none")]
    pub acct_tp_ind: Option<RequestedIndicator>,
    #[serde(rename = "NtryAmtInd", skip_serializing_if = "Option::is_none")]
    pub ntry_amt_ind: Option<RequestedIndicator>,
    #[serde(rename = "AcctCcyInd", skip_serializing_if = "Option::is_none")]
    pub acct_ccy_ind: Option<RequestedIndicator>,
    #[serde(rename = "NtryStsInd", skip_serializing_if = "Option::is_none")]
    pub ntry_sts_ind: Option<RequestedIndicator>,
    #[serde(rename = "NtryDtInd", skip_serializing_if = "Option::is_none")]
    pub ntry_dt_ind: Option<RequestedIndicator>,
    #[serde(rename = "AcctSvcrInd", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ind: Option<RequestedIndicator>,
    #[serde(rename = "AcctOwnrInd", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_ind: Option<RequestedIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
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
pub struct AccountIdentification4Choice {
    #[serde(flatten)]
    pub value: AccountIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPersonIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: PersonIdentificationSchemeName1ChoiceEnum,
}
#[derive(
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
pub struct LongPaymentIdentification2 {
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<Max35Text>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<UuiDv4Identifier>,
    #[validate]
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: IsoDate,
    #[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
    pub pmt_mtd: Option<PaymentOrigin1Choice>,
    #[validate]
    #[serde(rename = "InstgAgt")]
    pub instg_agt: BranchAndFinancialInstitutionIdentification6,
    #[validate]
    #[serde(rename = "InstdAgt")]
    pub instd_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "NtryTp", skip_serializing_if = "Option::is_none")]
    pub ntry_tp: Option<EntryTypeIdentifier>,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<Max35Text>,
}
#[derive(
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
pub struct AddressType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddressType3Choice {
    #[serde(flatten)]
    pub value: AddressType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionQuery6 {
    #[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<QueryType2Code>,
    #[serde(rename = "TxCrit", skip_serializing_if = "Option::is_none")]
    pub tx_crit: Option<TransactionCriteria6Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentificationSearchCriteria2ChoiceEnum {
    #[serde(rename = "CTTxt", skip_serializing_if = "Option::is_none")]
    pub ct_txt: Option<Max35Text>,
    #[serde(rename = "NCTTxt", skip_serializing_if = "Option::is_none")]
    pub nct_txt: Option<Max35Text>,
    #[serde(rename = "EQ", skip_serializing_if = "Option::is_none")]
    pub eq: Option<AccountIdentification4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentificationSearchCriteria2Choice {
    #[serde(flatten)]
    pub value: AccountIdentificationSearchCriteria2ChoiceEnum,
}
#[derive(
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
pub struct ExternalFinancialInstitutionIdentification1Code {
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
pub struct UuiDv4Identifier {
    #[validate(regex = "UUI_DV_4_IDENTIFIER_REGEX")]
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
pub struct ImpliedCurrencyAndAmountRange1 {
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAmountRange1Choice,
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
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QueryType2Code {
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "CHNG")]
    Chng,
    #[serde(rename = "MODF")]
    Modf,
    #[serde(rename = "DELD")]
    Deld,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MessageHeader9 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
    pub req_tp: Option<RequestType4Choice>,
}
#[derive(
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
pub struct GetTransactionV09<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "MsgHdr")]
    pub msg_hdr: MessageHeader9,
    #[serde(rename = "TxQryDef", skip_serializing_if = "Option::is_none")]
    pub tx_qry_def: Option<TransactionQuery6>,
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
pub struct DatePeriodSearch1ChoiceEnum {
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<IsoDate>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DatePeriod2>,
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[serde(rename = "EQDt", skip_serializing_if = "Option::is_none")]
    pub eq_dt: Option<IsoDate>,
    #[serde(rename = "NEQDt", skip_serializing_if = "Option::is_none")]
    pub neq_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DatePeriodSearch1Choice {
    #[serde(flatten)]
    pub value: DatePeriodSearch1ChoiceEnum,
}
