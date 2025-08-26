package api

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

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
	c.Header("ETag", `"`+objectkey+`"`)
	c.Header("Content-Length", string(rune(size)))
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

	contentType := http.DetectContentType(data)

	var contentDisposition string
	if strings.HasPrefix(contentType, "image/") ||
		strings.HasPrefix(contentType, "text/") ||
		contentType == "application/pdf" ||
		strings.HasPrefix(contentType, "video/") ||
		strings.HasPrefix(contentType, "audio/") {
		contentDisposition = "inline"
	} else {
		contentDisposition = "attachment; filename=\"" + filepath.Base(objectkey) + "\""
	}

	c.Header("Content-Disposition", contentDisposition)
	c.Data(http.StatusOK, contentType, data)
}
