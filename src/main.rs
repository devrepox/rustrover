pub mod cxxqt_object;

use std::io::Read;
use std::thread;
use std::net::TcpListener;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }
    thread::spawn(|| {
        tcp();
    });
    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }

}
fn tcp() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established! from");
        println!("{:?}", stream.peer_addr().unwrap().port())
    }
}
