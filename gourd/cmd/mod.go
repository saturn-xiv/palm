package cmd

import (
	"fmt"
	"log"
	"log/slog"

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
	Use:     "gourd",
	Short:   "Gourd",
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
	gl_debug              bool
	gl_config             string
	gl_rpc_listen_address string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "rpc",
			Short: "Start a gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)

				if err := LaunchRpcServer(gl_rpc_listen_address, gl_config, git_version); err != nil {
					log.Fatalf("start rpc server: %s", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_rpc_listen_address, "address", "A", "127.0.0.1:9999", "network address to listen")
		root_cmd.AddCommand(cmd)
	}

}

func set_log(debug bool) {

	if debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}

	slog.Debug("run on debug mode")
}
