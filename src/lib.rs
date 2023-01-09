#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::if_same_then_else
)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use smash::lib::lua_const::*;

mod metaknight;
mod pit;
mod plizardon;
mod pitb;
mod ridley;
mod buddy;
mod palutena;
mod trail;
pub mod glide;
mod glide_checks;
mod jump_aerial;
mod fly;
mod utils;

pub fn is_glider(kind: i32) -> bool {
    [
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB,
        *FIGHTER_KIND_PLIZARDON,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_TRAIL,
        *FIGHTER_KIND_PALUTENA,
    ].contains(&kind)
}

#[skyline::main(name = "brawl_glide_port")]
pub fn main() {
    metaknight::install();
    pit::install();
    plizardon::install();
    pitb::install();
    ridley::install();
    buddy::install();
    palutena::install();
    trail::install();
    glide::install();
    jump_aerial::install();
    fly::install();
    glide_checks::install();
}