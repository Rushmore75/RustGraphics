struct Box2d {
    location: [[f64; 2]; 2],
    // this functions like a lambda
    click: fn(),
    // https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter
    // good explanation
    hover: fn()
}

impl Box2d {
    fn new(top_left: [f64; 2], bottom_right: [f64; 2], click: fn(), hover: fn()) -> Self{
                
        Self { 
            location: [top_left, bottom_right],
            click,
            hover,
        }
    }

    fn is_inside(&self, location: [f64; 2]) -> bool {
        let x = location[0];
        let y = location[1];

        if
        self.location[0][0] < x && x < self.location[1][0] // inside x
        &&
        self.location[0][1] < y && y < self.location[1][1] // inside y
        { true } else { false }
    }

    fn click(&self) {
        // TODO does this work?
        self.click;
    }

    fn hover(&self) {
        self.hover;
    }

}


pub struct Boxes {
    buttons: Vec<Box2d>
}

impl Boxes {

    pub fn new() -> Self {
        Self { 
            buttons: Vec::new()    
        }
    }

    pub fn add_button(&mut self, top_left: [f64; 2], bottom_right: [f64; 2], click: fn(), hover: fn()) -> &mut Self {
        self.buttons.push(Box2d::new(top_left, bottom_right, click, hover));
        self
    }

    /// Intakes a `mouse_coords` and a `click` parameter and checks if the mouse
    /// is inside of a button's bounding box. If it *is*, the __hover function__
    /// for the appropriate button is called. If click is `true`, the __click function__
    /// is called.
    pub fn process_mouse(&self, mouse_coords: [f64;2], click: bool) {
        
        // some big-brain math could probably make it so instead of looking thru 
        // all the buttons you instantly know where each button is.
        for button in &self.buttons {
            if button.is_inside(mouse_coords) {
                button.hover();
                if click { button.click(); }
            }
        }
    }
}
