use std::default;

#[derive(Debug, Clone, Copy)]
pub struct Message<T>(pub T, pub f32);

impl<T> Message<T>
{
    pub fn new(t: T, lifetime: f32) -> Self
    {
        Message(t, lifetime)
    }

    pub fn act<ACTION>(mut self, predicate: bool, mut action: ACTION, dt: f32) -> Option<Self>
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

type Duration = f32;

#[derive(Debug, Clone, Copy, Default)]
pub enum Message2<I>
{
    Alive(I),
    Dying(I, Duration),
    
    #[default]
    Dead
}

impl <I> Message2<I>
{
    pub fn create_dying(i: I, d: Duration) -> Self
    {
        Self::Dying(i, d)
    }

    pub fn set_dying(self, i: I, d: Duration) -> Self
    {
        Self::create_dying(i, d)
    }

    // Gets the associated value contained in the message, if it exists
    pub fn get_value(&self) -> Option<&I>
    {
        use Message2::*;
        match self
        {
            Alive(i) | Dying(i, _) =>
            {
                Some(i)
            },
            Dead => None,
        }
    }

    
}

impl<I> crate::GameObject<Message2<I>> for Message2<I>
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        
        Ok(())
    }
}