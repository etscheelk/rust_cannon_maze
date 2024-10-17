use std::collections::HashMap;

// local imports

#[derive(Debug, Clone)]
pub struct HashMapTracker<I, const MAX: u16 = 1024>(pub u16, pub HashMap<u16, I>);

pub enum Status
{
    Failure,
    Success,
}

impl<I, const MAX: u16> HashMapTracker<I, MAX>
{
    pub fn new() -> Self
    {
        Self(0, HashMap::new())
    }

    pub fn push(&mut self, i: I) -> Status
    {
        if self.1.len() + 1 > MAX as usize
        {
            Status::Failure
        }
        else
        {
            self.1.insert(self.0, i);
            self.0 = (self.0 + 1) % MAX;

            Status::Success
        }
    }

    // #[deprecated]
    // pub fn push_iter(&mut self, i: impl IntoIterator<Item = I>) -> u16
    // {
    //     let i = i.into_iter();

    //     let v = i.fold(0_u16,
    //     |acc, i|
    //     {
    //         acc + if let Status::Failure = self.push(i)
    //         {
    //             1
    //         } else { 0 }
    //     });

    //     // let v = i.map(
    //     // |i: I|
    //     // {
    //     //     self.push(i)
    //     // }).collect();

    //     return v;
    // }

    // Should basically always return Success. 
    pub fn delete(&mut self, index: u16) -> Status
    {   
        match self.1.remove(&index)
        {
            Some(_) => Status::Success,
            None    => Status::Failure,
        }
    }
}