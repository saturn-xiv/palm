package v2

import (
	"bytes"
	"encoding/base32"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	"time"

	"google.golang.org/protobuf/proto"
)

func (p *EmailSendRequest_Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}

func (p *PolicyUserRequest) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func (p *PolicyRolesResponse_Item) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func NewPolicyRole(code string) (*PolicyRolesResponse_Item, error) {
	buf, err := base64.StdEncoding.DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyRolesResponse_Item
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func NewPolicyUser(code string) (*PolicyUserRequest, error) {
	buf, err := base64.StdEncoding.DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyUserRequest
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

type s3Bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days *uint32
	year            int
}

func (p *s3Bucket) code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base32.StdEncoding.EncodeToString(buf.Bytes()), nil
}

func (p *S3CreateBucketRequest) BucketName(namespace string) (string, error) {
	bucket := s3Bucket{
		namespace:       namespace,
		name:            p.Name,
		public:          p.Public,
		expiration_days: p.ExpirationDays,
		year:            time.Now().Year(),
	}
	return bucket.code()
}
