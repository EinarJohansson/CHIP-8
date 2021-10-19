use std::collections::HashMap;
use minifb::Key;

pub struct Keyboard {
    keymap:         HashMap<Key, u8>,
    pub pressed:    Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        // Create a hashmap to map minifb keys to the chip-8 keyboard
        let keymap: HashMap<Key, u8> = [
            (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3), (Key::Key4, 0xC),
            (Key::Q, 0x4), (Key::W, 0x5), (Key::E, 0x6), (Key::R, 0xD),
            (Key::A, 0x7), (Key::S, 0x8), (Key::D, 0x9), (Key::F, 0xE),
            (Key::Z, 0xA), (Key::X, 0x0), (Key::C, 0xB), (Key::V, 0xF)
        ]
        .iter().
        cloned().
        collect();
        
        Self {
            keymap,
            pressed: None
        }
    }

    /// Set the current pressed key
    pub fn set_key(&mut self, key: &Key) {
        if self.keymap.contains_key(key) {
            self.pressed = Some(self.get_keyval(key));
        }
    } 

    /// Map the key's value to a u8 according to our hashmap
    fn get_keyval(&self, key: &Key) -> u8 {
        self.keymap.get(key).unwrap().clone()
    }

    /// Check if a key is pressed or not
    pub fn is_pressed(&self, keyval: u8) -> bool {
        // self.pressed.contains(keyval)
        if let Some(pressed_key) = self.pressed {
            keyval == pressed_key
        }
        else {
            false
        }
    }

    /// Unset the current pressed key 
    pub fn clear(&mut self) {
        self.pressed = None;
    }
}