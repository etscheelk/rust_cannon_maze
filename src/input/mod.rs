use std::collections::{HashMap, HashSet};
use ggez::{glam::Vec2, GameResult};

#[derive(Default, Debug)]
pub struct InputState
{
    // Represent a map of keyboard inputs to
    // game inputs
    pub input_map: InputMap,

    // the actual statuses that objects should read
    pub left_click: Option<Vec2>,
    pub cannon_rotate: Option<ActionCode>,
    pub set: HashSet<ActionCode>,
    pub mods: ggez::input::keyboard::KeyMods,
    pub camera_movements: HashSet<ActionCode>,

    pub shoot: bool,
}

#[non_exhaustive]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum ActionCode
{
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    TurnLeft,
    TurnRight,
    Shoot,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum ModCode
{
    Shift,
    Left, 
    Alt,
    Logo,
    Control,
}

#[derive(Debug)]
pub struct InputMap
{
    key_codes: HashMap<ggez::input::keyboard::KeyCode, Vec<ActionCode>>,
    mod_codes: HashMap<ModCode, Vec<ActionCode>>,
    mouse_codes: HashMap<ggez::input::mouse::MouseButton, Vec<ActionCode>>,
}



impl Default for InputMap
{
    fn default() -> Self 
    {
        let key_codes = 
        {
            use ggez::input::keyboard::KeyCode;
            HashMap::from(
                [
                    (KeyCode::W,        vec![ActionCode::CameraUp]),
                    (KeyCode::S,        vec![ActionCode::CameraDown]),
                    (KeyCode::A,        vec![ActionCode::CameraLeft]),
                    (KeyCode::D,        vec![ActionCode::CameraRight]),
                    (KeyCode::Left,     vec![ActionCode::TurnLeft]),
                    (KeyCode::Right,    vec![ActionCode::TurnRight]),
                    (KeyCode::Space,    vec![ActionCode::Shoot])
                ]
            )
        };
        
        let mod_codes = HashMap::new();

        let mouse_codes = 
        {
            use ggez::input::mouse::MouseButton;

            HashMap::from(
                [
                    (MouseButton::Left, vec![ActionCode::Shoot])
                ]
            )
        };

        InputMap
        {
            key_codes,
            mod_codes,
            mouse_codes,
        }
    }
}

impl ggez::event::EventHandler for InputState
{
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        panic!("InputState::update should never be called!")
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        panic!("InputState::draw should never be called!")
    }

    fn key_down_event(
        &mut self,
        _context: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult
    {
        let map = &self.input_map;

        if let Some(kc) = input.keycode
        {
            if let Some(s) = map.key_codes.get(&kc)
            {
                for &ac in s
                {
                    use ActionCode::*;
                    match ac 
                    {
                        CameraUp | CameraDown | CameraLeft | CameraRight => 
                        {
                            self.camera_movements.insert(ac);
                        },
                        Shoot =>
                        {
                            self.shoot = true;
                        },
                        TurnLeft | TurnRight =>
                        {
                            self.cannon_rotate = Some(ac);
                        },
                    };
                }
            }
        }

        Ok(())
    }

    fn key_up_event(
        &mut self, 
        _context: &mut ggez::Context, 
        input: ggez::input::keyboard::KeyInput
    ) -> GameResult
    {
        let map = &self.input_map;

        if let Some(kc) = input.keycode
        {
            if let Some(s) = map.key_codes.get(&kc)
            {
                for &ac in s
                {
                    use ActionCode::*;
                    match ac
                    {
                        CameraUp | CameraDown | CameraLeft | CameraRight =>
                        {
                            self.camera_movements.remove(&ac);
                        },
                        Shoot =>
                        {
                            self.shoot = false;
                        },
                        TurnLeft | TurnRight =>
                        {
                            self.cannon_rotate = None;
                        },
                    };
                }
            }
        }
        
        Ok(())
    }
}