fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut bytes = Vec::new();
    for i in s.chars() {
        let len = bytes.len();
        bytes.resize(len + i.len_utf8(), 0u8);
        i.encode_utf8(&mut bytes[len..]);
        bytes.push(0xe3);
        bytes.push(0x82);
        bytes.push(0x99);
    }
    println!("{}", String::from_utf8(bytes).unwrap());
}
