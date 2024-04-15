package crypto

import (
	"crypto/rand"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path"

	"github.com/tink-crypto/tink-go/v2/aead"
	"github.com/tink-crypto/tink-go/v2/insecurecleartextkeyset"
	"github.com/tink-crypto/tink-go/v2/jwt"
	"github.com/tink-crypto/tink-go/v2/keyset"
	"github.com/tink-crypto/tink-go/v2/mac"
)

func RandomBytes(len int) []byte {
	buf := make([]byte, len)
	rand.Read(buf)
	return buf
}

func Open(root string) (*Aes, *HMac, *Jwt, error) {
	{
		stat, err := os.Stat(root)
		if err == nil {
			if !stat.IsDir() {
				return nil, nil, nil, fmt.Errorf("%s is not a folder", root)
			}
		} else {
			if os.IsNotExist(err) {
				if err = os.MkdirAll(root, 0700); err != nil {
					return nil, nil, nil, err
				}
			}
		}
	}
	master_key_file_name := path.Join(root, "master.bin")
	jwt_key_file_name := path.Join(root, "jwt.bin")
	hmac_key_file_name := path.Join(root, "hmac.bin")
	aes_key_file_name := path.Join(root, "aes.bin")
	if _, err := os.Stat(master_key_file_name); errors.Is(err, os.ErrNotExist) {
		master_key, err := keyset.NewHandle(aead.AES256GCMKeyTemplate())
		if err != nil {
			return nil, nil, nil, err
		}
		secret, err := aead.New(master_key)
		if err != nil {
			return nil, nil, nil, err
		}

		slog.Warn(fmt.Sprintf("generate a new master key file %s", master_key_file_name))
		{
			file, err := os.Create(master_key_file_name)
			if err != nil {
				return nil, nil, nil, err
			}
			defer file.Close()

			writer := keyset.NewBinaryWriter(file)
			if err = insecurecleartextkeyset.Write(master_key, writer); err != nil {
				return nil, nil, nil, err
			}
			if err = os.Chmod(master_key_file_name, 0400); err != nil {
				return nil, nil, nil, err
			}
		}
		slog.Warn(fmt.Sprintf("generate a new aes key file %s", aes_key_file_name))
		if err = Dump(aes_key_file_name, secret, aead.AES256GCMKeyTemplate()); err != nil {
			return nil, nil, nil, err
		}
		slog.Warn(fmt.Sprintf("generate a new hmac key file %s", hmac_key_file_name))
		if err = Dump(hmac_key_file_name, secret, mac.HMACSHA512Tag512KeyTemplate()); err != nil {
			return nil, nil, nil, err
		}

		slog.Warn(fmt.Sprintf("generate a new jwt key file %s", jwt_key_file_name))
		if err = Dump(jwt_key_file_name, secret, jwt.HS512Template()); err != nil {
			return nil, nil, nil, err
		}

	}
	slog.Debug(fmt.Sprintf("load master key from %s", master_key_file_name))
	master_key_file, err := os.Open(master_key_file_name)
	if err != nil {
		return nil, nil, nil, err
	}
	defer master_key_file.Close()

	master_key_file_reader := keyset.NewBinaryReader(master_key_file)
	master_key, err := insecurecleartextkeyset.Read(master_key_file_reader)
	if err != nil {
		return nil, nil, nil, err
	}
	secret, err := aead.New(master_key)
	if err != nil {
		return nil, nil, nil, err
	}

	slog.Debug(fmt.Sprintf("load aes key from %s", aes_key_file_name))
	aes, err := NewAes(aes_key_file_name, secret)
	if err != nil {
		return nil, nil, nil, err
	}

	slog.Debug(fmt.Sprintf("load hmac key from %s", hmac_key_file_name))
	hmac, err := NewHMac(hmac_key_file_name, secret)
	if err != nil {
		return nil, nil, nil, err
	}

	slog.Debug(fmt.Sprintf("load jwt key from %s", jwt_key_file_name))
	jwt, err := NewJwt(jwt_key_file_name, secret)
	if err != nil {
		return nil, nil, nil, err
	}

	return aes, hmac, jwt, nil

}
