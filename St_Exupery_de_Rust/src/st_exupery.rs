

use crate::Epee;
use crate::Bouclier;
use crate::Potion;



/// Le struct St_Exupery modélise le joueur et ses actions avec
/// un struct composé.
/// 
/// # Attributs:
/// 
/// + *vie* (i16): représente la vitalité du joueur.
/// + *epee* (struct Epee): modèle Epee pour modéliser l'arme du joueur. 
/// + *bouclier* (struct Bouclier): modèle Bouclier modélise une arme pour 
/// réduire les dommages recus.
/// + *inventaire*
/// 

pub struct StExupery {
     vie: i16,
     pub epee: Epee,
     pub bouclier: Bouclier,
     pub inventaire: Vec<Potion>
}

impl Default for StExupery {
        fn default() -> Self{
            Self{
                vie:100,
                epee: Epee::default(),
                bouclier: Bouclier::default(),
                inventaire: Vec::<Potion>::new()

            }
        }
    }

    impl St_Exupery {
        pub fn new(vitalite: i16, epee: Epee, bouclier:Bouclier, inventaire: Vec<Potion>) -> Self {
            Self {vie: vitalite, 
                epee:epee,
                bouclier:bouclier,
                inventaire:inventaire
            }
        }
    }