use crate::nji::object::ObjectRef;
use crate::sys::*;

pub struct Turtle;
pub type TurtleRef = ObjectRef<Turtle>;

impl TurtleRef {
    pub fn new() -> Self {
        unsafe { Self::from_id_bits(syscall_v_s::<CREATE_TURTLE>()).unwrap() }
    }

    pub fn set_speed(&mut self, speed: i32) {
        unsafe { syscall_ss_v::<SET_TURTLE_SPEED>(self.id_bits(), speed as u32) }
    }

    pub fn pen_down(&mut self) {
        unsafe { syscall_s_v::<TURTLE_PEN_DOWN>(self.id_bits()) }
    }

    pub fn pen_up(&mut self) {
        unsafe { syscall_s_v::<TURTLE_PEN_UP>(self.id_bits()) }
    }

    pub fn forward(&mut self, pixels: f64) {
        unsafe { syscall_ds_v::<TURTLE_FORWARD>(pixels.to_bits(), self.id_bits()) }
    }

    pub fn left(&mut self, pixels: f64) {
        unsafe { syscall_ds_v::<TURTLE_LEFT>(pixels.to_bits(), self.id_bits()) }
    }

    pub fn right(&mut self, pixels: f64) {
        unsafe { syscall_ds_v::<TURTLE_RIGHT>(pixels.to_bits(), self.id_bits()) }
    }
}

impl Default for TurtleRef {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TurtleDisplayer;
pub type TurtleDisplayerRef = ObjectRef<TurtleDisplayer>;

impl TurtleDisplayerRef {
    #[inline(always)]
    pub fn new_with_turtle(turtle: &mut TurtleRef) -> Self {
        unsafe {
            Self::from_id_bits(syscall_s_s::<CREATE_TURTLE_DISPLAY_WITH_TURTLE>(
                turtle.id_bits(),
            ))
            .unwrap()
        }
    }

    pub fn close(self) {
        unsafe { syscall_s_v::<CLOSE_TURTLE_DISPLAYER>(self.id_bits()) }
    }
}
