use crate::base::Component;
use crate::base::Mesh;
use std::any::Any;

pub struct MeshRendererComponent {
    pub mesh: String,
    pub material: String
}

impl MeshRendererComponent {
    pub fn new(mesh: &str, material: &str) -> MeshRendererComponent {
        return MeshRendererComponent {
            mesh: String::from(mesh),
            material: String::from(material),
        };
    }
}

impl Component for MeshRendererComponent {
    fn get_name(&self) -> String { return String::from("mesh_renderer"); }
    fn is(&self, name: &str) -> bool { return name == "mesh_renderer"; }
    fn as_any(&self) -> &dyn Any {
        self
    }
}