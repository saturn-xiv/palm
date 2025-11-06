package graphql

type Root struct{}

func (p *Root) Query() *Query {
	return &Query{}
}

func (p *Root) Mutation() *Mutation {
	return &Mutation{}
}
