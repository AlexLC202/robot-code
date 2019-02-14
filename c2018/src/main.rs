#[macro_use]
extern crate debug_stub_derive;

mod cheesy_drive;
mod config;
mod subsystems;

use crate::subsystems::Subsystem;
use wpilib::ds::*;
use wpilib::RobotBase;

use bus::Bus;
use crossbeam_channel::unbounded;
use std::thread;
use subsystems::controller::*;
use subsystems::drive::*;

fn main() {
    let bus = Bus::new(0);
    let (drive_send, drive_recv) = unbounded();
    let (super_send, _super_recv) = unbounded();
    let base = RobotBase::new().unwrap();

    thread::spawn(move || {
        let drive = Drive::new(bus, drive_recv);
        println!("drive: {:?}", drive);
        drive.run();
    });

    let lj = JoystickPort::new(0).unwrap();
    let rj = JoystickPort::new(1).unwrap();
    let oi = JoystickPort::new(2).unwrap();
    let controls = StandardControls::new(base.make_ds(), lj, rj, oi).unwrap();

    RobotBase::start_competition();
    //NOTE: All new control bindings or functions should be added in subsystems/controller/mod.rs
    let controller = Controller::new(controls, drive_send, super_send, base.make_ds());
    println!("controller: {:?}", controller);
    controller.run();
}
