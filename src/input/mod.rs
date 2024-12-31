use std::collections::{HashMap, HashSet};
use ggez::GameResult;

use ggez::input::keyboard::KeyMods;

#[derive(Debug, Clone, Default)]
pub struct KeyInputState
{
    pub pressed_buttons:    HashSet<Button>,
    // pub pressed_mods:       HashSet<Button>, // contains only mods
    pub pressed_mods:       KeyMods,

    // blocked_buttons:        HashSet<Button>,
    // blocked_mods:           HashSet<Button>, // contains only mods
    blocked_mods:           KeyMods,

    alt_only_pending:       bool,
    shift_only_pending:     bool,
    control_only_pending:   bool,
    logo_only_pending:      bool,

    key_combos:             ComboToAction,
    pub held_actions:       HashSet<ActionCode>,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Mod
// {
//     LShift,
//     RShift,
//     LAlt,
//     RAlt,
//     RControl,
//     LControl,
// }

/// Block keys when pressed, or a combo is pressed. 
/// For instance, if I have a combo ALT + D, if I let go of ALT, nothing 
/// should happen that is bound to D until ALL the keys in the combo are released
/// 
/// This will also block if I am holding D and starting holding ALT. But anything 
/// bound to ALT will work, or another combo like ALT + C.
/// 
/// If a keycombo results in an action, then we add those
/// keys to some set, and no other action using those keys can be created until
/// all those keys have been released. 
/// 
/// 

impl crate::FixedUpdate<Self> for KeyInputState
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        // read set of pressed keys and detect if key combo, block if necessary
        


        

        for kc in self.key_combos.keys()
        {
            // check if there are any held actions whose
            // combos are not being held right now
            for &ac in &self.key_combos[kc]
            {
                if self.held_actions.contains(&ac)
                {
                    if !self.pressed_buttons.contains(&kc.1) && !self.blocked_mods.contains(kc.0)
                    {
                        self.held_actions.remove(&ac);
                    }
                }
            }

            let is_blocked_combo = 
            {
                // let button_not_blocked: bool = !self.blocked_buttons.contains(b);
                let button_not_blocked: bool = !self.pressed_buttons.contains(&kc.1);
                
                // if the currently-blocked modifiers contain ANY of this combo's mods, 
                // this combo is blocked
                // let mod_not_blocked: bool = (self.blocked_mods & *m) == KeyMods::NONE;
                let mod_not_blocked: bool = !self.blocked_mods.contains(kc.0);

                button_not_blocked & mod_not_blocked
            };

            if is_blocked_combo { continue }

            // current mods held are a superset of this combo's mods
            // let is_superset = (self.pressed_mods | *m) == self.modifiers;
            let is_superset = self.pressed_mods.contains(kc.0);

            // check that pressed buttons contains button
            if self.pressed_buttons.contains(&kc.1) && is_superset 
            {
                // we now know that this set of pressed keys contains this key combo.
                // we now block the keys and add its actions

                for &ac in &self.key_combos[&kc]
                {
                    self.held_actions.insert(ac);
                }

                // self.blocked_buttons.insert(*b);
                self.blocked_mods.insert(kc.0);
            }
        }

        Ok(())    
    }
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

        use ggez::input::keyboard::{KeyCode::*, KeyMods};
        match key
        {
            LAlt | RAlt         => { self.pressed_mods.insert(KeyMods::ALT); },
            LShift | RShift     => { self.pressed_mods.insert(KeyMods::SHIFT); },
            LControl | RControl => { self.pressed_mods.insert(KeyMods::CTRL); },
            _                   => (),
        };
        self.pressed_buttons.insert(key.into());
        
        Ok(())
    }

    fn key_up_event(
        &mut self, 
        _context: &mut ggez::Context, 
        input: ggez::input::keyboard::KeyInput
    ) -> GameResult 
    {
        let key = input.keycode.expect("Keycode in key-up will always be something");

        use ggez::input::keyboard::{KeyCode::*, KeyMods};
        match key
        {
            LAlt | RAlt         => { self.pressed_mods.remove(KeyMods::ALT); self.blocked_mods.remove(KeyMods::ALT); },
            LShift | RShift     => { self.pressed_mods.remove(KeyMods::SHIFT); self.blocked_mods.remove(KeyMods::SHIFT); },
            LControl | RControl => { self.pressed_mods.remove(KeyMods::CTRL); self.blocked_mods.remove(KeyMods::CTRL); },
            _                   => { self.pressed_buttons.remove(&key.into()); },
        };
        self.pressed_buttons.remove(&key.into());
        
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
        self.pressed_buttons.insert(button.into());
        
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
        self.pressed_buttons.remove(&button.into());
        
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
pub enum Button
{
    Mouse(ggez::input::mouse::MouseButton),
    Keyboard(ggez::input::keyboard::KeyCode),
    None,
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
pub struct KeyCombo(KeyMods, Button);

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
                ((KeyMods::NONE, KeyCode::D).into(), vec![ActionCode::CameraRight]),

                ((KeyMods::NONE, MouseButton::Left).into(), vec![ActionCode::Click]),

                ((KeyMods::ALT, KeyCode::LAlt).into(), vec![ActionCode::FlipDebugHitboxes]),
            ]
        );

        Self(map)    
    }
}