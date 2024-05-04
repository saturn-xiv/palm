package tex_to_pdf

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"log/slog"
	"mime/multipart"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/apache/thrift/lib/go/thrift"
	amqp "github.com/rabbitmq/amqp091-go"

	jasmine_v1 "github.com/saturn-xiv/palm/lily/env/jasmine/v1"
	v1 "github.com/saturn-xiv/palm/lily/services/v1"
)

func Launch(config_file string, consumer_name string, queue_name string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	ctx := context.Background()
	slog.Debug(fmt.Sprintf("open consumer rabbitmq://%s@%s:%d/%s", config.RabbitMq.User, config.RabbitMq.Host, config.RabbitMq.Port, config.RabbitMq.VirtualHost))
	con, err := amqp.Dial(config.RabbitMq.Url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	messages, err := ch.ConsumeWithContext(ctx, queue_name, consumer_name, true, false, false, false, nil)
	if err != nil {
		return nil
	}

	address := fmt.Sprintf("%s:%d", config.Jasmine.Host, config.Jasmine.Port)
	slog.Debug(fmt.Sprintf("open jasmine tcp://%s", address))
	thrift_config := thrift.TConfiguration{}
	protocol_factory := thrift.NewTBinaryProtocolFactoryConf(nil)
	transport_factory := thrift.NewTFramedTransportFactoryConf(thrift.NewTTransportFactory(), &thrift_config)

	transport, err := transport_factory.GetTransport(thrift.NewTSocketConf(address, &thrift_config))
	if err != nil {
		return err
	}
	defer transport.Close()
	if err := transport.Open(); err != nil {
		return err
	}
	in_protocol := protocol_factory.GetProtocol(transport)
	out_protocol := protocol_factory.GetProtocol(transport)

	client := jasmine_v1.NewS3Client(thrift.NewTStandardClient(in_protocol, out_protocol))
	for it := range messages {
		slog.Info(fmt.Sprintf("receive message (%s,%s)", it.MessageId, it.ContentType))
		if err = consume(ctx, client, it.Body); err != nil {
			return err
		}
	}
	return nil
}

func consume(ctx context.Context, client *jasmine_v1.S3Client, message []byte) error {
	var task v1.TexToPdfTask
	{
		t := thrift.NewTMemoryBufferLen(1024 * 1024 * (1 << 4))
		defer t.Close()

		p := thrift.NewTBinaryProtocolConf(t, &thrift.TConfiguration{})
		de := &thrift.TDeserializer{
			Transport: t,
			Protocol:  p,
		}
		if err := de.Read(ctx, &task, message); err != nil {
			return err
		}
	}

	work_dir, err := os.MkdirTemp("", "tex2pdf")
	if err != nil {
		return err
	}
	defer os.RemoveAll(work_dir)

	pdf_file := "main.pdf"
	if err = task.Tex.BuildPdf(work_dir, pdf_file); err != nil {
		return nil
	}

	url, err := client.UploadFile(ctx, task.Bucket, task.Object, int32(time.Duration.Hours(1)))
	if err != nil {
		return err
	}

	return upload_file(url, filepath.Join(work_dir, pdf_file))
}

func upload_file(url string, name string) error {
	slog.Info("upload", slog.String("url", url), slog.String("name", name))
	file, err := os.Open(name)
	if err != nil {
		return err
	}
	defer file.Close()

	body := &bytes.Buffer{}
	writer := multipart.NewWriter(body)
	part, err := writer.CreateFormFile("file", filepath.Base(name))
	if err != nil {
		return err
	}
	_, err = io.Copy(part, file)
	if err != nil {
		return err
	}

	err = writer.Close()
	if err != nil {
		return err
	}

	request, err := http.NewRequest("PUT", url, body)
	if err != nil {
		return err
	}
	request.Header.Set("Content-Type", writer.FormDataContentType())

	client := &http.Client{}
	response, err := client.Do(request)
	if err != nil {
		return err
	}
	slog.Info("succeed", slog.Int("status", response.StatusCode))
	return nil
}
