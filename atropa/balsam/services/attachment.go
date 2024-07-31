package services

import (
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewAttachmentService(db *gorm.DB) *AttachmentService {
	return &AttachmentService{db: db}
}

type AttachmentService struct {
	pb.UnimplementedAttachmentServer

	db *gorm.DB
}

// func (p *AttachmentService) Create(ctx context.Context, req *pb.AttachmentCreateRequest) (*emptypb.Empty, error) {
// }
// func (p *AttachmentService) SetUploadedAt(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
// }
// func (p *AttachmentService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
// }
// func (p *AttachmentService) Index(ctx context.Context, req *pb.Pager) (*pb.AttachmentIndexResponse, error) {
// }
// func (p *AttachmentService) SetTitle(ctx context.Context, req *pb.AttachmentSetTitleRequest) (*emptypb.Empty, error) {
// }
// func (p *AttachmentService) ById(ctx context.Context, req *pb.IdRequest) (*pb.AttachmentIndexResponse_Item, error) {
// }
// func (p *AttachmentService) ByUser(ctx context.Context, req *pb.IdRequest) (*pb.AttachmentListResponse, error) {
// }
// func (p *AttachmentService) ByResource(ctx context.Context, req *pb.ResourceRequest) (*pb.AttachmentListResponse, error) {
// }
