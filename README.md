# public-user-http-s3-media-loader
Handles the media loading requests of all publicily accessible user traffic, requests are made from the client through http and responded with a data stream containing files stored on the s3 backend. All traffic and interaction is isolated to public access, sensitive interactions/transactions are out of scope of this service.
