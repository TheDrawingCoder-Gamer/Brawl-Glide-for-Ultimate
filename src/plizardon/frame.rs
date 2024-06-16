use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{sv_information};

unsafe extern "C" fn plizardon_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK,
            *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_glide_loop"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            KineticModule::clear_speed_all(fighter.module_accessor);
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
    }
}        

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, plizardon_opff);
}
