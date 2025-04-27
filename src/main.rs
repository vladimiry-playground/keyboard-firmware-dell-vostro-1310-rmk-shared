#![no_main]
#![no_std]

use embassy_executor::{main, Spawner};
use embassy_stm32::{
    bind_interrupts,
    gpio::{Input, Output},
    peripherals,
    time::Hertz,
    usb, Config,
};
use panic_halt as _;
use rmk::{
    channel::EVENT_CHANNEL,
    config::{ControllerConfig, KeyboardUsbConfig, RmkConfig},
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::join3,
    initialize_keymap,
    input_device::Runnable,
    keyboard::Keyboard,
    light::LightController,
    matrix::Matrix,
    run_devices, run_rmk,
};

mod keymap;
mod macros;

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

#[defmt::global_logger]
struct Logger;

unsafe impl defmt::Logger for Logger {
    fn acquire() {}
    unsafe fn flush() {}
    unsafe fn release() {}
    unsafe fn write(_bytes: &[u8]) {}
}

#[main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        // from https://github.com/embassy-rs/embassy/blob/572e788b2e878436bde527ad66cf561775cebc66/examples/stm32f1/src/bin/usb_serial.rs#L23-L40
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            // Oscillator for bluepill, Bypass for nucleos.
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            src: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL9,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }
    let p = embassy_stm32::init(config);

    // <KEYBOARD-CONFIGURATION-SECTION>
    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b, // TODO change "vid"
        pid: 0x4643, // TODO change "pid"
        manufacturer: "[RMK Keyboard Firmware, Haobo]",
        product_name: "Dell Vostro 1310 Keyboard",
        serial_number: "rmk:f64c2b3c:000001", // TODO change "serial_number"
    };
    let (input_pins, output_pins) = macros::config_matrix_pins_stm32!(
        peripherals: p,
        input: [PD10, PC6, PB0, PA4, PA0, PC4, PA2, PA6],
        output: [PC0, PD1, PC12, PE14, PC10, PE12, PE10, PA8, PE8, PC8, PD8, PB14, PD3, PB12, PB10, PE3]
    );
    let usb_driver = usb::Driver::new(p.USB, Irqs, p.PA12, p.PA11);
    // </KEYBOARD-CONFIGURATION-SECTION>

    {
        let mut matrix =
            Matrix::<_, _, _, { keymap::ROW }, { keymap::COL }>::new(input_pins, output_pins, DefaultDebouncer::<{ keymap::ROW }, { keymap::COL }>::new());
        let rmk_config = RmkConfig {
            usb_config: keyboard_usb_config,
            ..Default::default()
        };
        let mut default_keymap = keymap::get_default_keymap();
        let keymap = initialize_keymap(&mut default_keymap, rmk_config.behavior_config.clone()).await;
        let mut keyboard = Keyboard::new(&keymap, rmk_config.behavior_config.clone());
        let light_controller: LightController<Output> = LightController::new(ControllerConfig::default().light_config);

        join3(
            run_devices! ((matrix) => EVENT_CHANNEL),
            keyboard.run(),
            run_rmk(&keymap, usb_driver, light_controller, rmk_config),
        )
        .await;
    }
}
