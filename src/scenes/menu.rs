use crate::elements::{AsScene, Cover, Button, ButtonStyle, SceneCommand};
use raylib::prelude::*;

pub struct MainScreen {
    name: &'static str,

    simple_cover: Cover,
    buttons: [Button; 5],
    start_time: f32,
    end_time: f32,
    deltatime: f32,

    frame_counter: i32,
    state: i32,
}

impl MainScreen {
    /// Create "menu" scene.    
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> MainScreen {
        MainScreen {
            name: "Menu",

            simple_cover: Cover::new(
                rl.load_texture(&thread, "resources/paper_mario_cover.png").unwrap(), 
                String::from("Paper Mario 64"),
                Vector2::new(0.0, 0.0), 
                Vector2::new(0.0, 0.0), 
                Vector2::new(0.0, 0.0)
            ),
            buttons: [
                Button::new(  // Start
                    Vector2::new(10.0, 10.0),
                    Vector2::new(100.0, 60.0),
                    String::from("Start"),
                    ButtonStyle::build_default_style()
                ),
                Button::new(  // Options
                    Vector2::new(250.0, 10.0),
                    Vector2::new(100.0, 60.0),
                    String::from("Options"),
                    ButtonStyle::build_default_style()
                ),
                Button::new(  // Exit
                    Vector2::new(540.0, 10.0),
                    Vector2::new(100.0, 60.0),
                    String::from("Exit"),
                    ButtonStyle::build_default_style()
                ),
            
                Button::new(  // Go Left
                    Vector2::new(0.0, 240.0),
                    Vector2::new(20.0, 80.0),
                    String::from("<"),
                    ButtonStyle::build_default_style()
                ),
                Button::new(  // Go Right
                    Vector2::new(620.0, 240.0),
                    Vector2::new(20.0, 80.0),
                    String::from(">"),
                    ButtonStyle::build_default_style()
                ),
            
            ],
            start_time: 0.0,
            end_time: 0.0,
            deltatime: 0.0,

            frame_counter: 0,
            state: 0
        }
    }
}

impl AsScene for MainScreen {  
    fn name(&self) -> &'static str {
        self.name
    }

    fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {   
        rl.set_target_fps(60);

        self.deltatime = (self.end_time - self.start_time) as f32;
    
        let mut standard_style = ButtonStyle::from_default(
            Color::BLUE, 
            Color::LIGHTGRAY
        );
        
        standard_style.edit_hover_style(
            Color::DARKBLUE, 
            Color::LIGHTGRAY
        );
    
        standard_style.edit_down_style(
            Color::DARKBLUE, 
            Color::GRAY
        );
    
        for button in &mut self.buttons{
            button.style = standard_style;
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneCommand {
        self.start_time = rl.get_time() as f32;
        let screen_size: Vector2 = Vector2::new(
            rl.get_screen_width() as f32, 
            rl.get_screen_height() as f32
        );

        let cursor = rl.get_mouse_position();

        for mut button in &mut self.buttons {
            if button.is_hover(cursor) {
                button.is_active = true;
            } else {
                button.is_active = false;
            };
        }

        self.buttons[1].position.x = (screen_size.x / 2.0) - self.buttons[1].size.x / 2.0; // Options display
        
        self.buttons[2].position.x = (screen_size.x - 10.0) - self.buttons[2].size.x; // Exit display
        
        let screen_split = screen_size.x / 3.0;
        self.buttons[0].size.x = screen_split - 10.0; // Start button division
        self.buttons[1].size.x = screen_split - 10.0; // Options button division
        self.buttons[2].size.x = screen_split - 10.0; // Exit button division

        self.buttons[4].position.x = screen_size.x - 20.0; // Go Right display
        self.buttons[4].position.y = screen_size.y / 2.0;

        self.buttons[3].position.y = screen_size.y / 2.0; // Go Left display
        
        self.simple_cover.pos = Vector2::new(20.0, 80.0);
        self.simple_cover.size = Vector2::new(screen_size.x - 80.0, screen_size.y - 80.0);
        
        for button in &mut self.buttons {
            if button.is_hover(cursor) && rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                button.is_down = true;
            } else {
                button.is_down = false;
            }
        }

        {
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            d.clear_background(Color::RAYWHITE);

            self.simple_cover.draw(&mut d, screen_size);
            self.simple_cover.draw_lines(&mut d);

            d.draw_line_ex( 
                // This line marks the upside of buttons and the downside of covers,
                // it means that cover area is screen's size minus 80 pixels.
                Vector2::new(0.0, 80.0),
                Vector2::new(screen_size.x, 80.0), 
                3.0, 
                Color::BLACK
            );

            for button in &mut self.buttons {
                button.draw(&mut d, self.deltatime)
            }

            
        }
        self.end_time = rl.get_time() as f32;
        self.deltatime = self.end_time - self.start_time;
        SceneCommand::continue_program()
    }
    
    fn unload(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){}
}