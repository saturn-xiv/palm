package crypto

import (
	"context"

	"github.com/tink-crypto/tink-go/v2/mac"
	"github.com/tink-crypto/tink-go/v2/tink"
	"google.golang.org/protobuf/types/known/emptypb"

	v2 "com.github/saturn_xiv/palm/daisy/crypto/v2"
)

type Hmac struct {
	primitive tink.MAC
}

func NewHmac(name string) (*Hmac, error) {
	handle, err := load_keyset_file(name, mac.HMACSHA512Tag256KeyTemplate())
	if err != nil {
		return nil, err
	}

	primitive, err := mac.New(handle)
	if err != nil {
		return nil, err
	}
	return &Hmac{primitive: primitive}, nil
}

func (p *Hmac) Compute(data []byte) ([]byte, error) {
	return p.primitive.ComputeMAC(data)
}
func (p *Hmac) Verify(mac []byte, data []byte) error {
	return p.primitive.VerifyMAC(mac, data)
}

type HmacServer struct {
	v2.UnimplementedHMacServer

	mac *Hmac
}

func NewHmacServer(mac *Hmac) *HmacServer {
	return &HmacServer{mac: mac}
}

func (p *HmacServer) Compute(ctx context.Context, req *v2.HMacComputeRequest) (*v2.HMacComputeResponse, error) {
	mac, err := p.mac.Compute(req.Data)
	if err != nil {
		return nil, err
	}

	return &v2.HMacComputeResponse{Mac: mac}, nil
}
func (p *HmacServer) Verify(ctx context.Context, req *v2.HMacVerifyRequest) (*emptypb.Empty, error) {
	if err := p.mac.Verify(req.Mac, req.Data); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
