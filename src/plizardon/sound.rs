use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn plizardon_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_start")); //Index 42
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_loop")); //Index 43    
    }
}

unsafe extern "C" fn plizardon_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_plizardon_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_tailswing"));
    }
}

unsafe extern "C" fn plizardon_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

unsafe extern "C" fn plizardon_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}   

pub fn install(agent: &mut Agent) {
    agent
        .acmd("sound_glidestart", plizardon_glidestartsfx, Priority::Low)
        .acmd("sound_glideattack", plizardon_glideattacksfx, Priority::Low)
        .acmd("sound_glidelanding", plizardon_glidelandingsfx, Priority::Low)
        .acmd("sound_glideendsfx", plizardon_glideendsfx, Priority::Low)
        ;
}
