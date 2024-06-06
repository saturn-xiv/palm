package services

import (
	"github.com/saturn-xiv/palm/atropa/env/crypto"

	pb "github.com/saturn-xiv/palm/atropa/services/v2"
)

type JwtService struct {
	pb.UnimplementedJwtServer

	jwt *crypto.Jwt
}

type AesService struct {
	pb.UnimplementedAesServer

	aes *crypto.Aes
}

type HmacService struct {
	pb.UnimplementedHMacServer

	hmac *crypto.HMac
}
