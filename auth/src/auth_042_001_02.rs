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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.042.001.02".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType20Code {
    #[serde(rename = "DIRY")]
    Diry,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentIdentification5ChoiceEnum {
    #[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
    pub bskt: Option<FinancialInstrument53>,
    #[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
    pub sngl: Option<FinancialInstrument48Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentIdentification5Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentIdentification5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType25Code {
    #[serde(rename = "DIST")]
    Dist,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityAmmonia1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType39Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType5Code {
    #[serde(rename = "BSLD")]
    Bsld,
    #[serde(rename = "FITR")]
    Fitr,
    #[serde(rename = "PKLD")]
    Pkld,
    #[serde(rename = "OFFP")]
    Offp,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType40Code {
    #[serde(rename = "DAPH")]
    Daph,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityForestry1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType21Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType15Code {
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
pub struct AgriculturalCommodityDairy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType20Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName5ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName5Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityInterEnergy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType26Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceStatus1Code {
    #[serde(rename = "PNDG")]
    Pndg,
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityLiveStock1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType22Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType36Code {
    #[serde(rename = "NSPT")]
    Nspt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityContainerBoard1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType35Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType46Code {
    #[serde(rename = "CSHP")]
    Cshp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType44Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType26Code {
    #[serde(rename = "INRG")]
    Inrg,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentReportingInvalidReferenceDataReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DtPrd")]
    pub dt_prd: Period4Choice,
    #[serde(rename = "NbOfRcrds", skip_serializing_if = "Option::is_none")]
    pub nb_of_rcrds: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "FinInstrms", default)]
    pub fin_instrms: Vec<SecuritiesInvalidReferenceDataReport4<A>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
}
#[derive(
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
pub struct AgriculturalCommodityGrain1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType5Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType15Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnergy1ChoiceEnum {
    #[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
    pub ntrl_gas: Option<EnergyCommodityNaturalGas1>,
    #[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
    pub elctrcty: Option<EnergyCommodityElectricity1>,
    #[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
    pub lght_end: Option<EnergyCommodityLightEnd1>,
    #[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
    pub coal: Option<EnergyCommodityCoal1>,
    #[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
    pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
    #[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
    pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
    #[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
    pub dstllts: Option<EnergyCommodityDistillates1>,
    #[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
    pub oil: Option<EnergyCommodityOil1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnergy1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnergy1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType1Code {
    #[serde(rename = "AGRI")]
    Agri,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType37Code {
    #[serde(rename = "PULP")]
    Pulp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FloatingInterestRate8 {
    #[serde(rename = "RefRate")]
    pub ref_rate: BenchmarkCurveName5Choice,
    #[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
    pub term: Option<InterestRateContractTerm2>,
}
#[derive(
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
pub enum AssetClassDetailedSubProductType15Code {
    #[serde(rename = "MWHT")]
    Mwht,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMetal1ChoiceEnum {
    #[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
    pub prcs: Option<MetalCommodityPrecious1>,
    #[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
    pub non_prcs: Option<MetalCommodityNonPrecious1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMetal1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityMetal1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType3Code {
    #[serde(rename = "ENVR")]
    Envr,
    #[default]
    Unknown,
}
#[derive(
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
pub struct AmountAndDirection61 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
}
#[derive(
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
pub struct AssetClassCommodityFertilizer1ChoiceEnum {
    #[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
    pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
    #[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
    pub ammn: Option<FertilizerCommodityAmmonia1>,
    #[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
    pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
    #[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
    pub ptsh: Option<FertilizerCommodityPotash1>,
    #[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
    pub slphr: Option<FertilizerCommoditySulphur1>,
    #[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
    pub urea: Option<FertilizerCommodityUrea1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFertilizer1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFertilizer1ChoiceEnum,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType30Code {
    #[serde(rename = "WTHR")]
    Wthr,
    #[default]
    Unknown,
}
#[derive(
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
pub enum OptionType2Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
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
pub struct AssetClassCommodityOtherC102ChoiceEnum {
    #[serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none")]
    pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
    #[serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none")]
    pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOtherC102Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityOtherC102ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeInterest3 {
    #[validate]
    #[serde(rename = "IntrstRate")]
    pub intrst_rate: FloatingInterestRate8,
    #[serde(rename = "FrstLegIntrstRate", skip_serializing_if = "Option::is_none")]
    pub frst_leg_intrst_rate: Option<InterestRate8Choice>,
    #[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
    pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "OthrLegIntrstRate", skip_serializing_if = "Option::is_none")]
    pub othr_leg_intrst_rate: Option<InterestRate8Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument48ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<FinancialInstrument58>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct FinancialInstrument48Choice {
    #[serde(flatten)]
    pub value: FinancialInstrument48ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityNewsprint1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType36Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPolypropylene1ChoiceEnum {
    #[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
    pub plstc: Option<PolypropyleneCommodityPlastic1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPolypropylene1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPolypropylene1ChoiceEnum,
}
#[derive(
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
pub struct Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType22Code {
    #[serde(rename = "LSTK")]
    Lstk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityOliveOil1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType3Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType4Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType5Code {
    #[serde(rename = "GRIN")]
    Grin,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityWeather1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType30Code,
}
#[derive(
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
pub struct SecurityInstrumentDescription9 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: IsinOct2015Identifier,
    #[validate]
    #[serde(rename = "FullNm")]
    pub full_nm: Max350Text,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "ClssfctnTp")]
    pub clssfctn_tp: CfiOct2015Identifier,
    #[serde(rename = "NtnlCcy")]
    pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
    #[validate]
    #[serde(rename = "CmmdtyDerivInd")]
    pub cmmdty_deriv_ind: TrueFalseIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max5Number {
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
pub struct FloatingInterestRate6 {
    #[serde(rename = "RefRate")]
    pub ref_rate: BenchmarkCurveName6Choice,
    #[validate]
    #[serde(rename = "Term")]
    pub term: InterestRateContractTerm2,
    #[validate]
    #[serde(rename = "BsisPtSprd")]
    pub bsis_pt_sprd: Max5Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodity3ChoiceEnum {
    #[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
    pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
    #[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
    pub infltn: Option<AssetClassCommodityInflation1>,
    #[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
    pub agrcltrl: Option<AssetClassCommodityAgricultural1Choice>,
    #[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
    pub frtlzr: Option<AssetClassCommodityFertilizer1Choice>,
    #[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
    pub ppr: Option<AssetClassCommodityPaper1Choice>,
    #[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
    pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
    #[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
    pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
    #[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
    pub frght: Option<AssetClassCommodityFreight1Choice>,
    #[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
    pub metl: Option<AssetClassCommodityMetal1Choice>,
    #[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
    pub plprpln: Option<AssetClassCommodityPolypropylene1Choice>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<AssetClassCommodityOther1>,
    #[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
    pub nrgy: Option<AssetClassCommodityEnergy1Choice>,
    #[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
    pub envttl: Option<AssetClassCommodityEnvironmental1Choice>,
    #[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
    pub othr_c_10: Option<AssetClassCommodityOtherC102Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodity3Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodity3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherC10CommodityNonDeliverable2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType11Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType48Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName6ChoiceEnum {
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
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
pub struct BenchmarkCurveName6Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityDry1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType31Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType14Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType42Code {
    #[serde(rename = "SLPH")]
    Slph,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityIndustrialProduct1ChoiceEnum {
    #[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
    pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
    #[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
    pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityIndustrialProduct1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeInstrument5 {
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[serde(rename = "PricMltplr", skip_serializing_if = "Option::is_none")]
    pub pric_mltplr: Option<NonNegativeDecimalNumber>,
    #[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
    pub undrlyg_instrm: Option<FinancialInstrumentIdentification5Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType2Code>,
    #[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
    pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
    #[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
    pub optn_exrc_style: Option<OptionStyle7Code>,
    #[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
    pub dlvry_tp: Option<PhysicalTransferType4Code>,
    #[serde(
        rename = "AsstClssSpcfcAttrbts",
        skip_serializing_if = "Option::is_none"
    )]
    pub asst_clss_spcfc_attrbts: Option<AssetClass2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType7Code {
    #[serde(rename = "BAKK")]
    Bakk,
    #[serde(rename = "BDSL")]
    Bdsl,
    #[serde(rename = "BRNT")]
    Brnt,
    #[serde(rename = "BRNX")]
    Brnx,
    #[serde(rename = "CNDA")]
    Cnda,
    #[serde(rename = "COND")]
    Cond,
    #[serde(rename = "DSEL")]
    Dsel,
    #[serde(rename = "DUBA")]
    Duba,
    #[serde(rename = "ESPO")]
    Espo,
    #[serde(rename = "ETHA")]
    Etha,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "FOIL")]
    Foil,
    #[serde(rename = "GOIL")]
    Goil,
    #[serde(rename = "GSLN")]
    Gsln,
    #[serde(rename = "HEAT")]
    Heat,
    #[serde(rename = "JTFL")]
    Jtfl,
    #[serde(rename = "KERO")]
    Kero,
    #[serde(rename = "LLSO")]
    Llso,
    #[serde(rename = "MARS")]
    Mars,
    #[serde(rename = "NAPH")]
    Naph,
    #[serde(rename = "NGLO")]
    Nglo,
    #[serde(rename = "TAPI")]
    Tapi,
    #[serde(rename = "WTIO")]
    Wtio,
    #[serde(rename = "URAL")]
    Ural,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFreight1ChoiceEnum {
    #[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
    pub wet: Option<FreightCommodityWet1>,
    #[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
    pub cntnr_ship: Option<FreightCommodityContainerShip1>,
    #[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
    pub dry: Option<FreightCommodityDry1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityFreight1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityFreight1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DebtInstrument2 {
    #[validate]
    #[serde(rename = "TtlIssdNmnlAmt")]
    pub ttl_issd_nmnl_amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "NmnlValPerUnit")]
    pub nmnl_val_per_unit: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "IntrstRate")]
    pub intrst_rate: InterestRate6Choice,
    #[serde(rename = "DebtSnrty", skip_serializing_if = "Option::is_none")]
    pub debt_snrty: Option<DebtInstrumentSeniorityType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingVenueAttributes1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: MicIdentifier,
    #[validate]
    #[serde(rename = "IssrReq")]
    pub issr_req: TrueFalseIndicator,
    #[serde(
        rename = "AdmssnApprvlDtByIssr",
        skip_serializing_if = "Option::is_none"
    )]
    pub admssn_apprvl_dt_by_issr: Option<IsoDateTime>,
    #[serde(rename = "ReqForAdmssnDt", skip_serializing_if = "Option::is_none")]
    pub req_for_admssn_dt: Option<IsoDateTime>,
    #[serde(rename = "FrstTradDt", skip_serializing_if = "Option::is_none")]
    pub frst_trad_dt: Option<IsoDateTime>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeForeignExchange3 {
    #[serde(rename = "FxTp", skip_serializing_if = "Option::is_none")]
    pub fx_tp: Option<AssetFxSubProductType1Code>,
    #[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
    pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType29Code {
    #[serde(rename = "CRBR")]
    Crbr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType41Code {
    #[serde(rename = "PTSH")]
    Ptsh,
    #[default]
    Unknown,
}
#[derive(
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
pub struct Period2 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType7Code {
    #[serde(rename = "NGAS")]
    Ngas,
    #[default]
    Unknown,
}
#[derive(
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
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType14Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType45Code {
    #[serde(rename = "POTA")]
    Pota,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetPriceType1Code {
    #[serde(rename = "ARGM")]
    Argm,
    #[serde(rename = "BLTC")]
    Bltc,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "GBCL")]
    Gbcl,
    #[serde(rename = "IHSM")]
    Ihsm,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PLAT")]
    Plat,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityCoal1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType24Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType31Code {
    #[serde(rename = "DRYF")]
    Dryf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityDistillates1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType25Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityInflation1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType12Code,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType21Code {
    #[serde(rename = "FRST")]
    Frst,
    #[default]
    Unknown,
}
#[derive(
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
pub struct AssetClassCommodityAgricultural1ChoiceEnum {
    #[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
    pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
    #[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
    pub grn: Option<AgriculturalCommodityGrain1>,
    #[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
    pub sfd: Option<AgriculturalCommoditySeafood1>,
    #[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
    pub ptt: Option<AgriculturalCommodityPotato1>,
    #[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
    pub olv_oil: Option<AgriculturalCommodityOliveOil1>,
    #[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
    pub live_stock: Option<AgriculturalCommodityLiveStock1>,
    #[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
    pub dairy: Option<AgriculturalCommodityDairy1>,
    #[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
    pub soft: Option<AgriculturalCommoditySoft1>,
    #[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
    pub frstry: Option<AgriculturalCommodityForestry1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityAgricultural1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityAgricultural1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativeCommodity2 {
    #[serde(rename = "Pdct")]
    pub pdct: AssetClassCommodity3Choice,
    #[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
    pub tx_tp: Option<AssetClassTransactionType1Code>,
    #[serde(rename = "FnlPricTp", skip_serializing_if = "Option::is_none")]
    pub fnl_pric_tp: Option<AssetPriceType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NonNegativeDecimalNumber {
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
pub struct OtherC10CommodityDeliverable2 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType11Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType47Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RecordTechnicalData4 {
    #[serde(rename = "IncnsstncyInd", skip_serializing_if = "Option::is_none")]
    pub incnsstncy_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "LastUpd", skip_serializing_if = "Option::is_none")]
    pub last_upd: Option<IsoDateTime>,
    #[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
    pub submissn_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "RlvntCmptntAuthrty", skip_serializing_if = "Option::is_none")]
    pub rlvnt_cmptnt_authrty: Option<CountryCode>,
    #[serde(rename = "PblctnPrd", skip_serializing_if = "Option::is_none")]
    pub pblctn_prd: Option<Period4Choice>,
    #[serde(rename = "NvrPblshd", skip_serializing_if = "Option::is_none")]
    pub nvr_pblshd: Option<TrueFalseIndicator>,
    #[serde(rename = "RlvntTradgVn", skip_serializing_if = "Option::is_none")]
    pub rlvnt_tradg_vn: Option<MicIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType7Code {
    #[serde(rename = "METL")]
    Metl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice1 {
    #[serde(rename = "Pdg")]
    pub pdg: PriceStatus1Code,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType14Code {
    #[serde(rename = "DBCR")]
    Dbcr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityPotato1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType45Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType11Code {
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PLDM")]
    Pldm,
    #[serde(rename = "PTNM")]
    Ptnm,
    #[serde(rename = "SLVR")]
    Slvr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType6Code {
    #[serde(rename = "ELEC")]
    Elec,
    #[default]
    Unknown,
}
#[derive(
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
pub enum AssetClassSubProductType32Code {
    #[serde(rename = "WETF")]
    Wetf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate6ChoiceEnum {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<PercentageRate>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<FloatingInterestRate6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate6Choice {
    #[serde(flatten)]
    pub value: InterestRate6ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType33Code {
    #[serde(rename = "CSTR")]
    Cstr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityNaturalGas1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType7Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesInvalidReferenceDataReport4<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FinInstrm")]
    pub fin_instrm: SecuritiesReferenceDataReport6,
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
pub struct EnergyCommodityLightEnd1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType27Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType24Code {
    #[serde(rename = "COAL")]
    Coal,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice2ChoiceEnum {
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection61>,
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
    pub bsis_pts: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice2Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice4ChoiceEnum {
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<SecuritiesTransactionPrice2Choice>,
    #[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
    pub no_pric: Option<SecuritiesTransactionPrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice4Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType28Code {
    #[serde(rename = "RNNG")]
    Rnng,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType8Code {
    #[serde(rename = "PAPR")]
    Papr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType18Code {
    #[serde(rename = "PLST")]
    Plst,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType6Code {
    #[serde(rename = "INDP")]
    Indp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType6Code {
    #[serde(rename = "GASP")]
    Gasp,
    #[serde(rename = "LNGG")]
    Lngg,
    #[serde(rename = "NCGG")]
    Ncgg,
    #[serde(rename = "TTFG")]
    Ttfg,
    #[serde(rename = "NBPG")]
    Nbpg,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityContainerShip1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType46Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType10Code {
    #[serde(rename = "ALUM")]
    Alum,
    #[serde(rename = "ALUA")]
    Alua,
    #[serde(rename = "CBLT")]
    Cblt,
    #[serde(rename = "COPR")]
    Copr,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "MOLY")]
    Moly,
    #[serde(rename = "NASC")]
    Nasc,
    #[serde(rename = "NICK")]
    Nick,
    #[serde(rename = "STEL")]
    Stel,
    #[serde(rename = "TINN")]
    Tinn,
    #[serde(rename = "ZINC")]
    Zinc,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "LEAD")]
    Lead,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommoditySeafood1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType23Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityRenewableEnergy1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType28Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType23Code {
    #[serde(rename = "SEAF")]
    Seaf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType40Code,
}
#[derive(
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
pub enum AssetClassProductType9Code {
    #[serde(rename = "POLY")]
    Poly,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType38Code {
    #[serde(rename = "RCVP")]
    Rcvp,
    #[default]
    Unknown,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DebtInstrumentSeniorityType1Code {
    #[serde(rename = "SBOD")]
    Sbod,
    #[serde(rename = "SNDB")]
    Sndb,
    #[serde(rename = "MZZD")]
    Mzzd,
    #[serde(rename = "JUND")]
    Jund,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PolypropyleneCommodityPlastic1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType9Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType18Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityCarbonRelated1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType29Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityPulp1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType37Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType35Code {
    #[serde(rename = "CBRD")]
    Cbrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType2Code {
    #[serde(rename = "SOFT")]
    Soft,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max25Text {
    #[validate(length(min = 1, max = 25,))]
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
pub struct AgriculturalCommoditySoft1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType2Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType2Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType44Code {
    #[serde(rename = "UAAN")]
    Uaan,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType4Code {
    #[serde(rename = "FRGT")]
    Frgt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgriculturalCommodityOilSeed1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType1Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType1Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType1Code,
}
#[derive(
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
pub enum AssetClassDetailedSubProductType2Code {
    #[serde(rename = "ROBU")]
    Robu,
    #[serde(rename = "CCOA")]
    Ccoa,
    #[serde(rename = "BRWN")]
    Brwn,
    #[serde(rename = "WHSG")]
    Whsg,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType4Code {
    #[serde(rename = "LAMP")]
    Lamp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType15Code {
    #[serde(rename = "NPRM")]
    Nprm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetFxSubProductType1Code {
    #[serde(rename = "FXCR")]
    Fxcr,
    #[serde(rename = "FXEM")]
    Fxem,
    #[serde(rename = "FXMJ")]
    Fxmj,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassTransactionType1Code {
    #[serde(rename = "CRCK")]
    Crck,
    #[serde(rename = "DIFF")]
    Diff,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "MINI")]
    Mini,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "OTCT")]
    Otct,
    #[serde(rename = "ORIT")]
    Orit,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TAPO")]
    Tapo,
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType8Code {
    #[serde(rename = "OILP")]
    Oilp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType11Code {
    #[serde(rename = "OTHC")]
    Othc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType34Code {
    #[serde(rename = "MFTG")]
    Mftg,
    #[default]
    Unknown,
}
#[derive(
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
pub enum AssetClassSubProductType27Code {
    #[serde(rename = "LGHT")]
    Lght,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType12Code {
    #[serde(rename = "INFL")]
    Infl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndustrialProductCommodityManufacturing1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType34Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRateContractTerm2 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max3Number,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType10Code {
    #[serde(rename = "EMIS")]
    Emis,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType16Code {
    #[serde(rename = "PRME")]
    Prme,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType3Code {
    #[serde(rename = "OOLI")]
    Ooli,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MetalCommodityPrecious1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType16Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType11Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateBasis1Code {
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType1Code {
    #[serde(rename = "GROS")]
    Gros,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPaper1ChoiceEnum {
    #[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
    pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
    #[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
    pub nwsprnt: Option<PaperCommodityNewsprint1>,
    #[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
    pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
    #[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
    pub pulp: Option<PaperCommodityPulp1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityPaper1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityPaper1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType8Code {
    #[serde(rename = "CERE")]
    Cere,
    #[serde(rename = "ERUE")]
    Erue,
    #[serde(rename = "EUAE")]
    Euae,
    #[serde(rename = "EUAA")]
    Euaa,
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
pub struct Period4ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<IsoDate>,
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt_to_dt: Option<Period2>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType47Code {
    #[serde(rename = "DLVR")]
    Dlvr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType13Code {
    #[serde(rename = "MCEX")]
    Mcex,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityUrea1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType43Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType14Code {
    #[serde(rename = "OEST")]
    Oest,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityOther1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType15Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType5Code {
    #[serde(rename = "FRTL")]
    Frtl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnvironmentalCommodityEmission1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType3Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType10Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EnergyCommodityOil1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType8Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType7Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType48Code {
    #[serde(rename = "NDLV")]
    Ndlv,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesReferenceDataReport6 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "FinInstrmGnlAttrbts")]
    pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription9,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: LeiIdentifier,
    #[validate(length(min = 1,))]
    #[serde(rename = "TradgVnRltdAttrbts", default)]
    pub tradg_vn_rltd_attrbts: Vec<TradingVenueAttributes1>,
    #[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub debt_instrm_attrbts: Option<DebtInstrument2>,
    #[serde(rename = "DerivInstrmAttrbts", skip_serializing_if = "Option::is_none")]
    pub deriv_instrm_attrbts: Option<DerivativeInstrument5>,
    #[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
    pub tech_attrbts: Option<RecordTechnicalData4>,
}
#[derive(
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
pub struct IndustrialProductCommodityConstruction1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType6Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType33Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate8ChoiceEnum {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<PercentageRate>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<FloatingInterestRate8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InterestRate8Choice {
    #[serde(flatten)]
    pub value: InterestRate8ChoiceEnum,
}
#[derive(
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
pub struct TrueFalseIndicator {
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
pub struct EnergyCommodityElectricity1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType2Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType6Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType5Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument58 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: FloatingInterestRate8,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClass2 {
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<DerivativeCommodity2>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<DerivativeInterest3>,
    #[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
    pub fx: Option<DerivativeForeignExchange3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnvironmental1ChoiceEnum {
    #[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
    pub emssns: Option<EnvironmentalCommodityEmission1>,
    #[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
    pub wthr: Option<EnvironmentalCommodityWeather1>,
    #[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
    pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityEnvironmental1Choice {
    #[serde(flatten)]
    pub value: AssetClassCommodityEnvironmental1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaperCommodityRecoveredPaper1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType8Code,
    #[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
    pub sub_pdct: Option<AssetClassSubProductType38Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FreightCommodityWet1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType4Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType32Code,
    #[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
    pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType12Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType12Code {
    #[serde(rename = "TNKR")]
    Tnkr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassDetailedSubProductType1Code {
    #[serde(rename = "FWHT")]
    Fwht,
    #[serde(rename = "SOYB")]
    Soyb,
    #[serde(rename = "RPSD")]
    Rpsd,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CORN")]
    Corn,
    #[serde(rename = "RICE")]
    Rice,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommoditySulphur1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType42Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FertilizerCommodityPotash1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType5Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType41Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassProductType2Code {
    #[serde(rename = "NRGY")]
    Nrgy,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType39Code {
    #[serde(rename = "AMMO")]
    Ammo,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MetalCommodityNonPrecious1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType7Code,
    #[serde(rename = "SubPdct")]
    pub sub_pdct: AssetClassSubProductType15Code,
    #[serde(rename = "AddtlSubPdct")]
    pub addtl_sub_pdct: AssetClassDetailedSubProductType10Code,
}
#[derive(
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
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FinInstrmRptgInvldRefDataRpt")]
    pub fin_instrm_rptg_invld_ref_data_rpt:
        FinancialInstrumentReportingInvalidReferenceDataReportV02<A, B>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName2Code {
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "BBSW")]
    Bbsw,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
    #[serde(rename = "BasePdct")]
    pub base_pdct: AssetClassProductType13Code,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle7Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "BERM")]
    Berm,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssetClassSubProductType43Code {
    #[serde(rename = "UREA")]
    Urea,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument53 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ISIN", default)]
    pub isin: Vec<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LEI", default)]
    pub lei: Vec<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PhysicalTransferType4Code {
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "OPTL")]
    Optl,
    #[serde(rename = "CASH")]
    Cash,
    #[default]
    Unknown,
}
