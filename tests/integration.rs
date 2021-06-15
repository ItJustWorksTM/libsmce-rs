use std::cell::RefCell;
use std::error::Error;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use libsmce_rs::board::Board;
use libsmce_rs::board_config::{BoardConfig, DigitalDriver, FrameBuffer, GpioDriver, UartChannel};
use libsmce_rs::sketch::Sketch;
use libsmce_rs::sketch_config::{Library, SketchConfig};
use libsmce_rs::toolchain::Toolchain;
use std::collections::HashMap;

#[test]
fn test_compile() -> Result<(), Box<dyn Error>> {
    let mut smce_resources = PathBuf::from(env!("OUT_DIR"));

    let board_config = BoardConfig {
        gpio_drivers: vec![
            GpioDriver {
                pin_id: 0,
                digital_driver: DigitalDriver::default().into(),
                analog_driver: None,
            },
            GpioDriver {
                pin_id: 1,
                digital_driver: DigitalDriver::default().into(),
                analog_driver: None,
            },
        ],
        uart_channels: vec![UartChannel::default()],
        frame_buffers: vec![FrameBuffer::default()],
        ..Default::default()
    };

    let sketch_config = SketchConfig {
        fqbn: "arduino:avr:nano".into(),
        preproc_libs: vec![
            Library::RemoteArduinoLibrary {
                name: "MQTT".into(),
                version: "2.5.0".into(),
            },
            Library::RemoteArduinoLibrary {
                name: "WiFi".into(),
                version: "1.2.7".into(),
            },
        ],
        ..Default::default()
    };

    let mut sketch =
        Sketch::new(Path::new("./tests/sketches/mqtt/mqtt.ino"), sketch_config).unwrap();

    println!("{:#?}", board_config);
    println!("{:#?}", sketch);

    assert!(sketch.source().exists());

    let (compile_res, log) = Toolchain::compile(&mut sketch, &smce_resources);
    println!("{}", log);

    assert!(compile_res.is_ok());

    assert!(sketch.compiled());

    let mut board = Board::new();
    let mut handle = board.start(&board_config, &sketch)?;

    let mut view = handle.view();

    view.pins
        .get(0)
        .expect("pin 0 doesn't exist :(")
        .digital_write(true);

    assert!(view.pins[0].digital_read());
    view.pins[0].digital_write(false);
    assert!(!view.pins[0].digital_read());

    view.pins.get(1).expect("pin 1 doesnt exist :(");

    std::thread::sleep(Duration::from_secs(1));

    let mut uart = &view.uart_channels[0];

    let mut uart_out = String::new();
    uart.read_to_string(&mut uart_out);

    println!("UART:\n{}", uart_out);

    assert_eq!(handle.stop(), 0);

    Ok(())
}
