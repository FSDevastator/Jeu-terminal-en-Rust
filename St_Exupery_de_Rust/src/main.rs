use colored::Colorize;
use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::process;
use std::thread;
use std::time::Duration;
use pad::{PadStr,Alignment};


mod enums;
use crate::enums::Diff;
use crate::enums::CombatItem;
use crate::enums::Scenario;
use crate::enums::Nav;
use crate::enums::PotionAction;
use crate::enums::ItemAction;

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
        println!("\nQuitting...");
        process::exit(0);
    }

    let mut world_entity_hero = StExupery::default();
    let mut world_hero_death =false;

    loop {
        let world_nav_choice = world_navigate(&world_entity_monsters, &mut world_entity_hero);

        match world_nav_choice {
            Nav::Explorer => {

                let world_scenario = world_get_scenario();

                world_hero_death=world_set_scenario(world_scenario, &mut world_entity_monsters, &mut world_entity_hero);

                if world_hero_death == true {
                    break;
                }

            }
            Nav::Guerir => {
                world_entity_hero.boire_potion();
            }
            Nav::Quit => {
                println!("\nQuitting...");
                process::exit(0);
            }
        }

        if world_entity_monsters.len() ==0 {

            thread::sleep(Duration::from_millis(2000));
            println!("\n{}", "Toutes les créatures de ce territoire sont anéanties...".truecolor(230, 112, 11));
            thread::sleep(Duration::from_millis(2000));
            println!("\n{}","Vous prenez un genou et déposez votre arme.  Vous jurez en silence que Rust verra l'aube...".truecolor(230, 112, 11));
            thread::sleep(Duration::from_millis(5000));

            println!("\n");

            world_entity_monsters = main_menu();
            
            if world_entity_monsters.len()==0 {
                println!("\nQuitting...");
                process::exit(0);
            }
            world_entity_hero = StExupery::default();

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

    println!("{text:^width$}", text="St-Exupéry de Rust".on_truecolor(54, 235, 102), width=50);
    thread::sleep(Duration::from_secs(1));

    loop {
        input_string.clear();
        println!("\nChoisir le niveau de défi:\n\n (F)acile,\n (M)oyen\n (D)ifficile\n (Q)uitter\n");
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
    print!("+∩+ >> ");
    io::stdout().flush();
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

fn sit_rep (monsters: usize, hero:&mut StExupery) {
    thread::sleep(Duration::from_secs(1));
    println!("\nLe silence est lourd dans le royaume..mais vous sentez une menace omni-présente...");
    thread::sleep(Duration::from_secs(2));
    println!("\nIl reste {} créatures à anéantir...\n", monsters.to_string().yellow());
    thread::sleep(Duration::from_millis(1000));
    
    println!("{} a {} de vitalité et {} {}...", "St-Exupéry".truecolor(0,19,94), hero.get_health().to_string().yellow(),
     hero.get_inventory().to_string().yellow(), "potions".truecolor(207, 163, 234));

    thread::sleep(Duration::from_millis(500));
    println!("{}", hero.get_sword());
    thread::sleep(Duration::from_millis(500));
    println!("{}",hero.get_shield());
    thread::sleep(Duration::from_millis(1000));

}

fn world_navigate(monsters:&Vec<Monstre>, hero: &mut StExupery) -> Nav {

    sit_rep(monsters.len(), hero);

    println!("\n(E)xplorer le royaume abandonné.");
    println!("(B)oire une {}", "potion".truecolor(207, 163, 234));
    println!("(Q)uitter la quête");

    let mut world_choice = String::new();
    world_choice=get_input();

    Nav::from_str(&world_choice).unwrap()
}

fn world_get_scenario() -> Scenario {

    let mut rng = rand::thread_rng();
    let select: i16 = rng.gen_range(1..9);

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

            println!("\nVous avancez depuis quelques temps, et vous apercevez quelque chose qui capte votre attention...");
            thread::sleep(Duration::from_millis(500));
            println!("\n{} trouvée! (C)onsommer ou mettre en (I)nventaire?","Potion".truecolor(207, 163, 234));

            match PotionAction::from_str(&get_input().to_lowercase()).unwrap() {
                PotionAction::Consume => {
                    hero.boire_potion_trouvee();
                }

                PotionAction::Store => {
                    hero.ajouter_potion();
                }

            }

            false

        }

        Scenario::Epee(sword) => {

            thread::sleep(Duration::from_millis(1000));
            println!("\nVous avancez depuis quelques temps, et vous apercevez quelque chose qui capte votre attention...");
            thread::sleep(Duration::from_millis(2000));
            println!("\nUne épée...");
            thread::sleep(Duration::from_millis(1000));
            println!("\n{}",sword);
            thread::sleep(Duration::from_millis(500));
            println!("\n(E)quiper ou (a)bandonner?") ;

            match ItemAction::from_str(&get_input().to_lowercase()).unwrap() {
                ItemAction::Equip => {
                    hero.equip_combat_item(CombatItem::Sword((sword)));
                }
                ItemAction::Discard => {
                    println!("\nItem abandonné.")
                }
            }

            false
        }
        Scenario::Bouc(shield) => {

            thread::sleep(Duration::from_millis(1000));
            println!("\nVous avancez depuis quelques temps, et vous apercevez quelque chose qui capte votre attention...");
            thread::sleep(Duration::from_millis(2000));
            println!("\nUn bouclier...?");
            thread::sleep(Duration::from_millis(1000));
            println!("\n{}",shield);
            thread::sleep(Duration::from_millis(500));
            println!("\n(E)quiper ou (a)bandonner?") ;

            match ItemAction::from_str(&get_input().to_lowercase()).unwrap() {
                ItemAction::Equip => {
                    hero.equip_combat_item(CombatItem::Shield(shield));
                }
                ItemAction::Discard => {
                    println!("\nItem abandonné.")
                }
            }

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
            
            thread::sleep(Duration::from_millis(2000));
            println!("\nVous avancez depuis quelques temps...");
            thread::sleep(Duration::from_millis(1000));
            println!("\nSoudainement, une créature maléfique sort d'une ombre et court vers vous...");
            thread::sleep(Duration::from_millis(2000));
            println!("\n{}\n",world_entity_opponent);
            thread::sleep(Duration::from_millis(1500));
            
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

