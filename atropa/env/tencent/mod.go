package tencent

import (
	"log/slog"

	"github.com/tencentcloud/tencentcloud-sdk-go/tencentcloud/common"
	"github.com/tencentcloud/tencentcloud-sdk-go/tencentcloud/common/profile"
	cvm "github.com/tencentcloud/tencentcloud-sdk-go/tencentcloud/cvm/v20170312"
	sms "github.com/tencentcloud/tencentcloud-sdk-go/tencentcloud/sms/v20210111"
)

type Config struct {
	SecretID  string
	SecretKEY string
	Region    string
}

func (p *Config) SendSms(from string, to string, message string) error {
	slog.Info("send sms", slog.String("from", from), slog.String("to", to), slog.String("message", message))
	credential := common.NewCredential(p.SecretID, p.SecretKEY)
	client, err := sms.NewClient(credential, p.Region, profile.NewClientProfile())
	if err != nil {
		return err
	}
	// TODO
	res, err := client.SendSms(&sms.SendSmsRequest{})
	if err != nil {
		return err
	}
	slog.Info(res.ToJsonString())
	return nil
}

func (p *Config) CvmClient() (*cvm.Client, error) {
	credential := common.NewCredential(p.SecretID, p.SecretKEY)
	return cvm.NewClient(credential, p.Region, profile.NewClientProfile())
}
