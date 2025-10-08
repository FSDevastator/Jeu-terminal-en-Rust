use std::thread;
use std::time::Duration;
use colored::Colorize;
use crate::CombatItem;
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
                inventaire: vec![Potion::default()]
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

    
    impl StExupery {
        pub fn get_shield(&mut self) -> &mut Bouclier {
            &mut self.bouclier
        }
    }
    
    impl StExupery {
        pub fn get_sword(&mut self) -> &mut Epee {
            &mut self.epee
        }
    }

    impl StExupery {
        pub fn get_inventory(&self) -> usize {
            self.inventaire.len()
        }

    }

    impl StExupery {
        pub fn ajouter_potion(&mut self) {
            if self.inventaire.len()==3 {
                println!("\nInventaire plein! La potion a été abandonnée.")
            } else {
                self.inventaire.push(Potion::default());
                println!("\nPotion conservée pour plus tard.")
            }
        
        }
    }

    impl StExupery {
        pub fn boire_potion(&mut self){

            if self.get_health()==100 {
                println!("\nVitalité déjà au maximum!")
            } else {
                match self.inventaire.pop() {
                Some(potion) => {
                    if self.get_health() + potion.get_puissance() > 100 {
                        self.set_health(100);
                        println!("\nPotion consommé! Vitalité max!")
                    } else {
                        self.set_health(potion.get_puissance()+ self.get_health());
                        println!("\nPotion consommé!  Vitalité {}",self.get_health())
                    }
                }
                None => {
                    println!("\nVous n'avez aucune Potion!")
                }
            }
            
            }

        }
    }

    impl StExupery {

        pub fn boire_potion_trouvee (&mut self) {

            if self.get_health()==100 {
                println!("\nVitalité déjà au maximum!");
                self.ajouter_potion()

            } else if self.get_health() + Potion::default().get_puissance() > 100 {
                self.set_health(100);
                println!("\nPotion consommé! Vitalité max!");
            } else {
                self.set_health( self.get_health() + Potion::default().get_puissance());
                println!("\nPotion consommé!  Vitalité {}",self.get_health().to_string().yellow());
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
                thread::sleep(Duration::from_millis(1000));
                println!("\n{}","Le héro a péri dans le combat\n".truecolor(245,10,10));
                return true
                
            } else {
                self.set_health(self.get_health() - dam);
                println!("{} fait {} l'attaque avec {}!  Recoit seulement {} points de dommages!",
                        "St-Exupéry".truecolor(0,19,94),
                        "dévier".truecolor(87, 247, 87), self.get_shield().get_name(), dam.to_string().yellow());
                thread::sleep(Duration::from_millis(2000));
                println!("Vitalité à {}.\n", self.get_health().to_string().yellow());
                thread::sleep(Duration::from_millis(1000));

                return false
            }
        }
    }

    impl StExupery {
        pub fn attack(&mut self) ->i16 {
            let attack:i16 = self.get_health()/10 + self.get_sword().get_puissance();
            println!("{} {} avec {} pour {} dommages!", "St-Exupéry".truecolor(0,19,94),"attaque".red(), 
            self.get_sword().get_name(),attack.to_string().yellow());
            thread::sleep(Duration::from_millis(2000));
            attack
        }
    }

    impl StExupery {
        pub fn equip_combat_item(&mut self, item: CombatItem) {
            match item {
                CombatItem::Sword(new_sword) => {
                    self.get_sword().set_name(new_sword.get_name());
                    self.get_sword().set_puissance(new_sword.get_puissance());
                    println!("\n{} équippé!",new_sword.get_name());

                }
                CombatItem::Shield(new_shield) => {
                    self.get_shield().set_name(new_shield.get_name());
                    self.get_shield().set_def(new_shield.get_def());
                    println!("\n{} équippé!",new_shield.get_name());
                }
            }
        }
    }