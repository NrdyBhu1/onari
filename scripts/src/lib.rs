use gdnative::prelude::*;
use gdnative::api::KinematicBody2D;
use gdnative::init::*;
use gdnative::core_types::Vector2;

mod scene;

const GRAVITY: f32 = 1000.0;
const MOVEMENT_SPEED: f32 = 100.0;

enum MoveType {
    Left,
    Right,
    Up,
    Down,
    None
}

#[allow(dead_code)]
impl MoveType {

    fn to_vec2(&self) -> Vector2 {
        match *self {
            MoveType::Left => Vector2::new(-1.0*MOVEMENT_SPEED, 0.0*MOVEMENT_SPEED),
            MoveType::Right => Vector2::new(1.0*MOVEMENT_SPEED, 0.0*MOVEMENT_SPEED),
            MoveType::Up => Vector2::new(0.0*MOVEMENT_SPEED, -1.0*MOVEMENT_SPEED),
            MoveType::Down => Vector2::new(0.0*MOVEMENT_SPEED, 1.0*MOVEMENT_SPEED),
            MoveType::None => Vector2::ZERO
        }
    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player {
    move_type: MoveType,
    velocity: Vector2
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player { 
            move_type: MoveType::None, 
            velocity: Vector2::ZERO
        }
    }

    #[export]
    unsafe fn _ready(&self, owner: &KinematicBody2D) {
        owner.set_physics_process(true);
        //gdnative::godot_print!("Hello, World!");
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        // input
        let input = Input::godot_singleton();
        if input.is_action_just_pressed("ui_left", false) {
            self.move_type = MoveType::Left;
        } else if input.is_action_just_pressed("ui_right", false) {
            self.move_type = MoveType::Right;
        } else if input.is_action_just_pressed("ui_down", false) {
            self.move_type = MoveType::Down;
        } else if input.is_action_just_pressed("ui_up", false) {
            self.move_type = MoveType::Up;
        } 
        
        if input.is_action_just_released("ui_left", false) || 
        input.is_action_just_released("ui_right", false) ||
        input.is_action_just_released("ui_up", false) ||
        input.is_action_just_released("ui_down", false)
        {
            self.move_type = MoveType::None;
        }

        self.velocity += self.move_type.to_vec2();
        self.velocity.y += GRAVITY * delta as f32;
        self.velocity = owner.move_and_slide(self.velocity, Vector2::UP, true, 4, 0.785398, true);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<scene::RootScene>();
    handle.add_class::<Player>();
}

// godot_init!(init);
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();