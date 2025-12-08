package iso4217

import (
	_ "embed"
	"encoding/xml"
)

//go:embed list-one.xml
var gl_list_one_xml []byte

type ISO_4217 struct {
	XMLName xml.Name `xml:"ISO_4217"`

	Published string `xml:"Pblshd,attr"`
	Items     []Item `xml:"CcyTbl>CcyNtry"`
}

type Item struct {
	Country string `xml:"CtryNm"`
	Name    string `xml:"CcyNm"`
	Code    string `xml:"Ccy"`
	Number  uint   `xml:"CcyNbr"`
	Units   Units  `xml:"CcyMnrUnts"`
}

type Units struct {
	Fund  *bool  `xml:"IsFund,attr,omitempty"`
	Value string `xml:",chardata"`
}

func Iso4217() (*ISO_4217, error) {
	var it ISO_4217
	if err := xml.Unmarshal(gl_list_one_xml, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
