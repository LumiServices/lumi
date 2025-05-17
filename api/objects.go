package api

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

func PutObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	bucketpath := filepath.Join("data", bucketname)
	info, err := os.Stat(bucketpath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	objectpath := filepath.Join(bucketpath, objectkey)
	data, err := io.ReadAll(c.Request.Body)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	defer c.Request.Body.Close()
	err = os.WriteFile(objectpath, data, 0644)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
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
	c.Data(http.StatusOK, contentType, data)
}
