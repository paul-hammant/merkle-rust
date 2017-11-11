import unittest
import requests

class WebTests(unittest.TestCase):

    def __init__(self, testname):
        super(WebTests, self).__init__(testname)
        self.maxDiff = None

    def test_base_index_page_lists_the_root_sha1_and_dirctories_and_their_sha1s(self):
        get = requests.get("http://localhost:8080")
        self.assertEquals(get.status_code, 200)
        self.assertMultiLineEqual(get.content, self.no_leading_spaces(
        """33cf709e348a0bf57686ddc60398f755e9783517
           A 08efed1f72a710fc49cb5f23a1de846ac1535132
           C 2654546c4855cb1d57d3d4b7f5dfa8164786e614
           D bd8858e4df1353ab5dcce1245589a93501a25711
           F e709351603396f3166cf4ff8735a6d5c0086659d
           G 6eff73508a6390f6d0de4e84569390e655e5c293
           H 758cae240aadf5dcd0e7a3d0129bb8a062d78a3d
           I 35371f27a337c344af1540901c8ade202a9f3642
           K 4b834650506cf3e6d978351563b907c4136c833a
           L d01e5168918503b7bd85233de6dd9dc992cb194a
           M 85f67c3498152e0679622f29789cb35e7823cbd2
           N 8ac64ed691c0d1dc8ce3a49d485e33d8a2fcfa22
           O 7393a393a110acefb2b1c759d7447ae829c5d1ce
           P fdfb92f1e30c876b0334964c7e7d7f4c8a91df09
           R 25ec8349f21f976225f934b6247ae3c4f0e7cf7e
           S bf5d650af8458be26d9aa2d97b4c2500e4156057
           T 87d151c159e61a06e0d27244e7a69339f55b1bdc
           U 94c837a1b7cbf9efa91a7ca789d1987022f45ce2
           V 1e1e82914252e9cd64bb3c31dc2e600c52310e1b
           W b02dc2e86d08804a9a0ac25638ff10d01457050b
           """))

    def test_intermediate_index_page_lists_its_sha1_and_dirctories_and_their_sha1s(self):
        get = requests.get("http://localhost:8080/A/AK")
        self.assertEquals(get.status_code, 200)
        self.assertMultiLineEqual(get.content, self.no_leading_spaces(
        """91486ed80d168deebb7ecded3f596481d7f391f2
           A 0c9449edc809cad8ed8d613886467b2b56c4f865
           """))

    def test_leaf_index_page_does_its_lists_its_sha1_and_dirctories_and_json_files_and_their_sha1s(self):
        get = requests.get("http://localhost:8080/A/AK/A/Alaska")
        self.assertEquals(get.status_code, 200)
        self.assertMultiLineEqual(get.content, self.no_leading_spaces(
        """96b55f350ccb1911d7f9d7e7645d6d11178ad753
           2013.json 1674790a70b984c9041ab86c370f942861ead004
           2016.json 194f6519cd60b773a82857cf1aeba8dad4a223ed
           2020.json 20e3ff1ade2385c593f73fd44fd157391d2424e7
           2050.json 19b2da433a273840deddb7a46b16891acab16e3f
           2060.json 45418423999c155abc434e175d42ccf6534bee6d
           2068.json 69576d3632c7ce8b0b2a42d87e9e75049bdaff9d
           2070.json 2d50d3100744fb7c4c5ead72a2896909fbe2ba6a
           2090.json 434e72590bdaa03176ebabc18e52dc0a24918da9
           2100.json a884cf405af5a20276b3b1cc72885833905ffa97
           2105.json f7bfd6be756c919e4ea69cb466f31ae0b2fd213d
           2110.json c88ed14784907db94b11941b760892742bd043f3
           2122.json e5247216a7a1f32851807d4b4b9cd1cf152cd57d
           2130.json 375fc259d26710afe31e1367a59de3d5dea729b1
           2150.json b5c17ce941a9647fbc179cb1b769348cfdaebb98
           2164.json ab2f71a47910463bdde92a77313f7e5edba00063
           2170.json c1d0842e2c77d53bee5c684e0e8ad580a2fff05f
           2180.json 9759646b5a2811efc6bfaee84a2b813f85cb1e1a
           2185.json e2beddc3f245ca79908d9a1590aa177151e66e4d
           2188.json 5881cdc3a15eac0cba15a3e07ad54176cabeeedf
           2195.json b7085173d8685deecf70c6efb63d92ec6db2cf21
           2198.json 78a399df3eeb4f696812d6d46882e4029190cf67
           2220.json 594a20ec3d550eae2ad848fa8c0e08d50bd4e7fa
           2230.json ec4c2c8dfc3cf4c1e6719de0544aaeba7731fc9c
           2240.json 798b13c1c4aa7ee2e5744f8b8a2b553c1447229b
           2261.json 0cd460d99cc7a871230cbe0f6f2ab703339e1630
           2270.json 4e77074d4adc2ac7bb486408b00bda26f98820ea
           2275.json 91ce27fc8b643540dda88afdc5b5b62d97a026fa
           2282.json 3bec0f1f9e3ae818dd096303f796ce28e2fe6d08
           2290.json fd4014808dd77351b939060f1283d395deb0cea5
           """))

    def test_leaf_index_sha1_is_available(self):
        get = requests.get("http://localhost:8080/A/AK/A/Alaska/.sha1")
        self.assertEquals(get.status_code, 200)
        self.assertEquals(get.content, "96b55f350ccb1911d7f9d7e7645d6d11178ad753")

    def test_leaf_sha1_is_available(self):
        get = requests.get("http://localhost:8080/A/AK/A/Alaska/2013.json.sha1")
        self.assertEquals(get.status_code, 200)
        self.assertEquals(get.content, "1674790a70b984c9041ab86c370f942861ead004")

    def test_leaf_json_is_available(self):
        get = requests.get("http://localhost:8080/A/AK/A/Alaska/2013.json")
        self.assertEquals(get.status_code, 200)
        self.assertMultiLineEqual(get.content,
'''{
    "votes": {
        "diff": 37410, 
        "dem": 93003, 
        "tot": 246588, 
        "gop": 130413
    }, 
    "for": "AK/Alaska/2013", 
    "pct": {
        "dem": 37.72, 
        "per_point_diff": 15.17, 
        "gop": 52.89
    }
}''')

    def test_missing_file_handled(self):
        get = requests.get("http://localhost:8080/A/nothinghere.json")
        self.assertEquals(get.status_code, 404)
        self.assertEquals(get.content, 'not there')

    def test_missing_dir_handled(self):
        get = requests.get("http://localhost:8080/AAAAA/")
        self.assertEquals(get.status_code, 404)
        self.assertEquals(get.content, 'not there')

    def no_leading_spaces(self, string):
        c = [item.strip() for item in string.splitlines()]
        return '\n'.join(c)


if __name__ == '__main__':
    import sys

    unittest.util._MAX_LENGTH = 2000

    test_name = "ALL"

    if len(sys.argv) > 1:
        test_name = sys.argv[1]

    test_loader = unittest.TestLoader()
    test_names = test_loader.getTestCaseNames(WebTests)

    suite = unittest.TestSuite()
    if test_name == "ALL":
        for tname in test_names:
            suite.addTest(WebTests(tname))
    else:
        suite.addTest(WebTests(test_name))

    result = unittest.TextTestRunner().run(suite)
    sys.exit(not result.wasSuccessful())