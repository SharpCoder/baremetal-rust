/*
    This outlines the ARM CORTEX-A8 memory mapping
    as defined in the Technical Reference Manual. Check out
    page 177 for the start of the memory mappings.
*/
#![allow(dead_code)]
// Core memory locations
pub const CM_PER: u32 = 0x44E0_0000; // Clock Module Peripheral Registers
pub const CM_DPLL: u32 = 0x44E0_0500; // Clock Module PLL Registers
pub const CM_MPU: u32 = 0x44E0_0600;
pub const CM_DEVICE: u32 = 0x44E0_0700;
pub const CM_WKUP: u32 = 0x44E0_0400;
pub const PRM_PER: u32 = 0x44E0_0C00;
pub const PRM_IRQ: u32 = 0x44E0_0B00;
pub const PRM_MPU: u32 = 0x44E0_0E00;
pub const PRM_WKUP: u32 = 0x44E0_0D00;
pub const PRM_DEVICE: u32 = 0x44E0_0F00;
pub const GPIO1: u32 = 0x4804_C000;
pub const DMTIMER2: u32 = 0x4804_0000;
pub const EMIF0_SDRAM: u32 = 0x8000_0000; // 1GB SDRAM
pub const OCMC0_BASE_PTR: u32 = 0x4030_0000;
pub const INTCPS: u32 = 0x4820_0000; // Interrupt controller registers

// Some useful constants
pub const GB1: u32 = 0x3FFF_FFFF;
pub const KB64: u32 = 0x1_0000;
