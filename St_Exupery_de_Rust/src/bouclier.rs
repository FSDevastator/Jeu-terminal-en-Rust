use std::fmt;
#[derive(Clone)]
pub struct Bouclier {
    nom: String,
    def: i16,
}

impl fmt::Display for Bouclier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Vous avez trouvé le bouclier {} avec puissance {}! ", self.get_name(),self.get_def())
        }
    }

    impl Default for Bouclier {
    fn default() -> Self{
        Bouclier {
            nom: "Tin foil".to_string(),
            def: 1
        }
    }
}

impl Bouclier {
    pub fn new(nom: &str, defense: i16) -> Self {
        Self {
            nom: nom.to_string(),
            def: defense
        }
    }
}

impl Bouclier {
    pub fn get_name(&self) -> &String {
        &self.nom
    }
}

impl Bouclier {
    pub fn set_name(&mut self, nouv: &str) {
        self.nom = nouv.to_string();
    }
}

impl Bouclier {
    pub fn get_def(&self) -> i16 {
        self.def
    }
}

impl Bouclier {
    pub fn set_def(&mut self,defense:i16) {
        self.def=defense;
    }
}