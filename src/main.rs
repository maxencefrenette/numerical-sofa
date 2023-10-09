#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Pos2};
use geo::{polygon, AffineOps, AffineTransform, Coord, Polygon, Rect, Translate};

fn main() -> Result<(), eframe::Error> {
    let sofa = Rect::new((-1.0, -0.5), (1.0, 0.5)).to_polygon();
    let hallway = polygon![
        (x: 1.0, y: 1.0),
        (x: 1.0, y: -2.0),
        (x: 0.0, y: -2.0),
        (x: 0.0, y: 0.0),
        (x: -2.0, y: 0.0),
        (x: -2.0, y: 1.0),
    ];

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Numerical Sofa Results Viewer",
        options,
        Box::new(|_| {
            Box::new(MyApp {
                sofa,
                hallway,
                sofa_start: (-1.0, 0.5).into(),
            })
        }),
    )
}

struct MyApp {
    sofa: Polygon<f64>,
    hallway: Polygon<f64>,
    sofa_start: Coord,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let scale = 150.0;
            let padding = 0.2 * scale;
            let view_transform = AffineTransform::translate(
                2.0 * scale + padding,
                1.0 * scale + padding,
            )
            .scaled(scale, -scale, (0.0, 0.0));

            let p = ui.painter();

            // Make background white
            p.rect_filled(
                egui::Rect::from_min_size(Pos2::ZERO, ui.available_size()),
                0.0,
                egui::Color32::WHITE,
            );

            // draw hallway
            p.add(egui::Shape::closed_line(
                self.hallway
                    .affine_transform(&view_transform)
                    .exterior()
                    .coords()
                    .map(|c| Pos2::new(c.x as f32, c.y as f32))
                    .collect(),
                egui::Stroke::new(3.0, egui::Color32::BLACK),
            ));

            // draw sofa
            p.add(egui::Shape::closed_line(
                self.sofa
                    .translate(self.sofa_start.x, self.sofa_start.y)
                    .affine_transform(&view_transform)
                    .exterior()
                    .coords()
                    .map(|c| Pos2::new(c.x as f32, c.y as f32))
                    .collect(),
                egui::Stroke::new(2.0, egui::Color32::RED),
            ));
        });
    }
}
