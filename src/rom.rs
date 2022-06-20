#[derive(Debug)]
pub struct Rom {
    inner: Vec<u8>,
    len: usize,
}

impl Rom {
    pub fn new(inner: Vec<u8>) -> Self {
        let len = inner.len();

        Self { inner, len }
    }

    pub fn len(&self) -> usize {
        self.len
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
