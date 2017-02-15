use std::env::args;

fn usage() {
  let name = args().nth(0).unwrap_or("psilocybin".into()); {
    println!("{:?} mpd visualizer", name);
    println!("--file\tpath to mpd's fifo");
    println!("--rate\tredraw rate");
    println!("--help\tshow this message");
  };
}

#[derive(Debug)]
pub struct VisualizerOptions {
  fifo: String,
  rate: u16
}

fn main() {
  let mut opts = VisualizerOptions {
    fifo: "/tmp/mpd.fifo".into(),
    rate: 25
  };

  let mut iterator = args().skip(1).peekable();

  while let Some(arg) = iterator.next() {
    if arg == "--help" || arg == "-h" {
      usage();
      return
    }

    else if arg == "--file" || arg == "-f" {
      opts.fifo = iterator.peek().expect("No filename given!").clone();
    }

    else if arg == "--rate" || arg == "-r" {
      opts.rate = iterator.peek()
        .expect("No rate given!")
        .parse()
        .expect("Non-numeric rate given!");
    }
  }

  println!("{:?}", opts);
}