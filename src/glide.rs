use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*,
    smashline::*
};
use smash::phx::Vector3f;
use std::f32::consts::PI;

/*Contributors:
- Liam Estares (LKE Studios)
- Ben Hall (arthur!)
- WuBoytH*/

/*In Brawl, values for gliding were stored and retrieved from the character's param table. This is
 also true for Steve's param vl in Ultimate. However, since we can't really create working params in 
 the character's vl file, we would need to declare the values and variables here instead.*/

pub struct GlideParams {
    pub angle_max_up : f32, //#0 Max Upward Angle
    pub angle_max_down : f32, //#1 Max Downward Angle
    pub v_glide_start : f32, //#2 V Speed added on GlideStart
    pub gravity_start : f32, //#3 Gravity multiplier on GlideStart
    pub speed_mul_start : f32, //#4 H speed multiplier on GlideStart
    pub base_speed : f32, //#5 Base Power/Speed
    pub speed_change : f32, //#6 Power Rate
    pub max_speed : f32, //#7 Maximum Speed
    pub end_speed : f32, //#8 End Speed
    pub gravity_accel : f32, //#9 Gravity Acceleration
    pub gravity_speed : f32, //#10 Gravity Max Speed
    pub angle_extra : f32, //#11 Angle stuff but unknown what this is for
    pub angle_more_speed : f32, //#12 Angle to gain more speed
    pub down_speed_add : f32, //#13 Max added H speed gained aiming downward
    pub unknown : f32, //#14 Unknown
    pub radial_stick : f32, //#15 Radial Stick Sensitivity
    pub up_angle_accel : f32, //#16 Upward angular acceleration
    pub down_angle_accel : f32, //#17 Downward angular acceleration
    pub max_angle_speed : f32, //#18 Maximum angular speed
    pub add_angle_speed : f32 //#19 Added angular speed for when stick is center
}

/*Glide "params" for all gliding characters. Checks the "FIGHTER_KIND" and applies those values 
to the specified character*/

impl GlideParams {
    pub fn get(fighter: &mut L2CFighterCommon) -> GlideParams {
        let kind = fighter.global_table[0x2].get_i32();
        if kind == *FIGHTER_KIND_METAKNIGHT {
            return GlideParams {
                angle_max_up : 80.0,
                angle_max_down : -70.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.7,
                speed_change : 0.04,
                max_speed : 2.2,
                end_speed : 0.7,
                gravity_accel : 0.03,
                gravity_speed : 0.6,
                angle_extra : 15.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.03,
                unknown : 0.15,
                radial_stick : 0.25,
                up_angle_accel : 0.55,
                down_angle_accel : 0.75,
                max_angle_speed : 7.0,
                add_angle_speed : 1.0
            };
        }
        if kind == *FIGHTER_KIND_PIT {
            return GlideParams {
                angle_max_up : 70.0,
                angle_max_down : -60.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.8,
                speed_change : 0.05,
                max_speed : 2.3,
                end_speed : 1.0,
                gravity_accel : 0.05,
                gravity_speed : 0.4,
                angle_extra : 0.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.01,
                unknown : 0.1,
                radial_stick : 0.25,
                up_angle_accel : 2.0,
                down_angle_accel : 0.5,
                max_angle_speed : 6.0,
                add_angle_speed : 0.15
            };
        }
        if kind == *FIGHTER_KIND_PITB {
            return GlideParams {
                angle_max_up : 70.0,
                angle_max_down : -65.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.8,
                speed_change : 0.05,
                max_speed : 2.3,
                end_speed : 1.0,
                gravity_accel : 0.05,
                gravity_speed : 0.4,
                angle_extra : 0.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.015,
                unknown : 0.1,
                radial_stick : 0.25,
                up_angle_accel : 0.7,
                down_angle_accel : 1.0,
                max_angle_speed : 6.5,
                add_angle_speed : 0.15
            };
        }
        if kind == *FIGHTER_KIND_PLIZARDON {
            return GlideParams {
                angle_max_up : 50.0,
                angle_max_down : -50.0,
                v_glide_start : 1.2,
                gravity_start : 0.15,
                speed_mul_start : 1.0,
                base_speed : 1.2,
                speed_change : 0.05,
                max_speed : 1.9,
                end_speed : 0.5,
                gravity_accel : 0.06,
                gravity_speed : 0.4,
                angle_extra : 0.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.01,
                unknown : 0.1,
                radial_stick : 0.25,
                up_angle_accel : 0.8,
                down_angle_accel : 0.4,
                max_angle_speed : 4.0,
                add_angle_speed : 0.15
            };
        }
        if kind == *FIGHTER_KIND_RIDLEY {
            return GlideParams {
                angle_max_up : 60.0,
                angle_max_down : -50.0,
                v_glide_start : 1.0,
                gravity_start : 0.0,
                speed_mul_start : 0.2,
                base_speed : 1.5,
                speed_change : 0.05,
                max_speed : 2.1,
                end_speed : 0.4,
                gravity_accel : 0.06,
                gravity_speed : 0.4,
                angle_extra : 0.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.015,
                unknown : 0.1,
                radial_stick : 0.25,
                up_angle_accel : 0.65,
                down_angle_accel : 0.8,
                max_angle_speed : 5.0,
                add_angle_speed : 0.15
            };
        }
        if kind == *FIGHTER_KIND_BUDDY {
            return GlideParams {
                angle_max_up : 65.0, //#0 Max Upward Angle
                angle_max_down : -55.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.2, //#4 H speed multiplier on GlideStart
                base_speed : 1.6, //#5 Base Power/Speed
                speed_change : 0.05, //#6 Power Rate
                max_speed : 2.1, //#7 Maximum Speed
                end_speed : 0.8, //#8 End Speed
                gravity_accel : 0.055, //#9 Gravity Acceleration
                gravity_speed : 0.5, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -20.0, //#12 Angle to gain more speed
                down_speed_add : 0.02, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.8, //#16 Upward angular acceleration
                down_angle_accel : 0.75, //#17 Downward angular acceleration
                max_angle_speed : 4.5, //#18 Maximum angular speed
                add_angle_speed : 0.2 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_PALUTENA {
            return GlideParams {
                angle_max_up : 75.0, //#0 Max Upward Angle
                angle_max_down : -65.0, //#1 Max Downward Angle
                v_glide_start : 0.0, //#2 V Speed added on GlideStart
                gravity_start : 0.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 0.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.7, //#5 Base Power/Speed
                speed_change : 0.05, //#6 Power Rate
                max_speed : 2.3, //#7 Maximum Speed
                end_speed : 0.8, //#8 End Speed
                gravity_accel : 0.04, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.02, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 1.6, //#16 Upward angular acceleration
                down_angle_accel : 0.65, //#17 Downward angular acceleration
                max_angle_speed : 6.5, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_TRAIL {
            return GlideParams {
                angle_max_up : 40.0, //#0 Max Upward Angle
                angle_max_down : -45.0, //#1 Max Downward Angle
                v_glide_start : 1.5, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.4, //#5 Base Power/Speed
                speed_change : 0.04, //#6 Power Rate
                max_speed : 2.0, //#7 Maximum Speed
                end_speed : 0.6, //#8 End Speed
                gravity_accel : 0.05, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -15.0, //#12 Angle to gain more speed
                down_speed_add : 0.015, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 1.3, //#16 Upward angular acceleration
                down_angle_accel : 0.95, //#17 Downward angular acceleration
                max_angle_speed : 3.0, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        else {
            // if fighter kind not defined, just use Meta Knight's params.
            return GlideParams {
                angle_max_up : 80.0,
                angle_max_down : -70.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.7,
                speed_change : 0.04,
                max_speed : 2.2,
                end_speed : 0.7,
                gravity_accel : 0.03,
                gravity_speed : 0.6,
                angle_extra : 15.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.03,
                unknown : 0.15,
                radial_stick : 0.25,
                up_angle_accel : 0.55,
                down_angle_accel : 0.75,
                max_angle_speed : 7.0,
                add_angle_speed : 1.0
            };
        }
    }
}

/*KineticUtility isn't a real function in Ultimate, so we had to make it down here.*/

mod KineticUtility {
    // Resets and enables the kinetic energy type.
    // Unknown why there are two vectors required by reset_energy
    pub unsafe fn reset_enable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32, energy_reset_type: i32, speed_vec: smash::phx::Vector2f, other_vec: smash::phx::Vector3f) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::reset_energy(energy, energy_reset_type, &speed_vec, &other_vec, module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(energy);
    }

    // Clears and disables the kinetic energy type
    pub unsafe fn clear_unable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::clear_speed(energy);
        smash::app::lua_bind::KineticEnergy::unable(energy);
    }
}

/*Skyline hook for GlideStart Status. Calls the appropriate motion_kind to use and applies params #2-4.*/

#[skyline::hook(replace = L2CFighterCommon_status_GlideStart)]
pub unsafe fn status_glidestart(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get(fighter);

    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: params.speed_mul_start, y: params.v_glide_start, z: 0.0 });
    KineticModule::mul_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: params.gravity_start, z: 0.0 }, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GlideStart_Main as *const () as _))
}

/*Glide's init status. Specifies the base speed and gravity speed of the glide. Also uses our modded
KineticUtility function here, which is one of the vital things in getting the glide the work.*/

pub unsafe extern "C" fn status_init_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let params = GlideParams::get(fighter);
    WorkModule::set_float(fighter.module_accessor, params.base_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    
    /* Second lua const under reset_enable_energy must be set to *ENERGY_STOP_RESET_TYPE_FREE as this is exactly what 
    Brawl used.*/
    let initial_speed = params.base_speed * lr;
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: initial_speed, y: 0.0}, Vector3f{x: initial_speed, y: 0.0, z: 0.0});
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    0.into()
}

/*The hook below checks if the glide has already been used in the air or not. If it has, glide is disabled until
you land. Also calls appropriate motion_kind for the main status of the glide. GlideDirection's animation must be set 
to frame 90.0 and motion rate set to 0.0 by default. In addition, it also calls a second motion_kind (partial motions) which solely 
controls the wings.

Unlike in Brawl, we don't specify the bone hash/ID for partial motions here. For Ultimate, that 
information is stored both in the motion_kind's entry in the fighter's motion_list.bin file and the animation itself. 
The GlideWing motion has animations on all the wing bones only, where the rest of the bones that AREN'T 
wings must have no keyframe data whatsoever. In GlideWing's motion_list entry, the unk: value beneath
the motion filename (a07glidewing.nuanmb) must be set to 4. Dunno what putting a number there actually entails,
but my theory is that it makes the game completely ignore bones that have no keyframe data which
is what we want in order to get partial motions to work properly. Of course, all gliding animations must have
their entries added in the character's motion_list file, and their filenames and directories added in 
the mod's config.json under "new-dir-files" brackets.*/

#[skyline::hook(replace = L2CFighterCommon_status_Glide)]
pub unsafe fn status_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, /*Motion Kind*/ Hash40::new("glide_direction"), /*Start Frame*/ 90.0, /*Motion Rate*/ 0.0, /*Is Looped*/ true, /*unk*/ 0.0, /*unk*/ false, /*unk*/ false);
    MotionModule::add_motion_partial(fighter.module_accessor, /*Slot ID*/ *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, /*Motion Kind*/ Hash40::new("glide_wing"), /*Start Frame*/ 0.0, /*Motion Rate*/ 1.0, /*Is Looped*/ true, /*Rumble*/ false, /*unk*/ 0.0, /*unk*/ false, /*Is Active*/ true, /*unk*/ false);
    
    /*Palutena Only*/
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_PALUTENA {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_wing"), false, -1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Glide_Main as *const () as _))
}

/*Main exec status for glide that controls a lot of things. Runs once-per-frame.*/

// #[skyline::hook(replace = L2CFighterCommon_status_Glide_Main)]
unsafe extern "C" fn status_exec_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    let _energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let mut angle = WorkModule::get_float(fighter.module_accessor, /*Float Type*/ *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, /*Float Type*/ *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);

    /*Cliff/Ledge grab Check*/
    fighter.sub_air_check_fall_common();
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    
    /*This whole block handles the controls for gliding. Tilt the control stick up/left to ascend or
    down/right to descend.*/
    if lr <= 0.0 {
        let mut above_or_below = -1.0;
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / PI;
    }

    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt(); /*Square Root of Stick X^2 + Stick Y^2*/

    if stick_magnitude > params.radial_stick {
        let angle_accel = if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                -params.down_angle_accel
            }
            else {
                params.up_angle_accel
            }   
        }
        else {
            if stick_angle >= 45.0 {
                params.up_angle_accel
            }
            else {
                -params.down_angle_accel
            }
        };

        let scaled_angle_accel = angle_accel * (stick_magnitude - params.radial_stick) / (1.0 - params.radial_stick);

        if angle_speed * scaled_angle_accel < 0.0 {
            angle_speed = 0.0;
        }
        
        let mut new_angle_speed = angle_speed + scaled_angle_accel;
        
        /*Clamp is a value bounded by a minimum and a maximum. If input is less than min, then this 
        returns min. If input is greater than max then this returns max*/
        new_angle_speed = new_angle_speed.clamp(-params.max_angle_speed, params.max_angle_speed);
        WorkModule::set_float(fighter.module_accessor, /*Float Value*/ new_angle_speed, /*Float Type*/ *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        angle += new_angle_speed;
    }
    
    //Contols the Max Upward and Downward angles.
    angle = angle.clamp(params.angle_max_down, params.angle_max_up);
    
    /*Handles the angular glide speed, momentum and power rates.*/
    let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    power -= angle * params.speed_change / 90.0;
    /*Instead of setting the glide's specific status flag for touching a wall,
    we can just check it directly in this code which does the same thing*/
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        power -= 0.01;
    }
    if power < 0.0 {
        power = 0.0
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
        if angle < params.angle_more_speed {
            power += params.down_speed_add * (params.angle_more_speed - angle) / (params.angle_more_speed - params.angle_max_down);
        }
    }
    else if angle > 0.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
    }

    let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let mut new_gravity = gravity + params.gravity_accel;
    if new_gravity > params.gravity_speed {
        new_gravity = params.gravity_speed;
    }
    WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);

    let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
    angled.y -= new_gravity;

    let speed = (angled.x * angled.x + angled.y * angled.y).sqrt(); //Square Root of angled.x^2 + angled.y^2
    let ratio = params.max_speed / speed;

    if speed > params.max_speed {
        angled.x *= ratio;
        angled.y *= ratio;
    }
    if speed < params.end_speed || power <= 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);

    /*Bonus feature: Controls the pitch for wind sounds (based on angle) for gliding.*/
    let kind = fighter.global_table[0x2].get_i32();
    if kind == *FIGHTER_KIND_METAKNIGHT {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 1.0 + angle * -0.0035);
    }
    if kind == *FIGHTER_KIND_PIT {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 1.0 + angle * -0.0047);
    }
    if kind == *FIGHTER_KIND_PITB {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pitb_glide_loop"), 1.0 + angle * -0.0043);
    }
    if kind == *FIGHTER_KIND_PLIZARDON {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_plizardon_glide_loop"), 0.85 + angle * -0.006);
    }
    if kind == *FIGHTER_KIND_RIDLEY {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_glide_loop"), 0.8 + angle * -0.005);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_jump02"), 1.0 + angle * 0.003);
        if angle >= params.angle_max_down && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.005);
        }
        if angle <= params.angle_max_up && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.01);
        }
    }
    if kind == *FIGHTER_KIND_BUDDY {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_glide_loop"), 0.94 + angle * -0.005);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_wing"), 1.0 + angle * 0.0048);
        if angle >= params.angle_max_down && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.01);
        }
        if angle <= params.angle_max_up && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.018);
        }
    }
    if kind == *FIGHTER_KIND_PALUTENA {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_palutena_glide_loop"), 1.0 + angle * -0.0043);
    }
    if kind == *FIGHTER_KIND_TRAIL {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_trail_glide_loop"), 1.1 + angle * -0.0071);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
pub unsafe fn bind_address_call_status_end_glide(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
pub unsafe fn status_end_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_PALUTENA {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_glidestart,
            status_glide, bind_address_call_status_end_glide, status_end_glide, 
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);

    Agent::new("fighter")
        .status(Exec, *FIGHTER_STATUS_KIND_GLIDE, status_exec_glide)
        .status(Init, *FIGHTER_STATUS_KIND_GLIDE, status_init_glide)
        .install();

}
