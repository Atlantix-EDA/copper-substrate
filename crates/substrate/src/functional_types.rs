//! Functional type definitions for electronic components
//!
//! This module defines the various functional types that electronic components
//! can have in a PCB design, from passive components like resistors and capacitors
//! to active components like integrated circuits and microcontrollers.

/// Functional Type Enumeration
/// 
/// where string specifies the type, i.e. FPGA(Artix7) or MCU(Pico2) 
#[derive(Debug, Clone)]
pub enum FunctionalType {
    Resistor(String),
    Capacitor(String),
    Inductor(String),
    Connector(String),
    Fuse(String),
    Protection(String),
    IntegratedCircuit(String),
    ADC(String),
    DAC(String),
    FPGA(String),
    MCU(String),
    LED(String),
    LCD(String),
    IsolationIC(String),
    OpAmp(String),
    Timer(String),
}