package proxy

import "log/slog"

type Ssh struct {
	Host    string
	Port    uint16
	User    string
	KeyFile string
}

func (p *Ssh) StartHttpProxyServer(host string, port uint16) error {
	slog.Debug("connect to", "host", p.Host, "port", p.Port, "user", p.User, "key-file", p.KeyFile)
	slog.Info("listening at", "host", host, "port", port)
	return nil
}
