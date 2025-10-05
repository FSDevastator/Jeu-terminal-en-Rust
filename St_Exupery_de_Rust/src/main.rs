use rand::Rng;
mod st_exupery;
mod epee;
mod bouclier;
mod potion;
mod loot;
use crate::st_exupery::StExupery;
use crate::epee::Epee;
use crate::bouclier::Bouclier;
use crate::potion::Potion;
use crate::loot::Loot;

fn main() {
    println!("St-Exupéry de Rust.");

    let l = Loot::default();

    println!("{}",l.find_sword());
}

