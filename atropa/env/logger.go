package env

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"gorm.io/gorm/logger"
	"gorm.io/gorm/utils"
)

type gormLogger struct {
}

func (p *gormLogger) LogMode(_level logger.LogLevel) logger.Interface {
	return &gormLogger{}
}
func (p *gormLogger) Info(_ctx context.Context, format string, args ...interface{}) {
	slog.Info(fmt.Sprintf(format, args...))
}
func (p *gormLogger) Warn(_ctx context.Context, format string, args ...interface{}) {
	slog.Warn(fmt.Sprintf(format, args...))
}
func (p *gormLogger) Error(ctx context.Context, format string, args ...interface{}) {
	slog.Error(fmt.Sprintf(format, args...))
}
func (p *gormLogger) Trace(_ctx context.Context, begin time.Time, fc func() (sql string, rowsAffected int64), err error) {
	format := "%s\n[%.3fms] [rows:%v] %s"
	elapsed := time.Since(begin)
	sql, rows := fc()
	if rows == -1 {
		slog.Debug(fmt.Sprintf(format, utils.FileWithLineNum(), float64(elapsed.Nanoseconds())/1e6, "-", sql))
	} else {
		slog.Debug(fmt.Sprintf(format, utils.FileWithLineNum(), float64(elapsed.Nanoseconds())/1e6, rows, sql))
	}
}
