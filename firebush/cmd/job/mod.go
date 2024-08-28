package job

import "github.com/saturn-xiv/palm/firebush/env"

func Launch(inventory_name string, subset_name string, jobs []string, timeout uint) error {
	inventory, err := env.LoadInventory(inventory_name)
	if err != nil {
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
