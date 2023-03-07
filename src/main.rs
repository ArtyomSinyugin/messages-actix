use messages_actix::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()>{  // стр. 46

    let app = MessageApp::new(8000);
    app.run().await
}

// http://127.0.0.1:8000/