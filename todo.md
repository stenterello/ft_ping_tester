check the build prints about configuring and building other executables
add commands description in app
widgets which scroll all the way down when text is higher than widget
add check on exit code of ping and ft_ping
mancano i check di -v -? --help --usage in tests.json
rename "ping" in "ft_ping" after compare


Per appendere a file

use std::fs::OpenOptions;
use std::io::prelude::*;


let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("ciao.txt")
    .unwrap();

if let Err(e) = writeln!(file, "A new line!") {
    eprintln!("Couldn't write to file: {}", e);
}

Appunti:
    - l'organizzazione del codice è sensata?
        .in merito ai tratti, è corretto implementarli per le singole struct nel file delle struct piuttosto che nei file dei tratti?
    - tipo di ragionamento usato per il Comparer... **lifetimes**
