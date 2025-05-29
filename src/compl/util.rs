pub struct NameGenerator {
    counter: usize,
}

impl NameGenerator {
    pub fn new() -> Self {
        NameGenerator { counter: 0 }
    }

    pub fn next(&mut self, prefix: &str) -> String {
        let name = format!("{}{}", prefix, self.counter);
        self.counter += 1;
        name
    }
}
