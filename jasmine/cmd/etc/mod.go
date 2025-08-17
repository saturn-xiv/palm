package etc

import "embed"

//go:embed templates/*/* templates/*
var gl_templates_fs embed.FS
