add commands description in app
widgets which scroll all the way down when text is higher than widget
terminate threads when exit error handling widget

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
