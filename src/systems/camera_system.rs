use crate::application::*;
use crate::base::*;
use crate::components::camera_component::CameraComponent;

pub struct CameraSystem {
    
}

impl System for CameraSystem {
    fn execute(&self, app: &mut Application) {
        let camera = CameraComponent::main(app).unwrap();
        let aspectRatio = (app.canvas.width() as f32) / (app.canvas.height() as f32);
        let fov = 2.35619449;

        let entities = app.get_with_components("mesh_renderer");
        //let camera = CameraComponent::main(app);

        for entity in entities {
            entity.add_component(CameraComponent::from(camera));
        }

        //web_sys::console::log_1(&format!("Hello using web-sys {0}", aspectRatio).into());

        //camera.setProjectionMatrix(Matrix4::new_perspective(aspectRatio, fov, camera.zNear as f32, camera.zFar as f32));
        
    }
}