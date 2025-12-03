interface IProps {
  name: string;
  devices: string[];
}

// https://www.speedtest.cn/tools/ipCalculator
// 192.168.0~254.1/24~26
// 172.16~31.0~254.0/16~24

/*
A: 10.0.0.0～10.255.255.255 
B: 172.16.0.0～172.31.255.255 
C: 192.168.0.0～192.168.255.255 
*/

interface IFormValues {
  ip1: number;
  ip2: number;
  ip3: number;
  ip4: number;
  cidr: number;
  dns: string;
  enable: boolean;
}
const Widget = ({ name }: IProps) => {
  return (
    <div className="card">
      <header className="card-header">
        <p className="card-header-title">{name}</p>
        <button className="card-header-icon" aria-label="more options">
          <span className="icon">
            <i className="fas fa-angle-down" aria-hidden="true"></i>
          </span>
        </button>
      </header>
      <div className="card-content">
        <div className="content">
          <div></div>
        </div>
      </div>
      <footer className="card-footer"></footer>
    </div>
  );
};
export default Widget;
