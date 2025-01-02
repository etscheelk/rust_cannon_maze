// local imports
mod game_object;
mod util;
mod gui;
mod input;

use game_object::{enemy::Enemy, enemy_wall::EnemyWall, grid::{Chunk, Object, PackedU8}, Region, HasPosition};
use ggez::{glam::{Vec2, Vec3, Vec4}, mint::{Vector2, Vector4}};
use gui::GUIState;
use input::{ActionCode, ComboToAction, KeyInputState};
use serde::{Deserialize, Serialize};
use util::hash_map_tracker::HashMapTracker;
// use std::collections::HashMap;
use crate::game_object::HasRegion;


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

    gui_state: GUIState,
    gui: ggegui::Gui,

    key_input_state: KeyInputState,

    debug_state: DebugState,
}

#[derive(Default)]
struct DebugState
{
    draw_hitboxes: bool
}

impl MainState
{
    const WINDOW_X: f32 = 800.0;
    const WINDOW_Y: f32 = 800.0;
    const FIXED_PHYSICS_FRAMERATE: u32 = 60;
    const FIXED_PHYSICS_TIMESTEP: f32 = 1.0 / MainState::FIXED_PHYSICS_FRAMERATE as f32;

    fn new(context: &mut ggez::Context) -> ggez::GameResult<MainState>
    {
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
        let mut chunk2 = chunk.clone();
        chunk2.upper_left_position = [-16.0, 0.0].into();
        let mut chunk3 = chunk.clone();
        chunk3.upper_left_position = [-16.0, -16.0].into();
        let mut chunk4 = chunk.clone();
        chunk4.upper_left_position = [0.0, -16.0].into();

        let chunks = vec![chunk, chunk2, chunk3, chunk4];

        let world_pos = [0.5, 0.5].into();

        let enemies = vec![
            // <Enemy::default()
            // .position_set((12.0, 5.0).into()) as HasRegion<game_object::collider_type::Collider>>::region_set()
            // .region_set(((-0.75, -0.75), (0.75, 0.75)).into())

            HasRegion::<game_object::collider_type::Collider>::region_set(
                Enemy::default().position_set((12.0, 5.0).into()), 
                ((-0.75, -0.75), (0.75, 0.75)).into())
        ];

        let gui_state = GUIState::default();
        let gui = ggegui::Gui::new(&context);

        let key_input_state = KeyInputState::default();

        let debug_state = DebugState::default();

        let s = MainState
        {
            screen,
            assets,
            periscope,
            periscope_shader,
            cannon,
            missiles,
            enemy_walls,
            chunks,
            world_pos,
            enemies,
            gui_state,
            gui,

            key_input_state,

            debug_state,
        };

        Ok(s)
    }
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


impl Draw<ggegui::Gui> for MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
    {
        use ggez::graphics;

        canvas.draw(&self.gui, graphics::DrawParam::default().dest([0.0, 0.0]));
        
        Ok(())
    }
}

struct Assets
{
    player_image:   ggez::graphics::Image,
    cannon_image:   ggez::graphics::Image,
    missile_image:  ggez::graphics::Image,
    basic_object:   ggez::graphics::Image,
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

        // if context.time.ticks() % 120 == 0
        {
            // println!{"{:?}", context.time.fps()};
        }

        while context.time.check_update_time(MainState::FIXED_PHYSICS_FRAMERATE)
        {
            // FixedUpdate::<KeyInputState>::fixed_update(&mut self.key_input_state, context)?;
            self.key_input_state.fixed_update(context)?;

            // println!("{:?}", self.key_input_state.held_actions);
            // if context.time.ticks() % 200 == 0
            {
                // println!("{:?}", self.key_input_state.held_actions);
                // println!("{:?}", self.key_input_state.pressed_buttons);
            }

            // check debug state
            if self.key_input_state.held_actions.contains(&input::TriggerActionCode::FlipDebugHitboxes.into())
            {
                self.debug_state.draw_hitboxes = !self.debug_state.draw_hitboxes;
            }

            // update world pos
            let mut apply_movements = Vec2::ZERO;
            {
                use input::HoldActionCode::*;
                [CameraUp.into(), CameraDown.into(), CameraLeft.into(), CameraRight.into()]
                .into_iter()
                .for_each(|ac|
                {
                    if self.key_input_state.held_actions.contains(&ac)
                    {
                        // match ac
                        if let ActionCode::Hold(hac) = ac
                        {
                            match hac
                            {
                                CameraUp => apply_movements.y -= 1.0,
                                CameraDown => apply_movements.y += 1.0,
                                CameraLeft => apply_movements.x -= 1.0,
                                CameraRight => apply_movements.x += 1.0,
                                _ => (),
                            };

                        }
                    }
                });
            }

            self.world_pos += apply_movements;

            FixedUpdate::<Cannon>::fixed_update(self, context)?;
            FixedUpdate::<Vec<Chunk>>::fixed_update(self, context)?;
            FixedUpdate::<HashMapTracker<Missile>>::fixed_update(self, context)?;

            // self.key_input_state.held_actions.clear();
        }
        
        Update::<ggegui::Gui>::update(self, context)?;
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

        Draw::<ggegui::Gui>::draw(self, context, &mut canvas)?;

        // post effects
        // Draw::<PeriscopeUniform>::draw(self, context, &mut canvas)?;
        
        canvas.finish(context)?;
        
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
            &mut self,
            context: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            repeated: bool,
        ) -> ggez::GameResult 
    {
        // println!("{:?}", input);

        // self.input_state.key_down_event(context, input, repeated)
        self.key_input_state.key_down_event(context, input, repeated)
    }

    fn key_up_event(
        &mut self, 
        context: &mut ggez::Context, 
        input: ggez::input::keyboard::KeyInput
    ) -> ggez::GameResult 
    {
        // println!("\tkey up: {:?}", input);

        // self.input_state.key_up_event(context, input)  
        self.key_input_state.key_up_event(context, input)
    }

    fn mouse_button_down_event(
            &mut self,
            context: &mut ggez::Context,
            button: ggez::event::MouseButton,
            x: f32,
            y: f32,
        ) -> ggez::GameResult 
    {   
        self.key_input_state.mouse_button_down_event(context, button, x, y)
    }

    fn mouse_button_up_event(
            &mut self,
            context: &mut ggez::Context,
            button: ggez::event::MouseButton,
            x: f32,
            y: f32,
        ) -> ggez::GameResult
    {
        self.key_input_state.mouse_button_up_event(context, button, x, y)
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
