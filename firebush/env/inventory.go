package env

import (
	"fmt"
	"log/slog"
	"path/filepath"
	"sync"

	"gopkg.in/ini.v1"
)

type Inventory struct {
	Groups      []*Group
	Nodes       []*Node
	Environment map[string]interface{}
}

func LoadInventory(folder string) (*Inventory, error) {
	file := filepath.Join(folder, "config.ini")
	slog.Info("load inventory", slog.String("file", file))
	item := Inventory{
		Groups:      []*Group{},
		Nodes:       []*Node{},
		Environment: map[string]interface{}{},
	}
	cfg, err := ini.LoadSources(ini.LoadOptions{
		IgnoreInlineComment:      true,
		SpaceBeforeInlineComment: true,
	}, file)
	if err != nil {
		return nil, err
	}
	load_env(cfg.Section(""), item.Environment)

	for _, it := range cfg.Sections() {
		name := it.Name()
		slog.Debug("found section", slog.String("name", name))
		// FIXME
	}
	// {
	// 	file := filepath.Join(name, "vars.ini")
	// 	slog.Debug("load global environment", slog.String("file", file))
	// 	cfg, err := ini.Load(file)
	// 	if err != nil {
	// 		return nil, err
	// 	}
	// 	load_env(cfg.Section(""), item.Environment)
	// }
	// {
	// 	root := filepath.Join(name, "groups")
	// 	files, err := os.ReadDir(root)
	// 	if err != nil {
	// 		return nil, err
	// 	}
	// 	for _, it := range files {
	// 		file := it.Name()
	// 		if !it.Type().IsRegular() {
	// 			return nil, fmt.Errorf("bad group file %s", file)
	// 		}
	// 		name = strings.TrimSuffix(file, filepath.Ext(file))
	// 		slog.Debug("found group", slog.String("name", name))
	// 		cfg, err := ini.Load(filepath.Join(root, name))
	// 		if err != nil {
	// 			return nil, err
	// 		}
	// 		group, err := NewGroup(name, cfg)
	// 		if err != nil {
	// 			return nil, err
	// 		}
	// 		item.Groups = append(item.Groups, group)
	// 	}
	// }
	// {
	// 	root := filepath.Join(name, "hosts")
	// 	files, err := os.ReadDir(root)
	// 	if err != nil {
	// 		return nil, err
	// 	}
	// 	for _, it := range files {
	// 		file := it.Name()
	// 		if !it.Type().IsRegular() {
	// 			return nil, fmt.Errorf("bad host file %s", file)
	// 		}
	// 		name = strings.TrimSuffix(file, filepath.Ext(file))
	// 		slog.Debug("found host", slog.String("name", name))
	// 		cfg, err := ini.Load(filepath.Join(root, name))
	// 		if err != nil {
	// 			return nil, err
	// 		}
	// 		node, err := NewNode(name, cfg)
	// 		if err != nil {
	// 			return nil, err
	// 		}
	// 		item.Nodes = append(item.Nodes, node)
	// 	}
	// }
	return &item, nil
}

func (p *Inventory) subset(name string) []*Node {
	items := []*Node{}
	// TODO all
	// TODO by group
	// TODO by host
	return items
}

func (p *Inventory) Execute(subset string, handler func(*Node) error) {
	reports := map[string]string{}
	nodes := p.subset(subset)
	{
		var wg sync.WaitGroup
		for _, node := range nodes {
			wg.Add(1)
			go func(n *Node, r map[string]string) {
				defer wg.Done()
				if e := handler(n); e != nil {
					r[n.Host] = e.Error()
				}
			}(node, reports)
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
