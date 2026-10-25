use super::{Cancellation, ResolvedStream, transport::Transport};
use std::io::{self, Read, Seek, SeekFrom};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct StreamInput {
    transport: Transport,
    stream: ResolvedStream,
    cancel: Cancellation,
    position: u64,
    buffer: Vec<u8>,
    at: usize,
    failed: Arc<AtomicBool>,
}

impl StreamInput {
    pub(super) fn new(transport: Transport, stream: ResolvedStream, cancel: Cancellation) -> Self {
        Self {
            transport,
            stream,
            cancel,
            position: 0,
            buffer: Vec::new(),
            at: 0,
            failed: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn failure_flag(&self) -> Arc<AtomicBool> {
        self.failed.clone()
    }
}

impl Read for StreamInput {
    fn read(&mut self, destination: &mut [u8]) -> io::Result<usize> {
        self.cancel
            .check()
            .map_err(|_| io::Error::new(io::ErrorKind::ConnectionAborted, "stream_cancelled"))?;
        if destination.is_empty() || self.position == self.stream.length {
            return Ok(0);
        }
        if self.at == self.buffer.len() {
            let chunk = if self.position == 0 {
                64 * 1024
            } else {
                256 * 1024
            };
            let end = self
                .position
                .saturating_add(chunk - 1)
                .min(self.stream.length - 1);
            self.buffer = self
                .transport
                .range(
                    &self.stream.url,
                    self.position,
                    end,
                    self.stream.length,
                    &self.cancel,
                )
                .map_err(|_| {
                    if self.cancel.check().is_ok() {
                        self.failed.store(true, Ordering::Release);
                    }
                    io::Error::other("stream_fetch_failed")
                })?;
            self.at = 0;
        }
        let count = destination.len().min(self.buffer.len() - self.at);
        destination[..count].copy_from_slice(&self.buffer[self.at..self.at + count]);
        self.at += count;
        self.position += count as u64;
        Ok(count)
    }
}

impl Seek for StreamInput {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        if matches!(position, SeekFrom::Current(0)) {
            Ok(self.position)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "progressive_stream_cannot_seek",
            ))
        }
    }
}
