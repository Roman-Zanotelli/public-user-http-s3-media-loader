# public-user-http-s3-media-loader
Handles the media loading requests of all publicily accessible user traffic, requests are made from the client through http and responded with a data stream containing files stored on the s3 backend. All traffic and interaction is isolated to public access, sensitive interactions/transactions are out of scope of this service.




# Diagram
![public-user-http-s3-media-loader-diagram excalidraw](https://github.com/user-attachments/assets/97f95c10-2a48-413b-9448-dbfa7da68df5)


**Note:
Cache invalidation and other interactions NOT DIRECTLY RELATED TO PUBLIC MEDIA RETRIEVAL are handled elsewhere
