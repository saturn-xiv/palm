package cmd

import (
	"errors"
	"fmt"
	"log/slog"

	"github.com/spf13/cobra"

	"github.com/saturn-xiv/palm/sedge/env"
)

var (
	git_version  string
	author_email string
	author_name  string
	repo_url     string
	build_time   string
)

var root_cmd = &cobra.Command{
	Use:     "sedge",
	Short:   "Sedge",
	Long:    fmt.Sprintf("A lightweight, framework-agnostic database migration tool.(%s).", repo_url),
	Version: fmt.Sprintf("%s(%s) by %s<%s>", git_version, build_time, author_name, author_email),
	RunE: func(cmd *cobra.Command, args []string) error {
		return cmd.Help()
	},
}

func Execute() error {
	return root_cmd.Execute()
}

var (
	gl_debug            bool
	gl_url              string
	gl_migrations_dir   string
	gl_migrations_table string
	gl_schema_file      string

	gl_new_name string
)

func init() {
	root_cmd.PersistentFlags().BoolVarP(&gl_debug, "debug", "d", false, "run on debug mode")
	root_cmd.PersistentFlags().StringVarP(&gl_url, "url", "u", "sqlite3://file:db",
		`specify the database URL
Sqlite3: sqlite3://file:PATH?cache=shared&mode=memory
MySql: mysql://USER:PASSWORD@HOST:PORT/DBNAME?charset=utf8mb4&allowMultiQueries=true
PostgreSQL: postgres://USER:PASSWORD@HOST:PORT/DBNAME?sslmode=disable
SqlServer: sqlserver://USER:PASSWORD@HOST:PORT/INSTANCE?allowMultiQueries=true
Oracle: oracle:thin:@HOST:PORT/INSTANCE
DM8: dm://USER:PASSWORD@HOST:PORT?charset=utf8
`,
	)
	root_cmd.PersistentFlags().StringVar(&gl_migrations_dir, "migrations-dir", "migrations", "specify the directory containing migration files")
	root_cmd.PersistentFlags().StringVar(&gl_migrations_table, "migrations-table", "schema_migrations", "specify the database table to record migrations in")
	root_cmd.PersistentFlags().StringVar(&gl_schema_file, "schema-file", "schema.sql", "specify the schema file location")

	{
		var cmd = &cobra.Command{
			Use:   "migrate",
			Short: "Migrate to the latest version",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Migrate(gl_url, gl_migrations_dir, gl_migrations_table)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "rollback",
			Short: "Rollback the most recent migration",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Rollback(gl_url, gl_migrations_table)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "status",
			Short: "List applied and pending migrations",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Status(gl_url, gl_migrations_dir, gl_migrations_table)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "clear",
			Short: "Clear all schema in the database.",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Clear(gl_url, gl_migrations_dir, gl_migrations_table)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "reset",
			Short: "Rollback all records and migrate then.",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Reset(gl_url, gl_migrations_dir, gl_migrations_table)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "dump",
			Short: "Write the database schema to disk",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Dump(gl_url, gl_schema_file)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "load",
			Short: "Load schema file to the database",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				return env.Load(gl_url, gl_schema_file)
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "create",
			Short: "Create database",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				fmt.Println(env.Create())
				return nil
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "drop",
			Short: "Drop database",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				fmt.Println(env.Drop())
				return nil
			},
		}

		root_cmd.AddCommand(cmd)
	}
	{
		var cmd = &cobra.Command{
			Use:   "new",
			Short: "Generate a new migration file",
			RunE: func(cmd *cobra.Command, args []string) error {
				set_log(gl_debug)
				if gl_new_name == "" {
					return errors.New("please specify a name for the new migration")
				}
				return env.New(gl_migrations_dir, gl_new_name)
			},
		}
		cmd.Flags().StringVarP(&gl_new_name, "name", "n", "", "name")
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
