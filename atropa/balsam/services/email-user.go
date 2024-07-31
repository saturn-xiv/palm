package services

import (
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewEmailUserService(db *gorm.DB) *EmailUserService {
	return &EmailUserService{db: db}
}

type EmailUserService struct {
	pb.UnimplementedEmailUserServer

	db *gorm.DB
}

// rpc SignIn(UserSignInByEmailRequest) returns (UserSignInResponse) {}
//   rpc SignUp(UserSignUpByEmailRequest) returns (UserSignInResponse) {}
//   rpc ConfirmByEmail(UserByEmailRequest) returns (google.protobuf.Empty) {}
//   rpc ConfirmByToken(UserByTokenRequest) returns (google.protobuf.Empty) {}
//   rpc UnlockByEmail(UserByEmailRequest) returns (google.protobuf.Empty) {}
//   rpc UnlockByToken(UserByTokenRequest) returns (google.protobuf.Empty) {}
//   rpc ForgotPassword(UserByEmailRequest) returns (google.protobuf.Empty) {}
//   rpc ResetPassword(UserResetPasswordRequest) returns (google.protobuf.Empty) {}

//   rpc Confirm(IdRequest) returns (google.protobuf.Empty) {}
//   rpc Disable(IdRequest) returns (google.protobuf.Empty) {}
//   rpc Enable(IdRequest) returns (google.protobuf.Empty) {}
//   rpc Index(Pager) returns (EmailUserIndexResponse) {}
//   rpc ById(IdRequest) returns (EmailUserIndexResponse.Item) {}
//   rpc ByNickname(EmailUserByNicknameRequest)
//       returns (EmailUserIndexResponse.Item) {}
//   rpc ByEmail(EmailUserByEmailRequest) returns (EmailUserIndexResponse.Item) {}
