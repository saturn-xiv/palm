import base64

from google.protobuf.message import Message as ProtobufMessage
from google.protobuf.empty_pb2 import Empty

from .rbac_pb2 import Subject, Object, Action, Permission


def permission_from_line(line: list[str]):
    if len(line) != 3:
        raise ValueError("not a valid permission")
    sub = Subject()
    from_str(line[0], sub)
    obj = Object()
    from_str(line(1), obj)
    act = Action()
    from_str(line(2), act)
    return Permission(subject=sub, object=obj, action=act)


def action_by_read():
    return Action(read=Action.Read())


def action_by_write():
    return Action(write=Action.Write())


def action_by_append():
    return Action(append=Action.Append())


def action_by_execute():
    return Action(execute=Action.Execute())


def action_by_debit():
    return Action(debit=Action.Debit())


def action_by_credit():
    return Action(credit=Action.Credit())


def action_by_inquiry():
    return Action(inquiry=Action.Inquiry())


def action_by_code(code: str):
    return Action(code=code)


def subject_by_user_id(id: int):
    return Subject(user=Subject.User(id=id))


def subject_by_user_code(code: str):
    return Subject(user=Subject.User(code=code))


def user_from_str(s: str):
    it = Subject()
    from_str(s, it)
    return it.user


def subject_by_role_id(id: int):
    return Subject(role=Subject.Role(id=id))


def subject_by_role_code(code: str):
    return Subject(role=Subject.Role(code=code))


def subject_by_role_administrator():
    return Subject(role=Subject.Role(administrator=Subject.Role.Administrator()))


def subject_by_role_root():
    return Subject(role=Subject.Role(root=Subject.Role.Root()))


def role_from_str(s: str):
    it = Subject()
    from_str(s, it)
    return it.role


def object_by_id(type_: str, id: int):
    return Object(type=type_, id=id)


def object_by_code(type_: str, code: str):
    return Object(type=type_, code=code)


def object_by_type(type_: str):
    return rbac_pb2.Object(type=type_, all=Empty())


def to_str(m: ProtobufMessage):
    return base64.b85encode(m.SerializeToString()).decode()


def from_str(s, m: ProtobufMessage):
    m.ParseFromString(base64.b85decode(s.encode()))
