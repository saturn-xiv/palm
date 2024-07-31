package models

import (
	"reflect"

	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func CreateLog(db *gorm.DB, user uint64, lang string, ip string, level pb.UserLogsResponse_Item_Level, plugin any, resource_type any, resource_id *uint64, message string, args interface{}) error {
	return db.Create(&Log{
		UserID:       user,
		Plugin:       reflect.TypeOf(plugin).Elem().PkgPath(),
		ResourceType: reflect.TypeOf(resource_type).Elem().Name(),
		ResourceID:   resource_id,
		IP:           ip,
		Message:      T(db, lang, message, args),
		Level:        level.String(),
	}).Error
}
