use ggez::glam::Vec2;
use ggez::graphics;
use serde::{Deserialize, Serialize};

use crate::util::hash_map_tracker::{ForTracker, HashMapTracker, WithIndex};
use crate::MainState;

use super::{Draw, FixedUpdate};

#[derive(Debug, Clone, Default)]
pub struct EnemyWall
{
    center_position: Vec2,
    rect: graphics::Rect,
    transform: graphics::Transform,
    index: u16
}

impl EnemyWall
{
    pub fn center_position(mut self, pos: Vec2) -> Self
    {
        self.center_position = pos;
        self
    }

    pub fn rect(mut self, rect: graphics::Rect) -> Self
    {
        self.rect = rect;
        self
    }

    pub fn transform(mut self, transform: graphics::Transform) -> Self
    {
        self.transform = transform;
        self
    }
}

impl ForTracker for EnemyWall {}
impl WithIndex for EnemyWall
{
    fn with_index(mut self, index: u16) -> Self 
    {
        self.index = index;
        self
    }
}

// impl FixedUpdate<HashMapTracker<EnemyWall>> for MainState
// {
//     fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
//         Ok(())
//     }
// }

impl Draw<HashMapTracker<EnemyWall>> for MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        // use ggez::graphics;
        
        for (ind, wall) in self.enemy_walls.get_tracker()
        {
            let params = 
                graphics::DrawParam::new()
                .transform(wall.transform.to_bare_matrix())
                .color(graphics::Color::BLUE);

            canvas.draw(&graphics::Quad, params);
        }


        Ok(())
    }
}