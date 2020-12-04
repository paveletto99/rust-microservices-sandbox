#[derive(Default)]
pub struct Invoice {
    code: String,
    customerId: String
}

impl Invoice {

    pub fn setCode(&mut self, code: String) {
        self.code = code
    }
    
    pub fn getCode(&self) -> &String {
        &self.code
    }

    pub fn setCustomerId(&mut self, customerId: String) {
        self.customerId = customerId
    }

    pub fn getCustomerId(self) -> String {
        self.customerId
    }
}