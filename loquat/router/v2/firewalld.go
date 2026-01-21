package v2

import (
	_ "embed"
	"io"
	"log/slog"
	"maps"
	"slices"
	"text/template"
)

//go:embed templates/firewalld-header.txt
var gl_firewalld_header_txt string

//go:embed templates/firewalld-footer.txt
var gl_firewalld_footer_txt string

//go:embed templates/firewalld-external.txt
var gl_firewalld_external_txt string

func (p *Router) setup_firewalld(wrt io.Writer) error {
	slog.Debug("setup firewalld")

	if _, err := io.WriteString(wrt, gl_firewalld_header_txt); err != nil {
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
	if p.Wan != nil {
		tpl, err := template.New("").Parse(gl_firewalld_external_txt)
		if err != nil {
			return err
		}
		if err := tpl.Execute(wrt, map[string]interface{}{"items": slices.Collect(maps.Keys(p.Wan))}); err != nil {
			return err
		}
	}
	if _, err := io.WriteString(wrt, gl_firewalld_footer_txt); err != nil {
		return err
	}

	return nil
}

// ----------------------------------------------------------------------------
func (p *FirewallRule_Protocol) ToString() string {
	switch *p {
	case FirewallRule_TCP:
		return "tcp"
	case FirewallRule_UDP:
		return "udp"
	default:
		return ""
	}
}
