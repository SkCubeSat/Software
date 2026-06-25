//! Mock objects for use with CAN HAL unit tests.

use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Structure containing the input data to verify and/or result to return
/// when the MockStream's write function is called.
pub struct WriteStruct {
    input: RefCell<VecDeque<CanFrame>>,
    result: CanResult<()>,
}

impl WriteStruct {
    /// Set the result to be returned for any write() calls.
    pub fn set_result(&mut self, result: CanResult<()>) {
        self.result = result;
    }

    /// Set the input to validate for any write() calls.
    pub fn set_input(&mut self, input: CanFrame) {
        self.input.borrow_mut().push_back(input)
    }
}

/// Structure containing the output data or result to return
/// when the MockStream's read function is called.
pub struct ReadStruct {
    output: RefCell<VecDeque<CanFrame>>,
    result: CanResult<CanFrame>,
}

impl ReadStruct {
    /// Set the result to be returned for any read() calls.
    pub fn set_result(&mut self, result: CanResult<CanFrame>) {
        self.result = result;
    }

    /// Set the output frames returned by future read() calls.
    pub fn set_output(&mut self, output: Vec<CanFrame>) {
        self.output = RefCell::new(output.into())
    }
}

/// Mock object for simulating a CAN stream.
pub struct MockStream {
    /// Information to use when write() calls are made.
    pub write: WriteStruct,
    /// Information to use when read() calls are made.
    pub read: ReadStruct,
}

impl Default for MockStream {
    fn default() -> Self {
        MockStream {
            write: WriteStruct {
                result: Err(CanError::Timeout),
                input: RefCell::new(VecDeque::new()),
            },
            read: ReadStruct {
                result: Err(CanError::Timeout),
                output: RefCell::new(VecDeque::new()),
            },
        }
    }
}

impl Stream for MockStream {
    fn write(&self, frame: CanFrame) -> CanResult<()> {
        if self.write.input.borrow().is_empty() {
            self.write.result.clone()
        } else {
            let input = self.write.input.borrow_mut().pop_front().unwrap();
            assert_eq!(input, frame);
            Ok(())
        }
    }

    fn read(&self, _timeout: Duration) -> CanResult<CanFrame> {
        if let Some(frame) = self.read.output.borrow_mut().pop_front() {
            Ok(frame)
        } else {
            self.read.result.clone()
        }
    }

    fn read_frames(&self, count: usize, timeout: Duration) -> CanResult<Vec<CanFrame>> {
        let start = Instant::now();
        let mut frames = Vec::with_capacity(count);

        while frames.len() < count {
            if Instant::now() - start > timeout {
                return Err(CanError::Timeout);
            }

            frames.push(self.read(timeout)?);
        }

        Ok(frames)
    }

    fn read_payload(
        &self,
        expected_len: usize,
        timeout: Duration,
        filter: Option<FrameFilter>,
    ) -> CanResult<Vec<u8>> {
        let start = Instant::now();
        let mut payload = Vec::with_capacity(expected_len);

        while payload.len() < expected_len {
            if Instant::now() - start > timeout {
                return Err(CanError::Timeout);
            }

            let frame = self.read(timeout)?;
            if filter
                .as_ref()
                .map_or(true, |filter| filter.matches(&frame))
            {
                payload.extend_from_slice(&frame.data);
            }
        }

        payload.truncate(expected_len);
        Ok(payload)
    }
}
