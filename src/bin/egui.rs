use std::sync::Arc;

use eframe::egui;
use keyell::{
    render::{Background, Color, Colorer, Material, Plane, Sphere},
    types::{Normal, Point, Vec3},
    Scene,
};

// TODO: Play with std::mem::discriminant, I just want to get this done right now.
#[derive(Debug, PartialEq)]
enum MaterialType {
    Diffuse,
    Metal,
    Dialectric,
    Light,
}

impl MaterialType {
    fn to_default_material(&self) -> Material {
        match self {
            MaterialType::Diffuse => Material::Diffuse(Colorer::Bubblegum),
            MaterialType::Metal => Material::Metal {
                colorer: Colorer::Solid(Color::WHITE),
                fuzz: 0.,
            },
            MaterialType::Dialectric => Material::Dielectric {
                refraction_index: 0.8,
                colorer: Colorer::Solid(Color::WHITE),
            },
            MaterialType::Light => Material::Light(Colorer::Bubblegum),
        }
    }
}

impl From<&Material> for MaterialType {
    fn from(material: &Material) -> Self {
        match material {
            Material::Diffuse(_) => Self::Diffuse,
            Material::Metal { .. } => Self::Metal,
            Material::Dielectric { .. } => Self::Dialectric,
            Material::Light(_) => Self::Light,
        }
    }
}

#[derive(Debug, PartialEq)]
enum ColorerType {
    ZGradient,
    Solid,
    Bubblegum,
}

impl ColorerType {
    fn to_default_colorer(&self) -> Colorer {
        match self {
            ColorerType::ZGradient => Colorer::ZGradient {
                bottom: Color::BLACK,
                top: Color::WHITE,
            },
            ColorerType::Solid => Colorer::Solid(Color::WHITE),
            ColorerType::Bubblegum => Colorer::Bubblegum,
        }
    }
}

impl From<&Colorer> for ColorerType {
    fn from(colorer: &Colorer) -> Self {
        match colorer {
            Colorer::ZGradient { .. } => ColorerType::ZGradient,
            Colorer::Solid(_) => ColorerType::Solid,
            Colorer::Bubblegum => ColorerType::Bubblegum,
        }
    }
}

fn show_colorer_settings(ui: &mut egui::Ui, colorer: &mut Colorer) -> bool {
    let mut colorer_type = ColorerType::from(colorer as &_);
    let mut changed = false;
    egui::ComboBox::new(colorer as *const _, "colorer")
        .selected_text(format!("{:?}", colorer_type))
        .show_ui(ui, |ui| {
            changed |= ui
                .selectable_value(&mut colorer_type, ColorerType::Bubblegum, "Bubblegum")
                .changed();
            changed |= ui
                .selectable_value(&mut colorer_type, ColorerType::Solid, "Solid")
                .changed();
            changed |= ui
                .selectable_value(&mut colorer_type, ColorerType::ZGradient, "ZGradient")
                .changed();

            if changed {
                *colorer = colorer_type.to_default_colorer();
            }
        });

    changed
}

fn show_background_settings(ui: &mut egui::Ui, background: &mut Background) -> bool {
    let colorer = if let Material::Light(colorer) = &mut background.material {
        colorer
    } else {
        panic!("Expected background material to be a Light");
    };
    show_colorer_settings(ui, colorer)
}

fn show_point_settings(ui: &mut egui::Ui, point: &mut Point) -> bool {
    let mut changed = false;
    changed |= ui
        .add(egui::Slider::new(&mut point.x, (-1.)..=1.).text("x"))
        .changed();
    changed |= ui
        .add(egui::Slider::new(&mut point.y, (0.)..=1.).text("y"))
        .changed();
    changed |= ui
        .add(egui::Slider::new(&mut point.z, (-1.)..=1.).text("z"))
        .changed();
    changed
}

fn show_normal_settings(ui: &mut egui::Ui, normal: &mut Normal) -> bool {
    let mut changed = false;
    let mut show = |mut v: Vec3| {
        changed |= ui
            .add(egui::Slider::new(&mut v.x, (-1.)..=1.).text("x"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut v.y, (-1.)..=1.).text("y"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut v.z, (-1.)..=1.).text("z"))
            .changed();
        v
    };
    match normal {
        Normal::Inward(uv) => {
            *normal = Normal::Inward(show(uv.get().clone()).unit());
        }
        Normal::Outward(uv) => {
            *normal = Normal::Outward(show(uv.get().clone()).unit());
        }
    }
    changed
}

// TODO: don't reset the colorer, which is going to be available no matter the transition
fn show_material_settings(ui: &mut egui::Ui, material: &mut Material) -> bool {
    let mut material_type = MaterialType::from(material as &_);
    let mut changed = false;
    egui::ComboBox::new(material as *const _, "material")
        .selected_text(format!("{:?}", material_type))
        .show_ui(ui, |ui| {
            changed |= ui
                .selectable_value(&mut material_type, MaterialType::Diffuse, "Diffuse")
                .changed();
            changed |= ui
                .selectable_value(&mut material_type, MaterialType::Light, "Light")
                .changed();
            changed |= ui
                .selectable_value(&mut material_type, MaterialType::Metal, "Metal")
                .changed();
            changed |= ui
                .selectable_value(&mut material_type, MaterialType::Dialectric, "Dialectric")
                .changed();
            if changed {
                *material = material_type.to_default_material();
            }
        });

    match material {
        Material::Diffuse(ref mut colorer) | Material::Light(ref mut colorer) => {
            changed |= show_colorer_settings(ui, colorer)
        }
        Material::Metal {
            ref mut colorer,
            ref mut fuzz,
        } => {
            changed |= show_colorer_settings(ui, colorer);
            changed |= ui
                .add(egui::Slider::new(fuzz, (0.)..=2.).text("fuzz"))
                .changed();
        }
        Material::Dielectric {
            ref mut refraction_index,
            ref mut colorer,
        } => {
            changed |= show_colorer_settings(ui, colorer);
            changed |= ui
                .add(egui::Slider::new(refraction_index, (0.)..=2.).text("refraction index"))
                .changed();
        }
    };
    changed
}

fn show_plane_settings(ui: &mut egui::Ui, plane: &mut Plane) -> bool {
    let mut changed = false;
    changed |= show_material_settings(ui, &mut plane.material);
    changed |= show_point_settings(ui, &mut plane.point);
    changed |= show_normal_settings(ui, &mut plane.normal);
    changed
}

fn show_sphere_settings(ui: &mut egui::Ui, sphere: &mut Sphere) -> bool {
    let mut changed = false;
    changed |= show_material_settings(ui, &mut sphere.material);
    changed |= show_point_settings(ui, &mut sphere.center);
    changed |= ui
        .add(egui::Slider::new(&mut sphere.radius, (0.01)..=0.3).text("radius"))
        .changed();
    ui.separator();
    changed
}

fn main() -> Result<(), eframe::Error> {
    const HEIGHT: usize = 500;
    const WIDTH: usize = 500;

    let mut buffer = [0u8; 3 * HEIGHT * WIDTH];
    let mut color_image = Arc::new(egui::ColorImage::from_rgb([HEIGHT, WIDTH], &buffer));

    let mut scene = Scene {
        spheres: Vec::new(),
        planes: Vec::new(),
        background: Background {
            material: Material::Light(Colorer::ZGradient {
                bottom: Color::WHITE,
                top: Color::new(0.4, 0.3, 0.8),
            }),
        },
    };

    let mut samples_per_pixel = 10;
    let mut maximum_bounces = 10;

    let mut render = true;

    eframe::run_simple_native(
        "keyell",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                ui.add(egui::Label::new("Background"));
                render |= show_background_settings(ui, &mut scene.background);
                ui.separator();

                ui.add(egui::Label::new("Spheres"));
                for sphere in &mut scene.spheres {
                    render |= show_sphere_settings(ui, sphere);
                }
                if ui.button("Add sphere").clicked() {
                    scene.spheres.push(Sphere {
                        center: Point::new(0., 0.5, 0.),
                        radius: 0.1,
                        material: Material::Diffuse(Colorer::Solid(Color::random())),
                    });
                    render = true;
                }
                ui.separator();

                ui.add(egui::Label::new("Planes"));
                for plane in &mut scene.planes {
                    render |= show_plane_settings(ui, plane);
                }
                if ui.button("Add plane").clicked() {
                    scene.planes.push(Plane {
                        point: Point::new(0., 0., 0.),
                        normal: Normal::Outward(Vec3::new(0., 0., 1.).unit()),
                        material: Material::Metal {
                            colorer: Colorer::Solid(Color::WHITE),
                            fuzz: 0.,
                        },
                    });
                    render = true;
                }
                ui.separator();

                ui.add(egui::Label::new("Quality"));
                render |= ui
                    .add(
                        egui::Slider::new(&mut samples_per_pixel, 1..=100)
                            .text("samples per pixel"),
                    )
                    .changed();
                render |= ui
                    .add(egui::Slider::new(&mut maximum_bounces, 1..=100).text("maximum bounces"))
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
                        &scene,
                        &canvas,
                        &keyell::render::Camera::from_canvas(
                            &canvas,
                            keyell::types::Point::new(0., 0., 0.05),
                            keyell::render::Degrees::new(90.),
                        ),
                        samples_per_pixel,
                        maximum_bounces,
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
