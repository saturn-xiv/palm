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
	Use:     "camelia",
	Short:   "Camelia",
	Long:    fmt.Sprintf("A flexible s3 service by Go.(%s).", repo_url),
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
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "credentials.json", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "rpc",
			Short: "Start a gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_rpc_server(gl_rpc_port, gl_config); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "gc",
			Short: "Start a garbage-collection",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_gc_task(gl_config); err != nil {
					log.Fatalf("gc: %s", err)
				}
			},
		}
		root_cmd.AddCommand(cmd)
	}

}
