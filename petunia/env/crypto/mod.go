package crypto

import (
	"crypto/rand"
)

func RandomBytes(len int) []byte {
	buf := make([]byte, len)
	rand.Read(buf)
	return buf
}
