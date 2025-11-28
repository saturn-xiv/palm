package router

import (
	"os/exec"
)

type SystemStatus struct{}

func (p *SystemStatus) Top() (string, error) {
	buf, err := exec.Command("top", "-b", "-n", "1").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

func (p *SystemStatus) Sar() (string, error) {
	buf, err := exec.Command("sar", "-A").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

func (p *SystemStatus) Network() (string, error) {
	buf, err := exec.Command("netstat", "-a").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

func (p *SystemStatus) Cpu() (string, error) {
	buf, err := exec.Command("lscpu").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

func (p *SystemStatus) Memory() (string, error) {
	buf, err := exec.Command("vmstat").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
func (p *SystemStatus) DiskSpace() (string, error) {
	buf, err := exec.Command("df", "-a", "-h").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
func (p *SystemStatus) DiskIndexNodes() (string, error) {
	buf, err := exec.Command("df", "-i", "-a", "-h").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

func (p *SystemStatus) Hardware() (string, error) {
	buf, err := exec.Command("dmidecode").Output()
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
