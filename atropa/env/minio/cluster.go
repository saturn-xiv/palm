package minio

import (
	"github.com/mroth/weightedrand/v2"
)

type Cluster struct {
	Namespace string
	Chooser   *weightedrand.Chooser[*Client, uint8]
	Clients   []*Client
}
