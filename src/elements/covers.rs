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
            let mut dest_size = screen_size.x;
            if screen_size.y < dest_size {
                dest_size = screen_size.y
            }

            dest_size -= 80.0;
            let dest_rectangle = Rectangle::new(
                screen_size.x / 2.0, 
                screen_size.y / 2.0 + 40.0, 
                dest_size, 
                dest_size
            );
            let origin = Vector2::new(dest_rectangle.width / 2.0, dest_rectangle.height / 2.0);
            d.draw_texture_pro(
                &self.img, 
                img_rec, 
                dest_rectangle, 
                origin, 
                self.rotation, 
                Color::WHITE
            )
        } else {
            let mut width_scale = screen_size.x / img_rec.width;
            let mut height_scale: f32;
            if img_rec.height * width_scale > screen_size.y {
                height_scale = screen_size.y / img_rec.height;
            } else {
                height_scale = width_scale;
            }

            width_scale -= 0.016;
            height_scale -= 0.05;

            let dest_rectangle = Rectangle::new(
                screen_size.x / 2.0, 
                screen_size.y / 2.0 + 40.0, 
                width_scale * img_rec.width,
                height_scale * img_rec.height
            );
            let origin = Vector2::new(dest_rectangle.width / 2.0, dest_rectangle.height / 2.0);
            d.draw_texture_pro(
                &self.img, 
                img_rec, 
                dest_rectangle, 
                origin, 
                self.rotation, 
                Color::WHITE
            )
        }
    }

    pub fn draw_lines(&self, d: &mut RaylibDrawHandle){
        d.draw_rectangle_lines_ex(
            Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y), 
            3, 
            Color::RED
        )
    }
}