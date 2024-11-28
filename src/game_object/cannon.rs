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
    const MAX_ROT_PER_SEC: f32 = 6.28;
    const ROT_ACC: f32 = 1.0;
    const ROT_DE_ACC: f32 = 0.25;

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
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        let ref mut cannon = self.cannon;

        let delta_rot: f32;

        // delta_rot = Cannon::ROT_ACC * context.time.delta().as_secs_f32();
        delta_rot = Cannon::ROT_ACC / 60.0;
        // apply an accelerating force in some direction
        // if let Some(rd) = self.input_state.cannon_rotate
        // {
        //     use RotateDir::*;
        // println!("{}", cannon.rot_vel);

        
            
        //     if rd == Left
        //     {
        //         // cannon.facing.rotate(0.5);
        //         // cannon.facing = cannon.facing.rotate_by(0.5);
        //         // cannon.rot_vel = f32::min(
        //         //     cannon.rot_vel + Cannon::ROT_ACC * context.time.delta().as_secs_f32(),
        //         //     Cannon::MAX_ROT_PER_SEC
        //         // );
        //         delta_rot = Cannon::ROT_ACC * context.time.delta().as_secs_f32();
        //     }
        //     else
        //     {
        //         // cannon.facing = cannon.facing.rotate_by(-0.5);
        //         delta_rot = -Cannon::ROT_ACC * context.time.delta().as_secs_f32();
        //     }
        // }
        // // apply a deccelerating force in opposite direction
        // else 
        // {
        //     delta_rot = Cannon::ROT_DE_ACC * if cannon.rot_vel > 0.0 { -1.0 } else { 1.0 } * context.time.delta().as_secs_f32();
        // }

        let mut new_rot_vel = cannon.rot_vel + delta_rot;
        new_rot_vel = new_rot_vel.min(Cannon::MAX_ROT_PER_SEC);
        cannon.rot_vel = new_rot_vel;

        // println!("{new_rot_vel}");

        // let new_facing = cannon.facing + delta_rot;
        cannon.facing = cannon.facing.rotate_by(cannon.rot_vel / 60.0);

        // cannon.facing = new_facing;
        // cannon.facing = cannon.facing.rotate_by(cannon.rot_vel * context.time.delta().as_secs_f32());
        // self.input_state.cannon_rotate = None;


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