/* use std::str;
use std::io::Write;
use std::fmt::Debug; */

pub mod request;
pub mod response;

/* pub struct Buffer {
    pub data: Vec<u8>,
    pub len: usize
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        let writed_len = buf.len();
        self.len += writed_len;
        Ok(writed_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Debug for Buffer {
    fn fmt(&self, buffer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let readable_data = str::from_utf8(&self.data).unwrap();

        buffer.debug_struct("Buffer")
            .field("Data", &readable_data)
            .field("Data Length", &self.len)
            .finish()
    }
} */