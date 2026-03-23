package env

import (
	"fmt"
	"os"
)

var (
	git_version string
	build_time  string
)

func Version() string {
	return fmt.Sprintf("%s(%s)", git_version, build_time)
}

func Plugin() string {
	return "daisy"
}

func Id() (string, error) {
	buf, err := os.ReadFile("/sys/class/dmi/id/product_uuid")
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
