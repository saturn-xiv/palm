package v2

func (p *Pager) Offset(total int64) int {
	return (int(p.Page_(total)) - 1) * p.Size_()
}

func (p *Pager) Page_(total int64) int64 {
	size := int64(p.Size_())
	if total < size || p.Page < 1 {
		return 1
	}

	if p.Page*size > total {
		it := total / size
		if total%size == 0 {
			return it
		} else {
			return it + 1
		}

	}
	return p.Page
}

func (p *Pager) Size_() int {
	const MIN_SIZE = 1 << 2
	const MAX_SIZE = 1 << 12
	if p.Size < MIN_SIZE {
		return MIN_SIZE
	}
	if p.Size > MAX_SIZE {
		return MAX_SIZE
	}
	return int(p.Size)
}

func NewPagination(pager *Pager, total int64) *Pagination {
	size := int64(pager.Size_())
	page := int64(pager.Page_(total))

	return &Pagination{
		Size:        int64(size),
		Page:        page,
		Total:       total,
		HasNext:     (page*size < total),
		HasPrevious: (page > 1),
	}
}
