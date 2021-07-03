//! Reference:  https://stattrek.com/online-calculator/binomial.aspx

const BASE_ESCAPE_CHANCE: f64 = 0.04;
const BASE_NUM_TRIES: u8 = 3;

// These are Roman Numerals, not acronyms
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tier {
    I,
    II,
    III,
}

#[derive(Debug)]
pub struct HookEscapeChanceSettings {
    /// Whether the user has slippery meat
    pub slippery_meat: Option<Tier>,

    /// Each player's up the ante perk
    pub up_the_ante: [Option<Tier>; 4],

    /// The # of salty lips
    pub num_salty_lips: u8,

    /// The # of alive survivors
    pub num_alive_survivors: u8,
}

impl HookEscapeChanceSettings {
    pub fn new() -> Self {
        Self {
            slippery_meat: None,

            up_the_ante: [None; 4],

            num_salty_lips: 0,

            num_alive_survivors: 4,
        }
    }

    pub fn calculate(&self) -> f64 {
        let mut escape_chance = BASE_ESCAPE_CHANCE;
        let mut num_tries = BASE_NUM_TRIES;

        if let Some(tier) = self.slippery_meat {
            escape_chance += match tier {
                Tier::I => 0.02,
                Tier::II => 0.03,
                Tier::III => 0.04,
            };
            num_tries += 3;
        }

        for tier in self.up_the_ante.iter().flatten() {
            escape_chance += match tier {
                Tier::I => 0.01 * f64::from(num::clamp(self.num_alive_survivors, 1, 4) - 1),
                Tier::II => 0.02 * f64::from(num::clamp(self.num_alive_survivors, 1, 4) - 1),
                Tier::III => 0.03 * f64::from(num::clamp(self.num_alive_survivors, 1, 4) - 1),
            };
        }

        escape_chance += 0.03 * f64::from(num::clamp(self.num_salty_lips, 0, 4));

        // Begin binomial calculation
        {
            let x = 0u8;

            let ret: f64 = (0..=x)
                .map(|i| {
                    let binomial = num::integer::binomial(num_tries, i);

                    f64::from(binomial)
                        * escape_chance.powi(i32::from(i))
                        * (1.0 - &escape_chance).powi(i32::from(num_tries - i))
                })
                .sum();

            // This is the probability of failure. Invert it to get the success rate.
            1.0 - ret
        }
    }

    pub fn increment_slippery_meat_tier(&mut self) {
        self.slippery_meat = match self.slippery_meat {
            None => Some(Tier::I),
            Some(Tier::I) => Some(Tier::II),
            Some(Tier::II) => Some(Tier::III),
            Some(Tier::III) => Some(Tier::III),
        };
    }

    pub fn decrement_slippery_meat_tier(&mut self) {
        self.slippery_meat = match self.slippery_meat {
            None => None,
            Some(Tier::I) => None,
            Some(Tier::II) => Some(Tier::I),
            Some(Tier::III) => Some(Tier::II),
        };
    }

    pub fn increment_up_the_ante_tier(&mut self, index: u8) {
        if let Some(up_the_ante) = self.up_the_ante.get_mut(usize::from(index)) {
            *up_the_ante = match up_the_ante {
                None => Some(Tier::I),
                Some(Tier::I) => Some(Tier::II),
                Some(Tier::II) => Some(Tier::III),
                Some(Tier::III) => Some(Tier::III),
            };
        }
    }

    pub fn decrement_up_the_ante_tier(&mut self, index: u8) {
        if let Some(up_the_ante) = self.up_the_ante.get_mut(usize::from(index)) {
            *up_the_ante = match up_the_ante {
                None => None,
                Some(Tier::I) => None,
                Some(Tier::II) => Some(Tier::I),
                Some(Tier::III) => Some(Tier::II),
            };
        }
    }
}

impl Default for HookEscapeChanceSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_escape_chance() {
        let chance = HookEscapeChanceSettings::new().calculate();
        assert!((chance - 0.115264).abs() < 0.000001)
    }

    #[test]
    fn tier_3_slippery_meat() {
        let mut settings = HookEscapeChanceSettings::new();
        settings.slippery_meat = Some(Tier::III);
        let chance = settings.calculate();
        assert!((chance - 0.39364499865).abs() < 0.00000000001)
    }

    #[test]
    fn tier_3_slippery_meat_up_the_ante() {
        let mut settings = HookEscapeChanceSettings::new();
        settings.slippery_meat = Some(Tier::III);
        settings.up_the_ante[0] = Some(Tier::III);
        let chance = settings.calculate();
        assert!((chance - 0.673059626631).abs() < 0.00000000001)
    }

    #[test]
    fn salty_lips() {
        let mut settings = HookEscapeChanceSettings::new();
        settings.num_salty_lips = 4;
        let chance = settings.calculate();
        assert!((chance - 0.407296).abs() < 0.00000000001)
    }
}
