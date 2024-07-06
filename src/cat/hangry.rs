use crate::bad_rand::rand_index;
use crate::cat::eepy::EepyCat;

pub struct HangryCat {
    name: String,
    noises: &'static [&'static str],
    violence: &'static [&'static str],
}

impl HangryCat {
    pub(super) fn new(name: String) -> HangryCat {
        HangryCat {
            name,
            noises: &["MAAAAAAAAUUUUUUUW"],
            violence: &["Bat", "Nom", "Headbutt"],
        }
    }

    pub fn make_noise(&self) -> &'static str {
        self.noises[rand_index(self.noises.len())]
    }

    pub fn choose_violence(&self) -> &'static str {
        self.violence[rand_index(self.violence.len())]
    }

    pub fn feed(self) -> EepyCat {
        EepyCat::new(self.name)
    }

    pub fn describe(&self) -> String {
        let name = &self.name;
        let noise = self.make_noise();
        let violence = self.choose_violence();
        format!("{name} is letting you know you forgot to feed them by making a {noise} noise. When that doesn't work they {violence} you")
    }
}
