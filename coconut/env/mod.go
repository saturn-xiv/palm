package env

import (
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

type Plugin interface {
	Dump(target string) []*exec.Cmd
	Restore() []*exec.Cmd
}

func Dump(target string, keep uint, plugin Plugin) error {
	version := time.Now().UTC().Format("20060102150405")
	slog.Info("dump to", slog.String("folder", target), slog.String("version", version))
	root := filepath.Join(target, version)

	commands := plugin.Dump(target)
	commands = append([]*exec.Cmd{exec.Command("mkdir", "-p", target)}, commands...)
	commands = append(commands, exec.Command("tar", "--remove-files", "-cf", filepath.Join(target, fmt.Sprintf("%s.tar.xz", version)), "-C", root))
	for _, cmd := range commands {
		slog.Debug("exec", slog.String("command", cmd.String()))
		var out strings.Builder
		cmd.Stdout = &out
		if err := cmd.Run(); err != nil {
			return err
		}
		slog.Debug("ok", slog.String("stdout", out.String()))
	}

	{
		slog.Debug("check history backups", slog.Uint64("keep", uint64(keep)))
		entries, err := os.ReadDir(target)
		if err != nil {
			return err
		}
		// var files []fs.DirEntry
		// for _, item := range entries {
		// 	if item.Type().IsRegular() {
		// 		files = append(files, item)
		// 	}
		// }
		sort.Slice(entries, func(i, j int) bool {
			fi, err := entries[i].Info()
			if err != nil {
				slog.Error(err.Error())
				return false
			}
			fj, err := entries[j].Info()
			if err != nil {
				slog.Error(err.Error())
				return false
			}
			return fi.ModTime().After(fj.ModTime())
		})
		if len(entries) > int(keep) {
			for _, item := range entries[keep:] {
				slog.Warn("remove", slog.String("file", item.Name()))
				if err = os.RemoveAll(filepath.Join(target, item.Name())); err != nil {
					return err
				}
			}
		}
	}

	{
		fmt.Println("======= HOW TO RESTORE =======")
		commands := plugin.Restore()
		commands = append([]*exec.Cmd{
			exec.Command(
				"tar", "xf", fmt.Sprintf("%s.tar.xz", version),
			),
		}, commands...)
		for _, item := range commands {
			fmt.Println(item.String())
		}
		fmt.Println("======= END =======")
	}
	slog.Info("done.")
	return nil
}
