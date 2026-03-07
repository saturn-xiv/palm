package app

import (
	"log"
	"log/slog"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/daisy/env"
)

var (
	gl_localhost = "127.0.0.1"

	gl_config_file                            string
	gl_debug                                  bool
	gl_rpc_port                               uint16
	gl_http_port                              uint16
	gl_sms_send_worker_queue                  string
	gl_email_send_worker_queue                string
	gl_cups_worker_queue                      string
	gl_tex_worker_queue                       string
	gl_db_seeds_locales                       []string
	gl_create_user_by_email_email             string
	gl_create_user_by_email_name              string
	gl_create_user_by_email_password          string
	gl_reset_password_for_email_user_email    string
	gl_reset_password_for_email_user_password string
	gl_grant_role_to_user_user                string
	gl_grant_role_to_user_role                string
	gl_revoke_role_from_user_user             string
	gl_revoke_role_from_user_role             string

	gl_root_cmd = &cobra.Command{
		Use:     "daisy",
		Short:   "A portal website platform.",
		Version: env.Version(),
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
	gl_sms_send_worker_cmd = &cobra.Command{
		Use:   "sms-send-worker",
		Short: "Start a sms-send worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchSmsSendWorker(gl_config_file, gl_sms_send_worker_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_tex_worker_cmd = &cobra.Command{
		Use:   "tex-worker",
		Short: "Start a TexLive worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchTexWorker(gl_config_file, gl_tex_worker_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_cups_worker_cmd = &cobra.Command{
		Use:   "cups-worker",
		Short: "Start a Cups worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchCupsWorker(gl_config_file, gl_tex_worker_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_email_send_worker_cmd = &cobra.Command{
		Use:   "email-send",
		Short: "Start a email-send worker",
		Run: func(cmd *cobra.Command, args []string) {
			if err := LaunchEmailSendWorker(gl_config_file, gl_email_send_worker_queue, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_db_seeds_cmd = &cobra.Command{
		Use:   "db-seeds",
		Short: "Load seeds data into database",
		Run: func(cmd *cobra.Command, args []string) {
			if err := DbSeeds(gl_config_file, gl_db_seeds_locales, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_list_users_cmd = &cobra.Command{
		Use:   "list-users",
		Short: "List all users",
		Run: func(cmd *cobra.Command, args []string) {
			if err := ListUsers(gl_config_file, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_create_user_by_email_cmd = &cobra.Command{
		Use:   "create-user-by-email",
		Short: "Create an email account",
		Run: func(cmd *cobra.Command, args []string) {
			if err := CreateUserByEmail(gl_config_file, gl_create_user_by_email_name, gl_create_user_by_email_email, gl_create_user_by_email_password, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_reset_password_for_email_user_cmd = &cobra.Command{
		Use:   "reset-password-for-email-user",
		Short: "Reset password for email user",
		Run: func(cmd *cobra.Command, args []string) {
			if err := ResetPasswordForEmailUser(gl_config_file, gl_reset_password_for_email_user_email, gl_reset_password_for_email_user_password, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_grant_role_to_user_cmd = &cobra.Command{
		Use:   "grant-role-to-user",
		Short: "Grant role to user",
		Run: func(cmd *cobra.Command, args []string) {
			if err := GrantRoleToUser(gl_config_file, gl_grant_role_to_user_user, gl_grant_role_to_user_role, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
	gl_revoke_role_from_user_cmd = &cobra.Command{
		Use:   "revoke-role-from-user",
		Short: "Revoke role from user",
		Run: func(cmd *cobra.Command, args []string) {
			if err := RevokeRoleFromUser(gl_config_file, gl_revoke_role_from_user_user, gl_revoke_role_from_user_role, gl_debug); err != nil {
				log.Fatal(err)
			}
		},
	}
)

func Execute() error {
	return gl_root_cmd.Execute()
}

func init() {
	cobra.OnInitialize(init_logger)

	gl_root_cmd.PersistentFlags().StringVarP(&gl_config_file, "config", "c", "config.toml", "configuration file")
	gl_root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", true, "run on debug mode")

	gl_http_cmd.PersistentFlags().Uint16VarP(&gl_http_port, "port", "p", 8080, "listening port")

	gl_rpc_cmd.PersistentFlags().Uint16VarP(&gl_rpc_port, "port", "p", 8080, "listening port")

	gl_email_send_worker_cmd.PersistentFlags().StringVarP(&gl_email_send_worker_queue, "queue", "q", "emails", "queue name")

	gl_sms_send_worker_cmd.PersistentFlags().StringVarP(&gl_sms_send_worker_queue, "queue", "q", "sms", "queue name")

	gl_tex_worker_cmd.PersistentFlags().StringVarP(&gl_tex_worker_queue, "queue", "q", "sms", "queue name")
	gl_cups_worker_cmd.PersistentFlags().StringVarP(&gl_cups_worker_queue, "queue", "q", "sms", "queue name")

	gl_db_seeds_cmd.PersistentFlags().StringSliceVarP(&gl_db_seeds_locales, "locales", "l", []string{}, "locales folder path")

	gl_create_user_by_email_cmd.PersistentFlags().StringVarP(&gl_create_user_by_email_email, "email", "e", "", "email address")
	gl_create_user_by_email_cmd.PersistentFlags().StringVarP(&gl_create_user_by_email_name, "name", "n", "", "username")
	gl_create_user_by_email_cmd.PersistentFlags().StringVarP(&gl_create_user_by_email_password, "password", "p", "", "login password")

	gl_reset_password_for_email_user_cmd.PersistentFlags().StringVarP(&gl_reset_password_for_email_user_email, "email", "e", "", "email address")
	gl_reset_password_for_email_user_cmd.PersistentFlags().StringVarP(&gl_reset_password_for_email_user_password, "password", "p", "", "login password")

	gl_grant_role_to_user_cmd.PersistentFlags().StringVarP(&gl_grant_role_to_user_user, "user", "u", "", "user's sn")
	gl_grant_role_to_user_cmd.PersistentFlags().StringVarP(&gl_grant_role_to_user_role, "role", "r", "", "role's code")

	gl_revoke_role_from_user_cmd.PersistentFlags().StringVarP(&gl_revoke_role_from_user_user, "user", "u", "", "user's sn")
	gl_revoke_role_from_user_cmd.PersistentFlags().StringVarP(&gl_revoke_role_from_user_role, "role", "r", "", "role's code")

	gl_root_cmd.AddCommand(
		gl_http_cmd, gl_rpc_cmd,
		gl_email_send_worker_cmd, gl_sms_send_worker_cmd, gl_tex_worker_cmd, gl_cups_worker_cmd,
		gl_db_seeds_cmd,
		gl_list_users_cmd, gl_create_user_by_email_cmd, gl_reset_password_for_email_user_cmd, gl_grant_role_to_user_cmd, gl_revoke_role_from_user_cmd,
	)
}

func init_logger() {
	if gl_debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}
	slog.Debug("run on debug mode")
}
