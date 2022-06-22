use crate::io::Read;

#[derive(Debug, Clone)]
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

    pub fn iter(&self) -> impl Iterator<Item = &u8> + '_ {
        self.as_ref().iter()
    }
}

impl AsRef<[u8]> for Rom {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Read for Rom {
    fn read_byte(&mut self, addr: u16) -> crate::error::Result<u8> {
        Ok(self.0[addr as usize])
    }
}

impl IntoIterator for Rom {
    type Item = u8;
    type IntoIter = RomIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            data: self.0,
            index: 0,
        }
    }
}

pub struct RomIntoIterator {
    data: Vec<u8>,
    index: usize,
}

impl Iterator for RomIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.index).cloned();
        if result.is_some() {
            self.index += 1;
        }

        result
    }
}
