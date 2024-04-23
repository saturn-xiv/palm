package sms

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/sms/v2"
)

type Service struct {
	pb.UnimplementedSmsServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	queue    *rabbitmq.Config
	enforcer *casbin.Enforcer
}

func (p *Service) Send(ctx context.Context, req *pb.SmsSendRequest) (*emptypb.Empty, error) {
	user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	p.queue.ProducePB(ctx, req)
	return &emptypb.Empty{}, nil
}

func NewService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, queue *rabbitmq.Config) *Service {
	return &Service{jwt: jwt, db: db, queue: queue, enforcer: enforcer}
}
