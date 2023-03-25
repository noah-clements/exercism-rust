pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => {
                let new_mana = 
                    if self.level >= 10 { Some(100) } else { None };
                let new_player = Player{health: 100, mana: new_mana, level: self.level};
                Some(new_player)
            }
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            self.health = self.health.saturating_sub(mana_cost);
            0
        } else {
            let mana = self.mana.unwrap();
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            } else {
                0
            }
        }
    }
}
