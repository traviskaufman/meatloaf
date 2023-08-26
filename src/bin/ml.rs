use clap::Parser;

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Cli {
    /// Command line to start child process
    run: Vec<String>,
}

fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let cli = Cli::parse();
    let mut msg = zmq::Message::new();

    let cmd = match cli.run.join(" ") {
        c if !c.is_empty() => c,
        _ => String::from("echo 'Hello, World!'"),
    };
    requester.send(cmd.as_str(), 0).expect("Send failed");

    requester.recv(&mut msg, 0).expect("recv failed :(");
    println!("{}", msg.as_str().unwrap_or(""));
}
