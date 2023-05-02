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
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the “License”);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an “AS IS” BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalChannel1Code {
    #[serde(rename = "COUR")]
    Cour,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "MESS")]
    Mess,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "REGM")]
    Regm,
    #[serde(rename = "SEMA")]
    Sema,
    #[serde(rename = "SWFA")]
    Swfa,
    #[serde(rename = "SWIA")]
    Swia,
    #[serde(rename = "SWMT")]
    Swmt,
    #[serde(rename = "SWMX")]
    Swmx,
    #[serde(rename = "TELE")]
    Tele,
    #[serde(rename = "WEBM")]
    Webm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCancellationReason1Code {
    #[serde(rename = "AC03")]
    Ac03,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "AM09")]
    Am09,
    #[serde(rename = "BE16")]
    Be16,
    #[serde(rename = "COVR")]
    Covr,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "CUTA")]
    Cuta,
    #[serde(rename = "DS24")]
    Ds24,
    #[serde(rename = "DT01")]
    Dt01,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "FRAD")]
    Frad,
    #[serde(rename = "FRNA")]
    Frna,
    #[serde(rename = "FRTR")]
    Frtr,
    #[serde(rename = "INDM")]
    Indm,
    #[serde(rename = "MODT")]
    Modt,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "SVNR")]
    Svnr,
    #[serde(rename = "SYAD")]
    Syad,
    #[serde(rename = "TECH")]
    Tech,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "ENUE")]
    Enue,
    #[serde(rename = "UAPA")]
    Uapa,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "AC02")]
    Ac02,
    #[serde(rename = "BIAS")]
    Bias,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "DRTP")]
    Drtp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCreditorEnrolmentStatusReason1Code {
    #[serde(rename = "ACRD")]
    Acrd,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "FF01")]
    Ff01,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "RR10")]
    Rr10,
    #[serde(rename = "TRJT")]
    Trjt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalRelativeTo1Code {
    #[serde(rename = "GOOD")]
    Good,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "UNDG")]
    Undg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAgentInstruction1Code {
    #[serde(rename = "CHQB")]
    Chqb,
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "INQR")]
    Inqr,
    #[serde(rename = "PBEN")]
    Pben,
    #[serde(rename = "PHOA")]
    Phoa,
    #[serde(rename = "PHOB")]
    Phob,
    #[serde(rename = "TELA")]
    Tela,
    #[serde(rename = "TELB")]
    Telb,
    #[serde(rename = "TFRO")]
    Tfro,
    #[serde(rename = "TTIL")]
    Ttil,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingType1Code {
    #[serde(rename = "APAY")]
    Apay,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DPAY")]
    Dpay,
    #[serde(rename = "FINC")]
    Finc,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "IRBO")]
    Irbo,
    #[serde(rename = "MAIN")]
    Main,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "PERF")]
    Perf,
    #[serde(rename = "RETN")]
    Retn,
    #[serde(rename = "SHIP")]
    Ship,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "WARR")]
    Warr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBenchmarkCurveName1Code {
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "BBSW")]
    Bbsw,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "ESTR")]
    Estr,
    #[serde(rename = "SOFR")]
    Sofr,
    #[serde(rename = "SONA")]
    Sona,
    #[serde(rename = "RCTR")]
    Rctr,
    #[serde(rename = "CORA")]
    Cora,
    #[serde(rename = "BCOL")]
    Bcol,
    #[serde(rename = "HKIO")]
    Hkio,
    #[serde(rename = "BJUO")]
    Bjuo,
    #[serde(rename = "ETIO")]
    Etio,
    #[serde(rename = "EFFR")]
    Effr,
    #[serde(rename = "OBFR")]
    Obfr,
    #[serde(rename = "CZNA")]
    Czna,
    #[serde(rename = "TONA")]
    Tona,
    #[serde(rename = "TORF")]
    Torf,
    #[serde(rename = "SIBO")]
    Sibo,
    #[serde(rename = "SSOR")]
    Ssor,
    #[serde(rename = "SORA")]
    Sora,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalMarketInfrastructure1Code {
    #[serde(rename = "ABE")]
    Abe,
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "ACS")]
    Acs,
    #[serde(rename = "AIP")]
    Aip,
    #[serde(rename = "ART")]
    Art,
    #[serde(rename = "AVP")]
    Avp,
    #[serde(rename = "AZM")]
    Azm,
    #[serde(rename = "BAP")]
    Bap,
    #[serde(rename = "BCC")]
    Bcc,
    #[serde(rename = "BCE")]
    Bce,
    #[serde(rename = "BDS")]
    Bds,
    #[serde(rename = "BEL")]
    Bel,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHS")]
    Bhs,
    #[serde(rename = "BIS")]
    Bis,
    #[serde(rename = "BOF")]
    Bof,
    #[serde(rename = "BOJ")]
    Boj,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSP")]
    Bsp,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CAM")]
    Cam,
    #[serde(rename = "CBA")]
    Cba,
    #[serde(rename = "CBJ")]
    Cbj,
    #[serde(rename = "CHI")]
    Chi,
    #[serde(rename = "CHP")]
    Chp,
    #[serde(rename = "CIS")]
    Cis,
    #[serde(rename = "CLM")]
    Clm,
    #[serde(rename = "COE")]
    Coe,
    #[serde(rename = "COI")]
    Coi,
    #[serde(rename = "COU")]
    Cou,
    #[serde(rename = "DDK")]
    Ddk,
    #[serde(rename = "DKC")]
    Dkc,
    #[serde(rename = "EBA")]
    Eba,
    #[serde(rename = "ELS")]
    Els,
    #[serde(rename = "EMZ")]
    Emz,
    #[serde(rename = "EPM")]
    Epm,
    #[serde(rename = "EPN")]
    Epn,
    #[serde(rename = "ERP")]
    Erp,
    #[serde(rename = "FDA")]
    Fda,
    #[serde(rename = "FDN")]
    Fdn,
    #[serde(rename = "FDW")]
    Fdw,
    #[serde(rename = "FEY")]
    Fey,
    #[serde(rename = "GIS")]
    Gis,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HRM")]
    Hrm,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "INC")]
    Inc,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "KPS")]
    Kps,
    #[serde(rename = "LGS")]
    Lgs,
    #[serde(rename = "LKB")]
    Lkb,
    #[serde(rename = "LVL")]
    Lvl,
    #[serde(rename = "LVT")]
    Lvt,
    #[serde(rename = "MEP")]
    Mep,
    #[serde(rename = "MOS")]
    Mos,
    #[serde(rename = "MRS")]
    Mrs,
    #[serde(rename = "MUP")]
    Mup,
    #[serde(rename = "NAM")]
    Nam,
    #[serde(rename = "NOC")]
    Noc,
    #[serde(rename = "NPP")]
    Npp,
    #[serde(rename = "PCH")]
    Pch,
    #[serde(rename = "PDS")]
    Pds,
    #[serde(rename = "PEG")]
    Peg,
    #[serde(rename = "PNS")]
    Pns,
    #[serde(rename = "PTR")]
    Ptr,
    #[serde(rename = "PVE")]
    Pve,
    #[serde(rename = "ROL")]
    Rol,
    #[serde(rename = "ROS")]
    Ros,
    #[serde(rename = "RTG")]
    Rtg,
    #[serde(rename = "RTP")]
    Rtp,
    #[serde(rename = "SCL")]
    Scl,
    #[serde(rename = "SCP")]
    Scp,
    #[serde(rename = "SEC")]
    Sec,
    #[serde(rename = "SEU")]
    Seu,
    #[serde(rename = "SIC")]
    Sic,
    #[serde(rename = "SIP")]
    Sip,
    #[serde(rename = "SIT")]
    Sit,
    #[serde(rename = "SLB")]
    Slb,
    #[serde(rename = "SPG")]
    Spg,
    #[serde(rename = "SSK")]
    Ssk,
    #[serde(rename = "ST2")]
    St2,
    #[serde(rename = "STG")]
    Stg,
    #[serde(rename = "T2S")]
    T2S,
    #[serde(rename = "TBF")]
    Tbf,
    #[serde(rename = "TCH")]
    Tch,
    #[serde(rename = "TGT")]
    Tgt,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "THN")]
    Thn,
    #[serde(rename = "TIS")]
    Tis,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "UBE")]
    Ube,
    #[serde(rename = "UIS")]
    Uis,
    #[serde(rename = "VCS")]
    Vcs,
    #[serde(rename = "XCT")]
    Xct,
    #[serde(rename = "ZEN")]
    Zen,
    #[serde(rename = "ZET")]
    Zet,
    #[serde(rename = "ZIS")]
    Zis,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalClearingSystemMember1Code {
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalMandateStatus1Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "SUSP")]
    Susp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCreditorEnrolmentCancellationReason1Code {
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "TRJT")]
    Trjt,
    #[serde(rename = "UCRD")]
    Ucrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAgreementType1Code {
    #[serde(rename = "AUSL")]
    Ausl,
    #[serde(rename = "BIAG")]
    Biag,
    #[serde(rename = "CARA")]
    Cara,
    #[serde(rename = "CDEA")]
    Cdea,
    #[serde(rename = "CHMA")]
    Chma,
    #[serde(rename = "CHRA")]
    Chra,
    #[serde(rename = "CMOP")]
    Cmop,
    #[serde(rename = "CNBR")]
    Cnbr,
    #[serde(rename = "CSDA")]
    Csda,
    #[serde(rename = "DEMA")]
    Dema,
    #[serde(rename = "DERD")]
    Derd,
    #[serde(rename = "DERP")]
    Derp,
    #[serde(rename = "DERV")]
    Derv,
    #[serde(rename = "EFMA")]
    Efma,
    #[serde(rename = "ESRA")]
    Esra,
    #[serde(rename = "EUMA")]
    Euma,
    #[serde(rename = "FMAT")]
    Fmat,
    #[serde(rename = "FPCA")]
    Fpca,
    #[serde(rename = "FRFB")]
    Frfb,
    #[serde(rename = "GESL")]
    Gesl,
    #[serde(rename = "GMRA")]
    Gmra,
    #[serde(rename = "GMSL")]
    Gmsl,
    #[serde(rename = "IDMA")]
    Idma,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "JPBL")]
    Jpbl,
    #[serde(rename = "JPBR")]
    Jpbr,
    #[serde(rename = "JPSL")]
    Jpsl,
    #[serde(rename = "KRRA")]
    Krra,
    #[serde(rename = "KRSL")]
    Krsl,
    #[serde(rename = "MEFI")]
    Mefi,
    #[serde(rename = "MRAA")]
    Mraa,
    #[serde(rename = "MSLA")]
    Msla,
    #[serde(rename = "OSLA")]
    Osla,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalTechnicalInputChannel1Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "PAPR")]
    Papr,
    #[serde(rename = "TAPE")]
    Tape,
    #[serde(rename = "WEBI")]
    Webi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDebtorActivationAmendmentReason1Code {
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "TRJT")]
    Trjt,
    #[serde(rename = "UCRD")]
    Ucrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalClearingSystemIdentification1Code {
    #[serde(rename = "ATBLZ")]
    Atblz,
    #[serde(rename = "AUBSB")]
    Aubsb,
    #[serde(rename = "CACPA")]
    Cacpa,
    #[serde(rename = "CHBCC")]
    Chbcc,
    #[serde(rename = "CHSIC")]
    Chsic,
    #[serde(rename = "CNAPS")]
    Cnaps,
    #[serde(rename = "DEBLZ")]
    Deblz,
    #[serde(rename = "ESNCC")]
    Esncc,
    #[serde(rename = "GBDSC")]
    Gbdsc,
    #[serde(rename = "GRBIC")]
    Grbic,
    #[serde(rename = "HKNCC")]
    Hkncc,
    #[serde(rename = "IENCC")]
    Iencc,
    #[serde(rename = "INFSC")]
    Infsc,
    #[serde(rename = "ITNCC")]
    Itncc,
    #[serde(rename = "JPZGN")]
    Jpzgn,
    #[serde(rename = "NZNCC")]
    Nzncc,
    #[serde(rename = "PLKNR")]
    Plknr,
    #[serde(rename = "PTNCC")]
    Ptncc,
    #[serde(rename = "RUCBC")]
    Rucbc,
    #[serde(rename = "SESBA")]
    Sesba,
    #[serde(rename = "SGIBG")]
    Sgibg,
    #[serde(rename = "THCBC")]
    Thcbc,
    #[serde(rename = "TWNCC")]
    Twncc,
    #[serde(rename = "USABA")]
    Usaba,
    #[serde(rename = "USPID")]
    Uspid,
    #[serde(rename = "ZANCC")]
    Zancc,
    #[serde(rename = "NZRSA")]
    Nzrsa,
    #[serde(rename = "MZBMO")]
    Mzbmo,
    #[serde(rename = "CNCIP")]
    Cncip,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBillingRateIdentification1Code {
    #[serde(rename = "AEAR")]
    Aear,
    #[serde(rename = "ANXE")]
    Anxe,
    #[serde(rename = "CDRA")]
    Cdra,
    #[serde(rename = "DINR")]
    Dinr,
    #[serde(rename = "DSCR")]
    Dscr,
    #[serde(rename = "EALR")]
    Ealr,
    #[serde(rename = "FDIC")]
    Fdic,
    #[serde(rename = "ICDR")]
    Icdr,
    #[serde(rename = "MULT")]
    Mult,
    #[serde(rename = "NCBO")]
    Ncbo,
    #[serde(rename = "NGCO")]
    Ngco,
    #[serde(rename = "NGLO")]
    Nglo,
    #[serde(rename = "NLBO")]
    Nlbo,
    #[serde(rename = "NXME")]
    Nxme,
    #[serde(rename = "NXMU")]
    Nxmu,
    #[serde(rename = "PRIR")]
    Prir,
    #[serde(rename = "RRQR")]
    Rrqr,
    #[serde(rename = "RSRV")]
    Rsrv,
    #[serde(rename = "UFUR")]
    Ufur,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalMarketArea1Code {
    #[serde(rename = "ANYY")]
    Anyy,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "COPA")]
    Copa,
    #[serde(rename = "DERI")]
    Deri,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "FOEX")]
    Foex,
    #[serde(rename = "GUAR")]
    Guar,
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "MMKT")]
    Mmkt,
    #[serde(rename = "NDLF")]
    Ndlf,
    #[serde(rename = "OFFS")]
    Offs,
    #[serde(rename = "ONSH")]
    Onsh,
    #[serde(rename = "OPTI")]
    Opti,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "TFIN")]
    Tfin,
    #[serde(rename = "TREA")]
    Trea,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentRole1Code {
    #[serde(rename = "BKMG")]
    Bkmg,
    #[serde(rename = "LMMG")]
    Lmmg,
    #[serde(rename = "LQMG")]
    Lqmg,
    #[serde(rename = "PYMG")]
    Pymg,
    #[serde(rename = "REDR")]
    Redr,
    #[serde(rename = "STMG")]
    Stmg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPurpose1Code {
    #[serde(rename = "BKDF")]
    Bkdf,
    #[serde(rename = "BKFE")]
    Bkfe,
    #[serde(rename = "BKFM")]
    Bkfm,
    #[serde(rename = "BKIP")]
    Bkip,
    #[serde(rename = "BKPP")]
    Bkpp,
    #[serde(rename = "CBLK")]
    Cblk,
    #[serde(rename = "CDCB")]
    Cdcb,
    #[serde(rename = "CDCD")]
    Cdcd,
    #[serde(rename = "CDCS")]
    Cdcs,
    #[serde(rename = "CDDP")]
    Cddp,
    #[serde(rename = "CDOC")]
    Cdoc,
    #[serde(rename = "CDQC")]
    Cdqc,
    #[serde(rename = "ETUP")]
    Etup,
    #[serde(rename = "FCOL")]
    Fcol,
    #[serde(rename = "MTUP")]
    Mtup,
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "CSDB")]
    Csdb,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "INTC")]
    Intc,
    #[serde(rename = "INTP")]
    Intp,
    #[serde(rename = "LIMA")]
    Lima,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "CCIR")]
    Ccir,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "CCPM")]
    Ccpm,
    #[serde(rename = "CCSM")]
    Ccsm,
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
    #[serde(rename = "FWBC")]
    Fwbc,
    #[serde(rename = "FWCC")]
    Fwcc,
    #[serde(rename = "FWSB")]
    Fwsb,
    #[serde(rename = "FWSC")]
    Fwsc,
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "MBSB")]
    Mbsb,
    #[serde(rename = "MBSC")]
    Mbsc,
    #[serde(rename = "MGCC")]
    Mgcc,
    #[serde(rename = "MGSC")]
    Mgsc,
    #[serde(rename = "OCCC")]
    Occc,
    #[serde(rename = "OPBC")]
    Opbc,
    #[serde(rename = "OPCC")]
    Opcc,
    #[serde(rename = "OPSB")]
    Opsb,
    #[serde(rename = "OPSC")]
    Opsc,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "OTCD")]
    Otcd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "RPBC")]
    Rpbc,
    #[serde(rename = "RPCC")]
    Rpcc,
    #[serde(rename = "RPSB")]
    Rpsb,
    #[serde(rename = "RPSC")]
    Rpsc,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCRP")]
    Scrp,
    #[serde(rename = "SHBC")]
    Shbc,
    #[serde(rename = "SHCC")]
    Shcc,
    #[serde(rename = "SHSL")]
    Shsl,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "SLOA")]
    Sloa,
    #[serde(rename = "SWBC")]
    Swbc,
    #[serde(rename = "SWCC")]
    Swcc,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "SWSB")]
    Swsb,
    #[serde(rename = "SWSC")]
    Swsc,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "TBBC")]
    Tbbc,
    #[serde(rename = "TBCC")]
    Tbcc,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "AGRT")]
    Agrt,
    #[serde(rename = "AREN")]
    Aren,
    #[serde(rename = "BEXP")]
    Bexp,
    #[serde(rename = "BOCE")]
    Boce,
    #[serde(rename = "COMC")]
    Comc,
    #[serde(rename = "CPYR")]
    Cpyr,
    #[serde(rename = "GDDS")]
    Gdds,
    #[serde(rename = "GDSV")]
    Gdsv,
    #[serde(rename = "GSCB")]
    Gscb,
    #[serde(rename = "LICF")]
    Licf,
    #[serde(rename = "MP2B")]
    Mp2B,
    #[serde(rename = "POPE")]
    Pope,
    #[serde(rename = "ROYA")]
    Roya,
    #[serde(rename = "SCVE")]
    Scve,
    #[serde(rename = "SERV")]
    Serv,
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "SUPP")]
    Supp,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "COMT")]
    Comt,
    #[serde(rename = "MP2P")]
    Mp2P,
    #[serde(rename = "ECPG")]
    Ecpg,
    #[serde(rename = "ECPR")]
    Ecpr,
    #[serde(rename = "ECPU")]
    Ecpu,
    #[serde(rename = "EPAY")]
    Epay,
    #[serde(rename = "CLPR")]
    Clpr,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "DBTC")]
    Dbtc,
    #[serde(rename = "GOVI")]
    Govi,
    #[serde(rename = "HLRP")]
    Hlrp,
    #[serde(rename = "HLST")]
    Hlst,
    #[serde(rename = "INPC")]
    Inpc,
    #[serde(rename = "INPR")]
    Inpr,
    #[serde(rename = "INSC")]
    Insc,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "LBRI")]
    Lbri,
    #[serde(rename = "LIFI")]
    Lifi,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "LOAR")]
    Loar,
    #[serde(rename = "PENO")]
    Peno,
    #[serde(rename = "PPTI")]
    Ppti,
    #[serde(rename = "RELG")]
    Relg,
    #[serde(rename = "RINP")]
    Rinp,
    #[serde(rename = "TRFD")]
    Trfd,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FXNT")]
    Fxnt,
    #[serde(rename = "ADMG")]
    Admg,
    #[serde(rename = "ADVA")]
    Adva,
    #[serde(rename = "BCDM")]
    Bcdm,
    #[serde(rename = "BCFG")]
    Bcfg,
    #[serde(rename = "BLDM")]
    Bldm,
    #[serde(rename = "BNET")]
    Bnet,
    #[serde(rename = "CBFF")]
    Cbff,
    #[serde(rename = "CBFR")]
    Cbfr,
    #[serde(rename = "CCRD")]
    Ccrd,
    #[serde(rename = "CDBL")]
    Cdbl,
    #[serde(rename = "CFEE")]
    Cfee,
    #[serde(rename = "CGDD")]
    Cgdd,
    #[serde(rename = "CORT")]
    Cort,
    #[serde(rename = "COST")]
    Cost,
    #[serde(rename = "CPKC")]
    Cpkc,
    #[serde(rename = "DCRD")]
    Dcrd,
    #[serde(rename = "DSMT")]
    Dsmt,
    #[serde(rename = "DVPM")]
    Dvpm,
    #[serde(rename = "EDUC")]
    Educ,
    #[serde(rename = "FACT")]
    Fact,
    #[serde(rename = "FAND")]
    Fand,
    #[serde(rename = "FCPM")]
    Fcpm,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "GOVT")]
    Govt,
    #[serde(rename = "ICCP")]
    Iccp,
    #[serde(rename = "IDCP")]
    Idcp,
    #[serde(rename = "IHRP")]
    Ihrp,
    #[serde(rename = "INSM")]
    Insm,
    #[serde(rename = "IVPT")]
    Ivpt,
    #[serde(rename = "MCDM")]
    Mcdm,
    #[serde(rename = "MCFG")]
    Mcfg,
    #[serde(rename = "MSVC")]
    Msvc,
    #[serde(rename = "NOWS")]
    Nows,
    #[serde(rename = "OCDM")]
    Ocdm,
    #[serde(rename = "OCFG")]
    Ocfg,
    #[serde(rename = "OFEE")]
    Ofee,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PADD")]
    Padd,
    #[serde(rename = "PTSP")]
    Ptsp,
    #[serde(rename = "RCKE")]
    Rcke,
    #[serde(rename = "RCPT")]
    Rcpt,
    #[serde(rename = "REBT")]
    Rebt,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "RENT")]
    Rent,
    #[serde(rename = "REOD")]
    Reod,
    #[serde(rename = "RIMB")]
    Rimb,
    #[serde(rename = "RPNT")]
    Rpnt,
    #[serde(rename = "RRBN")]
    Rrbn,
    #[serde(rename = "RRCT")]
    Rrct,
    #[serde(rename = "RRTP")]
    Rrtp,
    #[serde(rename = "RVPM")]
    Rvpm,
    #[serde(rename = "SLPI")]
    Slpi,
    #[serde(rename = "SPLT")]
    Splt,
    #[serde(rename = "STDY")]
    Stdy,
    #[serde(rename = "TBAN")]
    Tban,
    #[serde(rename = "TBIL")]
    Tbil,
    #[serde(rename = "TCSC")]
    Tcsc,
    #[serde(rename = "TELI")]
    Teli,
    #[serde(rename = "TMPG")]
    Tmpg,
    #[serde(rename = "TPRI")]
    Tpri,
    #[serde(rename = "TPRP")]
    Tprp,
    #[serde(rename = "TRNC")]
    Trnc,
    #[serde(rename = "TRVC")]
    Trvc,
    #[serde(rename = "WEBI")]
    Webi,
    #[serde(rename = "IPAY")]
    Ipay,
    #[serde(rename = "IPCA")]
    Ipca,
    #[serde(rename = "IPDO")]
    Ipdo,
    #[serde(rename = "IPEA")]
    Ipea,
    #[serde(rename = "IPEC")]
    Ipec,
    #[serde(rename = "IPEW")]
    Ipew,
    #[serde(rename = "IPPS")]
    Ipps,
    #[serde(rename = "IPRT")]
    Iprt,
    #[serde(rename = "IPU2")]
    Ipu2,
    #[serde(rename = "IPUW")]
    Ipuw,
    #[serde(rename = "ANNI")]
    Anni,
    #[serde(rename = "CAFI")]
    Cafi,
    #[serde(rename = "CFDI")]
    Cfdi,
    #[serde(rename = "CMDT")]
    Cmdt,
    #[serde(rename = "DERI")]
    Deri,
    #[serde(rename = "DIVD")]
    Divd,
    #[serde(rename = "FREX")]
    Frex,
    #[serde(rename = "HEDG")]
    Hedg,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "PRME")]
    Prme,
    #[serde(rename = "SAVG")]
    Savg,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SEPI")]
    Sepi,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "UNIT")]
    Unit,
    #[serde(rename = "FNET")]
    Fnet,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "ANTS")]
    Ants,
    #[serde(rename = "CVCF")]
    Cvcf,
    #[serde(rename = "DMEQ")]
    Dmeq,
    #[serde(rename = "DNTS")]
    Dnts,
    #[serde(rename = "HLTC")]
    Hltc,
    #[serde(rename = "HLTI")]
    Hlti,
    #[serde(rename = "HSPC")]
    Hspc,
    #[serde(rename = "ICRF")]
    Icrf,
    #[serde(rename = "LTCF")]
    Ltcf,
    #[serde(rename = "MAFC")]
    Mafc,
    #[serde(rename = "MARF")]
    Marf,
    #[serde(rename = "MDCS")]
    Mdcs,
    #[serde(rename = "VIEW")]
    View,
    #[serde(rename = "CDEP")]
    Cdep,
    #[serde(rename = "SWFP")]
    Swfp,
    #[serde(rename = "SWPP")]
    Swpp,
    #[serde(rename = "SWRS")]
    Swrs,
    #[serde(rename = "SWUF")]
    Swuf,
    #[serde(rename = "ADCS")]
    Adcs,
    #[serde(rename = "AEMP")]
    Aemp,
    #[serde(rename = "ALLW")]
    Allw,
    #[serde(rename = "ALMY")]
    Almy,
    #[serde(rename = "BBSC")]
    Bbsc,
    #[serde(rename = "BECH")]
    Bech,
    #[serde(rename = "BENE")]
    Bene,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "CCHD")]
    Cchd,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CSLP")]
    Cslp,
    #[serde(rename = "GFRP")]
    Gfrp,
    #[serde(rename = "GVEA")]
    Gvea,
    #[serde(rename = "GVEB")]
    Gveb,
    #[serde(rename = "GVEC")]
    Gvec,
    #[serde(rename = "GVED")]
    Gved,
    #[serde(rename = "GWLT")]
    Gwlt,
    #[serde(rename = "HREC")]
    Hrec,
    #[serde(rename = "PAYR")]
    Payr,
    #[serde(rename = "PEFC")]
    Pefc,
    #[serde(rename = "PENS")]
    Pens,
    #[serde(rename = "PRCP")]
    Prcp,
    #[serde(rename = "RHBS")]
    Rhbs,
    #[serde(rename = "SALA")]
    Sala,
    #[serde(rename = "SPSP")]
    Spsp,
    #[serde(rename = "SSBE")]
    Ssbe,
    #[serde(rename = "LBIN")]
    Lbin,
    #[serde(rename = "LCOL")]
    Lcol,
    #[serde(rename = "LFEE")]
    Lfee,
    #[serde(rename = "LMEQ")]
    Lmeq,
    #[serde(rename = "LMFI")]
    Lmfi,
    #[serde(rename = "LMRK")]
    Lmrk,
    #[serde(rename = "LREB")]
    Lreb,
    #[serde(rename = "LREV")]
    Lrev,
    #[serde(rename = "LSFL")]
    Lsfl,
    #[serde(rename = "ESTX")]
    Estx,
    #[serde(rename = "FWLV")]
    Fwlv,
    #[serde(rename = "GSTX")]
    Gstx,
    #[serde(rename = "HSTX")]
    Hstx,
    #[serde(rename = "INTX")]
    Intx,
    #[serde(rename = "NITX")]
    Nitx,
    #[serde(rename = "PTXP")]
    Ptxp,
    #[serde(rename = "RDTX")]
    Rdtx,
    #[serde(rename = "TAXS")]
    Taxs,
    #[serde(rename = "VATX")]
    Vatx,
    #[serde(rename = "WHLD")]
    Whld,
    #[serde(rename = "TAXR")]
    Taxr,
    #[serde(rename = "B112")]
    B112,
    #[serde(rename = "BR12")]
    Br12,
    #[serde(rename = "TLRF")]
    Tlrf,
    #[serde(rename = "TLRR")]
    Tlrr,
    #[serde(rename = "AIRB")]
    Airb,
    #[serde(rename = "BUSB")]
    Busb,
    #[serde(rename = "FERB")]
    Ferb,
    #[serde(rename = "RLWY")]
    Rlwy,
    #[serde(rename = "TRPT")]
    Trpt,
    #[serde(rename = "CBTV")]
    Cbtv,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "ENRG")]
    Enrg,
    #[serde(rename = "GASB")]
    Gasb,
    #[serde(rename = "NWCH")]
    Nwch,
    #[serde(rename = "NWCM")]
    Nwcm,
    #[serde(rename = "OTLC")]
    Otlc,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "UBIL")]
    Ubil,
    #[serde(rename = "WTER")]
    Wter,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CABD")]
    Cabd,
    #[serde(rename = "CAEQ")]
    Caeq,
    #[serde(rename = "CBCR")]
    Cbcr,
    #[serde(rename = "DBCR")]
    Dbcr,
    #[serde(rename = "DICL")]
    Dicl,
    #[serde(rename = "EQTS")]
    Eqts,
    #[serde(rename = "FLCR")]
    Flcr,
    #[serde(rename = "EFTC")]
    Eftc,
    #[serde(rename = "EFTD")]
    Eftd,
    #[serde(rename = "MOMA")]
    Moma,
    #[serde(rename = "RAPI")]
    Rapi,
    #[serde(rename = "GAMB")]
    Gamb,
    #[serde(rename = "LOTT")]
    Lott,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "SASW")]
    Sasw,
    #[serde(rename = "AUCO")]
    Auco,
    #[serde(rename = "PCOM")]
    Pcom,
    #[serde(rename = "PDEP")]
    Pdep,
    #[serde(rename = "PLDS")]
    Plds,
    #[serde(rename = "PLRF")]
    Plrf,
    #[serde(rename = "GAFA")]
    Gafa,
    #[serde(rename = "GAHO")]
    Gaho,
    #[serde(rename = "CPEN")]
    Cpen,
    #[serde(rename = "DEPD")]
    Depd,
    #[serde(rename = "RETL")]
    Retl,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalSecuritiesLendingType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalMandateSuspensionReason1Code {
    #[serde(rename = "CTAM")]
    Ctam,
    #[serde(rename = "CTCA")]
    Ctca,
    #[serde(rename = "CTEX")]
    Ctex,
    #[serde(rename = "MCFC")]
    Mcfc,
    #[serde(rename = "MCOC")]
    Mcoc,
    #[serde(rename = "MSUC")]
    Msuc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDocumentType1Code {
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "CNFA")]
    Cnfa,
    #[serde(rename = "CONT")]
    Cont,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "DEBN")]
    Debn,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "DNFA")]
    Dnfa,
    #[serde(rename = "HIRI")]
    Hiri,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "MSIN")]
    Msin,
    #[serde(rename = "PROF")]
    Prof,
    #[serde(rename = "PUOR")]
    Puor,
    #[serde(rename = "QUOT")]
    Quot,
    #[serde(rename = "SBIN")]
    Sbin,
    #[serde(rename = "SPRR")]
    Sprr,
    #[serde(rename = "TISH")]
    Tish,
    #[serde(rename = "USAR")]
    Usar,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalSystemBalanceType1Code {
    #[serde(rename = "ADJT")]
    Adjt,
    #[serde(rename = "ADWR")]
    Adwr,
    #[serde(rename = "AIDR")]
    Aidr,
    #[serde(rename = "AVLB")]
    Avlb,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "BSCC")]
    Bscc,
    #[serde(rename = "BSCD")]
    Bscd,
    #[serde(rename = "CCPS")]
    Ccps,
    #[serde(rename = "CLSG")]
    Clsg,
    #[serde(rename = "COHB")]
    Cohb,
    #[serde(rename = "COLC")]
    Colc,
    #[serde(rename = "CPBL")]
    Cpbl,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CRRT")]
    Crrt,
    #[serde(rename = "CUSA")]
    Cusa,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DBIT")]
    Dbit,
    #[serde(rename = "DLOD")]
    Dlod,
    #[serde(rename = "DOHB")]
    Dohb,
    #[serde(rename = "DPBL")]
    Dpbl,
    #[serde(rename = "DSET")]
    Dset,
    #[serde(rename = "DWRD")]
    Dwrd,
    #[serde(rename = "EAST")]
    East,
    #[serde(rename = "EXRE")]
    Exre,
    #[serde(rename = "EXRR")]
    Exrr,
    #[serde(rename = "FCOL")]
    Fcol,
    #[serde(rename = "FCOU")]
    Fcou,
    #[serde(rename = "FORC")]
    Forc,
    #[serde(rename = "FSET")]
    Fset,
    #[serde(rename = "FUND")]
    Fund,
    #[serde(rename = "FUTB")]
    Futb,
    #[serde(rename = "INTM")]
    Intm,
    #[serde(rename = "IRDR")]
    Irdr,
    #[serde(rename = "IRLT")]
    Irlt,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LRLD")]
    Lrld,
    #[serde(rename = "LTSF")]
    Ltsf,
    #[serde(rename = "MSTR")]
    Mstr,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "NSET")]
    Nset,
    #[serde(rename = "OPNG")]
    Opng,
    #[serde(rename = "OTCC")]
    Otcc,
    #[serde(rename = "OTCG")]
    Otcg,
    #[serde(rename = "OTCN")]
    Otcn,
    #[serde(rename = "OTHB")]
    Othb,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "PIPO")]
    Pipo,
    #[serde(rename = "PRAV")]
    Prav,
    #[serde(rename = "PYMT")]
    Pymt,
    #[serde(rename = "REJB")]
    Rejb,
    #[serde(rename = "REPC")]
    Repc,
    #[serde(rename = "REPD")]
    Repd,
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "SAPC")]
    Sapc,
    #[serde(rename = "SAPD")]
    Sapd,
    #[serde(rename = "SAPP")]
    Sapp,
    #[serde(rename = "SCOL")]
    Scol,
    #[serde(rename = "SCOU")]
    Scou,
    #[serde(rename = "SELF")]
    XSelf,
    #[serde(rename = "THRE")]
    Thre,
    #[serde(rename = "TOHB")]
    Tohb,
    #[serde(rename = "TPBL")]
    Tpbl,
    #[serde(rename = "XCHC")]
    Xchc,
    #[serde(rename = "XCHG")]
    Xchg,
    #[serde(rename = "XCHN")]
    Xchn,
    #[serde(rename = "XCRD")]
    Xcrd,
    #[serde(rename = "XDBT")]
    Xdbt,
    #[serde(rename = "XPCD")]
    Xpcd,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalBankTransactionSubFamily1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBillingBalanceType1Code {
    #[serde(rename = "ABRR")]
    Abrr,
    #[serde(rename = "BEQU")]
    Bequ,
    #[serde(rename = "CBAM")]
    Cbam,
    #[serde(rename = "CBAN")]
    Cban,
    #[serde(rename = "CBAP")]
    Cbap,
    #[serde(rename = "DABR")]
    Dabr,
    #[serde(rename = "EDCB")]
    Edcb,
    #[serde(rename = "EDIB")]
    Edib,
    #[serde(rename = "FDIC")]
    Fdic,
    #[serde(rename = "FLBA")]
    Flba,
    #[serde(rename = "FLPP")]
    Flpp,
    #[serde(rename = "IBAL")]
    Ibal,
    #[serde(rename = "IBIB")]
    Ibib,
    #[serde(rename = "IBLB")]
    Iblb,
    #[serde(rename = "IBNC")]
    Ibnc,
    #[serde(rename = "IBNG")]
    Ibng,
    #[serde(rename = "IBPC")]
    Ibpc,
    #[serde(rename = "IBRR")]
    Ibrr,
    #[serde(rename = "LBAN")]
    Lban,
    #[serde(rename = "LBAP")]
    Lbap,
    #[serde(rename = "LBME")]
    Lbme,
    #[serde(rename = "LBNM")]
    Lbnm,
    #[serde(rename = "UCFU")]
    Ucfu,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalChequeCancellationStatus1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAuthenticationChannel1Code {
    #[serde(rename = "ATMA")]
    Atma,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "INBA")]
    Inba,
    #[serde(rename = "MOBI")]
    Mobi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalReservationType1Code {
    #[serde(rename = "BLKD")]
    Blkd,
    #[serde(rename = "CARE")]
    Care,
    #[serde(rename = "HPAR")]
    Hpar,
    #[serde(rename = "NSSR")]
    Nssr,
    #[serde(rename = "THRE")]
    Thre,
    #[serde(rename = "UPAR")]
    Upar,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalVerificationReason1Code {
    #[serde(rename = "AC01")]
    Ac01,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "DUPL")]
    Dupl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCategoryPurpose1Code {
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CBLK")]
    Cblk,
    #[serde(rename = "CCRD")]
    Ccrd,
    #[serde(rename = "CORT")]
    Cort,
    #[serde(rename = "DCRD")]
    Dcrd,
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "DVPM")]
    Dvpm,
    #[serde(rename = "EPAY")]
    Epay,
    #[serde(rename = "FCIN")]
    Fcin,
    #[serde(rename = "FCOL")]
    Fcol,
    #[serde(rename = "GP2P")]
    Gp2P,
    #[serde(rename = "GOVT")]
    Govt,
    #[serde(rename = "HEDG")]
    Hedg,
    #[serde(rename = "ICCP")]
    Iccp,
    #[serde(rename = "IDCP")]
    Idcp,
    #[serde(rename = "INTC")]
    Intc,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "LBOX")]
    Lbox,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "MP2B")]
    Mp2B,
    #[serde(rename = "MP2P")]
    Mp2P,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PENS")]
    Pens,
    #[serde(rename = "RPRE")]
    Rpre,
    #[serde(rename = "RRCT")]
    Rrct,
    #[serde(rename = "RVPM")]
    Rvpm,
    #[serde(rename = "SALA")]
    Sala,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SSBE")]
    Ssbe,
    #[serde(rename = "SUPP")]
    Supp,
    #[serde(rename = "TAXS")]
    Taxs,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "VATX")]
    Vatx,
    #[serde(rename = "WHLD")]
    Whld,
    #[serde(rename = "SWEP")]
    Swep,
    #[serde(rename = "TOPG")]
    Topg,
    #[serde(rename = "ZABA")]
    Zaba,
    #[serde(rename = "VOST")]
    Vost,
    #[serde(rename = "FCDT")]
    Fcdt,
    #[serde(rename = "CIPC")]
    Cipc,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "CGWV")]
    Cgwv,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalClaimNonReceiptRejection1Code {
    #[serde(rename = "NOOR")]
    Noor,
    #[serde(rename = "RNPR")]
    Rnpr,
    #[serde(rename = "ARJT")]
    Arjt,
    #[serde(rename = "ARDT")]
    Ardt,
    #[serde(rename = "RR04")]
    Rr04,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentTransactionStatus1Code {
    #[serde(rename = "ACCC")]
    Accc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACFC")]
    Acfc,
    #[serde(rename = "ACIS")]
    Acis,
    #[serde(rename = "ACSC")]
    Acsc,
    #[serde(rename = "ACSP")]
    Acsp,
    #[serde(rename = "ACTC")]
    Actc,
    #[serde(rename = "ACWC")]
    Acwc,
    #[serde(rename = "ACWP")]
    Acwp,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "CPUC")]
    Cpuc,
    #[serde(rename = "PATC")]
    Patc,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "PRES")]
    Pres,
    #[serde(rename = "RCVD")]
    Rcvd,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "ACPD")]
    Acpd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPackagingType1Code {
    #[serde(rename = "HDPE")]
    Hdpe,
    #[serde(rename = "LDPE")]
    Ldpe,
    #[serde(rename = "LLDP")]
    Lldp,
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
pub struct ExternalSystemPartyType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalTaxAmountType1Code {
    #[serde(rename = "CITY")]
    City,
    #[serde(rename = "CNTY")]
    Cnty,
    #[serde(rename = "LOCL")]
    Locl,
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "STAT")]
    Stat,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalTradeTransactionCondition1Code {
    #[serde(rename = "BCBL")]
    Bcbl,
    #[serde(rename = "BCBN")]
    Bcbn,
    #[serde(rename = "BCFD")]
    Bcfd,
    #[serde(rename = "BCPD")]
    Bcpd,
    #[serde(rename = "BCRO")]
    Bcro,
    #[serde(rename = "BCRP")]
    Bcrp,
    #[serde(rename = "BLKO")]
    Blko,
    #[serde(rename = "BTEX")]
    Btex,
    #[serde(rename = "BTMI")]
    Btmi,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "CBNS")]
    Cbns,
    #[serde(rename = "CCPN")]
    Ccpn,
    #[serde(rename = "CDIV")]
    Cdiv,
    #[serde(rename = "CLBR")]
    Clbr,
    #[serde(rename = "CLEN")]
    Clen,
    #[serde(rename = "CRST")]
    Crst,
    #[serde(rename = "CRTS")]
    Crts,
    #[serde(rename = "CWAR")]
    Cwar,
    #[serde(rename = "DIOR")]
    Dior,
    #[serde(rename = "DIRT")]
    Dirt,
    #[serde(rename = "DORD")]
    Dord,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FRAC")]
    Frac,
    #[serde(rename = "GTDL")]
    Gtdl,
    #[serde(rename = "HIST")]
    Hist,
    #[serde(rename = "MAPR")]
    Mapr,
    #[serde(rename = "MONT")]
    Mont,
    #[serde(rename = "NBFR")]
    Nbfr,
    #[serde(rename = "NCRS")]
    Ncrs,
    #[serde(rename = "NEGO")]
    Nego,
    #[serde(rename = "NMPR")]
    Nmpr,
    #[serde(rename = "PETA")]
    Peta,
    #[serde(rename = "PUTT")]
    Putt,
    #[serde(rename = "SETI")]
    Seti,
    #[serde(rename = "SPCU")]
    Spcu,
    #[serde(rename = "SPEX")]
    Spex,
    #[serde(rename = "SPSI")]
    Spsi,
    #[serde(rename = "SSTI")]
    Ssti,
    #[serde(rename = "TEFR")]
    Tefr,
    #[serde(rename = "TRFR")]
    Trfr,
    #[serde(rename = "WIBC")]
    Wibc,
    #[serde(rename = "WICD")]
    Wicd,
    #[serde(rename = "XBNS")]
    Xbns,
    #[serde(rename = "XCPN")]
    Xcpn,
    #[serde(rename = "XDIV")]
    Xdiv,
    #[serde(rename = "XRTS")]
    Xrts,
    #[serde(rename = "XWAR")]
    Xwar,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCreditLineType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
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
pub struct ExternalFinancialInstrumentProductType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalReversalReason1Code {
    #[serde(rename = "AC03")]
    Ac03,
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "AG02")]
    Ag02,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "AM09")]
    Am09,
    #[serde(rename = "MD01")]
    Md01,
    #[serde(rename = "MD05")]
    Md05,
    #[serde(rename = "MS02")]
    Ms02,
    #[serde(rename = "MS03")]
    Ms03,
    #[serde(rename = "RC07")]
    Rc07,
    #[serde(rename = "TM01")]
    Tm01,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDebtorActivationStatusReason1Code {
    #[serde(rename = "ACRD")]
    Acrd,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "FF01")]
    Ff01,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "RR10")]
    Rr10,
    #[serde(rename = "TRJT")]
    Trjt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUnableToApplyMissingData1Code {
    #[serde(rename = "MS01")]
    Ms01,
    #[serde(rename = "MS02")]
    Ms02,
    #[serde(rename = "MS03")]
    Ms03,
    #[serde(rename = "MS04")]
    Ms04,
    #[serde(rename = "MS05")]
    Ms05,
    #[serde(rename = "MS06")]
    Ms06,
    #[serde(rename = "MS07")]
    Ms07,
    #[serde(rename = "MS08")]
    Ms08,
    #[serde(rename = "MS09")]
    Ms09,
    #[serde(rename = "MS10")]
    Ms10,
    #[serde(rename = "MS11")]
    Ms11,
    #[serde(rename = "MS12")]
    Ms12,
    #[serde(rename = "MS13")]
    Ms13,
    #[serde(rename = "MS14")]
    Ms14,
    #[serde(rename = "MS15")]
    Ms15,
    #[serde(rename = "MS16")]
    Ms16,
    #[serde(rename = "MS17")]
    Ms17,
    #[serde(rename = "MS18")]
    Ms18,
    #[serde(rename = "NARR")]
    Narr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAuthorityExchangeReason1Code {
    #[serde(rename = "ADHR")]
    Adhr,
    #[serde(rename = "CABB")]
    Cabb,
    #[serde(rename = "CABC")]
    Cabc,
    #[serde(rename = "CABT")]
    Cabt,
    #[serde(rename = "CADB")]
    Cadb,
    #[serde(rename = "CADU")]
    Cadu,
    #[serde(rename = "CAEB")]
    Caeb,
    #[serde(rename = "CAFI")]
    Cafi,
    #[serde(rename = "CAIX")]
    Caix,
    #[serde(rename = "CAMB")]
    Camb,
    #[serde(rename = "CASB")]
    Casb,
    #[serde(rename = "CAST")]
    Cast,
    #[serde(rename = "CATV")]
    Catv,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalContractClosureReason1Code {
    #[serde(rename = "PCED")]
    Pced,
    #[serde(rename = "POTR")]
    Potr,
    #[serde(rename = "PRNR")]
    Prnr,
    #[serde(rename = "PSBT")]
    Psbt,
    #[serde(rename = "PSNR")]
    Psnr,
    #[serde(rename = "PTAA")]
    Ptaa,
    #[serde(rename = "RACC")]
    Racc,
    #[serde(rename = "RCED")]
    Rced,
    #[serde(rename = "REXP")]
    Rexp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalOrganisationIdentification1Code {
    #[serde(rename = "BANK")]
    Bank,
    #[serde(rename = "CBID")]
    Cbid,
    #[serde(rename = "CHID")]
    Chid,
    #[serde(rename = "CINC")]
    Cinc,
    #[serde(rename = "COID")]
    Coid,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DUNS")]
    Duns,
    #[serde(rename = "EMPL")]
    Empl,
    #[serde(rename = "GS1G")]
    Gs1G,
    #[serde(rename = "SREN")]
    Sren,
    #[serde(rename = "SRET")]
    Sret,
    #[serde(rename = "TXID")]
    Txid,
    #[serde(rename = "BDID")]
    Bdid,
    #[serde(rename = "BOID")]
    Boid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBillingCompensationType1Code {
    #[serde(rename = "BACS")]
    Bacs,
    #[serde(rename = "CTND")]
    Ctnd,
    #[serde(rename = "DEAD")]
    Dead,
    #[serde(rename = "EALL")]
    Eall,
    #[serde(rename = "EANA")]
    Eana,
    #[serde(rename = "EDAA")]
    Edaa,
    #[serde(rename = "EDAL")]
    Edal,
    #[serde(rename = "FESS")]
    Fess,
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "NBCS")]
    Nbcs,
    #[serde(rename = "PVCS")]
    Pvcs,
    #[serde(rename = "SCAB")]
    Scab,
    #[serde(rename = "SCAN")]
    Scan,
    #[serde(rename = "SCBT")]
    Scbt,
    #[serde(rename = "SCCP")]
    Sccp,
    #[serde(rename = "SCDB")]
    Scdb,
    #[serde(rename = "SCDI")]
    Scdi,
    #[serde(rename = "SCIN")]
    Scin,
    #[serde(rename = "TICD")]
    Ticd,
    #[serde(rename = "TXSC")]
    Txsc,
    #[serde(rename = "TXTS")]
    Txts,
    #[serde(rename = "WAIV")]
    Waiv,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalChequeAgentInstruction1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
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
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAccountIdentification1Code {
    #[serde(rename = "AIIN")]
    Aiin,
    #[serde(rename = "BBAN")]
    Bban,
    #[serde(rename = "CUID")]
    Cuid,
    #[serde(rename = "UPIC")]
    Upic,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalSystemMemberType1Code {
    #[serde(rename = "DRCT")]
    Drct,
    #[serde(rename = "IDRT")]
    Idrt,
    #[serde(rename = "RMTE")]
    Rmte,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "STEP")]
    Step,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalChargeType1Code {
    #[serde(rename = "BRKF")]
    Brkf,
    #[serde(rename = "BTCH")]
    Btch,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "SUMM")]
    Summ,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingChargeType1Code {
    #[serde(rename = "AMND")]
    Amnd,
    #[serde(rename = "CLAM")]
    Clam,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "COUR")]
    Cour,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "MISC")]
    Misc,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "TELE")]
    Tele,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPersonIdentification1Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "EMPL")]
    Empl,
    #[serde(rename = "NIDN")]
    Nidn,
    #[serde(rename = "SOSE")]
    Sose,
    #[serde(rename = "TELE")]
    Tele,
    #[serde(rename = "TXID")]
    Txid,
    #[serde(rename = "POID")]
    Poid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCashClearingSystem1Code {
    #[serde(rename = "ABE")]
    Abe,
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "ACS")]
    Acs,
    #[serde(rename = "AIP")]
    Aip,
    #[serde(rename = "ART")]
    Art,
    #[serde(rename = "AVP")]
    Avp,
    #[serde(rename = "AZM")]
    Azm,
    #[serde(rename = "BAP")]
    Bap,
    #[serde(rename = "BCC")]
    Bcc,
    #[serde(rename = "BCE")]
    Bce,
    #[serde(rename = "BDS")]
    Bds,
    #[serde(rename = "BEL")]
    Bel,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHS")]
    Bhs,
    #[serde(rename = "BIS")]
    Bis,
    #[serde(rename = "BOF")]
    Bof,
    #[serde(rename = "BOJ")]
    Boj,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSP")]
    Bsp,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CAM")]
    Cam,
    #[serde(rename = "CBA")]
    Cba,
    #[serde(rename = "CBC")]
    Cbc,
    #[serde(rename = "CBJ")]
    Cbj,
    #[serde(rename = "CCE")]
    Cce,
    #[serde(rename = "CHI")]
    Chi,
    #[serde(rename = "CHP")]
    Chp,
    #[serde(rename = "CIP")]
    Cip,
    #[serde(rename = "CIS")]
    Cis,
    #[serde(rename = "COE")]
    Coe,
    #[serde(rename = "COI")]
    Coi,
    #[serde(rename = "COU")]
    Cou,
    #[serde(rename = "DDK")]
    Ddk,
    #[serde(rename = "DKC")]
    Dkc,
    #[serde(rename = "EBA")]
    Eba,
    #[serde(rename = "ELS")]
    Els,
    #[serde(rename = "EMZ")]
    Emz,
    #[serde(rename = "EPM")]
    Epm,
    #[serde(rename = "EPN")]
    Epn,
    #[serde(rename = "ERP")]
    Erp,
    #[serde(rename = "FDA")]
    Fda,
    #[serde(rename = "FDN")]
    Fdn,
    #[serde(rename = "FDW")]
    Fdw,
    #[serde(rename = "FEY")]
    Fey,
    #[serde(rename = "FPS")]
    Fps,
    #[serde(rename = "GIS")]
    Gis,
    #[serde(rename = "HKL")]
    Hkl,
    #[serde(rename = "HKS")]
    Hks,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HRM")]
    Hrm,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "IBP")]
    Ibp,
    #[serde(rename = "INC")]
    Inc,
    #[serde(rename = "IMP")]
    Imp,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "KPS")]
    Kps,
    #[serde(rename = "LGS")]
    Lgs,
    #[serde(rename = "LKB")]
    Lkb,
    #[serde(rename = "LVL")]
    Lvl,
    #[serde(rename = "LVT")]
    Lvt,
    #[serde(rename = "LYX")]
    Lyx,
    #[serde(rename = "MEP")]
    Mep,
    #[serde(rename = "MOS")]
    Mos,
    #[serde(rename = "MQQ")]
    Mqq,
    #[serde(rename = "MRS")]
    Mrs,
    #[serde(rename = "MUP")]
    Mup,
    #[serde(rename = "NAM")]
    Nam,
    #[serde(rename = "NOC")]
    Noc,
    #[serde(rename = "NOR")]
    Nor,
    #[serde(rename = "NPP")]
    Npp,
    #[serde(rename = "NSS")]
    Nss,
    #[serde(rename = "NZE")]
    Nze,
    #[serde(rename = "PCH")]
    Pch,
    #[serde(rename = "PDS")]
    Pds,
    #[serde(rename = "PEG")]
    Peg,
    #[serde(rename = "PNS")]
    Pns,
    #[serde(rename = "PSA")]
    Psa,
    #[serde(rename = "PTR")]
    Ptr,
    #[serde(rename = "PVE")]
    Pve,
    #[serde(rename = "ROL")]
    Rol,
    #[serde(rename = "ROS")]
    Ros,
    #[serde(rename = "RTG")]
    Rtg,
    #[serde(rename = "RTP")]
    Rtp,
    #[serde(rename = "RTR")]
    Rtr,
    #[serde(rename = "SCL")]
    Scl,
    #[serde(rename = "SCP")]
    Scp,
    #[serde(rename = "SEC")]
    Sec,
    #[serde(rename = "SEU")]
    Seu,
    #[serde(rename = "SIC")]
    Sic,
    #[serde(rename = "SIP")]
    Sip,
    #[serde(rename = "SIT")]
    Sit,
    #[serde(rename = "SLB")]
    Slb,
    #[serde(rename = "SPG")]
    Spg,
    #[serde(rename = "SSK")]
    Ssk,
    #[serde(rename = "ST2")]
    St2,
    #[serde(rename = "STG")]
    Stg,
    #[serde(rename = "TBF")]
    Tbf,
    #[serde(rename = "TCH")]
    Tch,
    #[serde(rename = "TGT")]
    Tgt,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "THN")]
    Thn,
    #[serde(rename = "TIS")]
    Tis,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "UBE")]
    Ube,
    #[serde(rename = "UIS")]
    Uis,
    #[serde(rename = "UPI")]
    Upi,
    #[serde(rename = "VCS")]
    Vcs,
    #[serde(rename = "XCT")]
    Xct,
    #[serde(rename = "ZEN")]
    Zen,
    #[serde(rename = "ZET")]
    Zet,
    #[serde(rename = "ZIS")]
    Zis,
    #[serde(rename = "ISG")]
    Isg,
    #[serde(rename = "NBO")]
    Nbo,
    #[serde(rename = "ISW")]
    Isw,
    #[serde(rename = "I27")]
    I27,
    #[serde(rename = "B27")]
    B27,
    #[serde(rename = "UKD")]
    Ukd,
    #[serde(rename = "SCR")]
    Scr,
    #[serde(rename = "RIX")]
    Rix,
    #[serde(rename = "MOC")]
    Moc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCreditorEnrolmentAmendmentReason1Code {
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "TRJT")]
    Trjt,
    #[serde(rename = "UCRD")]
    Ucrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalReturnReason1Code {
    #[serde(rename = "AC01")]
    Ac01,
    #[serde(rename = "AC03")]
    Ac03,
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "AC06")]
    Ac06,
    #[serde(rename = "AC13")]
    Ac13,
    #[serde(rename = "AC14")]
    Ac14,
    #[serde(rename = "AC15")]
    Ac15,
    #[serde(rename = "AC16")]
    Ac16,
    #[serde(rename = "AC17")]
    Ac17,
    #[serde(rename = "AG01")]
    Ag01,
    #[serde(rename = "AG02")]
    Ag02,
    #[serde(rename = "AM01")]
    Am01,
    #[serde(rename = "AM02")]
    Am02,
    #[serde(rename = "AM03")]
    Am03,
    #[serde(rename = "AM04")]
    Am04,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "AM06")]
    Am06,
    #[serde(rename = "AM07")]
    Am07,
    #[serde(rename = "AM09")]
    Am09,
    #[serde(rename = "AM10")]
    Am10,
    #[serde(rename = "ARDT")]
    Ardt,
    #[serde(rename = "BE01")]
    Be01,
    #[serde(rename = "BE04")]
    Be04,
    #[serde(rename = "BE05")]
    Be05,
    #[serde(rename = "BE06")]
    Be06,
    #[serde(rename = "BE07")]
    Be07,
    #[serde(rename = "BE08")]
    Be08,
    #[serde(rename = "CN01")]
    Cn01,
    #[serde(rename = "CNOR")]
    Cnor,
    #[serde(rename = "CNPC")]
    Cnpc,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DNOR")]
    Dnor,
    #[serde(rename = "DS28")]
    Ds28,
    #[serde(rename = "DT01")]
    Dt01,
    #[serde(rename = "DT02")]
    Dt02,
    #[serde(rename = "ED01")]
    Ed01,
    #[serde(rename = "ED03")]
    Ed03,
    #[serde(rename = "ED05")]
    Ed05,
    #[serde(rename = "EMVL")]
    Emvl,
    #[serde(rename = "ERIN")]
    Erin,
    #[serde(rename = "FF05")]
    Ff05,
    #[serde(rename = "FOCR")]
    Focr,
    #[serde(rename = "FR01")]
    Fr01,
    #[serde(rename = "FRTR")]
    Frtr,
    #[serde(rename = "MD01")]
    Md01,
    #[serde(rename = "MD02")]
    Md02,
    #[serde(rename = "MD06")]
    Md06,
    #[serde(rename = "MD07")]
    Md07,
    #[serde(rename = "MS02")]
    Ms02,
    #[serde(rename = "MS03")]
    Ms03,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "NOAS")]
    Noas,
    #[serde(rename = "NOCM")]
    Nocm,
    #[serde(rename = "NOOR")]
    Noor,
    #[serde(rename = "PINL")]
    Pinl,
    #[serde(rename = "RC01")]
    Rc01,
    #[serde(rename = "RC07")]
    Rc07,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR01")]
    Rr01,
    #[serde(rename = "RR02")]
    Rr02,
    #[serde(rename = "RR03")]
    Rr03,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "RUTA")]
    Ruta,
    #[serde(rename = "SL01")]
    Sl01,
    #[serde(rename = "SL02")]
    Sl02,
    #[serde(rename = "SL11")]
    Sl11,
    #[serde(rename = "SL12")]
    Sl12,
    #[serde(rename = "SL13")]
    Sl13,
    #[serde(rename = "SL14")]
    Sl14,
    #[serde(rename = "SP01")]
    Sp01,
    #[serde(rename = "SP02")]
    Sp02,
    #[serde(rename = "SVNR")]
    Svnr,
    #[serde(rename = "TM01")]
    Tm01,
    #[serde(rename = "TRAC")]
    Trac,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "FF06")]
    Ff06,
    #[serde(rename = "RC08")]
    Rc08,
    #[serde(rename = "BE11")]
    Be11,
    #[serde(rename = "BE17")]
    Be17,
    #[serde(rename = "AC02")]
    Ac02,
    #[serde(rename = "RR11")]
    Rr11,
    #[serde(rename = "BE10")]
    Be10,
    #[serde(rename = "BE16")]
    Be16,
    #[serde(rename = "RC11")]
    Rc11,
    #[serde(rename = "RR12")]
    Rr12,
    #[serde(rename = "FF03")]
    Ff03,
    #[serde(rename = "FF07")]
    Ff07,
    #[serde(rename = "FF04")]
    Ff04,
    #[serde(rename = "RR09")]
    Rr09,
    #[serde(rename = "RR05")]
    Rr05,
    #[serde(rename = "RR07")]
    Rr07,
    #[serde(rename = "RR08")]
    Rr08,
    #[serde(rename = "RR06")]
    Rr06,
    #[serde(rename = "AG07")]
    Ag07,
    #[serde(rename = "G004")]
    G004,
    #[serde(rename = "MD05")]
    Md05,
    #[serde(rename = "AC07")]
    Ac07,
    #[serde(rename = "DC04")]
    Dc04,
    #[serde(rename = "RC04")]
    Rc04,
    #[serde(rename = "DT04")]
    Dt04,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "RC03")]
    Rc03,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalStatusReason1Code {
    #[serde(rename = "AB01")]
    Ab01,
    #[serde(rename = "AB02")]
    Ab02,
    #[serde(rename = "AB03")]
    Ab03,
    #[serde(rename = "AB04")]
    Ab04,
    #[serde(rename = "AB05")]
    Ab05,
    #[serde(rename = "AB06")]
    Ab06,
    #[serde(rename = "AB07")]
    Ab07,
    #[serde(rename = "AB08")]
    Ab08,
    #[serde(rename = "AB09")]
    Ab09,
    #[serde(rename = "AB10")]
    Ab10,
    #[serde(rename = "AB11")]
    Ab11,
    #[serde(rename = "AC01")]
    Ac01,
    #[serde(rename = "AC02")]
    Ac02,
    #[serde(rename = "AC03")]
    Ac03,
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "AC05")]
    Ac05,
    #[serde(rename = "AC06")]
    Ac06,
    #[serde(rename = "AC07")]
    Ac07,
    #[serde(rename = "AC08")]
    Ac08,
    #[serde(rename = "AC09")]
    Ac09,
    #[serde(rename = "AC10")]
    Ac10,
    #[serde(rename = "AC11")]
    Ac11,
    #[serde(rename = "AC12")]
    Ac12,
    #[serde(rename = "AC13")]
    Ac13,
    #[serde(rename = "AC14")]
    Ac14,
    #[serde(rename = "AC15")]
    Ac15,
    #[serde(rename = "AC16")]
    Ac16,
    #[serde(rename = "AG01")]
    Ag01,
    #[serde(rename = "AG02")]
    Ag02,
    #[serde(rename = "AG03")]
    Ag03,
    #[serde(rename = "AG04")]
    Ag04,
    #[serde(rename = "AG05")]
    Ag05,
    #[serde(rename = "AG06")]
    Ag06,
    #[serde(rename = "AG07")]
    Ag07,
    #[serde(rename = "AG08")]
    Ag08,
    #[serde(rename = "AG09")]
    Ag09,
    #[serde(rename = "AG10")]
    Ag10,
    #[serde(rename = "AG11")]
    Ag11,
    #[serde(rename = "AG12")]
    Ag12,
    #[serde(rename = "AG13")]
    Ag13,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "AM01")]
    Am01,
    #[serde(rename = "AM02")]
    Am02,
    #[serde(rename = "AM03")]
    Am03,
    #[serde(rename = "AM04")]
    Am04,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "AM06")]
    Am06,
    #[serde(rename = "AM07")]
    Am07,
    #[serde(rename = "AM09")]
    Am09,
    #[serde(rename = "AM10")]
    Am10,
    #[serde(rename = "AM11")]
    Am11,
    #[serde(rename = "AM12")]
    Am12,
    #[serde(rename = "AM13")]
    Am13,
    #[serde(rename = "AM14")]
    Am14,
    #[serde(rename = "AM15")]
    Am15,
    #[serde(rename = "AM16")]
    Am16,
    #[serde(rename = "AM17")]
    Am17,
    #[serde(rename = "AM18")]
    Am18,
    #[serde(rename = "AM19")]
    Am19,
    #[serde(rename = "AM20")]
    Am20,
    #[serde(rename = "AM21")]
    Am21,
    #[serde(rename = "AM22")]
    Am22,
    #[serde(rename = "AM23")]
    Am23,
    #[serde(rename = "BE01")]
    Be01,
    #[serde(rename = "BE04")]
    Be04,
    #[serde(rename = "BE05")]
    Be05,
    #[serde(rename = "BE06")]
    Be06,
    #[serde(rename = "BE07")]
    Be07,
    #[serde(rename = "BE08")]
    Be08,
    #[serde(rename = "BE09")]
    Be09,
    #[serde(rename = "BE10")]
    Be10,
    #[serde(rename = "BE11")]
    Be11,
    #[serde(rename = "BE12")]
    Be12,
    #[serde(rename = "BE13")]
    Be13,
    #[serde(rename = "BE14")]
    Be14,
    #[serde(rename = "BE15")]
    Be15,
    #[serde(rename = "BE16")]
    Be16,
    #[serde(rename = "BE17")]
    Be17,
    #[serde(rename = "BE18")]
    Be18,
    #[serde(rename = "BE19")]
    Be19,
    #[serde(rename = "BE20")]
    Be20,
    #[serde(rename = "BE21")]
    Be21,
    #[serde(rename = "BE22")]
    Be22,
    #[serde(rename = "BE23")]
    Be23,
    #[serde(rename = "CERI")]
    Ceri,
    #[serde(rename = "CH03")]
    Ch03,
    #[serde(rename = "CH04")]
    Ch04,
    #[serde(rename = "CH07")]
    Ch07,
    #[serde(rename = "CH09")]
    Ch09,
    #[serde(rename = "CH10")]
    Ch10,
    #[serde(rename = "CH11")]
    Ch11,
    #[serde(rename = "CH12")]
    Ch12,
    #[serde(rename = "CH13")]
    Ch13,
    #[serde(rename = "CH14")]
    Ch14,
    #[serde(rename = "CH15")]
    Ch15,
    #[serde(rename = "CH16")]
    Ch16,
    #[serde(rename = "CH17")]
    Ch17,
    #[serde(rename = "CH19")]
    Ch19,
    #[serde(rename = "CH20")]
    Ch20,
    #[serde(rename = "CH21")]
    Ch21,
    #[serde(rename = "CH22")]
    Ch22,
    #[serde(rename = "CHQC")]
    Chqc,
    #[serde(rename = "CNOR")]
    Cnor,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DNOR")]
    Dnor,
    #[serde(rename = "DS01")]
    Ds01,
    #[serde(rename = "DS02")]
    Ds02,
    #[serde(rename = "DS03")]
    Ds03,
    #[serde(rename = "DS04")]
    Ds04,
    #[serde(rename = "DS05")]
    Ds05,
    #[serde(rename = "DS06")]
    Ds06,
    #[serde(rename = "DS07")]
    Ds07,
    #[serde(rename = "DS08")]
    Ds08,
    #[serde(rename = "DS09")]
    Ds09,
    #[serde(rename = "DS0A")]
    Ds0A,
    #[serde(rename = "DS0B")]
    Ds0B,
    #[serde(rename = "DS0C")]
    Ds0C,
    #[serde(rename = "DS0D")]
    Ds0D,
    #[serde(rename = "DS0E")]
    Ds0E,
    #[serde(rename = "DS0F")]
    Ds0F,
    #[serde(rename = "DS0G")]
    Ds0G,
    #[serde(rename = "DS0H")]
    Ds0H,
    #[serde(rename = "DS0K")]
    Ds0K,
    #[serde(rename = "DS10")]
    Ds10,
    #[serde(rename = "DS11")]
    Ds11,
    #[serde(rename = "DS12")]
    Ds12,
    #[serde(rename = "DS13")]
    Ds13,
    #[serde(rename = "DS14")]
    Ds14,
    #[serde(rename = "DS15")]
    Ds15,
    #[serde(rename = "DS16")]
    Ds16,
    #[serde(rename = "DS17")]
    Ds17,
    #[serde(rename = "DS18")]
    Ds18,
    #[serde(rename = "DS19")]
    Ds19,
    #[serde(rename = "DS20")]
    Ds20,
    #[serde(rename = "DS21")]
    Ds21,
    #[serde(rename = "DS22")]
    Ds22,
    #[serde(rename = "DS23")]
    Ds23,
    #[serde(rename = "DS24")]
    Ds24,
    #[serde(rename = "DS25")]
    Ds25,
    #[serde(rename = "DS26")]
    Ds26,
    #[serde(rename = "DS27")]
    Ds27,
    #[serde(rename = "DT01")]
    Dt01,
    #[serde(rename = "DT02")]
    Dt02,
    #[serde(rename = "DT03")]
    Dt03,
    #[serde(rename = "DT04")]
    Dt04,
    #[serde(rename = "DT05")]
    Dt05,
    #[serde(rename = "DT06")]
    Dt06,
    #[serde(rename = "DU01")]
    Du01,
    #[serde(rename = "DU02")]
    Du02,
    #[serde(rename = "DU03")]
    Du03,
    #[serde(rename = "DU04")]
    Du04,
    #[serde(rename = "DU05")]
    Du05,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "ED01")]
    Ed01,
    #[serde(rename = "ED03")]
    Ed03,
    #[serde(rename = "ED05")]
    Ed05,
    #[serde(rename = "ED06")]
    Ed06,
    #[serde(rename = "ERIN")]
    Erin,
    #[serde(rename = "FF01")]
    Ff01,
    #[serde(rename = "FF02")]
    Ff02,
    #[serde(rename = "FF03")]
    Ff03,
    #[serde(rename = "FF04")]
    Ff04,
    #[serde(rename = "FF05")]
    Ff05,
    #[serde(rename = "FF06")]
    Ff06,
    #[serde(rename = "FF07")]
    Ff07,
    #[serde(rename = "FF08")]
    Ff08,
    #[serde(rename = "FF09")]
    Ff09,
    #[serde(rename = "FF10")]
    Ff10,
    #[serde(rename = "FF11")]
    Ff11,
    #[serde(rename = "G000")]
    G000,
    #[serde(rename = "G001")]
    G001,
    #[serde(rename = "G002")]
    G002,
    #[serde(rename = "G003")]
    G003,
    #[serde(rename = "G004")]
    G004,
    #[serde(rename = "G005")]
    G005,
    #[serde(rename = "G006")]
    G006,
    #[serde(rename = "ID01")]
    Id01,
    #[serde(rename = "MD01")]
    Md01,
    #[serde(rename = "MD02")]
    Md02,
    #[serde(rename = "MD05")]
    Md05,
    #[serde(rename = "MD06")]
    Md06,
    #[serde(rename = "MD07")]
    Md07,
    #[serde(rename = "MS02")]
    Ms02,
    #[serde(rename = "MS03")]
    Ms03,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "NERI")]
    Neri,
    #[serde(rename = "RC01")]
    Rc01,
    #[serde(rename = "RC02")]
    Rc02,
    #[serde(rename = "RC03")]
    Rc03,
    #[serde(rename = "RC04")]
    Rc04,
    #[serde(rename = "RC05")]
    Rc05,
    #[serde(rename = "RC06")]
    Rc06,
    #[serde(rename = "RC07")]
    Rc07,
    #[serde(rename = "RC08")]
    Rc08,
    #[serde(rename = "RC09")]
    Rc09,
    #[serde(rename = "RC10")]
    Rc10,
    #[serde(rename = "RC11")]
    Rc11,
    #[serde(rename = "RC12")]
    Rc12,
    #[serde(rename = "RCON")]
    Rcon,
    #[serde(rename = "RECI")]
    Reci,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR01")]
    Rr01,
    #[serde(rename = "RR02")]
    Rr02,
    #[serde(rename = "RR03")]
    Rr03,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "RR05")]
    Rr05,
    #[serde(rename = "RR06")]
    Rr06,
    #[serde(rename = "RR07")]
    Rr07,
    #[serde(rename = "RR08")]
    Rr08,
    #[serde(rename = "RR09")]
    Rr09,
    #[serde(rename = "RR10")]
    Rr10,
    #[serde(rename = "RR11")]
    Rr11,
    #[serde(rename = "RR12")]
    Rr12,
    #[serde(rename = "S000")]
    S000,
    #[serde(rename = "S001")]
    S001,
    #[serde(rename = "S002")]
    S002,
    #[serde(rename = "S003")]
    S003,
    #[serde(rename = "S004")]
    S004,
    #[serde(rename = "SL01")]
    Sl01,
    #[serde(rename = "SL02")]
    Sl02,
    #[serde(rename = "SL03")]
    Sl03,
    #[serde(rename = "SL11")]
    Sl11,
    #[serde(rename = "SL12")]
    Sl12,
    #[serde(rename = "SL13")]
    Sl13,
    #[serde(rename = "SL14")]
    Sl14,
    #[serde(rename = "TA01")]
    Ta01,
    #[serde(rename = "TD01")]
    Td01,
    #[serde(rename = "TD02")]
    Td02,
    #[serde(rename = "TD03")]
    Td03,
    #[serde(rename = "TK01")]
    Tk01,
    #[serde(rename = "TK02")]
    Tk02,
    #[serde(rename = "TK03")]
    Tk03,
    #[serde(rename = "TK09")]
    Tk09,
    #[serde(rename = "TKCM")]
    Tkcm,
    #[serde(rename = "TKSG")]
    Tksg,
    #[serde(rename = "TKSP")]
    Tksp,
    #[serde(rename = "TKVE")]
    Tkve,
    #[serde(rename = "TKXP")]
    Tkxp,
    #[serde(rename = "TM01")]
    Tm01,
    #[serde(rename = "TS01")]
    Ts01,
    #[serde(rename = "TS04")]
    Ts04,
    #[serde(rename = "CN01")]
    Cn01,
    #[serde(rename = "FOCR")]
    Focr,
    #[serde(rename = "FR01")]
    Fr01,
    #[serde(rename = "NOCM")]
    Nocm,
    #[serde(rename = "NOAS")]
    Noas,
    #[serde(rename = "RUTA")]
    Ruta,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "ALAC")]
    Alac,
    #[serde(rename = "AEXR")]
    Aexr,
    #[serde(rename = "ARFR")]
    Arfr,
    #[serde(rename = "ARJR")]
    Arjr,
    #[serde(rename = "ATNS")]
    Atns,
    #[serde(rename = "EDTR")]
    Edtr,
    #[serde(rename = "EDTL")]
    Edtl,
    #[serde(rename = "FRAD")]
    Frad,
    #[serde(rename = "IEDT")]
    Iedt,
    #[serde(rename = "IRNR")]
    Irnr,
    #[serde(rename = "NOAR")]
    Noar,
    #[serde(rename = "NOPG")]
    Nopg,
    #[serde(rename = "NRCH")]
    Nrch,
    #[serde(rename = "RTNS")]
    Rtns,
    #[serde(rename = "REPR")]
    Repr,
    #[serde(rename = "SPII")]
    Spii,
    #[serde(rename = "PINS")]
    Pins,
    #[serde(rename = "UCRD")]
    Ucrd,
    #[serde(rename = "FF12")]
    Ff12,
    #[serde(rename = "FF13")]
    Ff13,
    #[serde(rename = "DC02")]
    Dc02,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalIncoterms1Code {
    #[serde(rename = "CFR")]
    Cfr,
    #[serde(rename = "CIF")]
    Cif,
    #[serde(rename = "CIP")]
    Cip,
    #[serde(rename = "CPT")]
    Cpt,
    #[serde(rename = "DAP")]
    Dap,
    #[serde(rename = "DAT")]
    Dat,
    #[serde(rename = "DDP")]
    Ddp,
    #[serde(rename = "EXW")]
    Exw,
    #[serde(rename = "FAS")]
    Fas,
    #[serde(rename = "FCA")]
    Fca,
    #[serde(rename = "FOB")]
    Fob,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalReportingSource1Code {
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "ARPF")]
    Arpf,
    #[serde(rename = "ARPP")]
    Arpp,
    #[serde(rename = "CTDB")]
    Ctdb,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "DPCS")]
    Dpcs,
    #[serde(rename = "LKBX")]
    Lkbx,
    #[serde(rename = "RCPT")]
    Rcpt,
    #[serde(rename = "MIBO")]
    Mibo,
    #[serde(rename = "PFRE")]
    Pfre,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalEffectiveDateParameter1Code {
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "TRAD")]
    Trad,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalNotificationCancellationReason1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalSystemErrorHandling1Code {
    #[serde(rename = "X020")]
    X020,
    #[serde(rename = "X030")]
    X030,
    #[serde(rename = "X050")]
    X050,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalMandateSetupReason1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentControlRequestType1Code {
    #[serde(rename = "RT01")]
    Rt01,
    #[serde(rename = "RT02")]
    Rt02,
    #[serde(rename = "RT03")]
    Rt03,
    #[serde(rename = "RT04")]
    Rt04,
    #[serde(rename = "RT05")]
    Rt05,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalChequeCancellationReason1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalInformationType1Code {
    #[serde(rename = "INST")]
    Inst,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "RELY")]
    Rely,
    #[serde(rename = "SHPG")]
    Shpg,
    #[serde(rename = "SHPM")]
    Shpm,
    #[serde(rename = "SLDC")]
    Sldc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalRatesAndTenors1Code {
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "ESOI")]
    Esoi,
    #[serde(rename = "ESTR")]
    Estr,
    #[serde(rename = "GBPO")]
    Gbpo,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SOFR")]
    Sofr,
    #[serde(rename = "USOI")]
    Usoi,
    #[serde(rename = "USPO")]
    Uspo,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalInstructedAgentInstruction1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPendingProcessingReason1Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "ESCA")]
    Esca,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "LPRO")]
    Lpro,
    #[serde(rename = "MINF")]
    Minf,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "PRSY")]
    Prsy,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalEntryStatus1Code {
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "PDNG")]
    Pdng,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalServiceLevel1Code {
    #[serde(rename = "BKTR")]
    Bktr,
    #[serde(rename = "G001")]
    G001,
    #[serde(rename = "G002")]
    G002,
    #[serde(rename = "G003")]
    G003,
    #[serde(rename = "G004")]
    G004,
    #[serde(rename = "NPCA")]
    Npca,
    #[serde(rename = "NUGP")]
    Nugp,
    #[serde(rename = "NURG")]
    Nurg,
    #[serde(rename = "PRPT")]
    Prpt,
    #[serde(rename = "SDVA")]
    Sdva,
    #[serde(rename = "SEPA")]
    Sepa,
    #[serde(rename = "SVDE")]
    Svde,
    #[serde(rename = "URGP")]
    Urgp,
    #[serde(rename = "URNS")]
    Urns,
    #[serde(rename = "INST")]
    Inst,
    #[serde(rename = "SRTP")]
    Srtp,
    #[serde(rename = "SVAT")]
    Svat,
    #[serde(rename = "G006")]
    G006,
    #[serde(rename = "G007")]
    G007,
    #[serde(rename = "G005")]
    G005,
    #[serde(rename = "G009")]
    G009,
    #[serde(rename = "WFSM")]
    Wfsm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingDocumentType2Code {
    #[serde(rename = "BENS")]
    Bens,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "TRAF")]
    Traf,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalContractBalanceType1Code {
    #[serde(rename = "EXPC")]
    Expc,
    #[serde(rename = "EXPT")]
    Expt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAcceptedReason1Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "SMPG")]
    Smpg,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUnitOfMeasure1Code {
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "ACRE")]
    Acre,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "ALOW")]
    Alow,
    #[serde(rename = "ACCY")]
    Accy,
    #[serde(rename = "BARL")]
    Barl,
    #[serde(rename = "BCUF")]
    Bcuf,
    #[serde(rename = "BDFT")]
    Bdft,
    #[serde(rename = "BUSL")]
    Busl,
    #[serde(rename = "CEER")]
    Ceer,
    #[serde(rename = "CLRT")]
    Clrt,
    #[serde(rename = "CBME")]
    Cbme,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "DMET")]
    Dmet,
    #[serde(rename = "ENVC")]
    Envc,
    #[serde(rename = "ENVO")]
    Envo,
    #[serde(rename = "HUWG")]
    Huwg,
    #[serde(rename = "KWDC")]
    Kwdc,
    #[serde(rename = "KWHO")]
    Kwho,
    #[serde(rename = "KWHC")]
    Kwhc,
    #[serde(rename = "KMOC")]
    Kmoc,
    #[serde(rename = "KWMC")]
    Kwmc,
    #[serde(rename = "KWYC")]
    Kwyc,
    #[serde(rename = "MWDC")]
    Mwdc,
    #[serde(rename = "MWHO")]
    Mwho,
    #[serde(rename = "MWHC")]
    Mwhc,
    #[serde(rename = "MWMC")]
    Mwmc,
    #[serde(rename = "MMOC")]
    Mmoc,
    #[serde(rename = "MWYC")]
    Mwyc,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "MIBA")]
    Miba,
    #[serde(rename = "MBTU")]
    Mbtu,
    #[serde(rename = "OZTR")]
    Oztr,
    #[serde(rename = "UCWT")]
    Ucwt,
    #[serde(rename = "IPNT")]
    Ipnt,
    #[serde(rename = "PWRD")]
    Pwrd,
    #[serde(rename = "DGEU")]
    Dgeu,
    #[serde(rename = "GGEU")]
    Ggeu,
    #[serde(rename = "TOCD")]
    Tocd,
    #[serde(rename = "SHAS")]
    Shas,
    #[serde(rename = "THMS")]
    Thms,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "GWHO")]
    Gwho,
    #[serde(rename = "BRTU")]
    Brtu,
    #[serde(rename = "LOTS")]
    Lots,
    #[serde(rename = "BAGG")]
    Bagg,
    #[serde(rename = "BALE")]
    Bale,
    #[serde(rename = "BOTL")]
    Botl,
    #[serde(rename = "BOXX")]
    Boxx,
    #[serde(rename = "CRTN")]
    Crtn,
    #[serde(rename = "CNTR")]
    Cntr,
    #[serde(rename = "CRAT")]
    Crat,
    #[serde(rename = "CBIN")]
    Cbin,
    #[serde(rename = "CBML")]
    Cbml,
    #[serde(rename = "GBFO")]
    Gbfo,
    #[serde(rename = "GBTN")]
    Gbtn,
    #[serde(rename = "USBA")]
    Usba,
    #[serde(rename = "USFO")]
    Usfo,
    #[serde(rename = "USTN")]
    Ustn,
    #[serde(rename = "CDDA")]
    Cdda,
    #[serde(rename = "HDDA")]
    Hdda,
    #[serde(rename = "CPDA")]
    Cpda,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentCancellationRejection1Code {
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "ADAC")]
    Adac,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "AM04")]
    Am04,
    #[serde(rename = "ARDT")]
    Ardt,
    #[serde(rename = "ARPL")]
    Arpl,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "LEGL")]
    Legl,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "NOAS")]
    Noas,
    #[serde(rename = "NOOR")]
    Noor,
    #[serde(rename = "PTNA")]
    Ptna,
    #[serde(rename = "RQDA")]
    Rqda,
    #[serde(rename = "WSEQ")]
    Wseq,
    #[serde(rename = "IDMN")]
    Idmn,
    #[serde(rename = "ACLR")]
    Aclr,
    #[serde(rename = "AEXR")]
    Aexr,
    #[serde(rename = "ARFR")]
    Arfr,
    #[serde(rename = "ARJR")]
    Arjr,
    #[serde(rename = "PATE")]
    Pate,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "RCAR")]
    Rcar,
    #[serde(rename = "RCNR")]
    Rcnr,
    #[serde(rename = "RCPR")]
    Rcpr,
    #[serde(rename = "URTP")]
    Urtp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCardTransactionCategory1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
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
pub struct ExternalBankTransactionDomain1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBalanceType1Code {
    #[serde(rename = "CLAV")]
    Clav,
    #[serde(rename = "CLBD")]
    Clbd,
    #[serde(rename = "FWAV")]
    Fwav,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "ITAV")]
    Itav,
    #[serde(rename = "ITBD")]
    Itbd,
    #[serde(rename = "OPAV")]
    Opav,
    #[serde(rename = "OPBD")]
    Opbd,
    #[serde(rename = "PRCD")]
    Prcd,
    #[serde(rename = "XPCD")]
    Xpcd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalEnquiryRequestType1Code {
    #[serde(rename = "RT11")]
    Rt11,
    #[serde(rename = "RT12")]
    Rt12,
    #[serde(rename = "RT13")]
    Rt13,
    #[serde(rename = "RT14")]
    Rt14,
    #[serde(rename = "RT16")]
    Rt16,
    #[serde(rename = "RT15")]
    Rt15,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalGarnishmentType1Code {
    #[serde(rename = "GNCS")]
    Gncs,
    #[serde(rename = "GNDP")]
    Gndp,
    #[serde(rename = "GTPP")]
    Gtpp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalLocalInstrument1Code {
    #[serde(rename = "DDMC")]
    Ddmc,
    #[serde(rename = "DDMP")]
    Ddmp,
    #[serde(rename = "DDMU")]
    Ddmu,
    #[serde(rename = "BPA")]
    Bpa,
    #[serde(rename = "IPA")]
    Ipa,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "82")]
    X82,
    #[serde(rename = "83")]
    X83,
    #[serde(rename = "CPP")]
    Cpp,
    #[serde(rename = "RTR")]
    Rtr,
    #[serde(rename = "GST")]
    Gst,
    #[serde(rename = "DDT")]
    Ddt,
    #[serde(rename = "RDD")]
    Rdd,
    #[serde(rename = "CHN")]
    Chn,
    #[serde(rename = "STR")]
    Str,
    #[serde(rename = "SDD")]
    Sdd,
    #[serde(rename = "SRT")]
    Srt,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "SCN")]
    Scn,
    #[serde(rename = "SGT")]
    Sgt,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "05")]
    X05,
    #[serde(rename = "04")]
    X04,
    #[serde(rename = "ISE")]
    Ise,
    #[serde(rename = "BSE")]
    Bse,
    #[serde(rename = "58")]
    X58,
    #[serde(rename = "19")]
    X19,
    #[serde(rename = "ASTI")]
    Asti,
    #[serde(rename = "BACP")]
    Bacp,
    #[serde(rename = "MANP")]
    Manp,
    #[serde(rename = "SBTI")]
    Sbti,
    #[serde(rename = "85")]
    X85,
    #[serde(rename = "08")]
    X08,
    #[serde(rename = "89")]
    X89,
    #[serde(rename = "60")]
    X60,
    #[serde(rename = "RIBA")]
    Riba,
    #[serde(rename = "RIDO")]
    Rido,
    #[serde(rename = "RIDV")]
    Ridv,
    #[serde(rename = "IDEAL")]
    Ideal,
    #[serde(rename = "INSTNT01")]
    Instnt01,
    #[serde(rename = "INSTTC01")]
    Insttc01,
    #[serde(rename = "INSTIDEAL")]
    Instideal,
    #[serde(rename = "INSTNT01IDEAL")]
    Instnt01Ideal,
    #[serde(rename = "INSTTC01IDEAL")]
    Insttc01Ideal,
    #[serde(rename = "NLDO")]
    Nldo,
    #[serde(rename = "NLUP")]
    Nlup,
    #[serde(rename = "SDN")]
    Sdn,
    #[serde(rename = "ACCEPT")]
    Accept,
    #[serde(rename = "ICMC")]
    Icmc,
    #[serde(rename = "NLGOV")]
    Nlgov,
    #[serde(rename = "0090")]
    X0090,
    #[serde(rename = "0091")]
    X0091,
    #[serde(rename = "0092")]
    X0092,
    #[serde(rename = "0002")]
    X0002,
    #[serde(rename = "0221")]
    X0221,
    #[serde(rename = "0224")]
    X0224,
    #[serde(rename = "0226")]
    X0226,
    #[serde(rename = "0225")]
    X0225,
    #[serde(rename = "0222")]
    X0222,
    #[serde(rename = "0227")]
    X0227,
    #[serde(rename = "0220")]
    X0220,
    #[serde(rename = "0223")]
    X0223,
    #[serde(rename = "0001")]
    X0001,
    #[serde(rename = "0000")]
    X0000,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "ONCL")]
    Oncl,
    #[serde(rename = "PERI")]
    Peri,
    #[serde(rename = "SDCL")]
    Sdcl,
    #[serde(rename = "DDNR")]
    Ddnr,
    #[serde(rename = "DDFA")]
    Ddfa,
    #[serde(rename = "CORE")]
    Core,
    #[serde(rename = "B2BAMIPM")]
    B2Bamipm,
    #[serde(rename = "B2B")]
    B2B,
    #[serde(rename = "CR1AMIPM")]
    Cr1Amipm,
    #[serde(rename = "CORAMIPM")]
    Coramipm,
    #[serde(rename = "COR1")]
    Cor1,
    #[serde(rename = "FADAMIPM")]
    Fadamipm,
    #[serde(rename = "CLSCCPERX")]
    Clsccperx,
    #[serde(rename = "CLSCCPLCH")]
    Clsccplch,
    #[serde(rename = "INST")]
    Inst,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "UDD")]
    Udd,
    #[serde(rename = "CCI")]
    Cci,
    #[serde(rename = "BTR")]
    Btr,
    #[serde(rename = "CKS")]
    Cks,
    #[serde(rename = "CTR")]
    Ctr,
    #[serde(rename = "CTP")]
    Ctp,
    #[serde(rename = "DEP")]
    Dep,
    #[serde(rename = "FFR")]
    Ffr,
    #[serde(rename = "FFS")]
    Ffs,
    #[serde(rename = "SVC")]
    Svc,
    #[serde(rename = "DRW")]
    Drw,
    #[serde(rename = "DRB")]
    Drb,
    #[serde(rename = "DRC")]
    Drc,
    #[serde(rename = "IAT")]
    Iat,
    #[serde(rename = "CCD")]
    Ccd,
    #[serde(rename = "CTX")]
    Ctx,
    #[serde(rename = "PPD")]
    Ppd,
    #[serde(rename = "CIE")]
    Cie,
    #[serde(rename = "RCK")]
    Rck,
    #[serde(rename = "ARC")]
    Arc,
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "POP")]
    Pop,
    #[serde(rename = "POS")]
    Pos,
    #[serde(rename = "TEL")]
    Tel,
    #[serde(rename = "ITP")]
    Itp,
    #[serde(rename = "MDP")]
    Mdp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPartyRelationshipType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
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
pub struct ExternalCommunicationFormat1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDocumentLineType1Code {
    #[serde(rename = "ADPI")]
    Adpi,
    #[serde(rename = "AISB")]
    Aisb,
    #[serde(rename = "ASNB")]
    Asnb,
    #[serde(rename = "CTNB")]
    Ctnb,
    #[serde(rename = "DBSP")]
    Dbsp,
    #[serde(rename = "EANN")]
    Eann,
    #[serde(rename = "EINB")]
    Einb,
    #[serde(rename = "GSNB")]
    Gsnb,
    #[serde(rename = "HIBC")]
    Hibc,
    #[serde(rename = "ISBN")]
    Isbn,
    #[serde(rename = "LTNB")]
    Ltnb,
    #[serde(rename = "MDNB")]
    Mdnb,
    #[serde(rename = "PRNB")]
    Prnb,
    #[serde(rename = "PTCD")]
    Ptcd,
    #[serde(rename = "SKNB")]
    Sknb,
    #[serde(rename = "STNB")]
    Stnb,
    #[serde(rename = "TONB")]
    Tonb,
    #[serde(rename = "UPCC")]
    Upcc,
    #[serde(rename = "UPNB")]
    Upnb,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalTradeMarket1Code {
    #[serde(rename = "FDMS")]
    Fdms,
    #[serde(rename = "FEXP")]
    Fexp,
    #[serde(rename = "FFDM")]
    Ffdm,
    #[serde(rename = "FFDT")]
    Ffdt,
    #[serde(rename = "FIMP")]
    Fimp,
    #[serde(rename = "FREX")]
    Frex,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingAmountType1Code {
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "REDC")]
    Redc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDebtorActivationCancellationReason1Code {
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "TRJT")]
    Trjt,
    #[serde(rename = "UCRD")]
    Ucrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUnableToApplyIncorrectData1Code {
    #[serde(rename = "IN01")]
    In01,
    #[serde(rename = "IN02")]
    In02,
    #[serde(rename = "IN03")]
    In03,
    #[serde(rename = "IN04")]
    In04,
    #[serde(rename = "IN05")]
    In05,
    #[serde(rename = "IN06")]
    In06,
    #[serde(rename = "IN07")]
    In07,
    #[serde(rename = "IN08")]
    In08,
    #[serde(rename = "IN09")]
    In09,
    #[serde(rename = "IN10")]
    In10,
    #[serde(rename = "IN11")]
    In11,
    #[serde(rename = "IN12")]
    In12,
    #[serde(rename = "IN13")]
    In13,
    #[serde(rename = "IN14")]
    In14,
    #[serde(rename = "IN15")]
    In15,
    #[serde(rename = "IN16")]
    In16,
    #[serde(rename = "IN17")]
    In17,
    #[serde(rename = "IN18")]
    In18,
    #[serde(rename = "IN19")]
    In19,
    #[serde(rename = "IN33")]
    In33,
    #[serde(rename = "IN36")]
    In36,
    #[serde(rename = "IN37")]
    In37,
    #[serde(rename = "IN38")]
    In38,
    #[serde(rename = "IN39")]
    In39,
    #[serde(rename = "IN40")]
    In40,
    #[serde(rename = "MM20")]
    Mm20,
    #[serde(rename = "MM21")]
    Mm21,
    #[serde(rename = "MM22")]
    Mm22,
    #[serde(rename = "MM23")]
    Mm23,
    #[serde(rename = "MM24")]
    Mm24,
    #[serde(rename = "MM25")]
    Mm25,
    #[serde(rename = "MM26")]
    Mm26,
    #[serde(rename = "MM27")]
    Mm27,
    #[serde(rename = "MM28")]
    Mm28,
    #[serde(rename = "MM29")]
    Mm29,
    #[serde(rename = "MM30")]
    Mm30,
    #[serde(rename = "MM31")]
    Mm31,
    #[serde(rename = "MM32")]
    Mm32,
    #[serde(rename = "MM34")]
    Mm34,
    #[serde(rename = "MM35")]
    Mm35,
    #[serde(rename = "NARR")]
    Narr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalRejectedReason1Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "BDAY")]
    Bday,
    #[serde(rename = "CTRC")]
    Ctrc,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "INDT")]
    Indt,
    #[serde(rename = "MISM")]
    Mism,
    #[serde(rename = "NAUT")]
    Naut,
    #[serde(rename = "NINS")]
    Nins,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "ASBR")]
    Asbr,
    #[serde(rename = "AB15")]
    Ab15,
    #[serde(rename = "AB26")]
    Ab26,
    #[serde(rename = "AB12")]
    Ab12,
    #[serde(rename = "DT07")]
    Dt07,
    #[serde(rename = "AB13")]
    Ab13,
    #[serde(rename = "AB21")]
    Ab21,
    #[serde(rename = "ISWS")]
    Isws,
    #[serde(rename = "MISN")]
    Misn,
    #[serde(rename = "NOFR")]
    Nofr,
    #[serde(rename = "RC14")]
    Rc14,
    #[serde(rename = "RC16")]
    Rc16,
    #[serde(rename = "RC15")]
    Rc15,
    #[serde(rename = "RC13")]
    Rc13,
    #[serde(rename = "SBRN")]
    Sbrn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDocumentPurpose1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "FINV")]
    Finv,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDiscrepancy1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
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
pub struct ExternalBankTransactionFamily1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalLetterType1Code {
    #[serde(rename = "LFBK")]
    Lfbk,
    #[serde(rename = "LTBK")]
    Ltbk,
    #[serde(rename = "SUPP")]
    Supp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalNarrativeType1Code {
    #[serde(rename = "ADDI")]
    Addi,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "PRAS")]
    Pras,
    #[serde(rename = "TEFF")]
    Teff,
    #[serde(rename = "TRNF")]
    Trnf,
    #[serde(rename = "TVAR")]
    Tvar,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalSystemEventType1Code {
    #[serde(rename = "CRCO")]
    Crco,
    #[serde(rename = "CUSC")]
    Cusc,
    #[serde(rename = "ESTF")]
    Estf,
    #[serde(rename = "EUCO")]
    Euco,
    #[serde(rename = "EUSU")]
    Eusu,
    #[serde(rename = "FIRE")]
    Fire,
    #[serde(rename = "IBKC")]
    Ibkc,
    #[serde(rename = "LTDC")]
    Ltdc,
    #[serde(rename = "LTGC")]
    Ltgc,
    #[serde(rename = "LTNC")]
    Ltnc,
    #[serde(rename = "LVCC")]
    Lvcc,
    #[serde(rename = "LVCO")]
    Lvco,
    #[serde(rename = "LVRT")]
    Lvrt,
    #[serde(rename = "LWSU")]
    Lwsu,
    #[serde(rename = "NPCT")]
    Npct,
    #[serde(rename = "PCOT")]
    Pcot,
    #[serde(rename = "RECC")]
    Recc,
    #[serde(rename = "REOP")]
    Reop,
    #[serde(rename = "SSSC")]
    Sssc,
    #[serde(rename = "STDY")]
    Stdy,
    #[serde(rename = "STSU")]
    Stsu,
    #[serde(rename = "SYSC")]
    Sysc,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingStatusCategory1Code {
    #[serde(rename = "AMND")]
    Amnd,
    #[serde(rename = "AMTC")]
    Amtc,
    #[serde(rename = "AMTU")]
    Amtu,
    #[serde(rename = "APPL")]
    Appl,
    #[serde(rename = "BAMD")]
    Bamd,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "DEMD")]
    Demd,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "TERM")]
    Term,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalFinancialInstrumentIdentificationType1Code {
    #[serde(rename = "BELC")]
    Belc,
    #[serde(rename = "BLOM")]
    Blom,
    #[serde(rename = "CCCD")]
    Cccd,
    #[serde(rename = "CMED")]
    Cmed,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CTAC")]
    Ctac,
    #[serde(rename = "CUSP")]
    Cusp,
    #[serde(rename = "FIGC")]
    Figc,
    #[serde(rename = "FIGG")]
    Figg,
    #[serde(rename = "FIGI")]
    Figi,
    #[serde(rename = "ISDU")]
    Isdu,
    #[serde(rename = "ISDX")]
    Isdx,
    #[serde(rename = "LCHD")]
    Lchd,
    #[serde(rename = "OCCS")]
    Occs,
    #[serde(rename = "OPRA")]
    Opra,
    #[serde(rename = "RCMD")]
    Rcmd,
    #[serde(rename = "RICC")]
    Ricc,
    #[serde(rename = "SEDL")]
    Sedl,
    #[serde(rename = "SICC")]
    Sicc,
    #[serde(rename = "TIKR")]
    Tikr,
    #[serde(rename = "VALO")]
    Valo,
    #[serde(rename = "WKNR")]
    Wknr,
    #[serde(rename = "CCDC")]
    Ccdc,
    #[serde(rename = "DTID")]
    Dtid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalSecuritiesPurpose1Code {
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "SECL")]
    Secl,
    #[serde(rename = "STMT")]
    Stmt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentModificationRejection1Code {
    #[serde(rename = "UM01")]
    Um01,
    #[serde(rename = "UM02")]
    Um02,
    #[serde(rename = "UM03")]
    Um03,
    #[serde(rename = "UM04")]
    Um04,
    #[serde(rename = "UM05")]
    Um05,
    #[serde(rename = "UM06")]
    Um06,
    #[serde(rename = "UM07")]
    Um07,
    #[serde(rename = "UM08")]
    Um08,
    #[serde(rename = "UM09")]
    Um09,
    #[serde(rename = "UM10")]
    Um10,
    #[serde(rename = "UM11")]
    Um11,
    #[serde(rename = "UM12")]
    Um12,
    #[serde(rename = "UM13")]
    Um13,
    #[serde(rename = "UM14")]
    Um14,
    #[serde(rename = "UM15")]
    Um15,
    #[serde(rename = "UM16")]
    Um16,
    #[serde(rename = "UM17")]
    Um17,
    #[serde(rename = "UM18")]
    Um18,
    #[serde(rename = "UM19")]
    Um19,
    #[serde(rename = "UM20")]
    Um20,
    #[serde(rename = "UM21")]
    Um21,
    #[serde(rename = "UM22")]
    Um22,
    #[serde(rename = "UM23")]
    Um23,
    #[serde(rename = "UM24")]
    Um24,
    #[serde(rename = "UM25")]
    Um25,
    #[serde(rename = "UM26")]
    Um26,
    #[serde(rename = "UM27")]
    Um27,
    #[serde(rename = "UM28")]
    Um28,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDebtorAgentInstruction1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUndertakingDocumentType1Code {
    #[serde(rename = "BENS")]
    Bens,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "CLAF")]
    Claf,
    #[serde(rename = "DEMD")]
    Demd,
    #[serde(rename = "TRAF")]
    Traf,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalBalanceSubType1Code {
    #[serde(rename = "ADJT")]
    Adjt,
    #[serde(rename = "BCUR")]
    Bcur,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "BLKD")]
    Blkd,
    #[serde(rename = "DLOD")]
    Dlod,
    #[serde(rename = "EAST")]
    East,
    #[serde(rename = "FCOL")]
    Fcol,
    #[serde(rename = "FCOU")]
    Fcou,
    #[serde(rename = "FORC")]
    Forc,
    #[serde(rename = "FUND")]
    Fund,
    #[serde(rename = "INTM")]
    Intm,
    #[serde(rename = "LCUR")]
    Lcur,
    #[serde(rename = "LRLD")]
    Lrld,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "PIPO")]
    Pipo,
    #[serde(rename = "PRAV")]
    Prav,
    #[serde(rename = "RESV")]
    Resv,
    #[serde(rename = "SCOL")]
    Scol,
    #[serde(rename = "SCOU")]
    Scou,
    #[serde(rename = "THRE")]
    Thre,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalAuthorityIdentification1Code {
    #[serde(rename = "AUAS")]
    Auas,
    #[serde(rename = "ATFM")]
    Atfm,
    #[serde(rename = "BEFS")]
    Befs,
    #[serde(rename = "BENB")]
    Benb,
    #[serde(rename = "BGFS")]
    Bgfs,
    #[serde(rename = "CACS")]
    Cacs,
    #[serde(rename = "CYAS")]
    Cyas,
    #[serde(rename = "CYCB")]
    Cycb,
    #[serde(rename = "CYCY")]
    Cycy,
    #[serde(rename = "CZCN")]
    Czcn,
    #[serde(rename = "DEBA")]
    Deba,
    #[serde(rename = "DKFI")]
    Dkfi,
    #[serde(rename = "EEEF")]
    Eeef,
    #[serde(rename = "EUES")]
    Eues,
    #[serde(rename = "ESBD")]
    Esbd,
    #[serde(rename = "ESCN")]
    Escn,
    #[serde(rename = "FIFS")]
    Fifs,
    #[serde(rename = "FRAC")]
    Frac,
    #[serde(rename = "FRAM")]
    Fram,
    #[serde(rename = "GBFS")]
    Gbfs,
    #[serde(rename = "GBFC")]
    Gbfc,
    #[serde(rename = "GBPR")]
    Gbpr,
    #[serde(rename = "GIFS")]
    Gifs,
    #[serde(rename = "USCF")]
    Uscf,
    #[serde(rename = "GRBO")]
    Grbo,
    #[serde(rename = "GRHC")]
    Grhc,
    #[serde(rename = "HRHA")]
    Hrha,
    #[serde(rename = "HUPS")]
    Hups,
    #[serde(rename = "IECB")]
    Iecb,
    #[serde(rename = "ISFM")]
    Isfm,
    #[serde(rename = "ITBD")]
    Itbd,
    #[serde(rename = "ITCO")]
    Itco,
    #[serde(rename = "LIFM")]
    Lifm,
    #[serde(rename = "LTLS")]
    Ltls,
    #[serde(rename = "LUCS")]
    Lucs,
    #[serde(rename = "LVFK")]
    Lvfk,
    #[serde(rename = "MTMF")]
    Mtmf,
    #[serde(rename = "NLAF")]
    Nlaf,
    #[serde(rename = "NLDN")]
    Nldn,
    #[serde(rename = "NOFI")]
    Nofi,
    #[serde(rename = "PLKN")]
    Plkn,
    #[serde(rename = "PTBP")]
    Ptbp,
    #[serde(rename = "PTCM")]
    Ptcm,
    #[serde(rename = "ROAS")]
    Roas,
    #[serde(rename = "SEFI")]
    Sefi,
    #[serde(rename = "SIAT")]
    Siat,
    #[serde(rename = "SKNB")]
    Sknb,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalMandateReason1Code {
    #[serde(rename = "AC01")]
    Ac01,
    #[serde(rename = "AC04")]
    Ac04,
    #[serde(rename = "AC06")]
    Ac06,
    #[serde(rename = "AG01")]
    Ag01,
    #[serde(rename = "AG02")]
    Ag02,
    #[serde(rename = "AM02")]
    Am02,
    #[serde(rename = "AM03")]
    Am03,
    #[serde(rename = "AM05")]
    Am05,
    #[serde(rename = "BE01")]
    Be01,
    #[serde(rename = "BE04")]
    Be04,
    #[serde(rename = "BE05")]
    Be05,
    #[serde(rename = "BE06")]
    Be06,
    #[serde(rename = "BE07")]
    Be07,
    #[serde(rename = "DT01")]
    Dt01,
    #[serde(rename = "FF01")]
    Ff01,
    #[serde(rename = "MD01")]
    Md01,
    #[serde(rename = "MD02")]
    Md02,
    #[serde(rename = "MD07")]
    Md07,
    #[serde(rename = "MD08")]
    Md08,
    #[serde(rename = "MD09")]
    Md09,
    #[serde(rename = "MD10")]
    Md10,
    #[serde(rename = "MD11")]
    Md11,
    #[serde(rename = "MD12")]
    Md12,
    #[serde(rename = "MD13")]
    Md13,
    #[serde(rename = "MD14")]
    Md14,
    #[serde(rename = "MD15")]
    Md15,
    #[serde(rename = "MD16")]
    Md16,
    #[serde(rename = "MD17")]
    Md17,
    #[serde(rename = "MD18")]
    Md18,
    #[serde(rename = "MD19")]
    Md19,
    #[serde(rename = "MD20")]
    Md20,
    #[serde(rename = "MD21")]
    Md21,
    #[serde(rename = "MD22")]
    Md22,
    #[serde(rename = "MD23")]
    Md23,
    #[serde(rename = "MS02")]
    Ms02,
    #[serde(rename = "MS03")]
    Ms03,
    #[serde(rename = "NARR")]
    Narr,
    #[serde(rename = "RC01")]
    Rc01,
    #[serde(rename = "RF01")]
    Rf01,
    #[serde(rename = "RR01")]
    Rr01,
    #[serde(rename = "RR02")]
    Rr02,
    #[serde(rename = "RR03")]
    Rr03,
    #[serde(rename = "RR04")]
    Rr04,
    #[serde(rename = "SL01")]
    Sl01,
    #[serde(rename = "SL11")]
    Sl11,
    #[serde(rename = "SL12")]
    Sl12,
    #[serde(rename = "SL13")]
    Sl13,
    #[serde(rename = "SL14")]
    Sl14,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalSecuritiesUpdateReason1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalValidationRuleIdentification1Code {
    #[serde(rename = "CCTR")]
    Cctr,
    #[serde(rename = "ITRP")]
    Itrp,
    #[serde(rename = "MMSR")]
    Mmsr,
    #[serde(rename = "SMMD")]
    Smmd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalShipmentCondition1Code {
    #[serde(rename = "ADVN")]
    Advn,
    #[serde(rename = "PMNT")]
    Pmnt,
    #[serde(rename = "TRLN")]
    Trln,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDateFrequency1Code {
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "YEAR")]
    Year,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCreditorAgentInstruction1Code {
    #[serde(rename = "CHQB")]
    Chqb,
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "PHOB")]
    Phob,
    #[serde(rename = "PRTK")]
    Prtk,
    #[serde(rename = "RECI")]
    Reci,
    #[serde(rename = "TELB")]
    Telb,
    #[serde(rename = "TKCM")]
    Tkcm,
    #[serde(rename = "TKSG")]
    Tksg,
    #[serde(rename = "TKSP")]
    Tksp,
    #[serde(rename = "TKVE")]
    Tkve,
    #[serde(rename = "TKXP")]
    Tkxp,
    #[serde(rename = "TOKN")]
    Tokn,
    #[serde(rename = "VLTK")]
    Vltk,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalAuthenticationMethod1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalUnderlyingTradeTransactionType1Code {
    #[serde(rename = "CONT")]
    Cont,
    #[serde(rename = "DELV")]
    Delv,
    #[serde(rename = "PROF")]
    Prof,
    #[serde(rename = "PROJ")]
    Proj,
    #[serde(rename = "PUOR")]
    Puor,
    #[serde(rename = "QUOT")]
    Quot,
    #[serde(rename = "TEND")]
    Tend,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalRePresentmentReason1Code {
    #[serde(rename = "AMCR")]
    Amcr,
    #[serde(rename = "CLSD")]
    Clsd,
    #[serde(rename = "CRPI")]
    Crpi,
    #[serde(rename = "OTVA")]
    Otva,
    #[serde(rename = "VLSD")]
    Vlsd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalInvestigationExecutionConfirmation1Code {
    #[serde(rename = "ACDA")]
    Acda,
    #[serde(rename = "ACNR")]
    Acnr,
    #[serde(rename = "ACVA")]
    Acva,
    #[serde(rename = "CHRG")]
    Chrg,
    #[serde(rename = "CNCL")]
    Cncl,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "CVAA")]
    Cvaa,
    #[serde(rename = "CWFW")]
    Cwfw,
    #[serde(rename = "FTNA")]
    Ftna,
    #[serde(rename = "ICOV")]
    Icov,
    #[serde(rename = "IDUP")]
    Idup,
    #[serde(rename = "IPAY")]
    Ipay,
    #[serde(rename = "IPYI")]
    Ipyi,
    #[serde(rename = "MCOV")]
    Mcov,
    #[serde(rename = "MODI")]
    Modi,
    #[serde(rename = "MWFW")]
    Mwfw,
    #[serde(rename = "PDCR")]
    Pdcr,
    #[serde(rename = "PECR")]
    Pecr,
    #[serde(rename = "PURP")]
    Purp,
    #[serde(rename = "RJCR")]
    Rjcr,
    #[serde(rename = "RJNR")]
    Rjnr,
    #[serde(rename = "RJVA")]
    Rjva,
    #[serde(rename = "SMTC")]
    Smtc,
    #[serde(rename = "SMTI")]
    Smti,
    #[serde(rename = "UWFW")]
    Uwfw,
    #[serde(rename = "BIAS")]
    Bias,
    #[serde(rename = "IDNE")]
    Idne,
    #[serde(rename = "IVCR")]
    Ivcr,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "NINF")]
    Ninf,
    #[serde(rename = "PDNG")]
    Pdng,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentGroupStatus1Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACCC")]
    Accc,
    #[serde(rename = "ACSC")]
    Acsc,
    #[serde(rename = "ACSP")]
    Acsp,
    #[serde(rename = "ACTC")]
    Actc,
    #[serde(rename = "ACWC")]
    Acwc,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "RCVD")]
    Rcvd,
    #[serde(rename = "RJCT")]
    Rjct,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDiscountAmountType1Code {
    #[serde(rename = "APDS")]
    Apds,
    #[serde(rename = "STDS")]
    Stds,
    #[serde(rename = "TMDS")]
    Tmds,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalModelFormIdentification1Code {
    #[serde(rename = "ISP1")]
    Isp1,
    #[serde(rename = "ISP2")]
    Isp2,
    #[serde(rename = "ISP3")]
    Isp3,
    #[serde(rename = "ISP4")]
    Isp4,
    #[serde(rename = "ISP5")]
    Isp5,
    #[serde(rename = "ISP6")]
    Isp6,
    #[serde(rename = "ISP7")]
    Isp7,
    #[serde(rename = "ISP8")]
    Isp8,
    #[serde(rename = "UDG1")]
    Udg1,
    #[serde(rename = "UDG2")]
    Udg2,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalReceivedReason1Code {
    #[serde(rename = "NSTP")]
    Nstp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalEncryptedElementIdentification1Code {
    #[serde(rename = "8A")]
    X8A,
    #[serde(rename = "8C")]
    X8C,
    #[serde(rename = "8D")]
    X8D,
    #[serde(rename = "8E")]
    X8E,
    #[serde(rename = "8F")]
    X8F,
    #[serde(rename = "89")]
    X89,
    #[serde(rename = "90")]
    X90,
    #[serde(rename = "91")]
    X91,
    #[serde(rename = "92")]
    X92,
    #[serde(rename = "93")]
    X93,
    #[serde(rename = "94")]
    X94,
    #[serde(rename = "95")]
    X95,
    #[serde(rename = "96")]
    X96,
    #[serde(rename = "97")]
    X97,
    #[serde(rename = "98")]
    X98,
    #[serde(rename = "99")]
    X99,
    #[serde(rename = "9A")]
    X9A,
    #[serde(rename = "9B")]
    X9B,
    #[serde(rename = "9C")]
    X9C,
    #[serde(rename = "9D")]
    X9D,
    #[serde(rename = "9E")]
    X9E,
    #[serde(rename = "9F1F")]
    X9F1F,
    #[serde(rename = "9F20")]
    X9F20,
    #[serde(rename = "9F21")]
    X9F21,
    #[serde(rename = "9F22")]
    X9F22,
    #[serde(rename = "9F23")]
    X9F23,
    #[serde(rename = "9F24")]
    X9F24,
    #[serde(rename = "9F25")]
    X9F25,
    #[serde(rename = "9F26")]
    X9F26,
    #[serde(rename = "9F27")]
    X9F27,
    #[serde(rename = "9F28")]
    X9F28,
    #[serde(rename = "9F29")]
    X9F29,
    #[serde(rename = "9F2A")]
    X9F2A,
    #[serde(rename = "9F2B")]
    X9F2B,
    #[serde(rename = "9F2C")]
    X9F2C,
    #[serde(rename = "9F2D")]
    X9F2D,
    #[serde(rename = "9F2E")]
    X9F2E,
    #[serde(rename = "9F2F")]
    X9F2F,
    #[serde(rename = "9F30")]
    X9F30,
    #[serde(rename = "9F31")]
    X9F31,
    #[serde(rename = "9F32")]
    X9F32,
    #[serde(rename = "9F33")]
    X9F33,
    #[serde(rename = "9F34")]
    X9F34,
    #[serde(rename = "9F35")]
    X9F35,
    #[serde(rename = "9F36")]
    X9F36,
    #[serde(rename = "9F37")]
    X9F37,
    #[serde(rename = "9F38")]
    X9F38,
    #[serde(rename = "9F39")]
    X9F39,
    #[serde(rename = "9F3A")]
    X9F3A,
    #[serde(rename = "9F3B")]
    X9F3B,
    #[serde(rename = "9F3C")]
    X9F3C,
    #[serde(rename = "9F3D")]
    X9F3D,
    #[serde(rename = "9F3E")]
    X9F3E,
    #[serde(rename = "9F3F")]
    X9F3F,
    #[serde(rename = "9F40")]
    X9F40,
    #[serde(rename = "9F41")]
    X9F41,
    #[serde(rename = "9F42")]
    X9F42,
    #[serde(rename = "9F43")]
    X9F43,
    #[serde(rename = "9F44")]
    X9F44,
    #[serde(rename = "9F45")]
    X9F45,
    #[serde(rename = "9F46")]
    X9F46,
    #[serde(rename = "9F47")]
    X9F47,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalCashAccountType1Code {
    #[serde(rename = "CACC")]
    Cacc,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "CISH")]
    Cish,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CPAC")]
    Cpac,
    #[serde(rename = "LLSV")]
    Llsv,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "MOMA")]
    Moma,
    #[serde(rename = "NREX")]
    Nrex,
    #[serde(rename = "ODFT")]
    Odft,
    #[serde(rename = "ONDP")]
    Ondp,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SACC")]
    Sacc,
    #[serde(rename = "SLRY")]
    Slry,
    #[serde(rename = "SVGS")]
    Svgs,
    #[serde(rename = "TAXE")]
    Taxe,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "TRAS")]
    Tras,
    #[serde(rename = "VACC")]
    Vacc,
    #[serde(rename = "NFCA")]
    Nfca,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalPaymentCompensationReason1Code {
    #[serde(rename = "VADA")]
    Vada,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalTypeOfParty1Code {
    #[serde(rename = "ADVP")]
    Advp,
    #[serde(rename = "ANYB")]
    Anyb,
    #[serde(rename = "APPL")]
    Appl,
    #[serde(rename = "BENE")]
    Bene,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "CUB2")]
    Cub2,
    #[serde(rename = "CUB3")]
    Cub3,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "OBLG")]
    Oblg,
    #[serde(rename = "ORDR")]
    Ordr,
    #[serde(rename = "PRES")]
    Pres,
    #[serde(rename = "SADV")]
    Sadv,
    #[serde(rename = "SPEC")]
    Spec,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalDocumentFormat1Code {
    #[serde(rename = "DPDF")]
    Dpdf,
    #[serde(rename = "DXML")]
    Dxml,
    #[serde(rename = "SDSH")]
    Sdsh,
    #[serde(rename = "WORD")]
    Word,
    #[serde(rename = "XSLT")]
    Xslt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExternalProxyAccountType1Code {
    #[serde(rename = "TELE")]
    Tele,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "DNAM")]
    Dnam,
    #[serde(rename = "CINC")]
    Cinc,
    #[serde(rename = "COTX")]
    Cotx,
    #[serde(rename = "COID")]
    Coid,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "EIDN")]
    Eidn,
    #[serde(rename = "EWAL")]
    Ewal,
    #[serde(rename = "PVTX")]
    Pvtx,
    #[serde(rename = "LEIC")]
    Leic,
    #[serde(rename = "MBNO")]
    Mbno,
    #[serde(rename = "NIDN")]
    Nidn,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "SHID")]
    Shid,
    #[serde(rename = "SOSE")]
    Sose,
    #[serde(rename = "TOKN")]
    Tokn,
    #[serde(rename = "UBIL")]
    Ubil,
    #[serde(rename = "VIPN")]
    Vipn,
    #[serde(rename = "BIID")]
    Biid,
    #[default]
    Unknown,
}
