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
    def test_permission(self):
        pass

    def test_role(self):
        user_1 = rbac_pb2.Subject.User(code="u.1")
        user_2 = rbac_pb2.Subject.User(code="u.2")
        role_1 = rbac_pb2.Subject.Role(code="r.1")
        role_2 = rbac_pb2.Subject.Role(code="r.2")

    def setUp(self):
        logging.info("open gRPC client")

    def tearDown(self):
        logging.info("close gRPC client")


if __name__ == '__main__':
    logging.basicConfig(
        format='%(asctime)s [%(filename)s:%(lineno)d] %(levelname).1s %(message)s', level=logging.DEBUG)
    unittest.main()
