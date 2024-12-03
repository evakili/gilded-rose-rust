use gilded_rose_rust::{GildedRose, Item};

use std::fmt::Write;

fn pass_days(days: i32, items: Vec<Item>) -> String {
    let mut rose = GildedRose::new(items);

    let mut output = String::new();
    for i in 0..=days {
        write!(&mut output, "-------- day {} --------\n", i)
            .expect("Error in writing to the output: day header");
        write!(&mut output, "name, sellIn, quality\n")
            .expect("Error in writing to the output: item header");
        for item in &rose.items {
            write!(&mut output, "{}\n", item).expect("Error in writing to the output: item");
        }
        write!(&mut output, "\n").expect("Error in writing to the output: new line");
        rose.update_quality();
    }

    return output;
}

#[test]
fn test_passing_30_days_in_gilded_rose() {
    insta::assert_snapshot!(pass_days(30, vec![
                    Item::new("+5 Dexterity Vest", 10, 20),
                    Item::new("Aged Brie", 2, 0),
                    Item::new("Elixir of the Mongoose", 5, 7),
                    Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
                    Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
                    Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
                    Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
                    Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
                    Item::new("Conjured Mana Cake", 3, 6),
                ]));
}
