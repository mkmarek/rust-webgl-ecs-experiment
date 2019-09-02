use crate::base::Component;
use crate::application::*;
extern crate nalgebra as na;
use na::{Matrix4};
use std::any::Any;

pub struct CameraComponent {
    pub zNear: i32,
    pub zFar: i32,
    pub projectionMatrix: Matrix4<f32>
}

impl Component for CameraComponent {
    fn get_name(&self) -> String { return String::from("camera"); }
    fn is(&self, name: &str) -> bool { return name == "camera"; }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CameraComponent {
    pub fn new() -> CameraComponent {
        return CameraComponent {
            zNear: 1,
            zFar: 2000,
            projectionMatrix: Matrix4::from_euler_angles(0.0, 0.0, 0.0)
        }
    }

    pub fn from(source: &CameraComponent) -> CameraComponent {
        return CameraComponent {
            zNear: source.zNear,
            zFar: source.zFar,
            projectionMatrix: Matrix4::from(source.projectionMatrix)
        };
    }

    pub fn setProjectionMatrix(&mut self, matrix: Matrix4<f32>) {
        self.projectionMatrix = matrix;
    }

    pub fn main(app: &mut Application) -> Option<&CameraComponent> {
        let entities = app.get_with_components("camera");

        if (entities.len() == 0) {
            return None;
        }

        let component: &CameraComponent = match entities[0].get_component("camera") {
                Some(x) => match x.as_any().downcast_ref::<CameraComponent>().as_mut() {
                    Some(b) => b,
                    None => panic!("&a isn't a B!")
                },
                None => panic!("No camera component!"),
            };

        return Some(component);
    }
}
