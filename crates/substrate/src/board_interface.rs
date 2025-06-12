//! Board Composable Object, or "Meta-Object"
//!
//! Define the interface for a generic object that can be part of a printed circuit board. 
//! Obviously shapes like circles or cylinders could be a via, a mounting hole, or even a
//! component like a shielded inductor. So make a generic polymorphic interface object that
//! resolves to a specific type of pcb object. So this interface is like a substrate to any
//! pcb object. 
//!
//! This interface is used to define the properties and behaviors of components that can be
//! placed on a PCB, such as resistors, capacitors, ICs, etc. It includes methods for generating
//! KiCad footprints, bounding boxes, pad descriptors, and other properties necessary for PCB design.
//! 
use std::collections::HashMap;
use crate::layer_type::LayerType;
use crate::courtyard::Courtyard;
use crate::functional_types::FunctionalType;
pub trait BoardComposableObject {
    // Basic 
    fn is_smt(&self) -> bool;
    fn is_electrical(&self) -> bool;
    fn is_passive(&self) -> bool { false } // Default to false, can be overridden
    fn terminal_count(&self) -> usize;

    // Core identification
    fn functional_type(&self) -> FunctionalType;
    fn footprint_name(&self) -> String;
    fn library_name(&self) -> String;
    
    // Geometric properties
    fn bounding_box(&self) -> Rectangle;
    fn pad_descriptors(&self) -> Vec<PadDescriptor>;
    
    // Footprint generation - could be used for KiCad or **other** formats
    fn description(&self) -> Option<String>;
    fn tags(&self) -> Option<String>;
    fn fp_text_elements(&self) -> Vec<FpText>;
    fn graphic_elements(&self) -> Vec<GraphicElement>;
    fn model_3d(&self) -> Option<Model3D>;
    
    // Courtyard generation
    fn courtyard_margin(&self) -> f32 { 0.25 } // Default 0.25mm margin
    
    fn generate_courtyard(&self) -> Courtyard {
        let bbox = self.bounding_box();
        Courtyard::new(bbox, self.courtyard_margin())
    }
}

/// Associated constants moved to a separate trait for dyn compatibility
pub trait BoardComposableObjectInfo {
    fn is_electrical(&self) -> bool;
    fn is_smt(&self) -> bool;
    fn terminal_count(&self) -> usize;
}





/// Core geometric types
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}


/// KiCad-specific structures
#[derive(Debug, Clone)]
pub struct PadDescriptor {
    pub number: String,
    pub pad_type: PadType,
    pub shape: PadShape,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub drill_size: Option<f32>,
    pub layers: Vec<String>,
    pub roundrect_ratio: Option<f32>,  // For roundrect pads
    pub tenting: TentingSettings,
    pub uuid: String,
}

#[derive(Debug, Clone)]
pub enum PadType {
    SMD,
    ThroughHole,
    NPTH, // Non-plated through hole
}

#[derive(Debug, Clone)]
pub enum PadShape {
    Circle,
    Rect,
    Oval,
    RoundRect,
}

#[derive(Debug, Clone)]
pub struct TentingSettings {
    pub front: TentingType,
    pub back: TentingType,
}

#[derive(Debug, Clone)]
pub enum TentingType {
    None,
    Full,
    Partial,
}

#[derive(Debug, Clone)]
pub struct FpText {
    pub text_type: FpTextType,
    pub text: String,
    pub position: (f32, f32),
    pub rotation: Option<f32>,
    pub layer: String,
    pub uuid: String,
    pub font: FontSettings,
}

#[derive(Debug, Clone)]
pub enum FpTextType {
    Reference,
    Value,
    User,
}

#[derive(Debug, Clone)]
pub struct FootprintProperty {
    pub name: String,
    pub value: String,
    pub position: (f32, f32),
    pub rotation: Option<f32>,
    pub layer: String,
    pub hidden: bool,
    pub unlocked: bool,
    pub uuid: String,
    pub font: FontSettings,
}

#[derive(Debug, Clone)]
pub struct FontSettings {
    pub size: (f32, f32),
    pub thickness: f32,
}

#[derive(Debug, Clone)]
pub struct GraphicElement {
    pub element_type: GraphicType,
    pub layer: LayerType,
    pub stroke: Stroke,
    pub uuid: String,
}

#[derive(Debug, Clone)]
pub enum GraphicType {
    Line { start: (f32, f32), end: (f32, f32) },
    Rectangle { bounds: Rectangle },
    Circle { center: (f32, f32), radius: f32 },
}


#[derive(Debug, Clone)]
pub struct Stroke {
    pub width: f32,
    pub stroke_type: StrokeType,
}

#[derive(Debug, Clone)]
pub enum StrokeType {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Debug, Clone)]
pub struct Model3D {
    pub path: String,
    pub offset: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

// Layer-specific types for the original traits
#[derive(Debug, Clone)]
pub struct CopperLayer {
    pub layer_name: String,
    pub elements: Vec<GraphicElement>,
}

#[derive(Debug, Clone)]
pub struct SilkscreenElement {
    pub element: GraphicElement,
}

#[derive(Debug, Clone)]
pub struct MaskOpening {
    pub bounds: Rectangle,
}

// Pin and electrical types
pub type PinId = u32;
pub type NetId = u32;

#[derive(Debug, Clone)]
pub struct Pin {
    pub id: PinId,
    pub number: String,
    pub position: (f32, f32),
    pub electrical_type: ElectricalType,
}

#[derive(Debug, Clone)]
pub enum ElectricalType {
    Input,
    Output,
    Bidirectional,
    Power,
    Ground,
    Passive,
}



/// KiCad Export trait for generating .kicad_mod files
pub trait KiCadExportable {
    fn to_kicad_footprint(&self) -> String;
}

// Implementation moved to copper-exporters crate to avoid circular dependency

/// Rendering traits (unchanged from original)
pub trait ComponentRenderer {
    fn render(&self, component: &dyn BoardComposableObject, ctx: &mut egui::Painter);
}


pub trait LayerAware {
    fn copper_layers(&self) -> Vec<CopperLayer>;
    fn silkscreen_elements(&self) -> Vec<SilkscreenElement>;
    fn soldermask_openings(&self) -> Vec<MaskOpening>;
}

pub trait ElectricalComponent {
    fn pins(&self) -> Vec<Pin>;
    fn net_connections(&self) -> HashMap<PinId, NetId>;
}