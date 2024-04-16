package controllers

import (
	"net/http"
	"text/template"

	"github.com/gin-gonic/gin"
	"github.com/saturn-xiv/palm/lilac/services"
)

func EnvoyYaml() HandlerFunc {
	return func(c *gin.Context) error {
		domain := c.Query("domain")
		if err := services.IsDomain(domain); err != nil {
			return err
		}
		tpl, err := template.ParseFS(gl_templates_fs, "templates/envoy.yml")
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
