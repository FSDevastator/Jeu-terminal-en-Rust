use rand::Rng;
use std::io;
use std::str::FromStr;
use std::process;

mod enums;
use crate::enums::Diff;
use crate::enums::CombatItem;
use crate::enums::Scenario;
use crate::enums::Nav;

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

    let mut world_entity_monsters = main_menu();

    if world_entity_monsters.len()==0 {
        println!("Quitting...");
        process::exit(0);
    }

    let mut world_entity_hero = StExupery::default();
    let mut world_hero_death =false;

    loop {
        let world_nav_choice = world_navigate(&world_entity_monsters, &world_entity_hero);

        match world_nav_choice {
            Nav::Explorer => {

                println!("here");
                let world_scenario = get_scenario();

                world_hero_death=world_set_scenario(Scenario::Monstre, &mut world_entity_monsters, &mut world_entity_hero);

                if world_hero_death == true {
                    break;
                }

            }
            Nav::Guerir => {
                world_entity_hero.boire_potion();
            }
            Nav::Quit => {
                println!("Quitting...");
                process::exit(0);
            }
        }

        if world_entity_monsters.len() ==0 {
            println!("\nToutes les créatures de ce territoire sont anéanties...");
            println!("\nVous prenez un genou et déposez votre arme.  Vous jurez en silence que Rust verra l'aube...");
        }
    }

}

fn main_menu() -> Vec<Monstre> {
    
    let mut input_string = String::new();
    
    let quit_values = vec![
        "q".to_string(),
        "quitter".to_string()
        ];

    let diff_values = vec![
        "f".to_string(),
        "m".to_string(),
        "d".to_string()
        ];
    
    let mut world_monsters = Vec::<Monstre>::default();

    println!("St-Exupéry de Rust.\n");

    loop {
        input_string.clear();
        println!("Choisir le niveau de défi, (F)acile, (M)oyen ou (D)ifficile ou taper (q)uitter,>> ");
        input_string = get_input();

        if quit_values.contains(&input_string.to_lowercase()) {
            return world_monsters
        }

        if diff_values.contains(&input_string.to_lowercase()) {
            initialize_monsters(Diff::from_str(&input_string.to_lowercase()).unwrap(), &mut world_monsters);
            break;
        }

    }

    world_monsters

}


fn get_input() -> String {
    let mut input_string = String::new();

    io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line.");

        input_string.trim().to_string()
}

fn initialize_monsters (Difficulty: Diff, enemies: &mut Vec<Monstre>) {
    
    match Difficulty {
        Diff::F => {
            for i in 0..5 {
                enemies.push(Monstre::new(&Difficulty));
            }
        }

        Diff::M => {
            for i in 0..7 {
                enemies.push(Monstre::new(&Difficulty));
            }
        }

        Diff::D => {
            for i in 0..10 {
                enemies.push(Monstre::new(&Difficulty));
            }
        }
    }

}

fn sit_rep (monsters: usize, hero:&StExupery) {
    println!("\nLe silence est lourd dans le royaume..mais vous sentez une menace omni-présente...");
    println!("\nIl reste {} créatures à anéantir...", monsters);
    println!("St-Exupéry a {} de vitalité et {} potions...", hero.get_health(), hero.get_inventory());
}

fn world_navigate(monsters:&Vec<Monstre>, hero: &StExupery) -> Nav {

    sit_rep(monsters.len(), hero);

    println!("(E)xplorer le royaume abandonné.");
    println!("(B)oire une potion");
    println!("(Q)uitter la quête");

    let mut world_choice = String::new();
    world_choice=get_input();

    Nav::from_str(&world_choice).unwrap()
}

fn get_scenario() -> Scenario {

    let mut rng = rand::thread_rng();
    let select: i16 = rng.gen_range(1..4);

    if select == 1 {
        Scenario::Potion
    } else if select == 2 {
        Scenario::Epee((Loot::default().find_sword()).clone())
    } else if select == 3 {
        Scenario::Bouc(Loot::default().find_shield().clone())
    } else {
        Scenario::Monstre
    }

}

fn world_set_scenario (scene:Scenario, monsters:&mut Vec<Monstre>, hero:&mut StExupery) -> bool {
    match scene {
        Scenario::Potion => {
            
            println!("\nVous avancez et vous apercevez quelque chose qui capte votre attention...");
            println!("Potion trouvée! (C)onsommer ou mettre en (I)nventaire?");

        }

        Scenario::Epee(sword) => {
            false
        }
        Scenario::Bouc(shield) => {
            false
        }

        Scenario::Monstre => {

            let mut world_entity_opponent= match monsters.pop() {
                
                Some(value) => {
                    value
                }
                
                None => {
                    println!("Plus de monstres");
                    Monstre::default()
                }
            };
            
            println!("\nSoudainement, une créature maléfique sort d'une ombre et court vers vous...");
            println!("\n{}",world_entity_opponent);
            
            let mut HeroState:bool = false;
            let mut MonsterState:bool = false;



            loop {

                MonsterState = world_entity_opponent.take_damage(hero.attack());

                if MonsterState == true {
                    return false
                }

                HeroState = hero.take_damage(world_entity_opponent.attack());

                if HeroState==true {
                    return true
                }
            }

        }
    }
}   

fn world_found_item () {

}