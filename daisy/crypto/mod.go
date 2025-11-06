package crypto

import (
	"errors"
	"log/slog"
	"os"

	"github.com/tink-crypto/tink-go/v2/insecurecleartextkeyset"
	"github.com/tink-crypto/tink-go/v2/keyset"
	"github.com/tink-crypto/tink-go/v2/proto/tink_go_proto"
)

func create_keyset_file(name string, template *tink_go_proto.KeyTemplate) error {
	slog.Info("generate keyset to", "file", name)
	handle, err := keyset.NewHandle(template)
	if err != nil {
		return err
	}

	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()
	if insecurecleartextkeyset.Write(handle, keyset.NewBinaryWriter(file)); err != nil {
		return err
	}
	return nil
}

func load_keyset_file(name string, template *tink_go_proto.KeyTemplate) (*keyset.Handle, error) {
	if _, err := os.Stat(name); errors.Is(err, os.ErrNotExist) {
		if err = create_keyset_file(name, template); err != nil {
			return nil, err
		}
	}
	slog.Debug("load keyset from", "file", name)
	file, err := os.Open(name)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	return insecurecleartextkeyset.Read(keyset.NewBinaryReader(file))
}
