import unittest
import logging


from dahlia.protocols import (
    action_by_code, action_by_read, action_by_write, action_by_execute, action_by_append, action_by_debit, action_by_credit, action_by_inquiry,
    role_from_str, subject_by_role_id, subject_by_role_code, subject_by_role_administrator, subject_by_role_root,
    user_from_str,  subject_by_user_id, subject_by_user_code,
    to_str, from_str,
    object_by_type, object_by_id, object_by_code,  rbac_pb2)

logger = logging.getLogger(__name__)


class TestEnforcerServer(unittest.TestCase):
    def test_action_by_read(self):
        it = action_by_read()
        s = to_str(it)
        print(f"action by read: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'read')

    def test_action_by_write(self):
        it = action_by_write()
        s = to_str(it)
        print(f"action by write: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'write')

    def test_action_by_append(self):
        it = action_by_append()
        s = to_str(it)
        print(f"action by append: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'append')

    def test_action_by_execute(self):
        it = action_by_execute()
        s = to_str(it)
        print(f"action by execute: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'execute')

    def test_action_by_debit(self):
        it = action_by_debit()
        s = to_str(it)
        print(f"action by debit: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'debit')

    def test_action_by_credit(self):
        it = action_by_credit()
        s = to_str(it)
        print(f"action by credit: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'credit')

    def test_action_by_inquiry(self):
        it = action_by_inquiry()
        s = to_str(it)
        print(f"action by inquiry: {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.WhichOneof('by'), 'inquiry')

    def test_action_by_code(self):
        code = "a.c"

        it = action_by_code(code)
        s = to_str(it)
        print(f"action by code({code}): {s}")

        jt = rbac_pb2.Action()
        from_str(s, jt)
        self.assertEqual(jt.code, code)
        self.assertEqual(jt.WhichOneof('by'), 'code')

    def test_subject_by_role_id(self):
        id = 123

        it = subject_by_role_id(id)
        s = to_str(it)
        print(f"subject by role id({id}): {s}")

        jt = role_from_str(s)
        self.assertEqual(jt.id, id)
        self.assertEqual(jt.WhichOneof('by'), 'id')

    def test_subject_by_role_code(self):
        code = "r.c"

        it = subject_by_role_code(code)
        s = to_str(it)
        print(f"subject by role code({code}): {s}")

        jt = role_from_str(s)
        self.assertEqual(jt.code, code)
        self.assertEqual(jt.WhichOneof('by'), 'code')

    def test_subject_by_role_administrator(self):
        it = subject_by_role_administrator()
        s = to_str(it)
        print(f"subject by role administrator: {s}")

        jt = role_from_str(s)
        self.assertIsNotNone(jt.administrator)
        self.assertEqual(jt.WhichOneof('by'), 'administrator')

    def test_subject_by_role_root(self):
        it = subject_by_role_root()
        s = to_str(it)
        print(f"subject by role root: {s}")

        jt = role_from_str(s)
        self.assertIsNotNone(jt.root)
        self.assertEqual(jt.WhichOneof('by'), 'root')

    def test_subject_by_user_id(self):
        id = 123

        it = subject_by_user_id(id)
        s = to_str(it)
        print(f"subject by user id({id}): {s}")

        jt = user_from_str(s)
        self.assertEqual(jt.id, id)
        self.assertEqual(jt.WhichOneof('by'), 'id')

    def test_subject_by_user_code(self):
        code = "u.c"

        it = subject_by_user_code(code)
        s = to_str(it)
        print(f"subject by user code({code}): {s}")

        jt = user_from_str(s)
        self.assertEqual(jt.code, code)
        self.assertEqual(jt.WhichOneof('by'), 'code')

    def test_object_by_all(self):
        type_ = 'ttt'

        it = object_by_type(type_)
        s = to_str(it)
        print(f"object by all({type_}): {s}")

        jt = rbac_pb2.Object()
        from_str(s, jt)
        self.assertEqual(jt.type, type_)
        self.assertEqual(jt.WhichOneof('by'), 'all')

    def test_object_by_id(self):
        type_ = 'ttt'
        id = 123

        it = object_by_id(type_, id)
        s = to_str(it)
        print(f"object by id({type_}, {id}): {s}")

        jt = rbac_pb2.Object()
        from_str(s, jt)
        self.assertEqual(jt.type, type_)
        self.assertEqual(jt.id, id)
        self.assertEqual(jt.WhichOneof('by'), 'id')

    def test_object_by_code(self):
        type_ = 'ttt'
        code = 'ccc'

        it = object_by_code(type_, code)
        s = to_str(it)
        print(f"object by code({type_}, {code}): {s}")

        jt = rbac_pb2.Object()
        from_str(s, jt)
        self.assertEqual(jt.type, type_)
        self.assertEqual(jt.code, code)
        self.assertEqual(jt.WhichOneof('by'), 'code')


if __name__ == '__main__':
    logging.basicConfig(
        format='%(asctime)s [%(filename)s:%(lineno)d] %(levelname).1s %(message)s', level=logging.DEBUG)
    unittest.main()
