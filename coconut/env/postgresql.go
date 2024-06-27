package env

type PostgreSql struct{}

func (p *PostgreSql) Dump(target string, keep uint) []string {
	return []string{}
}
func (p *PostgreSql) Restore() string {
	return `
`
}
