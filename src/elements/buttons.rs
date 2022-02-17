use raylib::prelude::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum States {
    Idle = 0,
    Active,
    Down,
}

#[derive(Copy, Clone)]
pub struct ButtonStyle {
    pub idle_color: Color,
    pub idle_background: Color,
    
    pub hovering_color: Color,
    pub hovering_background: Color,

    pub down_color: Color,
    pub down_background: Color,
}

pub struct Button {
    pub position: Vector2,
    pub size: Vector2,
    pub is_active: bool,
    pub is_down: bool,
    pub label: &'static str,
    pub style: ButtonStyle,
    pub color: Color,
    pub background: Color,
    
    state: States,
    last_state: States,
    fraction: f32,
}

impl ButtonStyle {
    pub fn from_default(
        idle_color: Color, 
        idle_background: Color,
    ) -> ButtonStyle {
        ButtonStyle{
            idle_color: idle_color,
            idle_background: idle_background,

            hovering_color: idle_color,
            hovering_background: idle_background,
            
            down_color: idle_color,
            down_background: idle_background,

        }
    }

    pub fn edit_down_style(
        &mut self,
        down_color: Color, 
        down_background: Color,
    ) -> &mut ButtonStyle {
        self.down_color = down_color;
        self.down_background = down_background;
        self
    }

    pub fn edit_hover_style(
        &mut self,
        hover_color: Color,
        hover_background: Color,
    ) -> &mut ButtonStyle {
        self.hovering_color = hover_color;
        self.hovering_background = hover_background;
        self
    }

    pub fn edit_idle_style(
        &mut self,
        idle_color: Color,
        idle_background: Color,
    ) -> &mut ButtonStyle {
        self.idle_color = idle_color;
        self.idle_background = idle_background;
        self
    }

    pub fn build_default_style() -> ButtonStyle {
        ButtonStyle{
            idle_color: Color::BLACK, 
            idle_background: Color::LIGHTGRAY, 

            hovering_color: Color::RED,
            hovering_background: Color::LIGHTGRAY,
            
            down_color: Color::DARKBLUE,
            down_background: Color::GRAY,

        }
    }
}

impl Button {

    pub fn new(position: Vector2, size: Vector2, label: &'static str, button_style: ButtonStyle) -> Button {
        Button {
            position: position,
            size: size,
            is_active: false,
            is_down: false,
            state: States::Idle,
            label: label,
            style: button_style,
            color: button_style.idle_color,
            background: button_style.idle_background,
            fraction: 0.0,
            last_state: States::Idle
        }
    }

    pub fn draw(&mut self, drawing_context: &mut RaylibDrawHandle, dt: f32){
        let center_x = self.position.x + self.size.x / 2.0;
        let center_y = self.position.y + self.size.y / 2.0;
        let label_x = center_x as i32 - measure_text(self.label, 20) / 2;
        let label_y = center_y as i32 -  20 / 2;


        if self.is_active && self.is_down {
            self.state = States::Down
        }

        else if self.is_active {
            self.state = States::Active
        }

        else {
            self.state = States::Idle
        }

        match self.state {
            States::Active => {
                self.color = self.style.hovering_color;
                self.background = self.style.hovering_background;
            }

            States::Down => {
                self.color = self.style.down_color;
                self.background = self.style.down_background;
            }

            States::Idle => {
                self.color = self.style.idle_color;
                self.background = self.style.idle_background
            }
        };

        drawing_context.draw_rectangle(self.position.x as i32, self.position.y as i32, self.size.x as i32, self.size.y as i32, self.background);
        drawing_context.draw_text(self.label, label_x, label_y, 20, self.color);

    }

    pub fn is_hover(&self, position: Vector2) -> bool {
        if position.x >= self.position.x && position.x <= self.position.x + self.size.x {
            return position.y >= self.position.y && position.y <= self.position.y + self.size.y
        } else {
            false
        }
    }

}