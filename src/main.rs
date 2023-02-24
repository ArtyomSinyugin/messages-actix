use messages_actix::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()>{  // стр. 46
    std::env::set_var("RUST_LOG", "actix_web=info");  // первое - это ключ (key), второе - значение (value). Предположу, что где-то (например, в консоли) написать ключ, то выводится значение
    env_logger::init();
    let app = MessageApp::new(8000);
    app.run().await
}

// http://127.0.0.1:8000/