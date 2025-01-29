package cometbls

import (
	"fmt"
	"testing"
	"time"
)

// LightHeader gerçekten tanımlı mı kontrol edelim
func TestLightHeaderExists(t *testing.T) {
	// Eğer LightHeader yoksa, hata verecektir
	header := LightHeader{
		ChainId:            "test-chain",
		Height:             12345,
		Time:               time.Now(),
		ValidatorsHash:     []byte("validators"),
		NextValidatorsHash: []byte("next_validators"),
		AppHash:            []byte("app_hash"),
	}

	fmt.Printf("LightHeader struct tanımlı ve içeriği: %+v\n", header)
}
