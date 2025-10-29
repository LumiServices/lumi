# S3 Compatible Routes

| HTTP Method | Path              | Handler                    | S3 Command           |
|-------------|-------------------|----------------------------|----------------------|
| GET         | /                 | `list_buckets_handler`     | ListBuckets          |
| GET         | /{bucket}/        | `list_objects_v2_handler`  | ListObjectsV2        |
| GET         | /{bucket}/{*key}  | `get_object_handler`       | GetObject / HeadObject |
| PUT         | /{bucket}/        | `create_bucket_handler`    | CreateBucket         |
| PUT         | /{bucket}/{*key}  | `put_object_handler`       | PutObject / CopyObject |
| DELETE      | /{bucket}/        | `delete_bucket_handler`    | DeleteBucket         |
| DELETE      | /{bucket}/{*key}  | `delete_object_handler`    | DeleteObject         |
| GET         | /health           | `health`                   | Health Check         |