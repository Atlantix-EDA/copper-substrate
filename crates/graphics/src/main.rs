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
use copper_graphics::presets;

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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("three-d can only be run with the glow backend");

        let custom_3d = Arc::new(Mutex::new(Custom3d::new(gl)));

        Self {
            custom_3d,
            angle: 0.0,
            tilt: 0.0,
            zoom: 1.0,
        }
    }
}

impl eframe::App for CuGraphicsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("View Controls");
            
            ui.add(egui::Slider::new(&mut self.angle, -180.0..=180.0).text("Rotation"));
            ui.add(egui::Slider::new(&mut self.tilt, -90.0..=90.0).text("Tilt"));
            ui.add(egui::Slider::new(&mut self.zoom, 0.1..=3.0).text("Zoom"));
            
            if ui.button("Reset View").clicked() {
                self.angle = 0.0;
                self.tilt = 0.0;
                self.zoom = 1.0;
            }
            
            ui.separator();
            
            ui.heading("PCB Stack-up");
            ui.label("4-Layer Board");
            ui.label("• Top Solder Mask");
            ui.label("• Top Copper (Signal)");
            ui.label("• Prepreg");
            ui.label("• Inner Layer 1 (GND)");
            ui.label("• Core (FR4)");
            ui.label("• Inner Layer 2 (PWR)");
            ui.label("• Prepreg");
            ui.label("• Bottom Copper (Signal)");
            ui.label("• Bottom Solder Mask");
            
            ui.separator();
            
            ui.label("Powered by:");
            ui.hyperlink("https://github.com/emilk/egui");
            ui.hyperlink("https://github.com/asny/three-d");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("3D PCB Stackup Visualization");
            
            // Create the frame for the 3D scene
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_3d_glow_painter(ui);
            });
        });
    }
}

impl CuGraphicsApp {
    fn custom_3d_glow_painter(&mut self, ui: &mut egui::Ui) {
        use egui_glow::CallbackFn;
        
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        // Handle drag for rotation and tilt
        self.angle += response.drag_delta().x * 0.01;
        self.tilt += response.drag_delta().y * 0.01;
        
        // Clamp tilt to prevent flipping
        self.tilt = self.tilt.clamp(-89.0, 89.0);

        // Handle scroll wheel for zooming
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                // Zoom in/out with scroll wheel
                self.zoom *= 1.0 + scroll_delta * 0.01;
                self.zoom = self.zoom.clamp(0.1, 3.0);
            }
        }

        let angle = self.angle;
        let tilt = self.tilt;
        let zoom = self.zoom;
        
        let custom_3d = self.custom_3d.clone();
        let callback = CallbackFn::new(move |info, _painter| {
            custom_3d.lock().paint(&info, angle, tilt, zoom);
        });

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(callback),
        };

        ui.painter().add(callback);
    }
}


/// Rectangle 3D construction


struct Custom3d {
    three_d: three_d::Context,
    camera: three_d::Camera,
    stack_renderer: copper_graphics::PcbStackRenderer,
    ambient_light: three_d::AmbientLight,
    light0: three_d::DirectionalLight,
    light1: three_d::DirectionalLight,
}

impl Custom3d {
    fn new(gl: &Arc<glow::Context>) -> Self {
        use three_d::*;

        // Create three-d context
        let three_d = three_d::Context::from_gl_context(gl.clone()).unwrap();
        
        // Create a standard 4-layer PCB stack
        let mut stack_renderer = presets::standard_4_layer_stack();
        stack_renderer.center_stack(); // Center the stack around Y=0
        stack_renderer.build_stack(&three_d);

        Self {
            three_d: three_d.clone(),
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
            stack_renderer,
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
        for layer in self.stack_renderer.rendered_layers_mut() {
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
            self.stack_renderer.rendered_layers().iter(),
            &[&self.ambient_light, &self.light0, &self.light1]
        );
    }
}