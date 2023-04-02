pub struct Input {
    input: String,
}

impl Input {
    pub fn new() -> Input {
        Input {
            input: String::from("")
        }
    }

    pub fn add(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn delete(&mut self) {
        self.input.pop();
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }
}

fn main() {
    println!("ehllow");
}
