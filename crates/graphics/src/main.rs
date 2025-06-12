
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release builds.

/// Copper-Graphics Engine
/// 
/// This is a simple example program that demonstrates how to use the three-d library
/// to render a 3D PCB stackup with multiple layers, including copper, prepreg, and core materials.
/// It showcases how to create a custom 3D painting application using eframe and three-d,
/// allowing for interactive rotation, tilt, and zooming of the 3D model.
/// 
/// In general, the goal of copper-subtrate is to provide the foundational architecture to operate
/// in a 3D space, both in terms of placing and routing a PCB, as well as visualizing it in a 3D environment.
/// 
/// This code is a first step. -James <atlantix-eda@proton.me> 


/*
    Example program to show how to use three-d with eframe.

    Code adapted from:
    https://github.com/emilk/egui/blob/08fb447fb55293b2d49343cf5ade2c59d436bc58/examples/custom_3d_glow/src/main.rs
    https://github.com/asny/three-d/blob/0e338e3ccea8ea4187397803eafb8e7f894e0a77/examples/triangle/src/main.rs
    https://github.com/emilk/egui/pull/1407
*/

use std::sync::Arc;

use eframe::{egui, egui::mutex::Mutex, egui_glow, egui_glow::glow};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 1400.0)),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using three-d",
        options,
        Box::new(|cc| Box::new(CuGraphicsApp::new(cc))),
    )
}

struct CuGraphicsApp {
    custom_3d: Arc<Mutex<Custom3d>>,
    angle: f32,
    tilt: f32,
    zoom: f32,
}

impl CuGraphicsApp {
    pub fn new(cc : &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl.as_ref().expect("You need to run eframe with the glow backend!");
        Self {
            custom_3d: Arc::new(Mutex::new(Custom3d::new(gl))),
            angle: 0.0,
            tilt: 0.0,
            zoom: 1.0,
        }
    }
}

impl eframe::App for CuGraphicsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                ui.hyperlink_to("three-d", "https://github.com/asny/three-d");
                ui.label(", a 3D rendering library for Rust.")
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.label("Drag horizontally to rotate, vertically to tilt! Scroll to zoom!");
        });
    }
}

impl CuGraphicsApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(egui::Vec2::splat(1024.0), egui::Sense::drag());

        // Handle drag for rotation and tilt
        self.angle += response.drag_delta().x * 0.01;
        self.tilt += response.drag_delta().y * 0.01;
        
        // Clamp tilt to prevent flipping
        self.tilt = self.tilt.clamp(-1.5, 1.5);

        // Handle scroll wheel for zooming
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                // Zoom in/out with scroll wheel
                self.zoom *= 1.0 + scroll_delta * 0.01;
                self.zoom = self.zoom.clamp(0.3, 5.0); // Limit zoom range to prevent clipping
            }
        }

        let angle = self.angle;
        let tilt = self.tilt;
        let zoom = self.zoom;
        let custom_3d = self.custom_3d.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |info, _painter| {
                custom_3d.lock().paint(&info, angle, tilt, zoom);
            })),
        };
        ui.painter().add(callback);
    }
}


/// Rectangle 3D construction
/// 
/// Construct a 3D rectangle using three-d, which will be painted in the eframe canvas.
/// There are at least four vertices. There could be eight vertices if you want to create a cube.

pub struct Cube {
    positions: Vec<three_d::Vec3>,
    colors: Vec<three_d::Srgba>,
    cpu_mesh: three_d::CpuMesh,
    width: f32,
    height: f32,
    depth: f32,
}

impl Cube {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        use three_d::*;
        // Using width, height, and depth to define the size of the cube.
        let width = width.max(1.0); // Ensure width is at least 1.0
        let height = height.max(1.0); // Ensure height is at least 1.0
        let depth = depth.max(1.0); // Ensure depth is at least 1.0
        let positionsx = vec![
            vec3(width / 2.0, -height / 2.0, depth / 2.0),  // front top right
            vec3(-width / 2.0, -height / 2.0, depth / 2.0), // front top left
            vec3(width / 2.0, height / 2.0, depth / 2.0),   // front bottom right
            vec3(-width / 2.0, height / 2.0, depth / 2.0),  // front bottom left
            vec3(width / 2.0, -height / 2.0, -depth / 2.0), // back top right
            vec3(-width / 2.0, -height / 2.0, -depth / 2.0),// back top left
            vec3(width / 2.0, height / 2.0, -depth / 2.0),  // back bottom right
            vec3(-width / 2.0, height / 2.0, -depth / 2.0), // back bottom left
        ];
        let colorsx = vec![
            // Aluminum/grey colors with subtle variations
            Srgba::new(180, 180, 185, 255),   // vertex 0
            Srgba::new(170, 170, 175, 255),   // vertex 1
            Srgba::new(190, 190, 195, 255),   // vertex 2
            Srgba::new(175, 175, 180, 255),   // vertex 3
            Srgba::new(160, 160, 165, 255),   // vertex 4
            Srgba::new(150, 150, 155, 255),   // vertex 5
            Srgba::new(165, 165, 170, 255),   // vertex 6
            Srgba::new(155, 155, 160, 255),   // vertex 7
        ];
        // Define indices for the cube faces (2 triangles per face, 6 faces total)
        let indices = vec![
            // Front face
            0, 2, 1,
            1, 2, 3,
            // Back face
            4, 5, 6,
            5, 7, 6,
            // Top face
            0, 1, 4,
            1, 5, 4,
            // Bottom face
            2, 6, 3,
            3, 6, 7,
            // Right face
            0, 4, 2,
            2, 4, 6,
            // Left face
            1, 3, 5,
            3, 7, 5,
        ];
        
        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(positionsx.clone()),
            colors: Some(colorsx.clone()),
            indices: Indices::U32(indices),
            ..Default::default()
        };
        
        // Compute normals for proper lighting
        cpu_mesh.compute_normals();

        let positions = positionsx.clone();
        let colors = colorsx.clone();

        Self {
            positions,
            colors,
            cpu_mesh,
            width,
            height,
            depth,
        }
    }

}


struct Custom3d {
    three_d: three_d::Context,
    camera: three_d::Camera,
    layers: Vec<three_d::Gm<three_d::Mesh, three_d::PhysicalMaterial>>,
    ambient_light: three_d::AmbientLight,
    light0: three_d::DirectionalLight,
    light1: three_d::DirectionalLight,
}

impl Custom3d {
    fn create_material(three_d: &three_d::Context, albedo: three_d::Srgba, roughness: f32, metallic: f32) -> three_d::PhysicalMaterial {
        use three_d::*;
        let mut material = PhysicalMaterial::new_opaque(
            three_d,
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
    
    fn create_transparent_material(three_d: &three_d::Context, albedo: three_d::Srgba, roughness: f32, metallic: f32) -> three_d::PhysicalMaterial {
        use three_d::*;
        let mut material = PhysicalMaterial::new_transparent(
            three_d,
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

    fn create_pcb_layer(three_d: &three_d::Context, width: f32, height: f32, thickness: f32, y_pos: f32, material: three_d::PhysicalMaterial) -> three_d::Gm<three_d::Mesh, three_d::PhysicalMaterial> {
        use three_d::*;
        
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
            0, 1, 2, 0, 2, 3,  // Back
            5, 4, 7, 5, 7, 6,  // Front
            4, 0, 3, 4, 3, 7,  // Left
            1, 5, 6, 1, 6, 2,  // Right
            3, 2, 6, 3, 6, 7,  // Top
            4, 5, 1, 4, 1, 0,  // Bottom
        ];
        
        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        };
        cpu_mesh.compute_normals();
        
        Gm::new(Mesh::new(three_d, &cpu_mesh), material)
    }

    fn new(gl: &Arc<glow::Context>) -> Self {
        use three_d::*;

        let three_d = Context::from_gl_context(gl.clone()).unwrap();

        let pcb_width = 50.0;
        let pcb_height = 40.0;
        let copper_thickness = 0.3;     // Make copper layers even thicker for visibility
        let prepreg_thickness = 0.5;    // Make prepreg thicker
        let core_thickness = 1.0;       // Make core thicker
        
        let mut layers = Vec::new();
        // Calculate total thickness including solder mask
        let soldermask_thickness = copper_thickness * 0.5;
        let total_thickness = (soldermask_thickness * 2.0) + (copper_thickness * 4.0) + (prepreg_thickness * 2.0) + core_thickness;
        let mut y_pos = -total_thickness / 2.0; // Start from bottom
        
        // Build stackup from bottom to top
        
        // Bottom solder mask (semi-transparent)
        let bottom_soldermask = Self::create_transparent_material(&three_d, Srgba::new(0, 100, 50, 200), 0.4, 0.0);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, soldermask_thickness, y_pos, bottom_soldermask));
        y_pos += soldermask_thickness;
        
        // Bottom copper (transparent)
        let bottom_copper = Self::create_transparent_material(&three_d, Srgba::new(255, 180, 120, 180), 0.15, 0.98);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, copper_thickness, y_pos, bottom_copper));
        y_pos += copper_thickness;
        
        // Prepreg 1
        let prepreg1 = Self::create_material(&three_d, Srgba::new(90, 90, 85, 255), 0.95, 0.0);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, prepreg_thickness, y_pos, prepreg1));
        y_pos += prepreg_thickness;
        
        // Inner layer 1 (ground plane - transparent)
        let inner1_copper = Self::create_transparent_material(&three_d, Srgba::new(255, 140, 50, 160), 0.2, 0.85);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, copper_thickness, y_pos, inner1_copper));
        y_pos += copper_thickness;
        
        // Core (FR4)
        let core = Self::create_material(&three_d, Srgba::new(80, 80, 75, 255), 0.95, 0.0);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, core_thickness, y_pos, core));
        y_pos += core_thickness;
        
        // Inner layer 2 (power plane - transparent)
        let inner2_copper = Self::create_transparent_material(&three_d, Srgba::new(255, 140, 50, 160), 0.2, 0.85);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, copper_thickness, y_pos, inner2_copper));
        y_pos += copper_thickness;
        
        // Prepreg 2
        let prepreg2 = Self::create_material(&three_d, Srgba::new(100, 100, 95, 255), 0.9, 0.0);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, prepreg_thickness, y_pos, prepreg2));
        y_pos += prepreg_thickness;
        
        // Top copper (transparent)
        let top_copper = Self::create_transparent_material(&three_d, Srgba::new(255, 180, 120, 180), 0.15, 0.98);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, copper_thickness, y_pos, top_copper));
        y_pos += copper_thickness;
        
        // Top solder mask (semi-transparent)
        let top_soldermask = Self::create_transparent_material(&three_d, Srgba::new(0, 100, 50, 200), 0.4, 0.0);
        layers.push(Self::create_pcb_layer(&three_d, pcb_width, pcb_height, soldermask_thickness, y_pos, top_soldermask));

        Self {
            three_d: three_d::Context::from_gl_context(gl.clone()).unwrap(),
            camera: Camera::new_perspective(
                Viewport {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                },
                vec3(20.0, 15.0, 25.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                degrees(45.0),
                0.01,
                1000.0,
            ),
            layers,
            ambient_light: AmbientLight::new(&three_d, 0.7, Srgba::WHITE),
            light0: DirectionalLight::new(&three_d, 0.8, Srgba::WHITE, &vec3(0.0, -0.5, -0.5)),
            light1: DirectionalLight::new(&three_d, 0.8, Srgba::WHITE, &vec3(0.0, 0.5, 0.5)),
        }
    }

    fn paint(&mut self, info: &egui::PaintCallbackInfo, angle: f32, tilt: f32, zoom: f32) {
        use three_d::*;

        let three_d = &self.three_d;
            
        let viewport_pixels = info.viewport_in_pixels();

        let viewport = Viewport {
                x: viewport_pixels.left_px.round() as _,
                y: viewport_pixels.from_bottom_px.round() as _,
                width: viewport_pixels.width_px.round() as _,
                height: viewport_pixels.height_px.round() as _,
        };

        // Update the viewport
        self.camera.set_viewport(viewport);

        // Update camera position based on zoom level
        let base_distance = 40.0 / zoom;
        self.camera.set_view(
            vec3(base_distance * 0.8, base_distance * 0.6, base_distance),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        // Set transformation for all layers (combine rotation and tilt)
        let transformation = Mat4::from_angle_y(radians(angle)) * Mat4::from_angle_x(radians(tilt));
        for layer in &mut self.layers {
            layer.set_transformation(transformation);
        }

        // Get a screen render target
        let screen = RenderTarget::screen(&three_d, viewport.width, viewport.height);
        
        // Clear the screen with scissor test for the viewport
        screen.clear_partially(
            viewport.into(),
            ClearState::color_and_depth(0.05, 0.05, 0.05, 1.0, 1.0)
        );
        
        // Render all layers with proper depth testing
        screen.render_partially(
            viewport.into(),
            &self.camera,
            self.layers.iter(),
            &[&self.ambient_light, &self.light0, &self.light1]
        );
    }
}
