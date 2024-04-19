package v2

import (
	"bytes"
	"encoding/base32"
	"encoding/gob"
	"strings"
	"time"

	"github.com/go-playground/validator/v10"
)

var (
	gl_validate *validator.Validate = validator.New(validator.WithRequiredStructEnabled())

	gl_bucket_name_validator_tag = "required,min=3,max=63"
)

type s3Bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days *uint32
	year            int
}

func IsS3BucketPublic(bucket string) (bool, error) {
	bin, err := base32.StdEncoding.WithPadding(base32.NoPadding).DecodeString(strings.ToUpper(bucket))
	if err != nil {
		return false, err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	var it s3Bucket
	if err := dec.Decode(&it); err != nil {
		return false, err
	}
	return it.public, nil
}

func (p *s3Bucket) code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	code := strings.ToLower(base32.StdEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes()))
	if err := gl_validate.Var(code, gl_bucket_name_validator_tag); err != nil {
		return "", err
	}
	return code, nil
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
