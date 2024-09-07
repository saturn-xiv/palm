package ping

import (
	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/firebush/env"
)

func Launch(inventory_file string, subset_name string, timeout uint) error {
	var inventory env.Inventory
	if _, err := toml.DecodeFile(inventory_file, &inventory); err != nil {
		return err
	}

	inventory.Execute(subset_name, func(n *env.Node) error {
		return n.Ping()
	})
	return nil
}
