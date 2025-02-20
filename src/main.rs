extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::sensors::{InfraredSensor, ColorSensor, SensorPort};
use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::{Ev3Result};

// The speed value. Listed as 500 - {val} where val is how fast you go - and the point at which your speed becomes negative.
const SPEED:i32 = 500 - 400;
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
        println!("Left: {}", left_speed);
        println!("Left: {}", right_speed);
        // Adjusts speed for each motor independently according to red value
        left_motor.set_speed_sp(left_speed)?;left_motor.run_forever()?;
        right_motor.set_speed_sp(right_speed)?;right_motor.run_forever()?;

        // In the case that a double black occurs, increases the speed buffer by 75
        // until such a time as
        if left_speed+adjustment_val < 0 && right_speed+adjustment_val < 0 {
            adjustment_val = 75;
        } else { adjustment_val = 0; }
    }
}
