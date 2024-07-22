package controllers

import "embed"

//go:embed views/*
var gl_views_fs embed.FS

//go:embed assets/*
var gl_assets_fs embed.FS
