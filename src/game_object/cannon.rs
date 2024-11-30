use std::f32::consts::PI;

use ggez::glam::Vec2;

use crate::{util::vec_extension::RotateBy, MainState};

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
    pub facing: Vec2,

    position: Vec2,
    rot_vel: f32,
}

impl Default for Cannon
{
    fn default() -> Self {
        Self { facing: Vec2::X, position: Default::default(), rot_vel: 0.0 }
    }
}

impl Cannon
{
    const VELOCITY : f32 = 50.0;
    const MAX_ROT_PER_SEC: f32 = PI / 2.0;
    const ROT_ACC: f32 = 0.5;
    const ROT_DE_ACC: f32 = Cannon::MAX_ROT_PER_SEC; // takes a second to brake to 0.

    fn new(facing: Vec2, position: Vec2) -> Self
    {
        Self
        {
            facing,
            position,
            rot_vel: 0.0,
        }
    }
}

impl crate::FixedUpdate<Cannon> for crate::MainState
{
    fn fixed_update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        let ref mut cannon = self.cannon;

        match self.input_state.cannon_rotate
        {
            // Either left or right is being held
            Some(d) =>
            {
                use RotateDir::*;

                let mut new_rot_vel: f32;
                new_rot_vel = cannon.rot_vel + Cannon::ROT_ACC * MainState::FIXED_PHYSICS_TIMESTEP * if d == Left { 1.0 } else { -1.0 };
                new_rot_vel = new_rot_vel.clamp(-Cannon::MAX_ROT_PER_SEC, Cannon::MAX_ROT_PER_SEC);

                cannon.rot_vel = new_rot_vel;
                cannon.facing = cannon.facing.rotate_by(cannon.rot_vel * MainState::FIXED_PHYSICS_TIMESTEP);
            },
            // button not held at this time. 
            // Apply damping force in the opposite direction
            None =>
            {
                let mut new_rot_vel: f32;
                if cannon.rot_vel > 0.0
                {
                    new_rot_vel = cannon.rot_vel - Cannon::ROT_DE_ACC * MainState::FIXED_PHYSICS_TIMESTEP;
                    new_rot_vel = new_rot_vel.max(0.0);

                }
                else
                {
                    new_rot_vel = cannon.rot_vel + Cannon::ROT_DE_ACC * MainState::FIXED_PHYSICS_TIMESTEP;
                    new_rot_vel = new_rot_vel.min(0.0);
                }

                cannon.rot_vel = new_rot_vel;
                cannon.facing = cannon.facing.rotate_by(cannon.rot_vel * MainState::FIXED_PHYSICS_TIMESTEP);
            },
        };

        Ok(())
    }
}

impl crate::Draw<Cannon> for crate::MainState
{
    fn draw(&self, _context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        use ggez::graphics;
        
        let ref cannon = self.cannon;
        let ref cannon_image = self.assets.cannon_image;
        let param = 
            graphics::DrawParam::new()
            .dest([200.0, 200.0])
            .rotation(cannon.facing.angle_between(Vec2::X))
            .scale([2.0, 2.0]);

        canvas.draw(cannon_image, param);

        Ok(())
    }
}