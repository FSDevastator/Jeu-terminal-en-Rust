use std::fmt;

#[derive(Clone)]
pub struct Epee {
        nom: String,
        puissance: i16,
    }

impl fmt::Display for Epee {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Vous avez trouvé l'épée {} avec puissance {}! ", self.get_name(),self.get_puissance())
        }
    }

    impl Default for Epee {
        fn default() -> Self {
            Epee {
                nom: "Potato Peeler".to_string(),
                puissance: 10
            }
        }
    }

    impl Epee {
        pub fn new(nom: &str, puissance: i16 ) -> Self {
            Self {nom: nom.to_string(),puissance}
        }
    }

    impl Epee {
        pub fn set_name (&mut self, nouv:&str) {
            self.nom = nouv.to_string();
        }
    }

    impl Epee {
        pub fn get_name (&self) -> &String {
            &self.nom
        }
    }

    impl Epee {
        pub fn set_puissance(&mut self, pwr: i16) {
            self.puissance = pwr;
        }
    }

    impl Epee {
        pub fn get_puissance (&self) -> i16{
            self.puissance
        }
    }