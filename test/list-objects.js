import { S3Client, ListObjectsCommand } from "@aws-sdk/client-s3";

const s3Client = new S3Client({
  endpoint: "http://localhost:80", 
  region: "us-east-1",
  forcePathStyle: true, 
  credentials: {
    accessKeyId: "", 
    secretAccessKey: ""
  }
});

async function testS3API() {
  const command = new ListObjectsCommand({
    Bucket: "test", // /bucket name
    Prefix: "folder/", // folder
    MaxKeys: 2 //no clue what this does
  });

  try {
    console.log("Sending request to S3-compatible API...");
    const response = await s3Client.send(command);
    console.log("✅ S3 API Response:", response);
  } catch (error) {
    console.error("Error:", error);
    if (error.$response && error.$response.body) {
      console.log("Raw response:", error.$response.body.toString());
    }
  }
}
testS3API();
