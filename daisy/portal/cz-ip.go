package portal

import (
	"github.com/tagphi/czdb-search-golang/pkg/db"
)

type QqWry struct {
	searcher *db.DBSearcher
}

// https://github.com/nmgliangwei/qqwry
func NewQqWry(file string) (*QqWry, error) {
	searcher, err := db.InitDBSearcher(file, "", db.MEMORY)
	if err != nil {
		return nil, err
	}
	return &QqWry{searcher}, nil
}

func (p *QqWry) Close() {
	db.CloseDBSearcher(p.searcher)
}

func (p *QqWry) Search(ip string) (string, error) {
	return db.Search(ip, p.searcher)
}
