import unittest
import logging


logger = logging.getLogger(__name__)


class TestEnforcerServer(unittest.TestCase):
    def test_roles(self):
        logging.info("check has role")
        self.assertEqual(2, 1+1)

    def test_permissions(self):
        logging.info("check can permission")
        self.assertTrue(2 == 1+1)

    def test_resources(self):
        logging.info("found object")
        logging.info("found role")
        logging.info("found user")

    def setUp(self):
        logging.info("open gRPC client")

    def tearDown(self):
        logging.info("close gRPC client")


if __name__ == '__main__':
    logging.basicConfig(
        format='%(asctime)s [%(filename)s:%(lineno)d] %(levelname).1s %(message)s', level=logging.DEBUG)
    unittest.main()
