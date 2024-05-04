package cmd

import (
	"fmt"
	"log"
	"log/slog"

	"github.com/spf13/cobra"

	send_sms_worker "github.com/saturn-xiv/palm/tuberose/cmd/send-sms-worker"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "daisy",
	Short:   "Daisy",
	Long:    fmt.Sprintf("A smtp worker(%s).", repo_url),
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

	gl_send_sms_worker_queue_name    string
	gl_send_sms_worker_consumer_name string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "send-email-worker",
			Short: "Start a smtp consumer",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := send_sms_worker.Launch(gl_config, gl_send_sms_worker_consumer_name, gl_send_sms_worker_queue_name); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_send_sms_worker_queue_name, "queue", "q", "sms", "queue name")
		cmd.Flags().StringVarP(&gl_send_sms_worker_consumer_name, "name", "n", "send-sms", "consumer name")
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
