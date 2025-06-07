use copper_substrate::prelude::*;
use uuid::Uuid;

struct SMTCapacitor0805 {
    value: String,
}

impl BoardComposableObject for SMTCapacitor0805 {

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
        FunctionalType::Capacitor(self.value.clone())
    }
    
    fn footprint_name(&self) -> String {
        "C_0402_1005Metric".to_string()
    }
    
    fn library_name(&self) -> String {
        "Capacitor_SMD".to_string()
    }
    
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            min_x: -0.5,
            min_y: -0.25,
            max_x: 0.5,
            max_y: 0.25,
        }
    }
    
    fn pad_descriptors(&self) -> Vec<PadDescriptor> {
        vec![
            PadDescriptor {
                number: "1".to_string(),
                pad_type: PadType::SMD,
                shape: PadShape::RoundRect,
                position: (-0.48, 0.0),
                size: (0.56, 0.62),
                drill_size: None,
                layers: vec!["F.Cu".to_string(), "F.Paste".to_string(), "F.Mask".to_string()],
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
                position: (0.48, 0.0),
                size: (0.56, 0.62),
                drill_size: None,
                layers: vec!["F.Cu".to_string(), "F.Paste".to_string(), "F.Mask".to_string()],
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
        Some("Capacitor SMD 0805 (2012 Metric), square (rectangular) end terminal, IPC_7351 nominal, (Body size source: IPC-SM-782 page 76, https://www.pcb-3d.com/wordpress/wp-content/uploads/ipc-sm-782a_amendment_1_and_2.pdf), generated with kicad-footprint-generator".to_string())
    }
    
    fn tags(&self) -> Option<String> {
        Some("capacitor".to_string())
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
                text: "C_0402_1005Metric".to_string(),
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
        vec![
            // Silkscreen lines
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (-0.107836, -0.36),
                    end: (0.107836, -0.36),
                },
                layer: LayerType::SilkScreen,
                stroke: Stroke { width: 0.12, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (-0.107836, 0.36),
                    end: (0.107836, 0.36),
                },
                layer: LayerType::SilkScreen,
                stroke: Stroke { width: 0.12, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            // Fab layer outline
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (-0.5, -0.25),
                    end: (0.5, -0.25),
                },
                layer: LayerType::Fabrication,
                stroke: Stroke { width: 0.1, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (-0.5, 0.25),
                    end: (-0.5, -0.25),
                },
                layer: LayerType::Fabrication,
                stroke: Stroke { width: 0.1, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (0.5, -0.25),
                    end: (0.5, 0.25),
                },
                layer: LayerType::Fabrication,
                stroke: Stroke { width: 0.1, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (0.5, 0.25),
                    end: (-0.5, 0.25),
                },
                layer: LayerType::Fabrication,
                stroke: Stroke { width: 0.1, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
        ]
    }
    
    fn model_3d(&self) -> Option<Model3D> {
        Some(Model3D {
            path: "${KICAD6_3DMODEL_DIR}/Capacitor_SMD.3dshapes/C_0402_1005Metric.wrl".to_string(),
            offset: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
        })
    }
    
    fn courtyard_margin(&self) -> f32 { 
        0.41 // 0402 needs larger margin to match KiCad standard
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating KiCad footprint for 0402 capacitor...");
    
    // Create the component
    let cap = SMTCapacitor0805 { 
        value: "100nF".to_string() 
    };
    
    // Generate the footprint
    let footprint_content = cap.to_kicad_footprint();
    
    // Write to file
    std::fs::write("C_0402_1005Metric.kicad_mod", footprint_content)?;
    
    println!("Footprint saved to C_0402_1005Metric.kicad_mod");
    
    Ok(())
}