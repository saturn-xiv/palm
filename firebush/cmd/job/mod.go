package job

import (
	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/firebush/env"
)

func Launch(inventory_file string, subset_name string, jobs []string, timeout uint) error {
	var inventory env.Inventory
	if _, err := toml.DecodeFile(inventory_file, &inventory); err != nil {
		return err
	}
	inventory.Execute(subset_name, func(n *env.Node) error {
		for _, j := range jobs {
			if e := n.Execute(j); e != nil {
				return e
			}
		}
		return nil
	})
	return nil
}
