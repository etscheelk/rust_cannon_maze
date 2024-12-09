use std::ops::{Index, IndexMut};

use ggez::glam::Vec2;

#[derive(Debug, Clone, Default, Copy)]
pub struct Object
{
    pub background_object: ObjectType,
    pub foreground_object: ObjectType,
    pub id: PackedU8,
}

/// permits the packaging of two four-bit numbers into the size of 8 bits
/// each a-b has a value 0-15
#[derive(Copy, Clone, Default, Hash)]
pub struct PackedU8(u8);

impl PackedU8
{
    pub fn new(a: u8, b: u8) -> Self
    {
        assert!(a <= 15, "a is greater than 15");
        assert!(b <= 15, "b is greater than 15");

        // PackedU8((a << 4) + b)
        PackedU8((b << 4) + a)
    }

    pub fn a(self) -> u8
    {
        self.0 & 15
    }

    pub fn b(self) -> u8
    {
        self.0 >> 4
    }
}

impl std::fmt::Debug for PackedU8
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "({}, {})", self.a(), self.b())
    }
}

impl From<(u8, u8)> for PackedU8
{
    fn from(value: (u8, u8)) -> Self {
        PackedU8::new(value.0, value.1)
    }
}

impl From<(f32, f32)> for PackedU8
{
    fn from(value: (f32, f32)) -> Self {
        PackedU8::new(value.0 as u8, value.1 as u8)
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub enum ObjectType
{
    #[default]
    None,
    Filled
}

#[derive(Debug, Clone)]
pub struct Chunk
{
    // array of 16x16 cells
    pub array: [Object; 256],
    pub id: PackedU8,
    pub upper_left_position: Vec2,
}

impl Index<PackedU8> for Chunk
{
    type Output = Object;

    fn index(&self, index: PackedU8) -> &Self::Output 
    {
        &self.array[index.0 as usize]
    }
}


impl IndexMut<PackedU8> for Chunk
{
    fn index_mut(&mut self, index: PackedU8) -> &mut Self::Output {
        &mut self.array[index.0 as usize]
    }
}

impl Chunk
{
    fn contains_world_point(&self, pt: Vec2) -> bool
    {
        let ref ul = self.upper_left_position;
        let x_range = ul.x..(ul.x+16.0); // a single tile is 1.0
        let y_range = ul.y..(ul.y+16.0);
        
        x_range.contains(&pt.x) && y_range.contains(&pt.y)
    }
}

impl Default for Chunk
{
    fn default() -> Self {
        let mut array = [Object::default(); 256];
        for i in 0..=255u8
        {
            // println!("{i}: {}, {}", i % 16, i / 16);
            array[i as usize].id = PackedU8::new(i % 16, i / 16);
        }
        Self { array, id: Default::default(), upper_left_position: Default::default() }
    }
}

impl crate::FixedUpdate<Vec<Chunk>> for crate::MainState
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        let ref mut chunks = self.chunks;

        // get mouse click location and map it to a cell and fill it if possible
        if let Some(mut pos) = self.input_state.mouse_click
        {
            // let mut pos: Vec2 = context.mouse.position().into();
            // map mouse click to somewhere in world coordinate space
            pos = pos / 16.0 + self.world_pos;

            for chunk in chunks
            {
                if chunk.contains_world_point(pos)
                {
                    pos = pos - chunk.upper_left_position;

                    // pos will now be a float [0.0, 16.0)
                    // truncate / round down to get x and y position within chunk
                    let Vec2 {x, y} = pos.trunc();
                    // println!("{}, {}", x, y);
                    chunk[(x, y).into()].foreground_object = ObjectType::Filled;

                    // chunk.array[0].foreground_object = ObjectType::Filled;


                    break;
                }

                
            }
        }
        
        Ok(())    
    }
}

impl crate::Draw<Vec<Chunk>> for crate::MainState
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
    {
        use ggez::graphics;

        // apparent size of a given Object: 16x16 pixels
        // this means a given chunk is 256x256 pixels
        let ref chunks = self.chunks;

        for chunk in chunks
        {
            let pos = chunk.upper_left_position - self.world_pos;
            for object in chunk.array
            {
                // object will get drawn to screen at a*16 + ul.x, a*16 + ul.y
                let dest_pos = 
                    16.0 * (pos + Vec2::new(object.id.a() as f32, object.id.b() as f32));

                let params = 
                    graphics::DrawParam::new()
                    .dest(dest_pos)
                    .z(-100);

                match object.foreground_object
                {
                    ObjectType::Filled =>
                    {
                        let img = graphics::Image::from_path(context, "/FilledObject.png")?;
                        canvas.draw(&img, params);
                    },
                    _ => canvas.draw(&self.assets.basic_object, params),
                };
            }
        }

        Ok(())
    }
}

struct SuperChunk
{
    array: [Chunk; 256],
    id: PackedU8,
}