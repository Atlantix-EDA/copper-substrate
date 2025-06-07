//! # Copper Substrate
//! 
//! A library for creating PCB footprints and managing board design elements.
//! 
//! ## Features
//! 
//! - Define custom SMT and THT components
//! - Generate KiCad footprints
//! - Export PCB designs
//! 
//! ## Example
//! 
//! ```no_run
//! use copper_substrate::prelude::*;
//! use uuid::Uuid;
//! 
//! struct MyComponent;
//! 
//! impl BoardComposableObject for MyComponent {
//!     // ... implementation details
//! #   fn is_smt(&self) -> bool { true }
//! #   fn is_electrical(&self) -> bool { true }
//! #   fn is_passive(&self) -> bool { true }
//! #   fn terminal_count(&self) -> usize { 2 }
//! #   fn functional_type(&self) -> FunctionalType { FunctionalType::Resistor("10k".to_string()) }
//! #   fn footprint_name(&self) -> String { "R_0805".to_string() }
//! #   fn library_name(&self) -> String { "Resistor_SMD".to_string() }
//! #   fn bounding_box(&self) -> Rectangle { Rectangle { min_x: -1.0, min_y: -0.625, max_x: 1.0, max_y: 0.625 } }
//! #   fn pad_descriptors(&self) -> Vec<PadDescriptor> { vec![] }
//! }
//! ```

pub mod substrate;
pub mod exporters;

/// Re-export commonly used types and traits
pub mod prelude {
    pub use crate::substrate::prelude::*;
    pub use crate::exporters::kicad_pcb_export::*;
}