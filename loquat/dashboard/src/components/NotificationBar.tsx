import { useAppSelector, useAppDispatch } from "../hooks";
import { selectNotification, close } from "../reducers/notification";

const Widget = () => {
  const { action, messages } = useAppSelector(selectNotification);
  const dispatch = useAppDispatch();
  return action ? (
    <div
      className={`notification is-${action}`}
      onClick={(e) => {
        e.preventDefault();
        dispatch(close());
      }}
    >
      <button className="delete"></button>
      {messages.map((it, id) => (
        <div key={id}>{it}</div>
      ))}
    </div>
  ) : (
    <></>
  );
};

export default Widget;
