use bracket_lib::prelude::{to_cp437, FontCharType, RGBA};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub(crate) struct Renderable {
    pub glyph: FontCharType,
    pub fg_col: RGBA,
    pub bg_col: RGBA,
}

impl Renderable {
    pub fn new(glyph: char, fg_col: (u8, u8, u8), bg_col: (u8, u8, u8)) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg_col: RGBA::named(fg_col),
            bg_col: RGBA::named(bg_col),
        }
    }
}
