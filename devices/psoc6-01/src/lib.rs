#![doc = "Peripheral access API for PSOC6_01 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn IOSS_INTERRUPTS_GPIO_0();
    fn IOSS_INTERRUPTS_GPIO_1();
    fn IOSS_INTERRUPTS_GPIO_2();
    fn IOSS_INTERRUPTS_GPIO_3();
    fn IOSS_INTERRUPTS_GPIO_4();
    fn IOSS_INTERRUPTS_GPIO_5();
    fn IOSS_INTERRUPTS_GPIO_6();
    fn IOSS_INTERRUPTS_GPIO_7();
    fn IOSS_INTERRUPTS_GPIO_8();
    fn IOSS_INTERRUPTS_GPIO_9();
    fn IOSS_INTERRUPTS_GPIO_10();
    fn IOSS_INTERRUPTS_GPIO_11();
    fn IOSS_INTERRUPTS_GPIO_12();
    fn IOSS_INTERRUPTS_GPIO_13();
    fn IOSS_INTERRUPTS_GPIO_14();
    fn IOSS_INTERRUPT_GPIO();
    fn IOSS_INTERRUPT_VDD();
    fn LPCOMP_INTERRUPT();
    fn SCB_8_INTERRUPT();
    fn SRSS_INTERRUPT_MCWDT_0();
    fn SRSS_INTERRUPT_MCWDT_1();
    fn SRSS_INTERRUPT_BACKUP();
    fn SRSS_INTERRUPT();
    fn PASS_INTERRUPT_CTBS();
    fn BLESS_INTERRUPT();
    fn CPUSS_INTERRUPTS_IPC_0();
    fn CPUSS_INTERRUPTS_IPC_1();
    fn CPUSS_INTERRUPTS_IPC_2();
    fn CPUSS_INTERRUPTS_IPC_3();
    fn CPUSS_INTERRUPTS_IPC_4();
    fn CPUSS_INTERRUPTS_IPC_5();
    fn CPUSS_INTERRUPTS_IPC_6();
    fn CPUSS_INTERRUPTS_IPC_7();
    fn CPUSS_INTERRUPTS_IPC_8();
    fn CPUSS_INTERRUPTS_IPC_9();
    fn CPUSS_INTERRUPTS_IPC_10();
    fn CPUSS_INTERRUPTS_IPC_11();
    fn CPUSS_INTERRUPTS_IPC_12();
    fn CPUSS_INTERRUPTS_IPC_13();
    fn CPUSS_INTERRUPTS_IPC_14();
    fn CPUSS_INTERRUPTS_IPC_15();
    fn SCB_0_INTERRUPT();
    fn SCB_1_INTERRUPT();
    fn SCB_2_INTERRUPT();
    fn SCB_3_INTERRUPT();
    fn SCB_4_INTERRUPT();
    fn SCB_5_INTERRUPT();
    fn SCB_6_INTERRUPT();
    fn SCB_7_INTERRUPT();
    fn CSD_INTERRUPT();
    fn CPUSS_INTERRUPTS_DW0_0();
    fn CPUSS_INTERRUPTS_DW0_1();
    fn CPUSS_INTERRUPTS_DW0_2();
    fn CPUSS_INTERRUPTS_DW0_3();
    fn CPUSS_INTERRUPTS_DW0_4();
    fn CPUSS_INTERRUPTS_DW0_5();
    fn CPUSS_INTERRUPTS_DW0_6();
    fn CPUSS_INTERRUPTS_DW0_7();
    fn CPUSS_INTERRUPTS_DW0_8();
    fn CPUSS_INTERRUPTS_DW0_9();
    fn CPUSS_INTERRUPTS_DW0_10();
    fn CPUSS_INTERRUPTS_DW0_11();
    fn CPUSS_INTERRUPTS_DW0_12();
    fn CPUSS_INTERRUPTS_DW0_13();
    fn CPUSS_INTERRUPTS_DW0_14();
    fn CPUSS_INTERRUPTS_DW0_15();
    fn CPUSS_INTERRUPTS_DW1_0();
    fn CPUSS_INTERRUPTS_DW1_1();
    fn CPUSS_INTERRUPTS_DW1_2();
    fn CPUSS_INTERRUPTS_DW1_3();
    fn CPUSS_INTERRUPTS_DW1_4();
    fn CPUSS_INTERRUPTS_DW1_5();
    fn CPUSS_INTERRUPTS_DW1_6();
    fn CPUSS_INTERRUPTS_DW1_7();
    fn CPUSS_INTERRUPTS_DW1_8();
    fn CPUSS_INTERRUPTS_DW1_9();
    fn CPUSS_INTERRUPTS_DW1_10();
    fn CPUSS_INTERRUPTS_DW1_11();
    fn CPUSS_INTERRUPTS_DW1_12();
    fn CPUSS_INTERRUPTS_DW1_13();
    fn CPUSS_INTERRUPTS_DW1_14();
    fn CPUSS_INTERRUPTS_DW1_15();
    fn CPUSS_INTERRUPTS_FAULT_0();
    fn CPUSS_INTERRUPTS_FAULT_1();
    fn CPUSS_INTERRUPT_CRYPTO();
    fn CPUSS_INTERRUPT_FM();
    fn CPUSS_INTERRUPTS_CM0_CTI_0();
    fn CPUSS_INTERRUPTS_CM0_CTI_1();
    fn CPUSS_INTERRUPTS_CM4_CTI_0();
    fn CPUSS_INTERRUPTS_CM4_CTI_1();
    fn TCPWM_0_INTERRUPTS_0();
    fn TCPWM_0_INTERRUPTS_1();
    fn TCPWM_0_INTERRUPTS_2();
    fn TCPWM_0_INTERRUPTS_3();
    fn TCPWM_0_INTERRUPTS_4();
    fn TCPWM_0_INTERRUPTS_5();
    fn TCPWM_0_INTERRUPTS_6();
    fn TCPWM_0_INTERRUPTS_7();
    fn TCPWM_1_INTERRUPTS_0();
    fn TCPWM_1_INTERRUPTS_1();
    fn TCPWM_1_INTERRUPTS_2();
    fn TCPWM_1_INTERRUPTS_3();
    fn TCPWM_1_INTERRUPTS_4();
    fn TCPWM_1_INTERRUPTS_5();
    fn TCPWM_1_INTERRUPTS_6();
    fn TCPWM_1_INTERRUPTS_7();
    fn TCPWM_1_INTERRUPTS_8();
    fn TCPWM_1_INTERRUPTS_9();
    fn TCPWM_1_INTERRUPTS_10();
    fn TCPWM_1_INTERRUPTS_11();
    fn TCPWM_1_INTERRUPTS_12();
    fn TCPWM_1_INTERRUPTS_13();
    fn TCPWM_1_INTERRUPTS_14();
    fn TCPWM_1_INTERRUPTS_15();
    fn TCPWM_1_INTERRUPTS_16();
    fn TCPWM_1_INTERRUPTS_17();
    fn TCPWM_1_INTERRUPTS_18();
    fn TCPWM_1_INTERRUPTS_19();
    fn TCPWM_1_INTERRUPTS_20();
    fn TCPWM_1_INTERRUPTS_21();
    fn TCPWM_1_INTERRUPTS_22();
    fn TCPWM_1_INTERRUPTS_23();
    fn UDB_INTERRUPTS_0();
    fn UDB_INTERRUPTS_1();
    fn UDB_INTERRUPTS_2();
    fn UDB_INTERRUPTS_3();
    fn UDB_INTERRUPTS_4();
    fn UDB_INTERRUPTS_5();
    fn UDB_INTERRUPTS_6();
    fn UDB_INTERRUPTS_7();
    fn UDB_INTERRUPTS_8();
    fn UDB_INTERRUPTS_9();
    fn UDB_INTERRUPTS_10();
    fn UDB_INTERRUPTS_11();
    fn UDB_INTERRUPTS_12();
    fn UDB_INTERRUPTS_13();
    fn UDB_INTERRUPTS_14();
    fn UDB_INTERRUPTS_15();
    fn PASS_INTERRUPT_SAR();
    fn AUDIOSS_INTERRUPT_I2S();
    fn AUDIOSS_INTERRUPT_PDM();
    fn PROFILE_INTERRUPT();
    fn SMIF_INTERRUPT();
    fn USB_INTERRUPT_HI();
    fn USB_INTERRUPT_MED();
    fn USB_INTERRUPT_LO();
    fn PASS_INTERRUPT_DACS();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 147] = [
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_0,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_1,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_2,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_3,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_4,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_5,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_6,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_7,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_8,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_9,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_10,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_11,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_12,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_13,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_14,
    },
    Vector {
        _handler: IOSS_INTERRUPT_GPIO,
    },
    Vector {
        _handler: IOSS_INTERRUPT_VDD,
    },
    Vector {
        _handler: LPCOMP_INTERRUPT,
    },
    Vector {
        _handler: SCB_8_INTERRUPT,
    },
    Vector {
        _handler: SRSS_INTERRUPT_MCWDT_0,
    },
    Vector {
        _handler: SRSS_INTERRUPT_MCWDT_1,
    },
    Vector {
        _handler: SRSS_INTERRUPT_BACKUP,
    },
    Vector {
        _handler: SRSS_INTERRUPT,
    },
    Vector {
        _handler: PASS_INTERRUPT_CTBS,
    },
    Vector {
        _handler: BLESS_INTERRUPT,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_1,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_2,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_3,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_4,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_5,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_6,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_7,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_8,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_9,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_10,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_11,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_12,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_13,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_14,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_IPC_15,
    },
    Vector {
        _handler: SCB_0_INTERRUPT,
    },
    Vector {
        _handler: SCB_1_INTERRUPT,
    },
    Vector {
        _handler: SCB_2_INTERRUPT,
    },
    Vector {
        _handler: SCB_3_INTERRUPT,
    },
    Vector {
        _handler: SCB_4_INTERRUPT,
    },
    Vector {
        _handler: SCB_5_INTERRUPT,
    },
    Vector {
        _handler: SCB_6_INTERRUPT,
    },
    Vector {
        _handler: SCB_7_INTERRUPT,
    },
    Vector {
        _handler: CSD_INTERRUPT,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_1,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_2,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_3,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_4,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_5,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_6,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_7,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_8,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_9,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_10,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_11,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_12,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_13,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_14,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW0_15,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_1,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_2,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_3,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_4,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_5,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_6,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_7,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_8,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_9,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_10,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_11,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_12,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_13,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_14,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_DW1_15,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_FAULT_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_FAULT_1,
    },
    Vector {
        _handler: CPUSS_INTERRUPT_CRYPTO,
    },
    Vector {
        _handler: CPUSS_INTERRUPT_FM,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_CM0_CTI_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_CM0_CTI_1,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_CM4_CTI_0,
    },
    Vector {
        _handler: CPUSS_INTERRUPTS_CM4_CTI_1,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_0,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_1,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_2,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_3,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_4,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_5,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_6,
    },
    Vector {
        _handler: TCPWM_0_INTERRUPTS_7,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_0,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_1,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_2,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_3,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_4,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_5,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_6,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_7,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_8,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_9,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_10,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_11,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_12,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_13,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_14,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_15,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_16,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_17,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_18,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_19,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_20,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_21,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_22,
    },
    Vector {
        _handler: TCPWM_1_INTERRUPTS_23,
    },
    Vector {
        _handler: UDB_INTERRUPTS_0,
    },
    Vector {
        _handler: UDB_INTERRUPTS_1,
    },
    Vector {
        _handler: UDB_INTERRUPTS_2,
    },
    Vector {
        _handler: UDB_INTERRUPTS_3,
    },
    Vector {
        _handler: UDB_INTERRUPTS_4,
    },
    Vector {
        _handler: UDB_INTERRUPTS_5,
    },
    Vector {
        _handler: UDB_INTERRUPTS_6,
    },
    Vector {
        _handler: UDB_INTERRUPTS_7,
    },
    Vector {
        _handler: UDB_INTERRUPTS_8,
    },
    Vector {
        _handler: UDB_INTERRUPTS_9,
    },
    Vector {
        _handler: UDB_INTERRUPTS_10,
    },
    Vector {
        _handler: UDB_INTERRUPTS_11,
    },
    Vector {
        _handler: UDB_INTERRUPTS_12,
    },
    Vector {
        _handler: UDB_INTERRUPTS_13,
    },
    Vector {
        _handler: UDB_INTERRUPTS_14,
    },
    Vector {
        _handler: UDB_INTERRUPTS_15,
    },
    Vector {
        _handler: PASS_INTERRUPT_SAR,
    },
    Vector {
        _handler: AUDIOSS_INTERRUPT_I2S,
    },
    Vector {
        _handler: AUDIOSS_INTERRUPT_PDM,
    },
    Vector {
        _handler: PROFILE_INTERRUPT,
    },
    Vector {
        _handler: SMIF_INTERRUPT,
    },
    Vector {
        _handler: USB_INTERRUPT_HI,
    },
    Vector {
        _handler: USB_INTERRUPT_MED,
    },
    Vector {
        _handler: USB_INTERRUPT_LO,
    },
    Vector {
        _handler: PASS_INTERRUPT_DACS,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - GPIO Port Interrupt #0"]
    IOSS_INTERRUPTS_GPIO_0 = 0,
    #[doc = "1 - GPIO Port Interrupt #1"]
    IOSS_INTERRUPTS_GPIO_1 = 1,
    #[doc = "2 - GPIO Port Interrupt #2"]
    IOSS_INTERRUPTS_GPIO_2 = 2,
    #[doc = "3 - GPIO Port Interrupt #3"]
    IOSS_INTERRUPTS_GPIO_3 = 3,
    #[doc = "4 - GPIO Port Interrupt #4"]
    IOSS_INTERRUPTS_GPIO_4 = 4,
    #[doc = "5 - GPIO Port Interrupt #5"]
    IOSS_INTERRUPTS_GPIO_5 = 5,
    #[doc = "6 - GPIO Port Interrupt #6"]
    IOSS_INTERRUPTS_GPIO_6 = 6,
    #[doc = "7 - GPIO Port Interrupt #7"]
    IOSS_INTERRUPTS_GPIO_7 = 7,
    #[doc = "8 - GPIO Port Interrupt #8"]
    IOSS_INTERRUPTS_GPIO_8 = 8,
    #[doc = "9 - GPIO Port Interrupt #9"]
    IOSS_INTERRUPTS_GPIO_9 = 9,
    #[doc = "10 - GPIO Port Interrupt #10"]
    IOSS_INTERRUPTS_GPIO_10 = 10,
    #[doc = "11 - GPIO Port Interrupt #11"]
    IOSS_INTERRUPTS_GPIO_11 = 11,
    #[doc = "12 - GPIO Port Interrupt #12"]
    IOSS_INTERRUPTS_GPIO_12 = 12,
    #[doc = "13 - GPIO Port Interrupt #13"]
    IOSS_INTERRUPTS_GPIO_13 = 13,
    #[doc = "14 - GPIO Port Interrupt #14"]
    IOSS_INTERRUPTS_GPIO_14 = 14,
    #[doc = "15 - GPIO All Ports"]
    IOSS_INTERRUPT_GPIO = 15,
    #[doc = "16 - GPIO Supply Detect Interrupt"]
    IOSS_INTERRUPT_VDD = 16,
    #[doc = "17 - Low Power Comparator Interrupt"]
    LPCOMP_INTERRUPT = 17,
    #[doc = "18 - Serial Communication Block #8 (DeepSleep capable)"]
    SCB_8_INTERRUPT = 18,
    #[doc = "19 - Multi Counter Watchdog Timer interrupt"]
    SRSS_INTERRUPT_MCWDT_0 = 19,
    #[doc = "20 - Multi Counter Watchdog Timer interrupt"]
    SRSS_INTERRUPT_MCWDT_1 = 20,
    #[doc = "21 - Backup domain interrupt"]
    SRSS_INTERRUPT_BACKUP = 21,
    #[doc = "22 - Other combined Interrupts for SRSS (LVD, WDT, CLKCAL)"]
    SRSS_INTERRUPT = 22,
    #[doc = "23 - CTBm Interrupt (all CTBms)"]
    PASS_INTERRUPT_CTBS = 23,
    #[doc = "24 - Bluetooth Radio interrupt"]
    BLESS_INTERRUPT = 24,
    #[doc = "25 - CPUSS Inter Process Communication Interrupt #0"]
    CPUSS_INTERRUPTS_IPC_0 = 25,
    #[doc = "26 - CPUSS Inter Process Communication Interrupt #1"]
    CPUSS_INTERRUPTS_IPC_1 = 26,
    #[doc = "27 - CPUSS Inter Process Communication Interrupt #2"]
    CPUSS_INTERRUPTS_IPC_2 = 27,
    #[doc = "28 - CPUSS Inter Process Communication Interrupt #3"]
    CPUSS_INTERRUPTS_IPC_3 = 28,
    #[doc = "29 - CPUSS Inter Process Communication Interrupt #4"]
    CPUSS_INTERRUPTS_IPC_4 = 29,
    #[doc = "30 - CPUSS Inter Process Communication Interrupt #5"]
    CPUSS_INTERRUPTS_IPC_5 = 30,
    #[doc = "31 - CPUSS Inter Process Communication Interrupt #6"]
    CPUSS_INTERRUPTS_IPC_6 = 31,
    #[doc = "32 - CPUSS Inter Process Communication Interrupt #7"]
    CPUSS_INTERRUPTS_IPC_7 = 32,
    #[doc = "33 - CPUSS Inter Process Communication Interrupt #8"]
    CPUSS_INTERRUPTS_IPC_8 = 33,
    #[doc = "34 - CPUSS Inter Process Communication Interrupt #9"]
    CPUSS_INTERRUPTS_IPC_9 = 34,
    #[doc = "35 - CPUSS Inter Process Communication Interrupt #10"]
    CPUSS_INTERRUPTS_IPC_10 = 35,
    #[doc = "36 - CPUSS Inter Process Communication Interrupt #11"]
    CPUSS_INTERRUPTS_IPC_11 = 36,
    #[doc = "37 - CPUSS Inter Process Communication Interrupt #12"]
    CPUSS_INTERRUPTS_IPC_12 = 37,
    #[doc = "38 - CPUSS Inter Process Communication Interrupt #13"]
    CPUSS_INTERRUPTS_IPC_13 = 38,
    #[doc = "39 - CPUSS Inter Process Communication Interrupt #14"]
    CPUSS_INTERRUPTS_IPC_14 = 39,
    #[doc = "40 - CPUSS Inter Process Communication Interrupt #15"]
    CPUSS_INTERRUPTS_IPC_15 = 40,
    #[doc = "41 - Serial Communication Block #0"]
    SCB_0_INTERRUPT = 41,
    #[doc = "42 - Serial Communication Block #1"]
    SCB_1_INTERRUPT = 42,
    #[doc = "43 - Serial Communication Block #2"]
    SCB_2_INTERRUPT = 43,
    #[doc = "44 - Serial Communication Block #3"]
    SCB_3_INTERRUPT = 44,
    #[doc = "45 - Serial Communication Block #4"]
    SCB_4_INTERRUPT = 45,
    #[doc = "46 - Serial Communication Block #5"]
    SCB_5_INTERRUPT = 46,
    #[doc = "47 - Serial Communication Block #6"]
    SCB_6_INTERRUPT = 47,
    #[doc = "48 - Serial Communication Block #7"]
    SCB_7_INTERRUPT = 48,
    #[doc = "49 - CSD (Capsense) interrupt"]
    CSD_INTERRUPT = 49,
    #[doc = "50 - CPUSS DataWire #0, Channel #0"]
    CPUSS_INTERRUPTS_DW0_0 = 50,
    #[doc = "51 - CPUSS DataWire #0, Channel #1"]
    CPUSS_INTERRUPTS_DW0_1 = 51,
    #[doc = "52 - CPUSS DataWire #0, Channel #2"]
    CPUSS_INTERRUPTS_DW0_2 = 52,
    #[doc = "53 - CPUSS DataWire #0, Channel #3"]
    CPUSS_INTERRUPTS_DW0_3 = 53,
    #[doc = "54 - CPUSS DataWire #0, Channel #4"]
    CPUSS_INTERRUPTS_DW0_4 = 54,
    #[doc = "55 - CPUSS DataWire #0, Channel #5"]
    CPUSS_INTERRUPTS_DW0_5 = 55,
    #[doc = "56 - CPUSS DataWire #0, Channel #6"]
    CPUSS_INTERRUPTS_DW0_6 = 56,
    #[doc = "57 - CPUSS DataWire #0, Channel #7"]
    CPUSS_INTERRUPTS_DW0_7 = 57,
    #[doc = "58 - CPUSS DataWire #0, Channel #8"]
    CPUSS_INTERRUPTS_DW0_8 = 58,
    #[doc = "59 - CPUSS DataWire #0, Channel #9"]
    CPUSS_INTERRUPTS_DW0_9 = 59,
    #[doc = "60 - CPUSS DataWire #0, Channel #10"]
    CPUSS_INTERRUPTS_DW0_10 = 60,
    #[doc = "61 - CPUSS DataWire #0, Channel #11"]
    CPUSS_INTERRUPTS_DW0_11 = 61,
    #[doc = "62 - CPUSS DataWire #0, Channel #12"]
    CPUSS_INTERRUPTS_DW0_12 = 62,
    #[doc = "63 - CPUSS DataWire #0, Channel #13"]
    CPUSS_INTERRUPTS_DW0_13 = 63,
    #[doc = "64 - CPUSS DataWire #0, Channel #14"]
    CPUSS_INTERRUPTS_DW0_14 = 64,
    #[doc = "65 - CPUSS DataWire #0, Channel #15"]
    CPUSS_INTERRUPTS_DW0_15 = 65,
    #[doc = "66 - CPUSS DataWire #1, Channel #0"]
    CPUSS_INTERRUPTS_DW1_0 = 66,
    #[doc = "67 - CPUSS DataWire #1, Channel #1"]
    CPUSS_INTERRUPTS_DW1_1 = 67,
    #[doc = "68 - CPUSS DataWire #1, Channel #2"]
    CPUSS_INTERRUPTS_DW1_2 = 68,
    #[doc = "69 - CPUSS DataWire #1, Channel #3"]
    CPUSS_INTERRUPTS_DW1_3 = 69,
    #[doc = "70 - CPUSS DataWire #1, Channel #4"]
    CPUSS_INTERRUPTS_DW1_4 = 70,
    #[doc = "71 - CPUSS DataWire #1, Channel #5"]
    CPUSS_INTERRUPTS_DW1_5 = 71,
    #[doc = "72 - CPUSS DataWire #1, Channel #6"]
    CPUSS_INTERRUPTS_DW1_6 = 72,
    #[doc = "73 - CPUSS DataWire #1, Channel #7"]
    CPUSS_INTERRUPTS_DW1_7 = 73,
    #[doc = "74 - CPUSS DataWire #1, Channel #8"]
    CPUSS_INTERRUPTS_DW1_8 = 74,
    #[doc = "75 - CPUSS DataWire #1, Channel #9"]
    CPUSS_INTERRUPTS_DW1_9 = 75,
    #[doc = "76 - CPUSS DataWire #1, Channel #10"]
    CPUSS_INTERRUPTS_DW1_10 = 76,
    #[doc = "77 - CPUSS DataWire #1, Channel #11"]
    CPUSS_INTERRUPTS_DW1_11 = 77,
    #[doc = "78 - CPUSS DataWire #1, Channel #12"]
    CPUSS_INTERRUPTS_DW1_12 = 78,
    #[doc = "79 - CPUSS DataWire #1, Channel #13"]
    CPUSS_INTERRUPTS_DW1_13 = 79,
    #[doc = "80 - CPUSS DataWire #1, Channel #14"]
    CPUSS_INTERRUPTS_DW1_14 = 80,
    #[doc = "81 - CPUSS DataWire #1, Channel #15"]
    CPUSS_INTERRUPTS_DW1_15 = 81,
    #[doc = "82 - CPUSS Fault Structure Interrupt #0"]
    CPUSS_INTERRUPTS_FAULT_0 = 82,
    #[doc = "83 - CPUSS Fault Structure Interrupt #1"]
    CPUSS_INTERRUPTS_FAULT_1 = 83,
    #[doc = "84 - CRYPTO Accelerator Interrupt"]
    CPUSS_INTERRUPT_CRYPTO = 84,
    #[doc = "85 - FLASH Macro Interrupt"]
    CPUSS_INTERRUPT_FM = 85,
    #[doc = "86 - CM0+ CTI #0"]
    CPUSS_INTERRUPTS_CM0_CTI_0 = 86,
    #[doc = "87 - CM0+ CTI #1"]
    CPUSS_INTERRUPTS_CM0_CTI_1 = 87,
    #[doc = "88 - CM4 CTI #0"]
    CPUSS_INTERRUPTS_CM4_CTI_0 = 88,
    #[doc = "89 - CM4 CTI #1"]
    CPUSS_INTERRUPTS_CM4_CTI_1 = 89,
    #[doc = "90 - TCPWM #0, Counter #0"]
    TCPWM_0_INTERRUPTS_0 = 90,
    #[doc = "91 - TCPWM #0, Counter #1"]
    TCPWM_0_INTERRUPTS_1 = 91,
    #[doc = "92 - TCPWM #0, Counter #2"]
    TCPWM_0_INTERRUPTS_2 = 92,
    #[doc = "93 - TCPWM #0, Counter #3"]
    TCPWM_0_INTERRUPTS_3 = 93,
    #[doc = "94 - TCPWM #0, Counter #4"]
    TCPWM_0_INTERRUPTS_4 = 94,
    #[doc = "95 - TCPWM #0, Counter #5"]
    TCPWM_0_INTERRUPTS_5 = 95,
    #[doc = "96 - TCPWM #0, Counter #6"]
    TCPWM_0_INTERRUPTS_6 = 96,
    #[doc = "97 - TCPWM #0, Counter #7"]
    TCPWM_0_INTERRUPTS_7 = 97,
    #[doc = "98 - TCPWM #1, Counter #0"]
    TCPWM_1_INTERRUPTS_0 = 98,
    #[doc = "99 - TCPWM #1, Counter #1"]
    TCPWM_1_INTERRUPTS_1 = 99,
    #[doc = "100 - TCPWM #1, Counter #2"]
    TCPWM_1_INTERRUPTS_2 = 100,
    #[doc = "101 - TCPWM #1, Counter #3"]
    TCPWM_1_INTERRUPTS_3 = 101,
    #[doc = "102 - TCPWM #1, Counter #4"]
    TCPWM_1_INTERRUPTS_4 = 102,
    #[doc = "103 - TCPWM #1, Counter #5"]
    TCPWM_1_INTERRUPTS_5 = 103,
    #[doc = "104 - TCPWM #1, Counter #6"]
    TCPWM_1_INTERRUPTS_6 = 104,
    #[doc = "105 - TCPWM #1, Counter #7"]
    TCPWM_1_INTERRUPTS_7 = 105,
    #[doc = "106 - TCPWM #1, Counter #8"]
    TCPWM_1_INTERRUPTS_8 = 106,
    #[doc = "107 - TCPWM #1, Counter #9"]
    TCPWM_1_INTERRUPTS_9 = 107,
    #[doc = "108 - TCPWM #1, Counter #10"]
    TCPWM_1_INTERRUPTS_10 = 108,
    #[doc = "109 - TCPWM #1, Counter #11"]
    TCPWM_1_INTERRUPTS_11 = 109,
    #[doc = "110 - TCPWM #1, Counter #12"]
    TCPWM_1_INTERRUPTS_12 = 110,
    #[doc = "111 - TCPWM #1, Counter #13"]
    TCPWM_1_INTERRUPTS_13 = 111,
    #[doc = "112 - TCPWM #1, Counter #14"]
    TCPWM_1_INTERRUPTS_14 = 112,
    #[doc = "113 - TCPWM #1, Counter #15"]
    TCPWM_1_INTERRUPTS_15 = 113,
    #[doc = "114 - TCPWM #1, Counter #16"]
    TCPWM_1_INTERRUPTS_16 = 114,
    #[doc = "115 - TCPWM #1, Counter #17"]
    TCPWM_1_INTERRUPTS_17 = 115,
    #[doc = "116 - TCPWM #1, Counter #18"]
    TCPWM_1_INTERRUPTS_18 = 116,
    #[doc = "117 - TCPWM #1, Counter #19"]
    TCPWM_1_INTERRUPTS_19 = 117,
    #[doc = "118 - TCPWM #1, Counter #20"]
    TCPWM_1_INTERRUPTS_20 = 118,
    #[doc = "119 - TCPWM #1, Counter #21"]
    TCPWM_1_INTERRUPTS_21 = 119,
    #[doc = "120 - TCPWM #1, Counter #22"]
    TCPWM_1_INTERRUPTS_22 = 120,
    #[doc = "121 - TCPWM #1, Counter #23"]
    TCPWM_1_INTERRUPTS_23 = 121,
    #[doc = "122 - UDB Interrupt #0"]
    UDB_INTERRUPTS_0 = 122,
    #[doc = "123 - UDB Interrupt #1"]
    UDB_INTERRUPTS_1 = 123,
    #[doc = "124 - UDB Interrupt #2"]
    UDB_INTERRUPTS_2 = 124,
    #[doc = "125 - UDB Interrupt #3"]
    UDB_INTERRUPTS_3 = 125,
    #[doc = "126 - UDB Interrupt #4"]
    UDB_INTERRUPTS_4 = 126,
    #[doc = "127 - UDB Interrupt #5"]
    UDB_INTERRUPTS_5 = 127,
    #[doc = "128 - UDB Interrupt #6"]
    UDB_INTERRUPTS_6 = 128,
    #[doc = "129 - UDB Interrupt #7"]
    UDB_INTERRUPTS_7 = 129,
    #[doc = "130 - UDB Interrupt #8"]
    UDB_INTERRUPTS_8 = 130,
    #[doc = "131 - UDB Interrupt #9"]
    UDB_INTERRUPTS_9 = 131,
    #[doc = "132 - UDB Interrupt #10"]
    UDB_INTERRUPTS_10 = 132,
    #[doc = "133 - UDB Interrupt #11"]
    UDB_INTERRUPTS_11 = 133,
    #[doc = "134 - UDB Interrupt #12"]
    UDB_INTERRUPTS_12 = 134,
    #[doc = "135 - UDB Interrupt #13"]
    UDB_INTERRUPTS_13 = 135,
    #[doc = "136 - UDB Interrupt #14"]
    UDB_INTERRUPTS_14 = 136,
    #[doc = "137 - UDB Interrupt #15"]
    UDB_INTERRUPTS_15 = 137,
    #[doc = "138 - SAR ADC interrupt"]
    PASS_INTERRUPT_SAR = 138,
    #[doc = "139 - I2S Audio interrupt"]
    AUDIOSS_INTERRUPT_I2S = 139,
    #[doc = "140 - PDM/PCM Audio interrupt"]
    AUDIOSS_INTERRUPT_PDM = 140,
    #[doc = "141 - Energy Profiler interrupt"]
    PROFILE_INTERRUPT = 141,
    #[doc = "142 - Serial Memory Interface interrupt"]
    SMIF_INTERRUPT = 142,
    #[doc = "143 - USB Interrupt"]
    USB_INTERRUPT_HI = 143,
    #[doc = "144 - USB Interrupt"]
    USB_INTERRUPT_MED = 144,
    #[doc = "145 - USB Interrupt"]
    USB_INTERRUPT_LO = 145,
    #[doc = "146 - Consolidated interrrupt for all DACs"]
    PASS_INTERRUPT_DACS = 146,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Peripheral interconnect"]
pub struct PERI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PERI {}
impl PERI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const peri::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for PERI {
    type Target = peri::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PERI::ptr() }
    }
}
#[doc = "Peripheral interconnect"]
pub mod peri;
#[doc = "CPU subsystem (CPUSS)"]
pub struct CPUSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPUSS {}
impl CPUSS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpuss::RegisterBlock {
        0x4021_0000 as *const _
    }
}
impl Deref for CPUSS {
    type Target = cpuss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPUSS::ptr() }
    }
}
#[doc = "CPU subsystem (CPUSS)"]
pub mod cpuss;
#[doc = "Fault structures"]
pub struct FAULT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FAULT {}
impl FAULT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fault::RegisterBlock {
        0x4022_0000 as *const _
    }
}
impl Deref for FAULT {
    type Target = fault::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FAULT::ptr() }
    }
}
#[doc = "Fault structures"]
pub mod fault;
#[doc = "IPC"]
pub struct IPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC {}
impl IPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipc::RegisterBlock {
        0x4023_0000 as *const _
    }
}
impl Deref for IPC {
    type Target = ipc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPC::ptr() }
    }
}
#[doc = "IPC"]
pub mod ipc;
#[doc = "Protection"]
pub struct PROT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PROT {}
impl PROT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prot::RegisterBlock {
        0x4024_0000 as *const _
    }
}
impl Deref for PROT {
    type Target = prot::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PROT::ptr() }
    }
}
#[doc = "Protection"]
pub mod prot;
#[doc = "Flash controller"]
pub struct FLASHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHC {}
impl FLASHC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flashc::RegisterBlock {
        0x4025_0000 as *const _
    }
}
impl Deref for FLASHC {
    type Target = flashc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASHC::ptr() }
    }
}
#[doc = "Flash controller"]
pub mod flashc;
#[doc = "SRSS Core Registers"]
pub struct SRSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRSS {}
impl SRSS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const srss::RegisterBlock {
        0x4026_0000 as *const _
    }
}
impl Deref for SRSS {
    type Target = srss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRSS::ptr() }
    }
}
#[doc = "SRSS Core Registers"]
pub mod srss;
#[doc = "SRSS Backup Domain"]
pub struct BACKUP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP {}
impl BACKUP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup::RegisterBlock {
        0x4027_0000 as *const _
    }
}
impl Deref for BACKUP {
    type Target = backup::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BACKUP::ptr() }
    }
}
#[doc = "SRSS Backup Domain"]
pub mod backup;
#[doc = "Datawire Controller"]
pub struct DW0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DW0 {}
impl DW0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dw0::RegisterBlock {
        0x4028_0000 as *const _
    }
}
impl Deref for DW0 {
    type Target = dw0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DW0::ptr() }
    }
}
#[doc = "Datawire Controller"]
pub mod dw0;
#[doc = "Datawire Controller"]
pub struct DW1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DW1 {}
impl DW1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dw0::RegisterBlock {
        0x4028_1000 as *const _
    }
}
impl Deref for DW1 {
    type Target = dw0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DW1::ptr() }
    }
}
#[doc = "EFUSE MXS40 registers"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        0x402c_0000 as *const _
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFUSE::ptr() }
    }
}
#[doc = "EFUSE MXS40 registers"]
pub mod efuse;
#[doc = "Energy Profiler IP"]
pub struct PROFILE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PROFILE {}
impl PROFILE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const profile::RegisterBlock {
        0x402d_0000 as *const _
    }
}
impl Deref for PROFILE {
    type Target = profile::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PROFILE::ptr() }
    }
}
#[doc = "Energy Profiler IP"]
pub mod profile;
#[doc = "High Speed IO Matrix (HSIOM)"]
pub struct HSIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSIOM {}
impl HSIOM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsiom::RegisterBlock {
        0x4031_0000 as *const _
    }
}
impl Deref for HSIOM {
    type Target = hsiom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSIOM::ptr() }
    }
}
#[doc = "High Speed IO Matrix (HSIOM)"]
pub mod hsiom;
#[doc = "GPIO port control/configuration"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4032_0000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO port control/configuration"]
pub mod gpio;
#[doc = "Programmable IO configuration"]
pub struct SMARTIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMARTIO {}
impl SMARTIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smartio::RegisterBlock {
        0x4033_0000 as *const _
    }
}
impl Deref for SMARTIO {
    type Target = smartio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMARTIO::ptr() }
    }
}
#[doc = "Programmable IO configuration"]
pub mod smartio;
#[doc = "Low Power Comparators"]
pub struct LPCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP {}
impl LPCOMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpcomp::RegisterBlock {
        0x4035_0000 as *const _
    }
}
impl Deref for LPCOMP {
    type Target = lpcomp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPCOMP::ptr() }
    }
}
#[doc = "Low Power Comparators"]
pub mod lpcomp;
#[doc = "Capsense Controller"]
pub struct CSD0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSD0 {}
impl CSD0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csd0::RegisterBlock {
        0x4036_0000 as *const _
    }
}
impl Deref for CSD0 {
    type Target = csd0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CSD0::ptr() }
    }
}
#[doc = "Capsense Controller"]
pub mod csd0;
#[doc = "Timer/Counter/PWM"]
pub struct TCPWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCPWM0 {}
impl TCPWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcpwm0::RegisterBlock {
        0x4038_0000 as *const _
    }
}
impl Deref for TCPWM0 {
    type Target = tcpwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCPWM0::ptr() }
    }
}
#[doc = "Timer/Counter/PWM"]
pub mod tcpwm0;
#[doc = "Timer/Counter/PWM"]
pub struct TCPWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCPWM1 {}
impl TCPWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcpwm0::RegisterBlock {
        0x4039_0000 as *const _
    }
}
impl Deref for TCPWM1 {
    type Target = tcpwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCPWM1::ptr() }
    }
}
#[doc = "LCD Controller Block"]
pub struct LCD0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD0 {}
impl LCD0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd0::RegisterBlock {
        0x403b_0000 as *const _
    }
}
impl Deref for LCD0 {
    type Target = lcd0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD0::ptr() }
    }
}
#[doc = "LCD Controller Block"]
pub mod lcd0;
#[doc = "Bluetooth Low Energy Subsystem"]
pub struct BLE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BLE {}
impl BLE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ble::RegisterBlock {
        0x403c_0000 as *const _
    }
}
impl Deref for BLE {
    type Target = ble::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BLE::ptr() }
    }
}
#[doc = "Bluetooth Low Energy Subsystem"]
pub mod ble;
#[doc = "USB Host and Device Controller"]
pub struct USBFS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBFS0 {}
impl USBFS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbfs0::RegisterBlock {
        0x403f_0000 as *const _
    }
}
impl Deref for USBFS0 {
    type Target = usbfs0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBFS0::ptr() }
    }
}
#[doc = "USB Host and Device Controller"]
pub mod usbfs0;
#[doc = "Serial Memory Interface"]
pub struct SMIF0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMIF0 {}
impl SMIF0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smif0::RegisterBlock {
        0x4042_0000 as *const _
    }
}
impl Deref for SMIF0 {
    type Target = smif0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMIF0::ptr() }
    }
}
#[doc = "Serial Memory Interface"]
pub mod smif0;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB0 {}
impl SCB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4061_0000 as *const _
    }
}
impl Deref for SCB0 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB0::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub mod scb0;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB1 {}
impl SCB1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4062_0000 as *const _
    }
}
impl Deref for SCB1 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB1::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB2 {}
impl SCB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4063_0000 as *const _
    }
}
impl Deref for SCB2 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB2::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB3 {}
impl SCB3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4064_0000 as *const _
    }
}
impl Deref for SCB3 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB3::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB4 {}
impl SCB4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4065_0000 as *const _
    }
}
impl Deref for SCB4 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB4::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB5 {}
impl SCB5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4066_0000 as *const _
    }
}
impl Deref for SCB5 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB5::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB6 {}
impl SCB6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4067_0000 as *const _
    }
}
impl Deref for SCB6 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB6::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB7 {}
impl SCB7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4068_0000 as *const _
    }
}
impl Deref for SCB7 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB7::ptr() }
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB8 {}
impl SCB8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        0x4069_0000 as *const _
    }
}
impl Deref for SCB8 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB8::ptr() }
    }
}
#[doc = "Continuous Time Block Mini"]
pub struct CTBM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTBM0 {}
impl CTBM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctbm0::RegisterBlock {
        0x4110_0000 as *const _
    }
}
impl Deref for CTBM0 {
    type Target = ctbm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTBM0::ptr() }
    }
}
#[doc = "Continuous Time Block Mini"]
pub mod ctbm0;
#[doc = "Continuous Time DAC"]
pub struct CTDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTDAC0 {}
impl CTDAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctdac0::RegisterBlock {
        0x4114_0000 as *const _
    }
}
impl Deref for CTDAC0 {
    type Target = ctdac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTDAC0::ptr() }
    }
}
#[doc = "Continuous Time DAC"]
pub mod ctdac0;
#[doc = "SAR ADC with Sequencer"]
pub struct SAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAR {}
impl SAR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sar::RegisterBlock {
        0x411d_0000 as *const _
    }
}
impl Deref for SAR {
    type Target = sar::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAR::ptr() }
    }
}
#[doc = "SAR ADC with Sequencer"]
pub mod sar;
#[doc = "PASS top-level MMIO (DSABv2, INTR)"]
pub struct PASS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PASS {}
impl PASS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pass::RegisterBlock {
        0x411f_0000 as *const _
    }
}
impl Deref for PASS {
    type Target = pass::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PASS::ptr() }
    }
}
#[doc = "PASS top-level MMIO (DSABv2, INTR)"]
pub mod pass;
#[doc = "I2S registers"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x42a1_0000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S registers"]
pub mod i2s0;
#[doc = "PDM registers"]
pub struct PDM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM0 {}
impl PDM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdm0::RegisterBlock {
        0x42a2_0000 as *const _
    }
}
impl Deref for PDM0 {
    type Target = pdm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDM0::ptr() }
    }
}
#[doc = "PDM registers"]
pub mod pdm0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PERI"]
    pub PERI: PERI,
    #[doc = "CPUSS"]
    pub CPUSS: CPUSS,
    #[doc = "FAULT"]
    pub FAULT: FAULT,
    #[doc = "IPC"]
    pub IPC: IPC,
    #[doc = "PROT"]
    pub PROT: PROT,
    #[doc = "FLASHC"]
    pub FLASHC: FLASHC,
    #[doc = "SRSS"]
    pub SRSS: SRSS,
    #[doc = "BACKUP"]
    pub BACKUP: BACKUP,
    #[doc = "DW0"]
    pub DW0: DW0,
    #[doc = "DW1"]
    pub DW1: DW1,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "PROFILE"]
    pub PROFILE: PROFILE,
    #[doc = "HSIOM"]
    pub HSIOM: HSIOM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SMARTIO"]
    pub SMARTIO: SMARTIO,
    #[doc = "LPCOMP"]
    pub LPCOMP: LPCOMP,
    #[doc = "CSD0"]
    pub CSD0: CSD0,
    #[doc = "TCPWM0"]
    pub TCPWM0: TCPWM0,
    #[doc = "TCPWM1"]
    pub TCPWM1: TCPWM1,
    #[doc = "LCD0"]
    pub LCD0: LCD0,
    #[doc = "BLE"]
    pub BLE: BLE,
    #[doc = "USBFS0"]
    pub USBFS0: USBFS0,
    #[doc = "SMIF0"]
    pub SMIF0: SMIF0,
    #[doc = "SCB0"]
    pub SCB0: SCB0,
    #[doc = "SCB1"]
    pub SCB1: SCB1,
    #[doc = "SCB2"]
    pub SCB2: SCB2,
    #[doc = "SCB3"]
    pub SCB3: SCB3,
    #[doc = "SCB4"]
    pub SCB4: SCB4,
    #[doc = "SCB5"]
    pub SCB5: SCB5,
    #[doc = "SCB6"]
    pub SCB6: SCB6,
    #[doc = "SCB7"]
    pub SCB7: SCB7,
    #[doc = "SCB8"]
    pub SCB8: SCB8,
    #[doc = "CTBM0"]
    pub CTBM0: CTBM0,
    #[doc = "CTDAC0"]
    pub CTDAC0: CTDAC0,
    #[doc = "SAR"]
    pub SAR: SAR,
    #[doc = "PASS"]
    pub PASS: PASS,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "PDM0"]
    pub PDM0: PDM0,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PERI: PERI {
                _marker: PhantomData,
            },
            CPUSS: CPUSS {
                _marker: PhantomData,
            },
            FAULT: FAULT {
                _marker: PhantomData,
            },
            IPC: IPC {
                _marker: PhantomData,
            },
            PROT: PROT {
                _marker: PhantomData,
            },
            FLASHC: FLASHC {
                _marker: PhantomData,
            },
            SRSS: SRSS {
                _marker: PhantomData,
            },
            BACKUP: BACKUP {
                _marker: PhantomData,
            },
            DW0: DW0 {
                _marker: PhantomData,
            },
            DW1: DW1 {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            PROFILE: PROFILE {
                _marker: PhantomData,
            },
            HSIOM: HSIOM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SMARTIO: SMARTIO {
                _marker: PhantomData,
            },
            LPCOMP: LPCOMP {
                _marker: PhantomData,
            },
            CSD0: CSD0 {
                _marker: PhantomData,
            },
            TCPWM0: TCPWM0 {
                _marker: PhantomData,
            },
            TCPWM1: TCPWM1 {
                _marker: PhantomData,
            },
            LCD0: LCD0 {
                _marker: PhantomData,
            },
            BLE: BLE {
                _marker: PhantomData,
            },
            USBFS0: USBFS0 {
                _marker: PhantomData,
            },
            SMIF0: SMIF0 {
                _marker: PhantomData,
            },
            SCB0: SCB0 {
                _marker: PhantomData,
            },
            SCB1: SCB1 {
                _marker: PhantomData,
            },
            SCB2: SCB2 {
                _marker: PhantomData,
            },
            SCB3: SCB3 {
                _marker: PhantomData,
            },
            SCB4: SCB4 {
                _marker: PhantomData,
            },
            SCB5: SCB5 {
                _marker: PhantomData,
            },
            SCB6: SCB6 {
                _marker: PhantomData,
            },
            SCB7: SCB7 {
                _marker: PhantomData,
            },
            SCB8: SCB8 {
                _marker: PhantomData,
            },
            CTBM0: CTBM0 {
                _marker: PhantomData,
            },
            CTDAC0: CTDAC0 {
                _marker: PhantomData,
            },
            SAR: SAR {
                _marker: PhantomData,
            },
            PASS: PASS {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            PDM0: PDM0 {
                _marker: PhantomData,
            },
        }
    }
}
