window.BENCHMARK_DATA = {
  "lastUpdate": 1673632413695,
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
      }
    ]
  }
}