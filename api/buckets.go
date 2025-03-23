package api

import (
	"encoding/xml"
	"net/http"

	"github.com/gin-gonic/gin"
)

//ALL EXAMPLE DATA FOR NOW

type Bucket struct {
	XMLName     xml.Name `xml:"ListBucketResult"`
	Xmlns       string   `xml:"Xmlns"`
	Name        string   `xml:"Name"`
	Prefix      string   `xml:"Prefix"`
	KeyCount    int      `xml:"KeyCount"`
	MaxKeys     int      `xml:"MaxKeys"`
	IsTruncated bool     `xml:"IsTruncated"`
	Content     []Object `xml:"Contents"`
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
	bucket := c.Param("bucket")
	if bucket == "" {
		c.XML(http.StatusBadRequest, gin.H{"error": "Bucket name is required"})
		return
	}
	prefix := c.DefaultQuery("prefix", "")
	response := Bucket{
		Xmlns:  "http://s3.amazonaws.com/doc/2006-03-01/",
		Name:   bucket,
		Prefix: prefix,
	}
	c.XML(http.StatusOK, response)
}
