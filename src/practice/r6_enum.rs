use std::net::TcpStream;
use native_tls::TlsStream;

#[derive(Debug)]
enum PockSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Heart(u8),
    Boom(u8),
}

#[test]
fn test_enum_1() {
    let clubs = PockSuit::Clubs(14);
    let diamonds = PockSuit::Diamonds(15);
    let boo = PockSuit::Boom(17);
    print_suit(clubs);
    print_suit(diamonds);
    print_suit(boo);
}

fn print_suit(suit: PockSuit) {
    println!("pocks is {:?}", suit);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn test_enum_2() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 21, y: 32 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(129, 111, 34);

    println!("{:?} \n{:?} \n{:?} \n{:?}", m1, m2, m3, m4)
}


fn handle_tcp_connection(stream: TcpStream) {
    //
}
fn handle_tls_connection(stream: TlsStream<TcpStream>) {
    //
}
//---------------------------------------------------------\\
enum Connection {
    Tcp(TcpStream),
    Tls(TlsStream<TcpStream>),
}
fn handle_connection(conn: Connection) {
    match conn {
        Connection::Tcp(stream) => {}
        Connection::Tls(stream) => {}
    }
}
