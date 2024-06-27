use std::net::TcpListener;

fn main() {
    // 监听传入的流并在接收到流时打印信息
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

// 在浏览器中打开 127.0.0.1:7878