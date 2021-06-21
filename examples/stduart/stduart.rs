/*
 *  stduart.rs
 *  Copyright 2021 ItJustWorksTM
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use std::result::Result::Ok;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;
use std::{env, io, thread};

use smce_rs::board::Board;
use smce_rs::board_config::{BoardConfig, SecureDigitalStorage, UartChannel};
use smce_rs::sketch::Sketch;
use smce_rs::sketch_config::Library::RemoteArduinoLibrary;
use smce_rs::sketch_config::SketchConfig;

use anyhow::anyhow;
use smce_rs::toolchain::Toolchain;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() != 3 {
        return Err(anyhow!(
            "Usage:  <fully-qualified-board-name> <path-to-sketch>"
        ));
    }

    let home = PathBuf::from(env!("OUT_DIR"));

    let mut sketch = Sketch::new(
        &PathBuf::from(args[2].clone()),
        SketchConfig {
            fqbn: args[1].clone(),
            preproc_libs: vec![
                RemoteArduinoLibrary {
                    name: "MQTT".into(),
                    version: "2.5.0".into(),
                },
                RemoteArduinoLibrary {
                    name: "WiFi".into(),
                    version: "1.2.7".into(),
                },
            ],
            ..Default::default()
        },
    )
    .expect("Failed to create Sketch");

    let (tc, mut log) = Toolchain::new(&home);

    let compile_handle = thread::spawn(move || {
        println!("Compiling...");
        let res = tc.compile(&mut sketch);
        (sketch, res)
    });

    let mut log_str = String::new();
    while let Ok(read) = log.read_to_string(&mut log_str) {
        if read > 0 {
            print!("{}", log_str);
            log_str.clear();
        } else if log.disconnected() {
            break;
        }
    }

    let (sketch, res) = compile_handle.join().unwrap();

    res?;

    println!("Done");

    let mut board = Board::new();
    let handle = board.start(
        &BoardConfig {
            uart_channels: vec![UartChannel {
                tx_buffer_length: 512,
                rx_buffer_length: 512,
                ..Default::default()
            }],
            sd_cards: vec![SecureDigitalStorage {
                cspin: 0,
                root_dir: ".".into(),
            }],
            ..Default::default()
        },
        &sketch,
    )?;

    assert_eq!(handle.view().uart_channels.len(), 1);

    let (sender, receiver) = channel();

    thread::spawn(move || loop {
        let mut line = String::new();
        print!("$> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        line.pop(); // pop away the new line
        if line.is_empty() || line == "\n" || line == "~QUIT" || sender.send(line).is_err() {
            println!("EXITING THREAD!");
            return;
        }
    });

    {
        let mut uart0 = &handle.view().uart_channels[0];
        let mut uart0_writebuf = BufWriter::new(uart0);
        let mut read_buf = String::new();
        loop {
            if uart0.read_to_string(&mut read_buf).unwrap() > 0 {
                println!("arduino: \"{}\"", read_buf.escape_default());
                read_buf.clear();
            }

            match receiver.try_recv() {
                Ok(line) => {
                    let _ = uart0_writebuf.write(line.as_bytes());
                }
                Err(TryRecvError::Disconnected) => {
                    break;
                }
                _ => {}
            }

            thread::sleep(Duration::from_millis(1));

            if let Err(exit_code) = handle.tick() {
                break;
            }

            let _ = uart0_writebuf.flush();
        }
    }

    println!("Stopped with exit code: {}", handle.stop());

    Ok(())
}
