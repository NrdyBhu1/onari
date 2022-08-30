use gdnative::api::Node2D;
use gdnative::prelude::{ NativeClass, methods, godot_print, Input };

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct RootScene;

#[methods]
impl RootScene {
    pub fn new(_owner: &Node2D)  -> Self {
        RootScene
    }

    #[export]
    pub unsafe fn _ready(&self, _owner: &Node2D) {
        godot_print!("Loaded Scene!");
    }

    #[export]
    pub unsafe fn _process(&self, owner: &Node2D, _delta: f64) {
        let input = Input::godot_singleton();

        if input.is_action_pressed("game_reload", true) {
            let tree = owner.get_tree().unwrap().assume_safe();
            tree.reload_current_scene().unwrap();
        }
    }
}