package rpc

import (
	"crypto/tls"
	"fmt"
	"log/slog"
	"reflect"

	"github.com/BurntSushi/toml"
	"github.com/apache/thrift/lib/go/thrift"

	"github.com/saturn-xiv/palm/jasmine/services"
	v1 "github.com/saturn-xiv/palm/jasmine/services/v1"
)

func Launch(port uint16, config_file string, ssl *Ssl) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

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

	processors := thrift.NewTMultiplexedProcessor()
	{
		name := serviceName((*v1.S3)(nil))
		handler := services.NewS3Handler(s3, config.Namespace)
		processor := v1.NewS3Processor(handler)
		slog.Debug("register processor", slog.String("name", name))
		processors.RegisterProcessor(name, processor)
	}

	transport_factory := thrift.NewTFramedTransportFactoryConf(thrift.NewTTransportFactory(), nil)
	protocol_factory := thrift.NewTBinaryProtocolFactoryConf(nil)
	server := thrift.NewTSimpleServer4(processors, transport, transport_factory, protocol_factory)
	return server.Serve()
}

func serviceName(it interface{}) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
