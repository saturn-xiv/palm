package db

import (
	"context"
	"fmt"
	"log/slog"
	"path/filepath"

	"github.com/BurntSushi/toml"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	http_ "github.com/saturn-xiv/palm/jasmine/cmd/http"
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

	if err = set_sample_page(ctx, cache); err != nil {
		return err
	}

	slog.Info("done.")
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
	sample := build_sample_data()
	data, err := web.ProtoBufMessageToJson(sample)
	if err != nil {
		return err
	}

	{
		tpl := "sample.show"
		key := "sample.show.data.en_US"
		url := fmt.Sprintf("/%s-%s.html", tpl, key)
		slog.Info("setup sample-show page", slog.String("path", url))
		if err = cli.Set(ctx, key, data, 0); err != nil {
			return err
		}
	}
	{
		tpl := "sample.details"
		key := "sample.details.data.en_US"
		url := fmt.Sprintf("/%s-%s.html", tpl, key)
		slog.Info("setup sample-tpl page", slog.String("path", url))

		arg := portal_v2.Theme_Bootstrap_Sample{
			Data:      string(data),
			Templates: make(map[string]string),
		}
		for _, it := range []string{"layout/footer", "layout/header", "item", "show"} {
			name := filepath.Join("views", "bootstrap", "sample", fmt.Sprintf("%s.html", it))
			buf, err := http_.ReadHtmlTemplate(name)
			if err != nil {
				return err
			}
			arg.Templates[name] = string(buf)
		}
		val, err := web.ProtoBufMessageToJson(&arg)
		if err != nil {
			return err
		}
		if err = cli.Set(ctx, key, val, 0); err != nil {
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
