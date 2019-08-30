use crate::base::Mesh;

pub fn get(name: &str) -> Mesh {
    let mesh = Mesh {
        name: String::from(name),
        vertices: [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0].to_vec()
    };

    return mesh;
}