package v2

import (
	_ "embed"
)

//go:embed templates/firewalld.txt
var gl_firewalld_txt string

//go:embed templates/netplan.txt
var gl_netplan_txt string

//go:embed templates/dnsmasq.txt
var gl_dnsmasq_txt string

//go:embed templates/header.txt
var gl_header_txt string

//go:embed templates/footer.txt
var gl_footer_txt string
