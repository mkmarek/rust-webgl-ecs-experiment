use crate::base::Shader;

pub fn get(name: String) -> Shader {
    return Shader {
        name: name,
        vertex_shader: String::from("
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }"),
        fragment_shader: String::from("
            void main() {
                gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ")
    };
}