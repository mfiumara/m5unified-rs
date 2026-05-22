use m5unified::{
    colors, M5Unified, PwmServoPins, StackChanMove, StackChanPose, StackChanPwmServoConfig,
    StackChanPwmServos,
};

fn main() -> Result<(), m5unified::Error> {
    let mut m5 = M5Unified::begin()?;

    m5.display.fill_screen(colors::BLACK);
    m5.display.set_text_color(colors::GREEN, colors::BLACK);
    m5.display.set_text_size(2);
    m5.display.set_cursor(4, 8);
    m5.display.println("Stack-chan PWM fallback")?;

    // Generic PWM fallback for non-official Stack-chan style pan/tilt builds:
    // pan -> CoreS3 Port A GPIO2, tilt -> CoreS3 Port A GPIO1, with an external
    // 5V servo supply and common ground. Official Stack-chan CoreS3 bodies use
    // StackChan-BSP Motion, M5StackChan.begin/update, VM_EN/IO-expander power
    // setup, and the MCP x/y/speed contract; they are not Port A PWM devices.
    let config = StackChanPwmServoConfig::pwm_pins(PwmServoPins::CORES3_PORT_A);
    let mut servos = match StackChanPwmServos::attach(config) {
        Ok(servos) => servos,
        Err(error) => {
            m5.display.set_cursor(4, 40);
            m5.display.println("PWM servo unavailable")?;
            m5.display.set_cursor(4, 64);
            m5.display.println(&format!("{error}"))?;
            return Ok(());
        }
    };

    run_pose(&mut m5, &mut servos, "neutral", StackChanPose::NEUTRAL)?;
    run_pose(&mut m5, &mut servos, "left", StackChanPose::LEFT)?;
    run_pose(&mut m5, &mut servos, "right", StackChanPose::RIGHT)?;
    run_pose(&mut m5, &mut servos, "up", StackChanPose::UP)?;
    run_pose(&mut m5, &mut servos, "down", StackChanPose::DOWN)?;

    servos.move_to(StackChanMove::from_mcp(0.0, 45.0, 50))?;

    for pose in [
        StackChanPose::new(-600, 450),
        StackChanPose::new(600, 450),
        StackChanPose::new(0, 300),
        StackChanPose::new(0, 700),
        StackChanPose::NEUTRAL,
    ] {
        servos.smooth_move_to(pose, 50, 20, |ms| m5.delay_ms(ms))?;
    }

    let _ = servos.detach();
    m5.display.set_cursor(4, 120);
    m5.display.println("done")?;
    Ok(())
}

fn run_pose(
    m5: &mut M5Unified,
    servos: &mut StackChanPwmServos,
    label: &str,
    pose: StackChanPose,
) -> Result<(), m5unified::Error> {
    servos.write_pose(pose)?;
    m5.display.set_cursor(4, 40);
    m5.display.println(&format!(
        "{label}: pan={} tilt={}",
        pose.pan_tenths, pose.tilt_tenths
    ))?;
    m5.delay_ms(400);
    Ok(())
}
