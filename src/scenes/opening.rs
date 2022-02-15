use crate::elements::{AsScene, SceneCommand};
use raylib::prelude::*;
use raylib::ease;

pub struct Opening {
    name: &'static str,
    // Insert future use variables there.
    title: &'static str,
    frame_counter: i32,
    state: i32,
    alpha: f32,
}

impl Opening {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Opening {
        Opening {
            name: "Opening",

            title: "Starframe",
            frame_counter: 0,
            state: 0,
            alpha: 0.0,
        }
    }
}

impl AsScene for Opening {
    fn name(&self) -> &'static str {
        self.name
    }

    fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){
        rl.set_target_fps(60);
    }
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneCommand {
        let (s_wid, s_heig) = (rl.get_screen_width(), rl.get_screen_height());
        match self.state {
            0 => {
                self.alpha = ease::sine_in(self.frame_counter as f32, 0.0, 2.0, 210.0);
                if self.frame_counter >= 210 {
                    self.state = 1;
                    self.frame_counter = 0;
                }
            }

            1 => {
                if self.frame_counter >= 840 {
                    self.alpha = 1.0;
                    self.state = 2;
                    self.frame_counter = 0;
                }
            }

            2 => {
                self.alpha = ease::sine_out(self.frame_counter as f32, 1.0, -2.0, 210.0);
                if self.frame_counter >= 260 {
                    return SceneCommand::jump_to_scene("Menu")
                }
            }
            _ => {}
        }
        {
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_text(
                self.title, 
                s_wid / 2 - measure_text(self.title, 20) / 2,
                s_heig / 2 - 10,
                20, 
                Color::WHITE.fade(self.alpha)
            );
        }

        self.frame_counter += 1;
        SceneCommand::continue_program()
    }
    fn unload(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){}
}
