use crate::MischievousCat;

pub struct EepyCat {
    name: String,
}

impl EepyCat {
    pub(super) fn new(name: String) -> EepyCat {
        EepyCat { name }
    }

    pub fn sleep(self) -> MischievousCat {
        MischievousCat::new(self.name)
    }

    pub fn describe(&self) -> String {
        let name = &self.name;
        format!("{name} is sleeping")
    }
}
