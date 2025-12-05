package v2

import (
	_ "embed"
	"fmt"
	"io"
	"log/slog"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"slices"
	"time"
)

//go:embed templates/header.txt
var gl_header_txt string

//go:embed templates/footer.txt
var gl_footer_txt string

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

func (p *Router) render_to_file(name string) error {
	slog.Info("generate shell script", "file", name)
	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()
	return p.Render(file)
}

func (p *Router) VerifyInterface() error {
	if err := p.check_wan(); err != nil {
		return err
	}
	if err := p.check_dmz(); err != nil {
		return err
	}
	if err := p.check_lan(); err != nil {
		return err
	}
	return nil
}

func (p *Router) check_wan() error {
	if p.Wan != nil {
		for name := range p.Wan.Interfaces {
			if _, err := net.InterfaceByName(name); err != nil {
				return err
			}
			if p.Dmz != nil {
				if slices.Contains(p.Dmz.Interfaces, name) {
					return fmt.Errorf("%s is used in wan & dmz", name)
				}
			}
			if p.Lan != nil {
				if slices.Contains(p.Lan.Interfaces, name) {
					return fmt.Errorf("%s is used in wan & lan", name)
				}
			}
		}
	}
	return nil
}

func (p *Router) check_dmz() error {
	if p.Dmz != nil {
		for _, name := range p.Dmz.Interfaces {
			if _, err := net.InterfaceByName(name); err != nil {
				return err
			}
			if p.Wan != nil {
				if _, ok := p.Wan.Interfaces[name]; ok {
					return fmt.Errorf("%s is used in dmz & wan", name)
				}
			}
			if p.Lan != nil {
				if slices.Contains(p.Lan.Interfaces, name) {
					return fmt.Errorf("%s is used in dmz & lan", name)
				}
			}
		}
	}
	return nil
}

func (p *Router) check_lan() error {
	if p.Lan != nil {
		for _, name := range p.Lan.Interfaces {
			if _, err := net.InterfaceByName(name); err != nil {
				return err
			}
			if p.Wan != nil {
				if _, ok := p.Wan.Interfaces[name]; ok {
					return fmt.Errorf("%s is used in lan & wan", name)
				}
			}
			if p.Dmz != nil {
				if slices.Contains(p.Dmz.Interfaces, name) {
					return fmt.Errorf("%s is used in lan & dmz", name)
				}
			}
		}
	}
	return nil
}
