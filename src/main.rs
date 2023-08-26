use std::process::{Command, Stdio};

fn run_command(s: &str) -> Result<String, String> {
    let mut parts = s.split(" ");
    let cmd = parts
        .next()
        .ok_or(String::from("Failed to get command name"))?;
    let output = Command::new(cmd)
        .args(parts)
        .stdout(Stdio::piped())
        .output()
        .or(Err(String::from("Could not spawn command")))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());
    println!("Awaiting commands on port :5555");
    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        let s = msg.as_str().unwrap();
        println!("Exec: {}", s);
        match run_command(s) {
            Ok(res) => responder.send(res.as_str(), 0).unwrap(),
            Err(msg) => responder
                .send(format!("Error: {}", msg).as_str(), 0)
                .unwrap(),
        }
    }
}
