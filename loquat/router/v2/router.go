package v2

import (
	_ "embed"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"time"
)

//go:embed templates/header.txt
var gl_header_txt string

//go:embed templates/footer.txt
var gl_footer_txt string

func (p *Router) Render(wrt io.Writer) error {
	if _, err := fmt.Fprintf(wrt, "%s", gl_header_txt); err != nil {
		return err
	}
	if err := p.setup_netplan(wrt); err != nil {
		return err
	}
	if p.Dmz != nil {
		p.Dmz.Network.dnsmasq(wrt, DMZ)
	}
	if p.Lan != nil {
		p.Lan.Network.dnsmasq(wrt, LAN)
	}
	if err := p.setup_firewalld(wrt); err != nil {
		return err
	}
	_, err := fmt.Fprintf(wrt, "%s", gl_footer_txt)
	return err
}

func (p *Router) setup_firewalld(wrt io.Writer) error {
	slog.Debug("setup nftables")
	// TODO
	return nil
}

func (p *Router) Apply(run bool) error {
	tmp := filepath.Join(os.TempDir(), fmt.Sprintf("%s.sh", time.Now().Format("20060102150405")))
	if err := p.render_to_file(tmp); err != nil {
		return err
	}
	if run {
		cmd := exec.Command("bash", tmp)
		return cmd.Run()
	}
	return nil
}

func (p *Router) render_to_file(name string) error {
	slog.Info("generate shell script", "file", name)
	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()
	return p.Render(file)
}
