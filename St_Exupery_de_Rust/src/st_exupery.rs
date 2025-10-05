

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
