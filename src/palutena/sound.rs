use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn palutena_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_dash_start"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_jump02"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide")); //75
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide_loop")); //76
    }
}

unsafe extern "C" fn palutena_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_special_l03"));
    }
}

unsafe extern "C" fn palutena_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

unsafe extern "C" fn palutena_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_escapeair"));
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_jump01"));
    }
}   

pub fn install(agent: &mut Agent) {
    agent
        .acmd("sound_glidestart", palutena_glidestartsfx, Priority::Low)
        .acmd("sound_glideattack", palutena_glideattacksfx, Priority::Low)
        .acmd("sound_glidelanding", palutena_glidelandingsfx, Priority::Low)
        .acmd("sound_glideend", palutena_glideendsfx, Priority::Low);
}
