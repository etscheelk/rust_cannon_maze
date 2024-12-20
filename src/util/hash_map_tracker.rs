use std::{collections::HashMap, ops::{Deref, DerefMut}};

// local imports

/// Acts as a tracker for a variety of objects with ID: u16, with a maximum number of elements.
/// 
/// The tracker is the owner of any object placed in itself.
/// The tracker has a circular ID ensuring any object it owns has a unique ID. 
/// 
/// User can push items that the tracker places in itself as an owner, 
/// giving them a circular unique ID of the element.
#[derive(Debug, Clone)]
pub struct HashMapTracker<I, const MAX: u16 = 1024>
{
    cur_index: u16,
    tracker: HashMap<u16, I>
}

impl<I, const MAX: u16> Deref for HashMapTracker<I, MAX>
{
    type Target = HashMap<u16, I>;

    fn deref(&self) -> &Self::Target {
        &self.tracker
    }
}

impl<I, const MAX: u16> DerefMut for HashMapTracker<I, MAX>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tracker
    }
}

pub enum Status
{
    Failure,
    Success,
}

/// Any object you want to place in a HashMapTracker must implement this trait
pub trait ForTracker: WithIndex {}

/// A trait imposing a requirement that an object has a consuming function which sets a particular ID on the object
pub trait WithIndex
{
    fn with_index(self, index: u16) -> Self;
}

impl<I, const MAX: u16> HashMapTracker<I, MAX>
where
    I: ForTracker
{
    pub fn new() -> Self
    {
        Self
        {
            cur_index: 0,
            tracker: HashMap::new(),
        }
    }

    /// Add an item `I` into the tracker, consuming it.
    /// The tracker is the ultimate owner of the item being tracked.
    /// 
    /// TODO: Could a circular ID wrap around to be a value of an object already in the map?
    /// YES. FIXME
    pub fn push(&mut self, i: I) -> Status
    {
        if self.tracker.len() + 1 > MAX as usize
        {
            Status::Failure
        }
        else
        {
            let ind = self.cur_index;
            self.tracker.insert(ind, i.with_index(ind));
            self.cur_index = (ind + 1) % MAX;

            Status::Success
        }
    }

    pub fn get_tracker(&self) -> &<Self as Deref>::Target
    {
        & *self
    }

    pub fn get_tracker_mut(&mut self) -> &mut<Self as Deref>::Target
    {
        &mut *self
    }

    // Should basically always return Success. 
    pub fn delete(&mut self, index: u16) -> Status
    {   
        match self.tracker.remove(&index)
        {
            Some(_) => Status::Success,
            None    => Status::Failure,
        }
    }
}