use crate::io::Read;

#[derive(Debug)]
pub struct Rom(Vec<u8>);

impl Rom {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<[u8]> for Rom {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Read for Rom {
    fn read_byte(&self, addr: u16) -> crate::error::Result<u8> {
        Ok(self.0[addr as usize])
    }
}
