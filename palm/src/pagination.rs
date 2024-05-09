use super::azalea::v1::{Pager, Pagination};

impl Pagination {
    pub fn new(pager: &Pager, total: i64) -> Self {
        let page = pager.page(total);
        let size = pager.size();

        Self {
            page,
            size,
            total,
            has_next: (page * size < total),
            has_previous: (page > 1),
        }
    }
}

impl Pager {
    pub fn offset(&self, total: i64) -> i64 {
        (self.page(total) - 1) * self.size()
    }

    pub fn page(&self, total: i64) -> i64 {
        let size = self.size();
        if total < size || self.page < 1 {
            return 1;
        }
        let page = self.page;
        if page * size > total {
            let it = total / size;
            return if total % size == 0 { it } else { it + 1 };
        }
        page
    }

    pub fn size(&self) -> i64 {
        let size = self.size;
        if size < Self::MIN_SIZE {
            return Self::MIN_SIZE;
        }
        if size > Self::MAX_SIZE {
            return Self::MAX_SIZE;
        }
        size
    }
    const MAX_SIZE: i64 = 1 << 12;
    const MIN_SIZE: i64 = 1 << 2;
}
