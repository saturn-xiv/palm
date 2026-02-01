package models

import "time"

// https://en.wikipedia.org/wiki/Left-child_right-sibling_binary_tree
// https://falsinsoft.blogspot.com/2013/01/tree-in-sql-database-nested-set-model.html
type Category struct {
	ID        uint      `gorm:"primarykey"`
	Name      string    `gorm:"uniqueIndex;not null;size:255"`
	Left      uint      `gorm:"not null"`
	Right     uint      `gorm:"not null"`
	Version   uint      `gorm:"not null;default:0"`
	CreatedAt time.Time `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`

	Resources []*CategoryResource
}

func (Category) TableName() string {
	return "categories"
}

type CategoryResource struct {
	ID           uint   `gorm:"primarykey"`
	CategoryID   uint   `gorm:"not null"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceId   uint
	CreatedAt    time.Time `gorm:"not null"`

	Category *Category
}

func (CategoryResource) TableName() string {
	return "categories_resources"
}
