use ggez::glam::Vec2;

use crate::util::vec_extension::RotateBy;

// use crate::util::vec_extension::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateDir
{
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct Cannon
{
    // Unit vector denoting direction
    facing: Vec2,

    position: Vec2,
}

impl Default for Cannon
{
    fn default() -> Self {
        Self { facing: Vec2::X, position: Default::default() }
    }
}

impl Cannon
{
    const VELOCITY : f32 = 50.0;

    fn new(facing: Vec2, position: Vec2) -> Self
    {
        Self
        {
            facing,
            position
        }
    }
}

impl crate::FixedUpdate<Cannon> for crate::MainState
{
    fn fixed_update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        let ref mut cannon = self.cannon;

        if let Some(rd) = self.input_state.cannon_rotate
        {
            use RotateDir::*;

            if rd == Left
            {
                // cannon.facing.rotate(0.5);
                cannon.facing = cannon.facing.rotate_by(0.5);
            }
            else
            {
                cannon.facing = cannon.facing.rotate_by(-0.5);
            }
        }
        self.input_state.cannon_rotate = None;


        Ok(())
    }
}

impl crate::Draw<Cannon> for crate::MainState
{
    fn draw(&self, _context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        use ggez::graphics;
        
        let ref cannon = self.cannon;
        let ref cannon_image = self.assets.cannon_image;
        let param = graphics::DrawParam::new().dest([200.0, 200.0]).rotation(cannon.facing.angle_between(Vec2::X));

        canvas.draw(cannon_image, param);

        Ok(())
    }
}