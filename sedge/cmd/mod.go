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
	Use:     "sedge",
	Short:   "Sedge",
	Long:    fmt.Sprintf("A lightweight, framework-agnostic database migration tool.(%s).", repo_url),
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
	gl_debug bool
	gl_url   string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_url, "url", "u", "db", "specify the database URL")

	{
		var cmd = &cobra.Command{
			Use:   "migrate",
			Short: "Migrate to the latest version",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "rollback",
			Short: "Rollback the most recent migration",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "status",
			Short: "List applied and pending migrations",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "dump",
			Short: "Write the database schema to disk",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "load",
			Short: "Load schema file to the database",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "create",
			Short: "Create database",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "drop",
			Short: "Drop database",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "new",
			Short: "Generate a new migration file",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				// TODO
			},
		}

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
