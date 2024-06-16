use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn buddy_glidestartgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 5.0, true);
    macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.045, /*B*/ 0.1);
}

unsafe extern "C" fn buddy_glide2gfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}
unsafe extern "C" fn buddy_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2.0, 4.0, 0, 0, 0, 0, 0.8, true);
    }
}  

unsafe extern "C" fn buddy_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent
        .acmd("effect_glidelanding", buddy_glidelandinggfx, Priority::Low)
        .acmd("effect_glideattack", buddy_glideattackgfx, Priority::Low)
        .acmd("effect_glidewing", buddy_glide2gfx, Priority::Low)
        .acmd("effect_glidestartgfx", buddy_glidestartgfx, Priority::Low);
}
