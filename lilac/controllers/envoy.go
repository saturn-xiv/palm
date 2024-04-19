package controllers

import (
	"net/http"
	"text/template"

	"github.com/gin-gonic/gin"
	"github.com/saturn-xiv/palm/lilac/auth"
)

func EnvoyYaml() HandlerFunc {
	return func(c *gin.Context) error {
		domain, err := auth.IsDomain(c.Query("domain"))
		if err != nil {
			return err
		}
		tpl, err := template.ParseFS(gl_templates_fs, "templates/envoy.yaml")
		if err != nil {
			return err
		}
		if err = tpl.Execute(c.Writer, gin.H{
			"domain": domain,
		}); err != nil {
			return err
		}
		c.Header(CONTENT_TYPE_HEADER, YAML_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}

func EnvoyService() HandlerFunc {
	return func(c *gin.Context) error {
		tpl, err := template.ParseFS(gl_templates_fs, "templates/envoy.service")
		if err != nil {
			return err
		}
		if err = tpl.Execute(c.Writer, gin.H{}); err != nil {
			return err
		}
		c.Header(CONTENT_TYPE_HEADER, TEXT_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}
