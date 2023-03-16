use crate::session::session_id::SessionID;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use chrono::Utc;
use dashmap::DashMap;
use simple_error::SimpleResult;

//The MessageStore interface provides methods to record and retrieve messages for resend purposes
#[async_trait]
pub trait MessageStore: Send + Sync {
    fn next_sender_msg_seq_num(&self) -> isize;
    fn next_target_msg_seq_num(&self) -> isize;
    async fn incr_next_sender_msg_seq_num(&mut self) -> SimpleResult<()>;
    async fn incr_next_target_msg_seq_num(&mut self) -> SimpleResult<()>;
    async fn set_next_sender_msg_seq_num(&mut self, next_seq_num: isize) -> SimpleResult<()>;
    async fn set_next_target_msg_seq_num(&mut self, next_seq_num: isize) -> SimpleResult<()>;
    fn creation_time(&self) -> NaiveDateTime;
    async fn save_message(&mut self, seq_num: isize, msg: Vec<u8>) -> SimpleResult<()>;
    async fn save_message_and_incr_next_sender_msg_seq_num(
        &mut self,
        seq_num: isize,
        msg: Vec<u8>,
    ) -> SimpleResult<()>;
    async fn get_messages(
        &self,
        begin_seq_num: isize,
        end_seq_num: isize,
    ) -> SimpleResult<Vec<Vec<u8>>>;
    async fn refresh(&self) -> SimpleResult<()>;
    async fn reset(&mut self) -> SimpleResult<()>;
    async fn close(&self) -> SimpleResult<()>;
}

//The MessageStoreFactory interface is used by session to create a session specific message store
#[async_trait]
pub trait MessageStoreFactory {
    async fn create(&self, session_id: SessionID) -> SimpleResult<Box<dyn MessageStore>>;
}

#[derive(Default)]
pub struct MemoryStore {
    pub sender_msg_seq_num: isize,
    pub target_msg_seq_num: isize,
    pub creation_time: NaiveDateTime,
    pub message_map: DashMap<isize, Vec<u8>>,
}

#[async_trait]
impl MessageStore for MemoryStore {
    fn next_sender_msg_seq_num(&self) -> isize {
        self.sender_msg_seq_num + 1
    }

    fn next_target_msg_seq_num(&self) -> isize {
        self.target_msg_seq_num + 1
    }

    async fn incr_next_sender_msg_seq_num(&mut self) -> SimpleResult<()> {
        self.sender_msg_seq_num += 1;
        Ok(())
    }

    async fn incr_next_target_msg_seq_num(&mut self) -> SimpleResult<()> {
        self.target_msg_seq_num += 1;
        Ok(())
    }

    async fn set_next_sender_msg_seq_num(&mut self, next_seq_num: isize) -> SimpleResult<()> {
        self.sender_msg_seq_num = next_seq_num - 1;
        Ok(())
    }

    async fn set_next_target_msg_seq_num(&mut self, next_seq_num: isize) -> SimpleResult<()> {
        self.target_msg_seq_num = next_seq_num - 1;
        Ok(())
    }

    fn creation_time(&self) -> NaiveDateTime {
        self.creation_time
    }

    async fn save_message(&mut self, seq_num: isize, msg: Vec<u8>) -> SimpleResult<()> {
        self.message_map.insert(seq_num, msg);
        Ok(())
    }

    async fn save_message_and_incr_next_sender_msg_seq_num(
        &mut self,
        seq_num: isize,
        msg: Vec<u8>,
    ) -> SimpleResult<()> {
        self.save_message(seq_num, msg).await?;
        Ok(self.incr_next_sender_msg_seq_num().await?)
    }

    async fn get_messages(
        &self,
        begin_seq_num: isize,
        end_seq_num: isize,
    ) -> SimpleResult<Vec<Vec<u8>>> {
        let mut msgs: Vec<Vec<u8>> = vec![];
        let mut seq_num = begin_seq_num;
        while seq_num <= end_seq_num {
            if self.message_map.contains_key(&seq_num) {
                msgs.push(self.message_map.get(&seq_num).unwrap().to_vec());
            }
            seq_num += 1;
        }

        Ok(msgs)
    }

    async fn refresh(&self) -> SimpleResult<()> {
        // nop, nothing to refresh
        Ok(())
    }

    async fn reset(&mut self) -> SimpleResult<()> {
        self.sender_msg_seq_num = 0;
        self.target_msg_seq_num = 0;
        self.creation_time = Utc::now().naive_utc();
        self.message_map.clear();
        Ok(())
    }

    async fn close(&self) -> SimpleResult<()> {
        // nop, nothing to close
        Ok(())
    }
}

pub struct MemoryStoreFactory;

#[async_trait]
impl MessageStoreFactory for MemoryStoreFactory {
    async fn create(&self, _session_id: SessionID) -> SimpleResult<Box<dyn MessageStore>> {
        let mut m = MemoryStore::default();
        let result = m.reset().await;
        if result.is_err() {
            return Err(simple_error!("reset: {}", result.unwrap_err()));
        }
        Ok(Box::new(m))
    }
}

impl MemoryStoreFactory {
    // new returns a MessageStoreFactory instance that created in-memory MessageStores
    pub fn new() -> Box<dyn MessageStoreFactory> {
        Box::new(MemoryStoreFactory {})
    }
}