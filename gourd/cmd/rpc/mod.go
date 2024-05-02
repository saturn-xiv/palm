package rpc

import (
	"crypto/tls"
	"fmt"
	"log/slog"
	"reflect"

	"github.com/apache/thrift/lib/go/thrift"

	"github.com/saturn-xiv/palm/gourd/services"
	v1 "github.com/saturn-xiv/palm/gourd/services/v1"
)

type Ssl struct {
	CaFile   string
	KeyFile  string
	CertFile string
}

func Launch(port uint16, config_file string, version string, ssl *Ssl) error {
	address := fmt.Sprintf("0.0.0.0:%d", port)
	var transport thrift.TServerTransport
	{
		var err error
		if ssl == nil {
			slog.Info(fmt.Sprintf("listen on tcp://%s", address))
			transport, err = thrift.NewTServerSocket(address)
		} else {
			slog.Info(fmt.Sprintf("listen on tcps://%s", address))
			cfg := new(tls.Config)
			if cert, err := tls.LoadX509KeyPair(ssl.CertFile, ssl.KeyFile); err == nil {
				cfg.Certificates = append(cfg.Certificates, cert)
			} else {
				return err
			}
			transport, err = thrift.NewTSSLServerSocket(address, cfg)

		}
		if err != nil {
			return err
		}
	}

	processor := thrift.NewTMultiplexedProcessor()
	{
		name := serviceName((*v1.Health)(nil))
		health_handler := services.NewHealthHandler(version)
		health_processor := v1.NewHealthProcessor(health_handler)
		slog.Debug("register processor", slog.String("name", name))
		processor.RegisterProcessor(name, health_processor)
	}

	transport_factory := thrift.NewTFramedTransportFactoryConf(thrift.NewTTransportFactory(), nil)
	protocol_factory := thrift.NewTBinaryProtocolFactoryConf(nil)
	server := thrift.NewTSimpleServer4(processor, transport, transport_factory, protocol_factory)
	return server.Serve()
}

func serviceName(it interface{}) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
