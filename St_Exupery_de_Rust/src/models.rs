
use std::thread;
use std::time::Duration;
use std::fmt;
use rand::Rng;

use colored::Colorize;
use crate::CombatItem;

use crate::enums::Diff;



/// St_Exupery modélise le joueur et ses actions avec
/// un struct composé.
/// 
/// # Attributs:
/// 
/// + *vie* (i16): représente la vitalité du joueur.
/// + *epee* (struct Epee): modèle Epee pour modéliser l'arme du joueur. 
/// + *bouclier* (struct Bouclier): modèle Bouclier modélise une arme pour 
/// réduire les dommages recus.
/// + *inventaire*
/// 
/// # Exemples
/// 
/// '''
/// let h = StExupery::new(&Diff::F);
/// '''
/// 
pub struct StExupery {
     vie: i16,
     pub epee: Epee,
     pub bouclier: Bouclier,
     pub inventaire: Vec<Potion>
}

impl Default for StExupery {

        /// Initialize une nouvelle instance 'StExupery' avec des valeurs par
        /// défaut pour les attributs.
        /// 
        /// #Arguments
        /// 
        /// * Aucun
        /// 
        /// #Retourne
        /// 
        /// * 'Self'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::default();
        /// '''
        fn default() -> Self{
            
            Self{
                vie:100,
                epee: Epee::default(),
                bouclier: Bouclier::default(),
                inventaire: vec![Potion::default()]
            }
        }
    }

    impl StExupery {

        /// Initialize une nouvelle instance 'StExupery'
        /// selon le niveau de difficulté choisit par l'utilisateur.
        /// 
        /// #Arguments
        /// 
        /// * 'diff' - niveau de difficulté 
        /// 
        /// #Retourne
        /// 
        /// * 'Self'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::new(diff);
        pub fn new(diff: &Diff) -> Self {

            match diff {
                Diff::F => {

                    Self {
                        vie: 100, 
                        epee:Epee::default(),
                        bouclier:Bouclier::default(),
                        inventaire: vec![Potion::default()]
                    }
                }

                Diff::M => {
                    
                    Self {
                        vie: 100, 
                        epee:Epee::default(),
                        bouclier:Bouclier::default(),
                        inventaire: vec![Potion::default();2]
                    }
                }

                Diff::D => {
                    Self {
                        vie: 100, 
                        epee:Epee::default(),
                        bouclier:Bouclier::default(),
                        inventaire: vec![Potion::default();3]
                    }
                }
            }
        
        }
    }


    impl StExupery {

        /// Retourne le niveau de vie d'une instance StExupery
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * 'self.vie'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::default();
        /// println!("{}", h.get_health())
        /// '''
        pub fn get_health(&self) -> i16 {
            self.vie
        }
    }

    impl StExupery {

        /// Modifie la valeur de l'attribut 'vie' d'une instance 
        /// StExupery
        /// 
        /// #Arguments
        /// 
        /// * 'valeur' 
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h  = StExupery::default();
        /// h.set_health(45)
        pub fn set_health(&mut self, valeur:i16) {
            self.vie = valeur;
        }
    }

    
    impl StExupery {

        /// Retourne la valeur de l'attribut 'bouclier' d'une instance
        /// StExupery
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * '&mut Bouclier'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::default();
        /// println!("{}",h.get_shield());
        /// '''
        pub fn get_shield(&mut self) -> &mut Bouclier {
            &mut self.bouclier
        }
    }
    
    impl StExupery {

        /// Retourne la valeur de l'attribut 'epee' d'une instance
        /// StExupery
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * '&mut Epee'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::default();
        /// println!("{}",h.get_sword());
        /// '''
        pub fn get_sword(&mut self) -> &mut Epee {
            &mut self.epee
        }
    }

    impl StExupery {

        /// Retourne le nombre de potions en inventaire d'une instance
        /// StExupery
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * 'usize'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery::default();
        /// println!("{}",h.get_inventory());
        /// '''
        pub fn get_inventory(&self) -> usize {
            self.inventaire.len()
        }

    }

    impl StExupery {

        /// Tente d'ajouter une potion dans l'inventaire d'une instance
        /// StExupery
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.ajouter_potion();
        /// '''
        pub fn ajouter_potion(&mut self) {
            if self.inventaire.len()==3 {
                println!("\nInventaire plein! La potion a été abandonnée.")
            } else {
                self.inventaire.push(Potion::default());
                println!("\nPotion conservée pour plus tard.")
            }
        
        }
    }

    impl StExupery {

        /// Tente de boire une potion de l'inventaire et indique la
        /// valeur de 'vie' d'une instance StExupery.
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.boire_potion();
        /// '''
        pub fn boire_potion(&mut self){

            if self.get_health()==100 {
                println!("\nVitalité déjà au maximum!")
            } else {
                match self.inventaire.pop() {
                Some(potion) => {
                    if self.get_health() + potion.get_puissance() > 100 {
                        self.set_health(100);
                        println!("\nPotion consommé! Vitalité {}!","max".yellow())
                    } else {
                        self.set_health(potion.get_puissance()+ self.get_health());
                        println!("\nPotion consommé!  Vitalité {}",self.get_health().to_string().yellow())
                    }
                }
                None => {
                    println!("\nVous n'avez aucune Potion!")
                }
            }
            
            }

        }
    }

    impl StExupery {

        /// Tente de boire une potion trouvée lors de la navigation du monde de jeu et indique la
        /// valeur de 'vie' d'une instance StExupery apr la suite.
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.boire_potion_trouvee();
        /// '''

        pub fn boire_potion_trouvee (&mut self) {

            if self.get_health()==100 {
                println!("\nVitalité déjà au maximum!");
                self.ajouter_potion()

            } else if self.get_health() + Potion::default().get_puissance() > 100 {
                self.set_health(100);
                println!("\nPotion consommé! Vitalité {}!","max".yellow());
            } else {
                self.set_health( self.get_health() + Potion::default().get_puissance());
                println!("\nPotion consommé!  Vitalité {}",self.get_health().to_string().yellow());
            }
        }
    }

    impl StExupery {

        /// Pour une instance StExupery: recoit la valeur de dommages infligés par un monstre, 
        /// ajuste cette valeur en fonction de la valeur d'un attribut de 'bouclier' et réduit 
        /// la valeur de 'vie' en conséquence.  Évalue si 'vie' <= 0 et retourne 'true' pour 
        /// indiqué le "décès" de l'instance.
        /// 
        /// #Arguments
        /// 
        /// * 'dam'
        /// 
        /// #Retourne
        /// 
        /// * bool 
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.take_damage(6)
        /// '''
        /// 
        pub fn take_damage (&mut self, mut dam:i16)-> bool {
			
            dam -= self.get_shield().get_def();

            if dam <= 0 {
                dam=0;
            }

            if self.get_health() - dam <= 0{
                thread::sleep(Duration::from_millis(1000));
                println!("\n{}","Le héro a péri dans le combat...\n".truecolor(245,10,10));
                thread::sleep(Duration::from_millis(3000));
                println!("\n{}","...la renaissance de Rust ne verra jamais l'aube...\n".truecolor(245,10,10));
                thread::sleep(Duration::from_millis(3000));
                
                return true
                
            } else {
                self.set_health(self.get_health() - dam);
                println!("{} fait {} l'attaque avec {}!  Recoit seulement {} points de dommages!",
                        "St-Exupéry".truecolor(212,151,11),
                        "dévier".truecolor(87, 247, 87), self.get_shield().get_name(), dam.to_string().yellow());
                thread::sleep(Duration::from_millis(2000));
                println!("Vitalité à {}.\n", self.get_health().to_string().yellow());
                thread::sleep(Duration::from_millis(1000));

                return false
            }
        }
    }

    impl StExupery {

        /// Additionne les points de vie d'une instance StExupery divisés par 10 et l'attribut
        /// de puissance de l'attribut 'epee' et retourne cette valeur.
        /// 
        /// #Arguments
        /// 
        /// *Aucuns
        /// 
        /// #Retourne
        /// 
        /// * i16
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.take_damage(6)
        /// '''
        /// 
        pub fn attack(&mut self) ->i16 {
            let attack:i16 = self.get_health()/10 + self.get_sword().get_puissance();
            println!("{} {} avec {} pour {} dommages!", "St-Exupéry".truecolor(212,151,11),"attaque".red(), 
            self.get_sword().get_name(),attack.to_string().yellow());
            thread::sleep(Duration::from_millis(2000));
            attack
        }
    }

    impl StExupery {

        /// Détermine le type d'attribut (épée ou bouclier) à modifier pour une instance StExupery
        /// et met à jour l'attribut suite choix de l'utilisateur d'équipper l'item.
        /// 
        /// #Arguments
        /// 
        /// * 'item'
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let h = StExupery.default();
        /// h.equip_combat_item(CombatItem::Sword(de_type_epee))
        /// '''
        /// 
        pub fn equip_combat_item(&mut self, item: CombatItem) {
            match item {
                CombatItem::Sword(new_sword) => {
                    *self.get_sword() = new_sword;
                    println!("\n{} équippé!",self.get_sword());

                }
                CombatItem::Shield(new_shield) => {
                    *self.get_shield() = new_shield;
                    println!("\n{} équippé!",self.get_shield());
                }
            }
        }
    }

/// Monstre modélise l'adversaire principal du jeu et ses
/// comportements.
/// 
/// # Attributs:
/// 
/// + *vitalite* (i16): représente la vitalité d'une instance Monstre.
/// + *nom* (String): contient le nom du monstre pour l'instance. 
/// + *min_domm*: dommage minimal d'attaque d'une instance. 
/// + *max_domm*: dommage maximal d'attaque d'une instance.
/// 
/// # Exemples
/// 
/// '''
/// let m = Monstre::new(&difficulty);
/// '''
/// 
pub struct Monstre {
    vitalite: i16,
    nom: String,
    min_domm: i16,
    max_domm: i16
}

impl fmt::Display for Monstre {

    /// Définition de fmt du trait 'Display' pour permettre d'afficher des informations
    /// sur les attributs d'une instance 'Monstre' dans le terminal.
    /// 
    /// #Arguments
    /// 
    /// * &self, f: &mut fmt::Formatter<'_>
    /// 
    /// #Retourne
    /// 
    /// * fmt::Result
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// println!("{}", m);
    /// '''
    /// 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Le monstre {} a {} points de vitalité.",self.get_name().truecolor(159, 120, 36), self.get_vitalite().to_string().yellow())
    }
}

impl Default for Monstre {

    /// Créer une instance 'Monstre' avec valeurs
    /// d'attributs fixes (par défaut).
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::default();
    /// 
    /// '''
    /// 
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
        
        let rng = rand::rng;
        let lemonstre:usize = rng().random_range(0..9);

        Self {
            vitalite: 100,
            nom: noms[lemonstre].to_string(),
            min_domm:5,
            max_domm:10
        }
    }
}

impl Monstre {

    /// Créer une instance 'Monstre' avec attributs spécifiques selon la difficulté 
    /// du jeu choisie par l'utilisateur.
    /// 
    /// #Arguments
    /// 
    /// * 'difficulty'
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// '''
    /// 
    pub fn new(difficulty :&Diff) ->Self {

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

        let rng = rand::rng;
        let le_nom:usize = rng().random_range(0..9);

        let mut vit:i16 = 0;
        let mut min_d:i16 = 0;
        let mut max_d:i16 = 0;

        match difficulty {
            Diff::F => {
                vit= rng().random_range(60..80);
                min_d= rng().random_range(8..9);
                max_d= rng().random_range(10..11);
            }
            Diff::M => {
                vit = rng().random_range(70..90);
                min_d = rng().random_range(9..10);
                max_d = rng().random_range(11..12);
            }
            Diff::D => {
                vit = rng().random_range(80..100);
                min_d = rng().random_range(10..11);
                max_d = rng().random_range(12..13);
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

    /// Retourne la valeur de l'attribut 'vitalite' d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucunes
    /// 
    /// #Retourne
    /// 
    /// * i16
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// m.get_vitalite();
    /// 
    /// '''
    /// 
    pub fn get_vitalite(&self) ->&i16 {
        &self.vitalite
    }
}

impl Monstre {

    /// Permet de fixer une valeur pour l'attribut 'vitalite'
    /// d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * 'value'
    /// 
    /// #Retourne
    /// 
    /// * ( )
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// m.set_vitalite(45);
    /// 
    /// '''
    /// 
    fn set_vitalite(&mut self,value:i16) {
        self.vitalite = value;
    }
}

impl Monstre {

    /// Retourne la valeur de l'attribut 'nom' d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &str
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// m.get_name();
    /// 
    /// '''
    /// 
    pub fn get_name(&self) -> &str {
        &self.nom
    }
}


impl Monstre {

    /// Établit la valeur d'une attaque vers le joueur et retourne cette valeur.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * 'attack'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// m.attack();
    /// 
    /// '''
    /// 
    pub fn attack(&self) -> i16 {
        let rng = rand::rng;
        
        let attack =rng().random_range(self.min_domm..self.max_domm);

        println!("{} {} pour {} points!", self.get_name().truecolor(159, 120, 36),"attaque".red(), attack.to_string().yellow());

        thread::sleep(Duration::from_millis(2000));

        attack

    }
}

impl Monstre {

    /// Recoit et soustrait la valeur d'une attaque du joueur de la valeur l'attribut
    /// 'vitalite' d'une instance et lui assigne.  Si <= 0, l'instance est considérée
    /// "anéanti.""
    /// 
    /// #Arguments
    /// 
    /// * 'dam'
    /// 
    /// #Retourne
    /// 
    /// * bool
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let m = Monstre::new(&difficulty);
    /// 
    /// '''
    /// 
    pub fn take_damage(&mut self,dam:i16) ->bool {
        
        if self.get_vitalite()-dam <=0 {
            println!("\n{} anéanti!",self.get_name().truecolor(159, 120, 36));
            thread::sleep(Duration::from_millis(2000));
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

/// Épée modélise une arme qui est un attribut d'une instance StExupery.
/// 
/// # Attributs:
/// 
/// + *nom* (String): représente un nom fictif.
/// + *puissance (i16)*: valeur de la puissance d'attaque de l'instance.
/// 
/// # Exemples
/// 
/// '''
/// let sword = Epee::default();
/// '''
/// 
#[derive(Clone)]
pub struct Epee {
        nom: String,
        puissance: i16,
    }

impl fmt::Display for Epee {

        /// Définition de fmt du trait 'Display' pour permettre d'afficher des informations
        /// sur les attributs d'une instance 'Epee' dans le terminal
        /// 
        /// #Arguments
        /// 
        /// *f: &mut fmt::Formatter<'_>
        /// 
        /// #Retourne
        /// 
        /// * fmt::Result
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// println!("{}", sword);
        /// 
        /// '''
        /// 
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} a l'{} {} avec puissance {}! ","St-Exupéry".truecolor(212,151,11), "épée".truecolor(59, 193, 255),self.get_name(),self.get_puissance().to_string().yellow())
        }
    }

    impl Default for Epee {

        /// Initialise les attributs d'une nouvelle instance avec des valeurs
        /// fixes.
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * 'Self'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// 
        /// '''
        /// 
        fn default() -> Self {
            Epee {
                nom: "Potato Peeler".to_string(),
                puissance: 8
            }
        }
    }

    impl Epee {

        /// Initialise les attributs d'une nouvelle instance avec
        /// des valeurs spécifiques.
        /// 
        /// #Arguments
        /// 
        /// * 'nom', 'puissance'
        /// 
        /// #Retourne
        /// 
        /// * 'Self'
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::new("Plastic fork", 0);
        /// 
        /// '''
        /// 
        pub fn new(nom: &str, puissance: i16 ) -> Self {
            Self {nom: nom.to_string(),puissance}
        }
    }

    impl Epee {

        /// Assigne une valeur à l'attribut 'nom' d'une instance.
        /// 
        /// #Arguments
        /// 
        /// * 'nouv'
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// sword.set_name("Pool noodle");
        /// 
        /// '''
        /// 
        pub fn set_name (&mut self, nouv:&str) {
            self.nom = nouv.to_string();
        }
    }

    impl Epee {

        /// Retourne la valeur de l'attribut 'nom'
        /// d'une instance.
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * &String
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// println!("{}",sword.get_name());
        /// 
        /// '''
        /// 
        pub fn get_name (&self) -> &String {
            &self.nom
        }
    }

    impl Epee {

        /// Assigne une valeur à l'attribut 'puissance' d'une
        /// instance.
        /// 
        /// #Arguments
        /// 
        /// * 'pwr'
        /// 
        /// #Retourne
        /// 
        /// * ( )
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// 
        /// sword.set_puissance(100);
        /// 
        /// '''
        /// 
        pub fn set_puissance(&mut self, pwr: i16) {
            self.puissance = pwr;
        }
    }

    impl Epee {

        /// Retourne la valeur de l'attribut 'puissance' d'une
        /// instance.
        /// 
        /// #Arguments
        /// 
        /// * Aucuns
        /// 
        /// #Retourne
        /// 
        /// * i16
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let sword = Epee::default();
        /// 
        /// println!("{}",sword.get_puissance());
        /// 
        /// '''
        /// 
        pub fn get_puissance (&self) -> i16{
            self.puissance
        }
    }

/// Bouclier modélise une arme défensive qui est un attribut d'une instance StExupery.
/// 
/// # Attributs:
/// 
/// + *nom* (String): représente un nom fictif.
/// + *def (i16)*: valeur de la capacité défensive de l'instance.
/// 
/// # Exemples
/// 
/// '''
/// let shield = Bouclier::default();
/// '''
/// 
#[derive(Clone)]
pub struct Bouclier {
    nom: String,
    def: i16,
}

impl fmt::Display for Bouclier {

        /// Définition de fmt du trait 'Display' pour permettre d'afficher des informations
        /// sur les attributs d'une instance 'Bouclier' dans le terminal
        /// 
        /// #Arguments
        /// 
        /// *f: &mut fmt::Formatter<'_>
        /// 
        /// #Retourne
        /// 
        /// * fmt::Result
        /// 
        /// # Exemples
        /// 
        /// '''
        /// let shield = Bouclier::default();
        /// println!("{}", shield);
        /// 
        /// '''
        /// 
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} a le {} {} avec puissance {}! ","St-Exupéry".truecolor(212,151,11), "bouclier".truecolor(87, 247, 87),self.get_name(),self.get_def().to_string().yellow())
        }
    }

    impl Default for Bouclier {

    /// Créer une nouvelle instance et assigne des valeurs
    /// fixent aux attributs.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let s = Bouclier::default();
    /// 
    /// '''
    /// 
    fn default() -> Self{
        Bouclier {
            nom: "Tin foil".to_string(),
            def: 2
        }
    }
}

impl Bouclier {

    /// Créer une nouvelle instance et assigne des valeurs précisées
    /// aux attributs.
    /// 
    /// #Arguments
    /// 
    /// * 'nom', 'defense'
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let s = Bouclier::new("Styrofoam plate", 0);
    /// 
    /// '''
    /// 
    pub fn new(nom: &str, defense: i16) -> Self {
        Self {
            nom: nom.to_string(),
            def: defense
        }
    }
}

impl Bouclier {

    /// Retourne la valeur de l'attibut 'nom' d'une 
    /// instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &String
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let shield = Bouclier::default();
    /// 
    /// println!("{}",shield.get_name());
    /// 
    /// '''
    /// 
    pub fn get_name(&self) -> &String {
        &self.nom
    }
}

impl Bouclier {

    /// Assigne une valeur précisée à l'attribut 'nom'
    /// d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * 'nouv'
    /// 
    /// #Retourne
    /// 
    /// * ( )
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let shield = Bouclier::default();
    /// 
    /// shield.set_name("Iron Shield");
    /// 
    /// '''
    /// 
    pub fn set_name(&mut self, nouv: &str) {
        self.nom = nouv.to_string();
    }
}

impl Bouclier {

    /// Retourne la valeur de l'attribut 'def'
    /// pour l'instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * i16
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let shield = Bouclier::default();
    /// 
    /// println!(shield.get_def());
    /// 
    /// '''
    /// 
    pub fn get_def(&self) -> i16 {
        self.def
    }
}

impl Bouclier {

    /// Assigne une valeur précisée à l'attribut 'def' de
    /// l'instance.
    /// 
    /// #Arguments
    /// 
    /// * 
    /// 
    /// #Retourne
    /// 
    /// * 
    /// 
    /// # Exemples
    /// 
    /// '''
    /// 
    /// 
    /// '''
    /// 
    pub fn set_def(&mut self,defense:i16) {
        self.def=defense;
    }
}

/// Loot modélise un entrepôt d'instances uniques Epee et Bouclier.
/// 
/// # Attributs:
/// 
/// + *swords* (array): contient 20 instances uniques du modèle Epee en ordre
/// croissant de puissance.
/// + *shields* (array): contient 20 instances uniques du modèle Shield en ordre 
/// croissant de défense.
/// 
/// # Exemples
/// 
/// '''
/// let explore_chest = Loot::default();
/// '''
/// 
pub struct Loot {
    swords: [Epee;20],
    shields: [Bouclier;20]    
}

impl Default for Loot {
    /// Initialise les attributs d'une instance avec des valeurs fixes.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let explore_chest = Loot::default();
    /// 
    /// '''
    /// 
    fn default() -> Self {
        Self {
            swords :[
            Epee::new("Pocket knife",8),
            Epee::new("Paring blade",8),
            Epee::new("Turkey trimmer",8),
            Epee::new("Utility blade",8),
            Epee::new("Cleaver",10),
            Epee::new("Render",10),
            Epee::new("Cimeter blade",10),
            Epee::new("Carver blade",10),
            Epee::new("Naikiri",13),
            Epee::new("Santoku blade",13),
            Epee::new("Machete",13),
            Epee::new("Latin blade",16),
            Epee::new("Barong",16),
            Epee::new("Bolo blade",16),
            Epee::new("Kukri machete",19),
            Epee::new("Wakizashi sword",19),
            Epee::new("Khanda double-edge",19),
            Epee::new("Claymore",19),
            Epee::new("Katana",19),
            Epee::new("Ghostwalker blade",22)],
            shields:[
            Bouclier::new("Pizza tray",2),
            Bouclier::new("Non-stick skillet",2),
            Bouclier::new("Stainless sauce pot",2),
            Bouclier::new("Cast-iron skillet",3),
            Bouclier::new("Riot shield",3),
            Bouclier::new("Ceremonial shield",3),
            Bouclier::new("Kite shield",3),
            Bouclier::new("Heater shield",3),
            Bouclier::new("Pavise",3),
            Bouclier::new("Tower shield",4),
            Bouclier::new("Steel shield",4),
            Bouclier::new("Targe shield",4),
            Bouclier::new("Mindanao shield",4),
            Bouclier::new("Dome shield",4),
            Bouclier::new("Aspis shield",4),
            Bouclier::new("Alumina shield",5),
            Bouclier::new("Lonsdaleite shield",5),
            Bouclier::new("Silicon carbide shield",5),
            Bouclier::new("Boron carbide shield",5),
            Bouclier::new("Graphene composite",6)]
        }
    }
}

impl Loot {
    /// Retourne une référence à l'attribut 'swords' d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &[Epee]
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let explore_chest = Loot::default();
    /// 
    /// println!(explore_chest.get_swords()[0]);
    /// 
    /// '''
    /// 
    pub fn get_swords(&self) -> &[Epee]{
        &self.swords
    }
}

impl Loot {
    /// Retourne une référence à l'attribut 'shields' d'une instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &[Bouclier]
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let explore_chest = Loot::default();
    /// 
    /// println!(explore_chest.get_shields()[0]);
    /// 
    /// '''
    /// 
    pub fn get_shields(&self) -> &[Bouclier] {
        &self.shields
    }
}

impl Loot {
    /// Retourne une référence vers une instance Epee dans l'attribut 'swords',
    /// choisit de manière aléatoire.  Tente de diminuer les chances d'obtenir
    /// une arme au-dessus de l'index 6 (armes plus puissantes). 
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &Epee
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let explore_chest = Loot::default();
    /// 
    /// let new_sword = explore_chest.find_sword().clone();
    /// 
    /// '''
    /// 
    pub fn find_sword(&self) -> &Epee {
        let rng = rand::rng;
        let mut chance1:usize = rng().random_range(0..19);

        if chance1 >= 7{
            if rng().random_range(1..6) == 1 {
                chance1 = rng().random_range(7..19);
            } else {
                chance1 = rng().random_range(0..7);
            }
        }
        &self.get_swords()[chance1]
    }
}

impl Loot {
    /// Retourne une référence vers une instance Epee dans l'attribut 'shields',
    /// choisit de manière aléatoire.  Tente de diminuer les chances d'obtenir
    /// un bouclier au-dessus de l'index 8 (boucliers plus puissants). 
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * &Bouclier
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let explore_chest = Loot::default();
    /// 
    /// let new_shield = explore_chest.find_shield().clone();
    /// 
    /// '''
    /// 
    pub fn find_shield(&self) -> &Bouclier {
        let rng = rand::rng;
        let mut chance1:usize = rng().random_range(0..19);

        if chance1 >= 9{
            if rng().random_range(1..6) == 1 {
                chance1 = rng().random_range(9..19);
            } else {
                chance1 = rng().random_range(0..9);
            }
        }
        &self.get_shields()[chance1]
    }
}

/// Potion modélise un élément de l'attribut vector 'inventaire' d'une instance StExupery.
/// Cet item permet au joueur de rehausser la valeur de l'attribut 'vie' d'une instance
/// StExupery.
/// 
/// # Attributs:
/// 
/// + *puissance* (i16): représente la capacité de guérison d'un joueur.
///
/// 
/// # Exemples
/// 
/// '''
/// let potion = Potion::default();
/// '''
/// 
#[derive(Clone)]
pub struct Potion {
    puissance: i16
}

impl Default for Potion {
    /// Initialise l'attribut d'une nouvelle instance Potion avec 
    /// une valeur fixe.
    /// 
    /// #Arguments
    /// 
    /// *Aucuns
    /// 
    /// #Retourne
    /// 
    /// * 'Self'
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let potion = Potion::default();
    /// 
    /// '''
    /// 
    fn default()-> Self {
        Self {
            puissance: 35
        }
    }
}

impl Potion{
    /// Retourne la valeur de l'attribut 'puissance' pour une instance.
    /// 
    /// #Arguments
    /// 
    /// * Aucuns
    /// 
    /// #Retourne
    /// 
    /// * i16
    /// 
    /// # Exemples
    /// 
    /// '''
    /// let potion = Potion::default();
    /// 
    /// println!("{}",potion.get_puissance());
    /// 
    /// '''
    /// 
    pub fn get_puissance(&self) -> i16 {
        self.puissance
    }
} 