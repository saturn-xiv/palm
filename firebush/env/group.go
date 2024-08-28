package env

import "gopkg.in/ini.v1"

func NewGroup(name string, config *ini.File) (*Group, error) {
	item := Group{
		Name:        name,
		Nodes:       []*Node{},
		Environment: map[string]interface{}{},
	}
	load_env(config.Section(""), item.Environment)
	return &item, nil
}

type Group struct {
	Name        string
	Nodes       []*Node
	Environment map[string]interface{}
}
