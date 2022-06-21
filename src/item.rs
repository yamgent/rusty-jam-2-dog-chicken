use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use strum_macros::EnumIter;

use crate::assets;
use crate::wasm4::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
pub enum Item {
    Water = 0,
    Tree,
    IronOre,
    Ball,
    Fire,
    Cloth,
    Cow,
    Bottle,
    Comb,
    Plate,
    Spring,
    Bush,
    Steel,
    Axe,
    Twig,
    Flag,
    Medal,
    Loyalty,
    Milk,
    Grass,
    Bird,
    Beef,
    Cat,
    Fur,
    Dog,
    Chicken,
    DogChicken,
    Strong,
    Tong,
    Crab,
    Rust,
    Rustacean,
    Rum,
    Pirate,
    PirateRustacean,
    Swan,
    FoodChain,
    Hawk,
    Meal,
    Business,
}

static COMBO_DATA: Lazy<FxHashMap<(Item, Item), Item>> = Lazy::new(|| {
    let mut m = FxHashMap::default();

    [
        ((Item::IronOre, Item::Fire), Item::Steel),
        ((Item::Tree, Item::Steel), Item::Axe),
        ((Item::Tree, Item::Axe), Item::Twig),
        ((Item::Cloth, Item::Twig), Item::Flag),
        ((Item::Cloth, Item::Steel), Item::Medal),
        ((Item::Flag, Item::Medal), Item::Loyalty),
        ((Item::Cow, Item::Bottle), Item::Milk),
        ((Item::Axe, Item::Bush), Item::Grass),
        ((Item::Grass, Item::Twig), Item::Bird),
        ((Item::Fire, Item::Cow), Item::Beef),
        ((Item::Beef, Item::Milk), Item::Cat),
        ((Item::Comb, Item::Cat), Item::Fur),
        ((Item::Fur, Item::Loyalty), Item::Dog),
        ((Item::Bird, Item::Plate), Item::Chicken),
        ((Item::Dog, Item::Chicken), Item::DogChicken),
        ((Item::Ball, Item::Steel), Item::Strong),
        ((Item::Steel, Item::Spring), Item::Tong),
        ((Item::Strong, Item::Tong), Item::Crab),
        ((Item::Steel, Item::Water), Item::Rust),
        ((Item::Crab, Item::Rust), Item::Rustacean),
        ((Item::Bottle, Item::Water), Item::Rum),
        ((Item::Flag, Item::Rum), Item::Pirate),
        ((Item::Pirate, Item::Rustacean), Item::PirateRustacean),
        ((Item::Water, Item::Bird), Item::Swan),
        ((Item::Cat, Item::Bird), Item::FoodChain),
        ((Item::Strong, Item::Bird), Item::Hawk),
        ((Item::Beef, Item::Plate), Item::Meal),
        ((Item::Meal, Item::FoodChain), Item::Business),
    ]
    .into_iter()
    .for_each(|(key, value)| {
        if m.contains_key(&(key.0, key.1)) || m.contains_key(&(key.1, key.0)) {
            panic!("This input is a duplicate: {:?}", key);
        }
        m.insert(key, value);
    });

    m
});

pub static STARTING_ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
    vec![
        Item::Water,
        Item::Tree,
        Item::IronOre,
        Item::Ball,
        Item::Fire,
        Item::Cloth,
        Item::Cow,
        Item::Bottle,
        Item::Comb,
        Item::Plate,
        Item::Spring,
        Item::Bush,
    ]
});

impl Item {
    pub fn combine(&self, other: &Item) -> Option<Item> {
        [(*self, *other), (*other, *self)]
            .iter()
            .find_map(|key| COMBO_DATA.get(key).copied())
    }

    pub fn name(&self) -> &'static str {
        match self {
            Item::Water => "Water",
            Item::Tree => "Tree",
            Item::IronOre => "Iron Ore",
            Item::Ball => "Ball",
            Item::Fire => "Fire",
            Item::Cloth => "Cloth",
            Item::Cow => "Cow",
            Item::Bottle => "Bottle",
            Item::Comb => "Comb",
            Item::Plate => "Plate",
            Item::Spring => "Spring",
            Item::Bush => "Bush",
            Item::Steel => "Steel",
            Item::Axe => "Axe",
            Item::Twig => "Twig",
            Item::Flag => "Flag",
            Item::Medal => "Medal",
            Item::Loyalty => "Loyalty",
            Item::Milk => "Milk",
            Item::Grass => "Grass",
            Item::Bird => "Bird",
            Item::Beef => "Beef",
            Item::Cat => "Cat",
            Item::Fur => "Fur",
            Item::Dog => "Dog",
            Item::Chicken => "Chicken",
            Item::DogChicken => "DogChicken",
            Item::Strong => "Strong",
            Item::Tong => "Tong",
            Item::Crab => "Crab",
            Item::Rust => "Rust",
            Item::Rustacean => "Rustacean",
            Item::Rum => "Rum",
            Item::Pirate => "Pirate",
            Item::PirateRustacean => "PirateRustacean",
            Item::Swan => "Swan",
            Item::FoodChain => "Food Chain",
            Item::Hawk => "Hawk",
            Item::Meal => "Meal",
            Item::Business => "Business",
        }
    }
}

const OBJ_ALTAS_COL_COUNT: u32 = 8;
pub const SINGLE_OBJ_PIXELS: u32 = assets::objects::OBJECTS_PNG_WIDTH / OBJ_ALTAS_COL_COUNT;

pub fn draw_item(item_type: Item, x: i32, y: i32) {
    // TODO: Better color scheme?
    unsafe { *DRAW_COLORS = 0x234 };

    let src_x = ((item_type as u32) % OBJ_ALTAS_COL_COUNT) * SINGLE_OBJ_PIXELS;
    let src_y = ((item_type as u32) / OBJ_ALTAS_COL_COUNT) * SINGLE_OBJ_PIXELS;

    blit_sub(
        &assets::objects::OBJECTS_PNG,
        x,
        y,
        SINGLE_OBJ_PIXELS,
        SINGLE_OBJ_PIXELS,
        src_x,
        src_y,
        assets::objects::OBJECTS_PNG_WIDTH,
        assets::objects::OBJECTS_PNG_FLAGS,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hash::FxHashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn all_items_reachable() {
        #[derive(Debug)]
        struct Recipe {
            input: FxHashSet<Item>,
            output: Item,
        }

        let mut reachable_items = FxHashSet::default();
        let _a = FxHashMap::from_iter([(1, 2), (1, 3)].into_iter());

        let mut items_to_process = STARTING_ITEMS.clone();
        let mut recipes_to_process = COMBO_DATA
            .iter()
            .map(|(key, value)| Recipe {
                input: FxHashSet::from_iter([key.0, key.1].into_iter()),
                output: *value,
            })
            .collect::<Vec<_>>();

        while let Some(item) = items_to_process.pop() {
            reachable_items.insert(item);

            recipes_to_process.iter_mut().for_each(|recipe| {
                recipe.input.remove(&item);
            });
            recipes_to_process
                .iter()
                .filter(|recipe| recipe.input.is_empty())
                .for_each(|recipe| {
                    items_to_process.push(recipe.output);
                });
            recipes_to_process.retain(|recipe| recipe.input.len() > 0);
        }

        if !recipes_to_process.is_empty() {
            panic!(
                "We still have some recipes remaining and they are not usable: {:?}",
                recipes_to_process
            );
        }

        let all_items = FxHashSet::from_iter(Item::iter());
        assert_eq!(
            reachable_items,
            all_items,
            "Some stuff are not reachable: {:?}",
            all_items.difference(&reachable_items)
        );
    }
}
