package v2

import (
	"encoding/base64"
	"path/filepath"
	"time"

	"github.com/google/uuid"
	"google.golang.org/protobuf/proto"
)

func (p *Session) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(buf), nil
}

func NewSession(s string) (*Session, error) {
	tmp, err := base64.URLEncoding.DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Session
	if err := proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Page) Offset() int64 {
	return (p.Index - 1) * p.Size
}

func NewPagination(page *Page, total int64) *Pagination {
	size := page.Size
	if size < 20 {
		size = 20
	}
	if size > 1000 {
		size = 1000
	}
	index := page.Index
	if index < 1 {
		index = 1
	}
	pages := total / size
	if total%size > 0 {
		pages = pages + 1
	}
	if index*size > total {
		index = pages
	}
	return &Pagination{
		Current:     &Page{Index: index, Size: size},
		Total:       total,
		Pages:       pages,
		HasPrevious: index > 1,
		HasNext:     index < pages,
	}
}

func (p *UserCreateAttachmentRequest) Bucket() string {
	return "attachments-" + time.Now().Format(time.DateOnly)
}
func (p *UserCreateAttachmentRequest) Object() string {
	return uuid.New().String() + filepath.Ext(p.Title)
}
