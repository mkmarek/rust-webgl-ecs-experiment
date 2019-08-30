use crate::application::*;
use crate::base::*;
use crate::components::mesh_renderer_component::MeshRendererComponent;
use std::any::Any;
use crate::store::shader_store;
use crate::store::material_store;
use crate::store::mesh_store;
use web_sys::{WebGlRenderingContext};

pub struct RenderSystem {
    
}

impl System for RenderSystem {
    fn execute(&self, app: &Application) {
        let entities = app.get_with_components("mesh_renderer");

        for entity in &entities {
            let component: &MeshRendererComponent = match entity.get_component("mesh_renderer") {
                Some(x) => match x.as_any().downcast_ref::<MeshRendererComponent>() {
                    Some(b) => b,
                    None => panic!("&a isn't a B!")
                },
                None => panic!("No mesh_renderer component!"),
            };

            let material = material_store::get(&component.material);
            let shader = shader_store::get(material.shader);
            let mesh = mesh_store::get(&component.mesh);

            Some(shader.use_shader(&app.context));

            unsafe {
                let vert_array = js_sys::Float32Array::view(&mesh.vertices);

                &app.context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &vert_array,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }

            &app.context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
            &app.context.enable_vertex_attrib_array(0);

            &app.context.draw_arrays(
                WebGlRenderingContext::TRIANGLES,
                0,
                (mesh.vertices.len() / 3) as i32,
            );
        }
    }
}