package app

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
	git_version string
	build_time  string

	gl_config_file string
	gl_debug       bool
	gl_http_port   uint16

	gl_root_cmd = &cobra.Command{
		Use:     "loquat",
		Short:   "A smart router based on Debian linux",
		Version: fmt.Sprintf("%s(%s)", git_version, build_time),
	}

	gl_http_cmd = &cobra.Command{
		Use:   "http",
		Short: "Start a HTTP server",
		RunE: func(cmd *cobra.Command, args []string) error {
			return LaunchHttpServer(gl_config_file, gl_http_port, gl_debug)
		},
	}
	gl_net_scan_cmd = &cobra.Command{
		Use:   "net-scan",
		Short: "Scan network devices",
		RunE: func(cmd *cobra.Command, args []string) error {
			return NetScan(gl_config_file, gl_debug)
		},
	}
)

func Execute() error {
	return gl_root_cmd.Execute()
}

func init() {

	gl_root_cmd.PersistentFlags().StringVarP(&gl_config_file, "config", "c", "config.toml", "configuration file")
	gl_root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", true, "run on debug mode")

	gl_http_cmd.PersistentFlags().Uint16VarP(&gl_http_port, "port", "p", 8080, "listening port")

	gl_root_cmd.AddCommand(gl_http_cmd, gl_net_scan_cmd)
}
