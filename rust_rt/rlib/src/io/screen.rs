extern crate alloc;

use alloc::vec::Vec;

use crate::arch::{syscall_ss_v, syscall_v_ss, SCREEN_WIDTH_HEIGHT, SEND_MAIN_SCREEN_DRAW_CALL};

#[derive(Default, Clone, Copy)]
pub struct ScreeenPoint {
    pub x: i16,
    pub y: i16,
}

pub type ScreenPos = ScreeenPoint;
pub type ScreenVec = ScreeenPoint;

impl<T: Into<i16> + Copy> From<[T; 2]> for ScreeenPoint {
    fn from(v: [T; 2]) -> Self {
        ScreeenPoint {
            x: v[0].into(),
            y: v[1].into(),
        }
    }
}

impl<X: Into<i16>, Y: Into<i16>> From<(X, Y)> for ScreeenPoint {
    fn from((x, y): (X, Y)) -> Self {
        ScreeenPoint {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<ScreeenPoint> for (i16, i16) {
    fn from(v: ScreeenPoint) -> Self {
        (v.x, v.y)
    }
}

impl From<ScreeenPoint> for [i16; 2] {
    fn from(v: ScreeenPoint) -> Self {
        [v.x, v.y]
    }
}

pub enum ScreenCommand<'a> {
    SetColor([u8; 4]),
    Pixel(ScreenPos),
    Line(ScreenPos, ScreenPos),
    Clear,
    Text(&'a str, ScreenPos),
    Rect(ScreenPos, ScreenVec),
    FilledRect(ScreenPos, ScreenVec),
    Polygon(&'a [ScreenPos]),
    FilledPolygon(&'a [ScreenPos]),
    PolygonLine(&'a [ScreenPos]),
    Oval(ScreenPos, ScreenVec),
}

pub struct Screen {
    width: i16,
    height: i16,
    call_data: Vec<u8>,
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen {
    pub fn new() -> Self {
        let (width, height) = get_screen_width_height();
        Self {
            width,
            height,
            call_data: Vec::new(),
        }
    }
}

impl Screen {
    pub fn push_command(&mut self, command: ScreenCommand) {
        match command {
            ScreenCommand::Pixel(p) => {
                self.call_data.push(1);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
            }

            ScreenCommand::Line(p1, p2) => {
                self.call_data.push(2);
                self.push_data(&p1.x.to_be_bytes());
                self.push_data(&p1.y.to_be_bytes());
                self.push_data(&p2.x.to_be_bytes());
                self.push_data(&p2.y.to_be_bytes());
            }

            ScreenCommand::Clear => {
                self.call_data.push(3);
            }

            ScreenCommand::Text(text, p) => {
                self.call_data.push(4);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&(text.as_bytes().len() as i16).to_be_bytes());
                self.push_data(text.as_bytes());
            }

            ScreenCommand::Rect(p, v) => {
                self.call_data.push(5);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());
            }

            ScreenCommand::FilledRect(p, v) => {
                self.call_data.push(6);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());
            }

            ScreenCommand::SetColor(color) => {
                self.call_data.push(7);
                self.push_data(&color);
            }

            ScreenCommand::Polygon(points) => {
                self.call_data.push(8);
                self.push_data(&(points.len() as i16).to_be_bytes());
                for p in points {
                    self.push_data(&p.x.to_be_bytes());
                    self.push_data(&p.y.to_be_bytes());
                }
            }

            ScreenCommand::FilledPolygon(points) => {
                self.call_data.push(9);
                self.push_data(&(points.len() as i16).to_be_bytes());
                for p in points {
                    self.push_data(&p.x.to_be_bytes());
                    self.push_data(&p.y.to_be_bytes());
                }
            }

            ScreenCommand::PolygonLine(points) => {
                self.call_data.push(10);
                self.push_data(&(points.len() as i16).to_be_bytes());
                for p in points {
                    self.push_data(&p.x.to_be_bytes());
                    self.push_data(&p.y.to_be_bytes());
                }
            }

            ScreenCommand::Oval(p, v) => {
                self.call_data.push(11);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());
            }
        }
    }

    pub fn push_command_with_wrap(&mut self, command: ScreenCommand) {
        match command {
            ScreenCommand::Clear
            | ScreenCommand::SetColor(..)
            | ScreenCommand::Pixel(..)
            | ScreenCommand::Text(..) => self.push_command(command),
            ScreenCommand::Line(p1, p2) => {
                self.push_command(command);

                let mut x_off = 0;
                let mut y_off = 0;

                if p1.x < 0 || p2.x < 0 {
                    x_off = self.width
                }
                if p1.x >= self.width || p2.x >= self.width {
                    x_off = -self.width
                }
                if p1.y < 0 || p2.y < 0 {
                    y_off = self.height
                }
                if p1.y >= self.height || p2.y >= self.height {
                    y_off = -self.height
                }

                if x_off != 0 {
                    self.push_command(ScreenCommand::Line(
                        (p1.x + x_off, p1.y).into(),
                        (p2.x + x_off, p2.y).into(),
                    ));
                }
                if y_off != 0 {
                    self.push_command(ScreenCommand::Line(
                        (p1.x, p1.y + y_off).into(),
                        (p2.x, p2.y + y_off).into(),
                    ));
                }

                if x_off != 0 && y_off != 0 {
                    self.push_command(ScreenCommand::Line(
                        (p1.x + x_off, p1.y + y_off).into(),
                        (p2.x + x_off, p2.y + y_off).into(),
                    ));
                }
            }
            ScreenCommand::Rect(p, v) => {
                self.call_data.push(5);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());
            }

            ScreenCommand::FilledRect(p, v) => {
                self.call_data.push(6);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());
            }

            ScreenCommand::Polygon(points) => {
                self.call_data.push(8);
                self.wrapped_points(points);
            }
            ScreenCommand::FilledPolygon(points) => {
                self.call_data.push(9);
                self.wrapped_points(points);
            }
            ScreenCommand::PolygonLine(points) => {
                self.call_data.push(10);
                self.wrapped_points(points);
            }
            ScreenCommand::Oval(p, v) => {
                self.call_data.push(11);
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
                self.push_data(&v.x.to_be_bytes());
                self.push_data(&v.y.to_be_bytes());

                let mut x_off = 0;
                let mut y_off = 0;

                if p.x + v.x >= self.width {
                    x_off = -self.width
                }
                if p.x - v.x < 0 {
                    x_off = self.width
                }
                if p.y + v.y >= self.height {
                    y_off = -self.height
                }
                if p.y - v.y < 0 {
                    y_off = self.height
                }

                if x_off != 0 {
                    self.call_data.push(11);
                    self.push_data(&(p.x + x_off).to_be_bytes());
                    self.push_data(&p.y.to_be_bytes());
                    self.push_data(&v.x.to_be_bytes());
                    self.push_data(&v.y.to_be_bytes());
                }
                if y_off != 0 {
                    self.call_data.push(11);
                    self.push_data(&p.x.to_be_bytes());
                    self.push_data(&(p.y + y_off).to_be_bytes());
                    self.push_data(&v.x.to_be_bytes());
                    self.push_data(&v.y.to_be_bytes());
                }

                if x_off != 0 && y_off != 0 {
                    self.call_data.push(11);
                    self.push_data(&(p.x + x_off).to_be_bytes());
                    self.push_data(&(p.y + y_off).to_be_bytes());
                    self.push_data(&v.x.to_be_bytes());
                    self.push_data(&v.y.to_be_bytes());
                }
            }
        }
    }

    pub fn wrapped_points(&mut self, points: &[ScreenPos]) {
        self.push_data(&(points.len() as i16).to_be_bytes());

        let mut x_off = 0;
        let mut y_off = 0;

        for p in points {
            self.push_data(&p.x.to_be_bytes());
            self.push_data(&p.y.to_be_bytes());
            if p.x < 0 {
                x_off = self.width;
            }
            if p.x >= self.width {
                x_off = -self.width;
            }
            if p.y < 0 {
                y_off = self.height;
            }
            if p.y >= self.height {
                y_off = -self.height;
            }
        }

        if x_off != 0 {
            self.call_data.push(8);
            self.push_data(&(points.len() as i16).to_be_bytes());
            for p in points {
                self.push_data(&(p.x + x_off).to_be_bytes());
                self.push_data(&p.y.to_be_bytes());
            }
        }
        if y_off != 0 {
            self.call_data.push(8);
            self.push_data(&(points.len() as i16).to_be_bytes());
            for p in points {
                self.push_data(&p.x.to_be_bytes());
                self.push_data(&(p.y + y_off).to_be_bytes());
            }
        }

        if x_off != 0 && y_off != 0 {
            self.call_data.push(8);
            self.push_data(&(points.len() as i16).to_be_bytes());
            for p in points {
                self.push_data(&(p.x + x_off).to_be_bytes());
                self.push_data(&(p.y + y_off).to_be_bytes());
            }
        }
    }

    pub fn get_call_len(&self) -> usize {
        self.call_data.len()
    }

    pub fn get_width(&self) -> i16 {
        self.width
    }

    pub fn get_height(&self) -> i16 {
        self.height
    }

    pub fn send_draw_call(&mut self) {
        let ptr = self.call_data.as_ptr().addr() as u32;
        let len = self.call_data.len() as u32;
        unsafe { syscall_ss_v::<SEND_MAIN_SCREEN_DRAW_CALL>(ptr, len) }
        self.call_data.clear()
    }

    fn push_data(&mut self, data: &[u8]) {
        data.iter().for_each(|&d| self.call_data.push(d))
    }
}

pub fn get_screen_width_height() -> (i16, i16) {
    unsafe {
        let (width, height) = syscall_v_ss::<SCREEN_WIDTH_HEIGHT>();
        (width as i16, height as i16)
    }
}
