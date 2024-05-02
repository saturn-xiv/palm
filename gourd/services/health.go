package services

import (
	"context"
)

type HealthHandler struct {
	version string
}

func (p *HealthHandler) Check(ctx context.Context) (map[string]string, error) {
	response := make(map[string]string)
	response["version"] = p.version
	return response, nil
}

func NewHealthHandler(version string) *HealthHandler {
	return &HealthHandler{version: version}
}
