use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{sv_information};
use crate::glide::*;

unsafe extern "C" fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let params = GlideParams::get(fighter);
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
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        };
        /*if [
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        ].contains(&status_kind) {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 13.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            }
            if MotionModule::frame(fighter.module_accessor) > 15.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            if MotionModule::frame(fighter.module_accessor) > 30.0 {
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);*/
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
            WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: params.speed_mul_start, y: 0.0, z: 0.0});
            }   
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
            WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: params.speed_mul_start, y: 0.0, z: 0.0});
            }    
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
            if MotionModule::frame(fighter.module_accessor) > 31.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            };
        }
    }
}        

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, metaknight_opff);
}
