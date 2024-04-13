package env

import (
	"context"
	"time"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/utils"
)

type gormLogger struct {
}

func (p *gormLogger) LogMode(_level logger.LogLevel) logger.Interface {
	return &gormLogger{}
}
func (p *gormLogger) Info(_ctx context.Context, format string, args ...interface{}) {
	log.Infof(format, args...)
}
func (p *gormLogger) Warn(_ctx context.Context, format string, args ...interface{}) {
	log.Warnf(format, args...)
}
func (p *gormLogger) Error(ctx context.Context, format string, args ...interface{}) {
	log.Errorf(format, args...)
}
func (p *gormLogger) Trace(_ctx context.Context, begin time.Time, fc func() (sql string, rowsAffected int64), err error) {
	format := "%s\n[%.3fms] [rows:%v] %s"
	elapsed := time.Since(begin)
	sql, rows := fc()
	if rows == -1 {
		log.Debugf(format, utils.FileWithLineNum(), float64(elapsed.Nanoseconds())/1e6, "-", sql)
	} else {
		log.Debugf(format, utils.FileWithLineNum(), float64(elapsed.Nanoseconds())/1e6, rows, sql)
	}
}

func GinLogger() gin.HandlerFunc {
	return func(c *gin.Context) {
		begin := time.Now()

		log.Infof("%s [%s %s] %s %s",
			c.ClientIP(),
			c.Request.Proto,
			c.Request.Method,
			c.Request.RequestURI,
			c.Request.UserAgent(),
		)

		c.Next()

		elapsed := time.Since(begin)
		log.Infof("%d [%.3fms] %d bytes", c.Writer.Status(), float64(elapsed.Nanoseconds())/1e6, c.Writer.Size())
	}
}
