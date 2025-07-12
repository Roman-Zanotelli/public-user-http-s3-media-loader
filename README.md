# public-user-http-s3-media-loader (Deprecated/Archived)
Handles the media loading requests of all publicily accessible user traffic, requests are made from the client through http and responded with a data stream containing files stored on the s3 backend. All traffic and interaction is isolated to public access, sensitive interactions/transactions are out of scope of this service.

This project was the first iteration of many while attempting to develop cost effective hybrid infastructure using private and public S3-compatible infastructure

For my purposes the scalability of hybrid/public clouds barely outweight the benifits of a completely private minio infastrucutre used for S3-compatible storage on local disks

This repo serves as a milestone/archive of my journey as a cloud architect and developer and does NOT represent my current systems infastructure, but rather the inital phase of my journey.

Otherwise this system has been heavily reiterated resembling nothing similar in its current state.

[Most Recent (Public) Version](https://github.com/Roman-Zanotelli/media-cache-envoy-ext-authz) **[Also Deprecated]**



# High-Level System Interaction Overview
![public-user-http-s3-media-loader-diagram excalidraw](https://github.com/user-attachments/assets/97f95c10-2a48-413b-9448-dbfa7da68df5)


**Note:
Cache invalidation and other interactions !NOT DIRECTLY RELATED TO PUBLIC MEDIA RETRIEVAL! are handled elsewhere by another service
