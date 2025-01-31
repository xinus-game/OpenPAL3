use crate::directors::sce_vm::{SceCommand, SceState};

use crate::directors::SceneManagerExtensions;
use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Debug, Clone)]
pub struct SceCommandRoleActAutoStand {
    role_id: i32,
    auto_play_idle: i32,
}

impl SceCommand for SceCommandRoleActAutoStand {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        scene_manager.resolve_role_mut_do(state, self.role_id, |e, r| {
            r.get().set_auto_play_idle(self.auto_play_idle == 1);
        });

        true
    }
}

impl SceCommandRoleActAutoStand {
    pub fn new(role_id: i32, auto_play_idle: i32) -> Self {
        Self {
            role_id,
            auto_play_idle,
        }
    }
}
