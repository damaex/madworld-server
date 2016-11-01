use super::rustc_serialize::json::{self};
use super::ws;

mod types;
mod database;

pub struct Server {
    pub out: ws::Sender,
    #[cfg(feature = "ssl")]
    pub ssl: Rc<SslContext>,
}

impl ws::Handler for Server {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Server got message '{}'. ", msg);

        let x = msg.as_text();
        if x.is_ok() {
            println!("first okay");
            let check = json::decode(&x.unwrap());
            if check.is_ok() {
                let decoded: types::Message = check.unwrap();
                println!("Server decodec: {}", decoded);
            } else {
                return self.out.close(ws::CloseCode::Error);
            }
        } else {
            return self.out.close(ws::CloseCode::Error);
        }

        let object = types::Message {
            main: 0,
            sub: 1,
            data: "OK".to_string(),
        };

        // Serialize using `json::encode`
        let encoded = json::encode(&object).unwrap();
        let answer = ws::Message::from(encoded);

        self.out.send(answer)
    }

    #[cfg(feature = "ssl")]
    fn build_ssl(&mut self) -> ws::Result<Ssl> {
        Ssl::new(&self.ssl).map_err(ws::Error::from)
    }
}