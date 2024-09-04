use std::process::Command;
use std::path::Path;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn  main() {

    p!("cargo:rerun-if-changed=NULL");

    if !Path::new("inetutils-2.0.tar.xz").exists() {
        p!("Downloading inetutils-2.0...");

        Command::new("bash")
            .arg("-c")
            .arg("wget https://ftp.gnu.org/gnu/inetutils/inetutils-2.0.tar.xz")
            .output()
            .expect("failed to execute wget");

        p!("Downloaded inetutils-2.0 succesfully");
        p!("Extracting package...");
    
        Command::new("bash")
            .arg("-c")
            .arg("tar xf inetutils-2.0.tar.xz")
            .output()
            .expect("failed to execute extraction");
    
        p!("Downloaded inetutils-2.0 succesfully");
    }


    if !Path::new("inetutils-2.0/ping/Makefile").exists() {
        p!("Configuring inetutils-2.0...");

        Command::new("bash")
            .arg("-c")
            .arg("pushd inetutils-2.0; ./configure; popd;")
            .output()
            .expect("failed to configure inetutils-2.0");

        p!("Configured inetutils-2.0 succesfully");
    }

    p!("Compiling inetutils-2.0...");

    Command::new("bash")
        .arg("-c")
        .arg("pushd inetutils-2.0; make; popd;")
        .output()
        .expect("failed to compile inetutils-2.0");

    p!("Compiling inetutils-2.0 succesfully");

    p!("Compiling ft_ping...");
    Command::new("bash")
        .arg("-c")
        .arg("make -C ../ft_ping")
        .output()
        .expect("failed to compile ft_ping");

    p!("Compiled ft_ping succesfully");
}

