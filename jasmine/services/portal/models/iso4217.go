package models

import "gorm.io/gorm"

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
	gorm.Model
	Name    string
	Code    string
	Number  string
	Country string
	Units   *uint8
	Fund    *bool
}
