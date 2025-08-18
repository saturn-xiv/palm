package cmd

import (
	"fmt"
	"log"
	"log/slog"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/jasmine/cmd/db"
	"github.com/saturn-xiv/palm/jasmine/cmd/etc"
	"github.com/saturn-xiv/palm/jasmine/cmd/http"
	"github.com/saturn-xiv/palm/jasmine/cmd/rpc"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "jasmine",
	Short:   "Jasmine",
	Long:    fmt.Sprintf("A web portal application(%s).", repo_url),
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

	gl_rpc_port  uint16
	gl_web_port  uint16
	gl_web_theme string

	gl_etc_domain string
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
				if err := rpc.Launch(gl_rpc_port, gl_config, git_version); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}

		cmd.Flags().Uint16VarP(&gl_rpc_port, "port", "p", 9999, "port to listen")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "http",
			Short: "Start a HTTP server",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := http.Launch(gl_web_port, gl_config, gl_web_theme, git_version); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}

		cmd.Flags().Uint16VarP(&gl_web_port, "port", "p", 8080, "port to listen")
		cmd.Flags().StringVarP(&gl_web_theme, "theme", "t", "bootstrap", "website's theme")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "db-migrate",
			Short: "Migrate database to the latest version",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := db.Migrate(gl_config); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "db-status",
			Short: "List applied and pending migrations",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := db.Status(gl_config); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "db-rollback",
			Short: "Rollback the most recent migration",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := db.Rollback(gl_config); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "etc",
			Short: "Generate systemd & nginx configuration file",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)

				if err := etc.RpcSystemdConf(gl_etc_domain, gl_rpc_port); err != nil {
					log.Fatalf("%v", err)
				}
				if err := etc.WwwSystemdConf(gl_etc_domain, gl_web_port); err != nil {
					log.Fatalf("%v", err)
				}
				if err := etc.MinioSystemdConf(gl_etc_domain); err != nil {
					log.Fatalf("%v", err)
				}
				if err := etc.MinioNginxConf(gl_etc_domain); err != nil {
					log.Fatalf("%v", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_etc_domain, "domain", "D", "change-me.org", "domain name")
		cmd.Flags().Uint16Var(&gl_rpc_port, "rpc-port", 9999, "gRPC server port")
		cmd.Flags().Uint16Var(&gl_web_port, "web-port", 8080, "http server port")
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
