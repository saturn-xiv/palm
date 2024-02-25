package v2

import "gopkg.in/gomail.v2"

func (p *SendEmailTask_Address) Address() string {
	msg := gomail.NewMessage()
	return msg.FormatAddress(p.Email, p.Name)
}
