package app

import (
	"errors"
	"log/slog"

	"github.com/BurntSushi/toml"
	"github.com/saturn-xiv/palm/loquat/graphql"
	"github.com/saturn-xiv/palm/loquat/models"
	"gorm.io/gorm"
)

type NetScanConfig struct {
	PostgreSql PostgreSql `toml:"postgresql"`
}

func NetScan(config_file string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config NetScanConfig

	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.PostgreSql.Open(debug)
	if err != nil {
		return err
	}

	if err = nmap_scan(db); err != nil {
		return err
	}
	return nil
}

func nmap_scan(db *gorm.DB) error {
	router, err := graphql.Export(db)
	if err != nil {
		return err
	}

	network := []string{}
	if router.Dmz != nil {
		network = append(network, router.Dmz.Network.Address)
	}
	if router.Lan != nil {
		network = append(network, router.Lan.Network.Address)
	}
	if len(network) == 0 {
		slog.Debug("empty local network")
		return nil
	}

	hosts, err := models.ScanHosts(network...)
	if err != nil {
		return err
	}
	inserted := 0
	updated := 0
	if err = db.Transaction(func(tx *gorm.DB) error {
		for _, host := range hosts {
			var it models.Host
			err := db.Where(&host, "mac", "network").Take(&it).Error
			if err == nil {
				if err = db.Model(&it).Updates(map[string]interface{}{
					"name":    host.Name,
					"ip":      host.Ip,
					"vendor":  host.Vendor,
					"version": it.Version + 1,
				}).Error; err != nil {
					return err
				}
				updated += 1
			} else if errors.Is(err, gorm.ErrRecordNotFound) {
				if err = db.Create(&host).Error; err != nil {
					return err
				}
				inserted += 1
			} else {
				return err
			}
		}
		return nil
	}); err != nil {
		return err
	}
	slog.Info("succeed", "inserted", inserted, "updated", updated)
	return nil
}
