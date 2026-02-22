package portal

// func (p *UserServer) Index(ctx context.Context, req *v2.Page) (*v2.UserIndexResponse, error) {
// 	{
// 		ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
// 		if err != nil {
// 			return nil, err
// 		}
// 		if err = ss.IsAdministrator(p.enforcer); err != nil {
// 			return nil, err
// 		}
// 	}
// 	var total int64
// 	if err := p.db.Model(&models.User{}).Count(&total).Error; err != nil {
// 		return nil, err
// 	}
// 	pagination := v2.NewPagination(req, total)
// 	var items []models.User
// 	if err := p.db.Order("updated_at DESC").Offset(int(pagination.Current.Offset())).Limit(int(pagination.Current.Size)).Find(&items).Error; err != nil {
// 		return nil, err
// 	}
// 	res := v2.UserIndexResponse{
// 		Items:      []*v2.UserIndexResponse_Item{},
// 		Pagination: pagination,
// 	}
// 	for _, it := range items {
// 		res.Items = append(res.Items, rbac.NewUser(&it))
// 	}
// 	return &res, nil
// }
