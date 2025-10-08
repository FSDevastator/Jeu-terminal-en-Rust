use std::fmt;
use rand::Rng;
use std::thread;
use std::time::Duration;
use colored::Colorize;

use crate::Diff;

pub struct Monstre {
    vitalite: i16,
    nom: String,
    min_domm: i16,
    max_domm: i16
}

impl fmt::Display for Monstre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Le monstre {} a {} points de vitalité.",self.get_name().truecolor(159, 120, 36), self.get_vitalite().to_string().yellow())
    }
}

impl Default for Monstre {
    fn default()-> Self {

        let noms:[&str;10] = [
            "Demondra, The Decayed Serpent",
            "Nightclot, The Disfigured Canine",
            "Gruemera, The Cursed Slasher",
            "Bloodlisk, The Monstrous Raven",
            "Blackclot, The Horned Monster",
            "Verminclot, The Horned Serpent",
            "Gutmoth, The Corrupt Revenant",
            "Wispbody, The Vengeful Demon",
            "Poisonbane, The Wild Witch",
            "Morbidling, The Cursed Demon"
        ];
        
        let mut rng = rand::thread_rng();
        let mut lemonstre:usize = rng.gen_range(0..9);

        Self {
            vitalite: 100,
            nom: noms[lemonstre].to_string(),
            min_domm:5,
            max_domm:10
        }
    }
}

impl Monstre {
    pub fn new(Difficulty :&Diff) ->Self {

        let noms:[&str;10] = [
            "Demondra, The Decayed Serpent",
            "Nightclot, The Disfigured Canine",
            "Gruemera, The Cursed Slasher",
            "Bloodlisk, The Monstrous Raven",
            "Blackclot, The Horned Monster",
            "Verminclot, The Horned Serpent",
            "Gutmoth, The Corrupt Revenant",
            "Wispbody, The Vengeful Demon",
            "Poisonbane, The Wild Witch",
            "Morbidling, The Cursed Demon"
        ];

        let mut rng = rand::thread_rng();
        let mut le_nom:usize = rng.gen_range(0..9);

        let mut vit:i16 = 0;
        let mut min_d:i16 = 0;
        let mut max_d:i16 = 0;

        match Difficulty {
            Diff::F => {
                vit= rng.gen_range(60..80);
                min_d= rng.gen_range(8..9);
                max_d= rng.gen_range(10..11);
            }
            Diff::M => {
                vit = rng.gen_range(150..170);
                min_d = rng.gen_range(11..14);
                max_d = rng.gen_range(15..18);
            }
            Diff::D => {
                vit = rng.gen_range(200..220);
                min_d = rng.gen_range(14..17);
                max_d = rng.gen_range(18..21);
            }
        }

        Self {
            vitalite:vit,
            nom: noms[le_nom].to_string(),
            min_domm:min_d,
            max_domm:max_d
        }
    }
}

impl Monstre {
    pub fn get_vitalite(&self) ->&i16 {
        &self.vitalite
    }
}

impl Monstre {
    fn set_vitalite(&mut self,value:i16) {
        self.vitalite = value;
    }
}

impl Monstre {
    pub fn get_name(&self) -> &str {
        &self.nom
    }
}


impl Monstre {
    pub fn attack(&self) -> i16 {
        let mut rng = rand::thread_rng();
        
        let attack =rng.gen_range(self.min_domm..self.max_domm);

        println!("{} {} pour {} points!", self.get_name().truecolor(159, 120, 36),"attaque".red(), attack.to_string().yellow());

        thread::sleep(Duration::from_millis(2000));

        attack

    }
}

impl Monstre {
    pub fn take_damage(&mut self,dam:i16) ->bool {
        
        if self.get_vitalite()-dam <=0 {
            println!("\n{} anéanti!",self.get_name().truecolor(159, 120, 36));
            thread::sleep(Duration::from_millis(1000));
            return true
        } else {
            self.set_vitalite(*self.get_vitalite()-dam);
            println!("{} est atteint pour {} dommages!", self.get_name().truecolor(159, 120, 36), dam.to_string().yellow());
            thread::sleep(Duration::from_millis(2000));
            println!("Encore {} points de vitalité!\n",self.get_vitalite().to_string().yellow());
            thread::sleep(Duration::from_millis(1000));
            return false
        }   
    }
}