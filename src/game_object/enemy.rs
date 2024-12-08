use derive_setters::Setters;
use ggez::glam::Vec2;
use ggez::graphics::Rect;

#[derive(Default, Debug, Clone, Setters)]
pub struct Enemy
{
    health: i32,
    collision_box: ((f32, f32), (f32, f32)),
    position: Vec2
}

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
        }
        
        Ok(())    
    }
}