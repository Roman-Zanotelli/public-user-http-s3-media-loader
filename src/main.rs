mod media;
use media::MediaStream; //Custom Stream
use axum::{ //Http library
    body::Body, extract::Path, response::{IntoResponse, Response}, routing::get, Router
};

//Important Rust Notes:
//This is async meaning many fuction return Futures, if you see .await that is when the function starts execution and begins blocking the main thread.
//



#[tokio::main]
async fn main() {
    //Route requests to the different handlers
    //The symbol ":" in the path denotes a variable extracted from the url, matching the variable of the handler function
    //  for example "./listing/:listing_id/media/image/:image_id" extracts the handler image variables from ":listind_id" value and ":image_id" value to their respective image(Path((listing_id, image_id)) function paramaters
    let app = Router::new().route("./listing/:listing_id/media/image/:image_id", get(image)).route("./listing/:listing_id/media/video/:video_id", get(video)).route("./listing/:listing_id/media/thumbnail/:thumbnail_id", get(thumnail));
    
    //Sets up the ip and port to listen on for requests
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    //Start serving the app from the listening port
    // The unwrap() function causes the thread to panic if an error is thrown,
    //  this is often discouraged over explicit error handling similar to the psuedo code below.
    axum::serve(listener, app).await.unwrap();
    
    
    // match axum::serve(listener, app).await {
    //     Ok(_) => todo!(),
    //     Err(err) => todo!(),
    // };
}

 async fn image(Path((listing_id, image_id)): Path<(String, String)>) -> impl IntoResponse{ //Image Handler
    //Rebuilds the path formatted to the S3 bucket structure
    let file_path = format!("./listing/{}/image/{}", listing_id, image_id);
    //Creates a new response generated from my custom stream object
    //The MediaStream::get function returns a Stream object for the related file path
    Response::new(Body::from_stream(MediaStream::get(&file_path)).into_data_stream())
 } //the video and thumnail functions work functionally the same
 
 async fn video(Path((listing_id, image_id)): Path<(String, String)>) -> impl IntoResponse{ //Video Handler
    let file_path = format!("./listing/{}/video/{}", listing_id, image_id);
    Response::new(Body::from_stream(MediaStream::get(&file_path)).into_data_stream())
 }
 async fn thumnail(Path((listing_id, image_id)): Path<(String, String)>) -> impl IntoResponse{ //Thumbnail Handler
    let file_path = format!("./listing/{}/thumbanil/{}", listing_id, image_id);
    Response::new(Body::from_stream(MediaStream::get(&file_path)).into_data_stream())
 }