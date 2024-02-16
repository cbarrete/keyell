use std::{
    f32::{INFINITY, MIN_POSITIVE},
    sync::Arc,
};

use eframe::egui;
use keyell::{
    render::{Background, Color, Colorer, Hittable, Material, Plane, Ray, Sphere},
    types::{Normal, Point, Vec3},
    Scene,
};

// TODO: auto scroll on keyboard navigation

#[derive(PartialEq)]
enum Object {
    Sphere(usize),
    Plane(usize),
}

fn get_hit_object(scene: &Scene, ray: &Ray) -> Option<Object> {
    let mut hit_object = None;
    let mut closest_travel = INFINITY;

    for (i, sphere) in scene.spheres.iter().enumerate() {
        if let Some(hit) = sphere.hit(ray, MIN_POSITIVE, closest_travel) {
            hit_object = Some(Object::Sphere(i));
            closest_travel = hit.travel;
        }
    }

    for (i, plane) in scene.planes.iter().enumerate() {
        if let Some(hit) = plane.hit(ray, 0.001, closest_travel) {
            hit_object = Some(Object::Plane(i));
            closest_travel = hit.travel;
        }
    }

    hit_object
}

// TODO: Play with std::mem::discriminant, I just want to get this done right now.
#[derive(Debug, PartialEq)]
enum MaterialType {
    Diffuse,
    Metal,
    Dialectric,
    Light,
}

impl MaterialType {
    fn to_material(&self, colorer: Colorer) -> Material {
        match self {
            MaterialType::Diffuse => Material::Diffuse(colorer),
            MaterialType::Metal => Material::Metal { colorer, fuzz: 0. },
            MaterialType::Dialectric => Material::Dielectric {
                refraction_index: 0.8,
                colorer,
            },
            MaterialType::Light => Material::Light(colorer),
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
    fn to_colorer(&self, previous_color: Color) -> Colorer {
        match self {
            ColorerType::ZGradient => Colorer::ZGradient {
                top: previous_color,
                bottom: Color::WHITE,
            },
            ColorerType::Solid => Colorer::Solid(previous_color),
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
                let previous_color = match colorer {
                    Colorer::ZGradient { top, .. } => top.clone(),
                    Colorer::Solid(c) => c.clone(),
                    Colorer::Bubblegum => Color::random(),
                };
                *colorer = colorer_type.to_colorer(previous_color);
            }
        });

    match colorer {
        Colorer::ZGradient { bottom, top } => {
            let mut top_rgb = [top.r, top.g, top.b];
            changed |= egui::color_picker::color_edit_button_rgb(ui, &mut top_rgb).changed();
            top.r = top_rgb[0];
            top.g = top_rgb[1];
            top.b = top_rgb[2];
            let mut bottom_rgb = [bottom.r, bottom.g, bottom.b];
            changed |= egui::color_picker::color_edit_button_rgb(ui, &mut bottom_rgb).changed();
            bottom.r = bottom_rgb[0];
            bottom.g = bottom_rgb[1];
            bottom.b = bottom_rgb[2];
        }
        Colorer::Solid(ref mut color) => {
            let mut rgb = [color.r, color.g, color.b];
            changed |= egui::color_picker::color_edit_button_rgb(ui, &mut rgb).changed();
            color.r = rgb[0];
            color.g = rgb[1];
            color.b = rgb[2];
        }
        Colorer::Bubblegum => {}
    }

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
                *material = material_type.to_material(material.get_colorer());
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

fn make_frame(ui: &egui::Ui, selected: bool) -> egui::Frame {
    let mut frame = egui::Frame::group(ui.style());
    if selected {
        frame = frame.stroke(egui::Stroke::new(2., egui::Color32::WHITE));
    }
    frame
}

fn show_plane_settings(ui: &mut egui::Ui, plane: &mut Plane, selected: bool) -> bool {
    let mut changed = false;
    make_frame(ui, selected).show(ui, |ui| {
        changed |= show_material_settings(ui, &mut plane.material);
        ui.label("Point");
        changed |= show_point_settings(ui, &mut plane.point);
        ui.label("Normal");
        changed |= show_normal_settings(ui, &mut plane.normal);
    });
    changed
}

fn show_sphere_settings(ui: &mut egui::Ui, sphere: &mut Sphere, selected: bool) -> bool {
    let mut changed = false;
    make_frame(ui, selected).show(ui, |ui| {
        changed |= show_material_settings(ui, &mut sphere.material);
        changed |= show_point_settings(ui, &mut sphere.center);
        changed |= ui
            .add(egui::Slider::new(&mut sphere.radius, (0.01)..=0.3).text("radius"))
            .changed();
    });
    changed
}

fn main() -> Result<(), eframe::Error> {
    const HEIGHT: usize = 500;
    const WIDTH: usize = 500;

    let canvas = keyell::render::Canvas {
        width: WIDTH,
        height: HEIGHT,
    };
    let camera = keyell::render::Camera::from_canvas(
        &canvas,
        keyell::types::Point::new(0., 0., 0.05),
        keyell::render::Degrees::new(90.),
    );

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
    let mut selected_object = Option::<Object>::None;

    let mut render = true;

    eframe::run_simple_native(
        "keyell",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            ctx.input_mut(|i| {
                let selected = match &selected_object {
                    Some(o) => o,
                    None => return,
                };

                let point = match selected {
                    Object::Sphere(i) => &mut scene.spheres[*i].center,
                    Object::Plane(i) => &mut scene.planes[*i].point,
                };

                if i.consume_key(egui::Modifiers::NONE, egui::Key::W) {
                    point.z += 0.01;
                    render = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::S) {
                    point.z -= 0.01;
                    render = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::A) {
                    point.x -= 0.01;
                    render = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::D) {
                    point.x += 0.01;
                    render = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Q) {
                    point.y -= 0.01;
                    render = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::E) {
                    point.y += 0.01;
                    render = true;
                }
            });

            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::Label::new("Background"));
                    render |= show_background_settings(ui, &mut scene.background);
                    ui.separator();

                    egui::CollapsingHeader::new("Spheres")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            for (i, sphere) in scene.spheres.iter_mut().enumerate() {
                                let selected = selected_object == Some(Object::Sphere(i));
                                render |= show_sphere_settings(ui, sphere, selected);
                            }
                            if ui.button("Add sphere").clicked() {
                                scene.spheres.push(Sphere {
                                    center: Point::new(0., 0.5, 0.),
                                    radius: 0.1,
                                    material: Material::Diffuse(Colorer::Solid(Color::random())),
                                });
                                selected_object = Some(Object::Sphere(scene.spheres.len() - 1));
                                render = true;
                            }
                        });
                    ui.separator();

                    egui::CollapsingHeader::new("Planes")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            for (i, plane) in scene.planes.iter_mut().enumerate() {
                                let selected = selected_object == Some(Object::Plane(i));
                                render |= show_plane_settings(ui, plane, selected);
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
                                selected_object = Some(Object::Plane(scene.planes.len() - 1));
                                render = true;
                            }
                        });
                    ui.separator();

                    ui.add(egui::Label::new("Quality"));
                    render |= ui
                        .add(
                            egui::Slider::new(&mut samples_per_pixel, 1..=100)
                                .text("samples per pixel"),
                        )
                        .changed();
                    render |= ui
                        .add(
                            egui::Slider::new(&mut maximum_bounces, 1..=100)
                                .text("maximum bounces"),
                        )
                        .changed();
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                if render {
                    render = false;
                    let mut pixels = vec![keyell::render::Color::BLACK; HEIGHT * WIDTH];
                    keyell::render_scene(
                        &mut pixels,
                        &scene,
                        &canvas,
                        &camera,
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

                let image_data = egui::ImageData::Color(color_image.clone());
                let handle = ctx.load_texture(
                    String::from("pixels"),
                    image_data,
                    egui::TextureOptions::default(),
                );
                let response = ui
                    .add(egui::Image::new(&handle))
                    .interact(egui::Sense::click());
                if let Some(pos) = response.interact_pointer_pos() {
                    let rect = response.rect;
                    let x = pos.x - rect.min.x;
                    let y = pos.y - rect.min.y;

                    selected_object = get_hit_object(
                        &scene,
                        &camera.get_ray(
                            x / canvas.width as f32,
                            (canvas.height as f32 - y) / canvas.height as f32,
                        ),
                    );
                }
            });
        },
    )
}
