package db

import (
	"context"
	"fmt"
	"log/slog"
	"path/filepath"

	"github.com/BurntSushi/toml"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	http_ "github.com/saturn-xiv/palm/jasmine/cmd/http"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	portal_v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func Seed(config_file string, locales_folder string) error {
	slog.Debug("load configuration from", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	aes, _, _, err := crypto.Open(config.SecretsStore)
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err = db.Transaction(func(tx *gorm.DB) error {
		if len(locales_folder) > 0 {
			if err := load_locales(tx, locales_folder); err != nil {
				return err
			}
		}
		if err := load_currencies(tx); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}

	ctx := context.Background()
	cache, err := config.Redis.Open(ctx)
	if err != nil {
		return err
	}
	if err = set_home_page(ctx, db, cache, aes); err != nil {
		return err
	}
	if err = set_sample_page(ctx, cache); err != nil {
		return err
	}

	slog.Info("done.")
	return nil
}

func set_home_page(ctx context.Context, db *gorm.DB, redis *redis.Client, aes *crypto.Aes) error {
	languages, err := models.Languages(db)
	if err != nil {
		return err
	}
	for _, lang := range languages {
		page := portal_v2.HtmlPage{Hash: fmt.Sprintf("home.%s", lang)}
		if err = redis.GetJson(ctx, &page); err != nil {
			page.Template = "home"
			slog.Warn("set homepage", slog.String("lang", lang), slog.String("hash", page.Hash), slog.String("template", page.Template))
			buf, err := web.ProtoBufMessageToJson(&portal_v2.Theme_Bootstrap_Home{})
			if err != nil {
				return err
			}
			page.Body = &portal_v2.HtmlPage_Data{Data: buf}
			if err = redis.SetJson(ctx, &page, 0); err != nil {
				return err
			}
		}
	}

	return nil
}

func build_sample_data() *portal_v2.Theme_Bootstrap_Sample_Data {
	data := portal_v2.Theme_Bootstrap_Sample_Data{
		Header: &portal_v2.Theme_Bootstrap_Sample_Data_Header{
			Title: "Sample page title",
		},
		Footer: &portal_v2.Theme_Bootstrap_Sample_Data_Footer{
			Copyright: "Copyright &copy; 2025 My Website. All rights reserved.",
		},
		Body: &portal_v2.Theme_Bootstrap_Sample_Data_Body{
			Title:     "Page title",
			CreatedAt: timestamppb.Now(),
		},
	}

	for i := range 12 {
		it := portal_v2.Theme_Bootstrap_Sample_Data_Body_Item{
			Title:       fmt.Sprintf("Item title %d", i),
			Description: fmt.Sprintf("Item description %d", i),
			Panels:      make(map[string]*portal_v2.Theme_Bootstrap_Sample_Data_Body_Panel),
		}
		for j := range 6 {
			panel := portal_v2.Theme_Bootstrap_Sample_Data_Body_Panel{
				Title:       fmt.Sprintf("Item title (%d,%d)", i, j),
				Description: fmt.Sprintf("Item description (%d,%d)", i, j),
			}
			for k := range 4 {
				panel.Links = append(panel.Links, &portal_v2.Theme_Bootstrap_Sample_Data_Body_Link{
					Label: fmt.Sprintf("link (%d,%d,%d)", i, j, k),
					Href:  fmt.Sprintf("#sample-(%d,%d,%d)", i, j, k),
				})
			}
			it.Panels[fmt.Sprintf("panel-%d", j)] = &panel
		}
		data.Body.Items = append(data.Body.Items, &it)
	}

	return &data
}

func set_sample_page(ctx context.Context, cli *redis.Client) error {
	opts := protojson.MarshalOptions{
		Indent: "  ",
	}

	sample, err := opts.Marshal(build_sample_data())
	if err != nil {
		return err
	}

	{

		page := portal_v2.HtmlPage{Hash: "samples-tpl-show", Template: "samples.tpl.show", Body: &portal_v2.HtmlPage_Data{Data: sample}}
		slog.Info("setup page", slog.String("path", page.Path()), slog.String("template", page.Template))
		if err := cli.SetJson(ctx, &page, 0); err != nil {
			return err
		}
	}

	{

		arg := portal_v2.Theme_Bootstrap_Sample{
			Data:      string(sample),
			Templates: make(map[string]string),
		}
		for _, it := range []string{"layout", "item", "show"} {
			name := filepath.Join("bootstrap", "sample", fmt.Sprintf("%s.html", it))
			buf, err := http_.ReadHtmlTemplate(filepath.Join("views", name))
			if err != nil {
				return err
			}
			arg.Templates[name] = string(buf)
		}
		data, err := web.ProtoBufMessageToJson(&arg)
		if err != nil {
			return err
		}
		page := portal_v2.HtmlPage{Hash: "samples-tpl", Template: "samples.tpl.index", Body: &portal_v2.HtmlPage_Data{Data: data}}
		slog.Info("setup page", slog.String("path", page.Path()), slog.String("template", page.Template))
		if err := cli.SetJson(ctx, &page, 0); err != nil {
			return err
		}
	}
	return nil
}

func load_locales(db *gorm.DB, folder string) error {
	total, inserted, err := models.LoadLocalesByToml(db, folder)
	if err != nil {
		return err
	}
	slog.Info("locales", slog.Int("found", total), slog.Int("inserted", inserted))
	return nil
}

func load_currencies(db *gorm.DB) error {
	var count int64
	if err := db.Model(&models.Currency{}).Count(&count).Error; err != nil {
		return err
	}
	if count > 0 {
		slog.Warn("currency tables isn't empty, skipped")
		return nil
	}
	total, inserted, err := models.LoadIso4217ListOne(db)
	if err != nil {
		return err
	}
	slog.Info("currencies", slog.Int("found", total), slog.Int("inserted", inserted))
	return nil
}
