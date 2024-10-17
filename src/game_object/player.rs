use ggez::glam::Vec2;

// local imports
use crate::util::message::Message;

#[derive(Default, Debug, Clone, derive_setters::Setters)]
pub struct Player
{
    pos: Vec2,
    vel: Vec2,
    feet_offset: Vec2,
    jump: Option<Message<()>>,
    grounded: bool,
}

impl Player
{
    pub const JUMP_TIME_BUFFER: f32 = 0.25;
    pub const _COYOTE_TIME: f32 = 0.25;

    pub fn issue_jump(&mut self)
    {
        // a jump message with some lifetime
        self.jump = Some(Message::new((), Player::JUMP_TIME_BUFFER));
    }
}

impl crate::FixedUpdate<Player> for crate::MainState
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        let player = &mut self.player;

        player.jump = match player.jump
        {
            Some(m) => 
            {
                m.act(player.grounded, || {player.vel.y -= 30.0; player.grounded = false; }, context.time.average_delta().as_secs_f32())
            },
            None => None,
        };

        // gravity
        player.vel.y += 9.8 / 30.0;

        player.pos += player.vel / 30.0;

        if (player.pos + player.feet_offset).y > 400.0
        {
            player.pos.y = 400.0 - player.feet_offset.y;
            player.grounded = true;
            player.vel.y = 0.0;
        }

        if player.pos.x > 400.0
        {
            player.pos.x = 0.0;
        }

        Ok(())
    }
}

impl crate::Draw<Player> for crate::MainState
{
    fn draw(&self, _context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        let draw_params = ggez::graphics::DrawParam::new().dest(self.player.pos).offset(Vec2::new(0.5, 0.5));
        canvas.draw(&self.assets.player_image, draw_params);

        Ok(())
    }
}