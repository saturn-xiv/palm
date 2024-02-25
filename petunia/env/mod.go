package env

import (
	"crypto/rand"
)

func RandomBytes(len int) []byte {
	buf := make([]byte, len)
	rand.Read(buf)
	return buf
}

// func QueueName(i any) string {
// 	return reflect.TypeOf(i).Elem().Name()
// }
