use std::sync::Arc;

use eframe::egui;
use keyell::{
    render::{Color, Colorer, Material, Plane, Sphere},
    types::{Normal, Point, Vec3},
    Scene,
};

struct SceneParams {
    y1: f32,
    z1: f32,
    y2: f32,
    z2: f32,
    y3: f32,
    z3: f32,
    r3: f32,
    ri3: f32,
    fuzz: f32,
    spp: usize,
    mb: usize,
}

fn make_scene(params: &SceneParams) -> Scene {
    let spheres = vec![
        Sphere {
            center: Point::new(-0.1, params.y1, params.z1),
            radius: 0.1,
            material: Material::Diffuse(Colorer::Solid(Color::new(0.9, 0.2, 0.3))),
        },
        Sphere {
            center: Point::new(0.1, params.y2, params.z2),
            radius: 0.05,
            material: Material::Light(Colorer::Solid(Color::new(0.8, 0.8, 0.8))),
        },
        Sphere {
            center: Point::new(0.0, params.y3, params.z3),
            radius: params.r3,
            material: Material::Dielectric {
                refraction_index: params.ri3,
                colorer: Colorer::Solid(Color::WHITE),
            },
        },
    ];
    let planes = vec![
        Plane {
            point: Point::new(0., 0., 0.),
            normal: Normal::Outward(Vec3::new(0., 0., 1.).unit()),
            material: Material::Metal {
                fuzz: params.fuzz,
                colorer: Colorer::Solid(Color::WHITE),
            },
        },
        Plane {
            point: Point::new(-0.3, 0., 0.),
            normal: Normal::Outward(Vec3::new(1., 0., 0.).unit()),
            material: Material::Metal {
                fuzz: 0.,
                colorer: Colorer::Solid(Color::new(0.2, 0.5, 0.2)),
            },
        },
    ];
    Scene {
        spheres,
        planes,
        background: keyell::render::Background {
            material: Material::Light(Colorer::ZGradient {
                bottom: Color::WHITE,
                top: Color::new(0.4, 0.3, 0.8),
            }),
        },
    }
}

fn main() -> Result<(), eframe::Error> {
    const HEIGHT: usize = 500;
    const WIDTH: usize = 500;

    let mut buffer = [0u8; 3 * HEIGHT * WIDTH];
    let mut color_image = Arc::new(egui::ColorImage::from_rgb([HEIGHT, WIDTH], &buffer));

    let mut scene_params = SceneParams {
        y1: 0.53,
        z1: 0.41,
        y2: 0.72,
        z2: 0.46,
        y3: 0.63,
        z3: 0.19,
        r3: 0.11,
        ri3: 0.12,
        fuzz: 0.02,
        spp: 10,
        mb: 10,
    };
    let mut render = true;

    eframe::run_simple_native(
        "keyell",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                ui.add(egui::Label::new("Spheres"));
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.y1, (0.)..=1.).text("y1"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.z1, (0.)..=1.).text("z1"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.y2, (0.)..=1.).text("y2"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.z2, (0.)..=1.).text("z2"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.y3, (0.)..=1.).text("y3"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.z3, (0.)..=1.).text("z3"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.r3, (0.)..=1.).text("r3"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.ri3, (0.)..=2.).text("ri3"))
                    .changed();
                ui.add(egui::Label::new("Planes"));
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.fuzz, (0.)..=2.).text("fuzz"))
                    .changed();
                ui.add(egui::Label::new("Quality"));
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.spp, 0..=100).text("spp"))
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut scene_params.mb, 0..=100).text("mb"))
                    .changed();
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                let image_data = egui::ImageData::Color(color_image.clone());
                let handle = ctx.load_texture(
                    String::from("pixels"),
                    image_data,
                    egui::TextureOptions::default(),
                );
                ui.add(egui::Image::new(&handle));

                if render {
                    render = false;
                    let mut pixels = vec![keyell::render::Color::BLACK; HEIGHT * WIDTH];
                    let canvas = keyell::render::Canvas {
                        width: WIDTH,
                        height: HEIGHT,
                    };
                    keyell::render_scene(
                        &mut pixels,
                        &make_scene(&scene_params),
                        &canvas,
                        &keyell::render::Camera::from_canvas(
                            &canvas,
                            keyell::types::Point::new(0., 0., 0.05),
                            keyell::render::Degrees::new(90.),
                        ),
                        scene_params.spp,
                        scene_params.mb,
                    );
                    for (triplet, pixel) in buffer.chunks_exact_mut(3).zip(pixels) {
                        triplet[0] = (255.999 * pixel.r).floor() as u8;
                        triplet[1] = (255.999 * pixel.g).floor() as u8;
                        triplet[2] = (255.999 * pixel.b).floor() as u8;
                    }
                    color_image = Arc::new(egui::ColorImage::from_rgb([HEIGHT, WIDTH], &buffer));
                }
            });
        },
    )
}
