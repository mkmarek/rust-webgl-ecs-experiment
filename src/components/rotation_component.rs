use crate::base::Component;
use crate::application::*;
use std::any::Any;
extern crate nalgebra as na;
use na::{Quaternion};

pub struct RotationComponent {
    pub rotation: Quaternion<f32>
}

impl Component for RotationComponent {
    fn get_name(&self) -> String { return String::from("position"); }
    fn is(&self, name: &str) -> bool { return name == "position"; }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RotationComponent {
    pub fn new() -> RotationComponent {
        return RotationComponent {
            rotation: Quaternion::identity()
        }
    }
}
