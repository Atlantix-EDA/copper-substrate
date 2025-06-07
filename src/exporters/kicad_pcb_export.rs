use std::fmt::Write;
use crate::substrate::board_interface::*;


/// Helper functions for KiCad output formatting
pub fn write_fp_text(output: &mut String, fp_text: &FpText) {
    let text_type_str = match fp_text.text_type {
        FpTextType::Reference => "reference",
        FpTextType::Value => "value",
        FpTextType::User => "user",
    };
    
    write!(output, "\t(fp_text {} \"{}\"", text_type_str, fp_text.text).unwrap();
    
    if let Some(rotation) = fp_text.rotation {
        write!(output, " (at {} {} {})", fp_text.position.0, fp_text.position.1, rotation).unwrap();
    } else {
        write!(output, " (at {} {})", fp_text.position.0, fp_text.position.1).unwrap();
    }
    
    writeln!(output, " (layer \"{}\")", fp_text.layer).unwrap();
    writeln!(output, "\t\t(effects (font (size {} {}) (thickness {})))", 
             fp_text.font.size.0, fp_text.font.size.1, fp_text.font.thickness).unwrap();
    writeln!(output, "\t\t(tstamp \"{}\")", fp_text.uuid).unwrap();
    writeln!(output, "\t)").unwrap();
}

pub fn write_property(output: &mut String, prop: &FootprintProperty) {
    writeln!(output, "\t(property \"{}\" \"{}\"", prop.name, prop.value).unwrap();
    
    // Write position with optional rotation
    if let Some(rotation) = prop.rotation {
        writeln!(output, "\t\t(at {} {} {})", prop.position.0, prop.position.1, rotation).unwrap();
    } else {
        writeln!(output, "\t\t(at {} {} 0)", prop.position.0, prop.position.1).unwrap();
    }
    
    // Write unlocked if true
    if prop.unlocked {
        writeln!(output, "\t\t(unlocked yes)").unwrap();
    }
    
    // Write layer
    writeln!(output, "\t\t(layer \"{}\")", prop.layer).unwrap();
    
    // Write hide if hidden
    if prop.hidden {
        writeln!(output, "\t\t(hide yes)").unwrap();
    }
    
    writeln!(output, "\t\t(uuid \"{}\")", prop.uuid).unwrap();
    writeln!(output, "\t\t(effects").unwrap();
    writeln!(output, "\t\t\t(font").unwrap();
    writeln!(output, "\t\t\t\t(size {} {})", prop.font.size.0, prop.font.size.1).unwrap();
    writeln!(output, "\t\t\t\t(thickness {})", prop.font.thickness).unwrap();
    writeln!(output, "\t\t\t)").unwrap();
    writeln!(output, "\t\t)").unwrap();
    writeln!(output, "\t)").unwrap();
}

pub fn write_graphic_element(output: &mut String, element: &GraphicElement) {
    match &element.element_type {
        GraphicType::Line { start, end } => {
            writeln!(output, "\t(fp_line").unwrap();
            writeln!(output, "\t\t(start {} {})", start.0, start.1).unwrap();
            writeln!(output, "\t\t(end {} {})", end.0, end.1).unwrap();
            writeln!(output, "\t\t(stroke").unwrap();
            writeln!(output, "\t\t\t(width {})", element.stroke.width).unwrap();
            writeln!(output, "\t\t\t(type solid)").unwrap();
            writeln!(output, "\t\t)").unwrap();
            writeln!(output, "\t\t(layer \"{}\")", element.layer.to_kicad_string()).unwrap();
            writeln!(output, "\t\t(tstamp \"{}\")", element.uuid).unwrap();
            writeln!(output, "\t)").unwrap();
        },
        _ => {
            // Implement other graphic types as needed
        }
    }
}

pub fn write_detailed_pad(output: &mut String, pad: &PadDescriptor) {
    write!(output, "\t(pad \"{}\" {} {}", 
           pad.number, 
           match pad.pad_type {
               PadType::SMD => "smd",
               PadType::ThroughHole => "thru_hole",
               PadType::NPTH => "np_thru_hole",
           },
           match pad.shape {
               PadShape::RoundRect => "roundrect",
               PadShape::Rect => "rect",
               PadShape::Circle => "circle",
               PadShape::Oval => "oval",
           }).unwrap();
           
    writeln!(output).unwrap();
    writeln!(output, "\t\t(at {} {})", pad.position.0, pad.position.1).unwrap();
    writeln!(output, "\t\t(size {} {})", pad.size.0, pad.size.1).unwrap();
    
    // Layers
    write!(output, "\t\t(layers").unwrap();
    for layer in &pad.layers {
        write!(output, " \"{}\"", layer).unwrap();
    }
    writeln!(output, ")").unwrap();
    
    // Round rect ratio
    if let Some(ratio) = pad.roundrect_ratio {
        writeln!(output, "\t\t(roundrect_rratio {})", ratio).unwrap();
    }
    
    writeln!(output, "\t\t(tstamp \"{}\")", pad.uuid).unwrap();
    writeln!(output, "\t)").unwrap();
}

pub fn to_kicad_footprint<T: BoardComposableObject>(component: &T) -> String {
    let mut output = String::new();
    
    // Header
    writeln!(output, "(footprint \"{}\"", component.footprint_name()).unwrap();
    writeln!(output, "\t(version 20250401)").unwrap();
    writeln!(output, "\t(generator \"custom_pcb_tool\")").unwrap();
    writeln!(output, "\t(generator_version \"1.0\")").unwrap();
    writeln!(output, "\t(layer \"F.Cu\")").unwrap();
    
    // Description and tags
    if let Some(desc) = component.description() {
        writeln!(output, "\t(descr \"{}\")", desc).unwrap();
    }
    if let Some(tags) = component.tags() {
        writeln!(output, "\t(tags \"{}\")", tags).unwrap();
    }
    
    // Remove properties section as we're using fp_text instead
    
    // Attributes
    let is_smt = component.pad_descriptors().iter().any(|pad| matches!(pad.pad_type, PadType::SMD));
    if is_smt {
        writeln!(output, "\t(attr smd)").unwrap();
    }
    writeln!(output, "\t(duplicate_pad_numbers_are_jumpers no)").unwrap();
    
    // fp_text elements
    for fp_text in component.fp_text_elements() {
        write_fp_text(&mut output, &fp_text);
    }
    
    // Graphic elements (combine user-defined + auto-generated courtyard)
    let mut all_graphics = component.graphic_elements();
    let courtyard = component.generate_courtyard();
    all_graphics.extend(courtyard.to_graphic_elements());
    
    for element in all_graphics {
        write_graphic_element(&mut output, &element);
    }
    
    // Pads
    for pad in component.pad_descriptors() {
        write_detailed_pad(&mut output, &pad);
    }
    
    // 3D model reference
    if let Some(model) = component.model_3d() {
        writeln!(output, "\t(model \"{}\"", model.path).unwrap();
        writeln!(output, "\t\t(offset").unwrap();
        writeln!(output, "\t\t\t(xyz {} {} {})", 
                 model.offset.0, model.offset.1, model.offset.2).unwrap();
        writeln!(output, "\t\t)").unwrap();
        writeln!(output, "\t\t(scale").unwrap();
        writeln!(output, "\t\t\t(xyz {} {} {})", 
                 model.scale.0, model.scale.1, model.scale.2).unwrap();
        writeln!(output, "\t\t)").unwrap();
        writeln!(output, "\t\t(rotate").unwrap();
        writeln!(output, "\t\t\t(xyz {} {} {})", 
                 model.rotation.0, model.rotation.1, model.rotation.2).unwrap();
        writeln!(output, "\t\t)").unwrap();
        writeln!(output, "\t)").unwrap();
    }
    
    writeln!(output, "\t(embedded_fonts no)").unwrap();
    writeln!(output, ")").unwrap();
    output
}