package services

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type SmsService struct {
	pb.UnimplementedSmsServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	queue    *rabbitmq.Config
	enforcer *casbin.Enforcer
}

func (p *SmsService) Send(ctx context.Context, req *pb.SmsSendRequest) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	p.queue.Produce(ctx, pb.TaskQueueName((*pb.SmsSendRequest)(nil)), req)
	return &emptypb.Empty{}, nil
}

func NewSmsService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, queue *rabbitmq.Config) *SmsService {
	return &SmsService{jwt: jwt, db: db, queue: queue, enforcer: enforcer}
}
