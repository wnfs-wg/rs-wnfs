window.BENCHMARK_DATA = {
  "lastUpdate": 1686577431887,
  "repoUrl": "https://github.com/wnfs-wg/rs-wnfs",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "name": "wnfs-wg",
            "username": "wnfs-wg"
          },
          "committer": {
            "name": "wnfs-wg",
            "username": "wnfs-wg"
          },
          "id": "5e40b59f202e3e5f6dd48516133325f0eae40a64",
          "message": "Initial Benchmark Work",
          "timestamp": "2022-10-25T12:14:32Z",
          "url": "https://github.com/wnfs-wg/rs-wnfs/pull/75/commits/5e40b59f202e3e5f6dd48516133325f0eae40a64"
        },
        "date": 1666779590702,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 261436,
            "range": "± 8566",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get/0",
            "value": 181940,
            "range": "± 9132",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove/0",
            "value": 310927,
            "range": "± 1683",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42878,
            "range": "± 1582",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 320285,
            "range": "± 19041",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9383,
            "range": "± 478",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b00eed1503d7641885df026a733c67ac542a8ed4",
          "message": "Initial Benchmark Work (#75)\n\n* Add hamt and namefilter benchmarks\r\n\r\n* Move criterion to dev-deps\r\n\r\n* Add suggestions by @zeeshanlakhani\r\n\r\n- Unrestrict version path\r\n- Bench throughput\r\n- Add github action\r\n\r\n* Fix cargo bench extra args issue\r\n\r\n* Add throughput in namefilter bench\r\n\r\n* Update logo link\r\n\r\n* Move files after rebase\r\n\r\n* Change relative readme links to absolute\r\n\r\n* Adjust namefilter benchmarks & add Sampleable\r\n\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>\r\n\r\n* Extract out benchmarks into its own crate\r\n\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>\r\n\r\n* Sort imports & fix bench github action\r\n\r\n* Hamt set operation doesn't need &mut store\r\n\r\n* Setup big random HAMT before node set\r\n\r\n* Fix CI issues\r\n\r\n* More fixes\r\n\r\n* Remove unnecessary throughput\r\n\r\n* Add docs, link to gh page, etc.\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>",
          "timestamp": "2022-10-29T00:08:12+01:00",
          "tree_id": "969b0ea2d4cae8c243db6345d3b50f7f244e7858",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b00eed1503d7641885df026a733c67ac542a8ed4"
        },
        "date": 1666999053626,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9035,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177854,
            "range": "± 4632",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 316227,
            "range": "± 4025",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41947,
            "range": "± 1338",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 305405,
            "range": "± 7549",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9344,
            "range": "± 287",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13138,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48996,
            "range": "± 1394",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 193,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7a187bc05fd11c14b14099145a6b81d6e810d3a1",
          "message": "Make private forest multivalue (#78)\n\n* Make private forest multivalue\r\n\r\n* Write own implementation of `BTreeSet::first`\r\n\r\nit's not in stable yet (CI).\r\nAlso added a test\r\n\r\n* Remove const `BTreeSet::new`, as it's also unstable\r\n\r\n* Make lil' clippy happy\r\n\r\n* Fix wasm tests\r\n\r\n* Remove unused `BTreeSet` imports",
          "timestamp": "2022-11-02T14:25:30+01:00",
          "tree_id": "541515509f8d7a61b6778155cebb1223e04b6421",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7a187bc05fd11c14b14099145a6b81d6e810d3a1"
        },
        "date": 1667395879848,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7150,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 161906,
            "range": "± 456",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 272648,
            "range": "± 864",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40890,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 264432,
            "range": "± 2298",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7758,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10762,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42548,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 155,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ce7d988157884115e32d7db412b7d4cedf56d23e",
          "message": "use src for lib.rs rust files (#82)",
          "timestamp": "2022-11-03T10:40:59-04:00",
          "tree_id": "af7f485524c747d850f31edc874af620bd675be5",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ce7d988157884115e32d7db412b7d4cedf56d23e"
        },
        "date": 1667487087112,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8464,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172807,
            "range": "± 12432",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 303618,
            "range": "± 27865",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44588,
            "range": "± 1900",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 311641,
            "range": "± 18767",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13401,
            "range": "± 1067",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13281,
            "range": "± 426",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 47916,
            "range": "± 2071",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 217,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "80dbe82dd41e9eebda77960b930458d4d1feeb69",
          "message": "Rename `RatchetKey` to `RevisionKey`, encrypt `RevisionKey` within `PrivateRef` (#83)\n\n* Encrypt the ratchet key\r\n\r\n* Rename `RatchetKey` to `RevisionKey`\r\n\r\n* Fix wasm crate\r\n\r\n* Rename `*Serde` into `*Serializable`\r\n\r\n* Rename `_serde` variables into `_serializable`",
          "timestamp": "2022-11-04T15:40:44+01:00",
          "tree_id": "c2b6a0faee05510ae00213db16d0fc01c4b5db6e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/80dbe82dd41e9eebda77960b930458d4d1feeb69"
        },
        "date": 1667573041785,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7811,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 146640,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 258501,
            "range": "± 933",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33604,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 261463,
            "range": "± 861",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7704,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9870,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40378,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 163,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9ff76f6c1b8ff5dee39c2c0637e578aa5fbcc278",
          "message": "Use `Rc::try_unwrap` for (possibly?) better performance (#85)\n\n* Benchmarks first\r\n\r\n* Use `Rc::try_unwrap` in `Node::set_value`\r\n\r\nAnd in `Node::remove_value`\r\n\r\n* Remove `RemoveResult` type alias\r\n\r\n* Add invariant checks on deserialization\r\n\r\n* Merge a level of `if`s into `match`\r\n\r\n* clippy: Use non-panicing `Utc.timestamp_opt`",
          "timestamp": "2022-11-17T13:41:06+01:00",
          "tree_id": "ea0698c82673a989f0192df90a66e9eeca14a6f3",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9ff76f6c1b8ff5dee39c2c0637e578aa5fbcc278"
        },
        "date": 1668689370632,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6813,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4278406,
            "range": "± 16527",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148662,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167000,
            "range": "± 872",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34230,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170223,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8987,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11934,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40980,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 166,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9d3b5c2d4357f57919668e3bc31aea497f41b6a4",
          "message": "Private File Sharding (#88)\n\n* Implement file content with default sharding\r\n\r\n* Fix tests and wasm/js apis\r\n\r\n* Fix doc tests\r\n\r\n* Try wasm-js-tests on maco-latest\r\n\r\n* Refactor `stream_content` to accept index and limit arg, etc.\r\n\r\n- Remove basic heuristic and default to file sharding\r\n- Fix shard label generation to conform with spec\r\n\r\n* Fix deprecated timestamp and remove edge browser tests\r\n\r\n* Remove limit param\r\n\r\n* Remove unwraps in timestamp conversion\r\n\r\n* Fix link in readmes\r\n\r\n* Change empty function to new\r\n\r\n- Use array in create_private_file_result\r\n\r\n* Add docs to test_setup macros\r\n\r\n- Add spec reference to `MAX_BLOCK_CONTENT_SIZE` constant",
          "timestamp": "2022-11-21T18:27:45+01:00",
          "tree_id": "e1ffb9365cf9bc6bb3063208fe3f11752d567cc7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9d3b5c2d4357f57919668e3bc31aea497f41b6a4"
        },
        "date": 1669052044214,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7142,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4394527,
            "range": "± 18958",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152359,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170235,
            "range": "± 1530",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35207,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169346,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8566,
            "range": "± 216",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9590,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41358,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 155,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen A",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f6cadeb3765f0cd2f6e7b4f0e42ed657d956be82",
          "message": "Allow user-provided ratchet seed and inumber (#91)\n\n* Support user-specified ratchet seed\r\n\r\n* Support user-provided inumber\r\n\r\n- Add tests\r\n\r\n* Add js api and bump versions\r\n\r\n* Rename test\r\n\r\n* Add `PrivateRef::with_seed`\r\n\r\n- Remove Result from `PrivateNodeHeader::get_private_ref`\r\n\r\n* Add new test for creating deterministic privateref",
          "timestamp": "2022-12-02T11:41:09+01:00",
          "tree_id": "6e70654fcfa8f0df971a00214c8ab6fc2a8c9a74",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f6cadeb3765f0cd2f6e7b4f0e42ed657d956be82"
        },
        "date": 1669978107686,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7159,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4460920,
            "range": "± 17185",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 154707,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170069,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34819,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171164,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 16275,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10654,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42339,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 171,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3e926129e4e135af75275570841519a86c1c69a3",
          "message": "chore: put release-please process in place (#96)\n\nIncludes:\r\n    * pre-commit update for conventional commits\r\n    * dependabot addition for deps\r\n    * reverts *.toml v0.1.10 so that we can use the automated process on the next `fix`\r\n      commit\r\n    * won't publish bench and 0's out its version",
          "timestamp": "2022-12-05T15:56:34-05:00",
          "tree_id": "3e89518af97ea199e180be7cbf552d9c25134c47",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/3e926129e4e135af75275570841519a86c1c69a3"
        },
        "date": 1670274294292,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10148,
            "range": "± 316",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5360826,
            "range": "± 210156",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 199081,
            "range": "± 6694",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 218893,
            "range": "± 13692",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48577,
            "range": "± 5193",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 213854,
            "range": "± 7102",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13203,
            "range": "± 711",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13369,
            "range": "± 915",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 56286,
            "range": "± 2113",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 253,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "43a109d6d99a23cb804d5ba3d64ca3d967662e94",
          "message": "chore(ci)(deps): bump actions/upload-artifact from 2 to 3 (#97)\n\nBumps [actions/upload-artifact](https://github.com/actions/upload-artifact) from 2 to 3.\r\n- [Release notes](https://github.com/actions/upload-artifact/releases)\r\n- [Commits](https://github.com/actions/upload-artifact/compare/v2...v3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/upload-artifact\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-05T16:16:12-05:00",
          "tree_id": "a481772b90e2dd6a1cddb65a7a7cd379dc39f288",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/43a109d6d99a23cb804d5ba3d64ca3d967662e94"
        },
        "date": 1670277021139,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9186,
            "range": "± 516",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5141129,
            "range": "± 346692",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 185897,
            "range": "± 17185",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 207105,
            "range": "± 12770",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44211,
            "range": "± 2066",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 207999,
            "range": "± 8587",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12640,
            "range": "± 727",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12580,
            "range": "± 673",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50523,
            "range": "± 1767",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 227,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "307ba5bd4d1bed156239523f98b2cb2061edb431",
          "message": "refactor: dependabot labels (#109)",
          "timestamp": "2022-12-05T20:04:45-05:00",
          "tree_id": "5f6e11728036a81a75a54699032e76ae30599271",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/307ba5bd4d1bed156239523f98b2cb2061edb431"
        },
        "date": 1670288892972,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6992,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4437623,
            "range": "± 16473",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151855,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170055,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35680,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170460,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7477,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9389,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42610,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 160,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ecfae45d3042d8dd35beec2802eb9beb5d1c3677",
          "message": "chore(npm)(deps-dev): bump css-loader from 6.7.1 to 6.7.2 in /wnfs-wasm (#98)\n\nBumps [css-loader](https://github.com/webpack-contrib/css-loader) from 6.7.1 to 6.7.2.\r\n- [Release notes](https://github.com/webpack-contrib/css-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/css-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/css-loader/compare/v6.7.1...v6.7.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: css-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-05T20:05:21-05:00",
          "tree_id": "aa23ae535fb485acfde4f17e5589dddf10108329",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ecfae45d3042d8dd35beec2802eb9beb5d1c3677"
        },
        "date": 1670288941144,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7133,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4215135,
            "range": "± 17486",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148012,
            "range": "± 1299",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166525,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34274,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168077,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7575,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9792,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40382,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 170,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "92b5ef53d440d76ee644159c717175f24ba396c8",
          "message": "chore(npm)(deps-dev): bump webpack from 5.72.1 to 5.75.0 in /wnfs-wasm (#99)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.72.1 to 5.75.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.72.1...v5.75.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-05T20:05:35-05:00",
          "tree_id": "1131e350dbbd4984cedb7dbcc57e0a78cb0c386e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/92b5ef53d440d76ee644159c717175f24ba396c8"
        },
        "date": 1670289194420,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8131,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4213637,
            "range": "± 17340",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148794,
            "range": "± 241",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166293,
            "range": "± 389",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33916,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168485,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7536,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9789,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40364,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 170,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "549d51548d4bb4660c8bab930ef6e2c4ca9d00f6",
          "message": "chore(rust)(deps): update env_logger requirement in /wnfs (#102)\n\nUpdates the requirements on [env_logger](https://github.com/rust-cli/env_logger) to permit the latest version.\r\n- [Release notes](https://github.com/rust-cli/env_logger/releases)\r\n- [Changelog](https://github.com/rust-cli/env_logger/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-cli/env_logger/compare/v0.9.0...v0.10.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: env_logger\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T04:07:48-05:00",
          "tree_id": "e5060a3fe0fc55b334a780dade5a7e063d6d28f2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/549d51548d4bb4660c8bab930ef6e2c4ca9d00f6"
        },
        "date": 1670317884078,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7064,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4445511,
            "range": "± 16834",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152768,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169551,
            "range": "± 739",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35244,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170977,
            "range": "± 429",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7476,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9390,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42971,
            "range": "± 506",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 163,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4fea43397f897d008e12b305ba17d62f00904869",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#112)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.22.0 to 1.28.1.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.22.0...v1.28.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T04:09:24-05:00",
          "tree_id": "5f0cae1defab34b8655ff85f1dfef3ad81b5902a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4fea43397f897d008e12b305ba17d62f00904869"
        },
        "date": 1670317975680,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7039,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4444571,
            "range": "± 16924",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151828,
            "range": "± 929",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169998,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34936,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169964,
            "range": "± 673",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7479,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9399,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42486,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 161,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd7da466d477e264d797d279e0eb10bbbb42c6cf",
          "message": "chore(npm)(deps-dev): bump ts-loader from 9.3.0 to 9.4.2 in /wnfs-wasm (#111)\n\nBumps [ts-loader](https://github.com/TypeStrong/ts-loader) from 9.3.0 to 9.4.2.\r\n- [Release notes](https://github.com/TypeStrong/ts-loader/releases)\r\n- [Changelog](https://github.com/TypeStrong/ts-loader/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/TypeStrong/ts-loader/compare/v9.3.0...v9.4.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: ts-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T10:29:19+01:00",
          "tree_id": "95f94647208a05cea5dc5e353e7286e1af239e74",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/fd7da466d477e264d797d279e0eb10bbbb42c6cf"
        },
        "date": 1670319175898,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7025,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4240116,
            "range": "± 17066",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148994,
            "range": "± 1811",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166399,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34394,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168712,
            "range": "± 303",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7574,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9785,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40342,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 171,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e8e8824441571acc5dc1ac300cf3a735161cd8d7",
          "message": "chore(rust)(deps): update hashbrown requirement in /wnfs (#106)\n\nUpdates the requirements on [hashbrown](https://github.com/rust-lang/hashbrown) to permit the latest version.\r\n- [Release notes](https://github.com/rust-lang/hashbrown/releases)\r\n- [Changelog](https://github.com/rust-lang/hashbrown/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/hashbrown/compare/v0.12.0...v0.13.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: hashbrown\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T10:30:05+01:00",
          "tree_id": "8a0b1fea2f0e3edd78f909a5c3e03523132d7af3",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e8e8824441571acc5dc1ac300cf3a735161cd8d7"
        },
        "date": 1670319242040,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7479,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4200207,
            "range": "± 16663",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 147525,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166254,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33991,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168444,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8599,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11745,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40988,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 173,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8f9ceaa63ce96bf4f65b407af8002429b1e93f95",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#104)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 9.6.5 to 10.0.2.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v9.6.5...v10.0.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T11:42:49+01:00",
          "tree_id": "70c4fc90a2de781d59b3940a399d868ccd1107ec",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8f9ceaa63ce96bf4f65b407af8002429b1e93f95"
        },
        "date": 1670323581970,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8729,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4983277,
            "range": "± 50717",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176715,
            "range": "± 1272",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197900,
            "range": "± 2517",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40252,
            "range": "± 375",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 201230,
            "range": "± 1298",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10326,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14009,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 49002,
            "range": "± 641",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 331,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ea9ffddf0ea84e60832124e297317f739d334d32",
          "message": "chore(npm)(deps-dev): bump typescript from 4.6.4 to 4.9.3 in /wnfs-wasm (#101)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 4.6.4 to 4.9.3.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v4.6.4...v4.9.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T11:43:08+01:00",
          "tree_id": "b17548213bf20174b04bf6377c8eac3cda3f8186",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ea9ffddf0ea84e60832124e297317f739d334d32"
        },
        "date": 1670323587995,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7149,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4470716,
            "range": "± 17817",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153338,
            "range": "± 1390",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170023,
            "range": "± 1663",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35020,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171182,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7508,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10289,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42074,
            "range": "± 394",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 158,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6df076e2cbdb18a7cfb013bcf12a50cdf1669840",
          "message": "chore(npm)(deps-dev): bump webpack-cli in /wnfs-wasm (#100)\n\nBumps [webpack-cli](https://github.com/webpack/webpack-cli) from 4.10.0 to 5.0.1.\r\n- [Release notes](https://github.com/webpack/webpack-cli/releases)\r\n- [Changelog](https://github.com/webpack/webpack-cli/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-cli/compare/webpack-cli@4.10.0...webpack-cli@5.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-cli\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T11:43:32+01:00",
          "tree_id": "dc62e0645efbca5cce0a8505ab4998b7d6a995f2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6df076e2cbdb18a7cfb013bcf12a50cdf1669840"
        },
        "date": 1670323600666,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7850,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3704980,
            "range": "± 24779",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 130422,
            "range": "± 2174",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 146677,
            "range": "± 1237",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 29847,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 148652,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7596,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10292,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 36200,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 155,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2a6897f7872efa5e4bd7e5b506720dec1d6ae90b",
          "message": "Some renaming pre-release (#95)\n\n* Rename  ->  & mod tests\r\n\r\n* Convert more mentions of \"HAMT\" into private forest\r\n\r\n* Upgrade `libipld` and `aes-gcm`. Remove `multihash`\r\n\r\n* Enable `fs` feature for getrandom",
          "timestamp": "2022-12-06T13:53:22+01:00",
          "tree_id": "a9d5dbdd912a5e454331ffc0921e6f82df489bc8",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2a6897f7872efa5e4bd7e5b506720dec1d6ae90b"
        },
        "date": 1670331431916,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6991,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4254907,
            "range": "± 15723",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149243,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167562,
            "range": "± 282",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33914,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170302,
            "range": "± 411",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8602,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11722,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41014,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 175,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "303192bfccca0a48090bdf2750a741b10f96bc7c",
          "message": "Fix the readme links (#113)",
          "timestamp": "2022-12-06T15:01:42+01:00",
          "tree_id": "6795f8cc5ac9a543fa52bfac48c399de32d304cf",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/303192bfccca0a48090bdf2750a741b10f96bc7c"
        },
        "date": 1670335490572,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7703,
            "range": "± 493",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4317892,
            "range": "± 294663",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 156661,
            "range": "± 22659",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 177782,
            "range": "± 12978",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 37118,
            "range": "± 2810",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 185828,
            "range": "± 13554",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12122,
            "range": "± 990",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10764,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 46271,
            "range": "± 3673",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 302,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen A",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9cbc32076d80a5b7d3138ea891180c689411123f",
          "message": "chore: release 0.1.10 (#114)\n\nRelease-As: 0.1.10",
          "timestamp": "2022-12-06T17:56:12+01:00",
          "tree_id": "6795f8cc5ac9a543fa52bfac48c399de32d304cf",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f"
        },
        "date": 1670345960441,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6742,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4418518,
            "range": "± 17441",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151999,
            "range": "± 1857",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170809,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35208,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169260,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7477,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10005,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41996,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 159,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a3707490237a8c2037b2bcafb16c4c522c20bf13",
          "message": "chore: release main (#115)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T18:26:05+01:00",
          "tree_id": "2c89076c43d70859ba301a35f265722ff3506685",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/a3707490237a8c2037b2bcafb16c4c522c20bf13"
        },
        "date": 1670347786196,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6557,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4443762,
            "range": "± 16746",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151678,
            "range": "± 6151",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170419,
            "range": "± 222",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35018,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169229,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8286,
            "range": "± 161",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10129,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41479,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 156,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9ffad56e6ab402c8636b13563a5bf516fb962037",
          "message": "chore: rename to wnfs-wasm and actions fix *maybe* (#116)\n\nRelease-As: 0.1.10",
          "timestamp": "2022-12-06T13:55:35-05:00",
          "tree_id": "73e040c0f7239c522aa817fe0b08e97a6d251a4e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9ffad56e6ab402c8636b13563a5bf516fb962037"
        },
        "date": 1670353108605,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7146,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4479248,
            "range": "± 16528",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151756,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170140,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35473,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171491,
            "range": "± 455",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7551,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10103,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41203,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 156,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b6c3955d63077840e3c13c8e44b3f608dcc0fa96",
          "message": "chore: release main (#117)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T14:13:57-05:00",
          "tree_id": "3a5750b71899812d8b14c74e571e9352da40b81d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b6c3955d63077840e3c13c8e44b3f608dcc0fa96"
        },
        "date": 1670354236712,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9754,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5209976,
            "range": "± 151769",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 186902,
            "range": "± 7686",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 208950,
            "range": "± 7610",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44296,
            "range": "± 1897",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 210512,
            "range": "± 15478",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13349,
            "range": "± 696",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12581,
            "range": "± 834",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 55565,
            "range": "± 2144",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 236,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan.lakhani@gmail.com",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1cc42ef2e26d8c696cfaa170f17c11fd6a5c685b",
          "message": "chore: fix up needs/names (#118)\n\nRelease-As: 0.1.11",
          "timestamp": "2022-12-06T16:41:29-05:00",
          "tree_id": "13eb551c0ef85bd301eb469339a94ac8805f4138",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/1cc42ef2e26d8c696cfaa170f17c11fd6a5c685b"
        },
        "date": 1670363137771,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6614,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4424109,
            "range": "± 16651",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153086,
            "range": "± 1838",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170317,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35037,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170135,
            "range": "± 334",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7479,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9958,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42038,
            "range": "± 334",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 155,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b6c3955d63077840e3c13c8e44b3f608dcc0fa96",
          "message": "chore: release main (#117)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T14:13:57-05:00",
          "tree_id": "3a5750b71899812d8b14c74e571e9352da40b81d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b6c3955d63077840e3c13c8e44b3f608dcc0fa96"
        },
        "date": 1670363234505,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8616,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5082816,
            "range": "± 29062",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177756,
            "range": "± 4946",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 198737,
            "range": "± 3340",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40306,
            "range": "± 561",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 198669,
            "range": "± 4722",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10230,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13775,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48730,
            "range": "± 746",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 332,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b6c3955d63077840e3c13c8e44b3f608dcc0fa96",
          "message": "chore: release main (#117)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T14:13:57-05:00",
          "tree_id": "3a5750b71899812d8b14c74e571e9352da40b81d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b6c3955d63077840e3c13c8e44b3f608dcc0fa96"
        },
        "date": 1670363460594,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8672,
            "range": "± 265",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5011057,
            "range": "± 96935",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172420,
            "range": "± 6146",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194343,
            "range": "± 7801",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 39551,
            "range": "± 2560",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 202542,
            "range": "± 6446",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10544,
            "range": "± 546",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14015,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 49187,
            "range": "± 539",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 332,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b6c3955d63077840e3c13c8e44b3f608dcc0fa96",
          "message": "chore: release main (#117)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-06T14:13:57-05:00",
          "tree_id": "3a5750b71899812d8b14c74e571e9352da40b81d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b6c3955d63077840e3c13c8e44b3f608dcc0fa96"
        },
        "date": 1670370438329,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8738,
            "range": "± 551",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5049375,
            "range": "± 327891",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177050,
            "range": "± 14103",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 198417,
            "range": "± 12199",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42675,
            "range": "± 3960",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 194972,
            "range": "± 11463",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12830,
            "range": "± 999",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12040,
            "range": "± 1033",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 51742,
            "range": "± 2798",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 360,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan.lakhani@gmail.com",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5babeb14dcc5e3410242c7b1e0a2fa33ba5fc8cb",
          "message": "chore: fix-release flow (#119)",
          "timestamp": "2022-12-07T07:29:04-08:00",
          "tree_id": "9353aa7ab1dbc8dda0ac6c44261a9ae987830978",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5babeb14dcc5e3410242c7b1e0a2fa33ba5fc8cb"
        },
        "date": 1670427124093,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6688,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4447328,
            "range": "± 18081",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151913,
            "range": "± 2245",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170016,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35096,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169842,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7475,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10006,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41895,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 156,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0f6264a30537c62a1f2ec0a0f803237bded74e74",
          "message": "chore(npm)(deps-dev): bump webpack-dev-server in /wnfs-wasm (#120)\n\nBumps [webpack-dev-server](https://github.com/webpack/webpack-dev-server) from 4.7.4 to 4.11.1.\r\n- [Release notes](https://github.com/webpack/webpack-dev-server/releases)\r\n- [Changelog](https://github.com/webpack/webpack-dev-server/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-dev-server/compare/v4.7.4...v4.11.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-dev-server\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-15T10:36:31-05:00",
          "tree_id": "077ae3918af2c05b37277db9e41dcd59a5a65613",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0f6264a30537c62a1f2ec0a0f803237bded74e74"
        },
        "date": 1671118994974,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7080,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4448337,
            "range": "± 16280",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151184,
            "range": "± 3605",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170375,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35336,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169789,
            "range": "± 1521",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7503,
            "range": "± 291",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10018,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41152,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 159,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dvargas92495@gmail.com",
            "name": "David Vargas",
            "username": "dvargas92495"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4aa20a719f3b05531dc9c83628c597cf4ec3909d",
          "message": "Fix typo for main script (#123)",
          "timestamp": "2022-12-15T10:36:20-05:00",
          "tree_id": "f05bfefa57b7e4290cae4a83012c9b654ba72537",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4aa20a719f3b05531dc9c83628c597cf4ec3909d"
        },
        "date": 1671119093427,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10421,
            "range": "± 751",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5735759,
            "range": "± 246056",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 199292,
            "range": "± 9609",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 226990,
            "range": "± 10262",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47886,
            "range": "± 1534",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 224307,
            "range": "± 12799",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 14396,
            "range": "± 704",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14077,
            "range": "± 853",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 55261,
            "range": "± 2465",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 265,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "92a30ac35015566910170a36cd9bb43d9130c998",
          "message": "chore(npm)(deps-dev): bump typescript from 4.9.3 to 4.9.4 in /wnfs-wasm (#121)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 4.9.3 to 4.9.4.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v4.9.3...v4.9.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-15T10:36:41-05:00",
          "tree_id": "70f0d6926bf84aab30f3012ae56227831e041c05",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/92a30ac35015566910170a36cd9bb43d9130c998"
        },
        "date": 1671119384101,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9448,
            "range": "± 533",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4961468,
            "range": "± 366328",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 184134,
            "range": "± 12462",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 196913,
            "range": "± 12459",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47845,
            "range": "± 3544",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 200190,
            "range": "± 30644",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13352,
            "range": "± 899",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12591,
            "range": "± 801",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48684,
            "range": "± 3055",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 227,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "98ff7f57934f50744355a49946e63816407ea101",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#126)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.28.1 to 1.29.0.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.28.1...v1.29.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-19T09:45:54+01:00",
          "tree_id": "8b765dec072ce86f1a38db08ea30b73b72fd9f39",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/98ff7f57934f50744355a49946e63816407ea101"
        },
        "date": 1671439939120,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6787,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4258573,
            "range": "± 16116",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149148,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167563,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34524,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169908,
            "range": "± 9351",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8487,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12676,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40502,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 174,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b258a6bf53d02a10363e9edd2ebda918a99426c3",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#125)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 10.0.2 to 10.0.3.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v10.0.2...v10.0.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-19T09:47:12+01:00",
          "tree_id": "c461bdc20c5087d5430572ec9d83c6e57e5a620e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b258a6bf53d02a10363e9edd2ebda918a99426c3"
        },
        "date": 1671440007266,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6405,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3753227,
            "range": "± 16174",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 133580,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 147995,
            "range": "± 721",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 30057,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 149838,
            "range": "± 756",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7495,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11203,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 36714,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 152,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5d1f76f3d7ab9e602ff6e411b3733a92ef763e24",
          "message": "chore(npm)(deps-dev): bump css-loader from 6.7.2 to 6.7.3 in /wnfs-wasm (#124)\n\nBumps [css-loader](https://github.com/webpack-contrib/css-loader) from 6.7.2 to 6.7.3.\r\n- [Release notes](https://github.com/webpack-contrib/css-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/css-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/css-loader/compare/v6.7.2...v6.7.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: css-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-12-19T10:11:38+01:00",
          "tree_id": "02de4970ba167afc4d61fd129fb81ee0f7c36252",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5d1f76f3d7ab9e602ff6e411b3733a92ef763e24"
        },
        "date": 1671441275751,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6566,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4429954,
            "range": "± 16574",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152206,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169671,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35086,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169396,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7677,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9595,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41335,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 161,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bd5d6291df7f180a981e305248ea771f0d0f5ad4",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#129)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 10.0.3 to 11.0.0.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v10.0.3...v11.0.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-03T11:44:40+01:00",
          "tree_id": "d122f31b22482c79c6996ebff907e37e53fb4b6b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/bd5d6291df7f180a981e305248ea771f0d0f5ad4"
        },
        "date": 1672742938546,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6685,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4398256,
            "range": "± 16914",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152815,
            "range": "± 5112",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169803,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35320,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169392,
            "range": "± 391",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7833,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9616,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41495,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 158,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "60d4c24abfd371a4683c666a013045244469dff1",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#128)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.29.0 to 1.29.1.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.29.0...v1.29.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-03T11:45:09+01:00",
          "tree_id": "06bac0fd5c67988159bf237d67a12ba1e56d9d95",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/60d4c24abfd371a4683c666a013045244469dff1"
        },
        "date": 1672742964529,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7113,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4200555,
            "range": "± 17877",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 147968,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 165735,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34353,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169832,
            "range": "± 333",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8117,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12627,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40558,
            "range": "± 198",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 175,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "883b3ab7f9c0ec4c086e83afe7f0510c448f6bbb",
          "message": "feat(hamt): diff and merge implementation (#94)\n\n* Implement node diff and merge\r\n\r\n- lean diff method that focuses on the tree\r\n- exhaustive diff that holds a copy of changed key value pairs\r\n\r\n* Remove unnecessary spacing\r\n\r\n* Add unittests and merge impl\r\n\r\n* Fix HAMT PartialEq issue\r\n\r\n* Basic proptests\r\n\r\n- Add some docs\r\n- Not satisfied with the proptests yet\r\n\r\n* Add more proptests\r\n\r\n* Fix `get_node_at`, ...\r\n\r\n- Implement merge for `Node<k, V, H>`\r\n- Add more proptests, unittests and docs\r\n\r\n* Fix proptests\r\n\r\n- Remove hashbrown crate\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n* Fix benches\r\n\r\n* Fix tests and so on\r\n\r\n- Simplify tests\r\n- UnwrapOrClone trait\r\n- Prefer once_cell\r\n- Remove depth param from diff\r\n\r\n* Remove version checks and so on\r\n\r\n- Remove version checks in hamt\r\n- CHange HashKey to HashPrefix",
          "timestamp": "2023-01-06T19:03:13+01:00",
          "tree_id": "03f73e560d03af04974417681b13a9284fb45b81",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/883b3ab7f9c0ec4c086e83afe7f0510c448f6bbb"
        },
        "date": 1673028497590,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8776,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5046544,
            "range": "± 49754",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176622,
            "range": "± 3306",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197837,
            "range": "± 2804",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40398,
            "range": "± 636",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 199202,
            "range": "± 3115",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9732,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14113,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48478,
            "range": "± 358",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 333,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dca88c7688176f4555165b22247bdf8890d28b4e",
          "message": "chore: release main (#131)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-09T20:39:45+01:00",
          "tree_id": "7a97ced9611c818780e4bbd67c3505d5a4b87acd",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/dca88c7688176f4555165b22247bdf8890d28b4e"
        },
        "date": 1673293426934,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9580,
            "range": "± 751",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5352454,
            "range": "± 410608",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 191287,
            "range": "± 12894",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 213706,
            "range": "± 14748",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 45113,
            "range": "± 2979",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 210327,
            "range": "± 9811",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 16827,
            "range": "± 664",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12959,
            "range": "± 796",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 53991,
            "range": "± 2345",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 226,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "22b50c1f1b291b663ac6425574fd7d97643fb9ec",
          "message": "refactor: success check (#134)",
          "timestamp": "2023-01-09T15:49:23-05:00",
          "tree_id": "fef0ceac9a60cfe1a365dfb599a59d55ffa2c43d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/22b50c1f1b291b663ac6425574fd7d97643fb9ec"
        },
        "date": 1673297551833,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7069,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4242028,
            "range": "± 16607",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149593,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167919,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34460,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170064,
            "range": "± 233",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7767,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9390,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39082,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 182,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e38d039d3886f8590e00c7f87a530ca207f8a713",
          "message": "feat: private backpointer (#90)\n\n* Encrypt the ratchet key\r\n\r\n* Rename `RatchetKey` to `RevisionKey`\r\n\r\n* Fix wasm crate\r\n\r\n* Rename `*Serde` into `*Serializable`\r\n\r\n* Add `previous` backlinks to `PrivateDirectory` schema\r\n\r\n* Remember persisted CID in PrivateDirectory\r\n\r\nAnd store previous CID in previous links.\r\n\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>\r\n\r\n* Fixed not resetting `persisted_as` correctly\r\n\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>\r\n\r\n* Also implement `prepare_next_revision` for private files\r\n\r\n* Add TODOs for fixing up serialization\r\n\r\n* Make use of `let-else` :sparkles: :lipstick:\r\n\r\n* Store `previous` in `PrivateNodeHistory`.\r\n\r\n* Resolve bias in previous iterator towards `previous` nodes\r\n\r\n* Add docs & test\r\n\r\n* Also rotate the `inumber` when re-attaching trees\r\n\r\n* fix: Private shard block labels according to spec\r\n\r\nhttps://github.com/wnfs-wg/spec/blob/main/spec/private-wnfs.md#44-sharded-file-content-access\r\n\r\n* chore: Remove some logging, reduce diff\r\n\r\n* Documentation for `Encrypted`\r\n\r\n* Try to align on `impl BlockStore` and `impl RngCore`\r\n\r\n* Take a reference (fix incorrect manual merge)\r\n\r\n* Force a patch version for chrono\r\n\r\nCo-authored-by: Stephen <appcypher@users.noreply.github.com>",
          "timestamp": "2023-01-10T10:32:45+01:00",
          "tree_id": "4dd88aab4b49b6fd9a791d791312eaa7e9a99e8f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e38d039d3886f8590e00c7f87a530ca207f8a713"
        },
        "date": 1673343356943,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6751,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4448606,
            "range": "± 17178",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151917,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170275,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35303,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170739,
            "range": "± 539",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7801,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9579,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44892,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 161,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b70c31209a8371777d9539e6b6d437128d01a280",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#132)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.29.1 to 1.29.2.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.29.1...v1.29.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-10T11:25:22+01:00",
          "tree_id": "e9978ff21c40e648b13087ed55b1efa68473d7e2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b70c31209a8371777d9539e6b6d437128d01a280"
        },
        "date": 1673346499554,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6451,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4443789,
            "range": "± 17333",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150870,
            "range": "± 279",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170997,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34602,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170869,
            "range": "± 796",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7827,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9579,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44974,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 160,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "icid.asset@gmail.com",
            "name": "Steven Vandevelde",
            "username": "icidasset"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f02658b07b84e391a0984046d4e2fc4b949056a1",
          "message": "feat: Add as_file and is_file to PrivateNode (wasm) (#136)",
          "timestamp": "2023-01-10T14:38:16+01:00",
          "tree_id": "8cc2a85bef2144c2c7d7d06893a842c1be54cc54",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f02658b07b84e391a0984046d4e2fc4b949056a1"
        },
        "date": 1673358139839,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8843,
            "range": "± 661",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4790262,
            "range": "± 236591",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 171355,
            "range": "± 10649",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193156,
            "range": "± 7321",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40312,
            "range": "± 2801",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 192873,
            "range": "± 12565",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12584,
            "range": "± 850",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12091,
            "range": "± 644",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 51667,
            "range": "± 3820",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 318,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6969c088b71a95d6a4902ac246301d09c029db56",
          "message": "Fix calculations avoiding to exceed the 2^18 bytes block limit (#137)\n\n* Error out when exceeding maximum block size in MemoryBlockStore\r\n\r\n* fix: calculation to not exceed block limit",
          "timestamp": "2023-01-11T10:07:39+01:00",
          "tree_id": "c99999dd545fc84d9605dc685c8d10f2dc42927b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6969c088b71a95d6a4902ac246301d09c029db56"
        },
        "date": 1673428396870,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7984,
            "range": "± 332",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4945925,
            "range": "± 151171",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174714,
            "range": "± 5048",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 199166,
            "range": "± 2730",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40484,
            "range": "± 654",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 194397,
            "range": "± 7118",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8939,
            "range": "± 321",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11217,
            "range": "± 660",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 51833,
            "range": "± 1366",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 202,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2234b1157ead117bdecc776f1e4425fa5dcdfca6",
          "message": "chore: release main (#135)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-11T11:27:55+01:00",
          "tree_id": "11050c95e52243f6f0aa776774fa493d425884a4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2234b1157ead117bdecc776f1e4425fa5dcdfca6"
        },
        "date": 1673433067395,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9062,
            "range": "± 575",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5003986,
            "range": "± 359446",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 179819,
            "range": "± 15126",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197889,
            "range": "± 9317",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42859,
            "range": "± 4157",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 205543,
            "range": "± 12928",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 15664,
            "range": "± 1085",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12427,
            "range": "± 919",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 54403,
            "range": "± 37453",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 214,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "228d326291926c7e4b593ef66ebb089ce220dacb",
          "message": "feat(api): self lookup & store at construction (#138)\n\n* Add privateref serialization example\r\n\r\n* Empty path segment means self lookup\r\n\r\n* Add constructor with store\r\n\r\n* Update instructions\r\n\r\n* Minor fix\r\n\r\n* Fix typo\r\n\r\n* Add more re-exports and tidy debug output\r\n\r\n* Fix compilation errors\r\n\r\n* Expose search_latest and change constructor names\r\n\r\n* Minor rename",
          "timestamp": "2023-01-12T15:14:26+01:00",
          "tree_id": "0e2c6938296ccb484c39f666b4f6e47a05975dab",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/228d326291926c7e4b593ef66ebb089ce220dacb"
        },
        "date": 1673533649245,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7033,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4411001,
            "range": "± 17402",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152005,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 171760,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35520,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171560,
            "range": "± 333",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7807,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9617,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44679,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 134,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "758d4b41ab8922aeb4a49d1bbd6124eb0d2e764d",
          "message": "chore: release main (#139)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-12T16:50:31+01:00",
          "tree_id": "8c3f50178fa9156306655d7aa8e7978ceb34a2eb",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/758d4b41ab8922aeb4a49d1bbd6124eb0d2e764d"
        },
        "date": 1673538824989,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6947,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4417241,
            "range": "± 26016",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153011,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169982,
            "range": "± 486",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35268,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170147,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7828,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9623,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44733,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 129,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeeshan@fission.codes",
            "name": "Zeeshan Lakhani",
            "username": "zeeshanlakhani"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c2b1c7b1aa4a8097615b100f3d6e9832b7357378",
          "message": "chore: for dispatch (#140)",
          "timestamp": "2023-01-12T12:53:01-05:00",
          "tree_id": "2d70d5fc7e985f95b8cd7cfb5c5fac7c0e88ccc2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c2b1c7b1aa4a8097615b100f3d6e9832b7357378"
        },
        "date": 1673546157205,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7112,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4389761,
            "range": "± 18856",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152662,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170987,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35093,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169641,
            "range": "± 1429",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7827,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9600,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44816,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 128,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fb5893abce4895003cb8b42886fcf2baa8388e35",
          "message": "Fix metadata being passed out to JS as a float instead of a `BigInt` (#141)\n\n* test: Add metadata tests for public fs\r\n\r\n* fix: transfer i128 as f64 instead of i64 to JS\r\n\r\nCo-authored-by: Steven Vandevelde <icid.asset@gmail.com>",
          "timestamp": "2023-01-13T18:50:16+01:00",
          "tree_id": "48479ff94caf55000979c62a8f045de9fd14ca4b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/fb5893abce4895003cb8b42886fcf2baa8388e35"
        },
        "date": 1673632412673,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8449,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5013211,
            "range": "± 20721",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 178933,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 200446,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40953,
            "range": "± 710",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 202461,
            "range": "± 623",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9395,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11347,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 52910,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 206,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e20059521d16c27f63ad0be1cba710e43b428d58",
          "message": "chore(rust)(deps): update multihash requirement in /wnfs (#142)\n\nUpdates the requirements on [multihash](https://github.com/multiformats/rust-multihash) to permit the latest version.\r\n- [Release notes](https://github.com/multiformats/rust-multihash/releases)\r\n- [Changelog](https://github.com/multiformats/rust-multihash/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/rust-multihash/compare/v0.16.0...v0.18.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multihash\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-16T15:04:22+01:00",
          "tree_id": "71221996a296c0529e29b32d24a8c23c77b03de8",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e20059521d16c27f63ad0be1cba710e43b428d58"
        },
        "date": 1673878072393,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6907,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4427849,
            "range": "± 17601",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 154093,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 171093,
            "range": "± 222",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35251,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171223,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7803,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9803,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44817,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 130,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f2fa3d9748264d68b66f90b717fccef5d137d24b",
          "message": "chore(rust)(deps): update test-strategy requirement in /wnfs (#143)\n\nUpdates the requirements on [test-strategy](https://github.com/frozenlib/test-strategy) to permit the latest version.\r\n- [Release notes](https://github.com/frozenlib/test-strategy/releases)\r\n- [Commits](https://github.com/frozenlib/test-strategy/compare/v0.2.0...v0.3.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: test-strategy\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-16T15:04:33+01:00",
          "tree_id": "3f5475e941cf59142e4bf48c86f6d47d8d0277f7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f2fa3d9748264d68b66f90b717fccef5d137d24b"
        },
        "date": 1673878086091,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6884,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4430514,
            "range": "± 16952",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153141,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170698,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35402,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171326,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7809,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9811,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44757,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 159,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "icid.asset@gmail.com",
            "name": "Steven Vandevelde",
            "username": "icidasset"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7588f69440bfec14b8959f6aecd35eb5f848dacc",
          "message": "feat(api): adds missing metadata functions for the private side (#144)\n\n* feat: Add missing PrivateFile metadata functions\r\n\r\n* feat: Add missing PrivateDirectory metadata functions\r\n\r\n* test: Add tests for new private metadata functions\r\n\r\n* chore: Reorganise imports",
          "timestamp": "2023-01-16T19:22:53+01:00",
          "tree_id": "64ad94a5af49e9415d1a34ca33edc0560b383e1e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7588f69440bfec14b8959f6aecd35eb5f848dacc"
        },
        "date": 1673893558485,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6842,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4436609,
            "range": "± 16990",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153134,
            "range": "± 513",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170970,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35569,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171462,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7812,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9764,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44819,
            "range": "± 155",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 130,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a05abf0ece8d69d885aebc4894a845d12d4a0f34",
          "message": "Change actions trigger glob pattern (#145)",
          "timestamp": "2023-01-16T19:28:41+01:00",
          "tree_id": "e8ef62f30c1c859704b85819bac04c0d4c61fadc",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/a05abf0ece8d69d885aebc4894a845d12d4a0f34"
        },
        "date": 1673893911797,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7154,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4433323,
            "range": "± 17093",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153147,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 171457,
            "range": "± 8292",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35728,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170628,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7809,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9809,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44737,
            "range": "± 153",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 160,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "88e9f19a69fbbb99e3ee78c831eeb520a33f0b46",
          "message": "feat(api): adds missing metadata functions for the private side (#146)",
          "timestamp": "2023-01-16T19:50:21+01:00",
          "tree_id": "e8ef62f30c1c859704b85819bac04c0d4c61fadc",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/88e9f19a69fbbb99e3ee78c831eeb520a33f0b46"
        },
        "date": 1673895203902,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6950,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4437699,
            "range": "± 17183",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153055,
            "range": "± 640",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 172122,
            "range": "± 920",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35469,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 171751,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7818,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9772,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44764,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 128,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ac7864c4a2ce4ac4de77ed34aca7251532d57829",
          "message": "chore: release main (#147)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-16T19:50:48+01:00",
          "tree_id": "a190853b5fa2922c35f8313472a6de4d9881093a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ac7864c4a2ce4ac4de77ed34aca7251532d57829"
        },
        "date": 1673895234465,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7469,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4252987,
            "range": "± 24441",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149585,
            "range": "± 901",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 168255,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34219,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170883,
            "range": "± 582",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7699,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9470,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43888,
            "range": "± 1075",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 173,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "94d512eedd94e2af5850f0da80e7ee7167535a4e",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#150)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 11.0.0 to 11.0.1.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v11.0.0...v11.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-23T10:17:05+01:00",
          "tree_id": "3e5d2e288035f0cc5b431cbed839b374c460c181",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/94d512eedd94e2af5850f0da80e7ee7167535a4e"
        },
        "date": 1674465685431,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8913,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5066558,
            "range": "± 110322",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174451,
            "range": "± 6337",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197201,
            "range": "± 6471",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40811,
            "range": "± 752",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 201174,
            "range": "± 3416",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10077,
            "range": "± 215",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13849,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 47629,
            "range": "± 928",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 323,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9ba5f0fabec14d6b89eb76c8c771c03ad7f2d08d",
          "message": "refactor: Double random (#149)\n\n* refactor: double random\r\n\r\n* Re-factor single-use fn",
          "timestamp": "2023-01-24T01:14:16+11:00",
          "tree_id": "9c9109715a343ed6ccb9f7bf61e016c9cee7d119",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9ba5f0fabec14d6b89eb76c8c771c03ad7f2d08d"
        },
        "date": 1674483489398,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7147,
            "range": "± 438",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4429393,
            "range": "± 17938",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 153483,
            "range": "± 916",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 173802,
            "range": "± 682",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 36056,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 172824,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7758,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9686,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40622,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 163,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "703d1c193e5510d14652c97567f1b2f57b878d01",
          "message": "chore: use `#!/usr/bin/env bash` (#151)\n\ninstead of `#!/bin/bash`",
          "timestamp": "2023-01-24T13:30:59+01:00",
          "tree_id": "579d264d574e5a033eeeca5b918d48f773b13946",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/703d1c193e5510d14652c97567f1b2f57b878d01"
        },
        "date": 1674563707513,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6844,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4470788,
            "range": "± 17391",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 152481,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170771,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35402,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 170345,
            "range": "± 353",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7747,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9674,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40564,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 163,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "81407686e4f18613af16480f2ff8f459d907a783",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#155)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.29.2 to 1.30.0.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.29.2...v1.30.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-16T17:04:04+01:00",
          "tree_id": "868cd8ac0c4cc195a0ec6cabf086b74896d5c7d1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/81407686e4f18613af16480f2ff8f459d907a783"
        },
        "date": 1676563779643,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6865,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4406815,
            "range": "± 16243",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150961,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167572,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33865,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 167641,
            "range": "± 506",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7564,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9936,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41663,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 137,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8ce52744ad168ee28b9dc7b26020e67a01ddea21",
          "message": "chore(ci)(deps): bump katyo/publish-crates from 1 to 2 (#154)\n\nBumps [katyo/publish-crates](https://github.com/katyo/publish-crates) from 1 to 2.\r\n- [Release notes](https://github.com/katyo/publish-crates/releases)\r\n- [Commits](https://github.com/katyo/publish-crates/compare/v1...v2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: katyo/publish-crates\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-16T17:03:39+01:00",
          "tree_id": "307aa53e3fa349afe8d56fb389bf02c90a6337c1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8ce52744ad168ee28b9dc7b26020e67a01ddea21"
        },
        "date": 1676563813402,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9102,
            "range": "± 440",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5112323,
            "range": "± 241512",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 185017,
            "range": "± 9236",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 206896,
            "range": "± 7579",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44412,
            "range": "± 1728",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 210865,
            "range": "± 10248",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 16305,
            "range": "± 1107",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13902,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 59402,
            "range": "± 2313",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 248,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2",
          "message": "feat(private): shared private data (#148)\n\n* Add initial structure\r\n\r\n* Implement sharing\r\n\r\n* Fix sharing impl and add tests\r\n\r\n* Push wasm changes\r\n\r\n* Fix code\r\n\r\n* Fix lint error\r\n\r\n* Take u32 share counts from js",
          "timestamp": "2023-02-16T18:01:15+01:00",
          "tree_id": "020e5b6d988daebf5a19a17350598f8913f4c450",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2"
        },
        "date": 1676567055212,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7268,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4367731,
            "range": "± 17769",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149509,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 168886,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33751,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168650,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7636,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9664,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42235,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 160,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "17ad5f5ef8ba61d5cf2fa2fec3af14a545db125c",
          "message": "chore: release main (#166)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-17T13:13:56+01:00",
          "tree_id": "b33543951429d925de2431f016b5d807693accf5",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/17ad5f5ef8ba61d5cf2fa2fec3af14a545db125c"
        },
        "date": 1676636217454,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6484,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4390473,
            "range": "± 17563",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150714,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 168689,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34599,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 169001,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7624,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9538,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42672,
            "range": "± 375",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 159,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a3aa25b2a04ed0a0847d5da668ecc6a9c2ff06ba",
          "message": "chore(rust)(deps): update libipld requirement from 0.15 to 0.16 in /wnfs (#157)\n\n* chore(rust)(deps): update libipld requirement from 0.15 to 0.16 in /wnfs\r\n\r\nUpdates the requirements on [libipld](https://github.com/ipld/libipld) to permit the latest version.\r\n- [Release notes](https://github.com/ipld/libipld/releases)\r\n- [Commits](https://github.com/ipld/libipld/compare/libipld-pb-v0.15.0...libipld-pb-v0.16.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: libipld\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* Fix libipld deps\r\n\r\n* Fix deps\r\n\r\n---------\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-02-17T13:43:02+01:00",
          "tree_id": "185a0f805524abf845f08fe8a2e582cd59ed253a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/a3aa25b2a04ed0a0847d5da668ecc6a9c2ff06ba"
        },
        "date": 1676637969447,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6938,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4357399,
            "range": "± 17249",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150698,
            "range": "± 3292",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167753,
            "range": "± 1384",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33990,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168329,
            "range": "± 495",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7542,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9710,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41217,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 155,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c96c66a8281d6f82e489dcc7b1c9a470cc6a7601",
          "message": "chore(npm)(deps-dev): bump typescript from 4.9.4 to 4.9.5 in /wnfs-wasm (#160)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 4.9.4 to 4.9.5.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v4.9.4...v4.9.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-17T15:45:13+01:00",
          "tree_id": "6e4a5a00df111ebb43aebf4be060f6dab694ac6f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c96c66a8281d6f82e489dcc7b1c9a470cc6a7601"
        },
        "date": 1676645283707,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6900,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4184249,
            "range": "± 17884",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150074,
            "range": "± 593",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166692,
            "range": "± 731",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33541,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168330,
            "range": "± 337",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8519,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11726,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40537,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 290,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "95cba5340bd85a1a70d8a1934876d555e9f69f54",
          "message": "Implement non-nested encryption (#159)\n\n* Add initial structure\r\n\r\n* Implement sharing\r\n\r\n* refactor: Remove `content_key` from `PrivateRef`\r\n\r\nIt can always be derived from `revision_key`.\r\nStoring it will only make it possible for `revision_key` and\r\n`content_key` to get out-of-sync.\r\n\r\n* refactor: Extract out `PrivateDirectoryContent`\r\n\r\n* Key-wrap using AES-KWP\r\n\r\n* refactor: use AES-KWP for encrypting previous links\r\n\r\n* refactor: Previous to be a set of encrypted Cids\r\n\r\ninstead of an encrypted set of Cids\r\n\r\n* refactor: Add \"# of revisions back\" to backpointers\r\n\r\n* refactor: Extract private file content into struct\r\n\r\n* refactor: Move AES-KWP logic into `RevisionKey`\r\n\r\n* refactor: Move AES-GCM logic into `ContentKey`\r\n\r\n* chore: Add TODO comments for missing docs\r\n\r\n* fix: doctests due to refactor (whoops)\r\n\r\n* refactor: remove `KeyType` struct\r\n\r\n* refactor: Implement `load` & `store` for PNH\r\n\r\n* refactor: Split header & content, add disambiguation\r\n\r\nSo:\r\n- PrivateNodeHeader gets its own block\r\n- PrivateFile and PrivateDirectory refer back to the header via a CID\r\n- PrivateRef gets its own \"disambiguation pointer\" content_cid\r\n- PrivateForest now resolves PrivateRefs\r\n- PrivateRefs always refer to pre-existing content, never to \"open slots\"\r\n\r\n* refactor: Introduce `RevisionRef` & fix examples\r\n\r\n* refactor: Adjust doctests :white_check_mark:\r\n\r\n* refactor: remove `.derive_private_ref()`\r\n\r\n* refactor: Simplify `SharePointer` creation\r\n\r\n* refactor: Remove `Version` from private dir content\r\n\r\n* refactor: move `persisted_as` into dir content\r\n\r\n* refactor: Move `persisted_as` into private file content\r\n\r\n* refactor: Docs & more\r\n\r\n* clippy: configure to ignore `Encrypted` wrapper\r\n\r\n* refactor: Rename to `TemporalKey` & `SnapshotKey`\r\n\r\ninstead of `RevisionKey` and `ContentKey`, respectively.\r\n\r\n* refactor: Use `&mut Rc<PrivateForest>` and similar (#161)\r\n\r\nAlso, make use of `Rc::make_mut`, accordingly.\r\n\r\n* refactor: Some fixes for wasm\r\n\r\n* feat: Refactor & add stuff to the wasm interface\r\n\r\n* feat: Allow wasm extracting values out of `PrivateRef`\r\n\r\n* fix: Small fix in doctest\r\n\r\n* fix: Serialization and deserialization of share payloads\r\n\r\n* chore: Remove logging :mute:\r\n\r\n* refactor: Just use a different tagging mechanism\r\n\r\n* refactor: Use `test_setup!` more consistently\r\n\r\n* fix: incorrectly resolved merge conflicts\r\n\r\n* fix: wasm tests. Also: Pin newest wasm-bindgen version\r\n\r\n(it had some good bugfixes regarding FinalizationRegistries recently)\r\n\r\n* fix: undo accidentally committed commenting\r\n\r\n* fix: yield back received errors\r\n\r\nThanks clippy!\r\n\r\n* refactor: Always take owned Rc wrappers in bindings\r\n\r\n* fix: Type error in wasm\r\n\r\n* refactor: Primarily take borrowed types in bindings\r\n\r\n---------\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-02-20T17:26:02+01:00",
          "tree_id": "100492fc78fb1e773b802caaf1a1fbaf7427dba5",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/95cba5340bd85a1a70d8a1934876d555e9f69f54"
        },
        "date": 1676910696577,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7209,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4206805,
            "range": "± 15779",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148521,
            "range": "± 656",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 166025,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33175,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 167326,
            "range": "± 384",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7528,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9775,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40380,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "600334fbee5be0957f9237bc70ad65e7194a27b8",
          "message": "chore: Strip flags 2 (#171)\n\n* chore: Release wasm strip synbols\r\n\r\n* chore: Continue on-error for codecov upload\r\n\r\n* Add config.toml\r\n\r\n---------\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-02-21T11:07:37+01:00",
          "tree_id": "d9d40362737d2224345a4276c8a06bf37eda26a2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/600334fbee5be0957f9237bc70ad65e7194a27b8"
        },
        "date": 1676974245916,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7304,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4224884,
            "range": "± 19382",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148442,
            "range": "± 1060",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 169787,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33292,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168442,
            "range": "± 322",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7525,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9776,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40467,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 174,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d1bbb692864b428ec69e2b49b689b543567c16ae",
          "message": "Add hamt diff and merge benches (#164)",
          "timestamp": "2023-02-21T13:58:51+01:00",
          "tree_id": "4fe833a14db29d8001d6334254829a420c6d4c49",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d1bbb692864b428ec69e2b49b689b543567c16ae"
        },
        "date": 1676984526039,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7114,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4376407,
            "range": "± 16329",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 150069,
            "range": "± 1094",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167494,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34092,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 167243,
            "range": "± 355",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 238215,
            "range": "± 18902",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 401063,
            "range": "± 47187",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7474,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9569,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43404,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 162,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d7870bc78660458fe9c5252c551a474dcdd045f2",
          "message": "chore(exports)!: make re-exports more flexible (#167)\n\n* Make re-exports more flexible\r\n\r\n* Fix examples\r\n\r\n* Fix unrelated typos\r\n\r\n* Remove skip_ratchet re-export\r\n\r\n* Fix import",
          "timestamp": "2023-02-22T10:20:49+01:00",
          "tree_id": "dc47e139768b1226c143a1d3c0aff5dde0185f2a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d7870bc78660458fe9c5252c551a474dcdd045f2"
        },
        "date": 1677057853405,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6582,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4349783,
            "range": "± 16339",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149661,
            "range": "± 1105",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 168199,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33535,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 166649,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 235510,
            "range": "± 18618",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 399596,
            "range": "± 47191",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7471,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9581,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43306,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 161,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1bfe89bcaabdf679a5338a2c9aa97b76deb00b03",
          "message": "feat: Streaming write for PrivateFile (#163)\n\n* Add initial structure\r\n\r\n* Implement sharing\r\n\r\n* refactor: Remove `content_key` from `PrivateRef`\r\n\r\nIt can always be derived from `revision_key`.\r\nStoring it will only make it possible for `revision_key` and\r\n`content_key` to get out-of-sync.\r\n\r\n* refactor: Extract out `PrivateDirectoryContent`\r\n\r\n* Key-wrap using AES-KWP\r\n\r\n* refactor: use AES-KWP for encrypting previous links\r\n\r\n* refactor: Previous to be a set of encrypted Cids\r\n\r\ninstead of an encrypted set of Cids\r\n\r\n* refactor: Add \"# of revisions back\" to backpointers\r\n\r\n* refactor: Extract private file content into struct\r\n\r\n* refactor: Move AES-KWP logic into `RevisionKey`\r\n\r\n* refactor: Move AES-GCM logic into `ContentKey`\r\n\r\n* chore: Add TODO comments for missing docs\r\n\r\n* fix: doctests due to refactor (whoops)\r\n\r\n* refactor: remove `KeyType` struct\r\n\r\n* refactor: Implement `load` & `store` for PNH\r\n\r\n* refactor: Split header & content, add disambiguation\r\n\r\nSo:\r\n- PrivateNodeHeader gets its own block\r\n- PrivateFile and PrivateDirectory refer back to the header via a CID\r\n- PrivateRef gets its own \"disambiguation pointer\" content_cid\r\n- PrivateForest now resolves PrivateRefs\r\n- PrivateRefs always refer to pre-existing content, never to \"open slots\"\r\n\r\n* refactor: Introduce `RevisionRef` & fix examples\r\n\r\n* refactor: Adjust doctests :white_check_mark:\r\n\r\n* refactor: remove `.derive_private_ref()`\r\n\r\n* refactor: Simplify `SharePointer` creation\r\n\r\n* refactor: Remove `Version` from private dir content\r\n\r\n* refactor: move `persisted_as` into dir content\r\n\r\n* refactor: Move `persisted_as` into private file content\r\n\r\n* refactor: Docs & more\r\n\r\n* clippy: configure to ignore `Encrypted` wrapper\r\n\r\n* refactor: Rename to `TemporalKey` & `SnapshotKey`\r\n\r\ninstead of `RevisionKey` and `ContentKey`, respectively.\r\n\r\n* refactor: Use `&mut Rc<PrivateForest>` and similar (#161)\r\n\r\nAlso, make use of `Rc::make_mut`, accordingly.\r\n\r\n* refactor: Some fixes for wasm\r\n\r\n* feat: Refactor & add stuff to the wasm interface\r\n\r\n* feat: Allow wasm extracting values out of `PrivateRef`\r\n\r\n* fix: Small fix in doctest\r\n\r\n* feat: try implementing streaming write\r\n\r\nHaving an issue with it in tests though. It stack-overflows.\r\n\r\n* fix: Fix streaming write implementation\r\n\r\n* docs: Write 'em\r\n\r\n* fix: Serialization and deserialization of share payloads\r\n\r\n* chore: Remove logging :mute:\r\n\r\n* refactor: Just use a different tagging mechanism\r\n\r\n* refactor: Use `test_setup!` more consistently\r\n\r\n* fix: incorrectly resolved merge conflicts\r\n\r\n* fix: wasm tests. Also: Pin newest wasm-bindgen version\r\n\r\n(it had some good bugfixes regarding FinalizationRegistries recently)\r\n\r\n* refactor: Add a test fixture for testing streaming write\r\n\r\n* fix: :white_check_mark: Fix tests\r\n\r\n---------\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-02-22T17:14:08+01:00",
          "tree_id": "8573c3679ec5301ec90bc6bf63e3a9d963b71e74",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/1bfe89bcaabdf679a5338a2c9aa97b76deb00b03"
        },
        "date": 1677082667194,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8144,
            "range": "± 382",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4597819,
            "range": "± 254293",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 164594,
            "range": "± 8269",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 177350,
            "range": "± 9480",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 35559,
            "range": "± 2179",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 184329,
            "range": "± 9408",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 245880,
            "range": "± 22946",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 433759,
            "range": "± 52734",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8498,
            "range": "± 611",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11235,
            "range": "± 543",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45991,
            "range": "± 1925",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 210,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f75400f71aeb4245972c3a08c9967edbe8fd65a",
          "message": "refactor(hamt): refactor key value diff (#165)\n\n* Refactor key value diff\r\n\r\n* Change unwrap_or_clone code to Rc::make_mut\r\n\r\n* Remove node_diff",
          "timestamp": "2023-02-22T17:52:32+01:00",
          "tree_id": "a8b620e586909c3371d835e812f965c754433897",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5f75400f71aeb4245972c3a08c9967edbe8fd65a"
        },
        "date": 1677084972952,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9122,
            "range": "± 547",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5025152,
            "range": "± 318315",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 184941,
            "range": "± 10496",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 205856,
            "range": "± 9529",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 43068,
            "range": "± 1887",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 198138,
            "range": "± 8314",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 64674,
            "range": "± 7434",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 269054,
            "range": "± 39057",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12910,
            "range": "± 815",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13039,
            "range": "± 663",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50543,
            "range": "± 3523",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 225,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b0d6790af8639690a0b3ef7f2321ffd5c4cc25dc",
          "message": "chore(api): trigger re-exports release (#176)",
          "timestamp": "2023-02-22T17:54:22+01:00",
          "tree_id": "a8b620e586909c3371d835e812f965c754433897",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b0d6790af8639690a0b3ef7f2321ffd5c4cc25dc"
        },
        "date": 1677085070840,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7000,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4343651,
            "range": "± 16692",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151980,
            "range": "± 1102",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170300,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34200,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 168763,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58636,
            "range": "± 4701",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 235549,
            "range": "± 18764",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7472,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9574,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43313,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 162,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "89e4d36dc9b27ec1ab67db6fc214670efe768f32",
          "message": "chore: release 0.1.16 (#178)\n\nRelease-As: 0.1.16",
          "timestamp": "2023-02-22T18:20:12+01:00",
          "tree_id": "a8b620e586909c3371d835e812f965c754433897",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32"
        },
        "date": 1677086617737,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7361,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4176579,
            "range": "± 16656",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148452,
            "range": "± 4243",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 165163,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 33191,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 167727,
            "range": "± 279",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54548,
            "range": "± 4413",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 222859,
            "range": "± 17778",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7540,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9768,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40381,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 174,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9f7095db704db8442aa3e1b6ed22b8c57f128e6b",
          "message": "chore: release main (#179)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-22T18:34:47+01:00",
          "tree_id": "a4f9817f0501821587b9bc0201224f183a2a5f44",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9f7095db704db8442aa3e1b6ed22b8c57f128e6b"
        },
        "date": 1677087502044,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7030,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4340601,
            "range": "± 16286",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148488,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167891,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34264,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 166302,
            "range": "± 424",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58186,
            "range": "± 4626",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231236,
            "range": "± 18430",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7480,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10050,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41148,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 168,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "985a9262c4c668aea15cf13a395cc6e70fb3bcfe",
          "message": "chore: Allow clippy versions <1.67 (#180)",
          "timestamp": "2023-02-23T09:53:47+01:00",
          "tree_id": "1a36481b66084224eb47d599e9770b0aa6a23d3a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/985a9262c4c668aea15cf13a395cc6e70fb3bcfe"
        },
        "date": 1677142694170,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8931,
            "range": "± 367",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5135436,
            "range": "± 271979",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 186061,
            "range": "± 13745",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 206693,
            "range": "± 8505",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44481,
            "range": "± 2394",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 207264,
            "range": "± 12593",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 68408,
            "range": "± 11904",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 277381,
            "range": "± 37348",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13526,
            "range": "± 999",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12834,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 55993,
            "range": "± 2681",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 223,
            "range": "± 544",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f04fa89738e19a095d177e18b35d7e153c380833",
          "message": "feat: PrivateLink abstraction (#172)\n\n* refactor: Implement `PrivateLink`\r\n\r\n* feature: Implement `PrivateLink`\r\n\r\n* fix: :white_check_mark: on all non-doctests\r\n\r\n* fix: One doc test type error\r\n\r\n* fix: :white_check_mark: Fix all doctests\r\n\r\n* refactor: Expose `dir.store` instead of `forest.put`\r\n\r\n* refactor: Update wasm APIs accordingly\r\n\r\n* refactor: Relax parameter requirements\r\n\r\n* chore: de-noise diff\r\n\r\n* refactor: generally make use of `dagcbor` util module\r\n\r\n* chore: Polish\r\n\r\n* refactor: `forest.get` -> `PrivateNode::load`\r\n\r\n* refactor: Re-use `PrivateNode::store()` in wasm\r\n\r\n* refactor: Better `Aes256Gcm` initialization",
          "timestamp": "2023-02-23T12:04:38+01:00",
          "tree_id": "af5b909bca6c04176536e5f6120a963cdc4c10f5",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f04fa89738e19a095d177e18b35d7e153c380833"
        },
        "date": 1677150532569,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6579,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4349483,
            "range": "± 16984",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175998,
            "range": "± 1053",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 192287,
            "range": "± 562",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51733,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 172820,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58463,
            "range": "± 4595",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231141,
            "range": "± 18504",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7475,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9834,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41209,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 167,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "806bbb93b1f03983165375005e14a9b63ebe67c2",
          "message": "feat: Remove `base_history_on` and auto-track history instead (#174)\n\n* refactor: Implement `PrivateLink`\r\n\r\n* feature: Implement `PrivateLink`\r\n\r\n* fix: :white_check_mark: on all non-doctests\r\n\r\n* fix: One doc test type error\r\n\r\n* fix: :white_check_mark: Fix all doctests\r\n\r\n* refactor: Expose `dir.store` instead of `forest.put`\r\n\r\n* refactor: Update wasm APIs accordingly\r\n\r\n* refactor: Relax parameter requirements\r\n\r\n* chore: de-noise diff\r\n\r\n* refactor: generally make use of `dagcbor` util module\r\n\r\n* chore: Polish\r\n\r\n* refactor: `forest.get` -> `PrivateNode::load`\r\n\r\n* refactor: Re-use `PrivateNode::store()` in wasm\r\n\r\n* refactor: Move out `OnceCell<Cid>` from `Link`\r\n\r\n* refactor: Remove `version` from in-memory struct\r\n\r\n* feat: Remove `base_history_on` and auto-track history\r\n\r\n* refactor: Remove `baseHistoryOn` from Wasm API\r\n\r\n* refactor: Some polish. Remove `get_remembering_persistence`\r\n\r\n* refactor: Simplify previous link test\r\n\r\n* test: Test public file's previous links.\r\n\r\n* refactor: Rename into `RemembersCid`\r\n\r\n* refactor: Rename to `RemembersCid` in docs\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n* test: More tests covering `prepare_next_revision`\r\n\r\n---------\r\n\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-02-24T15:14:36+01:00",
          "tree_id": "bdfe61f0847ea657f3e586e5dd970d5954733c37",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/806bbb93b1f03983165375005e14a9b63ebe67c2"
        },
        "date": 1677248319623,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7447,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4409858,
            "range": "± 17139",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177053,
            "range": "± 1205",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 199783,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48709,
            "range": "± 171",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176969,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56531,
            "range": "± 4542",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 236373,
            "range": "± 18934",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8542,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11752,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40722,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 177,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "231ece4309cab86d4682693e8e31f8ed99478a1f",
          "message": "feat(api): add privateforest merge and diff bindings (#181)\n\n* Add privateforest merge and diff bindings\r\n\r\n* Fix imports",
          "timestamp": "2023-02-24T16:39:20+01:00",
          "tree_id": "cafaee67dd15f457b1eae6708ccb6e9ac82251c8",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/231ece4309cab86d4682693e8e31f8ed99478a1f"
        },
        "date": 1677253434375,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8581,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5272303,
            "range": "± 26620",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 212828,
            "range": "± 695",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 235451,
            "range": "± 3168",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 58281,
            "range": "± 1379",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 213812,
            "range": "± 1117",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 66305,
            "range": "± 5510",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 281335,
            "range": "± 22972",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10225,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14063,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48438,
            "range": "± 239",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 211,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b040c2f9f34363653fe1d025c805571fecd769d0",
          "message": "chore: release main (#188)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-02-24T16:54:30+01:00",
          "tree_id": "e0cb6421e15b53b6b25c91a281c294fc29349817",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b040c2f9f34363653fe1d025c805571fecd769d0"
        },
        "date": 1677254310337,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6614,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4589079,
            "range": "± 32074",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 180999,
            "range": "± 1570",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 202387,
            "range": "± 349",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 52637,
            "range": "± 140",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176891,
            "range": "± 784",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59545,
            "range": "± 4732",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243564,
            "range": "± 19420",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7477,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9595,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41210,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 166,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d36b2d8207eabef7590f1ba7f875c11984b1deb",
          "message": "refactor: separate into crates (#184)\n\n* Separate into crates\r\n\r\n* Add todos\r\n\r\n* Fix doc tests\r\n\r\n* Format code\r\n\r\n* Fix bench\r\n\r\n* Remove redundant license files\r\n\r\n* Fix doc links\r\n\r\n* Add minor doc and rename `make_digest`\r\n\r\n* Remove wnfs-wasm license file",
          "timestamp": "2023-02-27T16:27:50+01:00",
          "tree_id": "16f5dad8fd6efa4deb137e9c4f97eb6d2d652b4b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0d36b2d8207eabef7590f1ba7f875c11984b1deb"
        },
        "date": 1677512028662,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7663,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4438237,
            "range": "± 16246",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177339,
            "range": "± 471",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197998,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48468,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180005,
            "range": "± 462",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56362,
            "range": "± 4606",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 235614,
            "range": "± 18859",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7570,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9441,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40480,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 278,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6c95ca279af642777e2725939c79a205ad196076",
          "message": "chore: Redundant rsa (#194)",
          "timestamp": "2023-02-28T09:10:44+01:00",
          "tree_id": "577fd8fdb16e1df66eb60bf25b36d233749a7ade",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6c95ca279af642777e2725939c79a205ad196076"
        },
        "date": 1677572062301,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9063,
            "range": "± 588",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5286640,
            "range": "± 271007",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 223519,
            "range": "± 14100",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 248405,
            "range": "± 11764",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 63518,
            "range": "± 7998",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 213044,
            "range": "± 12341",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 65355,
            "range": "± 9349",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 275327,
            "range": "± 26426",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13027,
            "range": "± 1075",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13606,
            "range": "± 847",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 55480,
            "range": "± 1903",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 347,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9c55a116328c870773f5701cdbdfc1edd76238ef",
          "message": "chore: Redundant bitvec (#193)",
          "timestamp": "2023-02-28T09:11:06+01:00",
          "tree_id": "679e1160c7526a398c35a39c8fb9e16a3abfdba9",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9c55a116328c870773f5701cdbdfc1edd76238ef"
        },
        "date": 1677572065283,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9207,
            "range": "± 719",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5255760,
            "range": "± 320823",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 217157,
            "range": "± 14471",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 229627,
            "range": "± 14263",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 60282,
            "range": "± 3148",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 211590,
            "range": "± 9835",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 62146,
            "range": "± 11697",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 271236,
            "range": "± 29069",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12357,
            "range": "± 784",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12996,
            "range": "± 715",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 52053,
            "range": "± 2712",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 163,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e2174369d1168576ba9ef08c4654f640fe3ea4f4",
          "message": "chore: Redundant either (#192)",
          "timestamp": "2023-02-28T09:11:40+01:00",
          "tree_id": "c56efc9608023b7ca3f603a9be2f4565df18b2d2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e2174369d1168576ba9ef08c4654f640fe3ea4f4"
        },
        "date": 1677572095778,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7250,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4624977,
            "range": "± 18352",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 187584,
            "range": "± 989",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 202808,
            "range": "± 9196",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51918,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178423,
            "range": "± 693",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59814,
            "range": "± 4676",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 244398,
            "range": "± 19715",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8052,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9860,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50412,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 162,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "231f4e929378d7a02c9f7f8b095f1c2b1175ec2e",
          "message": "feat: Redundant sha2 (#191)",
          "timestamp": "2023-02-28T09:13:28+01:00",
          "tree_id": "d0ca3bdf01bf8a02664c973b8550c278d573f443",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/231f4e929378d7a02c9f7f8b095f1c2b1175ec2e"
        },
        "date": 1677572432711,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7767,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4447571,
            "range": "± 16614",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176517,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 198443,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48328,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180127,
            "range": "± 466",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56278,
            "range": "± 4643",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 236128,
            "range": "± 18929",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8617,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11718,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41626,
            "range": "± 1065",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "683d8e40c91fbe2b66a31c05f770ab07c9d80b07",
          "message": "chore: Redundant futures-util manifest (#190)",
          "timestamp": "2023-02-28T09:14:12+01:00",
          "tree_id": "95b3ef03b71d71c10acbf20786818aeb6ebec0c0",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/683d8e40c91fbe2b66a31c05f770ab07c9d80b07"
        },
        "date": 1677572505531,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7452,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4343259,
            "range": "± 19217",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175387,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197987,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48703,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178779,
            "range": "± 395",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54661,
            "range": "± 4412",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 232959,
            "range": "± 18589",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8611,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11700,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41641,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 281,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "36498018+pinkforest@users.noreply.github.com",
            "name": "pinkforest(she/her)",
            "username": "pinkforest"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "12cd8428514d7c145b443a78e279dc468fa01a91",
          "message": "feat: Make log optional (#189)",
          "timestamp": "2023-02-28T09:53:19+01:00",
          "tree_id": "fb86e602762a5a1b71b78a8770b7f0001595a033",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/12cd8428514d7c145b443a78e279dc468fa01a91"
        },
        "date": 1677574592729,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7777,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4365277,
            "range": "± 17559",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175934,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197196,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48929,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 177299,
            "range": "± 544",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54731,
            "range": "± 4488",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 232607,
            "range": "± 18609",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7545,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9405,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40568,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 279,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "69ffeeca20cc3106e6d733e2d5adf5f87987630c",
          "message": "fix: `find_latest_share_counter` finds the last share count (#197)\n\n* fix: `find_latest_share_count` finds the last share count\r\n\r\n* refactor: Also rename func in generated TS\r\n\r\n* fix: Correct imports in wasm tests\r\n\r\n* refactor: Readability",
          "timestamp": "2023-03-02T16:47:52+01:00",
          "tree_id": "21f32445ca77f83ebfac28d675e71665f6d37882",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/69ffeeca20cc3106e6d733e2d5adf5f87987630c"
        },
        "date": 1677772309688,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6910,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4559316,
            "range": "± 16479",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 187040,
            "range": "± 2517",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 202642,
            "range": "± 469",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 52308,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178422,
            "range": "± 585",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60009,
            "range": "± 4814",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 244308,
            "range": "± 19690",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8084,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10080,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42902,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 164,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0726da2d2ec3f94880d459b5422401377f47d0fd",
          "message": "refactor: dedup path and unused impls (#198)\n\n* Dedup import paths and remove unused impls\r\n\r\n* Fix doc test and remove other duplicates",
          "timestamp": "2023-03-02T17:07:32+01:00",
          "tree_id": "fd6bd634c646d25f8de1f8a3f2f62cebfd5090f7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0726da2d2ec3f94880d459b5422401377f47d0fd"
        },
        "date": 1677773483367,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7061,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4332497,
            "range": "± 23541",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176858,
            "range": "± 3190",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197534,
            "range": "± 266",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49123,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178198,
            "range": "± 434",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54205,
            "range": "± 4452",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231820,
            "range": "± 18717",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7579,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9498,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40436,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 175,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c9f1423c5550a4ce09469002731ce55c579edce",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#200)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.30.0 to 1.31.2.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.30.0...v1.31.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-09T14:16:53+01:00",
          "tree_id": "b471fcfe01eac7e1ee84354c1fd54258a827ead0",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8c9f1423c5550a4ce09469002731ce55c579edce"
        },
        "date": 1678368118638,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7871,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4402958,
            "range": "± 19247",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 177358,
            "range": "± 7304",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 198533,
            "range": "± 784",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48527,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178733,
            "range": "± 417",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55972,
            "range": "± 4534",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 234053,
            "range": "± 18873",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8492,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11747,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41200,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 173,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7e31e0a3761657250f3d0e7b98dd409ed6f86d8f",
          "message": "refactor(api): refactor out `PublicOpResult` (#201)\n\n* Dedup import paths and remove unused impls\r\n\r\n* Refactor out PublicOpResult\r\n\r\n* Fix wasm binding code\r\n\r\n* Fix lint",
          "timestamp": "2023-03-14T15:56:28+01:00",
          "tree_id": "2f53412da7c3a98ad4d32a5e4a7d0a34bdaeb21e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7e31e0a3761657250f3d0e7b98dd409ed6f86d8f"
        },
        "date": 1678806191962,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7247,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4358621,
            "range": "± 17646",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176134,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197311,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48452,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178015,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54376,
            "range": "± 4386",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 232976,
            "range": "± 18478",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8591,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9979,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40448,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 170,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4ef324de45f8b986d48d79abc5f53f59f4f62611",
          "message": "chore: Use cargo-deny & fix time CVE (#208)\n\n* chore: Use cargo-deny & ignore time CVE\r\n\r\n* chore: Turn off default chrono features\r\n\r\n* chore: Configure licenses\r\n\r\n* chore: Allow wnfs-wg git sources\r\n\r\n* test: Ensure that playwright tests wait for js\r\n\r\nSince js is loaded via a script with `defer` in it, it seems to be necessary to wait for it to load.",
          "timestamp": "2023-03-21T11:28:59+01:00",
          "tree_id": "9439cd1221615c3c7a092b9058acd1d28a3b122c",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4ef324de45f8b986d48d79abc5f53f59f4f62611"
        },
        "date": 1679394858585,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10399,
            "range": "± 499",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5521577,
            "range": "± 214956",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 226001,
            "range": "± 15709",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 252027,
            "range": "± 8204",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 66842,
            "range": "± 1333",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 225177,
            "range": "± 23234",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 70370,
            "range": "± 7190",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 293321,
            "range": "± 39599",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 16656,
            "range": "± 692",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13273,
            "range": "± 578",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 56528,
            "range": "± 1610",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 232,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e6c5e475354c3775f584e05260627091f68f36e7",
          "message": "chore(npm)(deps-dev): bump webpack-dev-server in /wnfs-wasm (#206)\n\nBumps [webpack-dev-server](https://github.com/webpack/webpack-dev-server) from 4.11.1 to 4.13.1.\r\n- [Release notes](https://github.com/webpack/webpack-dev-server/releases)\r\n- [Changelog](https://github.com/webpack/webpack-dev-server/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-dev-server/compare/v4.11.1...v4.13.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-dev-server\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-21T12:13:35+01:00",
          "tree_id": "fe9665fb6acc05f964e7ac1ed389d990abd40ccb",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e6c5e475354c3775f584e05260627091f68f36e7"
        },
        "date": 1679397406530,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7717,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4343123,
            "range": "± 16589",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174485,
            "range": "± 171",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 195490,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49002,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176169,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54351,
            "range": "± 4398",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231223,
            "range": "± 18479",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7682,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9931,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40444,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 281,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "10737d2df12e07d3d52cccb0958ed9ff2b285b95",
          "message": "chore(npm)(deps-dev): bump style-loader in /wnfs-wasm (#207)\n\nBumps [style-loader](https://github.com/webpack-contrib/style-loader) from 3.3.1 to 3.3.2.\r\n- [Release notes](https://github.com/webpack-contrib/style-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/style-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/style-loader/compare/v3.3.1...v3.3.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: style-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-21T12:13:04+01:00",
          "tree_id": "1dd2e5ceb5bb712b3e0d54bbce05922a3cf80968",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/10737d2df12e07d3d52cccb0958ed9ff2b285b95"
        },
        "date": 1679397413357,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10503,
            "range": "± 628",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4764164,
            "range": "± 37437",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 183257,
            "range": "± 479",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 202810,
            "range": "± 559",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 52034,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176688,
            "range": "± 696",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61442,
            "range": "± 5310",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 247174,
            "range": "± 20070",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8268,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9755,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43083,
            "range": "± 140",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 292,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4114ce7ffe6182f97f80e405ce2651bc2d5ad87d",
          "message": "chore(npm)(deps-dev): bump webpack from 5.75.0 to 5.76.2 in /wnfs-wasm (#205)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.75.0 to 5.76.2.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.75.0...v5.76.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-21T12:14:03+01:00",
          "tree_id": "9304461acb9d8e2d4a7dc9ea7f55ee4511d5397d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4114ce7ffe6182f97f80e405ce2651bc2d5ad87d"
        },
        "date": 1679397584058,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7304,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4580037,
            "range": "± 17901",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 181237,
            "range": "± 389",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 201564,
            "range": "± 314",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51628,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176617,
            "range": "± 372",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59705,
            "range": "± 4724",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243425,
            "range": "± 19531",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8213,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9706,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43041,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 164,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b90e96441d096d2bfb61db0393cc2bf5a52786d2",
          "message": "chore(npm)(deps-dev): bump typescript from 4.9.5 to 5.0.2 in /wnfs-wasm (#204)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 4.9.5 to 5.0.2.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v4.9.5...v5.0.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-21T12:14:43+01:00",
          "tree_id": "46f53d83cff581925ad917b8d379bcc4b8eb5931",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b90e96441d096d2bfb61db0393cc2bf5a52786d2"
        },
        "date": 1679397638450,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7202,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4579087,
            "range": "± 18177",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 182639,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 200560,
            "range": "± 401",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51973,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176790,
            "range": "± 444",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59930,
            "range": "± 4713",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243193,
            "range": "± 19740",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8301,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9644,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43131,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 166,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "02a52777fb0e339685b2289f6835d7409aff32ca",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#202)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 11.0.1 to 11.0.2.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v11.0.1...v11.0.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-21T13:37:42+01:00",
          "tree_id": "f3a43226847a53c0190facbdc8708db13ef94880",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/02a52777fb0e339685b2289f6835d7409aff32ca"
        },
        "date": 1679402483334,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8130,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5213397,
            "range": "± 21221",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 211953,
            "range": "± 338",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 236409,
            "range": "± 465",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 58150,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 213385,
            "range": "± 601",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 65045,
            "range": "± 5421",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 277233,
            "range": "± 22277",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9342,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11928,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48578,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 351,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "700c09fe8416f1524f9283115daab4c3440cf03f",
          "message": "refactor(api): refactor out PrivateOpResult  (#211)\n\n* Refactor privateopresult\r\n\r\n* fix: `PrivatLink::resolve_node_mut` disassociates `PrivateRef`\r\n\r\n* chore: Add an explainer comment\r\n\r\n* Fix tests and bindings\r\n\r\n* Fix readme\r\n\r\n* Fix fmt\r\n\r\n* Remove prints and fix resolve-value-mut\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n---------\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-03-23T10:49:41+01:00",
          "tree_id": "5df9a26fcc8773f398effba007848b8a24f10a74",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/700c09fe8416f1524f9283115daab4c3440cf03f"
        },
        "date": 1679565253736,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7090,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4595038,
            "range": "± 21630",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 181272,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 204877,
            "range": "± 508",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51525,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176258,
            "range": "± 553",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60021,
            "range": "± 4772",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243524,
            "range": "± 19504",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7520,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9621,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45739,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 170,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "129c53803f598e2d8a99eddb22bec3a836f0bf10",
          "message": "chore(rust)(deps): update rsa requirement from 0.7 to 0.8 in /wnfs (#168)\n\n* chore(rust)(deps): update rsa requirement from 0.7 to 0.8 in /wnfs\r\n\r\nUpdates the requirements on [rsa](https://github.com/RustCrypto/RSA) to permit the latest version.\r\n- [Release notes](https://github.com/RustCrypto/RSA/releases)\r\n- [Changelog](https://github.com/RustCrypto/RSA/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/RustCrypto/RSA/compare/v0.7.0...v0.8.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: rsa\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* fix: Adjust to rsa 0.8 API\r\n\r\n* refactor: merge imports\r\n\r\n---------\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-03-23T11:12:38+01:00",
          "tree_id": "00974db25ef8e3dc46123cd6beacfd35c673240a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/129c53803f598e2d8a99eddb22bec3a836f0bf10"
        },
        "date": 1679566630705,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7321,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4378298,
            "range": "± 38007",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175926,
            "range": "± 773",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 198396,
            "range": "± 415",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48861,
            "range": "± 794",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 179299,
            "range": "± 483",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54451,
            "range": "± 4453",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 232379,
            "range": "± 18553",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7595,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9366,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50070,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 284,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d68f8500c752c647e4093c22d5487d1de4aaec3d",
          "message": "chore: Release wnfs-common, wnfs-hamt, wnfs-namefilter (#212)",
          "timestamp": "2023-03-23T11:42:28+01:00",
          "tree_id": "9b6c27316a400e8eb506fda57232cd8c0c7de12b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d68f8500c752c647e4093c22d5487d1de4aaec3d"
        },
        "date": 1679568364663,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7313,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4577457,
            "range": "± 18634",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 180014,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 200649,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51773,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176794,
            "range": "± 711",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60269,
            "range": "± 4795",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243157,
            "range": "± 19612",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7846,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9676,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45539,
            "range": "± 307",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 292,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "beb1e5ff05bcc9c0fa6292623393fab627f2e8fc",
          "message": "chore: release main (#195)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-23T12:01:50+01:00",
          "tree_id": "7b40ba416a095a7389298609d8270aaffd62cb6e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/beb1e5ff05bcc9c0fa6292623393fab627f2e8fc"
        },
        "date": 1679569555852,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9546,
            "range": "± 567",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5489120,
            "range": "± 314668",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 224434,
            "range": "± 19031",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 248252,
            "range": "± 11695",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 70163,
            "range": "± 2608",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 226022,
            "range": "± 11475",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 65257,
            "range": "± 7386",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 285350,
            "range": "± 37971",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13748,
            "range": "± 668",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13901,
            "range": "± 943",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 58439,
            "range": "± 2464",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 236,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e9ee67824cdc803e9c8ebb7b9c2f6ce8b8c79cea",
          "message": "chore: Add crates to `release-please-config.json` (#213)",
          "timestamp": "2023-03-23T15:39:34+01:00",
          "tree_id": "d1ee60e533dc54897ad4a3f29fa2c1ee154fcd2f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e9ee67824cdc803e9c8ebb7b9c2f6ce8b8c79cea"
        },
        "date": 1679582823155,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6874,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4553303,
            "range": "± 17747",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 180578,
            "range": "± 298",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 203342,
            "range": "± 352",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51736,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176518,
            "range": "± 394",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59702,
            "range": "± 4775",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 242259,
            "range": "± 19513",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7488,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9654,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42972,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 285,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fcd23bc4f2f13c22b3b84245a44857485d26bdfe",
          "message": "chore: Use version & path dependencies inter-workspace at the same time (#215)\n\n* chore: Add versions to workspace deps, force release-as\r\n\r\n* chore: Try without release-as for now",
          "timestamp": "2023-03-23T17:08:30+01:00",
          "tree_id": "e29a8874bec2c17290ad7e0d91cac4bd8ae04a4b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/fcd23bc4f2f13c22b3b84245a44857485d26bdfe"
        },
        "date": 1679587933892,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9581,
            "range": "± 497",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5249398,
            "range": "± 1349763",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 222229,
            "range": "± 18162",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 260168,
            "range": "± 13405",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 68266,
            "range": "± 3550",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 215995,
            "range": "± 10352",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 66270,
            "range": "± 7033",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 291819,
            "range": "± 49765",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13315,
            "range": "± 3213",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13233,
            "range": "± 957",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 53314,
            "range": "± 3129",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 366,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e9b0573e69ab995f620543be3fe10422a5061381",
          "message": "chore: Specify release-as 0.1.19 (#216)",
          "timestamp": "2023-03-23T17:52:07+01:00",
          "tree_id": "d072edca36dc93729c8ac41a99bc70d5220f77c2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e9b0573e69ab995f620543be3fe10422a5061381"
        },
        "date": 1679590547808,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8949,
            "range": "± 763",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5103896,
            "range": "± 276600",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 221563,
            "range": "± 19819",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 248850,
            "range": "± 12483",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 69470,
            "range": "± 4441",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 197539,
            "range": "± 13909",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61815,
            "range": "± 7040",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 260724,
            "range": "± 35698",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12575,
            "range": "± 842",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12586,
            "range": "± 790",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50512,
            "range": "± 2553",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 336,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "distinct": true,
          "id": "1f37ec4d706b9bcb4305128451cc77063b4f211d",
          "message": "chore: release 0.1.19\n\nRelease-As: 0.1.19",
          "timestamp": "2023-03-23T18:38:03+01:00",
          "tree_id": "d072edca36dc93729c8ac41a99bc70d5220f77c2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d"
        },
        "date": 1679593299459,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7176,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4327090,
            "range": "± 23250",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174421,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 196985,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48405,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 175769,
            "range": "± 946",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53821,
            "range": "± 4338",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 230375,
            "range": "± 18117",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7614,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9843,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40391,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 148,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "18e3806619d6805aab79e859f4ff9760d72045ed",
          "message": "chore: release main (#214)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-23T18:51:01+01:00",
          "tree_id": "81ccefd82c501bd31f0f465c82e3a8794fa8fabf",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/18e3806619d6805aab79e859f4ff9760d72045ed"
        },
        "date": 1679594087822,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8026,
            "range": "± 975",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4626086,
            "range": "± 716477",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 220030,
            "range": "± 58997",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 231110,
            "range": "± 38819",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 67214,
            "range": "± 5468",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 198141,
            "range": "± 12017",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 63946,
            "range": "± 7607",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 245670,
            "range": "± 29691",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13710,
            "range": "± 1184",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12904,
            "range": "± 1018",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45830,
            "range": "± 3610",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 176,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "distinct": true,
          "id": "437421a94cf33c2eef2c19637a9528c74a02a349",
          "message": "chore: Remove release-as 0.1.19 from config",
          "timestamp": "2023-03-23T19:47:35+01:00",
          "tree_id": "4c05c207264254cd64be82af5ca4b3cbf1fb8b48",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/437421a94cf33c2eef2c19637a9528c74a02a349"
        },
        "date": 1679597465342,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7199,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4342985,
            "range": "± 18916",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175997,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197404,
            "range": "± 403",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48400,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176370,
            "range": "± 348",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53892,
            "range": "± 4411",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 230327,
            "range": "± 18369",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7691,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9859,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40429,
            "range": "± 214",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 282,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f85e1643418084350de3df547191e7b2abc4a30b",
          "message": "chore(npm)(deps-dev): bump webpack from 5.76.2 to 5.76.3 in /wnfs-wasm (#220)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.76.2 to 5.76.3.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.76.2...v5.76.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-27T12:21:26+02:00",
          "tree_id": "d35b71806c1633b7923a4eb0a36ee2edf696e3a4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f85e1643418084350de3df547191e7b2abc4a30b"
        },
        "date": 1679912769319,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7331,
            "range": "± 185",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4343466,
            "range": "± 18604",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 175541,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 195346,
            "range": "± 690",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48459,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176652,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54497,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231548,
            "range": "± 18443",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7613,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9899,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40600,
            "range": "± 497",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 160,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "c@laudiacay.cool",
            "name": "c r",
            "username": "laudiacay"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "05f3739cdc4b5b9cb02427c51e5ddff6803122bd",
          "message": "feat: adding mutation api for metadata. cleaning up clippy complaints (#217)\n\n* feat: adding mutation api for metadata. also cleaning up some clippy complaints in link.rs\r\n\r\n* fix: changing doctest imports",
          "timestamp": "2023-03-29T10:56:09+02:00",
          "tree_id": "1e58a6ee03ec5e73cfb010ee6b0a0d202a489b01",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/05f3739cdc4b5b9cb02427c51e5ddff6803122bd"
        },
        "date": 1680080579581,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6973,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4639450,
            "range": "± 17492",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 178844,
            "range": "± 1353",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 202780,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51201,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 177727,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60071,
            "range": "± 4731",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 244973,
            "range": "± 19670",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7821,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9625,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45627,
            "range": "± 400",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 287,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "c@laudiacay.cool",
            "name": "c r",
            "username": "laudiacay"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f80dbb19cee471447145245b8c0285608a25ebcc",
          "message": "feat: `open_file_mut` function for getting `&mut PrivateFile` references (#218)\n\n* feat: adding open_file_mut. the doctest is broken because you have to mutably borrow twice to write content to the file.\r\n\r\n* fix: fixing doctest a little (doctest is still broken on multiple mutable references to store)\r\n\r\n* fix: doctest works",
          "timestamp": "2023-03-30T06:31:54-04:00",
          "tree_id": "4b660d07fb72dfc2b66cc1a6e871d361225ba0ea",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f80dbb19cee471447145245b8c0285608a25ebcc"
        },
        "date": 1680172577401,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8543,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5131290,
            "range": "± 70571",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 205041,
            "range": "± 3311",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 234203,
            "range": "± 2860",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 57383,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 215395,
            "range": "± 9356",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 64583,
            "range": "± 6380",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 276016,
            "range": "± 22052",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9014,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11710,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 48033,
            "range": "± 840",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 169,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4c34f19d87093c2b2dafdc5496033db423202f77",
          "message": "chore: release main (#221)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-30T16:46:46+02:00",
          "tree_id": "5d4dc43d7570a73b4826cf250124ccc0f8170c00",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4c34f19d87093c2b2dafdc5496033db423202f77"
        },
        "date": 1680187855717,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6911,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4344436,
            "range": "± 17642",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 173063,
            "range": "± 963",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 196878,
            "range": "± 271",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47632,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176951,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54352,
            "range": "± 4419",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231353,
            "range": "± 18517",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7546,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9765,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40469,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 282,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "da0833487f49005e22ad8cd4fd4e867f230cd91c",
          "message": "chore(npm)(deps-dev): bump typescript from 5.0.2 to 5.0.3 in /wnfs-wasm (#225)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 5.0.2 to 5.0.3.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-03T13:04:53+02:00",
          "tree_id": "b32bab1ba10f93aafbcc62212fcd1d7585bcd6d9",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/da0833487f49005e22ad8cd4fd4e867f230cd91c"
        },
        "date": 1680520173053,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7308,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4376412,
            "range": "± 16184",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174551,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197414,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48586,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178352,
            "range": "± 518",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54118,
            "range": "± 4342",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231122,
            "range": "± 18420",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7641,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9881,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40454,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 282,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b9fda6fc43831f33824f83e658296dad3a1895fe",
          "message": "chore(ci)(deps): bump rustsec/audit-check from 0.1.0 to 1.3.2 (#227)\n\nBumps [rustsec/audit-check](https://github.com/rustsec/audit-check) from 0.1.0 to 1.3.2.\r\n- [Release notes](https://github.com/rustsec/audit-check/releases)\r\n- [Changelog](https://github.com/rustsec/audit-check/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/rustsec/audit-check/compare/v0.1.0...v1.3.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: rustsec/audit-check\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-03T13:04:34+02:00",
          "tree_id": "254d4a68611e9f5e06d5d0748676b77ce5a42e85",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b9fda6fc43831f33824f83e658296dad3a1895fe"
        },
        "date": 1680520188319,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9244,
            "range": "± 509",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5318683,
            "range": "± 285737",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 224202,
            "range": "± 13581",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 246410,
            "range": "± 15668",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 65094,
            "range": "± 4055",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 220547,
            "range": "± 11800",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 66014,
            "range": "± 9267",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 277663,
            "range": "± 42729",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 15768,
            "range": "± 796",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12879,
            "range": "± 864",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 54972,
            "range": "± 4307",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 197,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d77d3356953275e5bbd2645225c658b434a9ada3",
          "message": "chore(npm)(deps-dev): bump webpack from 5.76.3 to 5.77.0 in /wnfs-wasm (#224)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.76.3 to 5.77.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.76.3...v5.77.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-03T13:05:10+02:00",
          "tree_id": "daa3718b8120b29a404fdb8f7363d389ed50216a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d77d3356953275e5bbd2645225c658b434a9ada3"
        },
        "date": 1680520237889,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10621,
            "range": "± 487",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5897464,
            "range": "± 435148",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 244398,
            "range": "± 27165",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 269735,
            "range": "± 17452",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 69885,
            "range": "± 4335",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 235640,
            "range": "± 13807",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 71352,
            "range": "± 7806",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 294725,
            "range": "± 40011",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 16620,
            "range": "± 1397",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13962,
            "range": "± 639",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 58881,
            "range": "± 9130",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 219,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "99b003da380a64302ba192dba34d0c8a050431ac",
          "message": "chore(npm)(deps-dev): bump webpack-dev-server in /wnfs-wasm (#226)\n\nBumps [webpack-dev-server](https://github.com/webpack/webpack-dev-server) from 4.13.1 to 4.13.2.\r\n- [Release notes](https://github.com/webpack/webpack-dev-server/releases)\r\n- [Changelog](https://github.com/webpack/webpack-dev-server/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-dev-server/compare/v4.13.1...v4.13.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-dev-server\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-03T13:43:01+02:00",
          "tree_id": "869f22e0594bfb57023d9fbaa296867c02183438",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/99b003da380a64302ba192dba34d0c8a050431ac"
        },
        "date": 1680522461258,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6896,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4663531,
            "range": "± 18297",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 179792,
            "range": "± 229",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 203025,
            "range": "± 613",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51833,
            "range": "± 140",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178456,
            "range": "± 583",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59959,
            "range": "± 4953",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 246818,
            "range": "± 19875",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7624,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9749,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 43045,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 162,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0c5b3cae11c13c227bdb7089d7ee66462702ce3a",
          "message": "chore(npm)(deps-dev): bump webpack from 5.77.0 to 5.78.0 in /wnfs-wasm (#232)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.77.0 to 5.78.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.77.0...v5.78.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-12T16:52:49+01:00",
          "tree_id": "db65661da28af1014b06da36a372eb0e2f1f108d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0c5b3cae11c13c227bdb7089d7ee66462702ce3a"
        },
        "date": 1681315071952,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7818,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4344393,
            "range": "± 19576",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 176297,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197933,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48264,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178252,
            "range": "± 434",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54373,
            "range": "± 4442",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231054,
            "range": "± 18579",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7698,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9898,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40412,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 277,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "03d11d2dc0073c76ea4c45bd5b3271da751b1582",
          "message": "chore(ci)(deps): bump rustsec/audit-check from 1.3.2 to 1.4.1 (#230)\n\nBumps [rustsec/audit-check](https://github.com/rustsec/audit-check) from 1.3.2 to 1.4.1.\r\n- [Release notes](https://github.com/rustsec/audit-check/releases)\r\n- [Changelog](https://github.com/rustsec/audit-check/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/rustsec/audit-check/compare/v1.3.2...v1.4.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: rustsec/audit-check\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-12T16:53:46+01:00",
          "tree_id": "4351d1e8a9398ea774bad2712afc7d31fbc2ea91",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/03d11d2dc0073c76ea4c45bd5b3271da751b1582"
        },
        "date": 1681315174367,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10480,
            "range": "± 488",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5937847,
            "range": "± 530551",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 236971,
            "range": "± 11497",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 266724,
            "range": "± 12721",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 69557,
            "range": "± 4228",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 237172,
            "range": "± 9073",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 71965,
            "range": "± 9487",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 305045,
            "range": "± 45867",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 17609,
            "range": "± 1424",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13922,
            "range": "± 835",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 59570,
            "range": "± 2820",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 264,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2c7f9e64fb35b08e50c3ba6e18ff409d5c4798ff",
          "message": "chore(npm)(deps-dev): bump typescript from 5.0.3 to 5.0.4 in /wnfs-wasm (#231)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 5.0.3 to 5.0.4.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v5.0.3...v5.0.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-12T16:55:19+01:00",
          "tree_id": "55a9d276634d5670693aae3f33202cb08282b152",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2c7f9e64fb35b08e50c3ba6e18ff409d5c4798ff"
        },
        "date": 1681315295968,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7371,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4334428,
            "range": "± 16300",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174508,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 197017,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48143,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178553,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54152,
            "range": "± 4320",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 230780,
            "range": "± 18377",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7689,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9899,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40411,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 173,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6c48a44d1aef04394e3293395f7b7ac63887d428",
          "message": "chore(npm)(deps-dev): bump webpack-dev-server in /wnfs-wasm (#236)\n\nBumps [webpack-dev-server](https://github.com/webpack/webpack-dev-server) from 4.13.2 to 4.13.3.\r\n- [Release notes](https://github.com/webpack/webpack-dev-server/releases)\r\n- [Changelog](https://github.com/webpack/webpack-dev-server/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-dev-server/compare/v4.13.2...v4.13.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-dev-server\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T10:33:24+01:00",
          "tree_id": "a2c638484c2764d2fee330c87961ce43eef3d419",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6c48a44d1aef04394e3293395f7b7ac63887d428"
        },
        "date": 1681724321497,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7429,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4318619,
            "range": "± 17669",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174457,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 196711,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48205,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 176773,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54185,
            "range": "± 4466",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 232133,
            "range": "± 18533",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7610,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9857,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40497,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 278,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9df17c7642e3c3fa58da5b02fca28c84bbe82847",
          "message": "chore(npm)(deps-dev): bump html-webpack-plugin in /wnfs-wasm (#234)\n\nBumps [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin) from 5.5.0 to 5.5.1.\r\n- [Release notes](https://github.com/jantimon/html-webpack-plugin/releases)\r\n- [Changelog](https://github.com/jantimon/html-webpack-plugin/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/jantimon/html-webpack-plugin/compare/v5.5.0...v5.5.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: html-webpack-plugin\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T10:34:03+01:00",
          "tree_id": "2f8e7fa4669e4e5b38ae679bd24bd90e264596df",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9df17c7642e3c3fa58da5b02fca28c84bbe82847"
        },
        "date": 1681724344315,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7485,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4324587,
            "range": "± 16445",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174446,
            "range": "± 769",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 195712,
            "range": "± 299",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48346,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 177547,
            "range": "± 475",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54154,
            "range": "± 4386",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 230727,
            "range": "± 18579",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7608,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9837,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40416,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "164403807a59ede7e569bc6efa3628cc5ce8c603",
          "message": "chore(npm)(deps-dev): bump webpack from 5.78.0 to 5.79.0 in /wnfs-wasm (#235)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.78.0 to 5.79.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.78.0...v5.79.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-18T08:00:27+01:00",
          "tree_id": "e2957adb21f7f992d7983a7e7a452e967f020531",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/164403807a59ede7e569bc6efa3628cc5ce8c603"
        },
        "date": 1681801532042,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6865,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4319315,
            "range": "± 16882",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174221,
            "range": "± 836",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 196204,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48550,
            "range": "± 224",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 177332,
            "range": "± 571",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53916,
            "range": "± 4423",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 231330,
            "range": "± 18468",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7658,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9827,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 40460,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "frando@unbiskant.org",
            "name": "Franz Heinzmann",
            "username": "Frando"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1572f432b6ae5366436cdefda7defd71c23b0ca7",
          "message": "feat: Add `PrivateDirectory::entires`, `PrivateFile::read_at` and make `PrivateFile::get_content_size_upper_bound` public (#237)\n\n* additions for fuse support\r\n\r\n* cleanup: rename read_chunk to read_at and use stream_content\r\n\r\n* refactor: simplify PrivateFile::read_at\r\n\r\n* fix: PrivateDirectory::get_entries docs & rename\r\n\r\n* test: add proptest for PrivateFile::read_at\r\n\r\n* fix: PrivateFile::read_at error out on bad block",
          "timestamp": "2023-04-23T11:09:23+02:00",
          "tree_id": "377a495ec1338e4a6d125d93151f3ab5e0d3f061",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/1572f432b6ae5366436cdefda7defd71c23b0ca7"
        },
        "date": 1682241352367,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6802,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4408846,
            "range": "± 17686",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170511,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 190397,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41805,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 179273,
            "range": "± 525",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56207,
            "range": "± 4755",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 235355,
            "range": "± 18850",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8138,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12671,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44601,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 170,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "caf73b1c08ac53147aa41f7c8699459b5763b95d",
          "message": "chore(npm)(deps-dev): bump webpack-cli from 5.0.1 to 5.0.2 in /wnfs-wasm (#240)\n\nBumps [webpack-cli](https://github.com/webpack/webpack-cli) from 5.0.1 to 5.0.2.\r\n- [Release notes](https://github.com/webpack/webpack-cli/releases)\r\n- [Changelog](https://github.com/webpack/webpack-cli/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-cli/compare/webpack-cli@5.0.1...webpack-cli@5.0.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-cli\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-27T08:44:12+01:00",
          "tree_id": "0b21f7ea9ec78d02e2e732be44ea9ba2851fe997",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/caf73b1c08ac53147aa41f7c8699459b5763b95d"
        },
        "date": 1682581706461,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7491,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4422616,
            "range": "± 27776",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170669,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 190566,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42244,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 179679,
            "range": "± 506",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55931,
            "range": "± 4590",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 235407,
            "range": "± 18800",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9138,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12640,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44589,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 168,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8755d0bf6d815f602331459d7a32a53f99d3a40f",
          "message": "chore(npm)(deps-dev): bump @wasm-tool/wasm-pack-plugin in /wnfs-wasm (#241)\n\nBumps [@wasm-tool/wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin) from 1.6.0 to 1.7.0.\r\n- [Release notes](https://github.com/wasm-tool/wasm-pack-plugin/releases)\r\n- [Commits](https://github.com/wasm-tool/wasm-pack-plugin/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@wasm-tool/wasm-pack-plugin\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-27T08:44:25+01:00",
          "tree_id": "bb1787bea41ee8c0f4ef38d0e4d7e3722fe73eb3",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8755d0bf6d815f602331459d7a32a53f99d3a40f"
        },
        "date": 1682581707512,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6767,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4419362,
            "range": "± 16736",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170110,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 190663,
            "range": "± 458",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41981,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178866,
            "range": "± 452",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55958,
            "range": "± 4502",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 235350,
            "range": "± 18527",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8493,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12652,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44597,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 274,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bccc0ece5e41e4a22e2ad969091f31a6622a80d0",
          "message": "chore(npm)(deps-dev): bump webpack from 5.79.0 to 5.81.0 in /wnfs-wasm (#243)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.79.0 to 5.81.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.79.0...v5.81.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-27T12:55:00+01:00",
          "tree_id": "0b026e045b4bb2f896d92915309cb1eb6fee3a29",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/bccc0ece5e41e4a22e2ad969091f31a6622a80d0"
        },
        "date": 1682596741404,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7379,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4424261,
            "range": "± 17359",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 169918,
            "range": "± 186",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 189925,
            "range": "± 311",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42024,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178695,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55888,
            "range": "± 4545",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 234813,
            "range": "± 18990",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8493,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12648,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 44573,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 169,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4c16dcd4725c8b499a01184530e0e95ed8f4a9d5",
          "message": "fix: propagate missing chunk error (#252)\n\n* Propagate missing chunk error\r\n\r\n* Fix lint errors",
          "timestamp": "2023-05-05T12:04:49+01:00",
          "tree_id": "e6cf927fcc960793c78c3853130677a5d6302ce1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4c16dcd4725c8b499a01184530e0e95ed8f4a9d5"
        },
        "date": 1683285056778,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 11141,
            "range": "± 811",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 6016614,
            "range": "± 400847",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 235055,
            "range": "± 13927",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 259192,
            "range": "± 13071",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 58867,
            "range": "± 5490",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 240955,
            "range": "± 13133",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 75953,
            "range": "± 8911",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 319908,
            "range": "± 46310",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 14201,
            "range": "± 729",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12899,
            "range": "± 914",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 57901,
            "range": "± 2509",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 254,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5ed87fe6359a19abdea5f34dd0537fd5d62c98a8",
          "message": "fix!: get_node should return null on missing path (#253)\n\n* Return `Ok(None)` in get_node if node not found\r\n\r\n* Update bindings\r\n\r\n* Fix lint errors",
          "timestamp": "2023-05-05T12:07:40+01:00",
          "tree_id": "ff83edce1f6e6837b07c4cec289f2ae44d49c3c4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5ed87fe6359a19abdea5f34dd0537fd5d62c98a8"
        },
        "date": 1683285150377,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7658,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4513097,
            "range": "± 18332",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172744,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194379,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41705,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 184677,
            "range": "± 370",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56513,
            "range": "± 4616",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 240072,
            "range": "± 19228",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8338,
            "range": "± 777",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12607,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45069,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b3afbbcea6f2b27e43a6687e64fbd0220382cd53",
          "message": "chore: Write up an example of how to use tiered blockstores (#223)\n\n* chore: Write up an example of how to use tiered blockstores\r\n\r\n* docs: Uncomment `assert!`",
          "timestamp": "2023-05-05T14:46:09+02:00",
          "tree_id": "96b995b1c7fdaebf5c8f200110d089cd554a4436",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b3afbbcea6f2b27e43a6687e64fbd0220382cd53"
        },
        "date": 1683291051101,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6797,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3989391,
            "range": "± 16431",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 151534,
            "range": "± 1395",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 170966,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 36988,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 162255,
            "range": "± 264",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49771,
            "range": "± 4089",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 214042,
            "range": "± 15231",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7362,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11130,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39757,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 151,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2be9f4999d279acccfcda3b690d69dcbcdf8e60b",
          "message": "chore: release 0.1.21 (#255)\n\nRelease-As: 0.1.21",
          "timestamp": "2023-05-05T16:45:23+01:00",
          "tree_id": "96b995b1c7fdaebf5c8f200110d089cd554a4436",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b"
        },
        "date": 1683301847939,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9106,
            "range": "± 1143",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5371563,
            "range": "± 291445",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 212438,
            "range": "± 16504",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 235546,
            "range": "± 18669",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 53262,
            "range": "± 3356",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 193250,
            "range": "± 11973",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60565,
            "range": "± 8617",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 258002,
            "range": "± 32326",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 11372,
            "range": "± 531",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10497,
            "range": "± 526",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 49383,
            "range": "± 2374",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 207,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "94e6d66e7afa7571acdab0f18ca12fa15c8f327a",
          "message": "chore(npm)(deps-dev): bump webpack from 5.81.0 to 5.82.0 in /wnfs-wasm (#259)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.81.0 to 5.82.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.81.0...v5.82.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-08T18:48:37+01:00",
          "tree_id": "ae1f6f049656814489535eadbba44ed57f4d2d54",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/94e6d66e7afa7571acdab0f18ca12fa15c8f327a"
        },
        "date": 1683568497547,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8629,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5407058,
            "range": "± 28721",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 208232,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 232652,
            "range": "± 1177",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 50064,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 218387,
            "range": "± 974",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 67172,
            "range": "± 5650",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 286741,
            "range": "± 37442",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10187,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 15137,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 54001,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 332,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fda1ed2f637067be30a46713291f6a2fd1c06776",
          "message": "chore(npm)(deps-dev): bump webpack-cli from 5.0.2 to 5.1.0 in /wnfs-wasm (#261)\n\nBumps [webpack-cli](https://github.com/webpack/webpack-cli) from 5.0.2 to 5.1.0.\r\n- [Release notes](https://github.com/webpack/webpack-cli/releases)\r\n- [Changelog](https://github.com/webpack/webpack-cli/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-cli/compare/webpack-cli@5.0.2...webpack-cli@5.1.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-cli\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-09T11:13:25+01:00",
          "tree_id": "f76b3163482ae6867382f2b4cf2918ad0bc99378",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/fda1ed2f637067be30a46713291f6a2fd1c06776"
        },
        "date": 1683627578765,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8260,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4501922,
            "range": "± 17240",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 174453,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194374,
            "range": "± 422",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42099,
            "range": "± 457",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 183810,
            "range": "± 516",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56041,
            "range": "± 4560",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 240800,
            "range": "± 19425",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8965,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11696,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45564,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 172,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4b795cd00d93b18a333d0359343adb53bc2b2bae",
          "message": "chore(npm)(deps-dev): bump webpack-dev-server in /wnfs-wasm (#260)\n\nBumps [webpack-dev-server](https://github.com/webpack/webpack-dev-server) from 4.13.3 to 4.15.0.\r\n- [Release notes](https://github.com/webpack/webpack-dev-server/releases)\r\n- [Changelog](https://github.com/webpack/webpack-dev-server/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-dev-server/compare/v4.13.3...v4.15.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-dev-server\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-09T14:41:21+01:00",
          "tree_id": "0fbf011172af79f83223716d3a8e3fca633348d1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4b795cd00d93b18a333d0359343adb53bc2b2bae"
        },
        "date": 1683640056315,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7747,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4479374,
            "range": "± 17095",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 173058,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193625,
            "range": "± 327",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41968,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 182407,
            "range": "± 368",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56127,
            "range": "± 4585",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 239412,
            "range": "± 19108",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8958,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11709,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45543,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 171,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@vera.lgbt",
            "name": "Vera Gonzalez",
            "username": "organizedgrime"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f85ace3474941bed76dba09a63d3a90802456ecf",
          "message": "PrivateNodeHistory::of & PrivateHistory::of (#264)\n\n* fix: replaced PrivateNodeOnPathHistory function signature\r\n\r\n* fix: replaced PrivateNodeHistory function signature\r\n\r\n* fix: cargo fmt",
          "timestamp": "2023-05-15T12:15:17-04:00",
          "tree_id": "e12d2eb7c28cd9675cd3f1119e575df296a79951",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f85ace3474941bed76dba09a63d3a90802456ecf"
        },
        "date": 1684167696511,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7735,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4747260,
            "range": "± 19430",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172383,
            "range": "± 872",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193224,
            "range": "± 389",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40752,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 183991,
            "range": "± 562",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60022,
            "range": "± 4778",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 252395,
            "range": "± 20266",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8249,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9839,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42771,
            "range": "± 1444",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 161,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "df90158abc7a8e3c7958971f057dc230f1a82873",
          "message": "chore(npm)(deps-dev): bump webpack-cli from 5.1.0 to 5.1.1 in /wnfs-wasm (#262)\n\nBumps [webpack-cli](https://github.com/webpack/webpack-cli) from 5.1.0 to 5.1.1.\r\n- [Release notes](https://github.com/webpack/webpack-cli/releases)\r\n- [Changelog](https://github.com/webpack/webpack-cli/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-cli/compare/webpack-cli@5.1.0...webpack-cli@5.1.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-cli\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-17T09:06:31+01:00",
          "tree_id": "dec213753c12f5d1892502bb615b7b290594800a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/df90158abc7a8e3c7958971f057dc230f1a82873"
        },
        "date": 1684311241511,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9604,
            "range": "± 973",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5543524,
            "range": "± 431531",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 218534,
            "range": "± 16055",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 250560,
            "range": "± 27932",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 55644,
            "range": "± 4693",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 220640,
            "range": "± 16158",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 64867,
            "range": "± 9819",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 283898,
            "range": "± 45698",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12963,
            "range": "± 875",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12951,
            "range": "± 1291",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 52864,
            "range": "± 4871",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 222,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fedb793cdb4c32bc5d43f048178b81519a1f5bd0",
          "message": "chore(npm)(deps-dev): bump webpack from 5.82.0 to 5.82.1 in /wnfs-wasm (#263)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.82.0 to 5.82.1.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.82.0...v5.82.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-17T09:26:30+01:00",
          "tree_id": "243c717836189b75d80f5c6ebad7e06aabb1cb63",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/fedb793cdb4c32bc5d43f048178b81519a1f5bd0"
        },
        "date": 1684312410189,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8577,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5242217,
            "range": "± 88894",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 205401,
            "range": "± 3004",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 229325,
            "range": "± 3205",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48516,
            "range": "± 964",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 219743,
            "range": "± 15748",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 65099,
            "range": "± 5666",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 284823,
            "range": "± 22275",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10735,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13812,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 53333,
            "range": "± 909",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 216,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "appcypher@outlook.com",
            "name": "Stephen Akinyemi",
            "username": "appcypher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "020aac2432ee81c681ca68bc6d60c74193b20a99",
          "message": "chore(format): use capsule-type serialization  for fs nodes (#265)\n\n* Use capsule-type serialization  for fs nodes\r\n\r\n* Minor fix",
          "timestamp": "2023-05-17T11:53:29+01:00",
          "tree_id": "9218ce78fa8730b4dcc467962485c0d855577e28",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/020aac2432ee81c681ca68bc6d60c74193b20a99"
        },
        "date": 1684321197533,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7207,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4698322,
            "range": "± 19118",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172731,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193879,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41467,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 182048,
            "range": "± 933",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59608,
            "range": "± 4694",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 249826,
            "range": "± 20109",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8064,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9830,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42715,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 159,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@vera.lgbt",
            "name": "Vera Gonzalez",
            "username": "organizedgrime"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "874030d9b62d16239d192921873e79f4d926d197",
          "message": "Modified BlockStore trait (#257)\n\n* Modified BlockStore trait; implemented new BlockStores\r\n\r\n* fix lib.rs MAX_BLOCK_SIZE\r\n\r\nSigned-off-by: Vera Gonzalez <me@vera.lgbt>\r\n\r\n* Responding to PR reviews\r\n\r\nSigned-off-by: Vera Gonzalez <me@vera.lgbt>\r\nCo-Authored-By: Stephen Akinyemi <appcypher@outlook.com>\r\n\r\n* fix: Removed all new BlockStore implementations; reverted to old directory structure\r\n\r\n* fix: nits\r\n\r\n* nit: removed rand dev\r\n\r\n* fix: removed wrong rand\r\n\r\n---------\r\n\r\nSigned-off-by: Vera Gonzalez <me@vera.lgbt>\r\nCo-authored-by: Claudia Richoux <c@laudiacay.cool>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-05-22T11:07:25+02:00",
          "tree_id": "7c997a3a48954926a4dc9932bd399c812ebdd3b3",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/874030d9b62d16239d192921873e79f4d926d197"
        },
        "date": 1684746835104,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6779,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4458342,
            "range": "± 18591",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172406,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 195069,
            "range": "± 211",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42490,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 185675,
            "range": "± 473",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54305,
            "range": "± 4427",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 236503,
            "range": "± 18777",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8512,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11723,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 45554,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 277,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f27afe26e268cd8ddb3e8e6d4eb48c6c3f8857e5",
          "message": "chore(npm)(deps-dev): bump style-loader in /wnfs-wasm (#269)\n\nBumps [style-loader](https://github.com/webpack-contrib/style-loader) from 3.3.2 to 3.3.3.\r\n- [Release notes](https://github.com/webpack-contrib/style-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/style-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/style-loader/compare/v3.3.2...v3.3.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: style-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-12T14:38:09+01:00",
          "tree_id": "1bbe91ad4a20a711011167737038de7ff5b8a19a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f27afe26e268cd8ddb3e8e6d4eb48c6c3f8857e5"
        },
        "date": 1686577430578,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8607,
            "range": "± 222",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5203865,
            "range": "± 130266",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 202930,
            "range": "± 3917",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 226324,
            "range": "± 4042",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49846,
            "range": "± 1477",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 214634,
            "range": "± 2447",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 63959,
            "range": "± 5255",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 279637,
            "range": "± 22776",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 9939,
            "range": "± 944",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12636,
            "range": "± 417",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 47641,
            "range": "± 940",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 205,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}