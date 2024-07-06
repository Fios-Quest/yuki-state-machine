use crate::bad_rand::rand_index;
use crate::cat::needy::NeedyCat;

pub struct ScaredCat {
    name: String,
    hiding_spot: &'static [&'static str],
    violence: &'static [&'static str],
}

impl ScaredCat {
    pub(super) fn new(name: String) -> ScaredCat {
        ScaredCat {
            name,
            hiding_spot: &["Under the bed", "In the wardrobe", "Under the sofa"],
            violence: &["Scratch", "Bite"],
        }
    }

    pub fn find(&self) -> &'static str {
        self.hiding_spot[rand_index(self.hiding_spot.len())]
    }

    pub fn choose_violence(&self) -> &'static str {
        self.violence[rand_index(self.violence.len())]
    }

    pub fn pass_time(self) -> NeedyCat {
        NeedyCat::new(self.name)
    }

    pub fn describe(&self) -> String {
        let name = &self.name;
        let hiding_spot = self.find();
        let violence = self.choose_violence();
        format!(
            "{name} is hiding {hiding_spot}, when you reach for them, the try to {violence} you"
        )
    }
}
