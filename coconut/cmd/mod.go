package cmd

import (
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/coconut/env"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "coconut",
	Short:   "Coconut",
	Long:    fmt.Sprintf("A collection of backup/restore tools by Go.(%s).", repo_url),
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
	gl_keep   uint
	gl_target string

	gl_postgresql_host     string
	gl_postgresql_port     uint16
	gl_postgresql_db_name  string
	gl_postgresql_user     string
	gl_postgresql_password string

	gl_mysql_host     string
	gl_mysql_port     uint16
	gl_mysql_db_name  string
	gl_mysql_user     string
	gl_mysql_password string

	gl_dm8_home     string
	gl_dm8_user     string
	gl_dm8_password string

	gl_oracle_sid            string
	gl_oracle_home           string
	gl_oracle_user           string
	gl_oracle_password       string
	gl_oracle_directory_path string

	gl_sync_host     string
	gl_sync_port     uint16
	gl_sync_user     string
	gl_sync_password string
	gl_sync_key_file string
	gl_sync_source   string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_target, "target", "t", "tmp", "target folder")
	root_cmd.PersistentFlags().UintVarP(&gl_keep, "keep", "k", 7, "only keep N backup files")

	{
		var cmd = &cobra.Command{
			Use:   "postgresql",
			Short: "Backup PostgreSql",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				plugin, err := env.NewPostgreSql(gl_postgresql_host, gl_postgresql_port, gl_postgresql_db_name, gl_postgresql_user, gl_postgresql_password)
				if err != nil {
					return err
				}
				return env.Dump(gl_target, gl_keep, plugin)
			},
		}
		cmd.Flags().StringVarP(&gl_postgresql_host, "host", "H", "127.0.0.1", "host")
		cmd.Flags().Uint16VarP(&gl_postgresql_port, "port", "p", 5432, "port")
		cmd.Flags().StringVarP(&gl_postgresql_user, "user", "u", "postgres", "user")
		cmd.Flags().StringVarP(&gl_postgresql_password, "password", "P", "", "password")
		cmd.Flags().StringVarP(&gl_postgresql_db_name, "db-name", "n", "", "db name")
		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "mysql",
			Short: "Backup MySql",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				plugin, err := env.NewMySql(gl_mysql_host, gl_mysql_port, gl_mysql_db_name, gl_mysql_user, gl_mysql_password)
				if err != nil {
					return err
				}
				return env.Dump(gl_target, gl_keep, plugin)
			},
		}
		cmd.Flags().StringVarP(&gl_mysql_host, "host", "H", "127.0.0.1", "host")
		cmd.Flags().Uint16VarP(&gl_mysql_port, "port", "p", 3306, "port")
		cmd.Flags().StringVarP(&gl_mysql_user, "user", "u", "root", "user")
		cmd.Flags().StringVarP(&gl_mysql_password, "password", "P", "", "password")
		cmd.Flags().StringVarP(&gl_mysql_db_name, "db-name", "n", "", "db name")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "dm8",
			Short: "Backup Dameng v8",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				plugin, err := env.NewDm8(gl_dm8_home, gl_dm8_user, gl_dm8_password)
				if err != nil {
					return err
				}
				return env.Dump(gl_target, gl_keep, plugin)
			},
		}
		cmd.Flags().StringVarP(&gl_dm8_home, "home", "H", "/opt/dmdbms", "dm8 install home")
		cmd.Flags().StringVarP(&gl_dm8_user, "user", "u", "", "user")
		cmd.Flags().StringVarP(&gl_dm8_password, "password", "P", "", "password")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "oracle",
			Short: "Backup Oracle",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				plugin, err := env.NewOracle(gl_oracle_home, gl_oracle_sid, gl_oracle_user, gl_oracle_password, gl_oracle_directory_path)
				if err != nil {
					return err
				}
				return env.Dump(gl_target, gl_keep, plugin)
			},
		}
		cmd.Flags().StringVarP(&gl_oracle_home, "home", "H", "/opt/oracle/product/23c/dbhomeFree", "oracle install folder")
		cmd.Flags().StringVarP(&gl_oracle_sid, "sid", "s", "orclcdb", "sid")
		cmd.Flags().StringVarP(&gl_oracle_user, "user", "u", "", "user")
		cmd.Flags().StringVarP(&gl_oracle_password, "password", "P", "", "password")
		cmd.Flags().StringVarP(&gl_oracle_directory_path, "directory-path", "D", "/opt/oracle/admin/FREE/dpdump", "select directory_path from dba_directories where directory_name='DATA_PUMP_DIR'")
		root_cmd.AddCommand(cmd)
	}

	{
		var cmd = &cobra.Command{
			Use:   "sync",
			Short: "Backup by rsync(ssh)",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return nil
			},
		}
		home, _ := os.UserHomeDir()
		cmd.Flags().StringVarP(&gl_sync_host, "host", "H", "", "keep it empty if backup from local")
		cmd.Flags().Uint16VarP(&gl_sync_port, "port", "p", 22, "ssh port")
		cmd.Flags().StringVarP(&gl_sync_user, "user", "u", "root", "ssh user")
		cmd.Flags().StringVarP(&gl_sync_password, "password", "P", "", "ssh password")
		cmd.Flags().StringVarP(&gl_sync_key_file, "key-file", "K", filepath.Join(home, ".ssh", "id_ed25519"), "private ssh key file")
		cmd.Flags().StringVarP(&gl_sync_source, "source", "s", "", "source filepath")
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
