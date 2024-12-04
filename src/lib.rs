use num::clamp;
use std::fmt::{self, Display};

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }

    pub fn update_quality_by(&mut self, n: i32) {
        self.quality = clamp(self.quality + n, 0, 50);
    }

    pub fn pass_a_day(&mut self) {
        self.sell_in -= 1;
    }

    pub fn expired(&self) -> bool {
        return self.sell_in < 0;
    }

    pub fn update(&mut self) {
        if self.name == "Sulfuras, Hand of Ragnaros" {
            return;
        }

        self.pass_a_day();

        self.update_quality_by(match self.name.as_str() {
            "Aged Brie" => {
                if self.expired() {
                    2
                } else {
                    1
                }
            }
            "Backstage passes to a TAFKAL80ETC concert" => {
                if self.expired() {
                    -self.quality
                } else {
                    match self.sell_in {
                        ..=4 => 3,
                        ..=9 => 2,
                        _ => 1,
                    }
                }
            }
            "Conjured Mana Cake" => {
                if self.expired() {
                    -4
                } else {
                    -2
                }
            }
            _ => {
                if self.expired() {
                    -2
                } else {
                    -1
                }
            }
        });
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }
}
