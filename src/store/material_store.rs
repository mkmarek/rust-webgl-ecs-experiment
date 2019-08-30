use crate::base::Material;

pub fn get(name: &str) -> Material {
    return Material {
        name: String::from("stuff"),
        shader: String::from("otherStuff")
    };
}