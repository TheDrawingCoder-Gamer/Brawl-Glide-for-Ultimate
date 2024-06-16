use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn trail_glidestartgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 4.5, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.53);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_special_all_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.52);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.52);
    }
}

unsafe extern "C" fn trail_glidegfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

unsafe extern "C" fn trail_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x13dcc5dae1), Hash40::new_raw(0x1345cc8b5b), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_flare"), false, true);
        macros::AFTER_IMAGE_OFF(fighter, 6);
    }
}  

unsafe extern "C" fn trail_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent
     .acmd("effect_glidelanding", trail_glidelandinggfx, Priority::Low)
     .acmd("effect_glideattack", trail_glideattackgfx, Priority::Low)
     .acmd("effect_glidewing", trail_glidegfx, Priority::Low)
     .acmd("effect_glidestart", trail_glidestartgfx, Priority::Low)
     ;
}
