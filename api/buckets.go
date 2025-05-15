package api

import (
	"encoding/xml"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

// https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html
type Bucket struct {
	XMLName     xml.Name `xml:"ListBucketResult"`
	Xmlns       string   `xml:"Xmlns"`
	Name        string   `xml:"Name"`
	IsTruncated bool     `xml:"IsTruncated"`
	Contents    []Object `xml:"Contents"`
}

type Object struct {
	XMLName      xml.Name `xml:"Contents"`
	Key          string   `xml:"Key"`
	LastModified string   `xml:"LastModified"`
	Size         int64    `xml:"Size"`
	ETag         string   `xml:"ETag"`
	StorageClass string   `xml:"StorageClass"`
}

func ListObjectsV2Handler(c *gin.Context) {
	//check if the bucket path in /data exists
	prefix := c.DefaultQuery("prefix", "")
	bucketdir := filepath.Join("data", c.Param("bucket"))
	info, err := os.Stat(bucketdir)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, c.Param("bucket"))
	}
	//contents of bucket
	entries, err := os.ReadDir(bucketdir)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, "Could not read bucket directory")
		return
	}
	var contents []Object
	for _, entry := range entries {
		name := entry.Name()
		if strings.HasPrefix(name, prefix) {
			info, _ := entry.Info()
			contents = append(contents, Object{
				Key:          name,
				LastModified: info.ModTime().Format(time.RFC3339),
				Size:         info.Size(),
			})
		}
	}

	response := Bucket{
		Xmlns:       "http://s3.amazonaws.com/doc/2006-03-01/",
		Name:        c.Param("bucket"),
		IsTruncated: false,
		Contents:    contents,
	}
	c.XML(http.StatusOK, response)
}

func CreateBucketCommand(c *gin.Context) {
	//get bucket name from url
	bucketname := c.Param("bucket")
	if _, err := os.Stat(bucketname); err == nil {
		WriteErrorResponse(c, s3.ErrBucketAlreadyExists, "Bucket already exists")
		return
	} else if !os.IsNotExist(err) {
		WriteErrorResponse(c, s3.ErrInternalError, "Failed to check bucket status")
		return
	}
	//make folder in /data
	if err := os.MkdirAll(filepath.Join("data", bucketname), 0755); err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, "Failed to create bucket")
		return
	}
	c.Header("Location", c.Param("bucket"))
	c.String(200, "Location: %s", c.Param("bucket"))
}
