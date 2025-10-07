use rand::Rng;
use std::io;
use std::str::FromStr;
use std::process;

mod enums;
use crate::enums::Diff;
use crate::enums::CombatItem;
use crate::enums::Scenario;

mod epee;
mod bouclier;
mod potion;
mod loot;
mod monstre;
mod st_exupery;


use crate::st_exupery::StExupery;
use crate::epee::Epee;
use crate::bouclier::Bouclier;
use crate::potion::Potion;
use crate::loot::Loot;
use crate::monstre::Monstre;


fn main() {

    println!("St-Exupéry de Rust.\n");

    let mut hero = StExupery::default();

    println!("{}", hero.get_shield());

    hero.equip_combat_item(CombatItem::Shield(Loot::default().find_shield().clone()));

    println!("{}", hero.get_shield());

    
}

