pub mod systems;
pub mod store;
pub mod base;
pub mod application;
pub mod entity;
pub mod components;

use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn create_application() -> Result<application::Application, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let app = application::Application {
        document: document,
        context: context,
        entities: Vec::new(),
        systems: Vec::new()
    };

    return Ok(app);
}

fn bootstrap() -> Result<application::Application, JsValue> {
    let mut app = create_application()?;

    let mut triangle_entity = entity::Entity::new();
    let renderer_component = components::mesh_renderer_component::MeshRendererComponent::new("triangle", "triangle");
    triangle_entity.add_component(renderer_component);

    app.add_entity(triangle_entity);
    app.systems.push(Box::new(systems::render_system::RenderSystem {}));

    return Ok(app);
}

fn draw(app: &application::Application) -> Result<(), JsValue> {
    app.context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    app.context.enable_vertex_attrib_array(0);

    app.context.clear_color(0.0, 0.0, 0.0, 1.0);
    app.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    for system in &app.systems {
        system.deref().execute(app);
    }

    return Ok(());
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let app = bootstrap()?;
    
    draw(&app)?;
    draw(&app)?;
    draw(&app)?;

    Ok(())
}