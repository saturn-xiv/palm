package env

import (
	"fmt"
	"io"
)

const (
	GROUP_ALL       = "all"
	GROUP_KEY_HOSTS = "hosts"

	KEY_SSH_PORT          = "ssh.port"
	KEY_SSH_USER          = "ssh.user"
	KEY_SSH_PASSWORD      = "ssh.password"
	KEY_SSH_KEY_FILE      = "ssh.key-file"
	KEY_SSH_SUDO_PASSWORD = "ssh.sudo-password"
)

type Environment map[string]interface{}

func (p *Environment) Display(w io.Writer) {
	for k, v := range *p {
		fmt.Fprintf(w, "%s = %v\n", k, v)
	}
}

func (p Environment) Node(name string) *Node {
	it := Node{
		Host: name,
	}
	if v, ok := p[KEY_SSH_PORT]; ok {
		if v, ok := v.(int64); ok {
			p := uint16(v)
			it.Port = &p
		}
	}
	if v, ok := p[KEY_SSH_USER]; ok {
		if v, ok := v.(string); ok {
			it.User = &v
		}
	}
	if v, ok := p[KEY_SSH_PASSWORD]; ok {
		if v, ok := v.(string); ok {
			it.Password = &v
		}
	}
	if v, ok := p[KEY_SSH_SUDO_PASSWORD]; ok {
		if v, ok := v.(string); ok {
			it.SudoPassword = &v
		}
	}
	if v, ok := p[KEY_SSH_KEY_FILE]; ok {
		if v, ok := v.(string); ok {
			it.KeyFile = &v
		}
	}
	return &it
}
