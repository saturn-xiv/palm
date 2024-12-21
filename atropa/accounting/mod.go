package accounting

import (
	"net/http"

	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, db *gorm.DB, jwt *crypto.Jwt) error {
	group := router.PathPrefix("/ledgers/{token}").Subrouter()

	group.HandleFunc(`/{year:[0-9]{4}}-{month:[0-9]{2}}-{day:[0-9]{2}}`, hibiscus.Wrap(ShowStatementByYearAndMonthAndDay(db, jwt))).Methods(http.MethodGet)
	group.HandleFunc(`/{year:[0-9]{4}}-{month:[0-9]{2}}`, hibiscus.Wrap(ShowStatementByYearAndMonth(db, jwt))).Methods(http.MethodGet)
	group.HandleFunc(`/{year:[0-9]{4}}`, hibiscus.Wrap(ShowStatementByYear(db, jwt))).Methods(http.MethodGet)

	return nil
}
