#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - argument register"]
    pub arg: ARG,
    #[doc = "0x0c - command register"]
    pub cmd: CMD,
    #[doc = "0x10 - command response register"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - response 1..4 register"]
    pub resp1: RESP1,
    #[doc = "0x18 - response 1..4 register"]
    pub resp2: RESP2,
    #[doc = "0x1c - response 1..4 register"]
    pub resp3: RESP3,
    #[doc = "0x20 - response 1..4 register"]
    pub resp4: RESP4,
    #[doc = "0x24 - data timer register"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - data length register"]
    pub dlen: DLEN,
    #[doc = "0x2c - data control register"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - data counter register"]
    pub dcount: DCOUNT,
    #[doc = "0x34 - status register"]
    pub sta: STA,
    #[doc = "0x38 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x3c - mask register"]
    pub mask: MASK,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - data FIFO register"]
    pub fifo: FIFO,
}
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "power control register"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "argument register"]
pub mod arg;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "command register"]
pub mod cmd;
#[doc = "RESPCMD (r) register accessor: an alias for `Reg<RESPCMD_SPEC>`"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
#[doc = "command response register"]
pub mod respcmd;
#[doc = "RESP1 (r) register accessor: an alias for `Reg<RESP1_SPEC>`"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp3;
#[doc = "RESP4 (r) register accessor: an alias for `Reg<RESP4_SPEC>`"]
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp4;
#[doc = "DTIMER (rw) register accessor: an alias for `Reg<DTIMER_SPEC>`"]
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "data length register"]
pub mod dlen;
#[doc = "DCTRL (rw) register accessor: an alias for `Reg<DCTRL_SPEC>`"]
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
#[doc = "data control register"]
pub mod dctrl;
#[doc = "DCOUNT (r) register accessor: an alias for `Reg<DCOUNT_SPEC>`"]
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
#[doc = "data counter register"]
pub mod dcount;
#[doc = "STA (r) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "status register"]
pub mod sta;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "mask register"]
pub mod mask;
#[doc = "FIFOCNT (r) register accessor: an alias for `Reg<FIFOCNT_SPEC>`"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "data FIFO register"]
pub mod fifo;
