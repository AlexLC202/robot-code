//! Module contains configuration variables.
//! These should use `dimensioned` units wherever applicable, and contain units in the name otherwise.

use controls::const_unit;
use controls::units::*;
use std::time::Duration;

/// How long to sleep before checking for messages and other periodic tasks.
pub const SUBSYSTEM_SLEEP_TIME: Duration = Duration::from_millis(5);

pub mod drive {
    use super::*;

    /// ID of the master talon on the left side of the robot
    pub const LEFT_MASTER: i32 = -1; //TODO

    /// ID of the slave talon on the left side of the robot
    pub const LEFT_SLAVE: i32 = -1; //TODO

    /// ID of the master talon on the right side of the robot
    pub const RIGHT_MASTER: i32 = -1; //TODO

    /// ID of the slave talon on the right side of the robot
    pub const RIGHT_SLAVE: i32 = -1; //TODO

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
    use super::*;
    pub mod elevator {
        use super::*;
        pub const MIN: Meter<f64> = const_unit!(0.0);
        // TODO: Make the below actual values
        pub const MAX: Meter<f64> = const_unit!(0.0);
        pub const LOW: Meter<f64> = const_unit!(0.0);
        pub const MID_LOW: Meter<f64> = const_unit!(0.0);
        pub const MID_HIGH: Meter<f64> = const_unit!(0.0);
        pub const HIGH: Meter<f64> = const_unit!(0.0);
    }
}
