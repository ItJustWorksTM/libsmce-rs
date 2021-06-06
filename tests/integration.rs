use std::cell::RefCell;
use std::error::Error;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use libsmce_rs::board::Board;
use libsmce_rs::board_config::BoardConfig;
use libsmce_rs::sketch::Sketch;
use libsmce_rs::sketch_config::{Library, SketchConfig};
use libsmce_rs::toolchain::Toolchain;

#[test]
fn test_compile() -> Result<(), Box<dyn Error>> {
    let mut smce_resources = PathBuf::from(std::env::var("OUT_DIR")?);
    smce_resources.push("libsmce-rs/cmake");

    let mut sketch = Sketch::new(
        Path::new("./tests/sketches/mqtt/mqtt.ino"),
        SketchConfig {
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
        },
    )
    .unwrap();

    assert!(sketch.source().exists());

    let (compile_res, log) = Toolchain::compile(&mut sketch, &smce_resources);
    println!("{}", log);

    assert!(compile_res.is_ok());
    println!("{:#?}", sketch);

    assert!(sketch.compiled());

    let mut board = Board::default();
    let mut handle = board.start(&BoardConfig::default(), &sketch)?;

    let mut view = handle.view();
    view.digital_pin(0).unwrap().write(true);

    view.digital_pin(1).unwrap().write(false);

    std::thread::sleep(Duration::from_secs(10));

    let mut uart = view.uart(0).unwrap();

    let mut uart_out = String::new();
    uart.read_to_string(&mut uart_out);

    println!("UART:\n{}", uart_out);

    assert_eq!(handle.stop(), 0);

    Ok(())
}
