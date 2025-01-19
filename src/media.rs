use futures_core::stream::Stream;
use axum::body::Bytes;
pub struct MediaStream{

}

impl MediaStream{
    pub fn get(file_path: &str) -> Self{
        MediaStream{

        }
    }
}
impl Stream for MediaStream{
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}