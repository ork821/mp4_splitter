pub struct Trak {
    id : u32,
    duration : u32,
    timescale : u32,
    trak_type : &str,
    stts : Vec<u8>,
    stsc : Vec<u8>,
    stsz : Vec<u8>,
    stco : Vec<u8>,
    stss : Vec<u8>,
    durations : Vec<u8>,
    chunk_numbers : Vec<u8>,
    stsd : Vec<u8>,
    tkhd : Vec<u8>
}

impl Trak {
    pub fn parse(data : Vec<u8>) -> Trak {
        let mut buffer = Buffer::from(data);
        let mut counter : usize = 8;
        buffer.move_offset_forward(8);



        while counter < buffer.len() {
            let len = buffer.get_u32();
            let name = buffer.decode_next_bytes(4);
            let data = buffer.slice(counter, counter + len as usize).to_vec();
            buffer.move_offset_forward(1);
            counter += 1;

            match name.as_str() {
                
            }
        }
    }
}
