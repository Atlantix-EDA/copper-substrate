//! Copper Graphics Library
//! 
//! A 3D PCB visualization library built on three-d and egui for rendering
//! PCB stackups, layers, and components in an interactive 3D environment.

use three_d::*;

/// Represents different types of PCB layers with their visual properties
#[derive(Debug, Clone)]
pub enum LayerType {
    /// Copper layer (signal, power, ground)
    Copper { thickness: f32, color: Srgba },
    /// Prepreg dielectric layer
    Prepreg { thickness: f32, color: Srgba },
    /// Core dielectric layer  
    Core { thickness: f32, color: Srgba },
    /// Solder mask layer
    SolderMask { thickness: f32, color: Srgba },
    /// Silkscreen layer
    Silkscreen { thickness: f32, color: Srgba },
}

impl LayerType {
    /// Get the thickness of this layer
    pub fn thickness(&self) -> f32 {
        match self {
            LayerType::Copper { thickness, .. } => *thickness,
            LayerType::Prepreg { thickness, .. } => *thickness,
            LayerType::Core { thickness, .. } => *thickness,
            LayerType::SolderMask { thickness, .. } => *thickness,
            LayerType::Silkscreen { thickness, .. } => *thickness,
        }
    }
    
    /// Get the color of this layer
    pub fn color(&self) -> Srgba {
        match self {
            LayerType::Copper { color, .. } => *color,
            LayerType::Prepreg { color, .. } => *color,
            LayerType::Core { color, .. } => *color,
            LayerType::SolderMask { color, .. } => *color,
            LayerType::Silkscreen { color, .. } => *color,
        }
    }
    
    /// Get material properties for this layer type
    pub fn material_properties(&self) -> (f32, f32) {
        match self {
            LayerType::Copper { .. } => (0.1, 0.9), // low roughness, high metallic
            LayerType::Prepreg { .. } => (0.8, 0.0), // high roughness, non-metallic
            LayerType::Core { .. } => (0.7, 0.0), // medium roughness, non-metallic
            LayerType::SolderMask { .. } => (0.4, 0.0), // medium-low roughness, non-metallic
            LayerType::Silkscreen { .. } => (0.6, 0.0), // medium roughness, non-metallic
        }
    }
}

/// PCB Layer rendering structure
#[derive(Debug)]
pub struct PcbLayer {
    pub layer_type: LayerType,
    pub width: f32,
    pub height: f32,
    pub position_y: f32,
    pub name: String,
}

impl PcbLayer {
    /// Create a new PCB layer
    pub fn new(layer_type: LayerType, width: f32, height: f32, position_y: f32, name: String) -> Self {
        Self {
            layer_type,
            width,
            height,
            position_y,
            name,
        }
    }
}

/// Material factory for creating three-d materials
pub struct MaterialFactory;

impl MaterialFactory {
    /// Create an opaque physical material
    pub fn create_opaque_material(
        context: &Context,
        albedo: Srgba,
        roughness: f32,
        metallic: f32,
    ) -> PhysicalMaterial {
        let mut material = PhysicalMaterial::new_opaque(
            context,
            &CpuMaterial {
                albedo,
                roughness,
                metallic,
                ..Default::default()
            },
        );
        material.render_states.cull = Cull::Back;
        material
    }
    
    /// Create a transparent physical material
    pub fn create_transparent_material(
        context: &Context,
        albedo: Srgba,
        roughness: f32,
        metallic: f32,
    ) -> PhysicalMaterial {
        let mut material = PhysicalMaterial::new_transparent(
            context,
            &CpuMaterial {
                albedo,
                roughness,
                metallic,
                ..Default::default()
            },
        );
        material.render_states.cull = Cull::Back;
        material.render_states.blend = Blend::TRANSPARENCY;
        material
    }
    
    /// Create material from layer type
    pub fn material_from_layer(context: &Context, layer: &LayerType) -> PhysicalMaterial {
        let (roughness, metallic) = layer.material_properties();
        let color = layer.color();
        
        match layer {
            LayerType::Copper { .. } | LayerType::SolderMask { .. } | LayerType::Prepreg { .. } => {
                // Make copper layers transparent so we can see through the stack
                Self::create_transparent_material(context, color, roughness, metallic)
            }
            _ => {
                Self::create_opaque_material(context, color, roughness, metallic)
            }
        }
    }
}

/// Layer mesh factory for creating 3D layer geometries
pub struct LayerMeshFactory;

impl LayerMeshFactory {
    /// Create a rectangular PCB layer mesh
    pub fn create_layer_mesh(
        context: &Context,
        layer: &PcbLayer,
    ) -> Gm<Mesh, PhysicalMaterial> {
        let width = layer.width;
        let height = layer.height;
        let thickness = layer.layer_type.thickness();
        let y_pos = layer.position_y;
        
        let positions = vec![
            vec3(-width/2.0, y_pos - thickness/2.0, -height/2.0),
            vec3( width/2.0, y_pos - thickness/2.0, -height/2.0),
            vec3( width/2.0, y_pos + thickness/2.0, -height/2.0),
            vec3(-width/2.0, y_pos + thickness/2.0, -height/2.0),
            vec3(-width/2.0, y_pos - thickness/2.0,  height/2.0),
            vec3( width/2.0, y_pos - thickness/2.0,  height/2.0),
            vec3( width/2.0, y_pos + thickness/2.0,  height/2.0),
            vec3(-width/2.0, y_pos + thickness/2.0,  height/2.0),
        ];
        
        let indices = vec![
            // Bottom face
            0, 2, 1, 0, 3, 2,
            // Top face  
            4, 5, 6, 4, 6, 7,
            // Front face
            0, 1, 5, 0, 5, 4,
            // Back face
            2, 7, 6, 2, 3, 7,
            // Left face
            0, 4, 7, 0, 7, 3,
            // Right face
            1, 2, 6, 1, 6, 5,
        ];
        
        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        };
        
        cpu_mesh.compute_normals();
        
        let material = MaterialFactory::material_from_layer(context, &layer.layer_type);
        let mesh = Mesh::new(context, &cpu_mesh);
        
        Gm::new(mesh, material)
    }
}

/// PCB Stack renderer for managing multiple layers
pub struct PcbStackRenderer {
    pub layers: Vec<PcbLayer>,
    rendered_layers: Vec<Gm<Mesh, PhysicalMaterial>>,
    auto_position: bool,
}

impl PcbStackRenderer {
    /// Create a new PCB stack renderer
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            rendered_layers: Vec::new(),
            auto_position: true,
        }
    }
    
    /// Create a new PCB stack renderer with manual positioning
    pub fn new_manual() -> Self {
        Self {
            layers: Vec::new(),
            rendered_layers: Vec::new(),
            auto_position: false,
        }
    }
    
    /// Add a layer to the stack
    pub fn add_layer(&mut self, mut layer: PcbLayer) {
        if self.auto_position && !self.layers.is_empty() {
            // Calculate Y position based on previous layers
            let total_height: f32 = self.layers.iter()
                .map(|l| l.layer_type.thickness())
                .sum();
            layer.position_y = total_height;
        }
        self.layers.push(layer);
    }
    
    /// Add multiple layers at once
    pub fn add_layers(&mut self, layers: impl IntoIterator<Item = PcbLayer>) {
        for layer in layers {
            self.add_layer(layer);
        }
    }
    
    /// Build the rendered stack from the layer definitions
    pub fn build_stack(&mut self, context: &Context) {
        self.rendered_layers.clear();
        
        for layer in &self.layers {
            let rendered_layer = LayerMeshFactory::create_layer_mesh(context, layer);
            self.rendered_layers.push(rendered_layer);
        }
    }
    
    /// Get reference to rendered layers for drawing
    pub fn rendered_layers(&self) -> &[Gm<Mesh, PhysicalMaterial>] {
        &self.rendered_layers
    }
    
    /// Get mutable reference to rendered layers for transformations
    pub fn rendered_layers_mut(&mut self) -> &mut [Gm<Mesh, PhysicalMaterial>] {
        &mut self.rendered_layers
    }
    
    /// Calculate total stack height
    pub fn total_height(&self) -> f32 {
        self.layers.iter().map(|l| l.layer_type.thickness()).sum()
    }
    
    /// Get layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
    
    /// Clear all layers
    pub fn clear(&mut self) {
        self.layers.clear();
        self.rendered_layers.clear();
    }
    
    /// Center the stack around Y=0
    pub fn center_stack(&mut self) {
        let total_height = self.total_height();
        let offset = total_height / 2.0;
        
        let mut current_y = -offset;
        for layer in &mut self.layers {
            layer.position_y = current_y + layer.layer_type.thickness() / 2.0;
            current_y += layer.layer_type.thickness();
        }
    }
}

impl Default for PcbStackRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined layer configurations
pub mod presets {
    use super::*;
    
    /// Create a standard 4-layer PCB stack
    pub fn standard_4_layer_stack() -> PcbStackRenderer {
        let mut stack = PcbStackRenderer::new();
        
        let mut y_offset = 0.0;
        
        // Top solder mask
        let solder_mask_top = PcbLayer::new(
            LayerType::SolderMask { 
                thickness: 0.025, 
                color: Srgba::new(0, 120, 0, 180) 
            },
            50.0, 50.0, y_offset, "Top Solder Mask".to_string()
        );
        y_offset += solder_mask_top.layer_type.thickness();
        stack.add_layer(solder_mask_top);
        
        // Top copper
        let top_copper = PcbLayer::new(
            LayerType::Copper { 
                thickness: 0.035, 
                color: Srgba::new(255, 180, 120, 180) 
            },
            50.0, 50.0, y_offset, "Top Copper".to_string()
        );
        y_offset += top_copper.layer_type.thickness();
        stack.add_layer(top_copper);
        
        // Prepreg
        let prepreg = PcbLayer::new(
            LayerType::Prepreg { 
                thickness: 0.2, 
                color: Srgba::new(90, 90, 85, 240) 
            },
            50.0, 50.0, y_offset, "Prepreg".to_string()
        );
        y_offset += prepreg.layer_type.thickness();
        stack.add_layer(prepreg);
        
        // Inner copper 1
        let inner1 = PcbLayer::new(
            LayerType::Copper { 
                thickness: 0.035, 
                color: Srgba::new(255, 140, 50, 160) 
            },
            50.0, 50.0, y_offset, "Inner 1".to_string()
        );
        y_offset += inner1.layer_type.thickness();
        stack.add_layer(inner1);
        
        // Core
        let core = PcbLayer::new(
            LayerType::Core { 
                thickness: 1.2, 
                color: Srgba::new(80, 80, 75, 255) 
            },
            50.0, 50.0, y_offset, "Core".to_string()
        );
        y_offset += core.layer_type.thickness();
        stack.add_layer(core);
        
        // Inner copper 2
        let inner2 = PcbLayer::new(
            LayerType::Copper { 
                thickness: 0.035, 
                color: Srgba::new(255, 140, 50, 160) 
            },
            50.0, 50.0, y_offset, "Inner 2".to_string()
        );
        y_offset += inner2.layer_type.thickness();
        stack.add_layer(inner2);
        
        // Prepreg
        let prepreg2 = PcbLayer::new(
            LayerType::Prepreg { 
                thickness: 0.2, 
                color: Srgba::new(100, 100, 95, 240) 
            },
            50.0, 50.0, y_offset, "Prepreg 2".to_string()
        );
        y_offset += prepreg2.layer_type.thickness();
        stack.add_layer(prepreg2);
        
        // Bottom copper
        let bottom_copper = PcbLayer::new(
            LayerType::Copper { 
                thickness: 0.035, 
                color: Srgba::new(255, 180, 120, 180) 
            },
            50.0, 50.0, y_offset, "Bottom Copper".to_string()
        );
        y_offset += bottom_copper.layer_type.thickness();
        stack.add_layer(bottom_copper);
        
        // Bottom solder mask
        let solder_mask_bottom = PcbLayer::new(
            LayerType::SolderMask { 
                thickness: 0.025, 
                color: Srgba::new(0, 120, 0, 180) 
            },
            50.0, 50.0, y_offset, "Bottom Solder Mask".to_string()
        );
        stack.add_layer(solder_mask_bottom);
        
        stack
    }
}

/// Macro for easily creating layer stacks
#[macro_export]
macro_rules! pcb_stack {
    (
        $(
            $layer_type:ident {
                thickness: $thickness:expr,
                color: $color:expr,
                width: $width:expr,
                height: $height:expr,
                name: $name:expr
            }
        ),* $(,)?
    ) => {
        {
            let mut stack = $crate::PcbStackRenderer::new();
            let mut y_offset = 0.0f32;
            
            $(
                let layer = $crate::PcbLayer::new(
                    $crate::LayerType::$layer_type { 
                        thickness: $thickness, 
                        color: $color 
                    },
                    $width, $height, y_offset, $name.to_string()
                );
                y_offset += layer.layer_type.thickness();
                stack.add_layer(layer);
            )*
            
            stack
        }
    };
}