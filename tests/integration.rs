use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use smce_rs::board::Board;
use smce_rs::board_config::{BoardConfig, DigitalDriver, FrameBuffer, GpioDriver, UartChannel};
use smce_rs::sketch::Sketch;
use smce_rs::sketch_config::{Library, SketchConfig};
use smce_rs::toolchain::Toolchain;

#[test]
fn test_compile() -> Result<(), anyhow::Error> {
    let smce_resources = PathBuf::from(env!("OUT_DIR"));

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
        Sketch::new(Path::new("./tests/sketches/print/print.ino"), sketch_config).unwrap();

    println!("{:#?}", board_config);
    println!("{:#?}", sketch);

    assert!(sketch.source().exists());

    let (tc, mut log) = Toolchain::new(&smce_resources);
    let compile_res = tc.compile(&mut sketch);

    println!("{}", {
        let mut log_buf = String::new();
        log.read_to_string(&mut log_buf)?;
        log_buf
    });

    assert!(compile_res.is_ok());

    assert!(sketch.compiled());

    println!("Compile complete");

    let mut board = Board::new();
    let handle = board.start(&board_config, &sketch)?;
    println!("Sketch started");

    let view = handle.view();

    println!("Pin test");

    view.pins
        .get(0)
        .expect("pin 0 doesn't exist :(")
        .digital_write(true);

    assert!(view.pins[0].digital_read());
    view.pins[0].digital_write(false);
    assert!(!view.pins[0].digital_read());

    view.pins.get(1).expect("pin 1 doesnt exist :(");

    std::thread::sleep(Duration::from_secs(1));

    let mut uart_out = String::new();
    let read = (&view.uart_channels[0]).read_to_string(&mut uart_out)?;

    println!("UART ({}):\n{}", read, uart_out);

    assert_eq!(handle.stop(), 0);

    Ok(())
}
