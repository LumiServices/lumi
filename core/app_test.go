package core

import (
	"fmt"
	"testing"
)

func TestGenerateACSKey(t *testing.T) {
	length := 10
	key, err := GenerateACSKey(length, nil)
	if err != nil {
		t.Fatalf("Failed to generate ACS key: %v", err)
	}
	fmt.Print(key)
}

func TestGenerateSKey(t *testing.T) {
	length := 10
	key, err := GenerateSKey(length, nil)
	if err != nil {
		t.Fatalf("Failed to generate S key: %v", err)
	}
	fmt.Print(key)
}
