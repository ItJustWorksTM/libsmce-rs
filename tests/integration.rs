use std::cell::{Cell, RefCell};
use std::error::Error;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use libsmce_rs::board::{BoardVendor, Status};
use libsmce_rs::board_config::BoardConfig;
use libsmce_rs::board_view::Link;
use libsmce_rs::sketch::Sketch;
use libsmce_rs::toolchain::Toolchain;

#[test]
fn test_compile() -> Result<(), Box<dyn Error>> {
    let mut smce_resources = PathBuf::from(std::env::var("OUT_DIR")?);
    smce_resources.push("libsmce-rs/cmake");

    let mut tc = Toolchain::new(&smce_resources).unwrap();

    let mut sketch = Sketch::new(Path::new("./tests/sketches/echo/echo.ino")).unwrap();

    assert!(
        sketch.get_source().exists(),
        "pwd: {:?}/{:?}",
        std::env::current_dir().unwrap(),
        sketch.get_source()
    );

    assert!(tc.compile(&mut sketch).is_ok());
    assert!(sketch.is_compiled());

    let mut vendor = BoardVendor::new(BoardConfig::default());
    let mut board = vendor.use_sketch(&sketch).unwrap();

    assert_eq!(board.status(), Status::Ready);
    assert!(board.start());

    let mut view = board.view().ok_or("No view to be had")?;
    let mut digital = view.pin(0).digital().ok_or("Pin doesn't exist")?;
    digital.write(true);

    let mut uart = view.uart(0).ok_or("Uart channel doesn't exist")?;

    uart.write("Hello World".as_ref());

    std::thread::sleep(Duration::from_secs(3));

    let mut uart = view.uart(0).ok_or("Uart doesn't exist")?;

    let mut buf = String::new();
    buf.reserve(uart.available());
    let read = uart.read_to_string(&mut buf)?;

    println!("Got {} bytes, message: {}", read, buf);

    // let count = uart.read(&mut buf);

    assert!(board.suspend());
    assert_eq!(board.status(), Status::Suspended);
    assert!(board.terminate());
    assert_eq!(board.status(), Status::Ready);
    assert!(board.start());
    assert!(!board.start());
    assert!(board.terminate());
    assert_eq!(board.status(), Status::Ready);

    Ok(())
}
