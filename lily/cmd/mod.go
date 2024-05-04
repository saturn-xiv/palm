package cmd

import (
	"fmt"
	"log"
	"log/slog"

	tex_to_pdf "github.com/saturn-xiv/palm/lily/cmd/tex-to-pdf"
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
	Use:     "lily",
	Short:   "Lily",
	Long:    fmt.Sprintf("A collection of texlive services(%s).", repo_url),
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

	gl_tex_to_pdf_worker_queue_name    string
	gl_tex_to_pdf_worker_consumer_name string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_config, "config", "c", "config.toml", "load configuration file")

	{
		var cmd = &cobra.Command{
			Use:   "tex-to-pdf-worker",
			Short: "Start a tex-to-pdf consumer",
			Run: func(cmd *cobra.Command, args []string) {
				set_log(gl_debug)
				if err := tex_to_pdf.Launch(gl_config, gl_tex_to_pdf_worker_consumer_name, gl_tex_to_pdf_worker_queue_name); err != nil {
					log.Fatalf("start a tex-to-pdf worker: %s", err)
				}
			},
		}

		cmd.Flags().StringVarP(&gl_tex_to_pdf_worker_queue_name, "queue", "q", "tex2pdf", "queue name")
		cmd.Flags().StringVarP(&gl_tex_to_pdf_worker_consumer_name, "name", "n", "build-tex-to-pdf", "consumer name")
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
