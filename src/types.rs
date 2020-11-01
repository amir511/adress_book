/// AddressRecord holds the data of one record
pub struct AddressRecord {
    pub name: String,
    pub mobile: String, 
    pub address: String, 
}

impl AddressRecord{
    /// new creates a new AdressRecord and returns it
    pub fn new(name:String, mobile:String, address:String) -> AddressRecord {
            AddressRecord{
                name: name,
                mobile: mobile, 
                address: address
            }
    }
}

