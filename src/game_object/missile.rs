use ggez::glam::Vec2;

// local imports
use crate::util::hash_map_tracker::{HashMapTracker, ForTracker, WithIndex};

#[derive(Debug, Clone)]
pub struct Missile
{
    pos: Vec2,
    vel: Vec2,
    _size: (f32, f32),
    index: Option<u16>,
}

impl Missile
{
    pub const _WIDTH: f32 = 0.05;

    pub fn new(pos: Vec2, vel: Vec2) -> Self
    {
        Self
        {
            pos,
            vel,
            index: Default::default(),
            _size: (8.0, 8.0)
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
            missile.pos += missile.vel / 60.0;
        }
        

        // add a missile if necessary
        if let Some(point) = self.input_state.mouse_click
        {
            // let mut vel = 50.0 * self.cannon.facing;
            // vel.y *= -1.0;
            let mut missile_vel = 50.0 * self.cannon.facing;
            missile_vel.y *= -1.0;
            // let m = Missile::new(point, 50.0 * self.cannon.facing);
            let m = Missile::new(point, missile_vel);
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

            if x_range.contains(&m.pos.x) && y_range.contains(&m.pos.y)
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
    fn draw(&self, _context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        let missiles = &self.missiles;
        
        use ggez::graphics;

        // TODO: Update to use instanced array
        // for fast drawing
        for (_, missile) in missiles.get_tracker()
        {
            let param = 
                graphics::DrawParam::new()
                .dest(missile.pos)
                .rotation(-missile.vel.angle_between(Vec2::X))
                .scale([5.0, 5.0]);
            canvas.draw(&self.assets.missile_image, param);
        }
        
        Ok(())
    }
}