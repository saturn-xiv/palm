package env

import (
	"fmt"
	"log"
	"log/slog"
	"os"

	"golang.org/x/crypto/ssh"
	"gopkg.in/ini.v1"
)

type Execution func(env map[string]interface{}) ([]string, error)

func NewNode(host string, config *ini.File) (*Node, error) {
	item := Node{}
	// TODO
	return &item, nil
}

type Node struct {
	Host         string
	Port         *uint16
	User         *string
	Password     *string
	KeyFile      *string
	SudoPassword *string
	Environment  map[string]interface{}
}

func (p *Node) Execute(job string) error {
	address := p.address()
	slog.Info("execute", slog.String("host", address), slog.String("job", job))
	// TODO
	return p.execute([]Execution{})
}
func (p *Node) Ping() error {
	return p.execute([]Execution{
		func(_env map[string]interface{}) ([]string, error) {
			return []string{
				"date -u",
				"uname -a",
			}, nil
		},
	})
}

func (p *Node) execute(executions []Execution) error {
	address := p.address()
	var host_key ssh.PublicKey

	config := ssh.ClientConfig{
		User:            "root",
		Auth:            []ssh.AuthMethod{},
		HostKeyCallback: ssh.FixedHostKey(host_key),
	}
	if p.User != nil {
		config.User = *p.User
	}
	if p.Password != nil {
		config.Auth = append(config.Auth, ssh.Password(*p.Password))
	}
	if p.KeyFile != nil {
		buf, err := os.ReadFile(*p.KeyFile)
		if err != nil {
			return err
		}
		slog.Debug("load key file", slog.String("host", address), slog.String("file", *p.KeyFile))
		signer, err := ssh.ParsePrivateKey(buf)
		if err != nil {
			return err
		}
		config.Auth = append(config.Auth, ssh.PublicKeys(signer))
	}

	slog.Info("connect to ssh host", slog.String("host", address))
	client, err := ssh.Dial("tcp", address, &config)
	if err != nil {
		return err
	}
	defer client.Close()
	session, err := client.NewSession()
	if err != nil {
		log.Fatal("Failed to create session: ", err)
	}
	defer session.Close()

	for _, fn := range executions {
		commands, err := fn(p.Environment)
		if err != nil {
			return err
		}
		for _, cmd := range commands {
			slog.Debug("run", slog.String("host", address), slog.String("command", cmd))
			buf, err := session.Output(cmd)
			if err != nil {
				return err
			}
			slog.Info(string(buf), slog.String("host", address))
		}
	}
	return nil
}

func (p *Node) address() string {
	var port uint16 = 22
	if p.Port != nil {
		port = *p.Port
	}
	return fmt.Sprintf("%s:%d", p.Host, port)
}
