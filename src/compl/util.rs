pub struct NameGenerator {
    prefix: String,
    counter: usize,
}

impl NameGenerator {
    pub fn new(prefix: &str) -> Self {
        NameGenerator {
            prefix: prefix.to_string(),
            counter: 0,
        }
    }

    pub fn next(&mut self) -> String {
        let name = format!("{}_{}", self.prefix, self.counter);
        self.counter += 1;
        name
    }
}
