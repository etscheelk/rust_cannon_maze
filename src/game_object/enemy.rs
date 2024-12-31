use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::game_object::HasRegion;

use super::{has_region, has_position, Region, collider_type::*};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Enemy
{
    health: i32,
    collision_region: Region::<Collider>,
    selection_region: Region::<Selection>,
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    position: Vec2
}

has_position!(Enemy);
has_region!(Enemy, collision_region, Collider);
has_region!(Enemy, selection_region, Selection);


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

        let ref enemies = self.enemies;

        for enemy in enemies
        {
            let img = graphics::Image::from_path(context, "/enemy.png")?;

            let screen_pos = (enemy.position - self.world_pos) * 16.0;

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

            if self.debug_state.draw_hitboxes
            {
                HasRegion::<Collider>::region_get(enemy).draw(enemy, self.world_pos, screen_pos, context, canvas)?;
            }
        }
        
        Ok(())    
    }
}