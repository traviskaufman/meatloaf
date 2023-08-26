use clap::Parser;

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Cli {
    /// Command line to start child process
    run: Vec<String>,
}

fn say_hello() {
    let rainbow = rainbow_text::Rainbow::default();
    let _ = rainbow.write("🟫 MEATLOAF v0\n");
    println!();
    println!("To get started, try running a command, like:");
    println!("ml echo 'Hello, world!'");
}

fn main() {
    let cli = Cli::parse();

    let cmd = cli.run.join(" ");
    if cmd.is_empty() {
        say_hello();
        return;
    }

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    let mut msg = zmq::Message::new();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    requester.send(cmd.as_str(), 0).expect("Send failed");
    requester.recv(&mut msg, 0).expect("recv failed :(");
    println!("{}", msg.as_str().unwrap_or(""));
}
