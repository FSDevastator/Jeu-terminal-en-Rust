use crate::Epee;
use crate::Bouclier;
use rand::Rng;


pub struct Loot {
    swords: [Epee;20],
    shields: [Bouclier;20]    
}

impl Default for Loot {
    fn default() -> Self {
        Self {
            swords :[
            Epee::new("Pocket knife",11),
            Epee::new("Paring blade",11),
            Epee::new("Trimmer",11),
            Epee::new("Utility blade",11),
            Epee::new("Cleaver",13),
            Epee::new("Render",13),
            Epee::new("Cimeter blade",13),
            Epee::new("Carver blade",13),
            Epee::new("Naikiri",16),
            Epee::new("Santoku blade",16),
            Epee::new("Machete",16),
            Epee::new("Latin blade",17),
            Epee::new("Barong",17),
            Epee::new("Bolo blade",17),
            Epee::new("Kukri machete",18),
            Epee::new("Wakizashi sword",18),
            Epee::new("Khanda double-edge",18),
            Epee::new("Claymore",19),
            Epee::new("Katana",19),
            Epee::new("Ghostwalker blade",20)],
            shields:[
            Bouclier::new("Pizza tray",1),
            Bouclier::new("Non-stick skillet",1),
            Bouclier::new("Stainless sauce pot",1),
            Bouclier::new("Cast-iron skillet",1),
            Bouclier::new("Riot shield",2),
            Bouclier::new("Ceremonial shield",2),
            Bouclier::new("Kite shield",2),
            Bouclier::new("Heater shield",2),
            Bouclier::new("Pavise",2),
            Bouclier::new("Tower shield",3),
            Bouclier::new("Steel shield",3),
            Bouclier::new("Targe shield",3),
            Bouclier::new("Mindanao shield",3),
            Bouclier::new("Dome shield",3),
            Bouclier::new("Aspis shield",3),
            Bouclier::new("Alumina shield",4),
            Bouclier::new("Lonsdaleite shield",4),
            Bouclier::new("Silicon carbide shield",4),
            Bouclier::new("Boron carbide shield",4),
            Bouclier::new("Graphene composite",5)]
        }
    }
}

impl Loot {
    pub fn get_swords(&self) -> &[Epee]{
        &self.swords
    }
}

impl Loot {
    pub fn get_shields(&self) -> &[Bouclier] {
        &self.shields
    }
}

impl Loot {
    pub fn find_sword(&self) -> &Epee {
        let mut rng = rand::thread_rng();
        let mut chance1:usize = rng.gen_range(0..19);

        if chance1 >= 7{
            if rng.gen_range(1..8) == 1 {
                chance1 = rng.gen_range(7..19);
            } else {
                chance1 = rng.gen_range(0..7);
            }
        }
        &self.get_swords()[chance1]
    }
}

impl Loot {
    pub fn find_shield(&self) -> &Bouclier {
        let mut rng = rand::thread_rng();
        let mut chance1:usize = rng.gen_range(0..19);

        if chance1 >= 9{
            if rng.gen_range(1..8) == 1 {
                chance1 = rng.gen_range(9..19);
            } else {
                chance1 = rng.gen_range(0..9);
            }
        }
        &self.get_shields()[chance1]
    }
}