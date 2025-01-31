use crate::directors::sce_vm::{SceCommand, SceState};
use crate::directors::SceneManagerExtensions;
use imgui::Ui;
use radiance::{
    math::Vec3,
    scene::{Scene, SceneManager},
};

#[derive(Debug, Clone)]
pub struct SceCommandCameraDefault {}

impl SceCommand for SceCommandCameraDefault {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        let target = Vec3::new(0., 0., 0.);
        scene_manager
            .core_scene_mut_or_fail()
            .camera_mut()
            .transform_mut()
            .set_position(&Vec3::new(300., 200., 300.))
            .look_at(&target);
        return true;
    }
}

impl SceCommandCameraDefault {
    pub fn new(unknown: i32) -> Self {
        Self {}
    }
}
