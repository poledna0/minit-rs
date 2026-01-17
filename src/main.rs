use std::fs::{File, OpenOptions};
use std::io::{Write, Read};


fn comandos ( entrada : &str, console: &mut File ) {
    match entrada {
        "clear" => { writeln!(console, "\x1b[2J\x1b[H").ok(); }
        "?" => help(console),
        "batata" => {
            writeln!(console, "A batatinha quando nasce\nSe esparrama pelo chão\nMenininha quando dorme\nPõe a mão no coração").ok();
        }
        _ => { writeln!(console, "Comando não conhecido ").ok(); },
    }
    
}

fn help(console: &mut File){
    writeln!(console, "Menu de ajuda: ").ok();
    writeln!(console, "clear - limpa a tela").ok();
    writeln!(console, "batata - canta musica da batatinha").ok();
}


fn main() {

    let mut console = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/console")
        .expect("falha ao abrir /dev/console");

    writeln!(console, "Rodando como PID1").ok();

    let mut input = String::new();

    loop {

        input.clear();
        let mut buf = [0u8; 1];
        while console.read(&mut buf).unwrap() == 1 {
            if buf[0] == b'\n' {
                break;
            }

            input.push(buf[0] as char);
        }
        // enviar comando para o console
        comandos(&input, &mut console);
    }
}
