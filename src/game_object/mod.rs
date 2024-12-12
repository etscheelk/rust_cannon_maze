pub mod cannon;
pub mod missile;
// pub mod player;
pub mod enemy_wall;
pub mod grid;
pub mod enemy;

use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};

/// Update is a trait describing an object which should be updated every single frame
pub(crate) trait Update<I>
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult;
}

/// FixedUpdate is a trait describing an object which should be updated 
/// every fixed interval, such as 60 times a second for 60fps.
/// 
/// Use for things like physics especially.
/// 
/// Implement like 
/// `impl crate::FixedUpdate<Cannon> for crate::MainState`, where 
/// MainState has member variable(s) of type Cannon
pub(crate) trait FixedUpdate<I>
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult;
}

/// Draw is a trait describing an object which will be drawn to the 
/// supplied canvas in some way.
pub(crate) trait Draw<I>
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult;
}

/// A draw implementation that may impact the game state
/// Probably don't use this
pub(crate) trait DrawMut<I>
{
    fn draw_mut(&mut self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult;
}

/// A trait providing that a type has a field `position` with
/// an applicable getter and setter.
/// 
/// You should probably avoid implementing this yourself and instead
/// use `has_position!` proc-macro.
/// 
/// FIXME or TRYME: Make this trait dyn-possible by making position_set 
/// not return type self, and instead make it &mut self.
pub trait HasPosition
{
    fn position_set(self, position: Vec2) -> Self;
    fn position_get(&self) -> Vec2;
}

macro_rules! has_position {
    ($struct_name:ident) => {
        impl crate::game_object::HasPosition for $struct_name
        {
            fn position_set(mut self, position: Vec2) -> Self
            {
                self.position = position;
                self
            }

            fn position_get(&self) -> Vec2
            {
                self.position
            }
        }
    };
}

pub(crate) use has_position;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Region<T: ColliderType>
{
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    pub p0: Vec2,
    #[serde(with = "crate::util::vec_extension::_Vec2Ser")]
    pub p1: Vec2,
    t: std::marker::PhantomData<T>
}

pub trait ColliderType {}

pub mod collider_type
{
    use serde::{Serialize, Deserialize};

    use super::ColliderType; 

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub struct Collider; impl ColliderType for Collider {}
    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub struct Selection; impl ColliderType for Selection {}
}

impl<T: ColliderType> Region<T>
{
    const DRAWN_COLOR: ggez::graphics::Color = ggez::graphics::Color::MAGENTA;

    pub fn new(p0: Vec2, p1: Vec2) -> Self
    {
        assert!(p0.x <= p1.x);
        assert!(p0.y <= p1.y);

        Region::<T>
        {
            p0,
            p1,
            t: std::marker::PhantomData
        }
    }

    fn intersects(&self, local_pos: Vec2) -> bool
    {
        self.p0.x <= local_pos.x && self.p0.y <= local_pos.y &&
        local_pos.x <= self.p1.x && local_pos.y <= self.p1.y
    }

    pub fn draw<Parent>
    (
        &self, 
        parent: &Parent, 
        world_pos: Vec2, 
        screen_pos: Vec2, 
        context: &mut ggez::Context, 
        canvas: &mut ggez::graphics::Canvas
    ) -> ggez::GameResult
    where
        Parent: HasRegion<T> 
    {
        use ggez::graphics;

        // let cb = enemy.collision_box_get();
        let cb = self;
            
            
        let a: Vec2 = cb.p0;
        let b: Vec2 = (cb.p1.x, cb.p0.y).into();
        let c: Vec2 = cb.p1;
        let d: Vec2 = (cb.p0.x, cb.p1.y).into();

        let points = [a * 16.0, b * 16.0, c * 16.0, d * 16.0];

        let mouse_pos: Vec2 = context.mouse.position().into();
        let mouse_world_pos = mouse_pos / 16.0 + world_pos;

        let mut color = graphics::Color::MAGENTA;
        if parent.intersects_region(mouse_world_pos)
        {
            color = graphics::Color::BLACK;
        }
        
        let mesh = 
            graphics::Mesh::new_polygon(
                context, 
                graphics::DrawMode::stroke(2.0), 
                &points, 
                color
            )?;
        
        let cb_transform = 
            graphics::Transform::Values 
            { 
                dest: screen_pos.into(), 
                rotation: 0.0, 
                scale: [1.0, 1.0].into(), 
                offset: [0.0, 0.0].into() 
            };

        canvas.draw(&mesh, graphics::DrawParam::new().transform(cb_transform.to_bare_matrix()));
        
        Ok(())
    }
}

// impl crate::Draw<CollisionBox> for CollisionBox
// {
//     fn draw(&self, screen_pos: Vec2, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult 
//     {
//         todo!()    
//     }
// }

impl<T: ColliderType> From<(Vec2, Vec2)> for Region::<T>
{
    fn from(value: (Vec2, Vec2)) -> Self 
    {
        Region::<T> { p0: value.0, p1: value.1, t: std::marker::PhantomData }    
    }
}

impl<T: ColliderType> From<((f32, f32), (f32, f32))> for Region::<T>
{
    fn from(value: ((f32, f32), (f32, f32))) -> Self 
    {
        Region::<T> { p0: value.0.into(), p1: value.1.into(), t: std::marker::PhantomData }    
    }
}

/// A trait providing that a type has a field `collision_box`
/// with the appropriate getters and setters.
/// 
/// You should probably avoid implementing this yourself and use
/// proc-macro `has_collision_box!` instead.
pub trait HasRegion<T: ColliderType>: HasPosition
{
    fn region_get(&self) -> &Region::<T>;

    fn region_set(self, collision_box: Region::<T>) -> Self;

    fn intersects_region(&self, world_pos: Vec2) -> bool
    {
        let local_pos = world_pos - self.position_get();
        self.region_get().intersects(local_pos)
    }
}

macro_rules! has_region {
    ($struct_name:ty, $field_name:ident, $collider_type:ty) => 
    {
        impl crate::game_object::HasRegion<$collider_type> for $struct_name
        {
            fn region_get(&self) -> &Region<$collider_type>
            {
                &self.$field_name
            }

            fn region_set(mut self, $field_name: Region<$collider_type>) -> Self
            {
                self.$field_name = $field_name;
                self
            }
        }    
    };
}

pub(crate) use has_region;