package v2

import (
	"bytes"
	"encoding/base32"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	reflect "reflect"
	"strings"
	"time"

	"github.com/go-playground/validator/v10"
	"google.golang.org/protobuf/proto"
)

var (
	gl_validate *validator.Validate = validator.New(validator.WithRequiredStructEnabled())

	gl_bucket_name_validator_tag = "required,min=3,max=63"
)

func (p *PolicyRolesResponse_Item) Display() string {
	switch it := p.By.(type) {
	case *PolicyRolesResponse_Item_Administrator_:
		return "Administrator"
	case *PolicyRolesResponse_Item_Root_:
		return "Root"
	case *PolicyRolesResponse_Item_Member:
		return it.Member
	default:
		return "Unknown"
	}
}
func (p *EmailSendRequest_Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}

func (p *PolicyPermissionsResponse_Item_Resource) Display() string {
	if p.Id == nil {
		return fmt.Sprintf("%s://*", p.Type)
	}
	return fmt.Sprintf("%s://%d", p.Type, *p.Id)
}

func (p *PolicyUserRequest) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func (p *PolicyRolesResponse_Item) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func (p *PolicyPermissionsResponse_Item_Resource) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func NewPolicyResource(code string) (*PolicyPermissionsResponse_Item_Resource, error) {
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyPermissionsResponse_Item_Resource
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
func NewPolicyRole(code string) (*PolicyRolesResponse_Item, error) {
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
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
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
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

func (p *Pager) Offset(total int64) int {
	return (int(p.Page_(total)) - 1) * p.Size_()
}

func (p *Pager) Page_(total int64) int64 {
	size := int64(p.Size_())
	if total < size || p.Page < 1 {
		return 1
	}

	if p.Page*size > total {
		it := total / size
		if total%size == 0 {
			return it
		} else {
			return it + 1
		}

	}
	return p.Page
}

func (p *Pager) Size_() int {
	const MIN_SIZE = 1 << 2
	const MAX_SIZE = 1 << 12
	if p.Size < MIN_SIZE {
		return MIN_SIZE
	}
	if p.Size > MAX_SIZE {
		return MAX_SIZE
	}
	return int(p.Size)
}

func NewPagination(pager *Pager, total int64) *Pagination {
	size := int64(pager.Size_())
	page := int64(pager.Page_(total))

	return &Pagination{
		Size:        int64(size),
		Page:        page,
		Total:       total,
		HasNext:     (page*size < total),
		HasPrevious: (page > 1),
	}
}

func TaskQueueName(it proto.Message) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
