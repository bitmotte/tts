mod symbols;

use std::{fs::File, io::BufReader};
use rodio::{Decoder, OutputStreamBuilder, Sink};

fn main() {
    let stream_handler = OutputStreamBuilder::open_default_stream().expect("no stream");
    let sink = Sink::connect_new(&stream_handler.mixer());

    let file = File::open("audio/.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
