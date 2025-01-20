use std::task::Poll;

use futures_core::stream::Stream;
use axum::body::Bytes;
use lazy_static::lazy_static;



const chunk_size: usize = 1024 * 256;

pub struct MediaStream<'a>{
    media_ptr: &'a [u8]
}

impl MediaStream<'_>{
    pub fn get(file_path: &str) -> Self{
        MediaStream{
            media_ptr: byte_ptr_init(file_path)
        }
    }
}
impl Stream for MediaStream<'_>{
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let chunk = &self.media_ptr[..chunk_size];
        match chunk.len() {
            0 => {
                Poll::Ready(None)
            },
            _ => {
                self.media_ptr = &self.media_ptr[chunk_size..];
                Poll::Ready(Some(Ok(Bytes::copy_from_slice(chunk))))
            }
        }
        
    }
}

fn byte_ptr_init<'a>(file_path: &str) ->&'a [u8]{
    todo!()
}
