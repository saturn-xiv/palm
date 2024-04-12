package cmd

import (
	"fmt"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/lilac/cmd/rpc"
	"github.com/saturn-xiv/palm/lilac/cmd/web"
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
	Long:    fmt.Sprintf("A collection of user services based on Casbin & Tink.(%s).", repo_url),
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
	gl_debug     bool
	gl_config    string
	gl_rpc_port  int
	gl_http_port int
	gl_keys_dir  string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

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

				if err := rpc.Launch(gl_rpc_port, gl_config, gl_keys_dir); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		cmd.Flags().StringVarP(&gl_keys_dir, "keys-dir", "k", "staging", "load keys from")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "web",
			Short: "Start a HTTP server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
					gin.SetMode(gin.ReleaseMode)
				}
				log.Debugf("run on debug mode")

				if err := web.Launch(gl_http_port, gl_config, gl_keys_dir); err != nil {
					log.Fatalf("start HTTP server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_http_port, "port", "p", 8080, "listen port")
		cmd.Flags().StringVarP(&gl_keys_dir, "keys-dir", "k", "staging", "load keys from")
		root_cmd.AddCommand(cmd)
	}

}
