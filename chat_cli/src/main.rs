use chat_client::ChatClient;

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(action) = args.next() else {
        println!("Please specify an action");
        return;
    };

    println!("Action: {}", action);

    let mut client = ChatClient::new("127.0.0.1:18080").unwrap();

    if action == "fetch" {
        let messages = client.fetch_messages().unwrap();
        println!("Chat history:");
        println!("{:?}", messages);
        return;
    }

    if action == "append" {
        let Some(message) = args.next() else {
            println!("Please specify a message");
            return;
        };

        client.add_message(message).unwrap();
        return;
    }

    println!("incorrect args!");
}
