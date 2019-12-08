use std::io::{self, Read};

fn main() {
    let mut img = String::new();
    let mut out = vec![b'2'; 25 * 6];
    io::stdin().read_to_string(&mut img).unwrap();

    img.as_bytes().chunks_exact(25 * 6).for_each(|layer| {
        layer
            .iter()
            .zip(out.iter_mut())
            .filter(|(_, o)| **o == b'2')
            .for_each(|(l, o)| *o = *l)
    });

    out.chunks_exact(25)
        .for_each(|b| println!("{}", String::from_utf8(b.into()).unwrap().replace("0", " ")));
}
