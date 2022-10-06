use std::{collections::{HashMap, VecDeque}, hash::Hash};

use winit::event::{VirtualKeyCode, ElementState, MouseButton};

/**
This is just to keep track of the state of all the keys.
The actual code ran at any givin state should be handled
in your event handler.
 */
pub struct Peripheral {
    // Some of these are pub, they don't hold anything
    // of value.
    
    // Instead of holding the states of all the keys,
    // should these just be Vectors of keys pressed and depressed
    // that act as queues.
    // Then the tick method just eats the queues.
    keys: HashMap<VirtualKeyCode, ElementState>,
    mouse: HashMap<MouseButton, ElementState>,
    
    pub last_key: Option<VirtualKeyCode>,
    pub last_mouse: Option<MouseButton>,
    
    pointer: [f64;2],
    pub pointer_moved: bool,
}

// Any time an update function get's called, it will be because
// of an event, which implies change, thus, it is pointless
// to have these return boolean. They will always return true.
impl Peripheral {
    /** Don't call except for in State's constructor */
    pub fn new() -> Self {
        Self { 
            // State of all keys and mouse buttons
            keys: HashMap::new(),
            mouse: HashMap::new(),
            // queues to let the user know change happened.
            last_key: None,
            last_mouse: None,
            // Pointer stuff that doesn't fit the data
            // structure of everything else.
            pointer: [0.0,0.0],
            pointer_moved: true,
            // need mouse wheel (maybe)
        }
    }
    
    //======================
    //         Set
    //======================
    /*
        Function that changes the key states as they come in.
        If a new key is found register it.
        I'm not hard-coding that many keys.
    */
    pub fn update_key(&mut self, key: &VirtualKeyCode, state: &ElementState) -> bool {
        self.last_key = Some(*key);
        Self::update(&mut self.keys, key, state)
    }
    
    pub fn update_mouse(&mut self, button: &MouseButton, state: &ElementState) -> bool {
        self.last_mouse = Some(*button);
        Self::update(&mut self.mouse, button, state)
    }
    
    pub fn update_pointer(&mut self, location: [f64; 2]) {
        // Gets called by the mouse movement event,
        // thus, we don't have to check if it changed
        // to accurately set this value, we implicitly know it
        // did.
        self.pointer_moved = true;   
        self.pointer = location;
    }
    
    //======================
    //        Get
    //======================
    /// Returns `true` if `key` is pressed. Otherwise `false`.
    pub fn get_key(&self, key: &VirtualKeyCode) -> bool {
        match self.keys.get(key) {
            Some(value) => match value {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
            None => false,
        }
    }

    pub fn get_pointer(&self) -> [f64;2] {
        self.pointer
    }

    /// Returns `true` if `button` is pressed, `false` if not.
    pub fn get_mouse(&self, button: &MouseButton) -> bool {
        match self.mouse.get(&button) {
            Some(state) => match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
            // It would exist if it had been pressed, thus it isn't
            None => false,
        }
    }

    /// Returns boolean value based on if the state of the givin inputs is 
    /// different then the previous state.
    /// 
    /// `false` denotes no change.
    /// `true` means change.
    fn update<K: Hash + Eq + Copy>(hashmap: &mut HashMap<K, ElementState>, input: &K, state: &ElementState) -> bool {
        
        // (outline for function)
        // T can be either VirtualKeyCode or MouseButton
        // We need to diverge to different hashmaps depending on the type
        // But all the other code stays the same
        
        match hashmap.insert(*input, *state) {
            Some(old_state) => {
                // change in state?
                if old_state != *state { return true; } false
            },
            // We can safely assume that this is an update of state
            // as the key wasn't registered before.
            None => true,
        }
    }

}

