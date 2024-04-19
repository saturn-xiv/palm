package email

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	pb "github.com/saturn-xiv/palm/lilac/email/v2"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
)

type Service struct {
	pb.UnimplementedEmailServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	queue    *rabbitmq.Config
	enforcer *casbin.Enforcer
}

func (p *Service) Send(ctx context.Context, req *pb.EmailSendRequest) (*emptypb.Empty, error) {
	user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	p.queue.Produce(ctx, env.TaskQueueName((*pb.EmailSendRequest)(nil)), req)
	return &emptypb.Empty{}, nil
}

func NewService(
	db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, queue *rabbitmq.Config) *Service {
	return &Service{db: db, jwt: jwt, queue: queue, enforcer: enforcer}
}
