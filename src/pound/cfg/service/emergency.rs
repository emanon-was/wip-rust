// use pound::cfg::Block;
// use pound::fmt::Decode;
// use pound::fmt::Indent;

#[derive(Debug)]
pub struct Emergency {
    pub address: String,
    pub port: i32,
}

#[derive(Debug)]
pub struct EmergencyBuilder<Address, Port> {
    pub address: Address,
    pub port: Port,
}

impl Emergency {
    pub fn new() -> EmergencyBuilder<(), ()> {
        EmergencyBuilder::new()
    }
}

impl EmergencyBuilder<(), ()> {
    pub fn new() -> Self {
        EmergencyBuilder {
            address: (),
            port: (),
        }
    }
}

impl EmergencyBuilder<String, i32> {
    pub fn build(self) -> Emergency {
        Emergency {
            address: self.address,
            port: self.port,
        }
    }
}

impl<Port> EmergencyBuilder<(), Port> {
    pub fn address<S: Into<String>>(self, address: S) -> EmergencyBuilder<String, Port> {
        EmergencyBuilder {
            address: address.into(),
            port: self.port,
        }
    }
}

impl<Address> EmergencyBuilder<Address, ()> {
    pub fn port(self, port: i32) -> EmergencyBuilder<Address, i32> {
        EmergencyBuilder {
            address: self.address,
            port: port,
        }
    }
}

// impl Decode for Emergency {
//     fn decode(&self) -> String {
//         match &self {
//             Emergency::Address(s) => format!("Address\t{}", s.decode()),
//             Emergency::Port(i) => format!("IgnoreCase\t{}", i.decode()),
//         }
//     }
// }

// impl Decode for Block<Emergency> {
//     fn decode(&self) -> String {
//         let Block(v) = self;
//         return format!("Emergency\n{}\nEnd", v.decode().indent());
//     }
// }
