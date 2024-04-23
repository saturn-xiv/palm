package user

import (
	"fmt"

	"github.com/saturn-xiv/palm/lilac/models"
)

func (p *Command) List() error {
	var items []models.EmailUser
	if rst := p.db.Order("nickname ASC").Find(&items); rst.Error != nil {
		return rst.Error
	}
	fmt.Println("Nickname\tEmail\tReal Name")
	for _, it := range items {
		fmt.Printf("%s\t%s\t%s\n", it.Nickname, it.Email, it.RealName)
	}
	return nil
}
