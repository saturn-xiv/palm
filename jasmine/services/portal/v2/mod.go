package v2

import (
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
