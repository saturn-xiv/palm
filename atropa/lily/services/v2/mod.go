package v2

import (
	"fmt"
	"time"

	"github.com/google/uuid"
)

func (p *Format) File(name string) (string, error) {
	switch *p {
	case Format_Pdf:
		return name + ".pdf", nil
	case Format_Word:
		return name + ".docx", nil
	default:
		return "", fmt.Errorf("unsupported ext %s", p.String())
	}
}

func (p *TexRequest) S3() (*File, error) {
	name, err := p.Format.File(uuid.New().String())
	if err != nil {
		return nil, err
	}
	return &File{
		Bucket: time.Now().Format("2006-01"),
		Object: name,
	}, nil
}
