use super::Subsystem;
use crossbeam_channel::Receiver;

mod hatch_hardware;
use hatch_hardware::HatchHardware;

mod channel;
use ctre_elevator_tuning::Elevator;

#[derive(Debug, Clone)]
pub struct PeriodicOuts {
    pub intk_pct: f64,
    pub outk_pct: f64,
    pub intk_pnm: bool,
    pub elev_pos: controls::units::Meter<f64>,
}

impl Default for PeriodicOuts {
    fn default() -> Self {
        Self {
            intk_pct: 0.0,
            outk_pct: 0.0,
            intk_pnm: IntakeExt::Ext.into(),
            elev_pos: 0.0 * controls::units::M,
        }
    }
}

// Manual errors ikr
#[derive(Debug, Copy, Clone)]
pub enum HalCtreError {
    Hal(wpilib::HalError),
    Ctre(ctre::ErrorCode),
}
impl From<wpilib::HalError> for HalCtreError {
    fn from(err: wpilib::HalError) -> Self {
        HalCtreError::Hal(err)
    }
}
impl From<ctre::ErrorCode> for HalCtreError {
    fn from(err: ctre::ErrorCode) -> Self {
        HalCtreError::Ctre(err)
    }
}

// Utility
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HatchPneumaticExt {
    Extended,
    Retracted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IntakeExt {
    Ext,
    Retr,
}

impl From<IntakeExt> for bool {
    fn from(intake: IntakeExt) -> bool {
        intake == IntakeExt::Ext
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HatchState {
    pub extend: HatchPneumaticExt,
    pub outtake: HatchPneumaticExt,
}

/// Interface for the controlling channel
mod interface {
    use super::goal::BallGoalHeight;
    use super::goal::HatchGoalHeight;
    use super::HatchPneumaticExt;
    pub enum UserElevatorHeights {
        Low,
        Med,
        High,
        Cargo,
    }

    impl UserElevatorHeights {
        pub fn into_hatch(self) -> HatchGoalHeight {
            match self {
                UserElevatorHeights::Cargo => HatchGoalHeight::Low,
                UserElevatorHeights::Low => HatchGoalHeight::Low,
                UserElevatorHeights::Med => HatchGoalHeight::Med,
                UserElevatorHeights::High => HatchGoalHeight::High,
            }
        }

        pub fn into_ball(self) -> BallGoalHeight {
            match self {
                UserElevatorHeights::Cargo => BallGoalHeight::Cargo,
                UserElevatorHeights::Low => BallGoalHeight::Low,
                UserElevatorHeights::Med => BallGoalHeight::Med,
                UserElevatorHeights::High => BallGoalHeight::High,
            }
        }
    }

    pub enum Instruction {
        SetElevatorHeight(UserElevatorHeights),
        Unjam(bool),
        BallOuttake(bool),
        BallIntake(bool),
        ForceAbortBall,
        HatchExtend(HatchPneumaticExt),
        HatchOuttake(HatchPneumaticExt),
        Climb(bool),
        BeginElevatorPanic,
        ForceElevatorZero,
    }

}
pub use interface::*;

// Data types for goal states
mod goal {
    use super::HatchState;
    use controls::units;
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GoalState {
        Ball(BallGoalHeight),
        Hatch(HatchGoalHeight, HatchState),
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BallGoalHeight {
        None,
        Cargo,
        Low,
        Med,
        High,
    }

    impl From<BallGoalHeight> for units::Meter<f64> {
        fn from(goal_height: BallGoalHeight) -> units::Meter<f64> {
            use BallGoalHeight::*;
            match goal_height {
                None => 100.0 * ctre_elevator_tuning::METERS_PER_TICK,
                Low => 5325.0 * ctre_elevator_tuning::METERS_PER_TICK + 0.0343 * units::M,
                Cargo => 17100.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M,
                Med => 24000.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M,
                High => {
                    35100.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M
                        + 0.216 * units::M
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum HatchGoalHeight {
        Low,
        Med,
        High,
    }
    impl From<HatchGoalHeight> for units::Meter<f64> {
        fn from(goal_height: HatchGoalHeight) -> units::Meter<f64> {
            use HatchGoalHeight::*;
            match goal_height {
                Low => {
                    700.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M
                        + 0.216 * units::M
                }
                Med => {
                    17900.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M
                        + 0.216 * units::M
                }
                High => {
                    35100.0 * ctre_elevator_tuning::METERS_PER_TICK - 0.0343 * units::M
                        + 0.216 * units::M
                }
            }
        }
    }
}

mod unjam;

use ctre::motor_control::{ControlMode, DemandType, MotorController, TalonSRX};
use wpilib::{pneumatics::Solenoid, HalResult};
#[derive(Debug)]
pub struct Superstructure {
    goal: goal::GoalState,
    unjam: unjam::UnjamState,
    hatch_hardware: HatchHardware,
    channel: channel::Channel,
    elevator: Elevator,
    im: CachingTalon,
    is: CachingSolenoid,
    climb: CachingSolenoid,
    om: CachingTalon,
    pressure_sensor: Rev111107DS00PressureSensor,
    receiver: Receiver<Instruction>,
}
use crate::config::superstructure as config;
impl Superstructure {
    pub fn new(recv: Receiver<Instruction>) -> HalResult<Self> {
        Ok(Self {
            goal: goal::GoalState::Hatch(
                goal::HatchGoalHeight::Low,
                hatch_hardware::CLOSED_HATCH_STATE,
            ),
            unjam: unjam::UnjamState::Disabled,
            hatch_hardware: hatch_hardware::HatchHardware::new()?,
            channel: channel::Channel::new()?,
            elevator: Elevator::new()?,
            im: CachingTalon::new(TalonSRX::new(config::CHANNEL_TALON)),
            is: CachingSolenoid::new(Solenoid::new(config::INTAKE_SOLENOID)?)?,
            om: CachingTalon::new(TalonSRX::new(config::OUTTAKE_TALON)),
            climb: CachingSolenoid::new(Solenoid::new(config::CLIMB_SOLENOID)?)?,
            pressure_sensor: Rev111107DS00PressureSensor::new(AnalogInput::new(3)?, 5.0),
            receiver: recv,
        })
    }
}

impl Superstructure {
    fn flush_outs(&mut self, out: &PeriodicOuts) -> Result<(), HalCtreError> {
        // TODO replace with individual handling
        // or consider using Result::and() to chain these as is
        self.im.pct(out.intk_pct)?;
        self.is.set(out.intk_pnm)?;
        self.om.pct(out.outk_pct)?;
        self.elevator.set_goal(out.elev_pos);
        Ok(())
    }
}

impl Subsystem for Superstructure {
    fn run(mut self) {
        use goal::*;
        let mut pnm_print_count: u32 = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(5));
            if pnm_print_count > 100 {
                pnm_print_count = 0;
                println!(
                    "Pnuematic System PSI: {}",
                    self.pressure_sensor.get_psi().unwrap_or(std::f64::NAN)
                );
                println!("Elevator State: {:?}", self.elevator.state());
            }
            pnm_print_count += 1;
            let mut outs = PeriodicOuts::default();
            for msg in self.receiver.try_iter() {
                use Instruction::*;
                match msg {
                    HatchExtend(ext) => {
                        if let GoalState::Hatch(_, ref mut ext_state) = self.goal {
                            ext_state.extend = ext;
                        }
                    }
                    HatchOuttake(ext) => {
                        if let GoalState::Hatch(_, ref mut ext_state) = self.goal {
                            ext_state.outtake = ext;
                        }
                    }
                    Unjam(x) => self.unjam.set_enabled(x),
                    BallIntake(true) => self.goal = GoalState::Ball(BallGoalHeight::None),
                    BallIntake(false) => {
                        if self.channel.try_abort_intk() {
                            self.goal = GoalState::Hatch(
                                HatchGoalHeight::Low,
                                hatch_hardware::CLOSED_HATCH_STATE,
                            );
                        }
                    }
                    SetElevatorHeight(wanted) => match self.goal {
                        GoalState::Hatch(ref mut height, _) => {
                            *height = wanted.into_hatch();
                        }
                        GoalState::Ball(ref mut height) => {
                            if self.channel.is_in_carriage() {
                                *height = wanted.into_ball();
                            }
                        }
                    },
                    BallOuttake(true) => {
                        self.channel.try_init_outk();
                    }
                    BallOuttake(false) => {
                        self.channel.try_stop_outk();
                    }
                    ForceAbortBall => {
                        self.channel.force_abort();
                        self.goal = GoalState::Hatch(
                            HatchGoalHeight::Low,
                            hatch_hardware::CLOSED_HATCH_STATE,
                        );
                    }
                    Climb(do_ext) => {
                        self.climb.set(do_ext).ok();
                    }
                    BeginElevatorPanic => {
                        self.elevator.try_init_panic();
                    }
                    ForceElevatorZero => {
                        self.elevator.force_begin_zero();
                    }
                }
            }
            // process desired state
            match self.goal.clone() {
                GoalState::Hatch(height, ext_state) => {
                    // TODO log
                    self.channel.process_sensors(false).ok();
                    outs.elev_pos = height.into();
                    // TODO log
                    self.hatch_hardware.set(ext_state.clone()).ok();
                }
                GoalState::Ball(goal_height) => {
                    // TODO log error
                    self.hatch_hardware.set_closed().ok();
                    self.channel.idempotent_start();
                    if self.channel.is_done() {
                        self.goal = GoalState::Hatch(
                            HatchGoalHeight::Low,
                            hatch_hardware::CLOSED_HATCH_STATE,
                        );
                        self.channel.reset();
                    } else {
                        outs.elev_pos = goal_height.into();
                        // TODO log the two possible failures here
                        self.channel
                            .process_sensors(self.elevator.is_holding().unwrap_or(false))
                            .ok();
                    }
                }
            }
            // TODO log
            self.elevator.iterate().ok();

            self.channel.write_outs(&mut outs);
            // Unjam gets to override everyone else
            self.unjam.process();
            self.unjam.write_outs(&mut outs);
            // TODO log
            self.flush_outs(&outs).ok();
        }
    }
}

#[derive(Debug)]
struct CachingTalon(TalonSRX, (ControlMode, f64, DemandType, f64));

impl CachingTalon {
    pub fn new(x: TalonSRX) -> Self {
        Self(
            x,
            (
                ControlMode::Disabled,
                std::f64::NAN,
                DemandType::Neutral,
                std::f64::NAN,
            ),
        )
    }

    pub fn pct(&mut self, pct: f64) -> ctre::Result<()> {
        self.set(ControlMode::PercentOutput, pct, DemandType::Neutral, 0.0)
    }

    pub fn set(&mut self, c: ControlMode, f: f64, d: DemandType, g: f64) -> ctre::Result<()> {
        use controls::approx::abs_diff_eq;
        if self.1 .0 == c
            && abs_diff_eq!(self.1 .1, f)
            && self.1 .2 == d
            && abs_diff_eq!(self.1 .3, g)
        {
            return Ok(());
        }
        self.1 .0 = c;
        self.1 .1 = f;
        self.1 .2 = d;
        self.1 .3 = g;
        self.0.set(c, f, d, g)
    }

    #[allow(dead_code)]
    pub fn talon(&self) -> &TalonSRX {
        &self.0
    }

    #[allow(dead_code)]
    pub fn talon_mut(&mut self) -> &mut TalonSRX {
        // set the cache to something that will force a change next time
        self.1 .1 = std::f64::NAN;
        self.1 .3 = std::f64::NAN;
        &mut self.0
    }
}

#[derive(Debug)]
struct CachingSolenoid(Solenoid, bool);

impl CachingSolenoid {
    pub fn new(s: Solenoid) -> HalResult<Self> {
        s.set(false)?;
        Ok(Self(s, false))
    }

    pub fn set(&mut self, b: bool) -> HalResult<()> {
        if b == self.1 {
            return Ok(());
        }
        self.1 = b;
        self.0.set(b)
    }
}

use wpilib::AnalogInput;
#[derive(Debug)]
struct Rev111107DS00PressureSensor(AnalogInput, f64);

impl Rev111107DS00PressureSensor {
    fn new(ain: AnalogInput, supply_voltage: f64) -> Self {
        Self(ain, supply_voltage)
    }

    fn get_psi(&self) -> HalResult<f64> {
        Ok((250.0 * (self.0.voltage()? / self.1)) - 25.0)
    }
}
