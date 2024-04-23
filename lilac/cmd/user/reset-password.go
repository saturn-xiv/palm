package user

import (
	"time"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/models"
)

func (p *Command) Password(email_ string, password string) error {

	if err := auth.IsPassword(password); err != nil {
		return err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		eu, user, err := user_from_email(tx, email_)
		if err != nil {
			return err
		}
		{

			password, salt, err := models.ComputePassword(p.mac, password, 8)
			if err != nil {
				return err
			}
			eu.Password = password
			eu.Salt = salt
			eu.Version += 1
			eu.UpdatedAt = now
			if rst := tx.Save(eu); rst.Error != nil {
				return rst.Error
			}
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    user.ID,
			Message:   "Reset password.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return err
	}
	return nil
}
