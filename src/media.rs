use std::{ sync::Arc, task::Poll, usize};

use futures_core::stream::Stream;
use axum::body::Bytes;
use crate::s3loader::load_ref;

const CHUNK_SIZE: usize = 1024 * 256;

pub struct MediaStream{
    media: Arc<[u8]>, //currently a stagnent slice, but a vec or custom struct may be better for s3 file stream integration (so data can be streamed out by http as streamed in by s3)
    start: usize
}

impl MediaStream{
    pub fn get(file_path: &str, part: Option<&str>) -> Result<Self, ()>{
        match load_ref(file_path, part){
            Ok(media) => Ok(MediaStream{media, start: 0}),
            Err(_) => todo!(), // only should throw an error if resource doesnt exist or an internal recovery was not possible
        }
    }
    fn next(&mut self, chunk_size: usize) -> &[u8]{
        let data = &self.media[self.start..self.start+chunk_size];
        self.start += data.len();
        return data
    }
}
impl Stream for MediaStream{
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let byte_chunk = self.get_mut().next(CHUNK_SIZE);
        match byte_chunk.len() {
            0 => Poll::Ready(None),
            _ => {
                Poll::Ready(Some(Ok(Bytes::copy_from_slice(byte_chunk))))
            },
        }
    }
}
