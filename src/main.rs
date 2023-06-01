use device_query::{DeviceQuery, DeviceState, MouseState};
use enigo::*;
use quad_gamepad::{ControllerContext, ControllerState};
use std::{thread, time};

const WAIT_TIME: u32 = 1;

fn wait(milli_seconds: u32) {
    let time_millis = time::Duration::from_millis(milli_seconds.into());
    thread::sleep(time_millis);
}

fn key_input_handler(controller: &mut ControllerContext, enigo: &mut Enigo) {
    controller.update();
    let state = controller.state(0);
    println!("digita_state: {:?}", state.digital_state);

    let b = state.digital_state[0];
    let a = state.digital_state[0];
    let y = state.digital_state[2];
    let l = state.digital_state[8];
    let r = state.digital_state[10];
    let select = state.digital_state[14];

    let prev_b = state.digital_state_prev[0];
    let prev_a = state.digital_state_prev[1];
    let prev_y = state.digital_state_prev[2];
    let prev_l = state.digital_state_prev[8];
    let prev_r = state.digital_state_prev[10];
    let prev_select = state.digital_state_prev[14];

    let button_delay: u32 = 350;
    if b && b != prev_b {
        enigo.mouse_click(MouseButton::Left);
        wait(button_delay);
    }
    if y && y != prev_y {
        enigo.mouse_click(MouseButton::Right);
        wait(button_delay);
    }
    if r || prev_r {
        enigo.key_click(Key::Layout('a'));
    }
    if l || prev_l {
        enigo.key_click(Key::Layout('d'));
    }
    if select || prev_select {}

    let analog = state.analog_state;

    let rel_x = analog[0].ceil();
    let rel_y = analog[1].ceil();

    println!("{}, {}", rel_x, rel_y);
    println!("{} {}", b, y);

    if rel_x != 0.0 || rel_y != 0.0 {
        enigo.mouse_move_relative(rel_x as i32, rel_y as i32);
        wait(WAIT_TIME);
    }
}

fn main() {
    let mut enigo = Enigo::new();
    let device_state = DeviceState::new();

    let mouse_pos = device_state.get_mouse();

    let mut controller = match ControllerContext::new() {
        Some(c) => c,
        None => return,
    };

    let _info = controller.info(0);

    loop {
        key_input_handler(&mut controller, &mut enigo);
        // controller.update();
        // let state = controller.state(0);
        // // println!("digita_state: {:?}", state.digital_state);
        //
        // let mouse: MouseState = DeviceQuery::get_mouse(&device_state);
        //
        // let _mpos = mouse.coords.clone();
        //
        // let b1 = state.digital_state_prev[0];
        // let b2 = state.digital_state_prev[2];
        //
        // let button_delay: u32 = 350;
        // if b1 {
        //     enigo.mouse_click(MouseButton::Left);
        //     wait(button_delay);
        // }
        // if b2 {
        //     enigo.mouse_click(MouseButton::Right);
        //     wait(button_delay);
        // }
        //
        // let analog = state.analog_state;
        //
        // let x = analog[0].ceil();
        // let y = analog[1].ceil();
        //
        // println!("{}, {}", x, y);
        // println!("{} {}", b1, b2);
        //
        // enigo.mouse_move_relative(x as i32, y as i32);
        // wait(WAIT_TIME);
    }
}
