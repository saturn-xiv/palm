package v2

import (
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"strconv"
	"strings"
)

func NewCupsPrintersResponse() (*CupsPrintersResponse, error) {
	out, err := exec.Command("lpstat", "-p", "-l").Output()
	if err != nil {
		return nil, err
	}

	var res CupsPrintersResponse
	for _, line := range strings.Split(string(out), "\n") {
		items := strings.Fields(line)
		if len(items) < 4 {
			slog.Debug("invalid format", "line", line)
			continue
		}
		if items[0] != "printer" {
			slog.Debug("invalid prefix", "line", line)
		}
		res.Items = append(res.Items, &CupsPrintersResponse_Item{
			Name:    items[1],
			Status:  strings.TrimSuffix(items[3], "."),
			Details: strings.TrimSpace(line),
		})
	}

	return &res, nil
}

// https://man7.org/linux/man-pages/man1/lpr.1.html
func (p *Task) Execute() error {
	file, err := p.save()
	if err != nil {
		return err
	}
	defer os.Remove(file)

	slog.Info("print", "job", p.Name, "file", file)
	args := []string{"-T", p.Name, "-#", strconv.FormatUint(uint64(p.Copies), 10), "-r"}

	if len(p.NumberUp) > 0 {
		var pages []string
		for _, it := range p.NumberUp {
			pages = append(pages, strconv.FormatUint(uint64(it), 10))
		}
		args = append(args, "-o", fmt.Sprintf("number-up={%s}", strings.Join(pages, "|")))
	}

	switch p.Media {
	case Task_A3:
		args = append(args, "-o", "media=a3")
	case Task_A4:
		args = append(args, "-o", "media=a4")
	case Task_LETTER:
		args = append(args, "-o", "media=letter")
	}

	switch p.JobSheet {
	case Task_CLASSIFIED:
		args = append(args, "-o", "job-sheets=classified")
	case Task_CONFIDENTIAL:
		args = append(args, "-o", "job-sheets=confidential")
	case Task_SECRET:
		args = append(args, "-o", "job-sheets=secret")
	case Task_STANDARD:
		args = append(args, "-o", "job-sheets=standard")
	case Task_TOP_SECRET:
		args = append(args, "-o", "job-sheets=topsecret")
	case Task_UNCLASSIFIED:
		args = append(args, "-o", "job-sheets=unclassified")
	}

	switch p.Orientation {
	case Task_LANDSCAPE_COUNTER_CLOCKWISE90:
		args = append(args, "-o", "orientation-requested=4")
	case Task_LANDSCAPE_CLOCKWISE90:
		args = append(args, "-o", "orientation-requested=5")
	case Task_REVERSE_PORTRAIT:
		args = append(args, "-o", "orientation-requested=6")
	}

	switch p.Quality {
	case Task_DRAFT:
		args = append(args, "-o", "print-quality=3")
	case Task_NORMAL:
		args = append(args, "-o", "print-quality=4")
	case Task_BEST:
		args = append(args, "-o", "print-quality=5")
	}

	switch p.Sides {
	case Task_ONE:
		args = append(args, "-o", "sides=one-sided")
	case Task_TWO_LONG:
		args = append(args, "-o", "sides=two-sided-long-edge")
	case Task_TWO_SHORT:
		args = append(args, "-o", "sides=two-sided-short-edge")
	}
	args = append(args, file)

	slog.Debug("lpr", "args", args)
	out, err := exec.Command("lpr", args...).Output()
	if err != nil {
		return err
	}
	slog.Info("", "stdout", string(out))
	return nil
}

func (p *Task) save() (string, error) {
	file, err := os.CreateTemp("", "cups-*")
	if err != nil {
		return "", err
	}
	defer file.Close()
	if _, err = file.Write(p.Document); err != nil {
		return "", err
	}
	return file.Name(), nil
}
