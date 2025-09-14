use stp::server::StpServer;

struct Chat {
    messages: Vec<String>,
}
fn main() {
    let addr = "127.0.0.1:18080";
    let mut server = StpServer::bind(addr).unwrap();
    let mut chat = Chat::new();

    loop {
        let Ok(mut connection) = server.accept() else {
            println!("failed to accept connection");
            continue;
        };

        if let Err(e) = connection.process_request(|request| process_request_inner(request, &mut chat)) {
            println!("failed to process chat request: {:?}", e);
        };
    }
}

fn process_request_inner(request: String, chat: &mut Chat) -> String {
    if request == "fetch" {
        return chat.fetch_messages();
    }

    if let Some(msg) = request.strip_prefix("append:") {
        chat.add_message(msg.to_string());
        return "ok".to_string();
    }

    "error".to_string()
}

impl Chat {
    fn new() -> Chat {
        Chat { messages: Vec::new() }
    }

    fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    fn fetch_messages(&self) -> String {
        self.messages
            .iter()
            .map(|msg| format!("— {}\n", msg))
            .collect()
    }
}