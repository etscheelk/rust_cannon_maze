use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};

// local imports
use crate::{game_object::{HasPosition, HasRegion}, util::{hash_map_tracker::{ForTracker, HashMapTracker, WithIndex}, vec_extension::{Flip, RotateBy}}, MainState};

use super::{collider_type::Collider, has_position, has_region, Region};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Missile
{
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    position: Vec2,
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    vel: Vec2,
    collision_region: Region<Collider>,

    index: Option<u16>,
}

has_position!(Missile);
has_region!(Missile, collision_region, Collider);

impl Missile
{
    pub fn new(pos: Vec2, vel: Vec2) -> Self
    {
        Self
        {
            position: pos,
            vel,
            index: Default::default(),
            collision_region: ((-0.4, -0.4), (0.4, 0.4)).into(),
        }
    }
}

impl ForTracker for Missile {}
impl WithIndex for Missile
{
    fn with_index(mut self, index: u16) -> Self 
    {
        self.index = Some(index);
        self
    }
}

impl crate::FixedUpdate<HashMapTracker<Missile>> for crate::MainState
{
    fn fixed_update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        let missiles = &mut self.missiles;
        
        for (_, missile) in missiles.get_tracker_mut()
        {
            missile.position += missile.vel * MainState::FIXED_PHYSICS_TIMESTEP;
        }
        

        // add a missile if necessary
        if let Some(_mouse_click_point) = self.input_state.mouse_click
        {
            // let mut vel = 50.0 * self.cannon.facing;
            // vel.y *= -1.0;

            let mut missile_vel = 2.0 * self.cannon.facing;
            missile_vel = missile_vel.flip_y();
            // missile_vel.y *= -1.0; // account for flipped coords
            
            
            // the spawn point of the missile should be the cannon's tip
            // for now, we'll add the width of the cannon asset and account for its rotation
            // FIXME, this is terrible, do not base your spawning off of your assets
            // make the spawning fixed and modify assets
            // FIXME, image size does not account for having scaled it
            // let pos: Vec2 = self.cannon.position + self.assets.cannon_image.width() as f32 * self.cannon.facing.flip_y() / 16.0;
            
            // spawn offset from cannon (number of world units)
            let barrel_length = 3.0;
            let pos: Vec2 = self.cannon.position + barrel_length * self.cannon.facing.flip_y();
            let m = Missile::new(pos, missile_vel);

            println!("new missile pos: {pos}");

            // let m = Missile::new(point, 50.0 * self.cannon.facing);
            // let m = Missile::new(point, missile_vel);
            missiles.push(m);
        }
        self.input_state.mouse_click = None;

        // TODO: Do this boundary check elsewhere, in the first loop over all
        missiles
        .iter()
        .map(
        |(&ind, m)|
        {
            let x_range = -100.0..(Self::WINDOW_X+100.0);
            let y_range = -100.0..(Self::WINDOW_Y+100.0);

            if x_range.contains(&m.position.x) && y_range.contains(&m.position.y)
            {
                None
            }
            else
            {
                Some(ind)
            }
        })
        .flatten()
        .collect::<Vec<u16>>()
        .iter()
        .for_each(
        |&ind|
        {
            missiles.delete(ind);
        });

        Ok(())
    }
}

impl crate::Draw<HashMapTracker<Missile>> for crate::MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        let missiles = &self.missiles;
        
        use ggez::graphics;

        // TODO: Update to use instanced array
        // for fast drawing
        for (_, missile) in missiles.get_tracker()
        {
            let missile_screen_pos = 16.0 * (missile.position_get() - self.world_pos);

            let rotation = -missile.vel.angle_between(Vec2::X);

            let transform = 
            graphics::Transform::Values 
            { 
                dest: missile_screen_pos.into(), 
                rotation,
                scale: [1.0, 1.0].into(), 
                // offset: [0.0, self.assets.missile_image.height() as f32 / 2.0].into()
                // offset: Vec2::from([0.0, 0.0]).rotate_by(rotation).into()
                offset: [0.0, 8.0].into(), // offset by half the asset's height
            };

            let big_missile = graphics::Image::from_path(context, "/missile_big.png")?;

            let param = 
                graphics::DrawParam::new()
                .transform(transform.to_bare_matrix());
            // canvas.draw(&self.assets.missile_image, param);
            canvas.draw(&big_missile, param);

            missile.region_get().draw(missile, self.world_pos, missile_screen_pos, context, canvas)?;
        }
        
        Ok(())
    }
}