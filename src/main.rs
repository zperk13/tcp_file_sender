use std::io::prelude::*;
use std::net::ToSocketAddrs;

#[derive(Clone, Copy)]
enum SenderOrReceiver {
    Sender,
    Receiver,
}

impl std::fmt::Display for SenderOrReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SenderOrReceiver::Sender => write!(f, "Sender"),
            SenderOrReceiver::Receiver => write!(f, "Receiver"),
        }
    }
}

impl SenderOrReceiver {
    fn variants() -> [SenderOrReceiver; 2] {
        [SenderOrReceiver::Sender, SenderOrReceiver::Receiver]
    }
}

fn main() {
    match inquire::Select::new("Sender or Receiver?", SenderOrReceiver::variants().to_vec())
        .prompt()
        .unwrap()
    {
        SenderOrReceiver::Sender => run_sender(),
        SenderOrReceiver::Receiver => run_receiver(),
    }
}

fn get_ip() -> std::net::SocketAddr {
    loop {
        let s = inquire::Text::new("What is the ip? (include the port 526)")
            .prompt()
            .unwrap();
        match s.to_socket_addrs() {
            Ok(mut sa) => return sa.next().unwrap(),
            Err(e) => eprintln!("{e}"),
        }
    }
}

#[derive(Clone, Copy)]
struct FileAutocomplete;

impl inquire::Autocomplete for FileAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        Ok(std::env::current_dir()?
            .read_dir()?
            .filter_map(|de| {
                let s = de.ok()?.file_name().to_str()?.to_string();
                if s.contains(input) {
                    Some(s)
                } else {
                    None
                }
            })
            .collect())
    }

    fn get_completion(
        &mut self,
        _input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        Ok(Some(highlighted_suggestion.unwrap()))
    }
}

fn get_file() -> std::fs::File {
    loop {
        let s = inquire::Text::new("What file?")
            .with_autocomplete(FileAutocomplete {})
            .prompt()
            .unwrap();
        match std::fs::File::options().write(true).read(true).open(s) {
            Ok(f) => return f,
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn run_sender() {
    let mut file = get_file();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("Sending {} bytes", buf.len());
    std::net::TcpStream::connect(get_ip())
        .unwrap()
        .write_all(&buf)
        .unwrap();
    println!("Done");
}

fn run_receiver() {
    let mut file = get_file();
    let (mut stream, _) = std::net::TcpListener::bind("0.0.0.0:526")
        .unwrap()
        .accept()
        .unwrap();
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    println!("{} bytes received, writing to file", buffer.len());
    file.write_all(&buffer).unwrap();
    println!("Done");
}
