package s3

import (
	"bytes"
	"strings"
	"testing"
)

func TestContainsReservedChars(t *testing.T) {
	tests := []struct {
		input    string
		expected bool
	}{
		{"noSpecialChars", false},
		{"has=equals", true},
		{"has,comma", true},
		{"hasBoth=,", true},
	}

	for _, test := range tests {
		result := ContainsReservedChars(test.input)
		if result != test.expected {
			t.Errorf("ContainsReservedChars(%q) = %v; want %v", test.input, result, test.expected)
		}
	}
}

func TestGenerateAccessKey(t *testing.T) {
	t.Run("valid length", func(t *testing.T) {
		key, err := GenerateAccessKey(10, nil)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(key) != 10 {
			t.Errorf("expected length 10, got %d", len(key))
		}
	})

	t.Run("length below minimum", func(t *testing.T) {
		_, err := GenerateAccessKey(3, nil)
		if err == nil {
			t.Fatal("expected error for short access key, got none")
		}
	})

	t.Run("default length", func(t *testing.T) {
		key, err := GenerateAccessKey(0, nil)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(key) != MaxLeg_AccessKey {
			t.Errorf("expected length %d, got %d", MaxLeg_AccessKey, len(key))
		}
	})
}

func TestGenerateSecretKey(t *testing.T) {
	t.Run("valid length", func(t *testing.T) {
		key, err := GenerateSecretKey(20, nil)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(key) == 0 {
			t.Error("expected non-empty secret key")
		}
	})

	t.Run("length below minimum", func(t *testing.T) {
		_, err := GenerateSecretKey(3, nil)
		if err == nil {
			t.Fatal("expected error for short secret key, got none")
		}
	})

	t.Run("default length", func(t *testing.T) {
		key, err := GenerateSecretKey(0, nil)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(key) == 0 {
			t.Error("expected non-empty default secret key")
		}
	})
}

func TestGenerateAccessKey_CustomReader(t *testing.T) {
	reader := bytes.NewReader(bytes.Repeat([]byte{42}, 100))
	key, err := GenerateAccessKey(10, reader)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(key) != 10 {
		t.Errorf("expected length 10, got %d", len(key))
	}
}

func TestGenerateSecretKey_CustomReader(t *testing.T) {
	reader := bytes.NewReader(bytes.Repeat([]byte{42}, 100))
	key, err := GenerateSecretKey(20, reader)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(key) == 0 {
		t.Error("expected non-empty secret key")
	}
	if strings.Contains(key, "/") {
		t.Error("expected '/' to be replaced with '+', found '/'")
	}
}
