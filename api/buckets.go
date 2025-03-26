package api

import (
	"encoding/xml"
	"net/http"
	"strconv"
	"strings"

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
	bucket := c.Param("bucket")
	if bucket == "" {
		c.XML(http.StatusBadRequest, gin.H{"error": "Bucket name is required"})
		return
	}
	prefix := c.DefaultQuery("prefix", "")
	maxKeysStr := c.DefaultQuery("max-keys", "1000")
	maxKeys, err := strconv.Atoi(maxKeysStr)
	if err != nil || maxKeys < 1 {
		c.XML(http.StatusBadRequest, gin.H{"error": "Invalid max-keys value"})
		return
	}

	var sampleObjects = []Object{
		{Key: "file1.txt", LastModified: "2024-03-22T10:00:00Z", Size: 1234, ETag: `"fba9dede5f27731c9771645a39863328"`, StorageClass: "STANDARD"},
		{Key: "file2.txt", LastModified: "2024-03-21T09:00:00Z", Size: 5678, ETag: `"fba9dede5f27731c9771645a39863328"`, StorageClass: "STANDARD"},
		{Key: "folder/file3.txt", LastModified: "2024-03-20T08:00:00Z", Size: 91011, ETag: `"fba9dede5f27731c9771645a39863328"`, StorageClass: "STANDARD"},
	}
	var filteredObjects []Object
	for _, obj := range sampleObjects {
		if prefix == "" || strings.HasPrefix(obj.Key, prefix) {
			filteredObjects = append(filteredObjects, obj)
		}
	}
	if len(filteredObjects) > maxKeys {
		filteredObjects = filteredObjects[:maxKeys]
	}

	response := Bucket{
		Xmlns:       "http://s3.amazonaws.com/doc/2006-03-01/",
		Name:        bucket,
		Prefix:      prefix,
		KeyCount:    len(filteredObjects),
		MaxKeys:     maxKeys,
		IsTruncated: len(filteredObjects) > maxKeys,
		Contents:    filteredObjects,
	}
	c.XML(http.StatusOK, response)
}
