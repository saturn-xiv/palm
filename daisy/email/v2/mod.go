package v2

import (
	"bytes"
	"net/mail"

	go_mail "github.com/wneessen/go-mail"
)

func (p *Task) Build() (*go_mail.Msg, error) {
	msg := go_mail.NewMsg()
	msg.FromMailAddress(&mail.Address{Name: p.From.Name, Address: p.From.Email})
	msg.ToMailAddress(&mail.Address{Name: p.To.Name, Address: p.To.Email})
	for _, it := range p.Cc {
		msg.AddCcMailAddress(&mail.Address{Name: it.Name, Address: it.Email})
	}
	for _, it := range p.Bcc {
		msg.AddBccMailAddress(&mail.Address{Name: it.Name, Address: it.Email})
	}
	msg.Subject(p.Subject)
	if p.Body.Html {
		msg.SetBodyString(go_mail.TypeTextHTML, p.Body.Content)
	} else {
		msg.SetBodyString(go_mail.TypeTextPlain, p.Body.Content)
	}

	for _, it := range p.Attachments {
		rdr := bytes.NewReader(it.Content)
		if it.Inline {
			msg.EmbedReader(it.Name, rdr)
		} else {
			msg.AttachReader(it.Name, rdr)
		}
	}
	return msg, nil
}
