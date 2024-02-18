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
	Use:     "cactus",
	Short:   "Cactus",
	Long:    fmt.Sprintf("A sms worker by Go.(%s).", repo_url),
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
	gl_debug            bool
	gl_config           string
	gl_send_email_queue string
	gl_web_port         int
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "web",
			Short: "Start a HTTP server",
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

				if err := launch_send_sms_consumer(gl_send_email_queue, gl_config); err != nil {
					log.Fatalf("start send-sms consumer: %s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_send_email_queue, "queue", "q", "send-sms", "queue name")
		root_cmd.AddCommand(cmd)
	}

}
