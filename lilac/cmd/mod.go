package cmd

import (
	"fmt"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"

	email_send_consumer "github.com/saturn-xiv/palm/lilac/cmd/email-send-consumer"
	"github.com/saturn-xiv/palm/lilac/cmd/rpc"
	sms_send_consumer "github.com/saturn-xiv/palm/lilac/cmd/sms-send-consumer"
	"github.com/saturn-xiv/palm/lilac/cmd/web"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "lilac",
	Short:   "Lilac",
	Long:    fmt.Sprintf("A collection of user services based on Casbin & Tink.(%s).", repo_url),
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
	gl_debug              bool
	gl_config             string
	gl_rpc_listen_address string
	gl_web_listen_address string
	gl_keys_dir           string

	gl_email_send_queue    string
	gl_email_send_consumer string
	gl_sms_send_queue      string
	gl_sms_send_consumer   string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "rpc",
			Short: "Start a gRPC server",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)

				if err := rpc.Launch(gl_rpc_listen_address, gl_config, gl_keys_dir); err != nil {
					log.Fatalf("start gRPC server: %s", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_rpc_listen_address, "address", "A", "127.0.0.1:9999", "network address to listen")
		cmd.Flags().StringVarP(&gl_keys_dir, "keys-dir", "k", "staging", "folder to load keys")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "web",
			Short: "Start a HTTP server",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if !gl_debug {
					gin.SetMode(gin.ReleaseMode)
				}

				if err := web.Launch(gl_web_listen_address, gl_config, gl_keys_dir); err != nil {
					log.Fatalf("start HTTP server: %s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_web_listen_address, "address", "A", "127.0.0.1:8080", "network address to listen")
		cmd.Flags().StringVarP(&gl_keys_dir, "keys-dir", "k", "staging", "folder to load keys")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "email-send-consumer",
			Short: "Start an email-send consumer",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := email_send_consumer.Launch(gl_email_send_consumer, gl_email_send_queue, gl_config); err != nil {
					log.Fatalf("%s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_email_send_consumer, "consumer", "C", "email-send-consumer", "consumer name")
		cmd.Flags().StringVarP(&gl_email_send_queue, "queue", "q", "email-send", "queue name")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "sms-send-consumer",
			Short: "Start a sms-send consumer",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := sms_send_consumer.Launch(gl_sms_send_consumer, gl_sms_send_queue, gl_config); err != nil {
					log.Fatalf("%s", err)
				}
			},
		}
		cmd.Flags().StringVarP(&gl_sms_send_consumer, "consumer", "C", "sms-send-consumer", "consumer name")
		cmd.Flags().StringVarP(&gl_sms_send_queue, "queue", "q", "sms-send", "queue name")
		root_cmd.AddCommand(cmd)
	}
}

func set_log(debug bool) {
	if debug {
		log.SetLevel(log.DebugLevel)
	} else {
		log.SetLevel(log.InfoLevel)
	}
	log.Debugf("run on debug mode")
}
