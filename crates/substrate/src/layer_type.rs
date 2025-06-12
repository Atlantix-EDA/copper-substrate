#[derive(Debug, Clone)]
pub enum LayerType {
    SilkScreen,    // F.SilkS - visible markings
    Courtyard,     // F.CrtYd - component boundary
    Fabrication,   // F.Fab - manufacturing reference
    Copper,        // F.Cu - electrical layer
    Mask,          // F.Mask - solder mask
    Paste,         // F.Paste - solder paste
}

impl LayerType {
    pub fn to_kicad_string(&self) -> &'static str {
        match self {
            LayerType::SilkScreen => "F.SilkS",
            LayerType::Courtyard => "F.CrtYd",
            LayerType::Fabrication => "F.Fab",
            LayerType::Copper => "F.Cu",
            LayerType::Mask => "F.Mask",
            LayerType::Paste => "F.Paste",
        }
    }
}