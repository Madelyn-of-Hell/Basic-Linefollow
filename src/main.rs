extern crate ev3dev_lang_rust;

use std::io;
use std::io::Write;
use ev3dev_lang_rust::sensors::{InfraredSensor, ColorSensor, SensorPort};
use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::{Ev3Result};
use std::thread::sleep;
use std::time::Duration;

// A useful little enum so I can do numerical operations with sides but don't have to remember which is which
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
enum Side {
    Left = 0,
    Right = 1
}
impl Side {
    fn val(&self) -> usize { self.clone() as usize }
}
// The speed value. Listed as 500 - {val} where val is how fast you go - and the point at which your speed becomes negative.
const SPEED:i32 = 500 - 400;
const ADJUSTMENT_VALUE:i32 = 125;
const GREEN_OFFSET:i32 = 75;
fn main() -> Ev3Result<()> {
    // Declare Sensors
    let (left_sensor, right_sensor) = (ColorSensor::get(SensorPort::In2)?, ColorSensor::get(SensorPort::In1)?);
    // Declare Motors
    let (left_motor, right_motor) = (LargeMotor::get(MotorPort::OutA)?, LargeMotor::get(MotorPort::OutB)?);
    // Set sensors to rgb mode
    left_sensor.set_mode_rgb_raw()?;
    right_sensor.set_mode_rgb_raw()?;
    let (mut left_speed, mut right_speed);
    let mut adjustment_val = 0;
    // Main loop
    loop {
        left_speed = left_sensor.get_red()?-SPEED+adjustment_val;
        right_speed = right_sensor.get_red()?-SPEED+adjustment_val;

        // Adjusts speed for each motor independently according to red value
        left_motor.set_speed_sp(left_speed)?;left_motor.run_forever()?;
        right_motor.set_speed_sp(right_speed)?;right_motor.run_forever()?;
        // In the case that a double black occurs, increases the speed buffer by 75
        // until such a time as the sensors read a positive value without the buffer.
        if left_speed-adjustment_val < 0 && right_speed-adjustment_val < 0 {
            adjustment_val = ADJUSTMENT_VALUE;
        } else { adjustment_val = 0; }

        // Returns the side that a green value is detected on or none if one isn't detected.
        let green:Option<Side> =
                 if left_sensor.get_green()?  > GREEN_OFFSET + left_sensor.get_red()? {Some(Side::Left)}
            else if right_sensor.get_green()?  > GREEN_OFFSET + right_sensor.get_red()? {Some(Side::Right)}
            else { None };

        // Discards None values, then turns around the sensor that detected green.
        // TODO: Add a timeout value to prevent getting stuck around green squares.
        match green {
            None => {}
            Some(side) => {
                vec![&left_motor, &right_motor][side.val()].set_speed_sp(0)?;
                vec![&left_motor, &right_motor][side.val()].set_speed_sp(SPEED)?;
                sleep(Duration::from_secs(3));
            }
        }

    }
}