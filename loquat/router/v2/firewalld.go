package v2

import (
	_ "embed"
	"fmt"
	"io"
	"log/slog"
)

//go:embed templates/firewalld-header.txt
var gl_firewalld_header_txt string

func (p *Router) setup_firewalld(wrt io.Writer) error {
	slog.Debug("setup firewalld")
	if _, err := fmt.Fprintf(wrt, "%s", gl_firewalld_header_txt); err != nil {
		return err
	}
	if p.Dmz != nil {
		if err := p.Dmz.firewalld(wrt, "dmz", DMZ); err != nil {
			return err
		}
	}
	if p.Lan != nil {
		if err := p.Lan.firewalld(wrt, "internal", LAN); err != nil {
			return err
		}
	}
	if _, err := fmt.Fprintf(wrt, "firewall-cmd --reload\n"); err != nil {
		return err
	}

	return nil
}

// ----------------------------------------------------------------------------
func (p *FirewallRule_Protocol) ToString() string {
	switch *p {
	case FirewallRule_Tcp:
		return "tcp"
	case FirewallRule_Udp:
		return "udp"
	default:
		return ""
	}
}
