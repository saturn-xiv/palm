package auth

import (
	"context"
	"log/slog"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
)

type SiteService struct {
	pb.UnimplementedSiteServer

	jwt      *crypto.Jwt
	aes      *crypto.Aes
	db       *gorm.DB
	i18n     *i18n.I18n
	enforcer *casbin.Enforcer
	version  string
}

func (p *SiteService) Set(ctx context.Context, req *pb.KvSetRequest) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err = models.Set_(p.db, p.aes, req.Key, req.Value, req.Encrypt); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) Get(ctx context.Context, req *pb.KvGetRequest) (*pb.KvGetResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	buf, err := models.Get_(p.db, p.aes, req.Key)
	if err != nil {
		return nil, err
	}
	return &pb.KvGetResponse{Value: buf}, nil
}

func (p *SiteService) Layout(ctx context.Context, req *pb.SiteLayoutRequest) (*pb.SiteLayoutResponse, error) {
	now := time.Now()
	var keywords pb.SiteKeywords
	if err := models.GetPB_(p.db, p.aes, &keywords); err != nil {
		slog.Error(err.Error())
	}
	var authors pb.SiteAuthors
	if err := models.GetPB_(p.db, p.aes, &authors); err != nil {
		slog.Error(err.Error())
	}
	var favicon pb.SiteFavicon
	if err := models.GetPB_(p.db, p.aes, &favicon); err != nil {
		slog.Error(err.Error())
	}
	res := pb.SiteLayoutResponse{
		Title:       p.i18n.Tr(req.Lang, gl_site_title, map[string]interface{}{}),
		Subhead:     p.i18n.Tr(req.Lang, gl_site_subhead, map[string]interface{}{}),
		Description: p.i18n.Tr(req.Lang, gl_site_description, map[string]interface{}{}),
		Copyright: p.i18n.Tr(req.Lang, gl_site_copyright, map[string]interface{}{
			"year": now.Year(),
		}),
		Favicon:   favicon.Url,
		Keywords:  keywords.Items,
		Authors:   authors.Items,
		Locale:    req.Lang,
		Languages: p.i18n.Languages(),
		Version:   p.version,
	}
	var gab pb.SiteLayoutResponse_Gab
	if err := models.GetPB_(p.db, p.aes, &gab); err != nil {
		slog.Error(err.Error())
	} else {
		res.Gab = &gab
	}
	var icp pb.SiteLayoutResponse_Icp
	if err := models.GetPB_(p.db, p.aes, &icp); err != nil {
		slog.Error(err.Error())
	} else {
		res.Icp = &icp
	}
	return &res, nil
}

func (p *SiteService) SetInfo(ctx context.Context, req *pb.SetSiteInfoRequest) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := models.SetLocale(tx, req.Lang, gl_site_title, req.Title); err != nil {
			return err
		}
		if err := models.SetLocale(tx, req.Lang, gl_site_subhead, req.Subhead); err != nil {
			return err
		}
		if err := models.SetLocale(tx, req.Lang, gl_site_description, req.Description); err != nil {
			return err
		}
		if err := models.SetLocale(tx, req.Lang, gl_site_copyright, req.Copyright); err != nil {
			return err
		}
		if err := p.i18n.Reload(tx); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetInfo(ctx context.Context, req *pb.GetSiteInfoRequest) (*pb.GetSiteInfoResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	return &pb.GetSiteInfoResponse{
		Title:       p.i18n.Tr(req.Lang, gl_site_title, map[string]interface{}{}),
		Subhead:     p.i18n.Tr(req.Lang, gl_site_subhead, map[string]interface{}{}),
		Description: p.i18n.Tr(req.Lang, gl_site_description, map[string]interface{}{}),
		Copyright:   p.i18n.Tr(req.Lang, gl_site_copyright, map[string]interface{}{}),
	}, nil
}
func (p *SiteService) SetFavicon(ctx context.Context, req *pb.SiteFavicon) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err = models.SetPB_(p.db, p.aes, req, false); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetFavicon(ctx context.Context, req *emptypb.Empty) (*pb.SiteFavicon, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteFavicon
	if err = models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) SetAuthors(ctx context.Context, req *pb.SiteAuthors) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err = models.SetPB_(p.db, p.aes, req, false); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetAuthors(ctx context.Context, req *emptypb.Empty) (*pb.SiteAuthors, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteAuthors
	if err = models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) SetKeywords(ctx context.Context, req *pb.SiteKeywords) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err = models.SetPB_(p.db, p.aes, req, false); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetKeywords(ctx context.Context, req *emptypb.Empty) (*pb.SiteKeywords, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteKeywords
	if err = models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}

func (p *SiteService) SetIcpCode(ctx context.Context, req *pb.SiteLayoutResponse_Icp) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) GetIcpCode(ctx context.Context, req *emptypb.Empty) (*pb.SiteLayoutResponse_Icp, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteLayoutResponse_Icp
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteIcpCode(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.DeletePB_(tx, (*pb.SiteLayoutResponse_Icp)(nil))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) SetGabCode(ctx context.Context, req *pb.SiteLayoutResponse_Gab) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetGabCode(ctx context.Context, req *emptypb.Empty) (*pb.SiteLayoutResponse_Gab, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteLayoutResponse_Gab
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteGabCode(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.DeletePB_(tx, (*pb.SiteLayoutResponse_Gab)(nil))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) SetBaidu(ctx context.Context, req *pb.SiteBaidu) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetBaidu(ctx context.Context, req *emptypb.Empty) (*pb.SiteBaidu, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteBaidu
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteBaidu(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.DeletePB_(tx, (*pb.SiteBaidu)(nil))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) PingBaidu(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var it pb.SiteBaidu
	if err := models.GetPB_(p.db, p.aes, &it); err != nil {
		return nil, err
	}
	if err := it.Ping(); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) SetGoogle(ctx context.Context, req *pb.SiteGoogle) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetGoogle(ctx context.Context, req *emptypb.Empty) (*pb.SiteGoogle, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteGoogle
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteGoogle(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.DeletePB_(tx, (*pb.SiteGoogle)(nil))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) SetIndexNow(ctx context.Context, req *pb.SiteIndexNow) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetIndexNow(ctx context.Context, req *emptypb.Empty) (*pb.SiteIndexNow, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteIndexNow
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteIndexNow(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		models.DeletePB_(tx, (*pb.SiteIndexNow)(nil))
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) PingIndexNow(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var it pb.SiteIndexNow
	if err := models.GetPB_(p.db, p.aes, &it); err != nil {
		return nil, err
	}
	if err := it.Ping(); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SiteService) SetGoogleReCaptcha(ctx context.Context, req *pb.SiteGoogleReCaptcha) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetPB_(tx, p.aes, req, true)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *SiteService) GetSiteGoogleReCaptcha(ctx context.Context, req *emptypb.Empty) (*pb.SiteGoogleReCaptcha, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var res pb.SiteGoogleReCaptcha
	if err := models.GetPB_(p.db, p.aes, &res); err != nil {
		return nil, err
	}
	return &res, nil
}
func (p *SiteService) DeleteSiteGoogleReCaptcha(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.DeletePB_(tx, (*pb.SiteGoogleReCaptcha)(nil))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func NewSiteService(db *gorm.DB, aes *crypto.Aes, jwt *crypto.Jwt, enforcer *casbin.Enforcer, i18n *i18n.I18n, version string) *SiteService {
	return &SiteService{db: db, aes: aes, jwt: jwt, enforcer: enforcer, i18n: i18n, version: version}
}

const (
	gl_site_title       = "site.title"
	gl_site_subhead     = "site.subhead"
	gl_site_description = "site.description"
	gl_site_copyright   = "site.copyright"
)
