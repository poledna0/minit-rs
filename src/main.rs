use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ptr;

fn comandos(entrada: &str, console: &mut File) {
    writeln!(console, "").ok();

    match entrada {
        "clear" => {
            writeln!(console, "\x1b[2J\x1b[H").ok();
        }
        "?" => help(console),
        "batata" => {
            writeln!(console, "A batatinha quando nasce\nSe esparrama pelo chão\nMenininha quando dorme\nPõe a mão no coração").ok();
        }
        "pid" => {
            let pid = unsafe { libc::getpid() };
            writeln!(console, "O PID deste processo é: {}", pid).ok();
        }
        "sh" => sh(console),

        "uptime" => {
            let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
            unsafe { libc::sysinfo(&mut info) };

            writeln!(console, "Uptime: {}", info.uptime).ok();
        }

        _ => {
            writeln!(console, "Comando não conhecido ").ok();
        }
    }
}

fn sh(console: &mut File) {
    let pid = unsafe { libc::fork() };

    if pid == 0 {

        let path = CString::new("/bin/sh").unwrap();

        let argv = [
            path.as_ptr(),
            ptr::null(),
        ];

        let envp = [
            ptr::null(),
        ];

        unsafe {
            libc::execve(
                path.as_ptr(),
                argv.as_ptr(),
                envp.as_ptr(),
            );
        }

        // so chega aqui se execve falhar
        writeln!(console, "falha ao executar /bin/sh").ok();
        unsafe { libc::_exit(1); }

    } else if pid > 0 {
        unsafe {
            libc::waitpid(pid, ptr::null_mut(), 0);
        }

    } else {
        writeln!(console, "fork falhou").ok();
    }
}


fn help(console: &mut File) {
    writeln!(console, "Menu de ajuda: ").ok();
    writeln!(console, "clear - limpa a tela").ok();
    writeln!(console, "batata - canta musica da batatinha").ok();
    writeln!(console, "pid - mostra o PID do processo").ok();
    writeln!(console, "sh - inicia um shell").ok();
    writeln!(console, "uptime - mostra o tempo em segundos que a maquina esta ligada").ok();
    
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
        write!(console, "pato@minit:?# ").ok();
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
