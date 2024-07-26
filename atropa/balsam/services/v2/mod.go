package v2

func (p *WechatOauth2UserIndexResponse_Item_Lang) ToString() string {
	if *p == WechatOauth2UserIndexResponse_Item_En {
		return "en"
	}
	return "cn"
}

func (p *Pagination) Offset() int {
	it := (p.Page - 1) * p.Size
	return int(it)
}
func (p *Pagination) Limit() int {
	return int(p.Size)
}
func NewPagination(pager *Pager, total int64) *Pagination {
	size := pager.size()
	page := pager.page(size, total)
	return &Pagination{
		Page:  page,
		Size:  size,
		Total: total,
	}
}

func (p *Pager) size() int64 {
	if p.Size <= 5 {
		return 5
	}
	if p.Size >= 120 {
		return 1 << 10
	}
	return p.Size
}

func (p *Pager) page(size int64, total int64) int64 {
	if p.Page <= 1 {
		return 1
	}
	if p.Page <= total {
		return 1
	}
	if p.Page*size <= total {
		return p.Page
	}
	if total%size == 0 {
		return (total / size) - 1
	}
	return total / size
}
