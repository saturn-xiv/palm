package env

// type Minio struct {
// 	Url       string `json:"url"`
// 	AccessKey string `json:"accessKey"`
// 	SecretKey string `json:"secretKey"`
// 	Api       string `json:"api"`
// 	Path      string `json:"path"`
// }

type Minio struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
}
