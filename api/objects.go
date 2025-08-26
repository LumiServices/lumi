package api

import (
	"encoding/json"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

type ObjectMetadata struct {
	ContentType        string `json:"content_type"`
	ContentDisposition string `json:"content_disposition"`
	Size               int64  `json:"size"`
}

func PutObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	if len(objectkey) > 0 && objectkey[0] == '/' {
		objectkey = objectkey[1:]
	}
	bucketpath := filepath.Join("data", bucketname)
	info, err := os.Stat(bucketpath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	objectpath := filepath.Join(bucketpath, objectkey)
	if err := os.MkdirAll(filepath.Dir(objectpath), 0755); err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	file, err := os.Create(objectpath)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	defer file.Close()
	size, err := io.Copy(file, c.Request.Body)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		os.Remove(objectpath)
		return
	}
	defer c.Request.Body.Close()
	contentType := c.GetHeader("Content-Type")
	if contentType == "" {
		contentType = "application/octet-stream"
	}
	contentDisposition := c.GetHeader("Content-Disposition")
	meta := ObjectMetadata{
		ContentType:        contentType,
		ContentDisposition: contentDisposition,
		Size:               size,
	}
	metaBytes, err := json.Marshal(meta)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	err = os.WriteFile(objectpath+".meta", metaBytes, 0644)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	c.Header("ETag", `"`+objectkey+`"`)
	c.Header("Content-Length", strconv.FormatInt(size, 10))
	c.Status(http.StatusOK)
}

func DeleteObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	bucketpath := filepath.Join("data", bucketname)
	objectpath := filepath.Join(bucketpath, objectkey)
	info, err := os.Stat(bucketpath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	_, err = os.Stat(objectpath)
	if os.IsNotExist(err) {
		WriteErrorResponse(c, s3.ErrNoSuchKey, objectkey)
		return
	}
	err = os.Remove(objectpath)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	c.Status(http.StatusNoContent)
}

func DeleteObjects(c *gin.Context) {

}

func GetObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	if len(objectkey) > 0 && objectkey[0] == '/' {
		objectkey = objectkey[1:]
	}
	bucketpath := filepath.Join("data", bucketname)
	objectpath := filepath.Join(bucketpath, objectkey)
	info, err := os.Stat(bucketpath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	_, err = os.Stat(objectpath)
	if os.IsNotExist(err) {
		WriteErrorResponse(c, s3.ErrNoSuchKey, objectkey)
		return
	}
	data, err := os.ReadFile(objectpath)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	meta := ObjectMetadata{}
	metaPath := objectpath + ".meta"
	metaBytes, err := os.ReadFile(metaPath)
	if err == nil {
		json.Unmarshal(metaBytes, &meta)
	} else {
		meta.ContentType = http.DetectContentType(data)
	}
	contentType := meta.ContentType
	if contentType == "" {
		contentType = http.DetectContentType(data)
	}
	c.Header("Content-Type", contentType)
	if meta.ContentDisposition != "" {
		c.Header("Content-Disposition", meta.ContentDisposition)
	}

	c.Data(http.StatusOK, contentType, data)
}
