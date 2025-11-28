import { FormattedMessage } from "react-intl";

import type { IInternetBond } from "../../api/router";

interface IProps {
  name: string;
  item: IInternetBond;
}
const Widget = ({ name, item }: IProps) => {
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
          <div>Interfaces: {item.interfaces.join(",")}</div>
          <div>
            {item.enable ? (
              <button className="button is-success">
                <FormattedMessage id="buttons.enable" />
              </button>
            ) : (
              <button className="button is-danger" disabled>
                <FormattedMessage id="buttons.disable" />
              </button>
            )}
          </div>
        </div>
      </div>
      <footer className="card-footer"></footer>
    </div>
  );
};
export default Widget;
