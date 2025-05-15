package api

import (
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

func PutObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	bucketPath := filepath.Join("data", bucketname)
	info, err := os.Stat(bucketPath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	objectPath := filepath.Join(bucketPath, objectkey)
	data, err := ioutil.ReadAll(c.Request.Body)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	err = ioutil.WriteFile(objectPath, data, 0644)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	c.Status(http.StatusOK)
}

func GetObject(c *gin.Context) {
	bucketname := c.Param("bucket")
	objectkey := c.Param("key")
	if len(objectkey) > 0 && objectkey[0] == '/' {
		objectkey = objectkey[1:]
	}
	bucketPath := filepath.Join("data", bucketname)
	info, err := os.Stat(bucketPath)
	if os.IsNotExist(err) || !info.IsDir() {
		WriteErrorResponse(c, s3.ErrNoSuchBucket, bucketname)
		return
	}
	objectPath := filepath.Join(bucketPath, objectkey)
	_, err = os.Stat(objectPath)
	if os.IsNotExist(err) {
		WriteErrorResponse(c, s3.ErrNoSuchKey, objectkey)
		return
	}
	data, err := ioutil.ReadFile(objectPath)
	if err != nil {
		WriteErrorResponse(c, s3.ErrInternalError, err.Error())
		return
	}
	contentType := http.DetectContentType(data)
	c.Data(http.StatusOK, contentType, data)
}
