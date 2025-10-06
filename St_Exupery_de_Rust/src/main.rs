use rand::Rng;
use std::io;
use std::str::FromStr;
use std::process;

mod st_exupery;
mod epee;
mod bouclier;
mod potion;
mod loot;
mod monstre;
use crate::st_exupery::StExupery;
use crate::epee::Epee;
use crate::bouclier::Bouclier;
use crate::potion::Potion;
use crate::loot::Loot;
use crate::monstre::Monstre;


fn main() {

    println!("St-Exupéry de Rust.\n");

}

#[derive(Debug,PartialEq)]
enum Diff {
    F,
    M,
    D,
}

impl FromStr for Diff {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f" => Ok(Diff::F),
            "m" => Ok(Diff::M),
            "d" => Ok(Diff::D),
            _=> Err(format!("La valeur {} est invalide pour enum de type Diff.",s)),
        }
    }
}

enum Scenario {
    Monstre,
    Potion (Potion),
    Epee (Epee),
    Bouc (Bouclier),
}