package v2

import (
	"bytes"
	"encoding/base32"
	"encoding/gob"
	"fmt"
	"strings"
)

func (p *EmailSendRequest_Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}

type bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days int32
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func (p *S3CreateBucketRequest) Code(namespace string) (string, error) {
	it := bucket{
		namespace:       namespace,
		name:            p.Name,
		public:          p.Public,
		expiration_days: p.ExpirationDays,
	}
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(&it); err != nil {
		return "", err
	}
	code := base32.HexEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes())
	return strings.ToLower(code), nil
}

func NewS3CreateBucketRequestFromCode(code string) (string, *S3CreateBucketRequest, error) {
	bin, err := base32.HexEncoding.WithPadding(base32.NoPadding).DecodeString(strings.ToUpper(code))
	if err != nil {
		return "", nil, err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	var it bucket
	if err := dec.Decode(&it); err != nil {
		return "", nil, err
	}
	return it.namespace, &S3CreateBucketRequest{Name: it.name, Public: it.public, ExpirationDays: it.expiration_days}, nil
}
