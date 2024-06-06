package etc

import "embed"

//go:embed templates/minio/* templates/*
var gl_templates_fs embed.FS
