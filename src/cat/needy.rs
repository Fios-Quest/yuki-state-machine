use crate::bad_rand::rand_index;
use crate::cat::eepy::EepyCat;
use crate::MischievousCat;

pub struct NeedyCat {
    name: String,
    noises: &'static [&'static str],
}

impl NeedyCat {
    pub(super) fn new(name: String) -> NeedyCat {
        NeedyCat {
            name,
            noises: &["Maow"],
        }
    }

    pub fn make_noise(&self) -> &'static str {
        self.noises[rand_index(self.noises.len())]
    }

    pub fn gentle_cuddle(self) -> EepyCat {
        EepyCat::new(self.name)
    }

    pub fn over_cuddle(self) -> MischievousCat {
        MischievousCat::new(self.name)
    }

    pub fn describe(&self) -> String {
        let name = &self.name;
        let noise = self.make_noise();
        format!("{name} is trying to get your attention with a {noise} noise")
    }
}
