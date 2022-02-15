use raylib::prelude::*;

pub struct Cover {
    pub img: Texture2D,
    pub label: String,
    pub origin: Vector2,
    pub pos: Vector2,
    pub size: Vector2,
    pub rotation: f32
}

pub struct CoverBook{
    pub covers: Vec<Cover>,
}

impl CoverBook{
    pub fn new() -> CoverBook {
        CoverBook {
            covers: Vec::new()
        }
    }

    pub fn insert_cover(&mut self, cover: Cover){
        self.covers.push(cover);
    }
}

impl Cover {
    pub fn new(image: Texture2D, text_label: String, origin: Vector2, pos: Vector2, size: Vector2) -> Cover {
        Cover{
            img: image,
            label: text_label,
            origin: origin,
            pos: pos,
            size: size,
            rotation: 0.0
        }
    }
    
    pub fn draw(&self, d: &mut RaylibDrawHandle, screen_size: Vector2){
        let img_rec = Rectangle::new(0.0, 0.0, self.img.width as f32, self.img.height as f32);
        if img_rec.width == img_rec.height {
            d.draw_texture_pro(
                &self.img, 
                img_rec, 
                Rectangle::new(
                    self.pos.x + screen_size.x / 2.0, 
                    self.pos.y, 
                    self.size.x - screen_size.x / 2.0, 
                    self.size.y
                ), 
                self.origin, 
                self.rotation + 32.0, 
                Color::WHITE
            )
        }
        d.draw_texture_pro(
            &self.img, 
            img_rec, 
            Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y), 
            self.origin, 
            self.rotation, 
            Color::WHITE
        )
    }

    pub fn draw_lines(&self, d: &mut RaylibDrawHandle){
        d.draw_rectangle_lines_ex(
            Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y), 
            3, 
            Color::RED
        )
    }
}