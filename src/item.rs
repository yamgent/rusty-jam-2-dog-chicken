use crate::assets;
use crate::wasm4::*;

#[repr(u8)]
#[derive(Clone, Copy)]
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

impl Item {
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
    unsafe { *DRAW_COLORS = 0x123 };

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
