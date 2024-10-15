use ggez::{glam::Vec2, mint::{Point2, Vector2, Vector4}};
use std::collections::HashMap;

struct MainState
{
    called: u32,
    screen: ggez::graphics::ScreenImage,
    input_state: InputState,
    assets: Assets,
    player: Player,
    periscope: PeriscopeUniform,
    missiles: HashMapTracker<Missile, 1024>
}

struct HashMapTracker<I, const MAX: u16 = 1024>(u16, HashMap<u16, I>);

impl<I, const MAX: u16> HashMapTracker<I, MAX>
{
    fn push(&mut self, i: I) -> Option<u16>
    {
        if self.1.len() + 1 >= MAX as usize
        {
            None
        }
        else
        {
            self.1.insert(self.0, i);
            let ret = Some(self.0);
            self.0 = (self.0 + 1) % MAX;

            ret
        }
    }

    fn push_iter(&mut self, i: impl IntoIterator<Item = I>) -> Option<Vec<u16>>
    {
        let i = i.into_iter();

        let v = i.map(
        |i: I|
        {
            self.push(i)
        }).collect();

        return v;
    }
}

impl MainState
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<MainState>
    {
        let screen = 
            ggez::graphics::ScreenImage::new(
                context, 
                ggez::graphics::ImageFormat::Rgba8UnormSrgb, 
                1.0, 1.0, 1
            );
        
        let assets = Assets::new(context)?;

        let s = MainState
        {
            called: 0,
            input_state: InputState { ..std::default::Default::default() },
            screen,
            assets,
            player: Player::default().feet_offset(Vec2::new(0.0, 20.0)).grounded(false),
            periscope: PeriscopeUniform::new([0.0, 0.0].into(), 0.5),
            missiles: HashMapTracker(0, HashMap::<u16, Missile>::new())
        };

        Ok(s)
    }
}

#[derive(Default)]
struct InputState
{
    i: i32,
}

struct Missile
{
    position: Point2<f32>,
    velocity: Vector2<f32>
}

impl Missile
{
    const WIDTH: f32 = 0.05;
}

#[derive(Default, Debug, Clone, derive_setters::Setters)]
struct Player
{
    pos: Vec2,
    vel: Vec2,
    feet_offset: Vec2,
    jump: Option<Message<()>>,
    grounded: bool,
}

impl Player
{
    const JUMP_TIME_BUFFER: f32 = 0.25;
    const COYOTE_TIME: f32 = 0.25;

    fn issue_jump(&mut self)
    {
        // a jump message with some lifetime
        self.jump = Some(Message((), Self::JUMP_TIME_BUFFER));
    }
}

#[derive(Debug, Clone, Copy)]
struct Message<T>(T, f32);

impl<T> Message<T>
{
    fn act<ACTION>(mut self, predicate: bool, mut action: ACTION, dt: f32) -> Option<Self>
    where
        // PRED: FnOnce() -> bool,
        ACTION: FnMut() -> (),
    {
        if predicate
        {
            action();
            None
        }
        else
        {
            self.1 -= dt;
            if self.1 < 0.0 
            { 
                // println!("timer expired"); 
                None 
            }
            else { Some(self) }
        }
    }
}


/// A trait type specific to my game, recreating some of the functions of
/// ggez::event::EventHandler, but with extra context of the main state and the modifiable canvas
trait GameObject<I>
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult;
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult
    {
        Ok(())
    }
}

// This is probably a terrible idea in terms of compartmentalization 
impl GameObject<Player> for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
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

    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        let draw_params = ggez::graphics::DrawParam::new().dest(self.player.pos).offset(Vec2::new(0.5, 0.5));
        canvas.draw(&self.assets.player_image, draw_params);

        Ok(())
    }
}

impl GameObject<PeriscopeUniform> for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        let ps = &mut self.periscope;

        ps.position = context.mouse.position().into();

        Ok(())    
    }

    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
    {
        let ps = &self.periscope;
        
        use ggez::graphics;
        // canvas.set_blend_mode(graphics::BlendMode::ALPHA);
        let shader = graphics::ShaderBuilder::new().fragment_path("/periscope.wgsl").build(context)?;
        
        canvas.set_shader(&shader);
        let params = graphics::ShaderParamsBuilder::new(ps).build(context);
        canvas.set_shader_params(&params);

        let q = graphics::Quad;

        canvas.draw(&q, graphics::DrawParam::new().scale(Vec2::new(400.0, 400.0)));
        Ok(())
    }
}

struct Assets
{
    player_image: ggez::graphics::Image,
    missile_image: ggez::graphics::Image,
}

impl Assets
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<Assets>
    {
        let player_image = ggez::graphics::Image::from_path(context, "/dogRight0.png")?;
        let missile_image = ggez::graphics::Image::from_path(context, "/missile.png")?;
        // white square
        // let player_image = ggez::graphics::Image::from_color(context, 30, 30, None);

        Ok(
            Assets 
            { 
                player_image,
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

            GameObject::<Player>::update(self, context)?;

            // <self as GameObject>.self.update(context);

            self.player.pos.x = 20.0;

            // println!("{:?}", self.player.pos);
        }

        GameObject::<PeriscopeUniform>::update(self, context)?;

        if context.time.ticks() % 100 == 0
        {
            println!("fps: {}", context.time.fps())
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        // self.player.draw(context)?;

        // Our drawing is quite simple.
        // Just clear the screen...
        use ggez::graphics::{self, Color};
        let mut canvas = 
            // graphics::Canvas::from_screen_image(context, &mut self.screen, Color::BLACK);
            graphics::Canvas::from_frame(context, Color::BLACK);

        // let draw_params = graphics::DrawParam::new().dest(self.player.pos).offset(Vec2::new(0.5, 0.5));
        // canvas.draw(&self.assets.player_image, draw_params);
        GameObject::<Player>::draw(self, context, &mut canvas)?;

        // let a= 
        //     graphics::MeshBuilder::new().rectangle(graphics::DrawMode::Fill(FillOptions::DEFAULT), graphics::Rect::new(20.0, 20.0, 20.0, 20.0), Color::CYAN)?.build();
        // let a = graphics::Mesh::new_rectangle(context, graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT), graphics::Rect::new(0.0, 0.0, 400.0, 400.0), Color::CYAN)?;
        let a = graphics::Quad;
        let shader = graphics::ShaderBuilder::new().fragment_path("/shader_a.wgsl").build(context)?;
        canvas.set_shader(&shader);
        // // let mut mu = MyUniform { color: crevice::std140::Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0 } };
        // // let mut mu = CustomColor { color: [1.0, 0.0, 0.0, 1.0].into() };
        let uniform = CustomColor { color: [1.0, 0.0, 0.0, 0.5].into() };
        let uniform = graphics::ShaderParamsBuilder::new(&uniform).build(context);

        canvas.set_shader_params(&uniform);

        canvas.draw(&a, graphics::DrawParam::new().dest(Vec2::new(0.0, 0.0)).scale(Vec2::new(400.0, 400.0)));
        // // let mu = MyUniform { rate: 0.5 };

        // uniform.set_uniforms(context, &CustomColor { color: [0.0, 0.0, 1.0, 0.5].into() });
        // canvas.set_shader_params(&uniform);
        // canvas.set_blend_mode(graphics::BlendMode::ALPHA);
        
        // canvas.draw(&a, graphics::DrawParam::new().dest(Vec2::new(20.0, 20.0)).scale(Vec2::new(360.0, 360.0)));
        
        // let params = graphics::ShaderParamsBuilder::new(&mu);

        GameObject::<PeriscopeUniform>::draw(self, context, &mut canvas)?;
        


        canvas.finish(context)?;

        // context.gfx.present(&self.screen.image(context))?;
        
        // ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
            &mut self,
            ctx: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> ggez::GameResult 
    {
        use ggez::input::keyboard::KeyCode::*;
        if input.keycode == Some(Space)
        {
            // self.player.sent_jump = true;
            self.player.issue_jump();
        }

        Ok(())
        // todo!()
    }
}

// #[repr(C)]
#[derive(Clone, crevice::std140::AsStd140)]
struct CustomColor 
{
    color: Vector4<f32>
    // color: crevice::std140::Vec4
}

#[derive(Clone, crevice::std140::AsStd140)]
struct PeriscopeUniform
{
    position: Point2<f32>,
    width: f32
}

impl PeriscopeUniform
{
    fn new(position: Point2<f32>, width: f32) -> Self
    {
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

    // let piece_order = 
    // {
    //     use PieceType::*;
    //     vec![Rook,  Bishop, Knight, King,   Queen,  Knight, Bishop, Rook,
    //          Pawn,  Pawn,   Pawn,   Pawn,   Pawn,   Pawn,   Pawn,   Pawn]
    // };

    // let mut board: [Cell; 64] = array_macro::array!(Cell::default(); 64);

    // let mut pieces: Vec<Piece> = 
    //     piece_order
    //     .iter()
    //     // .cloned()
    //     .map(
    //     |&piece_type|
    //     {
    //         Piece { piece_type, ..Default::default() }
    //     })
    //     .collect();

    // pieces
    // .iter_mut()
    // .zip(board.iter_mut())
    // .for_each(
    // |(p, c)|
    // {
    //     c.piece = Some(p);
    // });

    // println!("board: {:#?}", board);

    // I hate file-global `use` statements
    // I prefer to aboslutely know where shit is coming from
    use ggez::*;
    let cb = 
        ContextBuilder::new("chess_thing", "Ethan Scheelk")
        .window_setup(conf::WindowSetup::default().title("Chess thing?").vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(400.0, 400.0))
        .add_resource_path(resource_dir);

    let (mut context, event_loop) = cb.build()?;

    // let game = MainState { called: 0 };
    let game = MainState::new(&mut context)?;
    event::run(context, event_loop, game);

    // Ok(())
}
