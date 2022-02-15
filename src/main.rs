use raylib::prelude::*;

mod elements;
mod scenes;
use crate::elements::{Button, ButtonStyle, Cover, SceneManager};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("starting...");
    let mut scene_manager = SceneManager::new();
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Starframe")
        .resizable()
        .build();

    // START OF SCENES BUILDING -------------------------

    let mut screen_0 = scenes::Opening::new(&mut rl, &thread);
    scene_manager.push_scene(Box::new(screen_0));

    // END OF SCENE 0 BUILDING --------------------------

    let mut screen_1 = scenes::MainScreen::new(&mut rl, &thread);
    scene_manager.push_scene(Box::new(screen_1));
    
    // END OF SCENE 1 BUILDING --------------------------

    scene_manager.set_current_scene("Opening");
    while !rl.window_should_close() {
        scene_manager.play_scene(&mut rl, &thread);
    }
}
