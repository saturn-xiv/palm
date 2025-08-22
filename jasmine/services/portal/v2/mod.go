package v2

import (
	context "context"
	"log/slog"
	"time"

	"github.com/redis/go-redis/v9"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

var (
	ErrorUserIsNotSignedIn        = status.Error(codes.PermissionDenied, "user is'not signed in")
	ErrorUserIsLocked             = status.Error(codes.Unavailable, "user is locked")
	ErrorUserMustHasAdministrator = status.Error(codes.PermissionDenied, "user must be an administrator")
	ErrorUserHasRoot              = status.Error(codes.PermissionDenied, "this is a root user")
	ErrorNotFound                 = status.Error(codes.NotFound, "not found")
	ErrorBadRequest               = status.Error(codes.InvalidArgument, "bad request")
)

const (
	gl_html_page_template = "template"
	gl_html_page_data     = "data"
)

func (p *HtmlPage) Save(ctx context.Context, db *redis.ClusterClient, key string, ttl time.Duration) error {
	if _, err := db.HSet(ctx, key, gl_html_page_template, p.Template, gl_html_page_data, p.Data).Result(); err != nil {
		return err
	}

	// FIXME https://redis.io/docs/latest/commands/hpexpire/
	if ttl > 0 {
		slog.Warn("HPEXPIRE Command available since: Redis Open Source 7.4.0")
	}
	// if _, err := db.HExpire(ctx, key, ttl, gl_html_page_template, gl_html_page_data).Result(); err != nil {
	// 	return err
	// }
	return nil
}

func (p *HtmlPage) Load(ctx context.Context, db *redis.ClusterClient, key string) error {
	tpl, err := db.HGet(ctx, key, gl_html_page_template).Result()
	if err != nil {
		return err
	}
	data, err := db.HGet(ctx, key, gl_html_page_data).Bytes()
	if err != nil {
		return err
	}
	p.Template = tpl
	p.Data = data
	return nil
}

func DelHtmlPage(ctx context.Context, db *redis.ClusterClient, key string) error {
	_, err := db.HDel(ctx, key, gl_html_page_template, gl_html_page_data).Result()
	return err
}

func ExistsHtmlPage(ctx context.Context, db *redis.ClusterClient, key string) (bool, error) {
	ok_t, err := db.HExists(ctx, key, gl_html_page_template).Result()
	if err != nil {
		return false, err
	}
	ok_d, err := db.HExists(ctx, key, gl_html_page_data).Result()
	if err != nil {
		return false, err
	}

	return ok_t && ok_d, nil
}
