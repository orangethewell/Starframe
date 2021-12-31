use raylib::prelude::*;

mod elements;

fn main() {
    println!("starting...");
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Starframe")
        .resizable()
        .build();

    let mut buttons = [
        elements::Button::new(  // Start
            Vector2::new(10.0, 10.0),
            Vector2::new(100.0, 60.0),
            String::from("Start"),
            elements::ButtonStyle::build_default_style()
        ),
        elements::Button::new(  // Options
            Vector2::new(250.0, 10.0),
            Vector2::new(100.0, 60.0),
            String::from("Options"),
            elements::ButtonStyle::build_default_style()
        ),
        elements::Button::new(  // Exit
            Vector2::new(540.0, 10.0),
            Vector2::new(100.0, 60.0),
            String::from("Exit"),
            elements::ButtonStyle::build_default_style()
        ),
    
        elements::Button::new(  // Go Left
            Vector2::new(0.0, 240.0),
            Vector2::new(20.0, 80.0),
            String::from("<"),
            elements::ButtonStyle::build_default_style()
        ),
        elements::Button::new(  // Go Right
            Vector2::new(620.0, 240.0),
            Vector2::new(20.0, 80.0),
            String::from(">"),
            elements::ButtonStyle::build_default_style()
        ),
    
    ];

    
    rl.set_target_fps(60);

    let mut start_time = 0.0;
    let mut end_time = 0.0;
    let mut deltatime = 0.0;

    let mut standard_style = elements::ButtonStyle::from_default(
        Color::GOLD, 
        Color::LIGHTGRAY, 
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 0.0), 
        false,
        true
    ).edit_hover_style(
        Color::RED, 
        Color::WHITE, 
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 0.0)
    ).build();

    for button in &mut buttons{
        button.style = standard_style;
    }
    while !rl.window_should_close() {
        start_time = rl.get_time();
        let screen_size: Vector2 = Vector2::new(
            rl.get_screen_width() as f32, 
            rl.get_screen_height() as f32
        );

        let cursor = rl.get_mouse_position();

        for mut button in &mut buttons {
            if button.is_hover(cursor) {
                button.is_active = true;
                println!("button label '{}': is active!", button.label)
            } else {
                button.is_active = false;
            };
        }

        buttons[1].position.x = (screen_size.x / 2.0) - buttons[1].size.x / 2.0; // Options display
        
        buttons[2].position.x = (screen_size.x - 10.0) - buttons[2].size.x; // Exit display
        
        let screen_split = screen_size.x / 3.0;
        buttons[0].size.x = screen_split - 10.0; // Start button division
        buttons[1].size.x = screen_split - 10.0; // Options button division
        buttons[2].size.x = screen_split - 10.0; // Exit button division

        buttons[4].position.x = screen_size.x - 20.0; // Go Right display
        buttons[4].position.y = screen_size.y / 2.0;

        buttons[3].position.y = screen_size.y / 2.0; // Go Left display

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) && buttons[2].is_hover(cursor) {
            buttons[2].is_down = true;
            break;
        } else {
            buttons[2].is_down = false;
        }
        {
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            d.clear_background(Color::RAYWHITE);

            d.draw_line_ex(
                Vector2::new(0.0, 80.0),
                Vector2::new(screen_size.x, 80.0), 
                3.0, 
                Color::BLACK
            );

            for button in &mut buttons {
                let mut style_updated = button.style;
                style_updated.update_relatives(&button);
                button.style = style_updated;
                button.draw(&mut d, deltatime)
            }
        }
        end_time = rl.get_time();
        deltatime = (end_time - start_time) as f32;
    }
}
