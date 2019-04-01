use wpilib::RobotBase;
extern crate ctre_elevator_tuning;
use ctre_elevator_tuning::Elevator;
use wpilib::ds::RobotState::Disabled;
use wpilib::ds::{JoystickAxis, JoystickPort};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("TEST\nTEST\nTEST\nTEST\nTEST\nTEST\nTEST\nTEST\nTEST\n");

    let base = RobotBase::new().expect("HAL");

    let ds = base.make_ds();
    let joy = JoystickPort::new(0).unwrap();
    let axis = JoystickAxis::new(1).unwrap();
    let mut elev = Elevator::new().expect("ELEVATOR");
    RobotBase::start_competition();
    loop {
        if let Disabled = ds.robot_state() {
            continue;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        let sp = f64::from(ds.stick_axis(joy, axis).unwrap_or(0.0) * -1.0 / 2.0 + 0.5)
            * Elevator::MAX_HEIGHT;
        elev.set_goal(sp);
        println!("sp {}", sp);
        elev.iterate().expect("ITER FAILED");
        println!("{:?}", elev.state());
    }
}
