use ggez::{glam::{Vec2, Vec3, Vec4}, mint::{Vector2, Vector4}};
use util::hash_map_tracker::HashMapTracker;
// use std::collections::HashMap;

// local imports
mod game_object;
mod util;

use crate::game_object::{
    cannon::{Cannon, RotateDir}, 
    missile::Missile, 
    player::Player, 
    Draw, 
    FixedUpdate, 
    Update
};

struct MainState
{
    screen: ggez::graphics::ScreenImage,
    input_state: InputState,
    assets: Assets,
    player: Player,
    
    periscope: PeriscopeUniform,
    periscope_shader: ggez::graphics::Shader,

    cannon: Cannon,
    missiles: HashMapTracker<Missile>
}

impl MainState
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<MainState>
    {
        let input_state = InputState { ..std::default::Default::default() };
        let screen = 
            ggez::graphics::ScreenImage::new(
                context, 
                ggez::graphics::ImageFormat::Rgba8UnormSrgb, 
                1.0, 1.0, 1
            );
        let assets = Assets::new(context)?;
        let player = Player::default().feet_offset([0.0, 20.0].into()).grounded(false);
        let periscope = PeriscopeUniform::new([0.0, 0.0], 0.5);
        let periscope_shader = 
            ggez::graphics::ShaderBuilder::new().fragment_path("/periscope.wgsl").build(context)?;
        let cannon = Cannon::default();
        let missiles = HashMapTracker::new();

        let s = MainState
        {
            input_state,
            screen,
            assets,
            player,
            periscope,
            periscope_shader,
            cannon,
            missiles
        };

        Ok(s)
    }
}

#[derive(Default)]
struct InputState
{
    mouse_click: Option<Vec2>,
    cannon_rotate: Option<RotateDir>
}

impl Update<PeriscopeUniform> for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        let ps = &mut self.periscope;

        ps.position = context.mouse.position().into();

        Ok(())    
    }
}

impl Draw<PeriscopeUniform> for MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
    {
        let ps = &self.periscope;
        
        use ggez::graphics;
        // canvas.set_blend_mode(graphics::BlendMode::DARKEN);
        canvas.set_blend_mode(graphics::BlendMode::MULTIPLY);
        // let shader = graphics::ShaderBuilder::new().fragment_path("/periscope.wgsl").build(context)?;
        
        canvas.set_shader(&self.periscope_shader);
        let params = graphics::ShaderParamsBuilder::new(ps).build(context);
        canvas.set_shader_params(&params);

        let q = graphics::Quad;

        canvas.draw(&q, graphics::DrawParam::new().scale([400.0, 400.0]));
        Ok(())
    }
}

struct Assets
{
    player_image:   ggez::graphics::Image,
    cannon_image:   ggez::graphics::Image,
    missile_image:  ggez::graphics::Image,
}

impl Assets
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<Assets>
    {
        use ggez::graphics::Image;
        let player_image    = Image::from_path(context, "/dogRight0.png")?;
        let cannon_image    = Image::from_path(context, "/cannon.png")?;
        let missile_image   = Image::from_path(context, "/missile.png")?;
        
        Ok(
            Assets 
            { 
                player_image,
                cannon_image,
                missile_image
            }
        )
    }
}

impl ggez::event::EventHandler for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        // fixed-update
        while context.time.check_update_time(60)
        {
            FixedUpdate::<Player>::fixed_update(self, context)?;
            FixedUpdate::<Cannon>::fixed_update(self, context)?;
            FixedUpdate::<Cannon>::fixed_update(self, context)?;
            FixedUpdate::<HashMapTracker<Missile>>::fixed_update(self, context)?;
        }

        Update::<PeriscopeUniform>::update(self, context)?;

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        // Our drawing is quite simple.
        // Just clear the screen...
        use ggez::graphics::{self, Color};
        let mut canvas = 
            graphics::Canvas::from_frame(context, Color::WHITE);

        // pixel scaling, nearest-neighbor
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        Draw::<Player>::draw(self, context, &mut canvas)?;
        Draw::<Cannon>::draw(self, context, &mut canvas)?;
        Draw::<HashMapTracker<Missile>>::draw(self, context, &mut canvas)?;

        // post effects
        Draw::<PeriscopeUniform>::draw(self, context, &mut canvas)?;
        
        canvas.finish(context)?;
        
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> ggez::GameResult 
    {
        use ggez::input::keyboard::KeyCode::*;
        if input.keycode == Some(Space)
        {
            self.player.issue_jump();
        }

        if input.keycode == Some(Left)
        {
            self.input_state.cannon_rotate = Some(RotateDir::Left);
        }

        if input.keycode == Some(Right)
        {
            self.input_state.cannon_rotate = Some(RotateDir::Right);
        }

        Ok(())
    }

    fn key_up_event(
        &mut self, 
        _ctx: &mut ggez::Context, 
        input: ggez::input::keyboard::KeyInput
    ) -> ggez::GameResult 
    {
        use ggez::input::keyboard::KeyCode::*;

        if input.keycode == Some(Left) || input.keycode == Some(Right)
        {
            self.input_state.cannon_rotate = None;
        }

        Ok(())    
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            button: ggez::event::MouseButton,
            x: f32,
            y: f32,
        ) -> ggez::GameResult 
    {
        use ggez::event::MouseButton::*;
        if button == Left
        {
            self.input_state.mouse_click = Some([x, y].into());
        }
        
        Ok(())
    }
}

#[derive(Clone, crevice::std140::AsStd140)]
struct PeriscopeUniform
{
    position: Vector2<f32>,
    width: f32
}

impl PeriscopeUniform
{
    fn new(position: impl Into<Vector2<f32>>, width: f32) -> Self
    {
        let position = position.into();
        PeriscopeUniform { position, width }
    }
}

fn main() -> ggez::GameResult
{
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    // I hate file-global `use` statements
    // I prefer to aboslutely know where shit is coming from
    use ggez::*;
    let cb = 
        ContextBuilder::new("chess_thing", "Ethan Scheelk")
        .window_setup(conf::WindowSetup::default().title("Chess thing?").vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(400.0, 400.0))
        .add_resource_path(resource_dir);

    let (mut context, event_loop) = cb.build()?;
    let game = MainState::new(&mut context)?;
    
    event::run(context, event_loop, game);
}
