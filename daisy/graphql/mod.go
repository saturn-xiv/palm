package graphql

import (
	"context"
	"crypto/rand"
	_ "embed"
	"encoding/base64"
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/go-playground/validator/v10"
	graphql "github.com/graph-gophers/graphql-go"
	"github.com/graph-gophers/graphql-go/relay"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/queue"
)

var (
	ContentType   = "Content-Type"
	Authorization = "Authorization"
	Bearer        = "Bearer "
	XForwardedFor = "X-Forwarded-For"
	XRealIp       = "X-Real-IP"
)

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

//go:embed schema.gql
var gl_schema_txt string

type headerKey string

func Handler(db *gorm.DB, redis *cache.RedisClient, rabbitmq *queue.RabbitMQ, aead *crypto.Aead, hmac *crypto.Hmac, jwt *crypto.Jwt, google_oauth2 GoogleOauth2Config) (http.Handler, error) {
	schema, err := graphql.ParseSchema(gl_schema_txt, &Root{db, redis, rabbitmq, aead, hmac, jwt, google_oauth2})
	if err != nil {
		return nil, err
	}

	handler := &relay.Handler{Schema: schema}
	return http.HandlerFunc(func(wrt http.ResponseWriter, req *http.Request) {
		ctx := req.Context()
		{
			auth := req.Header.Get(Authorization)
			if strings.HasPrefix(auth, Bearer) {
				token := strings.TrimPrefix(auth, Bearer)
				ctx = context.WithValue(ctx, headerKey(Authorization), token)
			}
		}
		ctx = context.WithValue(ctx, headerKey(XForwardedFor), req.Header.Get(XForwardedFor))
		ctx = context.WithValue(ctx, headerKey(XRealIp), req.Header.Get(XRealIp))

		handler.ServeHTTP(wrt, req.WithContext(ctx))
	}), nil

}

type Page struct {
	Index uint
	Size  uint
}

func (p *Page) Offset() uint {
	return (p.Index - 1) * p.Size
}

type Pagination struct {
	current     *Page
	pages       uint
	total       uint
	hasPrevious bool
	hasNext     bool
}

func (p *Pagination) Size() int32 {
	return int32(p.current.Size)
}

func (p *Pagination) Index() int32 {
	return int32(p.current.Index)
}
func (p *Pagination) Pages() int32 {
	return int32(p.pages)
}
func (p *Pagination) Total() int32 {
	return int32(p.total)
}

func (p *Pagination) HasNext() bool {
	return p.hasNext
}

func (p *Pagination) HasPrevious() bool {
	return p.hasPrevious
}

func NewPagination(page *Page, total uint) *Pagination {
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
		current:     &Page{Index: index, Size: size},
		total:       total,
		pages:       pages,
		hasPrevious: index > 1,
		hasNext:     index < pages,
	}
}

type Ok struct {
}

func (p *Ok) CreatedAt() graphql.Time {
	return graphql.Time{Time: time.Now()}
}

func ToId(id uint) graphql.ID {
	return graphql.ID(strconv.FormatUint(uint64(id), 36))
}
func FromId(id graphql.ID) (uint, error) {
	it, err := strconv.ParseUint(string(id), 36, 32)
	if err != nil {
		return 0, err
	}
	return uint(it), nil
}

func ClientIp(ctx context.Context) string {
	it, ok := ctx.Value(headerKey(XRealIp)).(string)
	if ok {
		return it
	}
	return "n/a"
}

func random_alphanumeric(l int) (string, error) {
	buf := make([]byte, l)
	if _, err := rand.Read(buf); err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
