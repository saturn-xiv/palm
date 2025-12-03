interface IProps {
  name: string;
  devices: string[];
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
          <div>Interfaces: </div>
          <div></div>
        </div>
      </div>
      <footer className="card-footer"></footer>
    </div>
  );
};
export default Widget;
