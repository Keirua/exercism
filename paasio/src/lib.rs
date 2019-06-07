use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    nb_reads: usize,
    nb_bytes: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats{wrapped, nb_reads: 0, nb_bytes: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nb_bytes
    }

    pub fn reads(&self) -> usize {
        self.nb_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.wrapped.read(buf).and_then(|r:usize| {
            self.nb_reads += 1;
            self.nb_bytes += r;
            Ok(r)
        })
    }
}

pub struct WriteStats<W>{
    wrapped: W,
    nb_writes: usize,
    nb_bytes: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats{wrapped, nb_writes: 0, nb_bytes: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nb_bytes
    }

    pub fn writes(&self) -> usize {
        self.nb_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.wrapped.write(buf).and_then(|w:usize| {
            self.nb_writes += 1;
            self.nb_bytes += w;
            Ok(w)
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
