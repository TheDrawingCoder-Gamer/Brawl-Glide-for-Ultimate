use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

unsafe extern "C" fn metaknight_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_appeal_s03"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start")); //Index 112
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop")); //Index 113
    }
}

unsafe extern "C" fn metaknight_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_attack100_03"));
    }
}

unsafe extern "C" fn metaknight_glidelandingsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

unsafe extern "C" fn metaknight_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_turn"));
    }
}    

/*#[acmd_script(//SpecialNStart
    agent = "metaknight", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_neutralbstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "metaknight", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_neutralbairstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialSStart
    agent = "metaknight", 
    script = "sound_specialsstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_sidebstartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_s01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_jump01"));
    }
}*/

unsafe extern "C" fn metaknight_upbsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
    }
}    

unsafe extern "C" fn metaknight_upbloopsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
       macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
    }
}   

pub fn install(agent: &mut Agent) {
    agent 
        .acmd("sound_glidestart", metaknight_glidestartsfx, Priority::Low)
        .acmd("sound_glideattack", metaknight_glideattacksfx, Priority::Low)
        .acmd("sound_glidelanding", metaknight_glidelandingsfx, Priority::Low)
        .acmd("sound_glideend", metaknight_glideendsfx, Priority::Low)
        .acmd("sound_specialhi", metaknight_upbsfx, Priority::Low)
        .acmd("sound_specialhiloop", metaknight_upbloopsfx, Priority::Low);
}
