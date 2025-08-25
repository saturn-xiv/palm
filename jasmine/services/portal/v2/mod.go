package v2

import (
	"fmt"
	"io"
	"log/slog"
	"net/http"

	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

var (
	ErrorUserIsNotSignedIn        = status.Error(codes.PermissionDenied, "user is'not signed in")
	ErrorUserIsLocked             = status.Error(codes.Unavailable, "user is locked")
	ErrorUserMustHasAdministrator = status.Error(codes.PermissionDenied, "user must be an administrator")
	ErrorUserHasRoot              = status.Error(codes.PermissionDenied, "this is a root user")
	ErrorNotFound                 = status.Error(codes.NotFound, "not found")
	ErrorBadRequest               = status.Error(codes.InvalidArgument, "bad request")
)

func (p *HtmlPage) Key() string {
	return fmt.Sprintf("pages.%s", p.Hash)
}

func (p *HtmlPage) Path() string {
	return fmt.Sprintf("/p-%s.html", p.Hash)
}

func (p *HtmlPage) Buffer() ([]byte, error) {
	switch x := p.Body.(type) {
	case *HtmlPage_Url:
		slog.Debug("fetch data from", slog.String("url", x.Url))
		res, err := http.Get(x.Url)
		if err != nil {
			return nil, err
		}
		defer res.Body.Close()
		body, err := io.ReadAll(res.Body)
		if err != nil {
			return nil, err
		}
		if res.StatusCode != http.StatusOK {
			slog.Error("failed", slog.Int("status", res.StatusCode), slog.String("body", string(body)))
			return nil, fmt.Errorf("failed to fetch %s", x.Url)
		}
		return body, nil
	case *HtmlPage_Data:
		return x.Data, nil

	default:
		return nil, fmt.Errorf("unexpected type %T", x)
	}
}
