package v2

import "fmt"

func (p *EmailTask_Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}
