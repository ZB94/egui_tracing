use crate::layer::LogRecord;
use crossbeam_channel::Receiver;

pub struct EguiLog {
    pub(crate) max_size: usize,
    pub(crate) receiver: Receiver<LogRecord>,
    pub(crate) log_list: Vec<LogRecord>,
}

impl EguiLog {
    pub fn ui(&mut self) {
        while let Ok(record) = self.receiver.try_recv() {
            self.log_list.push(record);
        }
        if self.log_list.len() > self.max_size {
            self.log_list.drain(0..self.log_list.len() - self.max_size);
        }

        println!("{:#?}", &self.log_list);
    }
}
