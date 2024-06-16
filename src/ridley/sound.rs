use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn ridley_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_glide_start")); //Index 87
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_special_h01"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_glide_loop")); //Index 88
    }
}

unsafe extern "C" fn ridley_glideattacksfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_special_s01"));
    }
}

unsafe extern "C" fn ridley_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

unsafe extern "C" fn ridley_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ridley_glide_loop"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_jump02"))
    }
}

pub fn install(agent: &mut Agent) {
    agent
        .acmd("sound_glidestart", ridley_glidestartsfx, Priority::Low)
        .acmd("sound_glideattack", ridley_glideattacksfx, Priority::Low)
        .acmd("sound_glideend", ridley_glideendsfx, Priority::Low)
        .acmd("sound_glidelanding", ridley_glidelandingsfx, Priority::Low);
}
