pub struct Potion {
    puissance: i16
}

impl Default for Potion {
    fn default()-> Self {
        Self {
            puissance: 25
        }
    }
}

impl Potion{
    pub fn get_puissance(&self) -> i16 {
        self.puissance
    }
} 