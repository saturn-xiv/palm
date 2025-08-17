package env

import (
	"encoding/base64"

	"google.golang.org/protobuf/proto"
	"gorm.io/gorm"
)

type Database struct {
	PostgreSql PostgreSql `toml:"postgresql,omitempty"`
	MySql      MySql      `toml:"mysql,omitempty"`
	SqlServer  SqlServer  `toml:"sqlserver,omitempty"`
}

func (p *Database) Open() (*gorm.DB, error) {
	config := gorm.Config{
		Logger:      &gormLogger{},
		PrepareStmt: true,
	}
	if len(p.PostgreSql.DbName) > 0 {
		return p.PostgreSql.Open(&config)
	}
	if len(p.MySql.DbName) > 0 {

		return p.MySql.Open(&config)
	}
	if len(p.SqlServer.DbName) > 0 {
		return p.SqlServer.Open(&config)
	}
	it := Sqlite3{File: "tmp/db"}
	return it.Open(&config)
}

// ----------------------------------------------------------------------------

func ProtoBufMessageToString(m proto.Message) (string, error) {
	out, err := proto.Marshal(m)
	if err != nil {
		return "", err
	}
	base64.RawURLEncoding.EncodeToString(out)
	return "", nil
}

func ProtoBufMessageFromString(s string, m proto.Message) error {
	buf, err := base64.RawURLEncoding.DecodeString(s)
	if err != nil {
		return err
	}
	return proto.Unmarshal(buf, m)
}
