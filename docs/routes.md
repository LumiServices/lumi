# S3 Compatible Routes

| HTTP Method | Path                     | Description        |
|-------------|--------------------------|--------------------|
| GET         | /{bucket}?key            | GetBucketLocation  |
| DELETE      | /:bucket/*key            | DeleteObject       |
| GET         | /:{bucket}               | ListObjectsV2      |
| PUT         | /:{bucket}/*key          | PutObject          |
| PUT         | /:{bucket}               | CreateBucketCommand |