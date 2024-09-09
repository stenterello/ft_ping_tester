check the build prints about configuring and building other executables
add commands description in app
widgets which scroll all the way down when text is higher than widget
add check on exit code of ping and ft_ping
mancano i check di -v -? --help --usage in tests.json


Per appendere a file

use std::fs::OpenOptions;
use std::io::prelude::*;


let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("my-file")
    .unwrap();

if let Err(e) = writeln!(file, "A new line!") {
    eprintln!("Couldn't write to file: {}", e);
}
