package app

import (
	"github.com/spf13/cobra"
)

var (
	gl_config_file      string
	gl_debug            bool
	gl_rpc_port         uint16
	gl_http_port        uint16
	gl_sms_send_queue   string
	gl_email_send_queue string

	gl_root_cmd = &cobra.Command{
		Use:     "daisy",
		Short:   "A total free education &amp; translation solution",
		Version: "2025.11.6",
	}

	gl_rpc_cmd = &cobra.Command{
		Use:   "rpc",
		Short: "Start a gRPC server",
		RunE: func(cmd *cobra.Command, args []string) error {
			// TODO
			return nil
		},
	}
	gl_http_cmd = &cobra.Command{
		Use:   "http",
		Short: "Start a HTTP server",
		RunE: func(cmd *cobra.Command, args []string) error {
			// TODO
			return nil
		},
	}
	gl_sms_send_cmd = &cobra.Command{
		Use:   "sms-send-worker",
		Short: "Start a sms-send worker",
		RunE: func(cmd *cobra.Command, args []string) error {
			// TODO
			return nil
		},
	}
	gl_email_send_cmd = &cobra.Command{
		Use:   "email-send",
		Short: "Start a email-send worker",
		RunE: func(cmd *cobra.Command, args []string) error {
			// TODO
			return nil
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
