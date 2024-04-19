package v2

import "fmt"

func (p *EmailSendRequest_Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}
