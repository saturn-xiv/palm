package cmd

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"

	"github.com/BurntSushi/toml"
	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/atropa/cmd/etc"
	generate_token "github.com/saturn-xiv/palm/atropa/cmd/generate-token"
	queue_worker "github.com/saturn-xiv/palm/atropa/cmd/queue-worker"
	"github.com/saturn-xiv/palm/atropa/cmd/rpc"
	"github.com/saturn-xiv/palm/atropa/cmd/web"
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
	Long:    fmt.Sprintf("A total free education & translation solution.(%s).", repo_url),
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

	gl_generate_token_years     uint8
	gl_generate_token_subject   string
	gl_generate_token_audiences []string
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
			Use:   "etc",
			Short: "Generate systemd & nginx configuration file",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)

				if err := etc.RpcSystemdConf(gl_etc_domain, gl_rpc_port); err != nil {
					return err
				}
				if err := etc.WwwSystemdConf(gl_etc_domain, gl_web_port); err != nil {
					return err
				}
				if err := etc.SmsSendWorkerSystemdConf(gl_etc_domain); err != nil {
					return err
				}
				if err := etc.EmailSendWorkerSystemdConf(gl_etc_domain); err != nil {
					return err
				}
				if err := etc.MinioSystemdConf(gl_etc_domain); err != nil {
					return err
				}
				if err := etc.MinioNginxConf(gl_etc_domain); err != nil {
					return err
				}
				return nil
			},
		}

		cmd.Flags().StringVarP(&gl_etc_domain, "domain", "D", "change-me.org", "domain name")
		cmd.Flags().Uint16Var(&gl_rpc_port, "rpc-port", 9999, "gRPC server port")
		cmd.Flags().Uint16Var(&gl_web_port, "web-port", 8080, "http server port")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "queue-consumer",
			Short: "Start a queue consumer",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				slog.Debug(fmt.Sprintf("load configuration from %s", gl_config))
				ctx := context.Background()
				switch gl_worker_task_name {
				case queue_worker.EmailSendTask:
					var cfg queue_worker.EmailSend
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case queue_worker.SmsSendTask:
					var cfg queue_worker.SmsSend
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case queue_worker.PandocTask:
					var cfg queue_worker.Pandoc
					if _, err := toml.DecodeFile(gl_config, &cfg); err != nil {
						return err
					}
					return cfg.Execute(ctx, gl_worker_consumer_name, gl_worker_queue_name)
				case queue_worker.TexliveTask:
					var cfg queue_worker.Texlive
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
			[]string{queue_worker.EmailSendTask, queue_worker.SmsSendTask, queue_worker.PandocTask, queue_worker.TexliveTask},
			",")))
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "generate-token",
			Short: "Generate a jwt token",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return generate_token.Launch(gl_config, gl_generate_token_subject, gl_generate_token_audiences, int(gl_generate_token_years))
			},
		}

		cmd.Flags().StringVar(&gl_generate_token_subject, "subject", "", "subject")
		cmd.Flags().Uint8Var(&gl_generate_token_years, "years", 10, "years")
		cmd.Flags().StringSliceVar(&gl_generate_token_audiences, "audience", []string{}, "audiences")

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
