pub mod kicad_pcb_export;

pub use kicad_pcb_export::*;
use copper_substrate::prelude::*;

// Helper function to generate KiCad footprints
pub fn to_kicad_footprint<T: BoardComposableObject>(component: &T) -> String {
    kicad_pcb_export::to_kicad_footprint(component)
}