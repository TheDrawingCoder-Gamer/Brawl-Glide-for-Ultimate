use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

// Glide Start
unsafe extern "C" fn trail_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_trail_jump03"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_jump02"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_throw_shiny"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_special_h02"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_glide_loop"));
    }
}

// Glide attack
unsafe extern "C" fn trail_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_trail_attackair_b03"));
        macros::PLAY_SE(fighter, Hash40::new("se_trail_attackhard_l02"));
        macros::PLAY_SE(fighter, Hash40::new("se_trail_attackhard_s02"));
    }
}

// Glide Landing
unsafe extern "C" fn trail_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

pub fn install(agent: &mut Agent) {
    agent
        .acmd("sound_glidelanding", trail_glidelandingsfx, Priority::Low)
        .acmd("sound_glideattack", trail_glideattacksfx, Priority::Low)
        .acmd("sound_glidestart", trail_glidestartsfx, Priority::Low);
}
