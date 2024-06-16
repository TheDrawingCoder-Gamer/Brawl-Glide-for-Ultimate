use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn ridley_glidestartgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 7.97, true);
    macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.39, /*G*/ 0.045, /*B*/ 1.55);
}

unsafe extern "C" fn ridley_glidegfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

unsafe extern "C" fn ridley_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ridley_grabbing_hold"), Hash40::new("havel"), -1.0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ridley_scratch"), Hash40::new("ridley_scratch"), Hash40::new("top"), -5.0, 8.0, 8.0, -124.0, 72.5, 70, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5)
    }
}  

unsafe extern "C" fn ridley_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent
        .acmd("effect_glidestart", ridley_glidestartgfx, Priority::Low)
        .acmd("effect_glidewing", ridley_glidegfx, Priority::Low)
        .acmd("effect_glideattack", ridley_glideattackgfx, Priority::Low)
        .acmd("effect_glidelanding", ridley_glidelandinggfx, Priority::Low);
}
