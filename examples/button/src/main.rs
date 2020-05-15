use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::{NativeObject, Object};
use lvgl_sys;
use std::time::Duration;

unsafe extern "C" fn on_click(obj: *mut lvgl_sys::lv_obj_t, evt: lvgl_sys::lv_event_t) {
    println!("obj: {:?}", obj);
    println!("evt: {:?}", evt);
    panic!("Callback triggered");
}

fn main() -> Result<(), String> {
    let mut display = SimulatorDisplay::new(Size::new(1000, 1000));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Hello World", &output_settings);

    unsafe {
        lvgl_sys::lv_init();
    }

    // Implement and register your display:
    let mut display_driver = lvgl::DisplayDriver::new(&mut display);

    // Create screen and widgets
    let mut screen = display_driver.get_active_screen();

    // Create a button
    let mut button = lvgl::Button::new(&mut screen);
    button.set_pos(5, 5);

    // Label the button
    let mut label = lvgl::Label::new(&mut button);
    label.set_text("Hello World!");

    // Set button callback
    unsafe {
        lvgl_sys::lv_obj_set_event_cb(button.raw().as_mut(), Some(on_click));
    };

    let mut i = 0;
    'running: loop {
        if i > 59 {
            i = 0;
        }
        //time.set_text(format!("21:{:02}\0", i).as_str());
        i = 1 + i;

        ::std::thread::sleep(Duration::from_millis(
            lvgl_sys::LV_DISP_DEF_REFR_PERIOD as u64,
        ));

        unsafe {
            lvgl_sys::lv_task_handler();
        }

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

// Reference to native font for LittlevGL, defined in the file: "fonts_noto_sans_numeric_80.c"
extern "C" {
    pub static mut noto_sans_numeric_80: lvgl_sys::lv_font_t;
}
