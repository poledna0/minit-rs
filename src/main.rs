use std::ffi::CString;
use std::fs;
use std::process::exit;

fn mount_fs(source: &str, target: &str, fstype: &str, data: Option<&str>) {
    let source = CString::new(source).unwrap(); // Cria uma string C (\0 no final)
    let target = CString::new(target).unwrap();
    let fstype = CString::new(fstype).unwrap();
    let data = match data {
        // se tiver alg coisa converte para C string se tiver None, continua none
        Some(d) => Some(CString::new(d).unwrap()),
        None => None,
};

// chamada direta FFI


/*
int mount(
    const char *source,
    const char *target,
    const char *filesystemtype,
    unsigned long mountflags,
    const void *data
);
*/
    unsafe {

        let data_ptr = match &data {
            Some(d) => d.as_ptr(),
            None => std::ptr::null(),
        };

        let data_ptr = data_ptr as *const libc::c_void;

            /*
            no momento source é um CString
            CString guarda uma string terminada em \0, como C exige

            as_ptr()
            Retorna um ponteiro C

            *const c_char

            ponteiro cru, diferente em rust q tem metadados no ponteiro, a n ser os primitivos, em rust &x q tem o valor 10, tem 
            só o apontamento para a memoria, já em rust &string_variavel ele teria o valrod a string, o tamanho, borrow....n é uma coisa direta
            para em um objeto ante
             */


        libc::mount(
            source.as_ptr(),
            target.as_ptr(),
            fstype.as_ptr(),
            0,
            data_ptr,
        );
    }
}

fn main() {

    if unsafe { libc::getpid() } != 1 {
        eprintln!("não sou PID 1");
        exit(1);
    }
    


    fs::create_dir_all("/proc").ok();
    fs::create_dir_all("/sys").ok();
    fs::create_dir_all("/dev").ok();
    fs::create_dir_all("/run").ok();
    fs::create_dir_all("/dev/pts").ok();
    fs::create_dir_all("/dev/shm").ok();


    mount_fs("proc", "/proc", "proc", None);
    mount_fs("sysfs", "/sys", "sysfs", None);
    mount_fs("devtmpfs", "/dev", "devtmpfs", None);
    mount_fs("tmpfs", "/run", "tmpfs", Some("mode=0755"));
    mount_fs("devpts", "/dev/pts", "devpts", None);
    mount_fs("tmpfs", "/dev/shm", "tmpfs", None);


    unsafe {
        libc::signal(libc::SIGCHLD, libc::SIG_IGN);
    }


    let init = CString::new("/sbin/init").unwrap();
    let argv = [init.as_ptr(), std::ptr::null()];

    unsafe {
        libc::execv(init.as_ptr(), argv.as_ptr());
    }


    loop {
        unsafe { libc::pause() };
    }
}
