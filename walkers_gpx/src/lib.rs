mod style;

use crate::style::Style;
use egui::{Color32, Pos2, Vec2};
use egui::{Painter, Response};
use gpx::errors::GpxError;
use gpx::Gpx;
use std::io::Read;
use walkers::{Plugin, Position, Projector};

pub struct WalkerGpx {
    gpx: Gpx,
    style: Style,
    select: Option<GpxIndex>,
}

impl WalkerGpx {
    pub fn read<R: Read>(reader: R) -> Result<WalkerGpx, GpxError> {
        Ok(WalkerGpx {
            gpx: gpx::read(reader)?,
            style: Default::default(),
            select: Default::default(),
        })
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
struct GpxIndex {
    track: usize,
    segment: usize,
    waypoint: usize,
}

impl Plugin for &mut WalkerGpx {
    fn run(&mut self, response: &Response, painter: Painter, projector: &Projector) {
        let mut clicked_at_screen = None;
        if !response.changed() && response.clicked_by(egui::PointerButton::Primary) {
            clicked_at_screen = response.interact_pointer_pos();

            if let (Some(s), Some(p)) = (self.select, clicked_at_screen) {
                let p = projector.unproject(p - response.rect.center());

                *self.gpx.tracks[s.track].segments[s.segment].points[s.waypoint].point_mut() =
                    geo_types::Point::new(p.lon(), p.lat());

                self.select = None;
                clicked_at_screen = None;
            }
        }

        let at_screen = painter.ctx().pointer_latest_pos();

        let mut current_index = GpxIndex::default();

        for (i, track) in self.gpx.tracks.iter().enumerate() {
            current_index.track = i;
            for (i, segment) in track.segments.iter().enumerate() {
                current_index.segment = i;
                let mut prev_screen_position: Option<_> = None::<Pos2>;
                for (i, waypoint) in segment.points.iter().enumerate() {
                    current_index.waypoint = i;

                    let position = self
                        .select
                        .filter(|&a| a == current_index)
                        .and(at_screen)
                        .map(|p| projector.unproject(p - response.rect.center()))
                        .unwrap_or(Position::from_lon_lat(
                            waypoint.point().x(),
                            waypoint.point().y(),
                        ));

                    let screen_position = projector.project(position).to_pos2();

                    let hovered = response
                        .hover_pos()
                        .map(|hover_pos| {
                            hover_pos.distance(screen_position) < self.style.track.point.radius
                        })
                        .unwrap_or(false);

                    let sp = self.style.track.select_point(hovered);
                    painter.circle(screen_position, sp.radius, sp.fill_color, sp.stroke);

                    if let Some(clicked_at_screen) = clicked_at_screen {
                        if clicked_at_screen.distance(screen_position)
                            < self.style.track.point.radius
                        {
                            self.select = Some(current_index);
                            println!(" Select {:?}", self.select)
                        }
                    }

                    if let Some(prev_screen_position) = prev_screen_position {
                        let hover = response
                            .hover_pos()
                            .map(|pos| on_segment(prev_screen_position, screen_position, pos))
                            .unwrap_or(false);

                        painter.line_segment(
                            [prev_screen_position, screen_position],
                            self.style.track.select_line(hover).stroke,
                        );
                    }

                    prev_screen_position = Some(screen_position);
                }
            }
        }
    }
}

/// todo add distance to line
pub fn on_segment(a: Pos2, b: Pos2, c: Pos2) -> bool {
    let min = a.min(b);
    let max = a.max(b);
    if !(c.x < max.x && c.x > min.x && c.y < max.y && c.y > min.y) {
        return false;
    }

    if (a.distance(c) < 0.1) || (b.distance(c) < 0.1) {
        return true;
    }

    let ac = (a.y - c.y) / (a.x - c.x); // calc hoek
    let bc = (b.y - c.y) / (b.x - c.x);

    (ac - bc).abs() < 0.1
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
