use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::process::exit;

fn main() {

    if unsafe { libc::getpid() } != 1 {
        eprintln!("não sou PID 1");
        exit(1);
    }

    let mut console = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/console")
        .expect("falha ao abrir /dev/console");

    writeln!(console, "INIT PID 1 rodando").ok();
    writeln!(console, "Digite algo e pressione Enter:\n").ok();

    let mut input = String::new();

    loop {
        input.clear();

        // Lê uma linha inteira
        let mut buf = [0u8; 1];
        while console.read(&mut buf).unwrap() == 1 {
            if buf[0] == b'\n' {
                break;
            }
            input.push(buf[0] as char);
        }

        // Repete
        writeln!(console, "{}", input).ok();
    }
}
