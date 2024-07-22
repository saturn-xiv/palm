package services

import (
	"context"
	"fmt"
	"time"

	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat "github.com/saturn-xiv/palm/atropa/env/wechat-mini-program"
	pb "github.com/saturn-xiv/palm/atropa/wechat/services/v2"
)

func NewMiniProgramService(redis *redis.Client, wechat *wechat.Config) *MiniProgramService {
	return &MiniProgramService{wechat: wechat, redis: redis}
}

type MiniProgramService struct {
	pb.UnimplementedMiniProgramServer

	wechat *wechat.Config
	redis  *redis.Client
}

func (p *MiniProgramService) Code2Session(ctx context.Context, req *pb.MiniProgramCode2SessionRequest) (*pb.MiniProgramCode2SessionResponse, error) {
	session, err := p.wechat.Code2Session(req.Code)
	if err != nil {
		return nil, err
	}
	if err = p.redis.Set(ctx, p.session_key(session.OpenID), session.SessionKey, time.Hour*24); err != nil {
		return nil, err
	}
	return &pb.MiniProgramCode2SessionResponse{
		OpenId:  session.OpenID,
		UnionId: session.UnionID,
	}, nil
}

func (p *MiniProgramService) session_key(open_id string) string {
	return fmt.Sprintf("/weichat-mini-program/%s/session-key/%s", p.wechat.AppID, open_id)
}
