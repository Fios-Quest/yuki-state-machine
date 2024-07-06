use crate::bad_rand::rand_index;
use crate::cat::hangry::HangryCat;
use crate::cat::scared::ScaredCat;

pub struct MischievousCat {
    name: String,
    noises: &'static [&'static str],
    activities: &'static [&'static str],
}

impl MischievousCat {
    pub fn new(name: String) -> Self {
        MischievousCat {
            name,
            noises: &["Mow", "Prrrr", "Reow"],
            activities: &[
                "Jumping on things",
                "Batting clothes in wardrobe",
                "Eating stuff on kitchen floor",
            ],
        }
    }

    pub fn make_noise(&self) -> &'static str {
        self.noises[rand_index(self.noises.len())]
    }

    pub fn choose_action(&self) -> &'static str {
        self.activities[rand_index(self.activities.len())]
    }

    pub fn forget_to_feed(self) -> HangryCat {
        HangryCat::new(self.name)
    }

    pub fn scare(self) -> ScaredCat {
        ScaredCat::new(self.name)
    }

    pub fn describe(&self) -> String {
        let name = &self.name;
        let noise = self.make_noise();
        let action = self.choose_action();
        format!("{name} is {action} while making a {noise} noise")
    }
}
