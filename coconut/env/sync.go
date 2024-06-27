package env

type Sync struct{}

func (p *Sync) Dump(target string, keep uint) []string {
	return []string{}
}
func (p *Sync) Restore() string {
	return `
`
}
