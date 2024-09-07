package cmd

import (
	"fmt"
	"log/slog"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/firebush/cmd/job"
	"github.com/saturn-xiv/palm/firebush/cmd/ping"
	"github.com/saturn-xiv/palm/firebush/env"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "firebush",
	Short:   "Firebush",
	Long:    fmt.Sprintf("A suite of software tools that enables infrastructure as code.(%s).", repo_url),
	Version: fmt.Sprintf("%s(%s) by %s<%s>", git_version, build_time, author_name, author_email),
	RunE: func(cmd *cobra.Command, args []string) error {
		return cmd.Help()
	},
}

func Execute() error {
	return root_cmd.Execute()
}

var (
	gl_debug     bool
	gl_inventory string
	gl_subset    string
	gl_jobs      []string
	gl_timeout   uint
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_inventory, "inventory", "i", "staging.toml", "specify inventory host filepath")
	root_cmd.PersistentFlags().StringVarP(&gl_subset, "subset", "l", env.GROUP_ALL, "further limit selected hosts to an additional pattern")
	root_cmd.PersistentFlags().UintVarP(&gl_timeout, "timeout", "T", 20, "the connection timeout in seconds")

	{
		var cmd = &cobra.Command{
			Use:   "ping",
			Short: "Testing ssh connections",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return ping.Launch(gl_inventory, gl_subset, gl_timeout)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "play",
			Short: "Runs playbooks, executing the defined tasks on the targeted hosts",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return job.Launch(gl_inventory, gl_subset, gl_jobs, gl_timeout)
			},
		}

		cmd.Flags().StringArrayVarP(&gl_jobs, "job", "j", []string{}, "playbook")
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
