package cmd

import (
	"fmt"

	"github.com/gin-gonic/gin"
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
	Use:     "petunia",
	Short:   "Petunia",
	Long:    fmt.Sprintf("A email worker by Go.(%s).", repo_url),
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
	gl_queue    string
	gl_web_port int
	gl_rpc_port int
	gl_app_id   string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")

	{
		var cmd = &cobra.Command{
			Use:   "twilio-callback-server",
			Short: "Start a Twilio callback server(HTTP)",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
					gin.SetMode(gin.ReleaseMode)
				}
				log.Debugf("run on debug mode")

				if err := launch_twilio_callback(gl_web_port); err != nil {
					log.Fatalf("start HTTP server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_web_port, "port", "p", 8080, "listen port")
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "send-sms-consumer",
			Short: "Start a send-sms consumer",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_send_sms_consumer(gl_queue, gl_config); err != nil {
					log.Fatalf("start send-sms consumer: %s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_queue, "queue", "q", "send-sms", "queue name")
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "send-email-consumer",
			Short: "Start a send-email consumer",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_send_email_consumer(gl_queue, gl_config); err != nil {
					log.Fatalf("start send-email consumer: %s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_queue, "queue", "q", "send-email", "queue name")
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "s3-rpc-server",
			Short: "Start a s3 gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_s3_rpc_server(gl_rpc_port, gl_config); err != nil {
					log.Fatalf("start s3 gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "s3-gc-worker",
			Short: "Start a s3 garbage-collection worker",
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
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "crypto-rpc-server",
			Short: "Start a crypto gRPC server(based on Google-Tink)",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_crypto_rpc_server(gl_rpc_port, gl_app_id); err != nil {
					log.Fatalf("start tink gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		cmd.Flags().StringVarP(&gl_app_id, "app-id", "I", "testing", "application id")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "rbac-rpc-server",
			Short: "Start a rbac gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_rbac_rpc_server(gl_rpc_port, gl_config); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		cmd.Flags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")
		root_cmd.AddCommand(cmd)
	}

}
