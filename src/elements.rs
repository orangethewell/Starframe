use raylib::prelude::*;

#[derive(PartialEq)]
pub enum States{
    Idle = 0,
    IdleToActive,
    Active,
    ActiveToDown,
    ActiveToIdle,
    Down,
    DownToActive,
    DownToIdle,
}

#[derive(Copy, Clone)]
pub struct ButtonStyle {
    origin_position: Vector2,
    origin_size: Vector2,

    pub idle_color: Color,
    pub idle_background: Color,
    pub idle_position: Vector2,
    pub idle_size: Vector2,
    
    pub smooth_trasitions: bool,
    pub relative_transforms: bool,
    
    pub hovering_color: Color,
    pub hovering_position: Vector2,
    pub hovering_size: Vector2,
    pub hovering_background: Color,

    pub down_color: Color,
    pub down_position: Vector2,
    pub down_size: Vector2,
    pub down_background: Color,
}

pub struct Button {
    pub position: Vector2,
    pub size: Vector2,
    pub is_active: bool,
    pub is_down: bool,
    pub label: String,
    pub style: ButtonStyle,
    color: Color,
    background: Color,
    state: States,
    fraction: f32,
}

pub fn interpolate_vectors(source: Vector2, target: Vector2, fraction: f32) -> Vector2 {
    if fraction == 1.0 {
        return target;
    }
    if fraction == 0.0 {
        return source;
    }

    if source == target {
        return source;
    }

    let diff = source - target;

    let result = source + diff * fraction;
    result
}

pub fn interpolate_colors(source: Color, target: Color, fraction: f32) -> Color {
    if fraction == 1.0 {
        return target;
    }
    if fraction == 0.0 {
        return source;
    }

    if source == target {
        return source;
    }
    
    let diff = [
        (source.r as f32 - target.r as f32),
        (source.g as f32 - target.g as f32),
        (source.b as f32 - target.b as f32),
        (source.a as f32 - target.a as f32)
    ];

    let new_colors = [
        source.r as f32 + diff[0] * fraction,
        source.g as f32 + diff[1] * fraction,
        source.b as f32 + diff[2] * fraction,
        source.a as f32 + diff[3] * fraction,
    ];

    Color::new(
        new_colors[0] as u8,
        new_colors[1] as u8,
        new_colors[2] as u8,
        new_colors[3] as u8
    )

}

impl ButtonStyle {
    pub fn from_default(
        idle_color: Color, 
        idle_background: Color,
        idle_position: Vector2,
        idle_size: Vector2,
        do_smooth_trasitions: bool,
        do_relative_transforms: bool
    ) -> ButtonStyle {
        ButtonStyle{
            idle_color: idle_color,
            idle_background: idle_background,
            idle_position: idle_position,
            idle_size: idle_size,
            
            smooth_trasitions: do_smooth_trasitions,
            relative_transforms: do_relative_transforms,

            hovering_color: idle_color,
            hovering_position: idle_position,
            hovering_size: idle_size,
            hovering_background: idle_background,
            
            down_color: idle_color,
            down_position: idle_position,
            down_size: idle_size,
            down_background: idle_background,

            origin_position: Vector2::zero(),
            origin_size: Vector2::zero()
        }
    }

    pub fn edit_down_style(
        &mut self,
        down_color: Color, 
        down_background: Color,
        down_position: Vector2,
        down_size: Vector2,
    ) -> &mut ButtonStyle {
        self.down_color = down_color;
        self.down_background = down_background;
        self.down_position = down_position;
        self.down_size = down_size;
        self
    }

    pub fn edit_hover_style(
        &mut self,
        hover_color: Color,
        hover_background: Color,
        hover_position: Vector2,
        hover_size: Vector2
    ) -> &mut ButtonStyle {
        self.hovering_color = hover_color;
        self.hovering_background = hover_background;
        self.hovering_position = hover_position;
        self.hovering_size = hover_size;
        self
    }

    pub fn edit_idle_style(
        &mut self,
        idle_color: Color,
        idle_background: Color,
        idle_position: Vector2,
        idle_size: Vector2
    ) -> &mut ButtonStyle {
        self.idle_color = idle_color;
        self.idle_background = idle_background;
        self.idle_position = idle_position;
        self.idle_size = idle_size;
        self
    }

    pub fn update_relatives(&mut self, parent: &Button){
        self.origin_position = parent.position;
        self.origin_size = parent.size;
    }

    pub fn get_unrelative_size(&self, state: States) -> Vector2 {
        match state {
            States::Idle => {self.origin_size + (self.idle_size)}
            States::Active => {self.origin_size + (self.hovering_size)}
            States::Down => {self.origin_size + (self.down_size)}
            _ => {panic!("Invalid state. Substates not supported!")}
        }
    }

    pub fn get_unrelative_position(&self, state: States) -> Vector2 {
        match state {
            States::Idle => {self.origin_position + (self.idle_position)}
            States::Active => {self.origin_position + (self.hovering_position)}
            States::Down => {self.origin_position + (self.down_position)}
            _ => {panic!("Invalid state. Substates not supported!")}
        }
    }

    pub fn build(&self) -> ButtonStyle {
        ButtonStyle {
            idle_color: self.idle_color,
            idle_background: self.idle_background,
            idle_position: self.idle_position,
            idle_size: self.idle_size,
            
            smooth_trasitions: self.smooth_trasitions,
            relative_transforms: self.relative_transforms,

            hovering_color: self.idle_color,
            hovering_position: self.idle_position,
            hovering_size: self.idle_size,
            hovering_background: self.idle_background,
            
            down_color: self.idle_color,
            down_position: self.idle_position,
            down_size: self.idle_size,
            down_background: self.idle_background,

            origin_position: Vector2::zero(),
            origin_size: Vector2::zero()
        }
    }

    pub fn build_default_style() -> ButtonStyle {
        ButtonStyle{
            idle_color: Color::BLACK, 
            idle_background: Color::LIGHTGRAY, 
            idle_position: Vector2::new(0.0, 0.0),
            idle_size: Vector2::new(0.0, 0.0), 
            
            smooth_trasitions: false,
            relative_transforms: true,

            hovering_color: Color::RED,
            hovering_position: Vector2::new(0.0, 0.0),
            hovering_size: Vector2::new(0.0, 0.0),
            hovering_background: Color::LIGHTGRAY,
            
            down_color: Color::DARKPURPLE,
            down_position: Vector2::new(0.0, 0.0),
            down_size:Vector2::new(0.0, 0.0),
            down_background: Color::LIGHTGRAY,

            origin_position: Vector2::zero(),
            origin_size: Vector2::zero()
        }
    }
}

impl Button {
    pub fn new(position: Vector2, size: Vector2, label: String, button_style: ButtonStyle) -> Button {
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
            fraction: 0.0
        }
    }

    pub fn draw(&mut self, drawing_context: &mut RaylibDrawHandle, dt: f32){
        let center_x = self.position.x + self.size.x / 2.0;
        let center_y = self.position.y + self.size.y / 2.0;
        let label_x = center_x as i32 - measure_text(self.label.as_str(), 20) / 2;
        let label_y = center_y as i32 -  20 / 2;
        
        if self.is_active && self.state == States::Idle && !self.style.smooth_trasitions {
            if self.is_active {
                println!("fraction: {}", self.fraction);
                println!("Color Channel: {}, {}, {}", self.color.r, self.color.g, self.color.b)
            }
            self.state = States::Active;
            self.color = self.style.hovering_color;
            self.background = self.style.hovering_background;
            self.position = self.style.hovering_position;
            self.size = self.style.hovering_size;
        }

        if self.is_active && self.is_down && self.state == States::Active && !self.style.smooth_trasitions {
            self.state = States::Down;
            self.color = self.style.down_color;
            self.background = self.style.down_background;
            self.position = self.style.down_position;
            self.size = self.style.down_size;
        }

        if self.is_active && !self.is_down && self.state == States::Down && !self.style.smooth_trasitions {
            self.state = States::Active;
            self.color = self.style.hovering_color;
            self.background = self.style.hovering_background;
            self.position = self.style.hovering_position;
            self.size = self.style.hovering_size;
        }

        if !self.is_active && !self.is_down && (self.state == States::Down || self.state == States::Active) && !self.style.smooth_trasitions {
            self.state = States::Idle;
            self.color = self.style.idle_color;
            self.background = self.style.idle_background;
            self.position = self.style.idle_position;
            self.size = self.style.idle_size;
        }

        if self.is_active && self.state == States::Idle && self.style.smooth_trasitions {
            self.state = States::IdleToActive;
            self.fraction = 0.0;
        }

        if self.is_down && self.is_active && self.state == States::Active && self.style.smooth_trasitions {
            self.state = States::ActiveToDown;
            self.fraction = 0.0;
        }

        if !self.is_down && self.is_active && self.state == States::Down && self.style.smooth_trasitions {
            self.state = States::DownToActive;
            self.fraction = 0.0;
        }

        if !self.is_active && self.state == States::Active && self.style.smooth_trasitions {
            self.state = States::ActiveToIdle;
            self.fraction = 0.0;
        }

        if !self.is_down && !self.is_active && self.state == States::Down && self.style.smooth_trasitions {
            self.state = States::DownToIdle;
            self.fraction = 0.0;
        }

        if self.state == States::IdleToActive {
            if self.fraction < 1.0 {
                self.fraction = if self.fraction < 1.1 {self.fraction + 0.5 * dt} else {1.0};
                self.color = interpolate_colors(self.color, self.style.hovering_color, self.fraction);
                self.background = interpolate_colors(self.background, self.style.hovering_background, self.fraction);
                self.size = interpolate_vectors(
                    self.size, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_size(States::Active)
                    } else {
                        self.style.hovering_size
                    }, 
                    self.fraction);
                self.position = interpolate_vectors(self.position, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_position(States::Active)
                    } else {
                        self.style.hovering_position
                    }, 
                    self.fraction);
            } else {
                self.state = States::Active;
            }
        }

        if self.state == States::ActiveToDown {
            if self.fraction < 1.0 {
                self.fraction = if self.fraction < 1.1 {self.fraction + 0.5 * dt} else {1.0};
                self.color = interpolate_colors(self.color, self.style.down_color, self.fraction);
                self.background = interpolate_colors(self.background, self.style.down_background, self.fraction);
                self.size = interpolate_vectors(
                    self.size, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_size(States::Down)
                    } else {
                        self.style.down_size
                    }, 
                    self.fraction);
                self.position = interpolate_vectors(
                    self.position, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_position(States::Down)
                    } else {
                        self.style.down_position
                    }, 
                    self.fraction);
            } else {
                self.state = States::Down;
            }
        }

        if self.state == States::DownToActive {
            if self.fraction < 1.0 {
                self.fraction = if self.fraction < 1.1 {self.fraction + 0.5 * dt} else {1.0};
                self.color = interpolate_colors(self.color, self.style.hovering_color, self.fraction);
                self.background = interpolate_colors(self.background, self.style.hovering_background, self.fraction);
                self.size = interpolate_vectors(
                    self.size, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_size(States::Active)
                    } else {
                        self.style.hovering_size
                    }, 
                    self.fraction);
                self.position = interpolate_vectors(
                    self.position, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_position(States::Active)
                    } else {
                        self.style.hovering_position
                    }, 
                    self.fraction);
            } else {
                self.state = States::Active;
            }
        }

        if self.state == States::ActiveToIdle {
            if self.fraction < 1.0 {
                self.fraction = if self.fraction < 1.1 {self.fraction + 0.5 * dt} else {1.0};
                self.color = interpolate_colors(self.color, self.style.idle_color, self.fraction);
                self.background = interpolate_colors(self.background, self.style.idle_background, self.fraction);
                self.size = interpolate_vectors(
                    self.size, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_size(States::Idle)
                    } else {
                        self.style.idle_size
                    }, 
                    self.fraction);
                self.position = interpolate_vectors(
                    self.position, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_position(States::Idle)
                    } else {
                        self.style.idle_position
                    }, 
                    self.fraction);
            } else {
                self.state = States::Idle;
            }
        }

        if self.state == States::DownToIdle {
            if self.fraction < 1.0 {
                self.fraction = if self.fraction < 1.1 {self.fraction + 0.5 * dt} else {1.0};
                self.color = interpolate_colors(self.color, self.style.idle_color, self.fraction);
                self.background = interpolate_colors(self.background, self.style.idle_background, self.fraction);
                self.size = interpolate_vectors(
                    self.size, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_size(States::Idle)
                    } else {
                        self.style.idle_size
                    }, 
                    self.fraction);
                self.position = interpolate_vectors(
                    self.position, 
                    if self.style.relative_transforms {
                        self.style.get_unrelative_position(States::Idle)
                    } else {
                        self.style.idle_position
                    }, 
                    self.fraction);
            } else {
                self.state = States::Idle;
            }
        }

        drawing_context.draw_rectangle(self.position.x as i32, self.position.y as i32, self.size.x as i32, self.size.y as i32, self.background);
        drawing_context.draw_text(self.label.as_str(), label_x, label_y, 20, self.color);

    }

    pub fn is_hover(&mut self, position: Vector2) -> bool {
        if position.x >= self.position.x && position.x <= self.position.x + self.size.x {
            if position.y >= self.position.y && position.y <= self.position.y + self.size.y {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

}