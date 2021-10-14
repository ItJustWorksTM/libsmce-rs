use std::{
    fs::{self, File},
    io::Write,
    io::{BufReader, Read},
    path::PathBuf,
    thread,
    time::Duration,
};

use smce_rs::{
    board::{Board, Status},
    board_config::SecureDigitalStorage,
    board_config::{BoardConfig, GpioDriver, UartChannel},
    board_view::GpioPin,
    sketch::Sketch,
    sketch_config::{PluginManifest, SketchConfig},
    toolchain::BuildLogReader,
    toolchain::Toolchain,
};

const TEST_HOME: &str = env!("SMCE_TEST_HOME");

fn build_sketch(
    path: &str,
    sketch_config: SketchConfig,
) -> anyhow::Result<(Sketch, BuildLogReader)> {
    let (tc, mut tclog) = Toolchain::new(TEST_HOME)?;
    let mut sketch = Sketch::new(path, sketch_config).unwrap();
    if let Err(err) = tc.compile(&mut sketch) {
        let mut log = String::new();
        let _ = tclog.read_to_string(&mut log)?;
        return Err(anyhow::anyhow!("Compile err: {} with log:\n{}", err, log));
    }
    Ok((sketch, tclog))
}

#[test]
fn suitable_env() {
    assert!(Toolchain::new(TEST_HOME).is_ok());
    assert!(Toolchain::new("_non_existent_").is_err());
}

#[test]
fn noop_compile() -> anyhow::Result<()> {
    let sketch = build_sketch("./tests/sketches/noop", Default::default())?.0;
    assert!(sketch.compiled());
    Ok(())
}

#[test]
fn tick_crash() -> anyhow::Result<()> {
    let sketch = build_sketch("./tests/sketches/uncaught", Default::default())?.0;

    let mut board = Board::new();
    let handle = board.start(&Default::default(), &sketch)?;

    for _ in 0..10 {
        if handle.tick().is_err() {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    assert_eq!(handle.status(), Status::Stopped);
    assert_ne!(handle.stop(), 0, "Expected non zero exitcode");
    Ok(())
}

#[test]
fn suspend_resume() -> anyhow::Result<()> {
    let sketch = build_sketch("./tests/sketches/noop", Default::default())?.0;

    let mut board = Board::new();
    let handle = board.start(&Default::default(), &sketch)?;

    assert_eq!(handle.status(), Status::Running);
    assert!(handle.suspend());
    assert_eq!(handle.status(), Status::Suspended);
    assert!(handle.resume());
    assert_eq!(handle.status(), Status::Running);
    assert!(!handle.resume());

    Ok(())
}

fn test_digital_pin_delayable(pin: &GpioPin, expected_value: bool) -> bool {
    for _ in 0..16384 {
        if pin.digital_read() == expected_value {
            return true;
        }
        thread::sleep(Duration::from_millis(1));
    }
    false
}

fn test_analog_pin_delayable(pin: &GpioPin, expected_value: u16) -> bool {
    for _ in 0..16384 {
        if pin.analog_read() == expected_value {
            return true;
        }
        thread::sleep(Duration::from_millis(1));
    }
    false
}

#[test]
fn boardview_gpio() -> anyhow::Result<()> {
    let sketch = build_sketch("./tests/sketches/pins", Default::default())?.0;

    let mut board = Board::new();
    let handle = board.start(
        &BoardConfig {
            gpio_drivers: vec![
                GpioDriver {
                    pin_id: 0,
                    allow_read: true,
                    allow_write: false,
                },
                GpioDriver {
                    pin_id: 2,
                    allow_read: false,
                    allow_write: true,
                },
            ],
            ..Default::default()
        },
        &sketch,
    )?;

    let bv = handle.view();

    let pin0 = &bv.pins[0];
    assert!(bv.pins.get(1).is_none());
    let pin2 = &bv.pins[2];

    thread::sleep(Duration::from_millis(1));

    pin0.digital_write(false);

    let read_log = || {
        let mut buf = String::new();
        let _ = handle.log().read_to_string(&mut buf);
        buf
    };

    assert!(test_digital_pin_delayable(pin2, true), "{}", read_log());
    pin0.digital_write(true);
    assert!(test_digital_pin_delayable(pin2, false), "{}", read_log());

    Ok(())
}

#[test]
fn uart() -> anyhow::Result<()> {
    let sketch = build_sketch("./tests/sketches/uart", Default::default())?.0;

    let mut board = Board::new();
    let handle = board.start(
        &BoardConfig {
            uart_channels: vec![UartChannel::default()],
            ..Default::default()
        },
        &sketch,
    )?;

    let mut uart0 = &handle.view().uart_channels[0];

    let mut echo_test = |input: &str| {
        assert_eq!(uart0.write(input.as_bytes()).unwrap(), input.len());

        let mut buf = String::with_capacity(input.len());
        for _ in 0..16000 {
            if uart0.read_to_string(&mut buf).unwrap() > 0 {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(buf, input);
    };

    echo_test("HELLO UART");
    echo_test("RUST IS COOL!");

    Ok(())
}

#[test]
fn mixed_sources() -> anyhow::Result<()> {
    let _ = build_sketch("./tests/sketches/with_cxx", Default::default())?;
    Ok(())
}

#[test]
fn remote_preproc_lib() -> anyhow::Result<()> {
    let _ = build_sketch(
        "./tests/sketches/remote_pp",
        SketchConfig {
            legacy_libs: vec!["MQTT".into()],
            ..Default::default()
        },
    )?;
    Ok(())
}

#[test]
fn wifi_intended_use() -> anyhow::Result<()> {
    let _ = build_sketch(
        "./tests/sketches/wifi",
        SketchConfig {
            legacy_libs: vec!["MQTT".into(), "WiFi".into()],
            ..Default::default()
        },
    )?;
    Ok(())
}

#[test]
fn patched_lib() -> anyhow::Result<()> {
    let sketch = build_sketch(
        "./tests/sketches/patch",
        SketchConfig {
            plugins: vec![PluginManifest {
                uri: "https://github.com/platisd/smartcar_shield/archive/refs/tags/7.0.1.tar.gz"
                    .into(),
                patch_uri: fs::canonicalize("./tests/patches/ESP32_analogRewrite")
                    .map(|st| st.to_string_lossy().into())?,
                ..Default::default()
            }],
            ..Default::default()
        },
    )?
    .0;

    let mut board = Board::new();
    let handle = board.start(
        &BoardConfig {
            gpio_drivers: vec![GpioDriver {
                pin_id: 0,
                allow_read: false,
                allow_write: true,
            }],
            ..Default::default()
        },
        &sketch,
    )?;

    thread::sleep(Duration::from_millis(1));

    assert!(test_analog_pin_delayable(&handle.view().pins[0], 42));

    Ok(())
}

#[test]
fn sdcard() -> anyhow::Result<()> {
    let sketch = build_sketch(
        "./tests/sketches/sd_fs",
        SketchConfig {
            legacy_libs: vec!["SD".into()],
            ..Default::default()
        },
    )?
    .0;

    let mut root_dir = {
        let mut root = PathBuf::from(TEST_HOME);
        root.push("storage");
        root
    };

    if root_dir.exists() {
        fs::remove_dir_all(&root_dir)?;
    }
    fs::create_dir_all(&root_dir)?;

    let mut board = Board::new();
    let handle = board.start(
        &BoardConfig {
            gpio_drivers: vec![GpioDriver {
                pin_id: 0,
                allow_read: false,
                allow_write: true,
            }],
            sd_cards: vec![SecureDigitalStorage {
                root_dir: root_dir.clone().to_string_lossy().into(),
                cspin: 0,
            }],
            ..Default::default()
        },
        &sketch,
    )?;

    assert!(test_digital_pin_delayable(&handle.view().pins[0], true));
    handle.stop();

    root_dir.push("foo");
    assert!(root_dir.is_dir());
    root_dir.pop();
    root_dir.push("bar");
    assert!(root_dir.is_dir());
    root_dir.push("baz");
    assert!(root_dir.is_file());
    let f = File::open(&root_dir)?;
    let mut reader = BufReader::new(f);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    assert_eq!(content, "quxx");

    Ok(())
}
