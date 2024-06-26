package cmd

import (
	"fmt"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "lilac",
	Short:   "Lilac",
	Long:    fmt.Sprintf("A rbac service based on Casbin.(%s).", repo_url),
	Version: fmt.Sprintf("%s(%s) by %s<%s>", git_version, build_time, author_name, author_email),
	Run: func(cmd *cobra.Command, args []string) {
		if err := cmd.Help(); err != nil {
			log.Fatal(err)
		}
	},
}

func Execute() {
	if err := root_cmd.Execute(); err != nil {
		log.Fatal(err)
	}
}

var (
	gl_debug    bool
	gl_config   string
	gl_rpc_port int
	gl_keys_dir string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "rpc",
			Short: "Start a rbac gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_rpc_server(gl_rpc_port, gl_config, gl_keys_dir); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		cmd.Flags().StringVarP(&gl_keys_dir, "keys-dir", "k", "staging", "load keys from")
		root_cmd.AddCommand(cmd)
	}

}
