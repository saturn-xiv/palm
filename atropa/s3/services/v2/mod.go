package v2

import (
	"encoding/base32"
	"strings"

	"google.golang.org/protobuf/proto"
)

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func (p *CreateBucketRequest) Code(namespace string) (string, error) {
	it := Bucket{
		Namespace:      namespace,
		Name:           p.Name,
		Public:         p.Public,
		ExpirationDays: p.ExpirationDays,
	}
	buf, err := proto.Marshal(&it)
	if err != nil {
		return "", err
	}
	code := base32.HexEncoding.WithPadding(base32.NoPadding).EncodeToString(buf)
	return strings.ToLower(code), nil
}

func NewS3CreateBucketRequestFromCode(code string) (string, *CreateBucketRequest, error) {
	buf, err := base32.HexEncoding.WithPadding(base32.NoPadding).DecodeString(strings.ToUpper(code))
	if err != nil {
		return "", nil, err
	}

	var it Bucket
	if err := proto.Unmarshal(buf, &it); err != nil {
		return "", nil, err
	}
	return it.Namespace, &CreateBucketRequest{Name: it.Name, Public: it.Public, ExpirationDays: it.ExpirationDays}, nil
}
