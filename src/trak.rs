use byte_buffer::Buffer;

pub struct Trak {
    id : Option<u32>,
    duration : Option<u32>,
    timescale : Option<u32>,
    trak_type : Option<&'static str>,
    stts : Option<Vec<u32>>,
    stsc : Option<Vec<u32>>,
    stsz : Option<Vec<u32>>,
    stco : Option<Vec<u32>>,
    stss : Option<Vec<u32>>,
    durations : Option<Vec<u32>>,
    chunk_numbers : Option<Vec<u32>>,
    stsd : Option<Vec<u8>>,
    tkhd : Option<Vec<u8>>
}

impl Trak {
    pub fn new(data : Vec<u8>) -> Trak {
        let trak = Trak{
            id: None,
            duration: None,
            timescale: None,
            trak_type: None,
            stts: None,
            stsc: None,
            stsz: None,
            stco: None,
            stss: None,
            durations: None,
            chunk_numbers: None,
            stsd: None,
            tkhd: None
        };
        trak.parse(data);
        trak
    }

    fn parse(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        let mut counter : usize = 8;
        buffer.move_offset_forward(8);


        while counter < buffer.len() {
            let len = buffer.get_u32();
            let name = buffer.decode_next_bytes(4);
            let data = buffer.slice(counter, counter + len as usize).to_vec();
            buffer.move_offset_forward(1);
            analyse_name(name, data);
            counter += 1;            
        }

        match &self.stss {
            Some(stss) => calculate_data(),
            None => ()
        };

    }

    fn analyse_name(&mut self, name : &str, data : Vec<u8>) {
        match name.as_str() {
            "tkhd" => {
                let mut buffer = Buffer::from(data);
                buffer.move_offset_forward(4 + 4 + 1 + 3 + 4 + 4);
                self.id = Some(buffer.get_u32());
            },
            "mdhd" => {
                let mut buffer = Buffer::from(data);
                buffer.move_offset_forward(4 + 4 + 1 + 3 + 4 + 4);
                self.timescale = Some(buffer.get_u32());
                self.duration = Some(buffer.get_u32());
            },
            "smhd" => self.trak_type = Some("audio"),
            "stts" => extractStts(data),
            "stsz" => extractStsz(data),
            "stco" => extractStco(data),
            "stsd" => self.stsd = Some(data),
            "stsc" => extractStsc(data),
            "stss" => extractStss(data),
            _ => ()
        }
    }

    pub fn calculate_data(&mut self) {
        /* if (this.stss.length === 0) return;
        if (this.chunkNumbers.length !== 1) return;
        let counter = 0;
        for (let i = 1; i < this.stss.length; i++) {
            let count = this.stss[i] - this.stss[i-1];
            let number = 0;
            while (number < count) {
                number += this.stsc[counter++];
            }
            this.chunkNumbers.push(counter + 1);
        } */
        let stss = self.stss.unwrap();
    }

    fn extract_stts(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        buffer.move_offset_forward(4 + 4 + 1 + 3);
        let entry_count = buffer.getInt32();
        let mut stts = Vec::with_capacity(2 * entry_count);
        for i in 0..entry_count {
            let count = buffer.getInt32();
            let delta = buffer.getInt32();

            for j in 0..count {
                stts.push(delta);
            }
        }
        self.stts = Some(stts);
    }

    fn extract_stsz(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        buffer.move_offset_forward(4 + 4 + 1 + 3);

        let sample_size = buffer.getInt32();
        let entry_count = buffer.getInt32();

        let mut stsz = Vec::with_capacity(entry_count);
        if sample_size == 0 {
            for i in 0..entry_count {
                let value = buffer.get_u32();
                stsz.push(value);
            }
        } else {
            for i in 0..entry_count {
                stsz.push(sample_size);
            }
        }
        self.stsz = Some(stsz);
    }
    
    fn extract_stco(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        buffer.move_offset_forward(4 + 4 + 1 + 3);

        let entry_count = buffer.get_u32();
        let mut stco = Vec::with_capacity(entry_count as usize);
        for i in 0..entry_count {
            let offset = buffer.get_u32();
            stco.push(offset);
        }
        self.stco = Some(stco);
    }

    fn extract_stsc(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        buffer.move_offset_forward(4 + 4 + 4);

        let entry_count = buffer.get_u32();
        let mut stsc = Vec::with_capacity((entry_count * 5) as usize);
        for i in 1..entry_count {
            let first_chunk = buffer.get_u32();
            let samples_per_chuck = buffer.get_u32();
            buffer.move_offset_forward(4);
            let first_chunk_next = buffer.get_u32();
            for j in first_chunk..first_chunk_next {
                stsc.push(samples_per_chuck);
            }
            if (i == entry_count - 1) {
                this.stsc.push(buffer.ger);
            }

            buffer.move_offset_back(4);
        }
    }

    fn extract_stss(&mut self, data : Vec<u8>) {
        let mut buffer = Buffer::from(data);
        buffer.move_offset_forward(4 + 4 + 4);

        let entry_count = buffer.get_u32();
        let mut stss = Vec::with_capacity(entry_count as usize);
        for i in 0..entry_count {
            let sample_number = buffer.get_u32();
            stss.push(sample_number);
        }
        let last = self.stts.unwrap().len() as u32;
        stss.push(last);
        self.stss = Some(stss);
    }
}
