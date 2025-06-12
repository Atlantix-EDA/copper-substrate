use uuid::Uuid;
use crate::layer_type::LayerType;
use crate::board_interface::{Rectangle, GraphicElement, GraphicType, Stroke, StrokeType};

/// Courtyard structure
#[derive(Debug, Clone)]
pub struct Courtyard {
    pub bounds: Rectangle,
    pub margin: f32,
    pub layer: LayerType, // Usually F.CrtYd or B.CrtYd
}

impl Courtyard {
    pub fn new(bounds: Rectangle, margin: f32) -> Self {
        Self {
            bounds: Rectangle {
                min_x: bounds.min_x - margin,
                min_y: bounds.min_y - margin,
                max_x: bounds.max_x + margin,
                max_y: bounds.max_y + margin,
            },
            margin,
            layer: LayerType::Courtyard,
        }
    }
    
    pub fn to_graphic_elements(&self) -> Vec<GraphicElement> {
        vec![
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (self.bounds.min_x, self.bounds.min_y),
                    end: (self.bounds.max_x, self.bounds.min_y),
                },
                layer: self.layer.clone(),
                stroke: Stroke { width: 0.05, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (self.bounds.max_x, self.bounds.min_y),
                    end: (self.bounds.max_x, self.bounds.max_y),
                },
                layer: self.layer.clone(),
                stroke: Stroke { width: 0.05, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (self.bounds.max_x, self.bounds.max_y),
                    end: (self.bounds.min_x, self.bounds.max_y),
                },
                layer: self.layer.clone(),
                stroke: Stroke { width: 0.05, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
            GraphicElement {
                element_type: GraphicType::Line {
                    start: (self.bounds.min_x, self.bounds.max_y),
                    end: (self.bounds.min_x, self.bounds.min_y),
                },
                layer: self.layer.clone(),
                stroke: Stroke { width: 0.05, stroke_type: StrokeType::Solid },
                uuid: Uuid::new_v4().to_string(),
            },
        ]
    }
}