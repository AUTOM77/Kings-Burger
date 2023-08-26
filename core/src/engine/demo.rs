use std::fmt;

#[derive(Clone, Debug)]
pub struct Task{
    addr: String,
    promo: String
}

impl Task {
    pub fn new(addr:String, promo: String) -> Self {
        Self { addr, promo }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub fn get_promo(&self) -> String {
        self.promo.clone()
    }

    pub fn run(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ref={}", self.promo)
    }
}