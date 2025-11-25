import { useAppSelector, useAppDispatch } from "../hooks";
import { selectNotification, close } from "../reducers/notification";
import Timestamp from "../components/Timestamp";

const Widget = () => {
  const { action, messages } = useAppSelector(selectNotification);
  const dispatch = useAppDispatch();
  return action ? (
    <article className={`message is-${action}`}>
      <div className="message-header">
        <p>
          <Timestamp value={new Date()} />
        </p>
        <button
          className="delete"
          aria-label="delete"
          onClick={(e) => {
            e.preventDefault();
            dispatch(close());
          }}
        />
      </div>
      <div className="message-body">
        {messages.map((it, id) => (
          <div key={id}>{it}</div>
        ))}
      </div>
    </article>
  ) : (
    <></>
  );
};

export const Notification = () => {
  const { action, messages } = useAppSelector(selectNotification);
  const dispatch = useAppDispatch();
  return action ? (
    <div className={`notification is-${action}`}>
      <button
        className="delete"
        onClick={(e) => {
          e.preventDefault();
          dispatch(close());
        }}
      />
      <div>
        <strong>
          <Timestamp value={new Date()} />
        </strong>
      </div>
      {messages.map((it, id) => (
        <div key={id}>{it}</div>
      ))}
    </div>
  ) : (
    <></>
  );
};

export default Widget;
