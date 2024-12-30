use std::collections::{HashMap, HashSet};
use ggez::{glam::Vec2, GameResult};

#[derive(Debug, Clone, Default)]
pub struct KeyInputState
{
    pressed_keys:           HashSet<ggez::input::keyboard::KeyCode>,
    pressed_mouse:          HashSet<ggez::input::mouse::MouseButton>,
    modifiers:              ggez::input::keyboard::KeyMods,
    alt_only_pending:       bool,
    shift_only_pending:     bool,
    control_only_pending:   bool,
    logo_only_pending:      bool,

    // mouse_position_curr:    Option<Vec2>,
    // mouse_position_prev:    Option<Vec2>,

    key_combos:             ComboToAction,
    pub held_actions:       HashSet<ActionCode>
}

impl ggez::event::EventHandler for KeyInputState
{
    fn update(&mut self, _: &mut ggez::Context) -> GameResult 
    {
        panic!("KeyInputState::update should never be called!")
    }

    fn draw(&mut self, _: &mut ggez::Context) -> GameResult 
    {
        panic!("KeyInputState::draw should never be called!")
    }

    fn key_down_event(
        &mut self,
        _context: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult
    {
        let key = input.keycode.expect("Keycode in key-down will always be something");

        self.pressed_keys.insert(key);
        self.modifiers = input.mods;

        use ggez::input::keyboard::KeyCode::*;
        match key
        {
            LAlt | RAlt =>          self.alt_only_pending       = true,
            RShift | LShift =>      self.shift_only_pending     = true,
            RControl | LControl =>  self.control_only_pending   = true,
            _ => (),
        };

        let key_combo: KeyCombo = (self.modifiers, key).into();
        if let Some(v) = self.key_combos.get(&key_combo)
        {
            for &ac in v
            {
                self.held_actions.insert(ac);
                self.alt_only_pending = false;
                self.shift_only_pending = false;
                self.control_only_pending = false;
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
        let key = input.keycode.expect("Keycode in key-up will always be something");

        self.pressed_keys.remove(&key);
        self.modifiers = input.mods;

        use ggez::input::keyboard::KeyCode::*;
        match key
        {
            LAlt | RAlt if self.alt_only_pending =>
            {
                let key_combo: KeyCombo = (self.modifiers, LAlt).into();
                if let Some(v) = self.key_combos.get(&key_combo)
                {
                    for &ac in v
                    {
                        self.held_actions.insert(ac);   
                        self.alt_only_pending = false;
                    }
                }
            },
            // LShift | RShift if self.shift_only_pending =>
            // {

            // },
            // LControl | RControl if self.control_only_pending =>
            // {

            // },
            _ => (),
        }
        
        Ok(())    
    }

    fn mouse_button_down_event(
        &mut self,
        _context: &mut ggez::Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult
    {
        self.pressed_mouse.insert(button);

        let key_combo: KeyCombo = (self.modifiers, button).into();
        if let Some(v) = self.key_combos.get(&key_combo)
        {
            for &ac in v
            {
                self.held_actions.insert(ac);
                self.alt_only_pending = false;
                self.shift_only_pending = false;
                self.control_only_pending = false;
            }
        }
        
        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _context: &mut ggez::Context,
            button: ggez::event::MouseButton,
            _x: f32,
            _y: f32,
    ) -> GameResult
    {
        self.pressed_mouse.remove(&button);
        
        Ok(())
    }
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
    FlipDebugHitboxes,
    Click
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Button
{
    Mouse(ggez::input::mouse::MouseButton),
    Keyboard(ggez::input::keyboard::KeyCode),
}

impl From<ggez::input::mouse::MouseButton> for Button
{
    fn from(value: ggez::input::mouse::MouseButton) -> Self 
    {
        Self::Mouse(value)
    }
}

impl From<ggez::input::keyboard::KeyCode> for Button
{
    fn from(value: ggez::input::keyboard::KeyCode) -> Self 
    {
        Self::Keyboard(value)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyCombo(ggez::input::keyboard::KeyMods, Button);

impl<B> From<(ggez::input::keyboard::KeyMods, B)> for KeyCombo
where
    B: Into<Button>
{
    fn from(value: (ggez::input::keyboard::KeyMods, B)) -> Self 
    {
        Self(value.0, value.1.into())
    }
}

#[derive(Clone, Debug)]
pub struct ComboToAction(HashMap<KeyCombo, Vec<ActionCode>>);

impl std::ops::Deref for ComboToAction
{
    type Target = HashMap<KeyCombo, Vec<ActionCode>>;

    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}

impl ComboToAction
{
    fn resolve_key_combo(&self, kc: &KeyCombo) -> Option<&Vec<ActionCode>>
    {
        self.get(kc)
    }
}

impl Default for ComboToAction
{
    fn default() -> Self 
    {
        use ggez::input::{mouse::MouseButton, keyboard::{KeyCode, KeyMods}};
        let map = HashMap::from(
            [
                ((KeyMods::ALT, KeyCode::D).into(), vec![ActionCode::Shoot]),
                ((KeyMods::NONE, KeyCode::Left).into(), vec![ActionCode::TurnLeft]),
                ((KeyMods::NONE, KeyCode::Right).into(), vec![ActionCode::TurnRight]),

                // ((KeyMods::NONE, KeyCode::D).into(), vec![ActionCode::Shoot]),

                ((KeyMods::NONE, KeyCode::W).into(), vec![ActionCode::CameraUp]),
                ((KeyMods::NONE, KeyCode::S).into(), vec![ActionCode::CameraDown]),
                ((KeyMods::NONE, KeyCode::A).into(), vec![ActionCode::CameraLeft]),
                ((KeyMods::NONE, KeyCode::D).into(), vec![ActionCode::CameraRight, ActionCode::Shoot]),

                ((KeyMods::NONE, MouseButton::Left).into(), vec![ActionCode::Click]),

                ((KeyMods::ALT, KeyCode::LAlt).into(), vec![ActionCode::FlipDebugHitboxes]),
            ]
        );

        Self(map)    
    }
}