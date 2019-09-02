use crate::base::Component;
use crate::application::*;
use std::any::Any;

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for PositionComponent {
    fn get_name(&self) -> String { return String::from("position"); }
    fn is(&self, name: &str) -> bool { return name == "position"; }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PositionComponent {
    pub fn new() -> PositionComponent {
        return PositionComponent {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}
