use ggez::glam::Vec2;

use crate::game_object::HasCollisionBox;

use super::{has_collision_box, has_position, CollisionBox};

#[derive(Default, Debug, Clone)]
pub struct Enemy
{
    health: i32,
    collision_box: CollisionBox,
    position: Vec2
}

has_position!(Enemy);
has_collision_box!(Enemy);

impl crate::FixedUpdate<Vec<Enemy>> for crate::MainState
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        
        
        Ok(())
    }
}



impl crate::Draw<Vec<Enemy>> for crate::MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
    {
        use ggez::graphics;

        for enemy in &self.enemies
        {
            let img = graphics::Image::from_path(context, "/enemy.png")?;

            let screen_pos = enemy.position * 16.0;

            let transform = 
                graphics::Transform::Values 
                { 
                    dest: screen_pos.into(), 
                    rotation: 0.0, 
                    scale: [1.0, 1.0].into(), 
                    offset: [img.width() as f32 / 2.0, img.height() as f32 /2.0].into() 
                };

                
            let params = graphics::DrawParam::new().transform(transform.to_bare_matrix());
            
            canvas.draw(&img, params);

            // draw collision box (as a debug option)
            let cb = enemy.collision_box_get();
            
            

            let a: Vec2 = cb.p0;
            let b: Vec2 = (cb.p1.x, cb.p0.y).into();
            let c: Vec2 = cb.p1;
            let d: Vec2 = (cb.p0.x, cb.p1.y).into();

            let points = [a * 16.0, b * 16.0, c * 16.0, d * 16.0];

            let mouse_pos: Vec2 = context.mouse.position().into();
            let mouse_world_pos = mouse_pos / 16.0 + self.world_pos;

            let mut color = graphics::Color::MAGENTA;
            if enemy.intersects_collision_box(mouse_world_pos)
            {
                color = graphics::Color::BLACK;
            }

            
            let mesh = 
                graphics::Mesh::new_polygon(
                    context, 
                    graphics::DrawMode::stroke(2.0), 
                    &points, 
                    color
                )?;
            
            let cb_transform = 
                graphics::Transform::Values 
                { 
                    dest: screen_pos.into(), 
                    rotation: 0.0, 
                    scale: [1.0, 1.0].into(), 
                    offset: [0.0, 0.0].into() 
                };

            canvas.draw(&mesh, graphics::DrawParam::new().transform(cb_transform.to_bare_matrix()));

            // println!("{:?}", points.clone());

            // let b = graphics::Mesh::new_polygon(context, graphics::DrawMode::stroke(2.0), &points, graphics::Color::MAGENTA)?;

            // canvas.draw(&b, graphics::DrawParam::new().dest(screen_pos));

            // canvas.draw(&a, graphics::DrawParam::new().dest(screen_pos));
        }
        
        Ok(())    
    }
}