use egui::accesskit::Point;
use egui::{Color32, FontId, Stroke};

/// Visual style of the place.
#[derive(Clone)]
pub struct Style {
    // pub label_font: FontId,
    // pub label_color: Color32,
    // pub label_background: Color32,
    // pub symbol_font: FontId,
    // pub symbol_color: Color32,
    // pub symbol_background: Color32,
    // pub symbol_stroke: Stroke,
    pub track: StyleTrack,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            // label_font: FontId::proportional(12.),
            // label_color: Color32::from_gray(200),
            // label_background: Color32::BLACK.gamma_multiply(0.8),
            // symbol_font: FontId::proportional(14.),
            // symbol_color: Color32::BLACK.gamma_multiply(0.8),
            // symbol_background: Color32::WHITE.gamma_multiply(0.8),
            // symbol_stroke: Stroke::new(2., Color32::BLACK.gamma_multiply(0.8)),
            track: StyleTrack {
                point: StylePoint {
                    radius: 10.0,
                    fill_color: Color32::BLACK.gamma_multiply(0.2),
                    stroke: Stroke::new(2., Color32::BLACK.gamma_multiply(0.5)),
                },
                line: StyleLine {
                    stroke: Stroke::new(2., Color32::BLACK.gamma_multiply(0.5)),
                },
                select_point: StylePoint {
                    radius: 10.0,
                    fill_color: Color32::BLACK.gamma_multiply(0.5),
                    stroke: Stroke::new(2., Color32::BLACK.gamma_multiply(0.8)),
                },
                select_line: StyleLine {
                    stroke: Stroke::new(3., Color32::BLACK.gamma_multiply(0.8)),
                },
            },
        }
    }
}

#[derive(Clone)]
pub struct StyleTrack {
    pub point: StylePoint,
    pub line: StyleLine,
    pub select_point: StylePoint,
    pub select_line: StyleLine,
}

impl StyleTrack {
    pub fn select_line(&self, select: bool) -> &StyleLine {
        if select {
            &self.select_line
        } else {
            &self.line
        }
    }

    pub fn select_point(&self, select: bool) -> &StylePoint {
        if select {
            &self.select_point
        } else {
            &self.point
        }
    }
}

#[derive(Clone)]
pub struct StylePoint {
    pub radius: f32,
    pub fill_color: Color32,
    pub stroke: Stroke,
}

#[derive(Clone)]
pub struct StyleLine {
    pub stroke: Stroke,
}
