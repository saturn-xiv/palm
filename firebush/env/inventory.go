package env

import (
	"fmt"
	"io"
	"log/slog"
	"slices"
	"sync"
)

type Inventory struct {
	Groups      map[string]Environment `toml:"groups"`
	Hosts       map[string]Environment `toml:"hosts"`
	Environment Environment            `toml:"env"`
}

func (p *Inventory) Display(w io.Writer) {
	fmt.Fprintln(w, "============ INVENTORY ============")
	for n, g := range p.Groups {
		fmt.Fprintf(w, "------ GROUP %s ------\n", n)
		g.Display(w)
	}
	for n, h := range p.Hosts {
		fmt.Fprintf(w, "------ HOST %s ------\n", n)
		h.Display(w)
	}
	{
		fmt.Fprintln(w, "------ GLOBAL ENV -------")
		p.Environment.Display(w)
	}
	fmt.Fprintln(w, "====================================")
}

func (p *Inventory) subset(name string) []*Node {
	items := map[string]Environment{}
	if name == GROUP_ALL {
		for group := range p.Groups {
			hosts := p.by_group(group)
			for k, v := range hosts {
				items[k] = v
			}
		}
		for host := range p.Hosts {
			items[host] = p.by_host(host)
		}
	}
	if _, ok := p.Groups[name]; ok {
		for k, v := range p.by_group(name) {
			items[k] = v
		}
	}
	if _, ok := p.Hosts[name]; ok {
		items[name] = p.by_host(name)
	}

	nodes := []*Node{}
	for k, v := range items {
		nodes = append(nodes, v.Node(k))
	}
	return nodes
}

func (p *Inventory) by_host(name string) Environment {
	item := Environment{}
	for k, v := range p.Environment {
		item[k] = v
	}

	for _, group := range p.Groups {
		if hosts, ok := group[GROUP_KEY_HOSTS]; ok {
			if hosts, ok := hosts.([]string); ok {
				if slices.Contains(hosts, name) {
					for k, v := range group {
						if k != GROUP_KEY_HOSTS {
							item[k] = v
						}
					}
				}
			}
		}
	}

	if host, ok := p.Hosts[name]; ok {
		for k, v := range host {
			item[k] = v
		}
	}
	return item
}

func (p *Inventory) by_group(name string) map[string]Environment {
	items := map[string]Environment{}
	if group, ok := p.Groups[name]; ok {
		if hosts, ok := group[GROUP_KEY_HOSTS]; ok {
			if hosts, ok := hosts.([]string); ok {
				for _, host := range hosts {
					if _, ok := items[host]; ok {
						slog.Warn("host already exists", slog.String("group", name), slog.String("name", host))
					}
					items[host] = p.by_host(host)
				}
			}
		}
	}
	return items
}

func (p *Inventory) Execute(subset string, handler func(*Node) error) {
	reports := map[string]string{}
	nodes := p.subset(subset)
	{
		var wg sync.WaitGroup
		for _, node := range nodes {
			wg.Add(1)
			go func(n *Node) {
				defer wg.Done()
				if er := handler(n); er != nil {
					reports[n.Host] = er.Error()
				}
			}(node)
		}
		wg.Wait()
	}

	for _, node := range nodes {
		msg := "done."
		if e, ok := reports[node.Host]; !ok {
			msg = e
		}
		fmt.Printf("%s\t%s\n", node.Host, msg)
	}
}
