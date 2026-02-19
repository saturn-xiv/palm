package cups

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"

	v2 "github.com/saturn-xiv/palm/daisy/cups/v2"
)

type Server struct {
	v2.UnimplementedCupsServer
}

func NewServer() *Server {
	return &Server{}
}

func (p *Server) GetAllSubjects(ctx context.Context, req *emptypb.Empty) (*v2.CupsPrintersResponse, error) {
	return v2.NewCupsPrintersResponse()
}
