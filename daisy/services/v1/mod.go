package v1

import "fmt"

func (p *Address) Display() string {
	return fmt.Sprintf("%s<%s>", p.Name, p.Email)
}
