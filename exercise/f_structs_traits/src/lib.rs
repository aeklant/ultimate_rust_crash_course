pub trait Bite {
    fn bite(&mut self);
}


#[derive(Debug)] // This enables using the debugging format string "{:?}"
pub struct Grapes {
    remaining: i32,
}

impl Grapes {
    pub fn new(remaining: i32) -> Self {
        Self{ remaining }
    }
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.remaining -= 1;
    }
}


#[derive(Debug)] // This enables using the debugging format string "{:?}"
pub struct Carrot {
    percent_left: f32,
}

impl Carrot {
    pub fn new(percent_left: f32) -> Self {
        Carrot{ percent_left }
    }
}

impl Bite for Carrot {
    fn bite(&mut self) {
        self.percent_left *= 0.8;
    }
}
