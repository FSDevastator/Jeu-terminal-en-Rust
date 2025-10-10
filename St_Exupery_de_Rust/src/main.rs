// Importation de modules std, colored et rand
use colored::Colorize;
use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::process;
use std::thread;
use std::time::Duration;

// Importation des models de models.rs
mod models;
use crate::models::StExupery;
use crate::models::Monstre;
use crate::models::Loot;

// Importation des enums de enums.rs
mod enums;
use crate::enums::Diff;
use crate::enums::CombatItem;
use crate::enums::Scenario;
use crate::enums::Nav;
use crate::enums::PotionAction;
use crate::enums::ItemAction;

/// main() contient le bloc d'exécution principal du programme qui orchestre la progression dans
/// le jeu.
/// 
/// Declare les entités principales et appelle main_menu() pour les initialiser 
/// selon le choix de difficulté de l'utilisateur.
/// 
/// main() gère la boucle principale de navigation du monde de jeu et permet
/// de progresser vers l'objectif en faisant appel à world_navigate, world_get_scenario
/// et world_set_scenario selon l'évaluation de 'match' pour un enum 'Nav'.
/// 
/// Permet de quitter le programme lorsque le joueur choisit de quitter à partir du menu principal ou
/// le menu de navigation.
/// 
/// #Arguments
/// 
/// * Aucuns
/// 
/// #Retourne
/// 
/// * ( )
/// 

fn main() {

    let (mut world_entity_hero, mut world_entity_monsters) = main_menu();

    if world_entity_monsters.len()==0 {
        println!("\nFermeture...\n");
        process::exit(0);
    }

    let mut world_hero_death =false;

    loop {
        let world_nav_choice = world_navigate(&world_entity_monsters, &mut world_entity_hero);

        match world_nav_choice {
            Nav::Explorer => {

                let world_scenario = world_get_scenario();

                world_hero_death=world_set_scenario(world_scenario, &mut world_entity_monsters, &mut world_entity_hero);

                if world_hero_death == true {
                    (world_entity_hero, world_entity_monsters)=main_menu();

                    if world_entity_monsters.len()==0 {
                        println!("\nFermeture...\n");
                        process::exit(0);
                    }
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

            (world_entity_hero, world_entity_monsters) = main_menu();
            
            if world_entity_monsters.len()==0 {
                println!("\nFermeture...\n");
                process::exit(0);
            }
            
        }

    }

}

/// Présente le menu principal au joueur et recoit son choix de difficulté du jeu.
/// Appelle la fonction world_initialize_entities en fonction du choix.
/// 
/// #Arguments
/// 
/// * Aucuns
/// 
/// #Retourne
/// 
/// * (StExupery, Vec<Monstre>)
/// 
/// # Exemples
/// 
/// '''
/// let (mut hero, mut monstres) = main_menu();
/// '''
fn main_menu() -> (StExupery,Vec<Monstre>) {
    
    let mut main_menu_sel = String::new();

    let mut hero = StExupery::default();
    let mut world_monsters = Vec::<Monstre>::default();

    println!("{text:^width$}", text="St-Exupéry de Rust".on_truecolor(212,151,11), width=50);
    thread::sleep(Duration::from_secs(1));


    println!("\nChoisir le niveau de défi:\n\n (F)acile,\n (M)oyen\n (D)ifficile\n (Q)uitter\n");
    main_menu_sel = get_validated_menu_input(&vec!["f".to_string(),
                                                        "m".to_string(),
                                                        "d".to_string(),
                                                        "q".to_string()]);

    if main_menu_sel =="q" {
        return (hero,world_monsters)
    } else {
        hero = world_initialize_entities(Diff::from_str(&main_menu_sel.to_lowercase()).unwrap(), 
                                                        &mut world_monsters);
    }

    (hero, world_monsters)

}

/// Utilise io::stdin pour recevoir la saisie de l'utilisateur au terminal et 
/// la retourner de la fonction.
/// 
/// #Arguments
/// 
/// * Aucuns
/// 
/// #Retourne
/// 
/// * String
/// 
/// # Exemples
/// 
/// '''
/// let choice = get_input();
/// '''
fn get_input() -> String {
    let mut input_string = String::new();
    print!("+∩+ >> ");
    let _ = io::stdout().flush();
    io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line.");

        input_string.trim().to_string()
}

/// Recoit un vecteur de valeurs valides pour la saisie au terminal de l'utilisateur
/// et appelle get_input().  Si la saisie n'est pas valide demande de saisir à nouveau.
/// 
/// #Arguments
/// 
/// * 'valid'
/// 
/// #Retourne
/// 
/// * String
/// 
/// # Exemples
/// 
/// '''
/// let valid_choices = ["a", "b", "c"];
/// 
/// let validated_menu_option = get_validated_menu_input(valid_choices);
/// 
/// '''
fn get_validated_menu_input(valid: &Vec<String>) -> String {
    
    let mut input = String::new();
    loop{
        input.clear();
        input = get_input();
        
        if valid.contains(&input.to_lowercase()) {
            return input
        } else {
            println!("Votre choix est invalide.")
        }
    }
}

/// 'Match' la valeur de 'difficulty'(enum Diff) et initialise les modèles Monstre et StExupery
/// en fonction.
/// 
/// #Arguments
/// 
/// * 'difficulty', 'enemies'
/// 
/// #Retourne
/// 
/// * StExupery
/// 
/// # Exemples
/// 
/// '''
/// let mut monsters = Vec::<Monstre>::new();
/// let mut hero = world_initialize_entities(Diff::F, &mut monsters);
/// 
/// '''
fn world_initialize_entities (difficulty: Diff, enemies: &mut Vec<Monstre>) -> StExupery {
    
    match difficulty {
        Diff::F => {
            for _i in 0..5 {
                enemies.push(Monstre::new(&difficulty));
            }

            return StExupery::new(&difficulty);
        }

        Diff::M => {
            for _i in 0..7 {
                enemies.push(Monstre::new(&difficulty));
            }

            return StExupery::new(&difficulty);
        }

        Diff::D => {
            for _i in 0..10 {
                enemies.push(Monstre::new(&difficulty));
            }

            return StExupery::new(&difficulty);
        }
    }

}

/// Cette fonction affiche un sommaire de valeurs dérivés des
/// entités et leurs attributs.  Ceci permet au joueur
/// de se situer vis-à-vis l'atteinte de l'objectif.
/// 
/// #Arguments
/// 
/// * 'monsters', 'hero'
/// 
/// #Retourne
/// 
/// * ( )
/// 
/// # Exemples
/// 
/// '''
/// world_summary(world_entity_monsters.len(), world_entity_hero);
/// '''
fn world_summary (monsters: usize, hero:&mut StExupery) {
    thread::sleep(Duration::from_secs(1));
    println!("\nLe silence est lourd dans le royaume..mais vous sentez une menace omni-présente...");
    thread::sleep(Duration::from_secs(2));
    println!("\nIl reste {} créatures à anéantir...\n", monsters.to_string().truecolor(229, 240, 115));
    thread::sleep(Duration::from_millis(1000));
    
    println!("{} a {} de vitalité et {} {}...", "St-Exupéry".truecolor(212,151,11), hero.get_health().to_string().yellow(),
     hero.get_inventory().to_string().yellow(), "potions".truecolor(207, 163, 234));

    thread::sleep(Duration::from_millis(500));
    println!("{}", hero.get_sword());
    thread::sleep(Duration::from_millis(500));
    println!("{}",hero.get_shield());
    thread::sleep(Duration::from_millis(1000));

}

/// Présente le menu de navigation principal au joueur et recoit un choix.
/// 
/// #Arguments
/// 
/// * 'monsters', 'hero'
/// 
/// #Retourne
/// 
/// * Enum 'Nav'
/// 
/// # Exemples
/// 
/// '''
/// let nav_choice = world_navigate(world_entity_monsters, world_entity_hero);
/// '''
fn world_navigate(monsters:&Vec<Monstre>, hero: &mut StExupery) -> Nav {

    let mut world_choice = String::new();

    
    world_summary(monsters.len(), hero);

    println!("\n(E)xplorer le royaume abandonné.");
    println!("(B)oire une {}", "potion".truecolor(207, 163, 234));
    println!("(Q)uitter la quête");

    world_choice.clear();
    world_choice=get_validated_menu_input(&vec!["e".to_string(),
                                                    "b".to_string(),
                                                    "q".to_string()]);

    Nav::from_str(&world_choice).unwrap()
    
}

/// Lorsque le joueur choisit d'explorer le monde du jeu, la fonction choisit un de quatres 
/// scénarios exploratoires possibles dans le jeu (combat, ou trouver un bouclier, une épée ou une potion) 
/// de manière aléatoire.
/// 
/// #Arguments
/// 
/// * Aucuns
/// 
/// #Retourne
/// 
/// * Enum 'Scenario'
/// 
/// # Exemples
/// 
/// '''
/// let scene = world_get_scenario();
/// '''
fn world_get_scenario() -> Scenario {

    let rng = rand::rng;
    let select: i16 = rng().random_range(1..9);

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

/// 'Match' le 'Scenario' produit par world_get_scenario() à un scénario parmi trouver une potion, 
/// une arme ou un bouclier ou, de combattre un monstre et exécute le scénario.
/// 
/// #Arguments
/// 
/// * 'scene', 'monsters', 'hero'
/// 
/// #Retourne
/// 
/// bool ('true' si points de vie de l'instance StExupery <= 0)
/// 
/// # Exemples
/// 
/// ''' 
/// let hero_state = world_set_scenario(scene, monsters, hero);
/// '''
fn world_set_scenario (scene:Scenario, monsters:&mut Vec<Monstre>, hero:&mut StExupery) -> bool {

    match scene {
        Scenario::Potion => {

            println!("\nVous avancez depuis quelques temps, et vous apercevez quelque chose qui capte votre attention...");
            thread::sleep(Duration::from_millis(500));
            println!("\n{} trouvée! (C)onsommer ou mettre en (I)nventaire?","Potion".truecolor(207, 163, 234));

            match PotionAction::from_str(&get_validated_menu_input(&vec!["c".to_string(),"i".to_string()]).to_lowercase()).unwrap() {
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

            match ItemAction::from_str(&get_validated_menu_input(&vec!["e".to_string(),"a".to_string()]).to_lowercase()).unwrap() {
                ItemAction::Equip => {
                    hero.equip_combat_item(CombatItem::Sword(sword));
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

            match ItemAction::from_str(&get_validated_menu_input(&vec!["e".to_string(),"a".to_string()]).to_lowercase()).unwrap() {
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
                    Monstre::default()
                }
            };
            
            thread::sleep(Duration::from_millis(2000));
            println!("\nVous avancez depuis quelques temps...");
            thread::sleep(Duration::from_millis(1000));
            println!("\nSoudainement, une créature inhumaine, maléfique sort d'une ombre et court vers vous...");
            thread::sleep(Duration::from_millis(2000));
            println!("\n{}\n",world_entity_opponent);
            thread::sleep(Duration::from_millis(1500));
            
            let mut hero_state:bool = false;
            let mut monster_state:bool = false;

            loop {

                monster_state = world_entity_opponent.take_damage(hero.attack());

                if monster_state == true {
                    return false
                }

                hero_state = hero.take_damage(world_entity_opponent.attack());

                if hero_state==true {
                    return true
                }
            }

        }
    }
}   

