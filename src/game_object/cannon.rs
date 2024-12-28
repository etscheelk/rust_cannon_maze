use core::f32;
use std::f32::consts::PI;

use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{game_object::HasPosition, util::{message::Message, vec_extension::{Flip, RotateBy}}, MainState};
use super::{has_position, missile::Missile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateDir
{
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cannon
{
    // Unit vector denoting direction
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    pub facing: Vec2,
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    pub position: Vec2,
    rot_vel: f32,
    refire_block: Message
}

has_position!(Cannon);

impl Default for Cannon
{
    /// Spawn the cannon in the center of the screen
    fn default() -> Self {
        let center_pos = [0.0, 0.0].into();
        Self { facing: Vec2::X, position: center_pos, rot_vel: 0.0, refire_block: Default::default() }
    }
}

impl Cannon
{
    const VELOCITY : f32 = 20.0;
    const REFIRE_DELAY: f32 = 1.0; // FIXME: Express in ticks rather than float seconds for greater precision
    const BARREL_LENGTH: f32 = 3.0;
    
    /// constants relateed to rotation of cannon.
    /// Numbers seem to act twice as high as expected
    const MAX_ROT_PER_SEC: f32 = PI;
    const ROT_ACC: f32 = PI / 2.0;
    const ROT_DE_ACC: f32 = Cannon::MAX_ROT_PER_SEC * 2.0; // takes 0.5 second to brake to 0.

    fn new(facing: Vec2, position: Vec2) -> Self
    {
        let mut s = Self::default();
        s.facing = facing;
        s.position = position;

        s
    }
}

impl crate::FixedUpdate<Cannon> for crate::MainState
{
    fn fixed_update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        let ref mut cannon = self.cannon;

        // Handle potential fire-action (if unblocked)
        // check that mouse is clicked & refire block is inactive
        let ref input_state = self.input_state;
        if input_state.shoot
        {
            if let Message::Inactive = cannon.refire_block
            {
                // fire a missile
                let missile_vel = Cannon::VELOCITY * cannon.facing.flip_y();
                let spawn_pos = cannon.position + Cannon::BARREL_LENGTH * cannon.facing.flip_y();

                let m = Missile::new(spawn_pos, missile_vel);
                self.missiles.push(m);

                // set refire block with refire delay
                cannon.refire_block.set_active_ticking((), Cannon::REFIRE_DELAY);
            }
        }

        let mut new_rot_vel: f32 = 0.0;
        match self.input_state.cannon_rotate
        {
            // Either left or right is being held
            Some(d) =>
            {
                // use RotateDir::*;
                use crate::input::ActionCode::TurnLeft;

                // if the desired acceleration is in the opposite direction
                // of movement, pick the maximum of ROT_ACC and ROT_DE_ACC to
                // helpfully do the fastest turn
                let sign = if d == TurnLeft { 1.0 } else { -1.0 };
                let mut acc = Cannon::ROT_ACC;
                if cannon.rot_vel.signum() != sign
                {
                    acc = f32::max(Cannon::ROT_ACC, Cannon::ROT_DE_ACC);
                }
                acc *= sign;

                new_rot_vel = cannon.rot_vel + acc * MainState::FIXED_PHYSICS_TIMESTEP;
                new_rot_vel = new_rot_vel.clamp(-Cannon::MAX_ROT_PER_SEC, Cannon::MAX_ROT_PER_SEC);
            },
            // button not held at this time. 
            // Apply damping force in the opposite direction
            None =>
            {
                if cannon.rot_vel.abs() > 0.0
                {
                    let sign = cannon.rot_vel.signum();
                    new_rot_vel = cannon.rot_vel - sign * Cannon::ROT_DE_ACC * MainState::FIXED_PHYSICS_TIMESTEP;
                    if new_rot_vel.signum() != sign { new_rot_vel = 0.0; }
                }
            },
        };

        cannon.rot_vel = new_rot_vel;
        cannon.facing = cannon.facing.rotate_by(cannon.rot_vel * MainState::FIXED_PHYSICS_TIMESTEP);

        cannon.refire_block.tick(MainState::FIXED_PHYSICS_TIMESTEP);

        Ok(())
    }
}

impl crate::Draw<Cannon> for crate::MainState
{
    fn draw(&self, _context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        use ggez::graphics;
        
        let ref cannon = self.cannon;
        let ref cannon_image = self.assets.cannon_image;

        let cannon_screen_pos = 16.0 * (cannon.position_get() - self.world_pos);

        let transform = 
        graphics::Transform::Values 
        { 
            dest: cannon_screen_pos.into(), 
            rotation: cannon.facing.angle_between(Vec2::X), 
            scale: [2.0, 2.0].into(), 
            offset: [0.0, cannon_image.height() as f32 / 2.0].into(),
        };

        let param = 
            graphics::DrawParam::new()
            .transform(transform.to_bare_matrix());

        canvas.draw(cannon_image, param);

        let center_dot = graphics::Quad;
        let center_param = 
            graphics::DrawParam::new()
            .dest(cannon_screen_pos)
            .color(graphics::Color::MAGENTA)
            .scale([2.0, 2.0]);

        canvas.draw(&center_dot, center_param);

        Ok(())
    }
}