//! A Piston library for handling scrolling areas.

#![deny(missing_docs)]

extern crate input;

use input::GenericEvent;

/// Stores scroll settings.
#[derive(Clone, Debug)]
pub struct ScrollSettings {
    speed: f64,
}

impl ScrollSettings {
    /// Creates a new `ScrollSettings` object.
    pub fn new() -> ScrollSettings {
        ScrollSettings {
            speed: 1.0,
        }
    }

    /// Sets scroll speed.
    pub fn speed(mut self, val: f64) -> ScrollSettings {
        self.speed = val;
        self
    }
}

/// Stores information for scrolling.
#[derive(Clone, Debug)]
pub struct ScrollController {
    cursor: [f64; 2],
    /// The offset of visible area.
    pub offset: [f64; 2],
    /// Visible bounds.
    pub bounds: [f64; 4],
    /// The size of the scrollable area.
    pub area: [f64; 2],
    /// The scroll speed.
    pub speed: f64,
}

impl ScrollController {
    /// Creates a new `ScrollController`.
    pub fn new(bounds: [f64; 4], area: [f64; 2], settings: &ScrollSettings) -> ScrollController {
        ScrollController {
            cursor: [0.0; 2],
            offset: [0.0; 2],
            bounds: bounds,
            area: area,
            speed: settings.speed,
        }
    }

    /// Returns the visible rectangle that intersects with the area.
    /// This is used to find what part of the area the user is looking at.
    pub fn visible_area_rect(&self) -> [f64; 4] {
        [
            -self.offset[0],
            -self.offset[1],
            self.bounds[2].min(self.area[0]),
            self.bounds[3].min(self.area[1])
        ]
    }

    /// Transform a rectangle from area coordinates to view.
    pub fn rect_from_area_to_view(&self, rect: [f64; 4]) -> [f64; 4] {
        [
            self.bounds[0] - self.offset[0] - rect[0],
            self.bounds[1] - self.offset[1] - rect[1],
            rect[2],
            rect[3]
        ]
    }

    fn cursor_inside(&self) -> bool {
        let c = self.cursor;
        let b = self.bounds;
        c[0] >= b[0] && c[1] >= b[1] && c[0] < b[0] + b[2] && c[1] < b[1] + b[3]
    }

    fn clamp_offset_to_scroll_area(&self, offset: [f64; 2]) -> [f64; 2] {
        let b = self.bounds;
        let a = self.area;
        [
            if offset[0] > 0.0 || b[2] > a[0] {
                0.0
            } else if offset[0] < -(a[0] - b[2]) {
                -(a[0] - b[2])
            } else {
                offset[0]
            },
            if offset[1] > 0.0 || b[3] > a[1] {
                0.0
            } else if a[1] > b[3] && offset[1] < -(a[1] - b[3]) {
                -(a[1] - b[3])
            } else {
                offset[1]
            }
        ]
    }

    /// Handles event.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor = pos;
        }
        if let Some(pos) = e.mouse_scroll_args() {
            if self.cursor_inside() {
                let new_offset = [
                    self.offset[0] - pos[0] * self.speed,
                    self.offset[1] + pos[1] * self.speed
                ];
                self.offset = self.clamp_offset_to_scroll_area(new_offset);
            }
        }
    }
}
