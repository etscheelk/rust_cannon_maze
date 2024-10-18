use ggez::glam::Vec2;

// local imports
use crate::util::hash_map_tracker::HashMapTracker;

#[derive(Debug, Clone)]
pub struct Missile
{
    pos: Vec2,
    vel: Vec2,
    _size: (f32, f32),
    index: u16,
}

impl Missile
{
    pub const _WIDTH: f32 = 0.05;

    pub fn new(pos: Vec2, vel: Vec2, index: u16) -> Self
    {
        Self
        {
            pos,
            vel,
            index,
            _size: (8.0, 8.0)
        }
    }
}

impl crate::FixedUpdate<HashMapTracker<Missile>> for crate::MainState
{
    fn fixed_update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        let missiles = &mut self.missiles;

        // update each missile
        for (&_index, missile) in &mut missiles.1
        {
            missile.pos += missile.vel / 60.0; // FIXME: Hardcoded
        }

        // add a missile if necessary
        if let Some(point) = self.input_state.mouse_click
        {
            let m = Missile::new(point, 50.0 * self.cannon.facing, missiles.0);
            missiles.push(m);
        }
        self.input_state.mouse_click = None;

        // TODO
        // Check missile boundaries and despawn if necessary
        missiles.1
        .iter()
        .fold(vec![], 
        |mut acc, (&ind, m)|
        {
            if m.pos.x < -100.0 || m.pos.x > 500.0 || m.pos.y < -100.0 || m.pos.y > 500.0
            {
                acc.push(ind);
            }
            acc
        })
        .iter()
        .for_each(
        |&ind|
        {
            missiles.delete(ind);
        });

        

        // todo!()
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
        for (_, missile) in &missiles.1
        {
            let param = 
                graphics::DrawParam::new()
                .dest(missile.pos)
                .rotation(missile.vel.angle_between(Vec2::X))
                .scale([5.0, 5.0]);
            canvas.draw(&self.assets.missile_image, param);
        }
        
        Ok(())
    }
}