package models

import (
	"log/slog"
	"time"

	"gorm.io/gorm"
)

// struct Currency {
//   int id;
//   std::string name;
//   std::string code;
//   std::string number;
//   std::string country;
//   boost::optional<int> units;
//   boost::optional<int> is_fund;
// };

// type Currency struct{

// }

// https://www.iso.org/iso-4217-currency-codes.html
type Currency struct {
	ID        uint32 `gorm:"primaryKey"`
	Name      string
	Code      string
	Number    string
	Country   string
	Units     *uint8
	Fund      *bool
	CreatedAt time.Time
}

func (Currency) TableName() string {
	return "currencies"
}

type currencyItem struct {
	name    string
	code    string
	number  string
	country string
	units   *uint8
	fund    *bool
}

func LoadIso4217ListOne(db *gorm.DB) (int, int, error) {

	items, err := load_currencies_from_iso4317_list_one()
	if err != nil {
		return 0, 0, err
	}
	inserted := 0
	now := time.Now()
	for _, it := range items {
		slog.Debug("create", slog.String("name", it.name), slog.String("code", it.code), slog.String("country", it.country))
		db.Create(&Currency{Name: it.name, Code: it.code, Number: it.number, Country: it.country, Units: it.units, Fund: it.fund, CreatedAt: now})
		inserted = inserted + 1
	}
	return len(items), inserted, nil
}

func load_currencies_from_iso4317_list_one() ([]currencyItem, error) {
	slog.Debug("load currencies from ISO4217/list-one.xml")
	items := make([]currencyItem, 0)
	return items, nil

}
