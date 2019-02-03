use wpilib::ds::*;

pub struct Xbox<'a> {
    ds: &'a DriverStation<'a>,
    port: JoystickPort,
    left_x: JoystickAxis,
    left_y: JoystickAxis,
    right_x: JoystickAxis,
    right_y: JoystickAxis,
    arrow_pad: JoystickPOV
}

macro_rules! get_button {
    ($ds:expr, $port:expr, $axis:expr) => {
        $ds.stick_button($port, $axis).unwrap_or(false)
    }
}

macro_rules! get_axis {
    ($ds:expr, $port:expr, $axis:expr) => (
        $ds.stick_axis($port, $axis).unwrap_or(0.0)
    );
}

macro_rules! get_pov {
    ($ds:expr, $port:expr, $pov:expr) => (
        $ds.stick_pov($port, $pov).unwrap_or(0)
    );
}

impl<'a> Xbox<'a> {
    fn new_from_channel(channel: u8, ds: &'a DriverStation<'a>) -> Result<Self, JoystickError> {
        Self::new_from_port(JoystickPort::new(channel)?, ds)
    }

    fn new_from_port(port: JoystickPort, ds: &'a DriverStation<'a>) -> Result<Self, JoystickError> {
        Ok(Self {
            ds,
            port,
            left_x: JoystickAxis::new(0)?,
            left_y: JoystickAxis::new(1)?,
            right_x: JoystickAxis::new(4)?,
            right_y: JoystickAxis::new(5)?,
            arrow_pad: JoystickPOV::new(0)?
        })
    }

    fn a(&self) -> bool {
        get_button!(self.ds, self.port, 1)
    }

    fn b(&self) -> bool {
        get_button!(self.ds, self.port, 2)
    }
    fn x(&self) -> bool {
        get_button!(self.ds, self.port, 3)
    }

    fn y(&self) -> bool {
        get_button!(self.ds, self.port, 4)
    }

    fn back(&self) -> bool {
        get_button!(self.ds, self.port, 7)
    }

    fn start(&self) -> bool {
        get_button!(self.ds, self.port, 8)
    }
    fn left_bumper(&self) -> bool {
        get_button!(self.ds, self.port, 5)
    }
    fn right_button(&self) -> bool {
        get_button!(self.ds, self.port, 6)
    }
    fn left_stick_pressed(&self) -> bool {
        get_button!(self.ds, self.port, 9)
    }
    fn right_stick_pressed(&self) -> bool {
        get_button!(self.ds, self.port, 10)
    }
    fn left_x(&self) -> f32 {
        get_axis!(self.ds, self.port, self.left_x)
    }
    fn left_y(&self) -> f32 {
        get_axis!(self.ds, self.port, self.left_y)
    }
    fn right_x(&self) -> f32 {
        get_axis!(self.ds, self.port, self.right_x)
    }
    fn right_y(&self) -> f32 {
        get_axis!(self.ds, self.port, self.right_y)
    }
    fn arrow_pad(&self) -> i16 {
        get_pov!(self.ds, self.port, self.arrow_pad)
    }
}
