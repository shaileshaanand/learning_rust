#[derive(Debug)]
enum IPAddrKind {
    V6,
    V4,
}

fn main() {
    let myip1 = IPAddrKind::V4;
    let myip2 = IPAddrKind::V6;

    println!("{:?} {:?}", myip1, myip2);
    route(myip1);
}

fn route(ip: IPAddrKind) {
    println!("{:?}", ip);
}
    let myip1=IPAddrKind::V4;
