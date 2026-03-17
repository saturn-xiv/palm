package portal

import (
	"net/netip"

	"github.com/oschwald/geoip2-golang/v2"
)

// https://dev.maxmind.com/geoip/geolite2-free-geolocation-data/
type GeoIp struct {
	reader *geoip2.Reader
}

func NewGeoIp(file string) (*GeoIp, error) {
	reader, err := geoip2.Open("GeoIP2-City.mmdb")
	if err != nil {
		return nil, err
	}
	return &GeoIp{reader}, nil
}
func (p *GeoIp) Close() {
	p.reader.Close()
}
func (p *GeoIp) City(ip string) (*geoip2.City, error) {
	it, err := netip.ParseAddr(ip)
	if err != nil {
		return nil, err
	}
	return p.reader.City(it)
}
