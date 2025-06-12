//! Package type definitions for electronic components
//!
//! This module defines the various physical package types that electronic components
//! can have, including surface mount (SMT), through-hole, BGA, and QFP packages.
//! It provides the Package enum and the PackageType trait for polymorphic package handling.

/// Package Enumeration
/// 
/// Defines the different types of packages that components can have.
#[derive(Debug, Clone)]
pub enum Package {
    SMT { size: (f32, f32), pitch: Option<f32> }, // 0603, 0805, etc.
    ThroughHole { spacing: f32, drill_size: f32 },
    BGA { pitch: f32, array_size: (u32, u32) },
    QFP { pitch: f32, pin_count: u32 },
}

pub trait PackageType: std::fmt::Debug + Clone {}
impl PackageType for Package {}