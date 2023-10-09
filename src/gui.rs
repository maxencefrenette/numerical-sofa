use eframe::egui::{self, Pos2};
use geo::{AffineOps, AffineTransform, Coord, Polygon, Translate};

pub struct Gui {
    pub sofa: Polygon<f64>,
    pub hallway: Polygon<f64>,
    pub sofa_start: Coord,
}

impl Gui {
    pub fn run(self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1280.0, 720.0)),
            ..Default::default()
        };

        eframe::run_native(
            "Numerical Sofa Results Viewer",
            options,
            Box::new(|_| Box::new(self)),
        )
    }
}

impl eframe::App for Gui {
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
