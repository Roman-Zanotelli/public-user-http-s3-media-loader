use std::{collections::VecDeque, sync::Arc, task::Poll};

use axum::body::Bytes;
use futures_core::Stream;

use crate::datamgr::{DataEntry, GlobalDataRef};

pub struct GlobalDataStream{
    owned_data: VecDeque<Arc<DataEntry>>,
    _data_ref: Arc<GlobalDataRef>
}
impl GlobalDataStream{
    pub async fn new(path: &str, part: Option<&str>) -> Result<Self, ()>{
        Self::_new(GlobalDataRef::new(path, part)?)
    }
    fn _new(_data_ref: Arc<GlobalDataRef>) -> Result<Self, ()>{
       Ok(GlobalDataStream{
        owned_data: todo!(),
        _data_ref
       })
    }
}
impl Stream for GlobalDataStream{
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        match self.get_mut().owned_data.pop_front(){
            Some(entry) => {
                todo!()
            },
            None => Poll::Ready(None),
        }
    }
}
