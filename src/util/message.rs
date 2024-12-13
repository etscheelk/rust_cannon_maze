use serde::{Deserialize, Serialize};

type Duration = f32;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Message<I = ()>
{
    ActiveWithValue(I),
    ActiveTicking(I, Duration),
    
    #[default]
    Inactive,
}

impl<I> Message<I>
{
    pub fn create_active(value: I) -> Self
    {
        Message::ActiveWithValue(value)
    }

    pub fn create_active_ticking(value: I, duration: Duration) -> Self
    {
        Self::ActiveTicking(value, duration)
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
            ActiveWithValue(i) | ActiveTicking(i, _) =>
            {
                Some(i)
            },
            _ => None,
        }
    }

    pub fn tick(&mut self, dt: f32)
    {
        use Message::*;

        if let ActiveTicking(_, time) = self
        {
            *time -= dt;

            if *time < 0.0
            {
                *self = Inactive;
            }
        }
    }
}