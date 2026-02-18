package cups

import (
	"context"
	"log/slog"
	"os/exec"
	"strings"

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
	out, err := exec.Command("lpstat", "-p", "-l").Output()
	if err != nil {
		return nil, err
	}

	var res v2.CupsPrintersResponse
	for _, line := range strings.Split(string(out), "\n") {
		items := strings.Fields(line)
		if len(items) < 4 {
			slog.Debug("invalid format", "line", line)
			continue
		}
		if items[0] != "printer" {
			slog.Debug("invalid prefix", "line", line)
		}
		res.Items = append(res.Items, &v2.CupsPrintersResponse_Item{
			Name:    items[1],
			Status:  strings.TrimSuffix(items[3], "."),
			Details: strings.TrimSpace(line),
		})
	}

	return &res, nil
}
