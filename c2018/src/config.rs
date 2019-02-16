//! Module contains configuration variables.
//! These should use `dimensioned` units wherever applicable, and contain units in the name otherwise.

use ::controls::const_unit;
use ::controls::units::*;
use std::time::Duration;

/// How long to sleep before checking for messages and other periodic tasks.
pub const SUBSYSTEM_SLEEP_TIME: Duration = Duration::from_millis(5);

pub mod drive {
    use super::*;

    /// ID of the master talon on the left side of the robot
    pub const LEFT_MASTER: i32 = 12;

    /// ID of the slave talon on the left side of the robot
    pub const LEFT_SLAVE: i32 = 14;

    /// ID of the master talon on the right side of the robot
    pub const RIGHT_MASTER: i32 = 3;

    /// ID of the slave talon on the right side of the robot
    pub const RIGHT_SLAVE: i32 = 1;

    /// The number of meters per tick of the drive encoders
    pub const ENCODER_METERS_PER_TICK: Meter<f64> = const_unit!(-1.0); //TODO

    /// Distance between the wheels on each drive side. This value will be tweaked later when we do
    /// tests for calibration which will account for wheel skid.
    pub const DRIVE_BASE_WHEEL_WIDTH: Meter<f64> = const_unit!(-1.0); //TODO

    /// Maximum current allowed before disabling the talon. Units are in amps.
    pub const CURRENT_LIMIT_THRESHOLD: i32 = 60;

    /// Limit for sustained current in the motor. Units are in amps.
    pub const CURRENT_LIMIT: i32 = 45;

    /// Limit for duration of sustained current
    pub const CURRENT_LIMIT_DURATION_MS: i32 = 200;

    /// Communication timeout for setting the talon configurations
    pub const TALON_CFG_TO_MS: i32 = 10;

    /// Gear shifter for the drive base.
    pub mod shifter {
        /// Gear shifter solenoid channel ID for high gear
        pub const SOLENOID_CHANNEL: i32 = 0;
        pub const HIGH_GEAR: bool = true; // TODO
    }
}

pub mod superstructure {
    // TODO find
    pub const GATE1: i32 = -1;
    pub const GATE2: i32 = -1;
    pub const GATE3: i32 = -1;

    use super::*;
    pub mod elevator {
        use super::*;
        #[allow(dead_code)]
        pub const MIN: Meter<f64> = const_unit!(0.0);
        // TODO: Make the below actual values
        pub const MAX: Meter<f64> = const_unit!(0.0);
        pub const LOW: Meter<f64> = const_unit!(0.0);
        pub const MID_LOW: Meter<f64> = const_unit!(0.0);
        pub const MID_HIGH: Meter<f64> = const_unit!(0.0);
        pub const HIGH: Meter<f64> = const_unit!(0.0);

        // todo IDS
        pub const MASTER_TALON: i32 = -1;
        pub const SLAVE_TALON1: i32 = -1;
        pub const SLAVE_TALON2: i32 = -1;
        pub const LIMIT_SWITCH: i32 = -1;
    }

    // TODO ids
    pub const INTAKE_SOLENOID: i32 = -1;
    pub const CHANNEL_TALON: i32 = -1;
    pub const OUTTAKE_TALON: i32 = -1;

    pub mod hatch {
        // TODO set ids
        pub const EXTEND_PNEUMATICS_ID: i32 = -1;
        pub const OUTTAKE_PNEUMATICS_ID: i32 = -1;
    }
}
