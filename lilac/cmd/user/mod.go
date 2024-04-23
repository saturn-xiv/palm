package user

import (
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

type Config struct {
	Namespace string        `toml:"namespace"`
	Redis     redis.Cluster `toml:"redis"`
	Database  env.Database  `toml:"database"`
}

func Open(config_file string, keys_dir string) (*Command, error) {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return nil, err
	}

	db, err := config.Database.Open()
	if err != nil {
		return nil, err
	}
	if err = models.AutoMigrate(db); err != nil {
		return nil, err
	}
	cache, err := config.Redis.Open(config.Namespace)
	if err != nil {
		return nil, err
	}
	i18n, err := i18n.New(db)
	if err != nil {
		return nil, err
	}
	enforcer, err := env.OpenCasbinEnforcer(config.Namespace, db, config.Redis.Options().Addrs)
	if err != nil {
		return nil, err
	}

	_, mac, _, err := crypto.Open(keys_dir)
	if err != nil {
		return nil, err
	}
	return &Command{mac: mac, i18n: i18n, cache: cache, db: db, enforcer: enforcer}, nil
}

type Command struct {
	mac      *crypto.HMac
	i18n     *i18n.I18n
	cache    *redis.Client
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func user_from_email(db *gorm.DB, email_ string) (*models.EmailUser, *models.User, error) {
	email, err := auth.IsEmail(email_)
	if err != nil {
		return nil, nil, err
	}
	var user models.User
	var eu models.EmailUser
	{
		if rst := db.Where(&models.EmailUser{
			Email: email,
		}).First(&eu); rst.Error != nil {
			return nil, nil, rst.Error
		}

		if rst := db.First(&user, eu.UserID); rst.Error != nil {
			return nil, nil, rst.Error
		}
	}
	return &eu, &user, nil
}
func role_from_code(code string) (*pb.Role, error) {
	role, err := auth.IsCode(code)
	if err != nil {
		return nil, err
	}
	if role == "administrator" {
		return &pb.Role{
			By: &pb.Role_Administrator_{},
		}, nil
	}
	if role == "root" {
		return &pb.Role{
			By: &pb.Role_Root_{},
		}, nil
	}
	return &pb.Role{
		By: &pb.Role_Member{Member: role},
	}, nil
}
