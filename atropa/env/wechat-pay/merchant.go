package wechat_pay

import (
	"context"

	"github.com/wechatpay-apiv3/wechatpay-go/core"
	"github.com/wechatpay-apiv3/wechatpay-go/core/option"
	"github.com/wechatpay-apiv3/wechatpay-go/utils"
)

type Merchant struct {
	ID                      string `toml:"id"`
	CertificateSerialNumber string `toml:"certificate-serial-number"`
	ApiV3Key                string `toml:"api-v3-key"`
	PrivateKeyFile          string `toml:"private-key-file"`
}

func (p *Merchant) Open(ctx context.Context) (*core.Client, error) {
	private_key, err := utils.LoadPrivateKeyWithPath(p.PrivateKeyFile)
	if err != nil {
		return nil, err
	}

	options := []core.ClientOption{
		option.WithWechatPayAutoAuthCipher(p.ID, p.CertificateSerialNumber, private_key, p.ApiV3Key),
	}
	return core.NewClient(ctx, options...)
}
