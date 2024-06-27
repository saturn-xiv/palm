package env

type Oracle struct{}

func (p *Oracle) Dump(target string, keep uint) []string {
	return []string{}
}
func (p *Oracle) Restore() string {
	return `
`
}
