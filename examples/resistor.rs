use copper_substrate::prelude::*;
use uuid::Uuid;

struct SMTResistor0805 {
    value: String,
}

impl BoardComposableObject for SMTResistor0805 {

    fn is_smt(&self) -> bool {
        true
    }
    fn is_electrical(&self) -> bool {
        true
    }
    fn is_passive(&self) -> bool {
        true
    }
    fn terminal_count(&self) -> usize {
        2
    }
    
    fn functional_type(&self) -> FunctionalType {
        FunctionalType::Resistor(self.value.clone())
    }
    
    fn footprint_name(&self) -> String {
        "R_0805_2012Metric".to_string()
    }
    
    fn library_name(&self) -> String {
        "Resistor_SMD".to_string()
    }
    
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            min_x: -1.0,
            min_y: -0.625,
            max_x: 1.0,
            max_y: 0.625,
        }
    }
    
    fn pad_descriptors(&self) -> Vec<PadDescriptor> {
        vec![
            PadDescriptor {
                number: "1".to_string(),
                pad_type: PadType::SMD,
                shape: PadShape::RoundRect,
                position: (-0.95, 0.0),
                size: (1.0, 1.45),
                drill_size: None,
                layers: vec!["F.Cu".to_string(), "F.Mask".to_string(), "F.Paste".to_string()],
                roundrect_ratio: Some(0.25),
                tenting: TentingSettings {
                    front: TentingType::None,
                    back: TentingType::None,
                },
                uuid: Uuid::new_v4().to_string(),
            },
            PadDescriptor {
                number: "2".to_string(),
                pad_type: PadType::SMD,
                shape: PadShape::RoundRect,
                position: (0.95, 0.0),
                size: (1.0, 1.45),
                drill_size: None,
                layers: vec!["F.Cu".to_string(), "F.Mask".to_string(), "F.Paste".to_string()],
                roundrect_ratio: Some(0.25),
                tenting: TentingSettings {
                    front: TentingType::None,
                    back: TentingType::None,
                },
                uuid: Uuid::new_v4().to_string(),
            },
        ]
    }
    
    fn description(&self) -> Option<String> {
        Some("Resistor SMD 0805 (2012 Metric), square (rectangular) end terminal".to_string())
    }
    
    fn tags(&self) -> Option<String> {
        Some("resistor 0805".to_string())
    }
    
    fn fp_text_elements(&self) -> Vec<FpText> {
        vec![
            FpText {
                text_type: FpTextType::Reference,
                text: "REF**".to_string(),
                position: (0.0, -1.16),
                rotation: None,
                layer: "F.SilkS".to_string(),
                uuid: Uuid::new_v4().to_string(),
                font: FontSettings {
                    size: (1.0, 1.0),
                    thickness: 0.15,
                },
            },
            FpText {
                text_type: FpTextType::Value,
                text: "R_0805_2012Metric".to_string(),
                position: (0.0, 1.16),
                rotation: None,
                layer: "F.Fab".to_string(),
                uuid: Uuid::new_v4().to_string(),
                font: FontSettings {
                    size: (1.0, 1.0),
                    thickness: 0.15,
                },
            },
            FpText {
                text_type: FpTextType::User,
                text: "${REFERENCE}".to_string(),
                position: (0.0, 0.0),
                rotation: None,
                layer: "F.Fab".to_string(),
                uuid: Uuid::new_v4().to_string(),
                font: FontSettings {
                    size: (0.25, 0.25),
                    thickness: 0.04,
                },
            },
        ]
    }
    
    
    fn graphic_elements(&self) -> Vec<GraphicElement> {
        // Additional graphics like silkscreen markings would go here
        vec![]
    }
    
    fn model_3d(&self) -> Option<Model3D> {
        Some(Model3D {
            path: "${KICAD9_3DMODEL_DIR}/Resistor_SMD.3dshapes/R_0805_2012Metric.wrl".to_string(),
            offset: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating KiCad footprint for 0805 resistor...");
    
    // Create the component
    let resistor = SMTResistor0805 { 
        value: "10k".to_string() 
    };
    
    // Generate the footprint
    let footprint_content = copper_exporters::to_kicad_footprint(&resistor);
    
    // Write to file
    std::fs::write("R_0805_2012Metric.kicad_mod", footprint_content)?;
    
    println!("Footprint saved to R_0805_2012Metric.kicad_mod");
    
    Ok(())
}