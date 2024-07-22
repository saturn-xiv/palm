package models

import (
	balsam "github.com/saturn-xiv/palm/atropa/balsam/models"
)

type TwilioSmsLogs struct {
	balsam.Model
	Body []byte `gorm:"not null"`
}
