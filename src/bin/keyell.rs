use std::{
    f32::{INFINITY, MIN_POSITIVE},
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    sync::Arc,
};

use eframe::egui;
use keyell::{
    net::Remote,
    render::{Background, Color, Colorer, Hittable, Material, Plane, Ray, Sphere},
    types::{Normal, Point, Vec3},
    Scene,
};

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

struct PreviewState {
    samples_per_pixel: usize,
    maximum_bounces: usize,
    canvas: keyell::render::Canvas,
    buffer: Vec<u8>,
    texture_handle: Option<egui::TextureHandle>,
}

impl PreviewState {
    fn new() -> Self {
        let width = 640;
        let height = 360;
        Self {
            samples_per_pixel: 10,
            maximum_bounces: 10,
            canvas: keyell::render::Canvas { width, height },
            buffer: vec![0u8; 3 * width * height],
            texture_handle: None,
        }
    }
}

struct ExportParams {
    remotes: Vec<Remote>,
    samples_per_pixel: usize,
    maximum_bounces: usize,
    canvas: keyell::render::Canvas,
    file_name: String,
    overwrite: bool,
}

impl ExportParams {
    fn new() -> Self {
        Self {
            remotes: Vec::new(),
            samples_per_pixel: 100,
            maximum_bounces: 30,
            canvas: keyell::render::Canvas {
                width: 1920,
                height: 1080,
            },
            file_name: String::from("out"),
            overwrite: false,
        }
    }
}

struct Status {
    color: egui::Color32,
    text: String,
}

fn load_scene(file_name: &str, scene: &mut Scene, status: &mut Status) {
    let file_name = format!("{file_name}.json");
    let file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            status.color = egui::Color32::RED;
            status.text = format!("Failed to open {file_name}: {e}");
            return;
        }
    };

    match serde_json::from_reader(BufReader::new(file)) {
        Ok(s) => *scene = s,
        Err(e) => {
            status.color = egui::Color32::RED;
            status.text = format!("Failed to open {file_name}: {e}");
            return;
        }
    }

    status.color = egui::Color32::GREEN;
    status.text = format!("Loaded scene from {file_name}");
}

fn create_file(file_name: &str, overwrite: bool, status: &mut Status) -> Option<File> {
    let mut open_options = OpenOptions::new();
    if overwrite {
        open_options.create(true);
    } else {
        open_options.create_new(true);
    }

    open_options
        .write(true)
        .open(file_name)
        .map_err(|e| {
            status.color = egui::Color32::RED;
            status.text = format!("Failed to create {file_name}: {e}");
        })
        .ok()
}

fn save_scene(file_name: &str, scene: &Scene, status: &mut Status, overwrite: bool) {
    let file = match create_file(&format!("{file_name}.json"), overwrite, status) {
        Some(f) => f,
        None => return,
    };

    if let Err(e) = serde_json::to_writer(BufWriter::new(file), scene) {
        status.color = egui::Color32::RED;
        status.text = format!("Failed to save scene to {file_name}: {e}");
        return;
    }

    status.color = egui::Color32::GREEN;
    status.text = format!("Saved scene to {file_name}");
}

fn export_file(
    file_name: &str,
    scene: &Scene,
    params: &ExportParams,
    status: &mut Status,
    overwrite: bool,
) {
    let file = match create_file(&format!("{file_name}.ppm"), overwrite, status) {
        Some(f) => f,
        None => return,
    };

    let camera = keyell::render::Camera::from_canvas(
        &params.canvas,
        keyell::types::Point::new(0., 0., 0.05),
        keyell::render::Degrees::new(90.),
    );
    let mut pixels = vec![keyell::render::Color::BLACK; params.canvas.height * params.canvas.width];
    keyell::net::render_scene_distributed(
        &params.remotes,
        &mut pixels,
        &scene,
        &params.canvas,
        &camera,
        params.samples_per_pixel,
        params.maximum_bounces,
    );

    let mut writer = keyell::ppm::PpmWriter::new(BufWriter::new(file), &params.canvas);
    writer.write_header().unwrap();
    for pixel in pixels {
        writer.write_pixel(&pixel).unwrap();
    }

    status.color = egui::Color32::GREEN;
    status.text = format!("Exported to {file_name}");
}

fn main() -> Result<(), eframe::Error> {
    let mut preview = PreviewState::new();
    let mut export = ExportParams::new();

    let mut selected_object = Option::<Object>::None;
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

    let mut status = Status {
        color: egui::Color32::GREEN,
        text: String::new(),
    };

    let mut render_preview = true;

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
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::S) {
                    point.z -= 0.01;
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::A) {
                    point.x -= 0.01;
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::D) {
                    point.x += 0.01;
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Q) {
                    point.y -= 0.01;
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::E) {
                    point.y += 0.01;
                    render_preview = true;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::X) {
                    if let Some(o) = &selected_object {
                        match o {
                            Object::Sphere(i) => {
                                scene.spheres.remove(*i);
                                let i = (*i).min(scene.spheres.len() - 1);
                                selected_object = scene.spheres.get(i).map(|_| Object::Sphere(i));
                            }
                            Object::Plane(i) => {
                                scene.planes.remove(*i);
                                let i = (*i).min(scene.planes.len() - 1);
                                selected_object = scene.planes.get(i).map(|_| Object::Plane(i));
                            }
                        }
                        render_preview = true;
                    }
                }
            });

            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::CollapsingHeader::new("Preview")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            render_preview |= ui
                                .add(
                                    egui::Slider::new(&mut preview.samples_per_pixel, 1..=10)
                                        .text("samples per pixel"),
                                )
                                .changed();
                            render_preview |= ui
                                .add(
                                    egui::Slider::new(&mut preview.maximum_bounces, 1..=100)
                                        .text("maximum bounces"),
                                )
                                .changed();
                            ui.horizontal(|ui| {
                                render_preview |= ui
                                    .add(egui::DragValue::new(&mut preview.canvas.width))
                                    .changed();
                                ui.label("x");
                                render_preview |= ui
                                    .add(egui::DragValue::new(&mut preview.canvas.height))
                                    .changed();
                            });
                        });
                    ui.separator();

                    egui::CollapsingHeader::new("File")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut export.samples_per_pixel));
                                ui.label("samples per pixel");
                            });
                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut export.maximum_bounces));
                                ui.label("maximum bounces");
                            });

                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut export.canvas.width));
                                ui.label("x");
                                ui.add(egui::DragValue::new(&mut export.canvas.height));
                            });

                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut export.file_name);
                                ui.checkbox(&mut export.overwrite, "overwrite");
                            });

                            ui.horizontal(|ui| {
                                if ui.button("Export").clicked() {
                                    export_file(
                                        &export.file_name,
                                        &scene,
                                        &export,
                                        &mut status,
                                        export.overwrite,
                                    );
                                }

                                if ui.button("Save scene").clicked() {
                                    save_scene(
                                        &export.file_name,
                                        &scene,
                                        &mut status,
                                        export.overwrite,
                                    );
                                }

                                if ui.button("Load scene").clicked() {
                                    load_scene(&export.file_name, &mut scene, &mut status);
                                    render_preview = true;
                                }
                            });

                            for remote in &mut export.remotes {
                                ui.text_edit_singleline(&mut remote.ip);
                                ui.add(
                                    egui::Slider::new(&mut remote.rows, 0..=export.canvas.height)
                                        .text("rows"),
                                );
                            }

                            if ui.button("Add remote").clicked() {
                                export.remotes.push(Remote {
                                    ip: String::from("192.168.1.129:3544"),
                                    rows: 3 * export.canvas.height / 4,
                                });
                            };
                        });
                    ui.separator();

                    egui::CollapsingHeader::new("Background")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            render_preview |= show_background_settings(ui, &mut scene.background);
                            ui.separator();
                        });

                    egui::CollapsingHeader::new("Spheres")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
                            if ui.button("Add sphere").clicked() {
                                scene.spheres.push(Sphere {
                                    center: Point::new(0., 0.5, 0.),
                                    radius: 0.1,
                                    material: Material::Diffuse(Colorer::Solid(Color::random())),
                                });
                                selected_object = Some(Object::Sphere(scene.spheres.len() - 1));
                                render_preview = true;
                            }
                            for (i, sphere) in scene.spheres.iter_mut().enumerate() {
                                let selected = selected_object == Some(Object::Sphere(i));
                                render_preview |= show_sphere_settings(ui, sphere, selected);
                            }
                        });
                    ui.separator();

                    egui::CollapsingHeader::new("Planes")
                        .default_open(true)
                        .show_unindented(ui, |ui| {
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
                                render_preview = true;
                            }
                            for (i, plane) in scene.planes.iter_mut().enumerate() {
                                let selected = selected_object == Some(Object::Plane(i));
                                render_preview |= show_plane_settings(ui, plane, selected);
                            }
                        });
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                let camera = keyell::render::Camera::from_canvas(
                    &preview.canvas,
                    keyell::types::Point::new(0., 0., 0.05),
                    keyell::render::Degrees::new(90.),
                );

                if render_preview {
                    render_preview = false;
                    let mut pixels = vec![
                        keyell::render::Color::BLACK;
                        preview.canvas.height * preview.canvas.width
                    ];
                    keyell::render_scene(
                        &mut pixels,
                        &scene,
                        &preview.canvas,
                        &camera,
                        preview.samples_per_pixel,
                        preview.maximum_bounces,
                        0..preview.canvas.height,
                    );
                    preview
                        .buffer
                        .resize(3 * preview.canvas.height * preview.canvas.width, 0);
                    for (triplet, pixel) in preview.buffer.chunks_exact_mut(3).zip(pixels) {
                        triplet[0] = (255.999 * pixel.r).floor() as u8;
                        triplet[1] = (255.999 * pixel.g).floor() as u8;
                        triplet[2] = (255.999 * pixel.b).floor() as u8;
                    }
                    preview.texture_handle = Some(ctx.load_texture(
                        String::from("pixels"),
                        egui::ImageData::Color(Arc::new(egui::ColorImage::from_rgb(
                            [preview.canvas.width, preview.canvas.height],
                            &preview.buffer,
                        ))),
                        egui::TextureOptions::default(),
                    ));
                }
                ui.colored_label(status.color, &status.text);

                let response = ui
                    .add(egui::Image::new(preview.texture_handle.as_ref().unwrap()))
                    .interact(egui::Sense::click());
                if let Some(pos) = response.interact_pointer_pos() {
                    let rect = response.rect;
                    let x = pos.x - rect.min.x;
                    let y = pos.y - rect.min.y;

                    selected_object = get_hit_object(
                        &scene,
                        &camera.get_ray(
                            x / preview.canvas.width as f32,
                            (preview.canvas.height as f32 - y) / preview.canvas.height as f32,
                        ),
                    );
                }
            });
        },
    )
}
