use crate::scene::SceneManager;
use crate::ui::Ui;

pub trait DebugLayer {
    fn update(&mut self, scene_manager: &mut dyn SceneManager, ui: &mut Ui, delta_sec: f32);
}
