#!/usr/bin/env python3
"""Unit tests for ci/bucket-copy-sample.py. No AWS: python3 ci/test-bucket-copy-sample.py"""

import importlib.util
import os
import unittest

spec = importlib.util.spec_from_file_location('sample', os.path.join(os.path.dirname(__file__), 'bucket-copy-sample.py'))
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

RUN = 'data/selfplay/v1/gen=3/run=20261031-41180888933-1'
OBJECTS = {
    **{f'{RUN}/task=t{t}/{kind}/t{t}-w{w:03d}-s{s:05d}.{ext}': 100 + w + s
       for t in range(2) for w in range(5) for s in range(20)
       for kind, ext in (('positions', 'txt.gz'), ('games', 'jsonl.gz'), ('manifests', 'json'))},
    **{f'{RUN}/task=t{t}/task.json': 13 for t in range(2)},
}


class Strata(unittest.TestCase):
    def test_an_objects_stratum_is_its_directory(self):
        self.assertEqual(m.stratum(f'{RUN}/task=t0/positions/t0-w000-s00000.txt.gz'), f'{RUN}/task=t0/positions')
        self.assertEqual(m.stratum(f'{RUN}/task=t1/task.json'), f'{RUN}/task=t1')

    def test_every_directory_is_sampled(self):
        chosen = m.select(OBJECTS, fraction=0.0, seed=1)
        self.assertEqual({m.stratum(k) for k in chosen}, {m.stratum(k) for k in OBJECTS})
        self.assertEqual(len(chosen), 8)  # 2 tasks x (3 kinds + task.json)

    def test_the_fraction_adds_random_objects_on_top(self):
        chosen = m.select(OBJECTS, fraction=0.1, seed=1)
        self.assertGreaterEqual(len(chosen), 8 + round(0.1 * len(OBJECTS)) - 8)
        self.assertLessEqual(len(chosen), 8 + round(0.1 * len(OBJECTS)))

    def test_a_seed_makes_the_sample_repeatable(self):
        self.assertEqual(m.select(OBJECTS, 0.05, seed=7), m.select(OBJECTS, 0.05, seed=7))
        self.assertNotEqual(m.select(OBJECTS, 0.05, seed=7), m.select(OBJECTS, 0.05, seed=8))

    def test_all_takes_every_object(self):
        self.assertEqual(m.select(OBJECTS, 0.0, seed=1, everything=True), sorted(OBJECTS))


class Compare(unittest.TestCase):
    def fetcher(self, source, destination):
        stores = {'src': source, 'dst': destination}
        return lambda bucket, key: stores[bucket].get(key)

    def test_identical_bytes_match(self):
        data = {'a/b/1': b'one', 'a/b/2': b'two'}
        result = m.compare(['a/b/1', 'a/b/2'], {'a/b/1': 3, 'a/b/2': 3}, 'src', 'dst', self.fetcher(data, dict(data)))
        self.assertEqual(result['matched'], 2)
        self.assertEqual((result['differs'], result['missing'], result['size_differs']), ([], [], []))
        self.assertEqual(result['bytes'], 6)

    def test_different_bytes_of_the_same_size_are_caught(self):
        result = m.compare(['k'], {'k': 3}, 'src', 'dst', self.fetcher({'k': b'abc'}, {'k': b'abd'}))
        self.assertEqual(result['differs'], ['k'])
        self.assertEqual(result['matched'], 0)

    def test_an_object_missing_from_either_side_is_reported(self):
        result = m.compare(['k'], {'k': 3}, 'src', 'dst', self.fetcher({'k': b'abc'}, {}))
        self.assertEqual(result['missing'], ['k'])

    def test_a_source_object_whose_size_changed_since_the_listing_is_reported(self):
        result = m.compare(['k'], {'k': 4}, 'src', 'dst', self.fetcher({'k': b'abc'}, {'k': b'abc'}))
        self.assertEqual(result['size_differs'], ['k'])

    def test_the_result_is_ok_only_when_everything_matched(self):
        self.assertTrue(m.ok({'matched': 1, 'differs': [], 'missing': [], 'size_differs': []}))
        self.assertFalse(m.ok({'matched': 0, 'differs': [], 'missing': [], 'size_differs': []}))
        self.assertFalse(m.ok({'matched': 1, 'differs': ['k'], 'missing': [], 'size_differs': []}))


if __name__ == '__main__':
    unittest.main()
