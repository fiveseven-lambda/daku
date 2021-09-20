fn main() {
    loop {
        let mut s = String::new();
        if std::io::stdin().read_line(&mut s).unwrap() == 0 {
            break;
        }
        let mut bytes = Vec::new();
        for i in s.chars() {
            let len = bytes.len();
            bytes.resize(len + i.len_utf8(), 0u8);
            i.encode_utf8(&mut bytes[len..]);
            bytes.extend([0xe3, 0x82, 0x99]);
        }
        println!("{}", String::from_utf8(bytes).unwrap());
    }
}
