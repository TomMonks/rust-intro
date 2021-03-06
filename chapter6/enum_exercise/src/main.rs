/*
enum type for internet protocol type
Note that we can also attach data to the instance
of the enum and data can vary depending on the type.
*/
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);
}


fn route(ip_kind: IpAddrKind) {

}
