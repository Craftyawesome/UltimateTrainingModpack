use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::lua_bind::*;
use smash::app::{self};
use smash::hash40;
use smash::lib::lua_const::*;

pub unsafe fn get_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    var: i32,
) -> Option<f32> {
    if var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X
        || var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y
    {
        if is_training_mode() && is_operation_cpu(module_accessor) && is_in_hitstun(module_accessor)
        {
            if menu.DI_STATE != NONE {
                let mut angle = (menu.DI_STATE - 1) as f64 * PI / 4.0;

                // Either 0 (right) or PI (left)
                if menu.DI_STATE == DI_RANDOM_IN_AWAY {
                    angle = app::sv_math::rand(hash40("fighter"), 2) as f64 * PI;
                }
                // If facing left, reverse angle
                if PostureModule::lr(module_accessor) != -1.0 {
                    angle -= PI;
                }

                if var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X {
                    return Some(angle.cos() as f32);
                }

                if var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y {
                    return Some(angle.sin() as f32);
                }
            }
        }
    }

    None
}
