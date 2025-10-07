

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

    impl StExupery {
        pub fn new(vitalite: i16, epee: Epee, bouclier:Bouclier, inventaire: Vec<Potion>) -> Self {
            Self {vie: vitalite, 
                epee:epee,
                bouclier:bouclier,
                inventaire:inventaire
            }
        }
    }

    impl StExupery {
        pub fn get_health(&self) -> i16 {
            self.vie
        }
    }

    impl StExupery {
        pub fn set_health(&mut self, valeur:i16) {
            self.vie = valeur;
        }
    }

    // New
    impl StExupery {
        pub fn get_shield(&mut self) -> &mut Bouclier {
            &mut self.bouclier
        }
    }
    // New
    impl StExupery {
        pub fn get_sword(&mut self) -> &mut Epee {
            &mut self.epee
        }
    }

    impl StExupery {
        pub fn ajouter_potion(&mut self) {
            if self.inventaire.len()==3 {
                println!("Inventaire plein! La potion a été abandonnée.")
            } else {
                self.inventaire.push(Potion::default());
                println!("Potion conservée pour plus tard.")
            }
        
        }
    }

    impl StExupery {
        pub fn boire_potion(&mut self){

            if self.get_health()==100 {
                println!("Vitalité déjà au maximum!")
            } else {
                match self.inventaire.pop() {
                Some(potion) => {
                    if self.get_health() + potion.get_puissance() > 100 {
                        self.set_health(100);
                        println!("Potion consommé! Vitalité max!")
                    } else {
                        self.set_health(potion.get_puissance()+ self.get_health());
                        println!("Potion consommé!  Vitalité {}",self.get_health())
                    }
                }
                None => {
                    println!("Vous n'avez aucune Potion!")
                }
            }
            
            }

        }
    }

    impl StExupery {

        pub fn boire_potion_trouvee (&mut self) {

            if self.get_health()==100 {
                println!("Vitalité déjà au maximum!");
            } else if self.get_health() + Potion::default().get_puissance() > 100 {
                self.set_health(100);
                println!("Potion consommé! Vitalité max!");
            } else {
                self.set_health( self.get_health() + Potion::default().get_puissance());
                println!("Potion consommé!  Vitalité {}",self.get_health());
            }
        }
    }

    impl StExupery {
        pub fn take_damage (&mut self, mut dam:i16)-> bool {
			
            dam -= self.get_shield().get_def();

            if dam <= 0 {
                dam=0;
            }

            if self.get_health() - dam <= 0{
                println!("Le héro a péri dans le combat.");
                return true
                
            } else {
                self.set_health(self.get_health() - dam);
                println!("St-Exupéry fait dévier l'attaque avec {}!  Recoit seulement {} points de dommages!",
                        self.get_shield().get_name(), dam);
                println!("\nVitalité à {}.", self.get_health());

                return false
            }
        }
    }

    impl StExupery {
        pub fn attack(&mut self) ->i16 {
            let attack:i16 = self.get_health()/10 + self.get_sword().get_puissance();
            println!("St-Exupéry attaque avec {} pour {} dommages!", self.get_sword().get_name(),attack);
            attack
        }
    }