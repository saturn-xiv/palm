package app

import (
	"fmt"
	"log/slog"
	"net/http"

	"github.com/saturn-xiv/palm/pansy/proxy"
)

type Ssh struct {
	Host    string
	Port    uint16
	User    string
	KeyFile string
}

func (p *Ssh) StartHttpProxyServer(host string, port uint16) error {
	server, err := proxy.NewServer(p.Host, p.Port, p.User, p.KeyFile)
	if err != nil {
		return err
	}
	addr := fmt.Sprintf("%s:%d", host, port)
	slog.Info("start a proxy server at", "address", addr)
	return http.ListenAndServe(addr, server)
}
