use std::fs;
use byte_buffer::Buffer;
mod mp4_parser;
use mp4_parser::Parser;
mod trak;
use trak::Trak;

fn main() {
    println!("Hello, world!");
    split("a-f.mp4");
}

pub fn split(path : &str) {
    let data = fs::read(path).unwrap();
    let mut buffer = Buffer::from(data);
    let mut counter : usize = 0;
    let mut parser = Parser::new();
    while counter < buffer.len() {
        let len : usize = buffer.get_u32() as usize;
        let name = buffer.decode_next_bytes(4);
        buffer.move_offset_back(4);
        buffer.move_offset_forward(len);
        let atom_buffer = buffer.slice(counter, counter + len).to_vec();
        counter += len;
        analyze_atom(name.as_str(), atom_buffer, &mut parser);
    }
    println!("{:?}", parser);
}

fn analyze_atom(name : &str, buffer : Vec<u8>, parser : &mut Parser) {
    match name {
        "ftyp" => parser.set_ftyp(buffer),
        "moov" => parse_moov(buffer, parser),
        _ => ()
    }

}

fn parse_moov(data : Vec<u8>, parser : &mut Parser) {
    let mut buffer = Buffer::from(data);
    let mut counter : usize = 8;
    buffer.move_offset_forward(8);
    while counter < buffer.len() {
        let len = buffer.get_u32();
        let name = buffer.decode_next_bytes(4);
        let data = buffer.slice(counter, counter + len as usize).to_vec();
        buffer.move_offset_forward((len - 4) as usize);

        match name.as_str() {
            "mvhd" => parser.set_mvhd(data),
            "trak" => {
                let trak = Trak::new(data);
                parser.add_trak(trak);
            }
            _ => ()
        }

    }
}
