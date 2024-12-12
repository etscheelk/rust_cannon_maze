use serde::{Deserialize, Serialize};

type Duration = f32;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Message<I>
{
    Active(I),
    ActiveTicking(I, Duration),
    
    #[default]
    Inactive
}

impl <I> Message<I>
{
    pub fn create_active_ticking(value: I, duration: Duration) -> Self
    {
        Self::ActiveTicking(value, duration)
    }

    pub fn set_active_ticking(self, value: I, duration: Duration) -> Self
    {
        Self::create_active_ticking(value, duration)
    }

    // Gets the associated value contained in the message, if it exists
    pub fn get_value(&self) -> Option<&I>
    {
        use Message::*;
        match self
        {
            Active(i) | ActiveTicking(i, _) =>
            {
                Some(i)
            },
            Inactive => None,
        }
    }

    pub fn tick(&mut self, dt: f32)
    {
        use Message::*;

        match *self
        {
            ActiveTicking(_, ref mut time) =>
            {
                *time -= dt;

                if *time < 0.0
                {
                    *self = Inactive
                }
            },
            _ => (),
        };
    }
}

// impl<I> crate::GameObject<Message2<I>> for Message2<I>
// {
//     fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
//     {
        
//         Ok(())
//     }
// }