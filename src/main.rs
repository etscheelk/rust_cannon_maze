use ggez::glam::Vec2;

struct MainState
{
    called: u32,
    screen: ggez::graphics::ScreenImage,
    input_state: InputState,
    assets: Assets,
    player: Player
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
            input_state: InputState {  },
            screen,
            assets,
            player: Player::default().feet_offset(Vec2::new(0.0, 10.0))
        };

        Ok(s)
    }
}

#[derive(Default, Debug, Clone)]
struct Cell<'a>
{
    piece: Option<&'a Piece>,
    index: u16,
    pos: Vec2,
    selected: bool
}


#[derive(Default, Debug, Clone)]
struct Piece
{
    piece_type: PieceType,
    piece_color: PieceColor
}

impl Piece
{
    fn move_piece(&mut self)
    {

    }
}

#[derive(Default, Debug, Clone, Copy)]
#[non_exhaustive]
enum PieceColor
{
    #[default]
    Uncolored,
    Light,
    Dark
}

#[derive(Default, Debug, Clone, Copy)]
#[non_exhaustive]
enum PieceType
{
    #[default]
    Pawn,
    Rook, 
    Bishop,
    Knight,
    Queen,
    King,
}

enum CellColor
{
    Light,
    Dark
}

struct InputState
{

}

#[derive(Default, Debug, Clone, derive_setters::Setters)]
struct Player
{
    pos: Vec2,
    vel: Vec2,
    feet_offset: Vec2
}

impl ggez::event::EventHandler for Player
{
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.vel.y += 4.9 / 60.0;
        self.pos += self.vel / 60.0;

        if (self.pos + self.feet_offset).y > 400.0
        {
            self.pos.y = 400.0 - self.feet_offset.y;
        }

        if self.pos.x > 400.0
        {
            self.pos.x = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        todo!()
    }
}

struct Assets
{
    player_image: ggez::graphics::Image
}

impl Assets
{
    fn new(context: &mut ggez::Context) -> ggez::GameResult<Assets>
    {
        let player_image = ggez::graphics::Image::from_path(context, "/dogRight0.png")?;

        Ok(Assets { player_image })
    }
}

impl ggez::event::EventHandler for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        
        

        while context.time.check_update_time(60)
        {
            self.player.update(context)?;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        // self.player.draw(context)?;

        // Our drawing is quite simple.
        // Just clear the screen...
        use ggez::graphics::{self, Color};
        let mut canvas = 
            graphics::Canvas::from_screen_image(context, &mut self.screen, Color::BLACK);

        let draw_params = graphics::DrawParam::new().dest(self.player.pos).offset(Vec2::new(0.5, 0.5));
        canvas.draw(&self.assets.player_image, draw_params);

        canvas.finish(context)?;

        context.gfx.present(&self.screen.image(context))?;

        ggez::timer::yield_now();
        Ok(())
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
        .window_setup(conf::WindowSetup::default().title("Chess thing?"))
        .window_mode(conf::WindowMode::default().dimensions(400.0, 400.0))
        .add_resource_path(resource_dir);

    let (mut context, event_loop) = cb.build()?;

    // let game = MainState { called: 0 };
    let game = MainState::new(&mut context)?;
    event::run(context, event_loop, game);

    // Ok(())
}
