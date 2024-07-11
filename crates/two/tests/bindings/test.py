import unittest

from zero import *
from one import *
from two import *

# BROKEN: error "attempted relative import with no known parent package"

class TestCoverall(unittest.TestCase):

    def test(self):
        # self.assertEqual(new_one_default(), new_one_default())
        print("works?")
                         
if __name__=='__main__':
    unittest.main()