#[derive(Default)]
pub struct LogEntry {
    data: String,
    id: u64
}

const LOG_ENTRIES: usize = 256;

pub struct DeviceLogs {
    ring: [LogEntry; LOG_ENTRIES],
    write_id: u64
}


impl DeviceLogs {
    pub fn new() -> Self {
        let ring = {
            let mut data: [std::mem::MaybeUninit<LogEntry>; LOG_ENTRIES] = unsafe {
                std::mem::MaybeUninit::uninit().assume_init()
            };
        
            for elem in &mut data[..] {
                unsafe { std::ptr::write(elem.as_mut_ptr(), LogEntry::default()); }
            }
        
            unsafe { std::mem::transmute::<_, [LogEntry; LOG_ENTRIES]>(data) }
        };
        DeviceLogs{
            ring: ring,
            write_id: 1
        }
    }

    pub fn get_logs(&self) -> Vec<&str> {
        let l_entries = LOG_ENTRIES as u64;
        let mut res = Vec::with_capacity(LOG_ENTRIES);
        for i in self.write_id..(self.write_id + l_entries) {
            let entry = &self.ring[(i % l_entries) as usize];
            if entry.id > 0 {
                res.push(&entry.data[..]);
            }
        }
        res
    }

    pub fn add_log(&mut self, data: String) {
        let id = self.write_id;
        self.write_id += 1;
        let entry = &mut self.ring[(id % (LOG_ENTRIES as u64)) as usize];
        entry.id = id;
        entry.data = data;
    }
}

impl Default for DeviceLogs {
    fn default() -> Self {
        Self::new()
    }
}
