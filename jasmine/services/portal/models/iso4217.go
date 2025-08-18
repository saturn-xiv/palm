package models

import (
	_ "embed"
	"encoding/xml"
	"log/slog"
	"strconv"
	"time"

	"gorm.io/gorm"
)

// https://www.iso.org/iso-4217-currency-codes.html
type Currency struct {
	ID        uint32 `gorm:"primaryKey"`
	Name      string
	Code      string
	Number    string
	Country   string
	Units     *uint32
	IsFund      *bool
	CreatedAt time.Time
}

func (Currency) TableName() string {
	return "currencies"
}

type iso_4217 struct {
	XMLName xml.Name `xml:"ISO_4217"`
	Pblshd  string   `xml:"Pblshd,attr"`
	CcyTbl  CcyTbl   `xml:"CcyTbl"`
}
type CcyTbl struct {
	CcyNtry []CcyNtry `xml:"CcyNtry"`
}
type CcyNtry struct {
	CtryNm     string  `xml:"CtryNm"`
	CcyNm      CcyNm   `xml:"CcyNm"`
	Ccy        string  `xml:"Ccy"`
	CcyNbr     string  `xml:"CcyNbr"`
	CcyMnrUnts *string `xml:"CcyMnrUnts"`
}
type CcyNm struct {
	IsFund *bool  `xml:"IsFund,attr"`
	Text   string `xml:",chardata"`
}

type currencyItem struct {
	name    string
	code    string
	number  string
	country string
	units   *int
	fund    *bool
}

func LoadIso4217ListOne(db *gorm.DB) (int, int, error) {
	items, err := load_currencies_from_iso4317_list_one()
	if err != nil {
		return 0, 0, err
	}
	inserted := 0
	for _, it := range items {
		slog.Debug("create", slog.String("name", it.name), slog.String("code", it.code), slog.String("country", it.country))
		var units *uint32
		if it.units != nil {
			v := uint32(*it.units)
			units = &v
		}
		if err = db.Create(&Currency{Name: it.name, Code: it.code, Number: it.number, Country: it.country, Units: units, IsFund: it.fund}).Error; err != nil {
			return len(items), 0, nil
		}
		inserted = inserted + 1
	}
	return len(items), inserted, nil
}

//go:embed iso4217/list-one.xml
var iso4217_list_one_xml []byte

func load_currencies_from_iso4317_list_one() ([]currencyItem, error) {
	slog.Debug("load currencies from ISO4217/list-one.xml")

	var root iso_4217
	if err := xml.Unmarshal(iso4217_list_one_xml, &root); err != nil {
		return nil, err
	}
	items := make([]currencyItem, 0)
	for _, it := range root.CcyTbl.CcyNtry {
		if it.CcyMnrUnts == nil {
			continue
		}
		jt := currencyItem{
			name:    it.CcyNm.Text,
			code:    it.Ccy,
			country: it.CtryNm,
			number:  it.CcyNbr,
			fund:    it.CcyNm.IsFund,
		}
		if *it.CcyMnrUnts != "N.A." {
			units, err := strconv.Atoi(*it.CcyMnrUnts)
			if err != nil {
				return nil, err
			}
			jt.units = &units
		}

		items = append(items, jt)
	}
	return items, nil

}
