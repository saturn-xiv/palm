package controllers

import (
	"context"
	"time"

	"github.com/gorilla/sessions"
	redis_ "github.com/redis/go-redis/v9"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
	v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
)

type Context struct {
	DB      *gorm.DB
	Redis   *redis.Client
	Session sessions.Store
	Jwt     *crypto.Jwt
	Aes     *crypto.Aes
}

func SetHtmlPage(ctx context.Context, cli *redis.Client, hash string, page *v2.HtmlPage, ttl time.Duration) error {
	return cli.Execute(func(db *redis_.ClusterClient, key func(string) string) error {
		return page.Save(ctx, db, key(hash), ttl)
	})

}

func GetHtmlPage(ctx context.Context, cli *redis.Client, hash string) (*v2.HtmlPage, error) {
	var page v2.HtmlPage
	if err := cli.Execute(func(db *redis_.ClusterClient, key func(string) string) error {
		return page.Load(ctx, db, key(hash))
	}); err != nil {
		return nil, err
	}

	return &page, nil
}

func DelHtmlPage(ctx context.Context, cli *redis.Client, hash string) error {
	return cli.Execute(func(db *redis_.ClusterClient, key func(string) string) error {
		return v2.DelHtmlPage(ctx, db, key(hash))
	})
}

// func ExistsHtmlPage(ctx context.Context, cli *redis.Client, hash string) (bool, error) {
// 	var ok bool
// 	err := cli.Execute(func(db *redis_.ClusterClient, key func(string) string) error {
// 		var err error
// 		ok, err = v2.ExistsHtmlPage(ctx, db, key(hash))
// 		return err
// 	})
// 	return ok, err
// }
