mod s3loader;
mod datamgr;
mod datastream;
use axum::{ //Http library
    body::Body, extract::{Path, Request}, response::{IntoResponse, Response}, routing::get, Router
};
use datastream::GlobalDataStream;

//Important Rust Notes:
//This is async meaning many fuction return Futures, if you see .await that is when the function starts execution and begins blocking the main thread.
//



#[tokio::main]
async fn main() {
    match datamgr::_mem_map_setup(){
      Ok(_) =>{},
      Err(err) => {
        todo!()
      }
    }
    //Route requests to the di  fferent handlers
    //The symbol ":" in the path denotes a variable extracted from the url, matching the variable of the handler function
    //  for example "./listing/:listing_id/media/image/:image_id" extracts the handler image variables from ":listind_id" value and ":image_id" value to their respective image(Path((listing_id, image_id)) function paramaters
    let app = Router::new()
    .route("./listing/:listing_id/image/:image_id", get(image))
    .route("./listing/:listing_id/video/:video_id", get(playlist))
    .route("./listing/:listing_id/video/:video_id/:video_version/:part_id", get(video))
    .route("./listing/:listing_id/thumbnail/:thumbnail_id", get(thumnail));
    
    //Sets up the ip and port to listen on for requests
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    match axum::serve(listener, app).await { //server the app, handling 
        Ok(_) => {}, //OK() signifies proper execution shutdown without errors
        Err(err) =>{
          //Error logic




        },
    };
}

 async fn image(Path((listing_id, image_id)): Path<(String, String)>, req: Request<Body>) -> impl IntoResponse{ //Image Handler
    //Rebuilds the path formatted to the S3 bucket structure
   let file_path = format!("./{}/image/{}", listing_id, image_id);
    //Creates a new response generated from my custom stream object
    //The MediaStream::get function returns a Stream object for the related file path
   match GlobalDataStream::new(&file_path, None).await{
        Ok(stream) =>  Response::new(Body::from_stream(stream)),
        Err(_) => todo!(),
   }
 } //the video and thumnail functions work functionally the same
 
 async fn video(Path((listing_id, video_id,video_version, part)): Path<(String, String, String, String)>, req: Request<Body>) -> impl IntoResponse{ //Video Handler
   let file_path = format!("./{}/video/{}/{}", listing_id, video_id, video_version);
   match GlobalDataStream::new(&file_path, Some(&part)).await{
      Ok(stream) =>  Response::new(Body::from_stream(stream)),
      Err(_) => todo!(),
   }
 }
 async fn playlist(Path((listing_id, video_id)): Path<(String, String)>, req: Request<Body>) -> impl IntoResponse{ //Video Handler
  let file_path = format!("./{}/video/{}", listing_id, video_id);
  match GlobalDataStream::new(&file_path, None).await{
     Ok(stream) =>  Response::new(Body::from_stream(stream)),
     Err(_) => todo!(),
  }
}
 async fn thumnail(Path((listing_id, thumnail_id)): Path<(String, String)>, req: Request<Body>) -> impl IntoResponse{ //Thumbnail Handler
   let file_path = format!("./{}/thumbanil/{}", listing_id, thumnail_id);
   match GlobalDataStream::new(&file_path,None).await{
      Ok(stream) =>  Response::new(Body::from_stream(stream)),
      Err(_) => todo!(),
   }
 }