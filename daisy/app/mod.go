package app

import (
	"fmt"
	"log"
	"log/slog"

	"github.com/spf13/cobra"
)

var (
	git_version string
	build_time  string

	gl_config_file      string
	gl_debug            bool
	gl_rpc_port         uint16
	gl_http_port        uint16
	gl_sms_send_queue   string
	gl_email_send_queue string

	gl_root_cmd = &cobra.Command{
		Use:     "daisy",
		Short:   "A total free education & translation solution",
		Version: fmt.Sprintf("%s(%s)", git_version, build_time),
	}

	gl_rpc_cmd = &cobra.Command{
		Use:   "rpc",
		Short: "Start a gRPC server",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchRpcServer(gl_config_file, gl_rpc_port, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_http_cmd = &cobra.Command{
		Use:   "http",
		Short: "Start a HTTP server",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchHttpServer(gl_config_file, gl_http_port, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_sms_send_cmd = &cobra.Command{
		Use:   "sms-send-worker",
		Short: "Start a sms-send worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchSmsSendWorker(gl_config_file, gl_sms_send_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_email_send_cmd = &cobra.Command{
		Use:   "email-send",
		Short: "Start a email-send worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchEmailSendWorker(gl_config_file, gl_email_send_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
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
	gl_rpc_cmd.PersistentFlags().Uint16VarP(&gl_rpc_port, "port", "p", 8080, "listening port")
	gl_email_send_cmd.PersistentFlags().StringVarP(&gl_email_send_queue, "queue", "q", "emails", "queue name")
	gl_sms_send_cmd.PersistentFlags().StringVarP(&gl_sms_send_queue, "queue", "q", "sms", "queue name")

	gl_root_cmd.AddCommand(gl_http_cmd, gl_rpc_cmd, gl_email_send_cmd, gl_sms_send_cmd)
}

func init_logger(debug bool) {
	if debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}
	slog.Debug("run on debug mode")
}
