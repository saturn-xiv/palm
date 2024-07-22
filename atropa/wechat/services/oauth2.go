package services

import (
	"bytes"
	"context"
	"encoding/gob"
	"time"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	wechat "github.com/saturn-xiv/palm/atropa/env/wechat-oauth2"
	pb "github.com/saturn-xiv/palm/atropa/wechat/services/v2"
)

var gl_wechat_oauth2_audience = "wechat.oauth2"

func NewOauth2Service(jwt *crypto.Jwt, wechat *wechat.Config) *Oauth2Service {
	return &Oauth2Service{jwt: jwt, wechat: wechat}
}

type Oauth2Service struct {
	pb.UnimplementedOauth2Server

	wechat *wechat.Config
	jwt    *crypto.Jwt
}

func (p *Oauth2Service) QrConnectUrl(ctx context.Context, req *pb.Oauth2QrConnectUrlRequest) (*pb.Oauth2QrConnectUrlResponse, error) {
	now := time.Now()
	exp := now.Add(time.Minute * 5)
	subject := ""
	if req.Subject != nil {
		subject = *req.Subject
	}
	token, err := p.jwt.Sign(env.JWT_ISSUER, subject, []string{gl_wechat_oauth2_audience}, map[string]interface{}{}, &now, &exp)
	if err != nil {
		return nil, err
	}
	return &pb.Oauth2QrConnectUrlResponse{
		Url: p.wechat.QrConnectURL(token, req.Lang.ToString()),
	}, nil
}

func (p *Oauth2Service) SignIn(ctx context.Context, req *pb.Oauth2SignInRequest) (*pb.Oauth2SignInResponse, error) {
	_, subject, _, err := p.jwt.Verify(req.State, env.JWT_ISSUER, gl_wechat_oauth2_audience)
	if err != nil {
		return nil, err
	}

	token, err := p.wechat.AccessToken(req.Code)
	if err != nil {
		return nil, err
	}
	var token_buf bytes.Buffer
	{
		enc := gob.NewEncoder(&token_buf)
		if err = enc.Encode(token); err != nil {
			return nil, err
		}
	}

	user_info, err := p.wechat.UserInfo(token.AccessToken, token.OpenID, req.Lang.ToString())
	if err != nil {
		return nil, err
	}

	it := pb.Oauth2SignInResponse{
		UserInfo: &pb.Oauth2SignInResponse_UserInfo{
			OpenId:     user_info.OpenID,
			Nickname:   user_info.Nickname,
			HeadImgUrl: user_info.HeadImgURL,
			UnionId:    user_info.UnionID,
			Sex:        user_info.Sex,
			City:       user_info.City,
			Country:    user_info.Country,
			Province:   user_info.Province,
			Privilege:  user_info.Privilege,
		},
		Token: token_buf.Bytes(),
	}
	if subject != "" {
		it.Subject = &subject
	}
	return &it, nil

}
