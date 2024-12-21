package cmd

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"

	"github.com/BurntSushi/toml"
	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/atropa/cmd/rpc"
	"github.com/saturn-xiv/palm/atropa/cmd/web"
	"github.com/saturn-xiv/palm/atropa/cmd/workers"
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
	Long:    fmt.Sprintf("A collection of gRpc services & controllers.(%s).", repo_url),
	Version: fmt.Sprintf("%s(%s) by %s<%s>", git_version, build_time, author_name, author_email),
	RunE: func(cmd *cobra.Command, args []string) error {
		return cmd.Help()
	},
}

func Execute() error {
	return root_cmd.Execute()
}

var (
	gl_debug  bool
	gl_config string

	gl_rpc_port uint16
	gl_web_port uint16

	gl_etc_domain string

	gl_worker_consumer_name string
	gl_worker_queue_name    string
	gl_worker_task_name     string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "web",
			Short: "Start a HTTP server",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return web.Launch(gl_web_port, gl_config, git_version, gl_debug)
			},
		}

		cmd.Flags().Uint16VarP(&gl_web_port, "port", "p", 8080, "port to listen")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "rpc",
			Short: "Start a gRPC server",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return rpc.Launch(gl_web_port, gl_config, git_version)
			},
		}

		cmd.Flags().Uint16VarP(&gl_web_port, "port", "p", 8080, "port to listen")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "worker",
			Short: "Start a queue consumer",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				slog.Debug(fmt.Sprintf("load configuration from %s", gl_config))
				ctx := context.Background()
				switch gl_worker_task_name {
				case workers.EmailSendTask:
					var cfg workers.EmailSend
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case workers.SmsSendTask:
					var cfg workers.SmsSend
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case workers.PandocTask:
					var cfg workers.Pandoc
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case workers.TexliveTask:
					var cfg workers.Texlive
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				default:
					return fmt.Errorf("unsupported task %s", gl_worker_task_name)
				}

			},
		}
		hostname, _ := os.Hostname()

		cmd.Flags().StringVar(&gl_worker_consumer_name, "consumer", fmt.Sprintf("%s-%d", hostname, os.Getpid()), "consumer name")
		cmd.Flags().StringVar(&gl_worker_queue_name, "queue", "my-queue", "queue name")
		cmd.Flags().StringVar(&gl_worker_task_name, "task", "email-send", fmt.Sprintf("task name(%s)", strings.Join(
			[]string{workers.EmailSendTask, workers.SmsSendTask, workers.PandocTask, workers.TexliveTask},
			",")))
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
