use game_object::{enemy::Enemy, enemy_wall::EnemyWall, grid::{Chunk, Object, PackedU8}, Region, HasPosition};
use ggez::{glam::{Vec2, Vec3, Vec4}, mint::{Vector2, Vector4}};
use serde::{Deserialize, Serialize};
use util::hash_map_tracker::HashMapTracker;
// use std::collections::HashMap;
use crate::game_object::HasRegion;

// local imports
mod game_object;
mod util;

use crate::game_object::{
    cannon::{Cannon, RotateDir}, 
    missile::Missile, 
    // player::Player, 
    Draw, 
    FixedUpdate, 
    Update
};

struct MainState
{
    screen: ggez::graphics::ScreenImage,
    input_state: InputState,
    assets: Assets,
    
    periscope: PeriscopeUniform,
    periscope_shader: ggez::graphics::Shader,

    /// The position of the camera, per se. 
    world_pos: Vec2,

    cannon: Cannon,
    missiles: HashMapTracker<Missile>,
    enemy_walls: HashMapTracker<EnemyWall>,
    chunks: Vec<Chunk>,

    enemies: Vec<Enemy>,
}

impl MainState
{
    const WINDOW_X: f32 = 800.0;
    const WINDOW_Y: f32 = 800.0;
    const FIXED_PHYSICS_FRAMERATE: u32 = 60;
    const FIXED_PHYSICS_TIMESTEP: f32 = 1.0 / MainState::FIXED_PHYSICS_FRAMERATE as f32;

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
        // let player = Player::default().feet_offset([0.0, 20.0].into()).grounded(false);
        let periscope = PeriscopeUniform::new([0.0, 0.0], 0.5);
        let periscope_shader = 
            ggez::graphics::ShaderBuilder::new().fragment_path("/periscope.wgsl").build(context)?;
        let cannon = Cannon::default();
        let missiles = HashMapTracker::new();
        let mut enemy_walls = HashMapTracker::new();

        let example_wall = 
            EnemyWall::default()
            .rect(ggez::graphics::Rect::new(0.0, 0.0, 50.0, 10.0))
            .transform(ggez::graphics::Transform::Values { dest: [600.0, 300.0].into(), rotation: -0.6, scale: [50.0, 10.0].into(), offset: [0.0, 0.0].into() });

        enemy_walls.push(example_wall);

        let chunk: Chunk = Chunk::default();
        let chunks = vec![chunk];

        let world_pos = [0.5, 0.5].into();

        let enemies = vec![
            // <Enemy::default()
            // .position_set((12.0, 5.0).into()) as HasRegion<game_object::collider_type::Collider>>::region_set()
            // .region_set(((-0.75, -0.75), (0.75, 0.75)).into())

            HasRegion::<game_object::collider_type::Collider>::region_set(
                Enemy::default().position_set((12.0, 5.0).into()), 
                ((-0.75, -0.75), (0.75, 0.75)).into())
        ];

        let s = MainState
        {
            input_state,
            screen,
            assets,
            periscope,
            periscope_shader,
            cannon,
            missiles,
            enemy_walls,
            chunks,
            world_pos,
            enemies
        };

        Ok(s)
    }
}

#[derive(Default)]
struct InputState
{
    left_click: Option<Vec2>,
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

        canvas.draw(&q, graphics::DrawParam::new().scale([Self::WINDOW_X, Self::WINDOW_Y]));
        Ok(())
    }
}

struct Assets
{
    player_image:   ggez::graphics::Image,
    cannon_image:   ggez::graphics::Image,
    missile_image:  ggez::graphics::Image,
    basic_object: ggez::graphics::Image,
}

impl Assets
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<Assets>
    {
        use ggez::graphics::Image;
        let player_image    = Image::from_path(context, "/dogRight0.png")?;
        let cannon_image    = Image::from_path(context, "/cannon.png")?;
        let missile_image   = Image::from_path(context, "/missile.png")?;
        let basic_object = Image::from_path(context, "/Object.png")?;
        
        Ok(
            Assets 
            { 
                player_image,
                cannon_image,
                missile_image,
                basic_object
            }
        )
    }
}

impl ggez::event::EventHandler for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        // fixed-update
        while context.time.check_update_time(MainState::FIXED_PHYSICS_FRAMERATE)
        {
            FixedUpdate::<Cannon>::fixed_update(self, context)?;
            FixedUpdate::<Vec<Chunk>>::fixed_update(self, context)?;
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

        Draw::<Cannon>::draw(self, context, &mut canvas)?;
        Draw::<HashMapTracker<Missile>>::draw(self, context, &mut canvas)?;

        Draw::<HashMapTracker<EnemyWall>>::draw(self, context, &mut canvas)?;

        Draw::<Vec<Chunk>>::draw(self, context, &mut canvas)?;

        Draw::<Vec<Enemy>>::draw(self, context, &mut canvas)?;

        // post effects
        // Draw::<PeriscopeUniform>::draw(self, context, &mut canvas)?;
        
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

        let ref mut input_state = self.input_state;

        let mut apply_movement = Vec2::ZERO;
        if let Some(kc) = input.keycode
        {
            match kc
            {
                Left    => input_state.cannon_rotate = Some(RotateDir::Left),
                Right   => input_state.cannon_rotate = Some(RotateDir::Right),
                W       => apply_movement.y -= 1.0,
                S       => apply_movement.y += 1.0,
                A       => apply_movement.x -= 1.0,
                D       => apply_movement.x += 1.0, 
                _       => ()
            };
        };

        self.world_pos += apply_movement;

        if input.keycode == Some(Space)
        {
            let s = postcard::to_stdvec(HasRegion::<game_object::collider_type::Collider>::region_get(&self.enemies[0])).unwrap();
            println!("enemy collider: {:?}", s);

            let s = postcard::to_stdvec(HasRegion::<game_object::collider_type::Selection>::region_get(&self.enemies[0])).unwrap();
            println!("enemy selection: {:?}", s);
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

        let ref mut input_state = self.input_state;

        if let Some(kc) = input.keycode
        {
            match kc
            {
                Left | 
                Right   => input_state.cannon_rotate = None,
                _       => ()
            }
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

        let ref mut input_state = self.input_state;

        match button
        {
            Left    => input_state.left_click = Some([x, y].into()),
            _       => (),
        };
        
        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut ggez::Context,
            button: ggez::event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> ggez::GameResult
    {
        use ggez::event::MouseButton::*;

        let ref mut input_state = self.input_state;
        
        match button
        {
            Left    => input_state.left_click = None,
            _       => (),
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
        .window_mode(conf::WindowMode::default().dimensions(MainState::WINDOW_X, MainState::WINDOW_Y))
        .add_resource_path(resource_dir);

    let (mut context, event_loop) = cb.build()?;
    let game = MainState::new(&mut context)?;
    
    event::run(context, event_loop, game);
}
