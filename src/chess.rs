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