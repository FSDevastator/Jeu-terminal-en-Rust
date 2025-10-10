use crate::models::Epee;
use crate::models::Bouclier;

use std::str::FromStr;

/// Représente les quatres scénarios que le joueur peut rencontrer lors 
/// d'exploration du monde de jeu.
pub enum Scenario {
    /// Un combat avec un monstre
    Monstre,
    /// Le joueur trouve une potion
    Potion,
    /// Le joueur trouve une épée.
    Epee (Epee),
    /// Le jouer trouve un bouclier.
    Bouc (Bouclier),
}

/// Enum utilisé pour recevoir une épée ou un bouclier trouvé dans le monde 
/// (selon Scenario).  L'item est passé à la méthode equip_combat_item d'une 
/// instance StExupery pour équiper l'item selon le choix du joueur.
pub enum CombatItem {
    Sword(Epee),
    Shield(Bouclier),
}

/// Représente le choix de difficulté du jeu à partir du menu principal.
#[derive(Debug,PartialEq)]
pub enum Diff {
    // Mode facile
    F,
    // Mode moyen
    M,
    // Mode difficile
    D,
}

impl FromStr for Diff {
    type Err = String;

    /// Définition de la méthode fromstr du trait FromStr pour établir 
    /// une valeur pour le enum en fonction de la saisie du joueur dans 
    /// le menu du terminal.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f" => Ok(Diff::F),
            "m" => Ok(Diff::M),
            "d" => Ok(Diff::D),
            _=> Err(format!("La valeur {} est invalide pour enum de type Diff.",s)),
        }
    }
}

/// Enum de navigation principal du jeu.  
#[derive(Debug,PartialEq)]
pub enum Nav {
    // Le joueur choisit d'explorer
    Explorer,
    // Le joueur choisit de "guérir" son instance StExupery.
    Guerir,
    // Le joueur choisit de quitter le programme.
    Quit
}

impl FromStr for Nav {
    type Err = String;

    /// Définition de la méthode fromstr du trait FromStr pour établir 
    /// une valeur pour le enum en fonction de la saisie du joueur dans 
    /// le menu du terminal.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Nav::Explorer),
            "b" => Ok(Nav::Guerir),
            "q" => Ok(Nav::Quit),
            _=> Err(format!("La valeur {} est invalide pour enum de type Nav.",s)),
        }
    }
}

/// Enum pour permettre au joueur de consommer ou conserver des potions 
/// trouvées lors de l'exploration du monde de jeu.
#[derive(Debug,PartialEq)]
pub enum PotionAction {
    Consume,
    Store,
}

impl FromStr for PotionAction {
    type Err = String;

    /// Définition de la méthode fromstr du trait FromStr pour établir 
    /// une valeur pour le enum en fonction de la saisie du joueur dans 
    /// le menu du terminal.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(PotionAction::Consume),
            "i" => Ok(PotionAction::Store),
            _=> Err(format!("La valeur {} est invalide pour enum de type PotionAction.",s)),
        }
    }
}

/// Enum pour permettre au joueur de choisir d'équipper ou abandonner un item équippable trouvé
/// lors d'exploration du monde de jeu.
#[derive(Debug,PartialEq)]
pub enum ItemAction {
    Equip,
    Discard,
}

impl FromStr for ItemAction {
    type Err = String;

    /// Définition de la méthode fromstr du trait FromStr pour établir 
    /// une valeur pour le enum en fonction de la saisie du joueur dans 
    /// le menu du terminal.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(ItemAction::Equip),
            "a" => Ok(ItemAction::Discard),
            _=> Err(format!("La valeur {} est invalide pour enum de type ItemAction.",s)),
        }
    }
}

