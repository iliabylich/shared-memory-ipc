pub(crate) struct Queue<const N: usize> {
    start: usize,
    end: usize,
    pub(crate) done_reading: bool,
    pub(crate) done_writing: bool,
    data: [u8; N],
}

impl<const N: usize> std::fmt::Debug for Queue<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("done_reading", &self.done_reading)
            .field("done_writing", &self.done_writing)
            .field("data", &self.data)
            .field("messages", &self.messages())
            .finish()
    }
}

impl<const N: usize> Queue<N> {
    pub(crate) fn from_ptr(ptr: *mut std::ffi::c_void) -> &'static mut Self {
        let ptr = ptr as *mut Queue<N>;
        unsafe { ptr.as_mut() }.unwrap()
    }

    pub(crate) fn push(&mut self, message: &[u8]) {
        // write length
        self.data[self.end] = message.len() as u8;
        self.end += 1;

        // write content
        self.data[self.end..self.end + message.len()].clone_from_slice(message);
        self.end += message.len();
    }

    pub(crate) fn can_push(&mut self, message: &[u8]) -> bool {
        let left = N - self.end;
        left >= message.len() + 1
    }

    fn message_at(&self, at: usize) -> Option<Vec<u8>> {
        let length = *self.data.get(at)?;
        if length == 0 {
            return None;
        }
        Some(self.data[at + 1..at + (length as usize) + 1].to_vec())
    }

    pub(crate) fn messages(&self) -> Vec<String> {
        let mut messages = vec![];
        let mut i = 0;
        while i < N {
            if let Some(message) = self.message_at(i) {
                i += message.len() + 1;
                messages.push(String::from_utf8(message).unwrap());
            } else {
                break;
            }
        }
        messages
    }

    pub(crate) fn can_pop(&self) -> bool {
        match self.data.get(self.start) {
            None | Some(0) => false,
            _ => true,
        }
    }

    pub(crate) fn pop(&mut self) -> Option<Vec<u8>> {
        // println!("[Reader] queue = {:?}", self);
        if let Some(message) = self.message_at(self.start) {
            self.start += message.len() + 1;
            Some(message)
        } else {
            if self.done_writing {
                self.done_reading = true;
            }

            return None;
        }
    }
}
