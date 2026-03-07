package graphql

import (
	"context"

	"gorm.io/gorm"
)

func (p *Mutation) SetIndexNow(ctx context.Context, args struct {
	Key    string
	Enable bool
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetBaiduSiteVerify(ctx context.Context, args struct {
	Id     string
	Enable bool
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetGoogleSiteVerify(ctx context.Context, args struct {
	Id     string
	Enable bool
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

// https://developers.google.com/recaptcha/intro
func (p *Mutation) SetReCaptcha(ctx context.Context, args struct {
	Key    string
	Secret string
	Enable bool
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetSiteKeywords(ctx context.Context, args struct {
	Items []string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetSiteInfo(ctx context.Context, args struct {
	Lang        string
	Title       string
	Subhead     string
	Description string
	Copyright   string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetSiteMaintenanceMode(ctx context.Context, args struct {
	Ok     bool
	Reason string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
func (p *Mutation) SetSiteFavicon(ctx context.Context, args struct {
	Url string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		// TODO
		return nil
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
