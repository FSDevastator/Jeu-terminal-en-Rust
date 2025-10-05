use crate::Epee;
use crate::Bouclier;
use rand::random;
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