use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn buddy_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_jump03_02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_glide_loop"));
        macros::PLAY_SE(fighter, Hash40::new("vc_buddy_special_h03_vc"));
    }
}

unsafe extern "C" fn buddy_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_buddy_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_buddy_attackair_b03"));
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_l02"));
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_s02"));
    }
}

unsafe extern "C" fn buddy_glidelandingsfx(fighter: &mut L2CAgentBase) {
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
        .acmd("sound_glidelanding", buddy_glidelandingsfx, Priority::Low)
        .acmd("sound_glideattack", buddy_glideattacksfx, Priority::Low)
        .acmd("sound_glidestart", buddy_glidestartsfx, Priority::Low);
}
