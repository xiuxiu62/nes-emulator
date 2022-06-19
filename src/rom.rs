#[derive(Debug)]
pub struct Rom {
    inner: Vec<u8>,
    len: Option<usize>,
}

impl Rom {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { inner, len: None }
    }

    pub fn len(&mut self) -> usize {
        match self.len {
            Some(len) => len,
            None => {
                let len = self.inner.len();
                self.len = Some(len);

                len
            }
        }
    }
}

impl AsRef<[u8]> for Rom {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

#[macro_export]
macro_rules! rom {
    [$($byte:expr),*] => {
        Rom::new(vec![$($byte),*])
    };
}
