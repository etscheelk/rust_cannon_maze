use serde::{Deserialize, Serialize};

type Duration = f32;

#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Message<I = ()>
{
    Active(I),
    ActiveTicking(I, Duration),
    
    #[default]
    Inactive,
}

impl<I> Message<I>
where
    I: std::fmt::Debug
{
    pub fn create_active(value: I) -> Self
    {
        Message::Active(value)
    }

    pub fn create_active_ticking(value: I, duration: Duration) -> Self
    {
        Message::ActiveTicking(value, duration)
    }

    pub fn create_inactive() -> Self
    {
        Message::Inactive
    }

    pub fn /*(I am here haha)*/set_active_ticking(&mut self, value: I, duration: Duration)
    {
        *self = Message::ActiveTicking(value, duration);
    }


    /// Gets the associated value contained in the message, if it exists.
    pub fn is_active(&self) -> Option<&I>
    {
        use Message::*;
        match self
        {
            Active(i) | ActiveTicking(i, _) =>
            {
                Some(i)
            },
            _ => None,
        }
    }

    pub fn tick(&mut self, dt: f32)
    {
        println!("message: {:?}", *self);

        use Message::*;

        if let ActiveTicking(_, time) = self
        {
            *time -= dt;

            if *time <= 0.0
            {
                *self = Inactive;
            }
        }
    }
}