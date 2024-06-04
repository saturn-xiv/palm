package cmd

import (
	"fmt"
	"log"
	"log/slog"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/atropa/cmd/rpc"
	"github.com/saturn-xiv/palm/atropa/cmd/service"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "atropa",
	Short:   "Atropa",
	Long:    fmt.Sprintf("A collection of rpc services by Go.(%s).", repo_url),
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
	gl_debug  bool
	gl_config string

	gl_rpc_port      uint16
	gl_rpc_ssl       bool
	gl_rpc_ca_file   string
	gl_rpc_key_file  string
	gl_rpc_cert_file string

	gl_service_name string
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

				var err error
				if gl_rpc_ssl {
					err = rpc.Launch(gl_rpc_port, gl_config, git_version, &rpc.Ssl{
						CaFile:   gl_rpc_ca_file,
						KeyFile:  gl_rpc_key_file,
						CertFile: gl_rpc_cert_file,
					})

				} else {
					err = rpc.Launch(gl_rpc_port, gl_config, git_version, nil)
				}

				if err != nil {
					log.Fatalf("%v", err)
				}
			},
		}

		cmd.Flags().Uint16VarP(&gl_rpc_port, "port", "p", 9999, "port to listen")
		cmd.Flags().BoolVarP(&gl_rpc_ssl, "ssl", "s", false, "enable ssl")
		cmd.Flags().StringVar(&gl_rpc_cert_file, "cert-file", "server.crt", "cert file")
		cmd.Flags().StringVar(&gl_rpc_key_file, "key-file", "server.key", "key file")
		cmd.Flags().StringVar(&gl_rpc_ca_file, "ca-file", "ca.crt", "ca file")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "systemd",
			Short: "Generate a systemd service file",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)

				if err := service.SystemdConf(gl_service_name, gl_rpc_port); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_service_name, "name", "n", "atropa", "service name")
		cmd.Flags().Uint16VarP(&gl_rpc_port, "port", "p", 9999, "port to listen")
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
