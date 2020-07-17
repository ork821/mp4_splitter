use crate::trak::Trak;

#[derive(Debug)]
pub struct Parser {
    ftyp : Vec<u8>,
    mvhd : Vec<u8>,
    traks : Vec<Trak>
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ftyp : vec![],
            mvhd : vec![],
            traks : Vec::with_capacity(2)
        }
    }

    pub fn set_ftyp(&mut self, ftyp : Vec<u8>) {
        self.ftyp = ftyp;
    }

    pub fn get_ftyp(&self) -> Vec<u8> {
        self.ftyp.to_vec()
    }

     pub fn set_mvhd(&mut self, mvhd : Vec<u8>) {
        self.mvhd = mvhd;
    }

    pub fn get_mvhd(&self) -> Vec<u8> {
        self.mvhd.to_vec()
    }

    pub fn add_trak(&mut self, trak : Trak) {
        self.traks.push(trak);
    }
}
