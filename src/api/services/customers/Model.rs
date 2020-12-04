pub struct Customer {
    customerId: String,
    customerName: String
}

impl Customer {

    pub fn New() -> Self {
        Self {
            customerId: "".to_string(),
            customerName: "".to_string()
        }
    }
    
    pub fn setCustomerId(&mut self, customerId: String) {
        self.customerId = customerId
    }
    
    pub fn getCustomerId(self) -> String {
        self.customerId
    }
    
    pub fn setCustomerName(&mut self, customerName: String) {
        self.customerName = customerName
    }

    pub fn getCustomerName(&self) -> &String {
        &self.customerName
    }
}