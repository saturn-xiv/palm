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
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "twilio-callback",
			Short: "Start a Twilio callback server(HTTP)",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
					gin.SetMode(gin.ReleaseMode)
				}
				log.Debugf("run on debug mode")

				if err := launch_web_server(gl_web_port, gl_config); err != nil {
					log.Fatalf("start HTTP server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_web_port, "port", "p", 8080, "listen port")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "send-sms",
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
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "send-email",
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
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "s3-rpc",
			Short: "Start a s3 gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				if gl_debug {
					log.SetLevel(log.DebugLevel)
				} else {
					log.SetLevel(log.InfoLevel)
				}
				log.Debugf("run on debug mode")

				if err := launch_rpc_server(gl_rpc_port, gl_config); err != nil {
					log.Fatalf("start s3 gRPC server: %s", err)
				}
			},
		}
		cmd.Flags().IntVarP(&gl_rpc_port, "port", "p", 9999, "listen port")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "s3-gc",
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
		root_cmd.AddCommand(cmd)
	}

}
