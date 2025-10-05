use std::fmt;

pub struct Epee {
        pub nom: String,
        pub puissance: i16,
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
