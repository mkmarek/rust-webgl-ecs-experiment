pub mod systems;
pub mod store;
pub mod base;
pub mod application;
pub mod entity;
pub mod components;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

fn bootstrap(app: &mut application::Application) {
    // camera
    let mut camera_entity = entity::Entity::new();
    camera_entity.add_component(components::camera_component::CameraComponent::new());
    camera_entity.add_component(components::position_component::PositionComponent::new());
    camera_entity.add_component(components::rotation_component::RotationComponent::new());
    app.add_entity(camera_entity);

    // basic triangle entity
    let mut triangle_entity = entity::Entity::new();
    let renderer_component = components::mesh_renderer_component::MeshRendererComponent::new("triangle", "triangle");
    triangle_entity.add_component(renderer_component);
    app.add_entity(triangle_entity);

    //Add systems
    app.systems.push(Box::new(systems::camera_system::CameraSystem {}));
    app.systems.push(Box::new(systems::render_system::RenderSystem {}));
}

fn updateCanvasSize(app: &application::Application) -> Result<(), JsValue> {
    match web_sys::window().unwrap().inner_width().unwrap().as_f64() {
        Some(width) => app.canvas.set_width(width as u32),
        None => app.canvas.set_width(400),
    }

    match web_sys::window().unwrap().inner_height().unwrap().as_f64() {
        Some(height) => app.canvas.set_height(height as u32),
        None => app.canvas.set_height(400),
    }

    return Ok(());
}

fn draw(app: &mut application::Application) -> Result<(), JsValue> {

    app.context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    app.context.enable_vertex_attrib_array(0);

    app.context.clear_color(0.0, 0.0, 0.0, 1.0);
    app.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    app.execute_systems();    

    return Ok(());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
     web_sys::window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

struct RenderLoop {
    animation_id: Option<i32>,
    pub closure: Option<Closure<Fn()>>,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let mut app = application::Application {
        document: document,
        context: context,
        canvas: canvas,
        entities: Vec::new(),
        systems: Vec::new()
    };

    bootstrap(&mut app);
    
    let render_loop: Rc<RefCell<RenderLoop>> = Rc::new(RefCell::new(RenderLoop {
        animation_id: None,
        closure: None
    }));

    let protectedApp =  Arc::new(Mutex::new(app));
    
    {
        let closure: Closure<Fn()> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || {
                let mut render_loop = render_loop.borrow_mut();
                updateCanvasSize(&mut protectedApp.lock().unwrap());
                draw(&mut protectedApp.lock().unwrap());
                render_loop.animation_id = if let Some(ref closure) = render_loop.closure {
                    Some(web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).expect("cannot set animation frame"))
                } else {
                    None
                }
            }))
        };
        let mut render_loop = render_loop.borrow_mut();
        render_loop.animation_id = Some(web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).expect("cannot set animation frame"));
        render_loop.closure = Some(closure);
    }

    Ok(())
}