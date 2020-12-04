#[derive(Default)]
pub struct Order {
    code: String
}

impl Order {

    pub fn setCode(&mut self, code: String) {
        self.code = code
    }
    
    pub fn getCode(&self) -> &String {
        &self.code
    }
}