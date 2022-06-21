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

const OBJ_ALTAS_COL_COUNT: u32 = 8;
pub const SINGLE_OBJ_PIXELS: u32 = assets::objects::OBJECTS_PNG_WIDTH / OBJ_ALTAS_COL_COUNT;

pub fn draw_item(item_type: Item, x: i32, y: i32) {
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
