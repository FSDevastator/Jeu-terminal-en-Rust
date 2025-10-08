use crate::potion::Potion;
use crate::epee::Epee;
use crate::bouclier::Bouclier;

use std::str::FromStr;


pub enum Scenario {
    Monstre,
    Potion,
    Epee (Epee),
    Bouc (Bouclier),
}

pub enum CombatItem {
    Sword(Epee),
    Shield(Bouclier),
}

#[derive(Debug,PartialEq)]
pub enum Diff {
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

#[derive(Debug,PartialEq)]
pub enum Nav {
    Explorer,
    Guerir,
    Quit
}

impl FromStr for Nav {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Nav::Explorer),
            "b" => Ok(Nav::Guerir),
            "q" => Ok(Nav::Quit),
            _=> Err(format!("La valeur {} est invalide pour enum de type Nav.",s)),
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum PotionAction {
    Consume,
    Store,
}

impl FromStr for PotionAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(PotionAction::Consume),
            "i" => Ok(PotionAction::Store),
            _=> Err(format!("La valeur {} est invalide pour enum de type PotionAction.",s)),
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum ItemAction {
    Equip,
    Discard,
}

impl FromStr for ItemAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(ItemAction::Equip),
            "a" => Ok(ItemAction::Discard),
            _=> Err(format!("La valeur {} est invalide pour enum de type ItemAction.",s)),
        }
    }
}

