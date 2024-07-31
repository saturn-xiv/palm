package env

// https://www.iana.org/assignments/media-types/media-types.xhtml
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
const (
	APPLICATION_GRPC_PROTO = "application/grpc+proto"
	APPLICATION_XML        = "application/xml; charset=UTF-8"
	TEXT_PLAIN             = "text/plain; charset=UTF-8"

	JWT_ISSUER = "palm.atropa"
	JWT_BEARER = "Bearer "

	GRPC_AUTHORIZATION_HEADER   = "authorization"
	GRPC_ACCEPT_LANGUAGE_HEADER = "accept-language"
	GRPC_X_FORWARDED_FOR_HEADER = "X-Forwarded-For"
)
