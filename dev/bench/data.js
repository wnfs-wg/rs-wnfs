window.BENCHMARK_DATA = {
  "lastUpdate": 1714057925883,
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
          "id": "2a32439e542c84131501a2417b2f9aba949554e2",
          "message": "chore(npm)(deps-dev): bump css-loader from 6.7.3 to 6.8.1 in /wnfs-wasm (#272)\n\nBumps [css-loader](https://github.com/webpack-contrib/css-loader) from 6.7.3 to 6.8.1.\r\n- [Release notes](https://github.com/webpack-contrib/css-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/css-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/css-loader/compare/v6.7.3...v6.8.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: css-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-12T14:38:52+01:00",
          "tree_id": "5c160df8c542bfb750623cb6cb19003d4bc3c221",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2a32439e542c84131501a2417b2f9aba949554e2"
        },
        "date": 1686577443585,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7147,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4470856,
            "range": "± 20394",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172282,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194015,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42562,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 182304,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55323,
            "range": "± 4466",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 237372,
            "range": "± 18886",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8472,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11741,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39949,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 184,
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
          "id": "c51c1b7f7173e56f7af15359bd323119d6f68268",
          "message": "chore: release main (#238)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-14T09:17:57+01:00",
          "tree_id": "b5e8e1387a3b6b8a76e9dc957a33bfec8b0170aa",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c51c1b7f7173e56f7af15359bd323119d6f68268"
        },
        "date": 1686730971533,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9462,
            "range": "± 271",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5334169,
            "range": "± 32584",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 206151,
            "range": "± 445",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 233640,
            "range": "± 2793",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51418,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 216640,
            "range": "± 598",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 64897,
            "range": "± 5387",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 284402,
            "range": "± 37582",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10183,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 14127,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 47964,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 354,
            "range": "± 12",
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
          "id": "2f8ad3f1456edc26be4c9a54319983fb65ae2a6a",
          "message": "chore(rust)(deps): update rsa requirement from 0.8 to 0.9 in /wnfs (#244)\n\n* chore(rust)(deps): update rsa requirement from 0.8 to 0.9 in /wnfs\r\n\r\nUpdates the requirements on [rsa](https://github.com/RustCrypto/RSA) to permit the latest version.\r\n- [Release notes](https://github.com/RustCrypto/RSA/releases)\r\n- [Changelog](https://github.com/RustCrypto/RSA/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/RustCrypto/RSA/compare/v0.8.0...v0.9.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: rsa\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* Fix import\r\n\r\n* Fromat\r\n\r\n* Fix lint\r\n\r\n* Fix lint\r\n\r\n---------\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-06-14T10:07:40+01:00",
          "tree_id": "a744da5b77b0de7e609ab30280a1a22dd682816b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2f8ad3f1456edc26be4c9a54319983fb65ae2a6a"
        },
        "date": 1686733864650,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7569,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4390354,
            "range": "± 17343",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170128,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193226,
            "range": "± 269",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42797,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180098,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53634,
            "range": "± 4425",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 233770,
            "range": "± 18764",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8485,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 10722,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39908,
            "range": "± 47",
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
          "id": "878218d6db5e1d82e9d462c5488d3068c777b6ca",
          "message": "chore(npm)(deps-dev): bump ts-loader from 9.4.2 to 9.4.3 in /wnfs-wasm (#273)\n\nBumps [ts-loader](https://github.com/TypeStrong/ts-loader) from 9.4.2 to 9.4.3.\r\n- [Release notes](https://github.com/TypeStrong/ts-loader/releases)\r\n- [Changelog](https://github.com/TypeStrong/ts-loader/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/TypeStrong/ts-loader/compare/v9.4.2...v9.4.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: ts-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-14T12:26:48+01:00",
          "tree_id": "25da1ec745e40b0c67012b0275ac3f8ccffb9902",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/878218d6db5e1d82e9d462c5488d3068c777b6ca"
        },
        "date": 1686742203321,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8507,
            "range": "± 613",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4987780,
            "range": "± 309590",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 195778,
            "range": "± 15921",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 220441,
            "range": "± 12869",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51349,
            "range": "± 4062",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 197381,
            "range": "± 12053",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61245,
            "range": "± 8188",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 268317,
            "range": "± 30403",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 11451,
            "range": "± 829",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11814,
            "range": "± 939",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 54432,
            "range": "± 4892",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 337,
            "range": "± 73",
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
          "id": "4752ee7876ad2c196514b18401835933daf2602f",
          "message": "chore(npm)(deps-dev): bump webpack from 5.82.1 to 5.86.0 in /wnfs-wasm (#277)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.82.1 to 5.86.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.82.1...v5.86.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-14T12:30:51+01:00",
          "tree_id": "ee3de6ec185e839e8e971dd37a8c6826f2f33888",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4752ee7876ad2c196514b18401835933daf2602f"
        },
        "date": 1686742460804,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8395,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5213392,
            "range": "± 43643",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 203053,
            "range": "± 5582",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 229814,
            "range": "± 4149",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 50433,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 215654,
            "range": "± 1573",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 63552,
            "range": "± 5243",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 278805,
            "range": "± 22047",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 10138,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12775,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 47506,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 202,
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
          "id": "c18a00c8e0db860b79d1c73b13f67ecd87cc3d2b",
          "message": "chore(rust)(deps): update multihash requirement in /wnfs (#276)\n\nUpdates the requirements on [multihash](https://github.com/multiformats/rust-multihash) to permit the latest version.\r\n- [Changelog](https://github.com/multiformats/rust-multihash/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/rust-multihash/compare/v0.18.0...v0.19.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multihash\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-14T12:52:23+01:00",
          "tree_id": "e9420b44eac863cadfecf2ec84c57cf633fab061",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c18a00c8e0db860b79d1c73b13f67ecd87cc3d2b"
        },
        "date": 1686743749142,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7129,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4563733,
            "range": "± 18847",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170285,
            "range": "± 599",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 192536,
            "range": "± 576",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42983,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178648,
            "range": "± 463",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59596,
            "range": "± 4743",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 244966,
            "range": "± 19585",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7551,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 8817,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41893,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 167,
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
          "id": "d7005768d786c3d78f7f99d26258d8a53fe8c420",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#278)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.31.2 to 1.35.0.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.31.2...v1.35.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-14T12:52:48+01:00",
          "tree_id": "af6a2d27fe6f38b690bcb1484b41490292c227bc",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d7005768d786c3d78f7f99d26258d8a53fe8c420"
        },
        "date": 1686743786742,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9837,
            "range": "± 870",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5434669,
            "range": "± 391081",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 223064,
            "range": "± 19504",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 249442,
            "range": "± 19826",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 60767,
            "range": "± 7204",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 223976,
            "range": "± 18580",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 69064,
            "range": "± 10292",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 302702,
            "range": "± 46960",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13158,
            "range": "± 798",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12874,
            "range": "± 1464",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 60172,
            "range": "± 3016",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 233,
            "range": "± 160",
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
          "id": "32600d37506fa962dab00c1844f71e2ac4bf2d42",
          "message": "Remove stale example (#279)",
          "timestamp": "2023-06-14T12:52:56+01:00",
          "tree_id": "5db3a1f00491d0effc3ae659b5c4c6237e02c9c2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/32600d37506fa962dab00c1844f71e2ac4bf2d42"
        },
        "date": 1686743919789,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6980,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4414349,
            "range": "± 18321",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170947,
            "range": "± 554",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193615,
            "range": "± 359",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42305,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180753,
            "range": "± 594",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54306,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 234987,
            "range": "± 18726",
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
            "value": 12527,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39934,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 174,
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
          "id": "24bf412b16b56c2d5cae966d728889361dee08fe",
          "message": "chore(api): mutable API preliminary work (#229)\n\n* Add mutable dirs\r\n\r\n* Refactor roottree\r\n\r\n* Fix fmt\r\n\r\n* Expose roottree api\r\n\r\n* Add store and load fns\r\n\r\n* Remove T time generic param\r\n\r\n* Fix lint\r\n\r\n* Take ref blockstore instead",
          "timestamp": "2023-06-14T13:47:35+01:00",
          "tree_id": "3ce0d2dcb64ac0b2c52f8c3a3630642496ac7603",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/24bf412b16b56c2d5cae966d728889361dee08fe"
        },
        "date": 1686747060489,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7187,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4552702,
            "range": "± 19096",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 169627,
            "range": "± 774",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 191984,
            "range": "± 398",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42835,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 179296,
            "range": "± 590",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59205,
            "range": "± 4695",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 244999,
            "range": "± 19783",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7601,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 8682,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41880,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 284,
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
          "id": "7018e5a23e8f6963242304faac0a7ffdcd44b6d8",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#280)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.35.0 to 1.35.1.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.35.0...v1.35.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-21T13:54:12+01:00",
          "tree_id": "684e964d3057d584dd468bb04f3947790ba4a650",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7018e5a23e8f6963242304faac0a7ffdcd44b6d8"
        },
        "date": 1687352310321,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10029,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5695432,
            "range": "± 341484",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 220647,
            "range": "± 13476",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 250648,
            "range": "± 15385",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 59409,
            "range": "± 3427",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 227264,
            "range": "± 11248",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 69558,
            "range": "± 6781",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 294582,
            "range": "± 40742",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13023,
            "range": "± 756",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 12646,
            "range": "± 744",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 55124,
            "range": "± 3668",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 193,
            "range": "± 64",
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
          "id": "85d84fcc29b3fb3adafa19b95c22259660bb31c4",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#281)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 11.0.2 to 12.0.1.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v11.0.2...v12.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-21T13:54:47+01:00",
          "tree_id": "73bcaddb021abeeae01dea501d02ae75ea87b59a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/85d84fcc29b3fb3adafa19b95c22259660bb31c4"
        },
        "date": 1687352341656,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 9446,
            "range": "± 779",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5498564,
            "range": "± 273805",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 206245,
            "range": "± 15841",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 240002,
            "range": "± 13235",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 55119,
            "range": "± 1836",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 209959,
            "range": "± 8147",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 63060,
            "range": "± 5752",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 282324,
            "range": "± 28643",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12387,
            "range": "± 691",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11667,
            "range": "± 932",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 50451,
            "range": "± 1648",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 184,
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
          "id": "ea197f8418efdb776f5ab492c5a72894b8ae091c",
          "message": "chore(npm)(deps-dev): bump webpack from 5.86.0 to 5.87.0 in /wnfs-wasm (#282)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.86.0 to 5.87.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.86.0...v5.87.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-21T16:54:52+01:00",
          "tree_id": "8f2cf5b1d6fef9f95aa4f39b069fc2d92e07c87e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ea197f8418efdb776f5ab492c5a72894b8ae091c"
        },
        "date": 1687363089233,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7286,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4442308,
            "range": "± 17293",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172402,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194317,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42345,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 182206,
            "range": "± 988",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54680,
            "range": "± 4727",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 236744,
            "range": "± 19050",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7644,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9292,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39394,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 179,
            "range": "± 29",
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
          "id": "5f3228cdb9a7e0c7d37196875e3bef8b070e66a1",
          "message": "chore(npm)(deps-dev): bump webpack-cli from 5.1.1 to 5.1.4 in /wnfs-wasm (#284)\n\nBumps [webpack-cli](https://github.com/webpack/webpack-cli) from 5.1.1 to 5.1.4.\r\n- [Release notes](https://github.com/webpack/webpack-cli/releases)\r\n- [Changelog](https://github.com/webpack/webpack-cli/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack/webpack-cli/compare/webpack-cli@5.1.1...webpack-cli@5.1.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack-cli\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-21T17:26:39+01:00",
          "tree_id": "b38afc2f6974be69e6f01454207c58c8fdc95eb2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5f3228cdb9a7e0c7d37196875e3bef8b070e66a1"
        },
        "date": 1687364992658,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6760,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4596964,
            "range": "± 18159",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 170534,
            "range": "± 819",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193699,
            "range": "± 614",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 43101,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 178962,
            "range": "± 832",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 59272,
            "range": "± 4745",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 245875,
            "range": "± 19833",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7584,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 8814,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 42166,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 283,
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
          "id": "3fb53924d104dc03f027aed04db731e5393dd844",
          "message": "chore: update READMEs (#287)\n\n* Update READMEs\r\n\r\n* Fix typo and broken link",
          "timestamp": "2023-06-23T15:44:38+02:00",
          "tree_id": "b1bce2e090e228d0e28d099840b03aaf6f71c5a6",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/3fb53924d104dc03f027aed04db731e5393dd844"
        },
        "date": 1687528078843,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7243,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4439128,
            "range": "± 18057",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172088,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 194068,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42924,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 182428,
            "range": "± 537",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54541,
            "range": "± 4452",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 237849,
            "range": "± 18970",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 7591,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9299,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39110,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 292,
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
          "id": "085242d15aa48db17d77ed45e1c7717d13ed105f",
          "message": "feat: make changes to BlockStore trait based on feedback (#286)\n\n* Replace IpldCodec with u64\r\n\r\n* Change `Vec<u8>` to `Bytes` on BlockStore trait",
          "timestamp": "2023-06-23T17:03:58+01:00",
          "tree_id": "f969e8bc2c51c2a9e1b2be1275c41696eda534d1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f"
        },
        "date": 1687536431307,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7218,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4596578,
            "range": "± 18083",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 171439,
            "range": "± 299",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 192718,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42454,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180298,
            "range": "± 964",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58329,
            "range": "± 4634",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 243252,
            "range": "± 19330",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8514,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 9123,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 41899,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 281,
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
          "id": "b749e74cd815ea1d6923d7ffb66f0a030f4cf23a",
          "message": "feat: make AccessKey the key entrypoint for public apis (#285)\n\n* Make AccessKey the key entrypoint for public apis\r\n\r\n* Remove todos\r\n\r\n* Update wnfs/src/private/share.rs\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nSigned-off-by: Stephen Akinyemi <appcypher@outlook.com>\r\n\r\n* Refactor get_multivalue\r\n\r\n---------\r\n\r\nSigned-off-by: Stephen Akinyemi <appcypher@outlook.com>\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-06-26T16:39:44+01:00",
          "tree_id": "a09173557f419019739d0675e98a82a4ed916469",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b749e74cd815ea1d6923d7ffb66f0a030f4cf23a"
        },
        "date": 1687794241298,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7885,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4449158,
            "range": "± 19154",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 172697,
            "range": "± 2803",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193316,
            "range": "± 423",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41916,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 184204,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54715,
            "range": "± 4444",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 237518,
            "range": "± 18817",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8481,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11704,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39962,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 225,
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
          "id": "25c521565de6704245dc01573d8848f439ddd270",
          "message": "chore: release main (#288)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-26T16:41:25+01:00",
          "tree_id": "f4cad08ca36ae068f9e31d50c4f535738f18bb92",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/25c521565de6704245dc01573d8848f439ddd270"
        },
        "date": 1687794393862,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 10854,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5801040,
            "range": "± 359801",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 227435,
            "range": "± 13727",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 246420,
            "range": "± 10539",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 59293,
            "range": "± 5741",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 231489,
            "range": "± 25525",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 70745,
            "range": "± 8784",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 301811,
            "range": "± 40980",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 12588,
            "range": "± 1095",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13837,
            "range": "± 662",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 60700,
            "range": "± 4689",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 260,
            "range": "± 30",
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
            "email": "contact@gozala.io",
            "name": "Irakli Gozalishvili",
            "username": "Gozala"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "982feff849a3f42bb859636a68324b3c6a550a91",
          "message": "feat: expose AccessKey encode/decode api (#296)\n\n* feat: expose AccessKey encode/decode api\r\n\r\n* Format using newer formatter\r\n\r\n---------\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-07-05T14:01:26+02:00",
          "tree_id": "3e945ae4857879d099a410ffada8a37c09847cc6",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/982feff849a3f42bb859636a68324b3c6a550a91"
        },
        "date": 1688558770095,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7190,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4531785,
            "range": "± 19174",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 173590,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 195645,
            "range": "± 280",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41859,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 188074,
            "range": "± 581",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56762,
            "range": "± 4644",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 242234,
            "range": "± 22012",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 8044,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11844,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 39960,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 293,
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
          "id": "7026a379443038fa1b0410df1c7d0bc23649f17a",
          "message": "feat: Switch from Namefilter to Name Accumulators (#247)\n\n* feat: Add wnfs-nameaccumulator crate\r\n\r\n* WIP: refactor\r\n\r\n* WIP\r\n\r\n* Type checker\r\n\r\n* Checkpoint: No failing unit tests\r\n\r\n* Remove wnfs-namefilter\r\n\r\n* feat: Benchmarks for name accumulators\r\n\r\n* chore: Use published skip ratchet version\r\n\r\n* refactor out Name abstraction\r\n\r\n* cargo clippy --fix\r\n\r\n* Run proptests less often\r\n\r\n* refactor: use shorter AccumulatorSetup names\r\n\r\n* Make header's name based on mountpoint\r\n\r\n* Mount files and directories to explicit paths\r\n\r\n* refactor: Some forest function renames & moving code around\r\n\r\n* Implement proof generation\r\n\r\n* Attach proof info to PrivateForest entries\r\n\r\n* One proof for each CID in forest\r\n\r\n* Implement batch proving library functions\r\n\r\n* Revert PrivateForest back to storing just sets of CIDs\r\n\r\n* Refactor out trait PrivateForest\r\n\r\n* Refactor PrivateForest trait to be Rc-independent\r\n\r\n* Move forest-related files into own directory\r\n\r\n* Don't re-expose, implement PrivateForest with and without Rc for HamtForest\r\n\r\n* Implement get_mut in hamt node\r\n\r\n* Implement private forest label proof aggregation\r\n\r\n* Fix mounting\r\n\r\n* Write an example for forest proofs\r\n\r\n* Fix some clippy warnings\r\n\r\n* Fix doctests\r\n\r\n* Move more structs into serializable.rs\r\n\r\n* Fix all clippy warnings.\r\n\r\n* Use once_cell crate instead of std::cell for backwards compat\r\n\r\n* Make clippy happy?\r\n\r\n* Make clippy nightly happy!\r\n\r\n* Possibly fix wasm bindings\r\n\r\n* Uncomment two tests\r\n\r\n* Also prove removals in proving forest\r\n\r\n* Go through some TODOs\r\n\r\n* Zeroize toxic waste\r\n\r\n* Consistently use `CryptoRngCore` instead of `RngCore`\r\n\r\n* Write docstrings\r\n\r\n* Document more & remove unused\r\n\r\n* Organize name accumulator tests\r\n\r\n* Move proofs example to example dir\r\n\r\n* Fix wasm types\r\n\r\n* Tests in proofs.rs\r\n\r\n* Small cleanup and fixes\r\n\r\n* Switch to big endian for BigUints in general (RSA moduli are usually encoded big-endian)\r\n\r\n* Update readme, remove `Namefilter` mentions\r\n\r\n* Canonicalize l hash derivation\r\n\r\n* Correctly encode big integers with padding\r\n\r\n* Add previously failing test case\r\n\r\n* Better domain separation strings\r\n\r\n* Derive the name segment directly from the key\r\n\r\n* Remove redundant `as_ref()` + format\r\n\r\n* Remove `with_seed` and related APIs\r\n\r\n* Use `HamtForest::load` and `store` everywhere\r\n\r\n* Remove generic from `HamtForest`\r\n\r\n* Update wnfs/src/private/keys/privateref.rs\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n* Update wnfs/examples/write_proofs.rs\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n* Rename \"saturated\" into \"revisioned\"\r\n\r\n* Run webpack as a separate CI step\r\n\r\n* Adjust new js test from main\r\n\r\n---------\r\n\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-07-07T17:42:52+02:00",
          "tree_id": "b492ea70796ee22406345162a9b1a674d936631f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a"
        },
        "date": 1688744855929,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8332,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5226646,
            "range": "± 33113",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 203955,
            "range": "± 779",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 230777,
            "range": "± 1070",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51249,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 218938,
            "range": "± 889",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 62927,
            "range": "± 5289",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 280360,
            "range": "± 23351",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::from_digest",
            "value": 2889907,
            "range": "± 329470",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2884547,
            "range": "± 289532",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2570394,
            "range": "± 83140",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1010,
            "range": "± 25",
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
          "id": "394a04899113a662d26a606389a76220cd0eed07",
          "message": "chore: Write example for seeded WNFS using BIP39 mnemonics (#302)\n\n* Write example for seeded WNFS using BIP39 mnemonics\r\n\r\n* Split out example into functions\r\n\r\n* Make example seed-based\r\n\r\n* Switch back to using BIP39 Mnemonics instead of a seed\r\n\r\n* Update wnfs/examples/mnemonic_based.rs\r\n\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\n\r\n---------\r\n\r\nSigned-off-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nCo-authored-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-07-10T13:58:07+02:00",
          "tree_id": "e3a8b26c1de18f217cc6f05b00b3d339d5564c53",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/394a04899113a662d26a606389a76220cd0eed07"
        },
        "date": 1688990545767,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7075,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4539628,
            "range": "± 18982",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 168622,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 193115,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42896,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 180529,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57997,
            "range": "± 4603",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 241728,
            "range": "± 19269",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::from_digest",
            "value": 2315225,
            "range": "± 222112",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2450363,
            "range": "± 231545",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2304953,
            "range": "± 76157",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 813,
            "range": "± 91",
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
          "id": "2a9afd228af544e6ea400530a8f4ae3111595ee2",
          "message": "chore: Remove `wnfs-namefilter` directory (#304)",
          "timestamp": "2023-07-11T17:35:58+02:00",
          "tree_id": "d4e7eb662eb8316466f4065ea8eb708862762510",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2a9afd228af544e6ea400530a8f4ae3111595ee2"
        },
        "date": 1689089998443,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8875,
            "range": "± 711",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5094017,
            "range": "± 325257",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 199463,
            "range": "± 29625",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 219577,
            "range": "± 24027",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 53148,
            "range": "± 4062",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 225770,
            "range": "± 25781",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 66892,
            "range": "± 8814",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 279221,
            "range": "± 37839",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::from_digest",
            "value": 2783992,
            "range": "± 319499",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2715066,
            "range": "± 287461",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2555788,
            "range": "± 148218",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1050,
            "range": "± 269",
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
          "id": "c17f6bb5bc9369d94d1c57cfa66c6cc2adf8174b",
          "message": "feat: Switch from AES-GCM to XChaCha20-Poly1305 (#305)\n\n* Switch to XChaCha20-Poly1305\r\n\r\n* Reduce code duplication & remove `AesKey` struct\r\n\r\n* Fix wnfs-wasm",
          "timestamp": "2023-07-11T17:50:29+02:00",
          "tree_id": "761b5d0ff60000d29cd1b6dbcc18fa1ab48c624a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c17f6bb5bc9369d94d1c57cfa66c6cc2adf8174b"
        },
        "date": 1689090852776,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8177,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 5149013,
            "range": "± 129282",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 188069,
            "range": "± 8907",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 216322,
            "range": "± 8436",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49565,
            "range": "± 767",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 202366,
            "range": "± 8075",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61593,
            "range": "± 6352",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 268019,
            "range": "± 24637",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::from_digest",
            "value": 2893023,
            "range": "± 397373",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2942104,
            "range": "± 403535",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2396809,
            "range": "± 149678",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 840,
            "range": "± 53",
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
          "id": "e164a1fc80c30d9446404a61b05fd995d7d88c0e",
          "message": "feat: Switch from SHA3-256 to BLAKE3-256 (#306)\n\nRelease-As: 0.1.23\r\n\r\n* feat: Switch from SHA3-256 to BLAKE3-256\r\n\r\n* Also use Blake3 by default in `BlockStore::create_cid`\r\n\r\n* Also use Blake3 in skip ratchet key derivation\r\n\r\n* Make use of `blake3::derive_key` algorithm\r\n\r\n* Un-expose temporal key bytes\r\n\r\n* Update domain separation string\r\n\r\n* Update prime hash fixture\r\n\r\n* Fix block naming consistency\r\n\r\n* Dedicated APIs for key structs & cleanup\r\n\r\n* Store a `base_name` in `ExternalFileContent`\r\n\r\nThis ensures you can re-generate all block labels, even if you don't have access to the\r\n PrivateNodeHeader`, e.g. when you only have snapshot access.\r\n\r\n* Lint\r\n\r\n* Give tests more stack space\r\n\r\n* Fix wasm-wnfs\r\n\r\n* Make external file content encoding more spec-adhering\r\n\r\n* Add a hiding segment to `base_name`\r\n\r\n* Depend on released skip ratchet crate",
          "timestamp": "2023-07-21T13:46:05+02:00",
          "tree_id": "d6afac03d2d773d27934809e6dbf8ffbf1c9d53e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e"
        },
        "date": 1689940298194,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7008,
            "range": "± 462",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2733099,
            "range": "± 177469",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 134221,
            "range": "± 8895",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 147507,
            "range": "± 9664",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 52196,
            "range": "± 4135",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 129648,
            "range": "± 8189",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 54821,
            "range": "± 5011",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 160754,
            "range": "± 19235",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2647437,
            "range": "± 324259",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2885362,
            "range": "± 327240",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2511626,
            "range": "± 180515",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 950,
            "range": "± 484",
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
          "id": "00f5b16bc2b97642a9a601f9ab93f4b95f7d0785",
          "message": "Fix `search_latest` on the root directory and implement `searchLatest` in wasm bindings (#310)\n\n* feat: Switch from SHA3-256 to BLAKE3-256\r\n\r\n* Also use Blake3 by default in `BlockStore::create_cid`\r\n\r\n* Also use Blake3 in skip ratchet key derivation\r\n\r\n* Make use of `blake3::derive_key` algorithm\r\n\r\n* Un-expose temporal key bytes\r\n\r\n* Update domain separation string\r\n\r\n* Update prime hash fixture\r\n\r\n* Fix block naming consistency\r\n\r\n* Dedicated APIs for key structs & cleanup\r\n\r\n* Store a `base_name` in `ExternalFileContent`\r\n\r\nThis ensures you can re-generate all block labels, even if you don't have access to the\r\n PrivateNodeHeader`, e.g. when you only have snapshot access.\r\n\r\n* Lint\r\n\r\n* Give tests more stack space\r\n\r\n* Fix wasm-wnfs\r\n\r\n* Make external file content encoding more spec-adhering\r\n\r\n* Add a hiding segment to `base_name`\r\n\r\n* Depend on released skip ratchet crate\r\n\r\n* Write failing test case\r\n\r\n* Make `search_latest` also effect the root dir\r\n\r\n* Implement `PrivateNode.searchLatest` for wasm\r\n\r\n* Whoops! Uncommented some lines",
          "timestamp": "2023-07-21T13:46:30+02:00",
          "tree_id": "71d1bd86405a8a579addcbaa385121b23167c036",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/00f5b16bc2b97642a9a601f9ab93f4b95f7d0785"
        },
        "date": 1689940315120,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6553,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2706600,
            "range": "± 111245",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 128426,
            "range": "± 7717",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 149880,
            "range": "± 8613",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 50024,
            "range": "± 1904",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 119148,
            "range": "± 4143",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 52737,
            "range": "± 6911",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 158421,
            "range": "± 16493",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2532488,
            "range": "± 357041",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2809380,
            "range": "± 313313",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2290311,
            "range": "± 126695",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 863,
            "range": "± 114",
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
          "id": "dae24764bb7294c729ad4487ad6168b24a0d9fd4",
          "message": "chore(rust)(deps): update serde_ipld_dagcbor requirement in /wnfs (#300)\n\nUpdates the requirements on [serde_ipld_dagcbor](https://github.com/ipld/serde_ipld_dagcbor) to permit the latest version.\r\n- [Commits](https://github.com/ipld/serde_ipld_dagcbor/compare/v0.3.0...v0.4.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_ipld_dagcbor\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:05:47+02:00",
          "tree_id": "1974117bbdccf592c728b897d0cd23ea733fda69",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/dae24764bb7294c729ad4487ad6168b24a0d9fd4"
        },
        "date": 1689941326750,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5783,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2465759,
            "range": "± 14545",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 113738,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 124484,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41224,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 110563,
            "range": "± 1071",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48275,
            "range": "± 2489",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 139513,
            "range": "± 10525",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2357847,
            "range": "± 238513",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2484696,
            "range": "± 244972",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2208372,
            "range": "± 84787",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 754,
            "range": "± 16",
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
          "id": "17ac6a7628dc31fa11794118c136322ebf56a476",
          "message": "chore(npm)(deps-dev): bump uint8arrays from 4.0.3 to 4.0.4 in /wnfs-wasm (#290)\n\nBumps [uint8arrays](https://github.com/achingbrain/uint8arrays) from 4.0.3 to 4.0.4.\r\n- [Release notes](https://github.com/achingbrain/uint8arrays/releases)\r\n- [Changelog](https://github.com/achingbrain/uint8arrays/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/achingbrain/uint8arrays/compare/v4.0.3...v4.0.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uint8arrays\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:06:07+02:00",
          "tree_id": "34f70980a8f4ad14595c3efb9bd7ef44a90108a2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/17ac6a7628dc31fa11794118c136322ebf56a476"
        },
        "date": 1689941368897,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6549,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2855227,
            "range": "± 76439",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 132914,
            "range": "± 4814",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 142843,
            "range": "± 3396",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47839,
            "range": "± 2110",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 130095,
            "range": "± 2563",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56520,
            "range": "± 4196",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 163477,
            "range": "± 12982",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2819967,
            "range": "± 313384",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2768553,
            "range": "± 343439",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2531830,
            "range": "± 115181",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 910,
            "range": "± 242",
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
          "id": "543634b37bfd2cf5d0f441d70cea20dbdb29322c",
          "message": "chore(npm)(deps-dev): bump html-webpack-plugin in /wnfs-wasm (#289)\n\nBumps [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin) from 5.5.1 to 5.5.3.\r\n- [Release notes](https://github.com/jantimon/html-webpack-plugin/releases)\r\n- [Changelog](https://github.com/jantimon/html-webpack-plugin/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/jantimon/html-webpack-plugin/compare/v5.5.1...v5.5.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: html-webpack-plugin\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:06:29+02:00",
          "tree_id": "fefec31dff2d0406109e1a5439220ab4ff8a7f4d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/543634b37bfd2cf5d0f441d70cea20dbdb29322c"
        },
        "date": 1689941427210,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6801,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2927992,
            "range": "± 8814",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 137944,
            "range": "± 1652",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 149960,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49431,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 133684,
            "range": "± 693",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57612,
            "range": "± 4005",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 166686,
            "range": "± 12675",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2830955,
            "range": "± 286945",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2903958,
            "range": "± 349363",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2623321,
            "range": "± 109967",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 955,
            "range": "± 63",
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
          "id": "eb17ea2fa03e248a189cb8db04a033ef542f26db",
          "message": "chore(wnfs-nameaccumulator): Initial release at 0.1.23\n\nRelease-As: 0.1.23",
          "timestamp": "2023-07-21T14:10:16+02:00",
          "tree_id": "fefec31dff2d0406109e1a5439220ab4ff8a7f4d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db"
        },
        "date": 1689941968112,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7257,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2924584,
            "range": "± 21642",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 136474,
            "range": "± 1236",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 149598,
            "range": "± 1147",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49845,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 134150,
            "range": "± 989",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58000,
            "range": "± 3988",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 165790,
            "range": "± 13008",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2851688,
            "range": "± 280663",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3030961,
            "range": "± 345556",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2630118,
            "range": "± 105306",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 944,
            "range": "± 28",
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
          "id": "cfb07832e811c79d80e0f5501ea9bf0068fe6c87",
          "message": "chore: release main (#301)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:58:32+02:00",
          "tree_id": "97df8e9009e90c268ec12d17673782f8be8c9f8d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/cfb07832e811c79d80e0f5501ea9bf0068fe6c87"
        },
        "date": 1689944493099,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5535,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2575079,
            "range": "± 8291",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 114319,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 125817,
            "range": "± 276",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 42125,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 111262,
            "range": "± 497",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49867,
            "range": "± 3446",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 146109,
            "range": "± 11211",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2308553,
            "range": "± 207118",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2458979,
            "range": "± 249966",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2336239,
            "range": "± 85571",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 963,
            "range": "± 60",
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
          "id": "070d105d810afb4bd51b21bbbb98abe587d006b8",
          "message": "chore(npm)(deps-dev): bump webpack from 5.87.0 to 5.88.2 in /wnfs-wasm (#312)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.87.0 to 5.88.2.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.87.0...v5.88.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:59:05+02:00",
          "tree_id": "e22c7fc5094c5164a2aa94455eeb07810292a762",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/070d105d810afb4bd51b21bbbb98abe587d006b8"
        },
        "date": 1689944535659,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5679,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2564574,
            "range": "± 8655",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 114613,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 126158,
            "range": "± 565",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41667,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 111824,
            "range": "± 2647",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 50085,
            "range": "± 3518",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 145904,
            "range": "± 11186",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2402359,
            "range": "± 236714",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2481663,
            "range": "± 240198",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2356315,
            "range": "± 79988",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 981,
            "range": "± 24",
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
          "id": "8f1601506ec382d117e06a8ea76f8ab29e9ec9bf",
          "message": "chore(npm)(deps-dev): bump ts-loader from 9.4.3 to 9.4.4 in /wnfs-wasm (#299)\n\nBumps [ts-loader](https://github.com/TypeStrong/ts-loader) from 9.4.3 to 9.4.4.\r\n- [Release notes](https://github.com/TypeStrong/ts-loader/releases)\r\n- [Changelog](https://github.com/TypeStrong/ts-loader/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/TypeStrong/ts-loader/compare/v9.4.3...v9.4.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: ts-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-21T14:59:19+02:00",
          "tree_id": "b12c62ca15e65072cc35e6952b1607ddd03b8e09",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8f1601506ec382d117e06a8ea76f8ab29e9ec9bf"
        },
        "date": 1689944600159,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6051,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2511129,
            "range": "± 15355",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 114460,
            "range": "± 535",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 125392,
            "range": "± 339",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41341,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 112153,
            "range": "± 446",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48389,
            "range": "± 3394",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 140605,
            "range": "± 10601",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2412220,
            "range": "± 223812",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2451144,
            "range": "± 259474",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2213234,
            "range": "± 76783",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 754,
            "range": "± 21",
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
          "id": "254defdfebfa757327453529d1d7a39db30e077e",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#313)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.35.1 to 1.36.1.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.35.1...v1.36.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-24T18:57:27+02:00",
          "tree_id": "e1626e32e64b51b1a8513da0e4323994136fb3b2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/254defdfebfa757327453529d1d7a39db30e077e"
        },
        "date": 1690218085818,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6325,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2551225,
            "range": "± 11563",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 116572,
            "range": "± 819",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 127332,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41696,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 112310,
            "range": "± 14413",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 50133,
            "range": "± 3527",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 142854,
            "range": "± 10926",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2393210,
            "range": "± 267533",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2475248,
            "range": "± 244078",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2233798,
            "range": "± 78344",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 890,
            "range": "± 41",
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
          "id": "e99862825edbbb718d79f6068575a7fda753cec8",
          "message": "chore(npm)(deps-dev): bump typescript from 5.0.4 to 5.1.6 in /wnfs-wasm (#298)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 5.0.4 to 5.1.6.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-24T18:57:40+02:00",
          "tree_id": "da126f6360343a72591cc9bb1cdfaaef1c0b3403",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e99862825edbbb718d79f6068575a7fda753cec8"
        },
        "date": 1690218112974,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7338,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3094092,
            "range": "± 169077",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 148664,
            "range": "± 18523",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 167769,
            "range": "± 9340",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 57348,
            "range": "± 3761",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 140260,
            "range": "± 10859",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58932,
            "range": "± 7318",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 170396,
            "range": "± 29084",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2730353,
            "range": "± 345418",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2829382,
            "range": "± 333034",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2566635,
            "range": "± 204310",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1005,
            "range": "± 430",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "7957636+bgins@users.noreply.github.com",
            "name": "Brian Ginsburg",
            "username": "bgins"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5920442677ea39997ca1e86d224d89b242cb48ca",
          "message": "chore: Update JS binding and packaging (#318)\n\n* Update JS binding and packaging\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nSigned-off-by: Brian Ginsburg <7957636+bgins@users.noreply.github.com>",
          "timestamp": "2023-07-28T08:45:58-07:00",
          "tree_id": "34c6c213b7b96028d54e0f615610841c0f4bf2d2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/5920442677ea39997ca1e86d224d89b242cb48ca"
        },
        "date": 1690559384007,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5711,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2548701,
            "range": "± 7707",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 114374,
            "range": "± 303",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 125595,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41561,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 111349,
            "range": "± 543",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 50158,
            "range": "± 3510",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 145759,
            "range": "± 11027",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2389304,
            "range": "± 228348",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2460352,
            "range": "± 231487",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2355014,
            "range": "± 71894",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1000,
            "range": "± 28",
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
          "id": "55cf2e013cb84cbaab2086c83866f93ecadb0a88",
          "message": "fix: Improve performance of `get_revision_name()` (#317)\n\n* fix: Cache a `revision_name` for better performance\r\n\r\nThis way we're doing fewer nameaccumulator calculations\r\n\r\n* Make sure caches are cleared\r\n\r\n* Make sure to skip comparing caches during `PartialEq`",
          "timestamp": "2023-07-31T14:34:48+02:00",
          "tree_id": "17b10196dd8060eb8a51ef60eee7c0077e150f68",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/55cf2e013cb84cbaab2086c83866f93ecadb0a88"
        },
        "date": 1690807151604,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7418,
            "range": "± 550",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2968649,
            "range": "± 223016",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 144379,
            "range": "± 9070",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 156089,
            "range": "± 10901",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 53753,
            "range": "± 3186",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 135640,
            "range": "± 11780",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 58113,
            "range": "± 8126",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 164147,
            "range": "± 21637",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2455594,
            "range": "± 276274",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2584514,
            "range": "± 294661",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2358351,
            "range": "± 155873",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 890,
            "range": "± 354",
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
          "id": "cebb956cdaf88ed6e2eb09b784eeec5d61bdf4c8",
          "message": "feat: Implement public directory cp & more efficient copy for `PrivateFile` (#319)\n\n* Implement `cp` for public directories\r\n\r\n* Implement leaner copy algorithms\r\n\r\n* Write wasm bindings for public cp & tests",
          "timestamp": "2023-08-02T10:08:54+02:00",
          "tree_id": "95b8232facae7d9d98959c4cb2e31ecf49b7e732",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/cebb956cdaf88ed6e2eb09b784eeec5d61bdf4c8"
        },
        "date": 1690963994727,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6244,
            "range": "± 499",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2561670,
            "range": "± 173547",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 127370,
            "range": "± 8413",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 129852,
            "range": "± 7357",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 46282,
            "range": "± 2747",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 117791,
            "range": "± 8639",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 47987,
            "range": "± 6518",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 138963,
            "range": "± 12348",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2202782,
            "range": "± 221284",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2308534,
            "range": "± 226917",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2036730,
            "range": "± 104873",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 816,
            "range": "± 71",
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
          "id": "89b779cd8257eccdf69089acbda5660498548117",
          "message": "chore: Fix CI: Run `yarn install` before npm publishing & use non-version dev-dependency (#322)\n\n* Run `yarn install` in publish step\r\n\r\n* Don't use `version = ` in dev-dependency with path",
          "timestamp": "2023-08-03T15:33:14+02:00",
          "tree_id": "d6e1e6eecc84560e3dfe11d19224b2699291c384",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/89b779cd8257eccdf69089acbda5660498548117"
        },
        "date": 1691069875014,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6672,
            "range": "± 596",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2779156,
            "range": "± 154655",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 137136,
            "range": "± 8320",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 147337,
            "range": "± 9397",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 51333,
            "range": "± 3130",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 125499,
            "range": "± 7402",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53829,
            "range": "± 7250",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 152526,
            "range": "± 16334",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2277069,
            "range": "± 263983",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2372983,
            "range": "± 287330",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2151807,
            "range": "± 128186",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 818,
            "range": "± 306",
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
          "id": "0cf41ca7b5a31fa09c7acd6389172f0f3904bd02",
          "message": "chore: Install `wasm32-unknown-unknown` in npm publish action (#324)",
          "timestamp": "2023-08-03T16:41:50+02:00",
          "tree_id": "a816aefe1e5e3276ffc10e4f51fcafc815290cf4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0cf41ca7b5a31fa09c7acd6389172f0f3904bd02"
        },
        "date": 1691073908724,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6698,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2936287,
            "range": "± 17324",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 137419,
            "range": "± 748",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 150310,
            "range": "± 546",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49517,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 133726,
            "range": "± 600",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57751,
            "range": "± 4007",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 167420,
            "range": "± 12792",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 3055754,
            "range": "± 378499",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3146968,
            "range": "± 314473",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2734612,
            "range": "± 103175",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1000,
            "range": "± 62",
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
          "id": "aa44005ff7269ef0af745c742e895e4ae6316fa1",
          "message": "chore: Release 0.1.24 (#329)\n\nRelease-As: 0.1.24\r\n\r\nAlso ignore incorrect clippy `needless_pass_by_ref_mut` lint until https://github.com/rust-lang/rust-clippy/pull/11314 is merged.",
          "timestamp": "2023-08-15T11:51:58+02:00",
          "tree_id": "d723105d9e43f9223510cb06f4ca16e171beeed8",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/aa44005ff7269ef0af745c742e895e4ae6316fa1"
        },
        "date": 1692093447828,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6947,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2940369,
            "range": "± 8021",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 135991,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 149086,
            "range": "± 276",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49443,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 132015,
            "range": "± 654",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57307,
            "range": "± 4588",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 166741,
            "range": "± 12745",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 3022092,
            "range": "± 355810",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3096944,
            "range": "± 318667",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2729981,
            "range": "± 138165",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1011,
            "range": "± 28",
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
          "id": "56827830eb75a2d5491001719d932c08059f015d",
          "message": "chore: Try rolling back release-please version",
          "timestamp": "2023-08-15T19:40:31+02:00",
          "tree_id": "e61c4a22ba28ae2ec0b456dcd4cb417a42d467c6",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/56827830eb75a2d5491001719d932c08059f015d"
        },
        "date": 1692121440216,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7016,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2905835,
            "range": "± 42087",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 135464,
            "range": "± 1822",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 147485,
            "range": "± 2167",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48754,
            "range": "± 845",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 132314,
            "range": "± 2312",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57174,
            "range": "± 4155",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 161916,
            "range": "± 13238",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2895679,
            "range": "± 342579",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3057459,
            "range": "± 366322",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2751181,
            "range": "± 119217",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 983,
            "range": "± 254",
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
          "id": "380ee8c7b07a73912100c2689334596e3ad8d9c0",
          "message": "fix: More reliably cache `NameAccumulator` modexps (#326)\n\n* fix: More reliably cache `NameAccumulator` modexps\r\n\r\n* Remove `PrivateNodeHeader::revision_name_cache`\r\n\r\n* Fix test\r\n\r\n* Make `PrivateNodeHeader` equality mountpoint-independent\r\n\r\n* Fix typo",
          "timestamp": "2023-08-17T14:09:42+02:00",
          "tree_id": "42e5503a9036f70fe356c6396e5c0dc818babb92",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/380ee8c7b07a73912100c2689334596e3ad8d9c0"
        },
        "date": 1692274394560,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5336,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2481618,
            "range": "± 6632",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 113812,
            "range": "± 884",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 123917,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41284,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 111207,
            "range": "± 543",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48417,
            "range": "± 2422",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 139731,
            "range": "± 10581",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2509104,
            "range": "± 262830",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2559844,
            "range": "± 266867",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2262020,
            "range": "± 81976",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 800,
            "range": "± 16",
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
          "id": "ca8798bb0d47c5561400ceea3cd01be79e6ac452",
          "message": "chore: data format snapshot tests (#314)\n\n* Add old snapshot impl\r\n\r\n* Update snapshots\r\n\r\n* Add support for snapshotting the entire fs public and private\r\n\r\n* Fix file extarnalcontent name issue\r\n\r\n* Fix non determinism in snapshot tests\r\n\r\n* Fix memoryblockstore serde issue\r\n\r\n* Fix doc\r\n\r\n* Fix wait-timeout dep issue\r\n\r\n* PrivateRef renames and serialization changes\r\n\r\n* Use serde_bytes and serde_byte_array for bytes and vecs\r\n\r\n* serde rename_all\r\n\r\n* Fixes from reviews\r\n\r\n* Fix Ipld decode issue\r\n\r\n* Fix snap files\r\n\r\n* Rebase fix\r\n\r\n---------\r\n\r\nSigned-off-by: Stephen Akinyemi <appcypher@outlook.com>",
          "timestamp": "2023-08-17T18:12:52+01:00",
          "tree_id": "2cfa13c95e909843be74c6db0f941b0cb7c6f7ee",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ca8798bb0d47c5561400ceea3cd01be79e6ac452"
        },
        "date": 1692292587076,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7101,
            "range": "± 478",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2916622,
            "range": "± 223811",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 130123,
            "range": "± 6181",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 142882,
            "range": "± 12477",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49719,
            "range": "± 3036",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 118723,
            "range": "± 7100",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 53883,
            "range": "± 5218",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 161034,
            "range": "± 18323",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2530051,
            "range": "± 291450",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2621512,
            "range": "± 298520",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2326309,
            "range": "± 146297",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 886,
            "range": "± 349",
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
          "id": "3bede29ffa90a7185067185ab080afc46124f143",
          "message": "chore: release main (#330)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-17T20:09:10+02:00",
          "tree_id": "dd45b81209a9908f5fc4b87f76dce064cbd1e66f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/3bede29ffa90a7185067185ab080afc46124f143"
        },
        "date": 1692295931110,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5730,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2546788,
            "range": "± 142945",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 107923,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119338,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41325,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 103480,
            "range": "± 841",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49534,
            "range": "± 3475",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 145541,
            "range": "± 10967",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2341600,
            "range": "± 254515",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2420166,
            "range": "± 245457",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2342423,
            "range": "± 73013",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 981,
            "range": "± 24",
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
          "id": "62b6a809f6a891caa0f4ec1d8ef6d54113f92119",
          "message": "fix: bump package.json version number to 0.1.24 (#331)",
          "timestamp": "2023-08-18T13:23:45+02:00",
          "tree_id": "e59c3b7279c20a3f6ec3d22b0f3cd3f1f277949a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/62b6a809f6a891caa0f4ec1d8ef6d54113f92119"
        },
        "date": 1692358003811,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6093,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2487833,
            "range": "± 7958",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 109151,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119055,
            "range": "± 321",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40749,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 105575,
            "range": "± 431",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48351,
            "range": "± 2467",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 139901,
            "range": "± 10521",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2521372,
            "range": "± 268927",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2616509,
            "range": "± 278124",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2281091,
            "range": "± 81799",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 800,
            "range": "± 15",
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
          "id": "8aea2e028a6c06ad98ff54914cfdb98f74cb247f",
          "message": "chore: release main (#332)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-18T13:38:50+02:00",
          "tree_id": "ac870bb00c1963295858169c9f7100a2b89fdf59",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8aea2e028a6c06ad98ff54914cfdb98f74cb247f"
        },
        "date": 1692358924058,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 7053,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2661165,
            "range": "± 147618",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 120563,
            "range": "± 7693",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 125133,
            "range": "± 8341",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 44042,
            "range": "± 2812",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 111054,
            "range": "± 5523",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 51962,
            "range": "± 5051",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 156101,
            "range": "± 13275",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2804912,
            "range": "± 360727",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2789080,
            "range": "± 370414",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2439526,
            "range": "± 168840",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 825,
            "range": "± 341",
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
          "id": "bc3d7c0b638e40e4072da72edd8279aa0adf9a5c",
          "message": "Update version and domain separation infos (#335)",
          "timestamp": "2023-08-25T12:19:57+02:00",
          "tree_id": "97c913b5914ebcdf546999243268d411eed6216c",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/bc3d7c0b638e40e4072da72edd8279aa0adf9a5c"
        },
        "date": 1692959109058,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5684,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2489582,
            "range": "± 7625",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 108717,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119344,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40281,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 107123,
            "range": "± 622",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 47910,
            "range": "± 2396",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 138749,
            "range": "± 10599",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2563609,
            "range": "± 243415",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2649634,
            "range": "± 270046",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2276132,
            "range": "± 85771",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 781,
            "range": "± 53",
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
          "id": "281e343e3a8294a890c9c62bc3f1180527df625b",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#334)\n\nBumps [@playwright/test](https://github.com/Microsoft/playwright) from 1.36.2 to 1.37.1.\r\n- [Release notes](https://github.com/Microsoft/playwright/releases)\r\n- [Commits](https://github.com/Microsoft/playwright/compare/v1.36.2...v1.37.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-25T11:21:14+01:00",
          "tree_id": "8951f1fd8cddae9df26406ec6235c3cd3977a7d7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/281e343e3a8294a890c9c62bc3f1180527df625b"
        },
        "date": 1692959218553,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6969,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2926378,
            "range": "± 42690",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 130438,
            "range": "± 1695",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 140959,
            "range": "± 1963",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 48262,
            "range": "± 626",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 124564,
            "range": "± 1668",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 55706,
            "range": "± 3877",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 163084,
            "range": "± 12891",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2949736,
            "range": "± 350717",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3043913,
            "range": "± 359045",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2687597,
            "range": "± 119436",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 944,
            "range": "± 256",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "7957636+bgins@users.noreply.github.com",
            "name": "Brian Ginsburg",
            "username": "bgins"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "04df13275ce0aeb5fc75c6b19ec8e8b30d28af48",
          "message": "chore: Update JS package exports (#336)\n\n* chore: Update JS package exports\r\n\r\n* chore: Bump JS wnfs version in `package.json`\r\n\r\n---------\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-08-28T15:32:38+02:00",
          "tree_id": "54ca99a3a2a1130a117c4269b361d26133a1f537",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/04df13275ce0aeb5fc75c6b19ec8e8b30d28af48"
        },
        "date": 1693229813557,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5760,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2572685,
            "range": "± 10138",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 106510,
            "range": "± 1571",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119008,
            "range": "± 399",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 39089,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 105327,
            "range": "± 820",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49680,
            "range": "± 3468",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 144310,
            "range": "± 10952",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2391075,
            "range": "± 269805",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2499324,
            "range": "± 220239",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2346243,
            "range": "± 77340",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 972,
            "range": "± 78",
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
          "id": "dae79cd1b95148cf54d6fdf57357b76adcf192ae",
          "message": "fix: Fix `search_latest` on `write` and file mountpoints (#341)\n\n* fix: Fix `search_latest` on `write` and file mountpoints\r\n\r\n* chore: Rename test",
          "timestamp": "2023-08-30T13:58:26+02:00",
          "tree_id": "88892411eca56a0b41409840812003564e9fb47f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/dae79cd1b95148cf54d6fdf57357b76adcf192ae"
        },
        "date": 1693396961838,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5594,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2483891,
            "range": "± 8052",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 108530,
            "range": "± 496",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 116889,
            "range": "± 485",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40777,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 104237,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 47254,
            "range": "± 2345",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 137405,
            "range": "± 10415",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2513647,
            "range": "± 273635",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2534400,
            "range": "± 325221",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2282648,
            "range": "± 90861",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 781,
            "range": "± 16",
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
          "id": "4b242dfab3743899b966060eddb9d1cfe85911b0",
          "message": "chore: Update `async-once-cell` to v0.5 (#339)",
          "timestamp": "2023-08-30T13:58:47+02:00",
          "tree_id": "07541010b181910236d8611545560dafddd67444",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4b242dfab3743899b966060eddb9d1cfe85911b0"
        },
        "date": 1693396986522,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5130,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2544576,
            "range": "± 13184",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 105825,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 117121,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 39266,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 104131,
            "range": "± 550",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49463,
            "range": "± 3381",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 144912,
            "range": "± 10953",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2392789,
            "range": "± 264983",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2478467,
            "range": "± 244815",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2365094,
            "range": "± 76469",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 963,
            "range": "± 26",
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
          "id": "70d227d43be2202aa65f4f9a21bf0960c8377bd6",
          "message": "chore(npm)(deps-dev): bump typescript from 5.1.6 to 5.2.2 in /wnfs-wasm (#337)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 5.1.6 to 5.2.2.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v5.1.6...v5.2.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-30T13:59:23+02:00",
          "tree_id": "edd7e2801fb93977e0343b7ba1a00d394d4f46e0",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/70d227d43be2202aa65f4f9a21bf0960c8377bd6"
        },
        "date": 1693397074018,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8752,
            "range": "± 464",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3219356,
            "range": "± 167973",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 144042,
            "range": "± 10600",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 158204,
            "range": "± 5960",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 55901,
            "range": "± 2566",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 133474,
            "range": "± 5640",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 60794,
            "range": "± 10087",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 180260,
            "range": "± 21727",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2851665,
            "range": "± 345981",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3052299,
            "range": "± 366781",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2632165,
            "range": "± 145682",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1144,
            "range": "± 131",
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
          "id": "bb653ff9aef7e125279323bcdfe7ba325c466fb0",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#338)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 12.0.1 to 12.1.0.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v12.0.1...v12.1.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-30T13:59:40+02:00",
          "tree_id": "eb181317888b9782a5a1f32d42793fc74bf0388a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/bb653ff9aef7e125279323bcdfe7ba325c466fb0"
        },
        "date": 1693397212872,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5927,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2491062,
            "range": "± 7409",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 108497,
            "range": "± 489",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 117433,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40475,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 105169,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48122,
            "range": "± 3329",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 139820,
            "range": "± 10925",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2506958,
            "range": "± 253528",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2570528,
            "range": "± 290585",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2265207,
            "range": "± 83423",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 772,
            "range": "± 16",
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
          "id": "c497afc20281dd1987ff47e8b4e0d8664588e1cf",
          "message": "chore: release main (#342)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-30T14:30:58+02:00",
          "tree_id": "0944caa5fac31a27c7604356f9ddbfd962db73d4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c497afc20281dd1987ff47e8b4e0d8664588e1cf"
        },
        "date": 1693398846261,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 5587,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2534876,
            "range": "± 9897",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 105983,
            "range": "± 278",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 117766,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 39132,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 103530,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49921,
            "range": "± 3507",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 145815,
            "range": "± 11035",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2399480,
            "range": "± 250538",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2455040,
            "range": "± 270633",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2356002,
            "range": "± 68722",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 990,
            "range": "± 30",
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
          "distinct": false,
          "id": "f39a8004fb133f411ea4aa6c10950d55a4e1835e",
          "message": "chore: Bump versions",
          "timestamp": "2023-09-01T16:25:37+02:00",
          "tree_id": "470baf802938accb65c88eaca4734284ec841325",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/f39a8004fb133f411ea4aa6c10950d55a4e1835e"
        },
        "date": 1693579150512,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 6981,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2911754,
            "range": "± 26588",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 131087,
            "range": "± 877",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 141803,
            "range": "± 1164",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47869,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 124403,
            "range": "± 1101",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56623,
            "range": "± 4036",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 166444,
            "range": "± 12450",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 3014273,
            "range": "± 370953",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3085430,
            "range": "± 350405",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2701608,
            "range": "± 127817",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 944,
            "range": "± 31",
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
          "id": "2d15fbdf61f0461435b1df4339879394859118b5",
          "message": "feat: Implement storing externally encrypted content in `Metadata` (#340)\n\n* feat: Implement storing externally encrypted content in `Metadata`\r\n\r\n* Add `file_variants` to example README.md\r\n\r\n* Remove debug println\r\n\r\n* feat: Also implement `get_metadata_mut`(-`_rc`) and `open_file_mut` on the public side\r\n\r\n* refactor: Implement and use `as_metadata_value`",
          "timestamp": "2023-09-04T14:10:04+02:00",
          "tree_id": "d4fe5bfade2b7d6e78001abe52b68a39223c144e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2d15fbdf61f0461435b1df4339879394859118b5"
        },
        "date": 1693829709871,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8349,
            "range": "± 492",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3171716,
            "range": "± 194985",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 149803,
            "range": "± 8789",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 159422,
            "range": "± 8857",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 54580,
            "range": "± 3333",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 137876,
            "range": "± 8580",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61191,
            "range": "± 6775",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 180015,
            "range": "± 19380",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2751774,
            "range": "± 369497",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2937709,
            "range": "± 324206",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2632153,
            "range": "± 183787",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1166,
            "range": "± 568",
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
          "id": "e6cee873273e154c7855d17e9c756717a635874b",
          "message": "feat: Remove `Share` struct, add documentation, add `rc` constructor variants (#343)\n\n* Cleanup\r\n\r\n* Add rc constructor variants; remove Share struct\r\n\r\n* Fix share fn in wasm\r\n\r\n* Fix wasm code\r\n\r\n* Update doc examples\r\n\r\n* refactor: Rename `rc` functions to `new_rc`\r\n\r\n---------\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-09-04T14:31:43+02:00",
          "tree_id": "f47d249e1e858e4bfbb2a269a77a1de5cde6e336",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e6cee873273e154c7855d17e9c756717a635874b"
        },
        "date": 1693831015154,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 8645,
            "range": "± 754",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3188783,
            "range": "± 158308",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 146310,
            "range": "± 6804",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 159609,
            "range": "± 6550",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 56076,
            "range": "± 2757",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 134378,
            "range": "± 7915",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 61228,
            "range": "± 7394",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 177322,
            "range": "± 22715",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2946249,
            "range": "± 344242",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2909401,
            "range": "± 323735",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2653172,
            "range": "± 186976",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1188,
            "range": "± 77",
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
          "id": "b06fc4c221d72582ba77635050af779f602b0ea2",
          "message": "fix: Don't drop data sometimes during re-serialization in wnfs-hamt (#348)\n\n* chore: Write failing test case\r\n\r\n* fix: Fix reserialization dropping data sometimes",
          "timestamp": "2023-09-10T17:14:42+02:00",
          "tree_id": "edfd1788bdd7052d8ac020a8e16db50498a22112",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b06fc4c221d72582ba77635050af779f602b0ea2"
        },
        "date": 1694359288524,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 84162,
            "range": "± 972",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3787810,
            "range": "± 146627",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 106725,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119007,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 41171,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 104119,
            "range": "± 671",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49677,
            "range": "± 3523",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 145225,
            "range": "± 11030",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2375019,
            "range": "± 211353",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2471137,
            "range": "± 259213",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2334428,
            "range": "± 76810",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 972,
            "range": "± 23",
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
          "id": "6111cf2c23c7287cd11747759236b5b72c90cb28",
          "message": "chore: Remove release please & bump versions (#347)\n\n* chore: Remove release please & bump versions\r\n\r\n* refactor: Use version-fixed dependencies between workspace crates\r\n\r\n* chore: Write wnfs-hamt changelog",
          "timestamp": "2023-09-10T17:32:54+02:00",
          "tree_id": "0207a51ceb7522fb3f2e07442496b1e96b093c77",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6111cf2c23c7287cd11747759236b5b72c90cb28"
        },
        "date": 1694360365934,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 81224,
            "range": "± 951",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3767446,
            "range": "± 146597",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 106032,
            "range": "± 2062",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 118578,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40860,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 103458,
            "range": "± 354",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49664,
            "range": "± 3503",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 144630,
            "range": "± 10777",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2384692,
            "range": "± 225791",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2491606,
            "range": "± 243619",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2338583,
            "range": "± 73046",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 981,
            "range": "± 29",
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
          "id": "491ce8555d811477e934e6a1a6b6e0d347a32357",
          "message": "chore: Bump wnfs-wasm version",
          "timestamp": "2023-09-10T17:40:18+02:00",
          "tree_id": "fc76782648320e087ce6435ea49b821a962c5644",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/491ce8555d811477e934e6a1a6b6e0d347a32357"
        },
        "date": 1694360811842,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 82778,
            "range": "± 1085",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3780265,
            "range": "± 143714",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 107794,
            "range": "± 626",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119379,
            "range": "± 327",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 40682,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 104036,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 49522,
            "range": "± 3497",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 144918,
            "range": "± 11064",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2382198,
            "range": "± 259777",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2391819,
            "range": "± 262510",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2345104,
            "range": "± 77960",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 972,
            "range": "± 21",
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
          "id": "7bac771259abfd491a57fea3fa012682dde0e092",
          "message": "chore(rust)(deps): update quick_cache requirement in /wnfs (#346)\n\nUpdates the requirements on [quick_cache](https://github.com/arthurprs/quick-cache) to permit the latest version.\r\n- [Commits](https://github.com/arthurprs/quick-cache/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: quick_cache\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-25T17:37:42+02:00",
          "tree_id": "26c30e399da8ad5e098fe4378ef6b0b1653b5eb1",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7bac771259abfd491a57fea3fa012682dde0e092"
        },
        "date": 1695656714132,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 78324,
            "range": "± 892",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 3704038,
            "range": "± 148894",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 108054,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 119782,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 43232,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 107699,
            "range": "± 548",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 48607,
            "range": "± 2429",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 141680,
            "range": "± 10702",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2505450,
            "range": "± 257941",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2599214,
            "range": "± 269487",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2277573,
            "range": "± 88553",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 772,
            "range": "± 16",
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
          "id": "6df4a6bf89a71010b7bc15c132512da979ebc438",
          "message": "chore(ci)(deps): bump actions/checkout from 3 to 4 (#350)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 3 to 4.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-25T17:38:07+02:00",
          "tree_id": "351f8f20054064853e3d6903a6144d4c03973cd7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6df4a6bf89a71010b7bc15c132512da979ebc438"
        },
        "date": 1695656790087,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 91633,
            "range": "± 2047",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4292124,
            "range": "± 687552",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 124869,
            "range": "± 1768",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 137077,
            "range": "± 2007",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 47644,
            "range": "± 869",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 123486,
            "range": "± 2300",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 56478,
            "range": "± 4139",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 167848,
            "range": "± 12529",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 3043350,
            "range": "± 362097",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 3111372,
            "range": "± 346810",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2653468,
            "range": "± 109361",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 966,
            "range": "± 184",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "fabrice@desre.org",
            "name": "Fabrice Desré",
            "username": "fabricedesre"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "30ed01b2da55670a9d444f9abf22b052488b73d4",
          "message": "[refactor] Switch from Rc<T> to Arc<T> (#366)",
          "timestamp": "2023-11-07T17:40:56+01:00",
          "tree_id": "d99e25961a8eacd9a835863a8e3a5766db956937",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/30ed01b2da55670a9d444f9abf22b052488b73d4"
        },
        "date": 1699375778629,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 94363,
            "range": "± 6146",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 4531655,
            "range": "± 576686",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 132207,
            "range": "± 4818",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 148658,
            "range": "± 5621",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 49851,
            "range": "± 1994",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 129353,
            "range": "± 5707",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 57768,
            "range": "± 5314",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 167080,
            "range": "± 17473",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 2731085,
            "range": "± 285103",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 2798554,
            "range": "± 250477",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 2541625,
            "range": "± 170231",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 1010,
            "range": "± 228",
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
          "id": "98d43cb8c7979af0996bdad177338df87bfe636b",
          "message": "feat: Make rs-wnfs work in multithreaded contexts (#372)\n\nThe main goal of this PR is to enable using rs-wnfs in multithreaded contexts, e.g. in axum webservers.\r\nWe have a test repo to check whether that works in an MVP: https://github.com/fabricedesre/wnfs-mtload/\r\n\r\nPreviously that wasn't possible, since the async futures from rs-wnfs weren't `Send`, so you couldn't have a multi-threaded work-stealing async runtime work on them.\r\nThe reasons they weren't sync were:\r\n- Futures would capture an `impl BlockStore`, and it's not necessarily known to be `Send`\r\n- Futures would capture an `impl PrivateForest` with the same problem\r\n- Some functions would produce `LocalBoxFuture` or `LocalBoxStream`, which aren't `Send`\r\n- We'd use `async_trait(?Send)` and `async_recursion(?Send)` which opt-in to not having `Send` bounds, since that's what we need for wasm\r\n- Futures would capture internal WNFS data structures like `PrivateNode`, which would use `Rc` internally instead of `Arc`, see also #250 \r\n\r\nSome of this work was already addressed in #366. This PR *should* cover the rest.\r\n\r\n---\r\n\r\nThere's a complication with Wasm, where we're e.g. using an external type `extern \"C\" { type BlockStore; ... }`, which isn't `Send` or `Sync`, and as such can't ever implement a `trait BlockStore: Send + Sync`.\r\nTo fix this, we're conditionally compiling in `Send` and `Sync` bounds (and `Arc` and `Rc` and similar) based on the target (See `send_sync_poly.rs`). This is pretty much just copying what noosphere is doing: https://github.com/subconsciousnetwork/noosphere/blob/main/rust/noosphere-common/src/sync.rs\r\n\r\nI'm hoping eventually we just fix this and thus enable multi-threaded Wasm, too. But for now this works.\r\n\r\n---\r\n\r\n* wip - still need to fix the SnapshotBlockStore implementation\r\n\r\n* Fix SnapshotBlockStore impl\r\n\r\n* Fix wnfs=hamt\r\n\r\n* fix: `Send` bounds & `BytesToIpld` implementations\r\n\r\nAlso: fix formatting\r\n\r\n* feat: Also make `PrivateForest` trait `Send + Sync`\r\n\r\nAlso: Remove unneeded `Sync` bounds left over from previous commits.\r\n\r\n* feat: Use `BoxFuture` instead of `LocalBoxFuture` in `PrivateForest` fn\r\n\r\n* feat: Remove `(?Send)` annotations everywhere\r\n\r\n* feat: Conditionally compile `Send + Sync` bounds\r\n\r\nThis relaxes the requirement if you're not on the `wasm32` target.\r\nThe problem is that foreign types in Wasm don't implement `Sync`.\r\n\r\n* chore: Fix all tests & doctests to use thread-safe RNGs\r\n\r\n---------\r\n\r\nCo-authored-by: Fabrice Desré <fabrice@desre.org>",
          "timestamp": "2023-11-28T19:42:05+01:00",
          "tree_id": "1093d18bc1ae2fbecefbea06c314042270e42bdb",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/98d43cb8c7979af0996bdad177338df87bfe636b"
        },
        "date": 1701197277448,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 63396,
            "range": "± 1472",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2910442,
            "range": "± 145944",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 89267,
            "range": "± 537",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 97057,
            "range": "± 2087",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34283,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 83779,
            "range": "± 1132",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37208,
            "range": "± 2192",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 109526,
            "range": "± 7805",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 1839643,
            "range": "± 173097",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 1786430,
            "range": "± 147831",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 1762806,
            "range": "± 56195",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 500,
            "range": "± 61",
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
          "id": "ce292b5a50823012c02357eb98821053f18c1ce5",
          "message": "feat: Implement `wnfs-unixfs-file` crate for encoding big byte arrays in IPLD (#375)\n\nThis adopts some code from iroh-unixfs (from the beetle codebase, previously called \"iroh\").\r\n\r\nThe WNFS spec uses UnixFS files as the byte array encoding for public WNFS files.\r\n\r\nWe've previously put the burden on anyone using rs-wnfs to encode byte arrays and get a CID themselves, now we've got a mechanism inside rs-wnfs to do that. (E.g. previously we've done byte array en/decoding from javascript via js-ipfs-unixfs)\r\n\r\n---\r\n\r\n* feat: Copy over iroh-unixfs without prost codegen\r\n\r\n* chore: Remove `hamt` stuff\r\n\r\n* refactor: Remove directory support\r\n\r\n* refactor: Remove Symlink support\r\n\r\n* refactor: Remove ability to read from file path\r\n\r\n* refactor: Replace `ContentLoader` with WNFS's `BlockStore`\r\n\r\n* refactor: Rename `UnixfsContentReader` to `..FileReader`\r\n\r\n* refactor: Borrow `BlockStore` instead of cloning\r\n\r\n* refactor: Delete unused structs & code\r\n\r\n* refactor: minor rename `Unixfs` -> `UnixFs`\r\n\r\n* refactor: Remove need to provide name for files\r\n\r\n* refactor: Write a round-trip proptest\r\n\r\n* refactor: Use `BlockStore` to compute hashes & store blocks\r\n\r\n* chore: Make sure `async_std` runtime also works\r\n\r\n* chore: Write README (add add a proptest for seeking)\r\n\r\n* chore: Write more readme\r\n\r\n* chore: Fix typo\r\n\r\n* chore: Fix lint\r\n\r\n* chore: Remove unused dependencies\r\n\r\n* chore: Document crate dependencies in the readme\r\n\r\n* fix: More accurate lifetimes for `AsyncRead` and `AsyncSeek`\r\n\r\n* feat: Add  constructor",
          "timestamp": "2023-11-30T22:16:09+01:00",
          "tree_id": "7b47ae8255782498e6c0653cd08b6bb474bc3c8d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ce292b5a50823012c02357eb98821053f18c1ce5"
        },
        "date": 1701379265395,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 63094,
            "range": "± 2051",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2891880,
            "range": "± 227187",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 89166,
            "range": "± 1398",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 95990,
            "range": "± 2125",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34032,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 82312,
            "range": "± 2282",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36589,
            "range": "± 2776",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 108970,
            "range": "± 6473",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 1807168,
            "range": "± 214188",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 1821946,
            "range": "± 198462",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 1758915,
            "range": "± 60404",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 505,
            "range": "± 14",
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
          "id": "2b67800bd6cc33cd24560540a3f461b0999444e8",
          "message": "feat: Bytes-based public file writing API (#376)\n\nPreviously we only had a CID-based API that just \"set\" the file CID to some value.\r\n\r\nThis now requires public files to encode their content as byte arrays.\r\n\r\nNow the public and private APIs are very similar, both operate on the byte-array level for files.\r\n\r\nMade possible through #375 \r\n\r\n* feat: Write public files using bytes/streams instead of CID\r\n\r\n* feat: Support wasm32 target (non-`Send` futures) in unixfs\r\n\r\n* feat: Write wasm bindings for public file content r&w\r\n\r\n* fix: Write exchange key itself, instead of CID of it\r\n\r\n* chore: Write missing docs",
          "timestamp": "2023-12-05T14:55:58+01:00",
          "tree_id": "ed981a99c1881d9aece0208c1eca7acc3ea9c4e7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/2b67800bd6cc33cd24560540a3f461b0999444e8"
        },
        "date": 1701784872138,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 64197,
            "range": "± 1100",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2954780,
            "range": "± 128179",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 89417,
            "range": "± 1390",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 97507,
            "range": "± 1934",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 34135,
            "range": "± 964",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 82772,
            "range": "± 2418",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36701,
            "range": "± 1884",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 108672,
            "range": "± 6337",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 1789612,
            "range": "± 234101",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 1808115,
            "range": "± 146962",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 1766351,
            "range": "± 58108",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 513,
            "range": "± 13",
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
          "id": "17c14c4284800ea296bd0f28bb6349b1fea0f390",
          "message": "refactor: Introduce `Storable` trait (#378)\n\nMain use case is allowing us to use `Link<UnixFsFile>` as a type in `PublicFile::userland`.\r\nOther use case is allowing us to nest the WNFS Hamt: `Node<String, Node<String, String>>` is now possible (previously it required `K: Serialize` and `V: Serialize` (and we don't generally want to make both `K: AsyncSerialize` and `V: AsyncSerialize`.\r\n\r\nAlso:\r\n- Remove `AsyncSerializable`\r\n- Remove `RemembersCid` (both of these are now handled by `Storable`)\r\n- Remove `TryFrom<Ipld>` instances that were used for deserialization of some types\r\n- Remove `BlockStore::put_async_serializable` function (you can just use `Storable::store` instead)\r\n- Introduce `NodeSerializable` and `HamtSerializable` for `wnfs-hamt` to follow the pattern used in `wnfs`.\r\n\r\n---\r\n\r\n* refactor: Absorb `RemembersCid` into `Storable`\r\n\r\nThis means `Storable` can do all of the logic for `store` and `load` appropriately.\r\n\r\n* refactor: Remove empty `serializable.rs`\r\n\r\n* refactor: Remove `AsyncSerialize`\r\n\r\n* feat: `impl Storable for UnixFsFile`\r\n\r\n* chore: Adjust comment\r\n\r\n* refactor: Adjust wnfs-wasm code\r\n\r\n* chore: Write some docs",
          "timestamp": "2023-12-05T19:04:38+01:00",
          "tree_id": "31baacbea6ace30451169e14bf09c2cc67ea4a7e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/17c14c4284800ea296bd0f28bb6349b1fea0f390"
        },
        "date": 1701799671963,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16541,
            "range": "± 662",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1973544,
            "range": "± 47495",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 42611,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49462,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4127,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57079,
            "range": "± 1424",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36004,
            "range": "± 2191",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102712,
            "range": "± 5602",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new_hashed",
            "value": 1761584,
            "range": "± 156269",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::new(rng)",
            "value": 1826251,
            "range": "± 174338",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::add",
            "value": 1777642,
            "range": "± 60851",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator serialization",
            "value": 520,
            "range": "± 34",
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
          "id": "c407818721bc6057b612a4345eb2adea41737218",
          "message": "feat: Big integer abstraction for Name accumulators & `rug`-based BigInt backend (#373)\n\nThis introduces the `Big` trait in the `wnfs-nameaccumulator` crate which abstracts the type & functions needed from the bigint library to support name accumulators.\r\nThis allows us to experiment with different backends.\r\nIn this I've implemented a `rug` based backend and tested it against the `num-bigint-dig` backend, it improves name accumulator performance by roughly 2x.\r\n\r\n---\r\n\r\n* refactor: Factor out  for bignums trait in name accumulators\r\n\r\n* feat: `rug` backend prototype\r\n\r\n* feat: Give a good compiler error message on exclusive features\r\n\r\n* fix: Reproduce snapshots with abstracted backend\r\n\r\n* chore: Topologically sort blocks in snapshots for better diffs\r\n\r\n* refactor: Only use big-endian in nameaccumulator protocols\r\n\r\n* chore: Add some snapshot test updates\r\n\r\n* feat: Enable `rug` feature for wnfs tests\r\n\r\n* chore: Add note about enabling the `rug` feature",
          "timestamp": "2023-12-06T14:18:15+01:00",
          "tree_id": "da87c74ea5b23ace71278b01f23e3166f0417276",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c407818721bc6057b612a4345eb2adea41737218"
        },
        "date": 1701868995516,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16057,
            "range": "± 613",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1943022,
            "range": "± 33903",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43259,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49736,
            "range": "± 687",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4052,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56225,
            "range": "± 1310",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36577,
            "range": "± 1903",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 103137,
            "range": "± 5736",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1774258,
            "range": "± 198702",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 314095,
            "range": "± 25724",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1810024,
            "range": "± 218607",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599346,
            "range": "± 22661",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1768043,
            "range": "± 60513",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003074,
            "range": "± 27845",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 483,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 664,
            "range": "± 42",
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
          "id": "980cf02411e4856d3dd9b19ffbc6f183a41ed3e6",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#377)\n\nBumps [@playwright/test](https://github.com/microsoft/playwright) from 1.37.1 to 1.40.1.\r\n- [Release notes](https://github.com/microsoft/playwright/releases)\r\n- [Commits](https://github.com/microsoft/playwright/compare/v1.37.1...v1.40.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:18:36+01:00",
          "tree_id": "11aa4739157a8d4c502b837389deb34420fa6695",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/980cf02411e4856d3dd9b19ffbc6f183a41ed3e6"
        },
        "date": 1701869033722,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 17054,
            "range": "± 404",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1974570,
            "range": "± 35239",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 42896,
            "range": "± 941",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49451,
            "range": "± 778",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4213,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56597,
            "range": "± 269",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37214,
            "range": "± 2543",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 105335,
            "range": "± 6034",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1788922,
            "range": "± 231029",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 315642,
            "range": "± 29756",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1808549,
            "range": "± 210336",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 597668,
            "range": "± 37433",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1756326,
            "range": "± 50300",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003296,
            "range": "± 57780",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 700,
            "range": "± 47",
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
          "id": "723571ca4d6860388f7e09294adf987255fc6d0a",
          "message": "chore(npm)(deps-dev): bump ts-loader from 9.4.4 to 9.5.1 in /wnfs-wasm (#371)\n\nBumps [ts-loader](https://github.com/TypeStrong/ts-loader) from 9.4.4 to 9.5.1.\r\n- [Release notes](https://github.com/TypeStrong/ts-loader/releases)\r\n- [Changelog](https://github.com/TypeStrong/ts-loader/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/TypeStrong/ts-loader/compare/v9.4.4...v9.5.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: ts-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:18:51+01:00",
          "tree_id": "96b8a638e392d7b437919cdf79ae1089e5774f64",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/723571ca4d6860388f7e09294adf987255fc6d0a"
        },
        "date": 1701869056508,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16696,
            "range": "± 1883",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1999232,
            "range": "± 47906",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 42629,
            "range": "± 650",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49874,
            "range": "± 3073",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4091,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56619,
            "range": "± 1113",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36921,
            "range": "± 2587",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 103821,
            "range": "± 5686",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1767036,
            "range": "± 178262",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 314128,
            "range": "± 19187",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1818710,
            "range": "± 161758",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 601143,
            "range": "± 25939",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1768087,
            "range": "± 56464",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000789,
            "range": "± 21096",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 503,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 677,
            "range": "± 34",
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
          "id": "426699100c2b1637c4e26f1008aab197582898a5",
          "message": "chore(ci)(deps): bump actions/setup-node from 3 to 4 (#365)\n\nBumps [actions/setup-node](https://github.com/actions/setup-node) from 3 to 4.\r\n- [Release notes](https://github.com/actions/setup-node/releases)\r\n- [Commits](https://github.com/actions/setup-node/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/setup-node\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:19:20+01:00",
          "tree_id": "7e1aa654ed26a581b0b8b0f1a9065bdce75ef1f5",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/426699100c2b1637c4e26f1008aab197582898a5"
        },
        "date": 1701869264293,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16273,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1948854,
            "range": "± 49419",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43151,
            "range": "± 593",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49611,
            "range": "± 638",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4124,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56723,
            "range": "± 1479",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36175,
            "range": "± 1837",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 104267,
            "range": "± 5715",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1788044,
            "range": "± 151580",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 307016,
            "range": "± 27441",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1811152,
            "range": "± 212907",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 603159,
            "range": "± 17419",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1768442,
            "range": "± 58903",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000296,
            "range": "± 39102",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 488,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 664,
            "range": "± 23",
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
          "id": "d35b308f5414bbb20e1c13a00a5c01d33b41df9f",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#364)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 12.1.0 to 12.1.3.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v12.1.0...v12.1.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:19:37+01:00",
          "tree_id": "7b0e7e1d728822df4cf06202f6a1205b728d065a",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/d35b308f5414bbb20e1c13a00a5c01d33b41df9f"
        },
        "date": 1701869275059,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16801,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2026544,
            "range": "± 51356",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 44147,
            "range": "± 1466",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50559,
            "range": "± 454",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4080,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56992,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36373,
            "range": "± 1932",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 104462,
            "range": "± 6198",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1781020,
            "range": "± 180696",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 314685,
            "range": "± 30982",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1826253,
            "range": "± 222492",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600730,
            "range": "± 18454",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1759302,
            "range": "± 55649",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000723,
            "range": "± 19135",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 677,
            "range": "± 29",
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
          "id": "24cbe1fd1336187f9f30721b4b8140ddaf34ba22",
          "message": "chore(npm)(deps-dev): bump wireit from 0.10.0 to 0.14.1 in /wnfs-wasm (#362)\n\nBumps [wireit](https://github.com/google/wireit) from 0.10.0 to 0.14.1.\r\n- [Changelog](https://github.com/google/wireit/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/google/wireit/compare/v0.10.0...v0.14.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: wireit\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:19:54+01:00",
          "tree_id": "b9d9c9d79cc9a92d71488bb7261a383130e843fa",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/24cbe1fd1336187f9f30721b4b8140ddaf34ba22"
        },
        "date": 1701869357136,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16132,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1968744,
            "range": "± 66449",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 42500,
            "range": "± 1700",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 49613,
            "range": "± 756",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4193,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56427,
            "range": "± 460",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 35985,
            "range": "± 1889",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102243,
            "range": "± 6369",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1784708,
            "range": "± 186208",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 309781,
            "range": "± 26975",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1809436,
            "range": "± 184451",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 602215,
            "range": "± 51302",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1790491,
            "range": "± 149465",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000987,
            "range": "± 22440",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 491,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 661,
            "range": "± 17",
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
          "id": "cd45b08c37dc075edcddf3168b857ec3dd07acaf",
          "message": "chore(npm)(deps-dev): bump webpack from 5.88.2 to 5.89.0 in /wnfs-wasm (#359)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.88.2 to 5.89.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.88.2...v5.89.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-06T14:20:09+01:00",
          "tree_id": "71d8919589ab07555bf0f7fe31f5f2fce27b9849",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/cd45b08c37dc075edcddf3168b857ec3dd07acaf"
        },
        "date": 1701869360251,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16383,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1943926,
            "range": "± 31140",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43261,
            "range": "± 1339",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50060,
            "range": "± 882",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4216,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56952,
            "range": "± 1078",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36163,
            "range": "± 2062",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102835,
            "range": "± 6042",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1754215,
            "range": "± 178804",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 317636,
            "range": "± 24204",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1793862,
            "range": "± 200797",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598912,
            "range": "± 48274",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1759221,
            "range": "± 59682",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000357,
            "range": "± 17524",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 480,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 660,
            "range": "± 52",
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
          "id": "0964ac4f6a679c3bd457338f21b1fe90167bc5bb",
          "message": "chore: Bump versions & write CHANGELOGs (#379)",
          "timestamp": "2023-12-06T16:08:06+01:00",
          "tree_id": "686732cd10be173850606492de1a65b9a4b0bf4b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/0964ac4f6a679c3bd457338f21b1fe90167bc5bb"
        },
        "date": 1701875526182,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16691,
            "range": "± 627",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1972023,
            "range": "± 33320",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 44105,
            "range": "± 186",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 51504,
            "range": "± 1465",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4141,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57056,
            "range": "± 1352",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36296,
            "range": "± 2758",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 103620,
            "range": "± 5837",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1758607,
            "range": "± 160569",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 315281,
            "range": "± 26003",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1778408,
            "range": "± 198947",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598846,
            "range": "± 18035",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1754014,
            "range": "± 51059",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1002797,
            "range": "± 39162",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 485,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 667,
            "range": "± 17",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "steven.vandevelde@hey.com",
            "name": "Steven Vandevelde",
            "username": "icidasset"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "db4295eb50092673ad1860e0d1aad3bdbbe8deae",
          "message": "feat: add `get_raw_content_cid` (#385)",
          "timestamp": "2023-12-18T20:08:23+01:00",
          "tree_id": "1dbf7bcb7ab1a7b0a4fa97973746593285b8553d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/db4295eb50092673ad1860e0d1aad3bdbbe8deae"
        },
        "date": 1702926870309,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16525,
            "range": "± 995",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1962176,
            "range": "± 41948",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 42764,
            "range": "± 1079",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50363,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4077,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57283,
            "range": "± 1050",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36227,
            "range": "± 2590",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102769,
            "range": "± 5542",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1802973,
            "range": "± 164534",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312252,
            "range": "± 28336",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1809310,
            "range": "± 186853",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 596805,
            "range": "± 21496",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1770201,
            "range": "± 77723",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001678,
            "range": "± 22218",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 506,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 655,
            "range": "± 25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "steven.vandevelde@hey.com",
            "name": "Steven Vandevelde",
            "username": "icidasset"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "502c4c372a189655b6bed1464f4a552249ae57a2",
          "message": "Match the public and private APIs for content reading (#386)\n\n`wnfs` changes:\r\n- Made the length param optional and renamed the offset param to match the public side.\r\n- Copied the doc test from the public side to the private side.\r\n- Made it so that `PrivateFile.get_content` uses its `read_at` method.\r\n- Added `PublicFile.get_content` to reflect the private side.\r\n- Made `PrivateFile.read_at` take `byte_offset` as `u64` instead of `usize`\r\n\r\n`wnfs-wasm` changes:\r\n- Added `PrivateFile.read_at`\r\n- Made it so that `PrivateFile.get_content` uses its `read_at` method.\r\n- Added `PublicFile.get_content`\r\n- Adjusted tests to use new and changed methods.\r\n\r\n---\r\n\r\n* refactor: (wnfs) make PrivateFile.read_at's length param an option to reflect the public side\r\n\r\n* refactor: `get_content` should use `read_at` and add `get_content` to `PublicFile`\r\n\r\n* feat: (wnfs-wasm) add `read_at` to `PrivateFile` and add `get_content` to `PublicFile`\r\n\r\n* refactor: (wnfs) use u64 type for block indexes\r\n\r\n* fix: (wnfs-wasm) adjust to new `read_at` `byte_offset` param type\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>\r\nSigned-off-by: Steven Vandevelde <steven.vandevelde@hey.com>\r\n\r\n* refactor: make `offset` a `u64` in `can_read_section_of_file`\r\n\r\n* fix: remove unstable let expression\r\n\r\n---------\r\n\r\nSigned-off-by: Steven Vandevelde <steven.vandevelde@hey.com>\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-12-20T17:36:17+01:00",
          "tree_id": "627f8ce7335d95b4f92be2a329d0fdfdd1886d30",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/502c4c372a189655b6bed1464f4a552249ae57a2"
        },
        "date": 1703090457734,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16341,
            "range": "± 390",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1966679,
            "range": "± 40270",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43235,
            "range": "± 1254",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50333,
            "range": "± 3353",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4284,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57015,
            "range": "± 908",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36386,
            "range": "± 1923",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102866,
            "range": "± 5678",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1763979,
            "range": "± 223365",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313169,
            "range": "± 22560",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1828848,
            "range": "± 173529",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598367,
            "range": "± 32445",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1760159,
            "range": "± 62501",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000309,
            "range": "± 17626",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 495,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 652,
            "range": "± 18",
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
          "id": "9eaa315b6876535602d00a76499064b232cfbf65",
          "message": "feat: Expose wasm bindings for `Name` ser/de (#383)",
          "timestamp": "2023-12-20T17:37:11+01:00",
          "tree_id": "3311bc993ef424aca24be37c6c1a6912e8c15bd2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9eaa315b6876535602d00a76499064b232cfbf65"
        },
        "date": 1703090484771,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16451,
            "range": "± 421",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1965413,
            "range": "± 55199",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 44833,
            "range": "± 686",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50480,
            "range": "± 497",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4012,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57665,
            "range": "± 1195",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36940,
            "range": "± 2413",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 103132,
            "range": "± 5648",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1789024,
            "range": "± 172156",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312492,
            "range": "± 29343",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1787667,
            "range": "± 197453",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600195,
            "range": "± 19267",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1754482,
            "range": "± 63161",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000568,
            "range": "± 22065",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 485,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 659,
            "range": "± 21",
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
          "id": "032bdf3a53fdb6ee86fee3a2bfd8e7b58984e698",
          "message": "chore(npm)(deps-dev): bump uint8arrays from 4.0.4 to 5.0.0 in /wnfs-wasm (#382)\n\nBumps [uint8arrays](https://github.com/achingbrain/uint8arrays) from 4.0.4 to 5.0.0.\r\n- [Release notes](https://github.com/achingbrain/uint8arrays/releases)\r\n- [Changelog](https://github.com/achingbrain/uint8arrays/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/achingbrain/uint8arrays/compare/v4.0.4...v5.0.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uint8arrays\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-20T17:37:52+01:00",
          "tree_id": "bf1ed927f3b14a506721e6e16e5e286a97e8617e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/032bdf3a53fdb6ee86fee3a2bfd8e7b58984e698"
        },
        "date": 1703090533581,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16439,
            "range": "± 399",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1952873,
            "range": "± 14176",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43875,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50750,
            "range": "± 620",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4196,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56935,
            "range": "± 567",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36640,
            "range": "± 2079",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102681,
            "range": "± 6006",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1768161,
            "range": "± 165825",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311002,
            "range": "± 26485",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1829245,
            "range": "± 179405",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599850,
            "range": "± 18498",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1761209,
            "range": "± 52176",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003612,
            "range": "± 32810",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 492,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 653,
            "range": "± 20",
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
          "id": "03b2f7eb84fce9dca7ab639a555d27aa227170f2",
          "message": "chore(npm)(deps-dev): bump typescript from 5.2.2 to 5.3.3 in /wnfs-wasm (#380)\n\nBumps [typescript](https://github.com/Microsoft/TypeScript) from 5.2.2 to 5.3.3.\r\n- [Release notes](https://github.com/Microsoft/TypeScript/releases)\r\n- [Commits](https://github.com/Microsoft/TypeScript/compare/v5.2.2...v5.3.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: typescript\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-20T17:38:38+01:00",
          "tree_id": "f626cee061131dbc2d1268fa2c0be988ea67a2c8",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/03b2f7eb84fce9dca7ab639a555d27aa227170f2"
        },
        "date": 1703090692200,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16871,
            "range": "± 256",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1950541,
            "range": "± 29231",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43338,
            "range": "± 233",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50148,
            "range": "± 546",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4133,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 57029,
            "range": "± 1909",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36690,
            "range": "± 2248",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102812,
            "range": "± 5726",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1769932,
            "range": "± 163831",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 314709,
            "range": "± 23212",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1823455,
            "range": "± 196156",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600027,
            "range": "± 23890",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1746647,
            "range": "± 55328",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001363,
            "range": "± 22993",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 654,
            "range": "± 53",
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
          "id": "75bfc119a048107658db9ad6e0cd31e60dd1b158",
          "message": "chore(npm)(deps-dev): bump html-webpack-plugin in /wnfs-wasm (#381)\n\nBumps [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin) from 5.5.3 to 5.5.4.\r\n- [Release notes](https://github.com/jantimon/html-webpack-plugin/releases)\r\n- [Changelog](https://github.com/jantimon/html-webpack-plugin/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/jantimon/html-webpack-plugin/compare/v5.5.3...v5.5.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: html-webpack-plugin\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-20T17:38:17+01:00",
          "tree_id": "7c9f4d980d7a0e4347686dd5af1f505b0cf4b081",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/75bfc119a048107658db9ad6e0cd31e60dd1b158"
        },
        "date": 1703090695280,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16634,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1974499,
            "range": "± 55512",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43345,
            "range": "± 803",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50208,
            "range": "± 821",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4258,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56796,
            "range": "± 883",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36461,
            "range": "± 2207",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 105350,
            "range": "± 6388",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1764261,
            "range": "± 181238",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312632,
            "range": "± 23263",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1805370,
            "range": "± 196334",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600871,
            "range": "± 23485",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1758726,
            "range": "± 56921",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1006957,
            "range": "± 26100",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 497,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 666,
            "range": "± 52",
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
          "id": "c515827806c4a67de0c182bf1f1f6f4510cf9d65",
          "message": "chore(ci)(deps): bump actions/upload-artifact from 3 to 4 (#384)\n\nBumps [actions/upload-artifact](https://github.com/actions/upload-artifact) from 3 to 4.\r\n- [Release notes](https://github.com/actions/upload-artifact/releases)\r\n- [Commits](https://github.com/actions/upload-artifact/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/upload-artifact\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-12-20T17:39:07+01:00",
          "tree_id": "cea61ca497f11b3718e4a12a1f8d28e7e9ed2002",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/c515827806c4a67de0c182bf1f1f6f4510cf9d65"
        },
        "date": 1703090747383,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16622,
            "range": "± 276",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1956815,
            "range": "± 12110",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43196,
            "range": "± 533",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50072,
            "range": "± 571",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4156,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56840,
            "range": "± 606",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36210,
            "range": "± 1870",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 101967,
            "range": "± 5640",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1779150,
            "range": "± 201133",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313947,
            "range": "± 31108",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1785468,
            "range": "± 255390",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599459,
            "range": "± 21186",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1754257,
            "range": "± 53717",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 998617,
            "range": "± 49721",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 491,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 651,
            "range": "± 18",
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
          "id": "a724b801e97cfeb71fa1e5819b702ee45faf6b07",
          "message": "chore(npm)(deps-dev): bump uint8arrays from 5.0.0 to 5.0.1 in /wnfs-wasm (#389)\n\nBumps [uint8arrays](https://github.com/achingbrain/uint8arrays) from 5.0.0 to 5.0.1.\r\n- [Release notes](https://github.com/achingbrain/uint8arrays/releases)\r\n- [Changelog](https://github.com/achingbrain/uint8arrays/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/achingbrain/uint8arrays/compare/v5.0.0...v5.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uint8arrays\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-01-02T10:46:29+01:00",
          "tree_id": "f418e944d076f1f2ad3c7685fd6f4535d2fdf33b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/a724b801e97cfeb71fa1e5819b702ee45faf6b07"
        },
        "date": 1704189160178,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16394,
            "range": "± 645",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1976668,
            "range": "± 39179",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 44565,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50453,
            "range": "± 667",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4531,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56253,
            "range": "± 310",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 35945,
            "range": "± 2145",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 103345,
            "range": "± 5902",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1947440,
            "range": "± 222998",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 310125,
            "range": "± 25311",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1949394,
            "range": "± 203235",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599035,
            "range": "± 18724",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 2097181,
            "range": "± 60680",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003172,
            "range": "± 22601",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 490,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 680,
            "range": "± 86",
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
          "id": "07acdd8fe37aa5a1f517bc0beb5589def75a2cec",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#388)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 12.1.3 to 13.0.0.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v12.1.3...v13.0.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-01-02T10:46:45+01:00",
          "tree_id": "864a830f463deb47e1eae6bd7ae9a11022bb67dc",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/07acdd8fe37aa5a1f517bc0beb5589def75a2cec"
        },
        "date": 1704189171521,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16369,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1937195,
            "range": "± 30274",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45269,
            "range": "± 742",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50690,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4612,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 56134,
            "range": "± 1211",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36050,
            "range": "± 2329",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 102180,
            "range": "± 6150",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1940374,
            "range": "± 226853",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 309502,
            "range": "± 27585",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1954035,
            "range": "± 154455",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598867,
            "range": "± 20570",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 2092149,
            "range": "± 60773",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1004335,
            "range": "± 31319",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 674,
            "range": "± 29",
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
          "id": "aff9cf1109f16f35d9627be8e81d1d43242f67c5",
          "message": "chore(npm)(deps-dev): bump html-webpack-plugin in /wnfs-wasm (#387)\n\nBumps [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin) from 5.5.4 to 5.6.0.\r\n- [Release notes](https://github.com/jantimon/html-webpack-plugin/releases)\r\n- [Changelog](https://github.com/jantimon/html-webpack-plugin/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/jantimon/html-webpack-plugin/compare/v5.5.4...v5.6.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: html-webpack-plugin\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-01-02T10:47:15+01:00",
          "tree_id": "cfa8c1de31c2e56877ca129ed37d31f1b58e464e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/aff9cf1109f16f35d9627be8e81d1d43242f67c5"
        },
        "date": 1704189204006,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16259,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 1946762,
            "range": "± 41310",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 44410,
            "range": "± 634",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50920,
            "range": "± 787",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 4366,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 55869,
            "range": "± 1229",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 35978,
            "range": "± 2051",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 101778,
            "range": "± 6033",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1939340,
            "range": "± 215072",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313972,
            "range": "± 21445",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 2021342,
            "range": "± 177588",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 596718,
            "range": "± 19529",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 2092704,
            "range": "± 59580",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000758,
            "range": "± 52360",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 497,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 680,
            "range": "± 32",
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
          "id": "3eaee3cab4915474f29fcd0b499ae70d396dbbcc",
          "message": "refactor: Evolve `BlockStore` trait (#402)\n\n- Use new RPITIT instead of `async_trait` macro for all traits\r\n- Expose blanket implementation `impl<B: BlockStore> BlockStore for &B` and `Box<B>`\r\n- Add two functions to `trait BlockStore`:\r\n  - `async fn has_block(&self, cid: &Cid) -> Result<bool>;` to find out whether a block is available locally\r\n  - `async fn put_block_keyed(&self, cid: &Cid, bytes: impl Into<Bytes> + CondSend) -> Result<()>;` to add a block with given CID to the blockstore. This allows us to support adding blocks using different hashing functions to the same blockstore.\r\n- Use explicit `BlockStoreError` type in `trait Blockstore` instead of `anyhow::Error`\r\n- Update `rug` dependency to `1.24`\r\n\r\n---\r\n\r\n* feat: Implement `BlockStore` for derefs, too\r\n\r\n* refactor: Use new RPITIT feature for `trait BlockStore`\r\n\r\n* refactor: Move `trait PrivateForest` to RPITIT\r\n\r\n* refactor: Use RPITIT in `trait PrivateKey` & `trait ExchangeKey`\r\n\r\n* refactor: Completely remove `async_trait`\r\n\r\n* chore: Fix warnings, remove unused `IpldEq` trait\r\n\r\n* fix: Update rug & enable std feature\r\n\r\n* feat: don't require `std` for `rug`, more efficient `to_bytes_be`\r\n\r\n* chore: Fix nightly warning\r\n\r\n* refactor: Blanket-impl for `&B` and `Box<B>` instead of `Deref`\r\n\r\nThis way is recommended by dtolnay (https://github.com/rust-lang/api-guidelines/discussions/158)\r\nand the \"Rust for Rustaceans\" book (paragraph \"Ergonomic Trait Implementations\").\r\nThis leads to better compiler error messages when you pass something that doesn't `impl BlockStore` the right way.\r\n`rand_core` explicitly decided against a `DerefMut` blanket implementation for `trait RngCore`.\r\n\r\n* refactor: Remove serializable things from `BlockStore`\r\n\r\nUse the `Storable` trait and its `store` and `load` functions instead.\r\n\r\n* feat: Add `has_block` to `trait BlockStore`\r\n\r\n* fix: Update accesskey snapshot\r\n\r\n* fix: Implement `has_block` for `ForeignBlockStore`\r\n\r\n* feat: Add `get_block_keyed` to `trait BlockStore`, fix wasm\r\n\r\n* refactor: Move blockstore interface close to extern\r\n\r\n* refactor: Use precise error type in `trait BlockStore`\r\n\r\n* feat: Return correct error in `ForeignBlockStore::get_block`\r\n\r\n* refactor: Use `libipld_core::serde::to_ipld` instead of dag-cbor\r\n\r\n* docs: Add comments explaining use of `boxed_fut`",
          "timestamp": "2024-02-15T18:46:40+01:00",
          "tree_id": "8a1390eb3d15efa76085d2897f85061f3a4c0e6b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/3eaee3cab4915474f29fcd0b499ae70d396dbbcc"
        },
        "date": 1708019556021,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16509,
            "range": "± 657",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2269748,
            "range": "± 62579",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46176,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54296,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3201,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62923,
            "range": "± 1571",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38468,
            "range": "± 2061",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114971,
            "range": "± 7352",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1831337,
            "range": "± 158382",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311555,
            "range": "± 26163",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1875653,
            "range": "± 162858",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 596000,
            "range": "± 17238",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1759519,
            "range": "± 56523",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003163,
            "range": "± 21490",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 472,
            "range": "± 344",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 637,
            "range": "± 285",
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
          "id": "4914e8f142305dfe740787c249c934f8f6a8d4c3",
          "message": "chore: Release version 0.2.0 (#403)\n\n* chore: Run CI with our MSRV\r\n\r\n* chore: Write CHANGELOGs and bump to 0.2.0",
          "timestamp": "2024-02-15T20:03:09+01:00",
          "tree_id": "d3d61d48a912bf5bbd95c6c500e80793860e89b7",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/4914e8f142305dfe740787c249c934f8f6a8d4c3"
        },
        "date": 1708024022106,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16165,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2245098,
            "range": "± 36368",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 49398,
            "range": "± 429",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 55057,
            "range": "± 589",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3223,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 64500,
            "range": "± 1897",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38177,
            "range": "± 2115",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114483,
            "range": "± 6660",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1821905,
            "range": "± 189477",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 317595,
            "range": "± 27221",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1830395,
            "range": "± 153804",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 596854,
            "range": "± 18595",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1761379,
            "range": "± 59460",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001967,
            "range": "± 24473",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 498,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 654,
            "range": "± 30",
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
          "id": "3f6cf347bf19158f78b994002a8df1f043019d4a",
          "message": "chore: Fix README dep graph",
          "timestamp": "2024-02-15T20:04:55+01:00",
          "tree_id": "796ba9334e1d5b6641af7a2912e960680ecbc2c9",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/3f6cf347bf19158f78b994002a8df1f043019d4a"
        },
        "date": 1708024146701,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16008,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2259216,
            "range": "± 24042",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46507,
            "range": "± 713",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54982,
            "range": "± 599",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3080,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62995,
            "range": "± 1024",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38540,
            "range": "± 2218",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114835,
            "range": "± 6316",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1815078,
            "range": "± 223054",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 310722,
            "range": "± 29606",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1844075,
            "range": "± 208549",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599865,
            "range": "± 29503",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1765867,
            "range": "± 56061",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000688,
            "range": "± 20658",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 492,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 644,
            "range": "± 25",
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
          "id": "8ac40218e5a66df2aa1f41c1d9244b06c2adfb59",
          "message": "chore: Actually fix README dep graph",
          "timestamp": "2024-02-15T20:05:49+01:00",
          "tree_id": "789d760761f92549e22e13863e1d0dbaadbe9a09",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/8ac40218e5a66df2aa1f41c1d9244b06c2adfb59"
        },
        "date": 1708024191206,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16633,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2269538,
            "range": "± 59212",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 47140,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54261,
            "range": "± 457",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3385,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 64087,
            "range": "± 3257",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38393,
            "range": "± 2110",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115550,
            "range": "± 7607",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1823426,
            "range": "± 178053",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313530,
            "range": "± 26963",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1856179,
            "range": "± 162165",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600010,
            "range": "± 18297",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1756935,
            "range": "± 62672",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001590,
            "range": "± 18834",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 476,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 644,
            "range": "± 29",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "steven.vandevelde@hey.com",
            "name": "Steven Vandevelde",
            "username": "icidasset"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "17c980c5e21b5fbe61840718a8eeec62f2b2a6e2",
          "message": "feat: Add a function to get the exact size of a `PublicFile` and `PrivateFile` (#405)\n\nAdds a `size` function to `PublicFile` and `PrivateFile`, and their wasm equivalents, that returns the exact size of the contents without downloading all the content blocks.\r\n\r\nAdded doc tests for the core lib, and tests for the wasm lib.\r\n\r\n---\r\n\r\n* feat: Add a function to get the exact size of a PublicFile and PrivateFile\r\n\r\n* refactor: Change `get_size` return type to `u64`\r\n\r\n* chore: Fix nightly lints\r\n\r\n* refactor: Rename `get_size` -> `size` in `wnfs` crate\r\n\r\n* chore: Fix nightly lints & wasm\r\n\r\n---------\r\n\r\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2024-02-28T14:14:38+01:00",
          "tree_id": "c1fe38a2ae73dbeeb9bd301a96a3ca0c8d7a06fd",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/17c980c5e21b5fbe61840718a8eeec62f2b2a6e2"
        },
        "date": 1709126447673,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16435,
            "range": "± 520",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2232996,
            "range": "± 18450",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45517,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53467,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3104,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62473,
            "range": "± 550",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37965,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114360,
            "range": "± 7223",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1769264,
            "range": "± 166317",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 314518,
            "range": "± 25426",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1807486,
            "range": "± 216529",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 604969,
            "range": "± 67487",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1776498,
            "range": "± 51016",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1004081,
            "range": "± 24180",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 711,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 649,
            "range": "± 22",
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
          "id": "e54c6d2b0ea693c7981c745139fc6a5de3c3b102",
          "message": "chore: Bump wnfs and wnfs-wasm versions to 0.2.1",
          "timestamp": "2024-02-28T14:28:33+01:00",
          "tree_id": "ae8055c8e0a675235b627a7cf8d6eed7981976e4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e54c6d2b0ea693c7981c745139fc6a5de3c3b102"
        },
        "date": 1709127187260,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 15571,
            "range": "± 573",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2132905,
            "range": "± 61680",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 43824,
            "range": "± 1224",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 50804,
            "range": "± 1313",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3089,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 61375,
            "range": "± 1718",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 36925,
            "range": "± 1971",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 111971,
            "range": "± 7222",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1734297,
            "range": "± 220137",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 305920,
            "range": "± 48390",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1749388,
            "range": "± 195632",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 595209,
            "range": "± 22926",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1711600,
            "range": "± 66058",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 973739,
            "range": "± 29949",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 474,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 628,
            "range": "± 25",
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
          "id": "534c312e6175cf2d1bbb40cfa301fffb6e624e5e",
          "message": "chore: Remove leftover logging value (#406)",
          "timestamp": "2024-02-28T14:53:41+01:00",
          "tree_id": "2061c3194896b9717c4bf517d31bc0b8fe80bee4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/534c312e6175cf2d1bbb40cfa301fffb6e624e5e"
        },
        "date": 1709128670877,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 15944,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2233267,
            "range": "± 22837",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46909,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53714,
            "range": "± 676",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3269,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62055,
            "range": "± 1305",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37817,
            "range": "± 2068",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114036,
            "range": "± 6661",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1816745,
            "range": "± 157452",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312449,
            "range": "± 26139",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1825860,
            "range": "± 192557",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 603417,
            "range": "± 21847",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1779670,
            "range": "± 56808",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003170,
            "range": "± 23583",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 479,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 668,
            "range": "± 20",
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
          "id": "b4bc5e2b2f2a571db0e33e2c4e432c326344f90f",
          "message": "feat: Allow (but don't require) overwriting `putBlock` in JS (#409)\n\n* feat: Allow (but don't require) overwriting `putBlock` in JS\r\n\r\n* chore: Write a test for overwriting the `putBlock` method",
          "timestamp": "2024-03-08T10:39:16+01:00",
          "tree_id": "ae1903e44d947d564b4018e1be93f6360a83873f",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b4bc5e2b2f2a571db0e33e2c4e432c326344f90f"
        },
        "date": 1709891027571,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16209,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2258952,
            "range": "± 75693",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46660,
            "range": "± 747",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 52642,
            "range": "± 610",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3099,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62312,
            "range": "± 1284",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38266,
            "range": "± 1977",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115840,
            "range": "± 6626",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1780585,
            "range": "± 173674",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 317749,
            "range": "± 26434",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1810004,
            "range": "± 145130",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 602544,
            "range": "± 14870",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1786264,
            "range": "± 57620",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003064,
            "range": "± 22022",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 486,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 648,
            "range": "± 22",
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
          "id": "ae2a2704dfd806c2d52678426cf7b4a149fef28b",
          "message": "chore: Bump `wnfs` and `wnfs-wasm` to 0.2.2 (#410)\n\n* chore: Fix lint\r\n\r\n* chore: Bump `wnfs-wasm` and `wnfs` to version 0.2.2",
          "timestamp": "2024-03-08T11:38:54+01:00",
          "tree_id": "3e7cc494ed123c0b58c99aafff317101ab8ee73b",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/ae2a2704dfd806c2d52678426cf7b4a149fef28b"
        },
        "date": 1709894607356,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16910,
            "range": "± 307",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2250847,
            "range": "± 36616",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46678,
            "range": "± 498",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53507,
            "range": "± 608",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3125,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 64837,
            "range": "± 2104",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37807,
            "range": "± 2259",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114279,
            "range": "± 6709",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1784433,
            "range": "± 188033",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 310450,
            "range": "± 26522",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1795251,
            "range": "± 176349",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600386,
            "range": "± 19260",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1777578,
            "range": "± 58451",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 999985,
            "range": "± 22204",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 499,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 648,
            "range": "± 16",
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
          "id": "97fa9fa1967c0904d3459ee8ac3eb2658786c669",
          "message": "chore(rust)(deps): update testresult requirement in /wnfs (#411)\n\nUpdates the requirements on [testresult](https://github.com/wiktor-k/testresult) to permit the latest version.\r\n- [Commits](https://github.com/wiktor-k/testresult/compare/v0.3.0...v0.4.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: testresult\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:04:28+01:00",
          "tree_id": "d55a6766d8899c57715a9e916a79c9972e4de16e",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/97fa9fa1967c0904d3459ee8ac3eb2658786c669"
        },
        "date": 1710756623577,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16613,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2239504,
            "range": "± 48021",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45762,
            "range": "± 564",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53310,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3119,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62213,
            "range": "± 1053",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37679,
            "range": "± 1997",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115707,
            "range": "± 6633",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1834614,
            "range": "± 245503",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312676,
            "range": "± 34141",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1833126,
            "range": "± 198230",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 597161,
            "range": "± 17121",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1758909,
            "range": "± 51427",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1002064,
            "range": "± 24743",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 497,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 664,
            "range": "± 26",
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
          "id": "e9e930bd3f5ca3ee3c98d68dcb945b1937cca9fd",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#408)\n\nBumps [@playwright/test](https://github.com/microsoft/playwright) from 1.40.1 to 1.42.1.\r\n- [Release notes](https://github.com/microsoft/playwright/releases)\r\n- [Commits](https://github.com/microsoft/playwright/compare/v1.40.1...v1.42.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:05:00+01:00",
          "tree_id": "3e50e986fcb65da353723fd82ed3e70f015294b2",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e9e930bd3f5ca3ee3c98d68dcb945b1937cca9fd"
        },
        "date": 1710756653737,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16590,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2244330,
            "range": "± 53567",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46379,
            "range": "± 1725",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53973,
            "range": "± 508",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3145,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62251,
            "range": "± 1067",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37648,
            "range": "± 2030",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114829,
            "range": "± 14186",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1836682,
            "range": "± 179969",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 315438,
            "range": "± 24357",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1862075,
            "range": "± 182075",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 603115,
            "range": "± 18576",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1766355,
            "range": "± 61226",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000994,
            "range": "± 18595",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 474,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 672,
            "range": "± 33",
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
          "id": "9545354b9f8cfee9466caae348e66d8123c3d5b5",
          "message": "chore(npm)(deps-dev): bump multiformats in /wnfs-wasm (#404)\n\nBumps [multiformats](https://github.com/multiformats/js-multiformats) from 13.0.0 to 13.1.0.\r\n- [Release notes](https://github.com/multiformats/js-multiformats/releases)\r\n- [Changelog](https://github.com/multiformats/js-multiformats/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/multiformats/js-multiformats/compare/v13.0.0...v13.1.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: multiformats\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:31:11+01:00",
          "tree_id": "6070d24775b60db8c25717d8a916002ca2225185",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/9545354b9f8cfee9466caae348e66d8123c3d5b5"
        },
        "date": 1710758118330,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 15981,
            "range": "± 672",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2239925,
            "range": "± 43773",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45683,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 52819,
            "range": "± 609",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3085,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62937,
            "range": "± 1626",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37612,
            "range": "± 1936",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115686,
            "range": "± 6984",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1865163,
            "range": "± 190898",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311049,
            "range": "± 30641",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1826697,
            "range": "± 206721",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598989,
            "range": "± 17248",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1769234,
            "range": "± 72586",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1002255,
            "range": "± 20852",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 648,
            "range": "± 39",
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
          "id": "80d53e0cf906d2f188fc15abf7fb270412e48dcb",
          "message": "chore(ci)(deps): bump codecov/codecov-action from 3 to 4 (#401)\n\nBumps [codecov/codecov-action](https://github.com/codecov/codecov-action) from 3 to 4.\r\n- [Release notes](https://github.com/codecov/codecov-action/releases)\r\n- [Changelog](https://github.com/codecov/codecov-action/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/codecov/codecov-action/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: codecov/codecov-action\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:31:27+01:00",
          "tree_id": "3dc72ba1e04d63293339074da8692d9ee6fb1d96",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/80d53e0cf906d2f188fc15abf7fb270412e48dcb"
        },
        "date": 1710758126332,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16334,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2253445,
            "range": "± 28629",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45920,
            "range": "± 208",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54000,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3084,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63294,
            "range": "± 2632",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38471,
            "range": "± 2612",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 113913,
            "range": "± 6321",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1822561,
            "range": "± 172877",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312690,
            "range": "± 32969",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1868620,
            "range": "± 183666",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600601,
            "range": "± 18242",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1761264,
            "range": "± 49190",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003114,
            "range": "± 23252",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 495,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 653,
            "range": "± 29",
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
          "id": "b425d8eaf3be6a91919c16dd51475219e039667a",
          "message": "chore(npm)(deps-dev): bump css-loader from 6.8.1 to 6.10.0 in /wnfs-wasm (#400)\n\nBumps [css-loader](https://github.com/webpack-contrib/css-loader) from 6.8.1 to 6.10.0.\r\n- [Release notes](https://github.com/webpack-contrib/css-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/css-loader/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/css-loader/compare/v6.8.1...v6.10.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: css-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:31:40+01:00",
          "tree_id": "3d19a468061d350b49e21722acd94bae83ecfc55",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/b425d8eaf3be6a91919c16dd51475219e039667a"
        },
        "date": 1710758137728,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 17607,
            "range": "± 593",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2256384,
            "range": "± 29925",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46358,
            "range": "± 1238",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 52259,
            "range": "± 705",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3042,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62107,
            "range": "± 3903",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37594,
            "range": "± 2137",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 113803,
            "range": "± 6557",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1844166,
            "range": "± 170977",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 318282,
            "range": "± 31314",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1858987,
            "range": "± 184900",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 605377,
            "range": "± 15077",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1774803,
            "range": "± 62691",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000431,
            "range": "± 19296",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 474,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 651,
            "range": "± 29",
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
          "id": "889b64f9fade5977984d2a4e6d58fc601834ede5",
          "message": "chore(npm)(deps-dev): bump wireit from 0.14.1 to 0.14.4 in /wnfs-wasm (#398)\n\nBumps [wireit](https://github.com/google/wireit) from 0.14.1 to 0.14.4.\r\n- [Changelog](https://github.com/google/wireit/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/google/wireit/compare/v0.14.1...v0.14.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: wireit\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:31:54+01:00",
          "tree_id": "45e66cc4ce77b131dc8e4974c58b066abcf9591d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/889b64f9fade5977984d2a4e6d58fc601834ede5"
        },
        "date": 1710758282556,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16723,
            "range": "± 902",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2276143,
            "range": "± 24049",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46267,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53338,
            "range": "± 2072",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3169,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 61954,
            "range": "± 1793",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37933,
            "range": "± 2045",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114473,
            "range": "± 6621",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1835404,
            "range": "± 179761",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311991,
            "range": "± 22078",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1866931,
            "range": "± 157128",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 606688,
            "range": "± 22977",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1769471,
            "range": "± 79655",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001073,
            "range": "± 22827",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 486,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 653,
            "range": "± 30",
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
          "id": "693c3cbaca0d5869cbb166e19c909fc235c9e68d",
          "message": "chore(npm)(deps-dev): bump style-loader in /wnfs-wasm (#394)\n\nBumps [style-loader](https://github.com/webpack-contrib/style-loader) from 3.3.3 to 3.3.4.\r\n- [Release notes](https://github.com/webpack-contrib/style-loader/releases)\r\n- [Changelog](https://github.com/webpack-contrib/style-loader/blob/v3.3.4/CHANGELOG.md)\r\n- [Commits](https://github.com/webpack-contrib/style-loader/compare/v3.3.3...v3.3.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: style-loader\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T11:35:13+01:00",
          "tree_id": "c17f8b5d69d561aefbe6228255d0cc5888cd63e6",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/693c3cbaca0d5869cbb166e19c909fc235c9e68d"
        },
        "date": 1710758369541,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16152,
            "range": "± 513",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2251801,
            "range": "± 29728",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46516,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53906,
            "range": "± 648",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3074,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 61987,
            "range": "± 3200",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37916,
            "range": "± 1998",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114196,
            "range": "± 6977",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1819378,
            "range": "± 202624",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 310442,
            "range": "± 34407",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1876830,
            "range": "± 215014",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 598446,
            "range": "± 19792",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1749999,
            "range": "± 67793",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001321,
            "range": "± 25943",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 492,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 654,
            "range": "± 27",
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
          "id": "eda6849542a66fea7d9e038cd3a7d8595147a651",
          "message": "chore(rust)(deps): update env_logger requirement in /wnfs (#395)\n\nUpdates the requirements on [env_logger](https://github.com/rust-cli/env_logger) to permit the latest version.\r\n- [Release notes](https://github.com/rust-cli/env_logger/releases)\r\n- [Changelog](https://github.com/rust-cli/env_logger/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-cli/env_logger/compare/v0.10.0...v0.11.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: env_logger\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-03-18T12:03:03+01:00",
          "tree_id": "7bffb087b2546e91007777dd237e2be4c158336c",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/eda6849542a66fea7d9e038cd3a7d8595147a651"
        },
        "date": 1710760019355,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16605,
            "range": "± 614",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2241570,
            "range": "± 26919",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45747,
            "range": "± 960",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53653,
            "range": "± 553",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3080,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63777,
            "range": "± 1221",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38140,
            "range": "± 1999",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115975,
            "range": "± 6549",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1830771,
            "range": "± 143824",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313396,
            "range": "± 33455",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1844748,
            "range": "± 154984",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600320,
            "range": "± 30837",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1771313,
            "range": "± 62288",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000877,
            "range": "± 19872",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 488,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 652,
            "range": "± 18",
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
          "id": "87839b7d7ab5ca673a2fb7cab5a9b9bc44d22dc6",
          "message": "chore(rust)(deps): update serde_ipld_dagcbor requirement in /wnfs (#419)\n\nUpdates the requirements on [serde_ipld_dagcbor](https://github.com/ipld/serde_ipld_dagcbor) to permit the latest version.\r\n- [Commits](https://github.com/ipld/serde_ipld_dagcbor/compare/v0.4.0...v0.6.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_ipld_dagcbor\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-04-02T11:33:49+02:00",
          "tree_id": "46b3e585f91d84957d613ddee3743652f9595b93",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/87839b7d7ab5ca673a2fb7cab5a9b9bc44d22dc6"
        },
        "date": 1712050791666,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16212,
            "range": "± 526",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2224123,
            "range": "± 105795",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46293,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 52913,
            "range": "± 828",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3173,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62999,
            "range": "± 2256",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37346,
            "range": "± 1895",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 113400,
            "range": "± 6224",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1827569,
            "range": "± 178490",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 316498,
            "range": "± 26444",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1884060,
            "range": "± 187641",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600515,
            "range": "± 25197",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1781053,
            "range": "± 58149",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 998714,
            "range": "± 27828",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 487,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 646,
            "range": "± 43",
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
          "id": "1554bb3cfb85ee5ee42fd9d2a2fc5320826cce03",
          "message": "feat: Preliminary `RootTree` APIs (#390)\n\n* feat: Expose & work on root tree module\r\n\r\n* feat: Add private root storing & loading in `RootTree`\r\n\r\nAlso:\r\n- Fix nameaccumulator lib not working when the \"rug\" features is on, but \"num-bigint-dig\" is off.\r\n- Avoid `.unwrap()` in `AccessKey`.\r\n\r\n* feat: `#[derive(Clone)] RootTree`\r\n\r\n* chore: Remove left-over debugging `println!`\r\n\r\n* fix: Make `RootTree` snapshot test deterministic\r\n\r\n* fix: Wasm compilation\r\n\r\n* feat: Add `PublicFile::size` function\r\n\r\n* fix: Handle greater than file size read limits\r\n\r\n* fix: Filesize computation\r\n\r\n* chore: Hide `root_tree` module from documentation\r\n\r\nIt's not yet documented/finished.",
          "timestamp": "2024-04-14T13:38:49+02:00",
          "tree_id": "b6aabea6c1f8cc1a5a1758280ed46adecd733b19",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/1554bb3cfb85ee5ee42fd9d2a2fc5320826cce03"
        },
        "date": 1713095087172,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16343,
            "range": "± 703",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2242944,
            "range": "± 50411",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 48362,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54746,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3221,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62819,
            "range": "± 1657",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38078,
            "range": "± 1979",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115933,
            "range": "± 7113",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1823181,
            "range": "± 135178",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 317121,
            "range": "± 25365",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1889283,
            "range": "± 453650",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599530,
            "range": "± 42379",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1757867,
            "range": "± 61008",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001916,
            "range": "± 19593",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 492,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 639,
            "range": "± 20",
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
          "id": "bae20c9eb461186cd6a204995fc5791c48212b9d",
          "message": "chore(npm)(deps-dev): bump uint8arrays from 5.0.1 to 5.0.3 in /wnfs-wasm (#414)\n\nBumps [uint8arrays](https://github.com/achingbrain/uint8arrays) from 5.0.1 to 5.0.3.\r\n- [Release notes](https://github.com/achingbrain/uint8arrays/releases)\r\n- [Changelog](https://github.com/achingbrain/uint8arrays/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/achingbrain/uint8arrays/compare/v5.0.1...v5.0.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uint8arrays\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-04-14T13:40:21+02:00",
          "tree_id": "646e7d0e990e37ce017b83108281c98d28e3a345",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/bae20c9eb461186cd6a204995fc5791c48212b9d"
        },
        "date": 1713095286580,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16497,
            "range": "± 648",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2271604,
            "range": "± 65725",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46926,
            "range": "± 935",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54019,
            "range": "± 673",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3058,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 64097,
            "range": "± 6200",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37786,
            "range": "± 2012",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 114360,
            "range": "± 6616",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1852060,
            "range": "± 149243",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 306240,
            "range": "± 25051",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1848792,
            "range": "± 194168",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599919,
            "range": "± 21663",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1774846,
            "range": "± 61541",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001463,
            "range": "± 25272",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 484,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 639,
            "range": "± 34",
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
          "id": "6882a8c327151713b69292d87aaed1df133f1b35",
          "message": "chore(npm)(deps-dev): bump webpack from 5.89.0 to 5.91.0 in /wnfs-wasm (#417)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.89.0 to 5.91.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.89.0...v5.91.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-04-14T13:40:53+02:00",
          "tree_id": "f67d158f795f37bd9afd460a9d8b03a822593564",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/6882a8c327151713b69292d87aaed1df133f1b35"
        },
        "date": 1713095359340,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16453,
            "range": "± 316",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2264276,
            "range": "± 46285",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46393,
            "range": "± 1041",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53558,
            "range": "± 536",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3291,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63387,
            "range": "± 1178",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37988,
            "range": "± 2013",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115972,
            "range": "± 7222",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1840523,
            "range": "± 159214",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 309602,
            "range": "± 26961",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1847595,
            "range": "± 201387",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 599049,
            "range": "± 29674",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1758661,
            "range": "± 59030",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1006340,
            "range": "± 19834",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 502,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 642,
            "range": "± 32",
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
          "id": "7931ad6d7127aeeb15a3325f4aac94e353efd524",
          "message": "chore(npm)(deps-dev): bump @playwright/test in /wnfs-wasm (#422)\n\nBumps [@playwright/test](https://github.com/microsoft/playwright) from 1.42.1 to 1.43.0.\r\n- [Release notes](https://github.com/microsoft/playwright/releases)\r\n- [Commits](https://github.com/microsoft/playwright/compare/v1.42.1...v1.43.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@playwright/test\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-04-14T13:41:40+02:00",
          "tree_id": "553964351f408e04ea22dcb7f9531559aeddf43d",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/7931ad6d7127aeeb15a3325f4aac94e353efd524"
        },
        "date": 1713095470081,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 17473,
            "range": "± 767",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2306190,
            "range": "± 28151",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46953,
            "range": "± 1202",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53443,
            "range": "± 2400",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3100,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63139,
            "range": "± 1365",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 37970,
            "range": "± 1943",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 113856,
            "range": "± 6416",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1834445,
            "range": "± 182572",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312883,
            "range": "± 22373",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1892682,
            "range": "± 173638",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600862,
            "range": "± 22326",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1767465,
            "range": "± 55445",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1003101,
            "range": "± 25148",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 507,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 642,
            "range": "± 37",
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
          "id": "dc0785a907b2c2fe80e3e3130fe660c1f5ab4c01",
          "message": "feat: implement `PublicDirectory` conflict reconciliation (#426)\n\nThis implements three functions:\r\n- `PublicDirectory::reconcile` which takes two directories & reconciles all changes between them. Think of this as similar to `get pull`, but where conflicts in files are merged automatically via a simple tie-breaking mechanism on the file hash. The return value indicates whether and which files were affected by the automatic merge, if at all.\r\n- `PublicDirectory::merge`, the underlying function for `reconcile`, but which doesn't take into account if one of the directories may have been \"ahead\" in history. Use only if you know what you're doing, otherwise opt for `reconcile`.\r\n- `PublicNode::causal_compare`, the underlying function in `reconcile` that figures out whether one version of a node is \"ahead\" of another or behind, or if they're two conflicting versions and need to be merged.\r\n\r\n---\r\n\r\n* feat: Conflict reconciliation for PublicDirectory (first draft)\r\n\r\n* fix: Use `async_recursion`\r\n\r\n* refactor: Use more `prop_assert`\r\n\r\n* chore: Write proptests for `causal_compare`\r\n\r\n* chore: Write tests for `PublicDirectory::merge`\r\n\r\n* chore: Write documentation\r\n\r\n* fix: Consistently merge metadata between nodes\r\n\r\n* fix: Test types\r\n\r\n* chore: Remove unused imports\r\n\r\n* fix: Bugs in merge implementation\r\n\r\nSpecifically:\r\n- trivially merge exactly equal nodes without creating a history entry\r\n- correctly reset `persisted_as` when creating a merge node\r\n- always advance the history entry when creating a merge node\r\n\r\n* fix: Don't merge equal files",
          "timestamp": "2024-04-18T15:11:04+02:00",
          "tree_id": "fc1370b2619bb0bcd4dd608ae3ee4736d20de3e4",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/dc0785a907b2c2fe80e3e3130fe660c1f5ab4c01"
        },
        "date": 1713446132229,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16263,
            "range": "± 296",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2244809,
            "range": "± 22276",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 47606,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54228,
            "range": "± 1261",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3126,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 62896,
            "range": "± 3014",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38250,
            "range": "± 1941",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115156,
            "range": "± 6517",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1771527,
            "range": "± 236873",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 312891,
            "range": "± 32210",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1806129,
            "range": "± 158589",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 596779,
            "range": "± 37863",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1754967,
            "range": "± 62274",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1000895,
            "range": "± 46375",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 496,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 641,
            "range": "± 23",
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
          "id": "a0bf23c0b5ac3956ec4120e008d920f9f109e4b7",
          "message": "fix: Public conflict reconciliation edge case on files with equal content but different metadata (#430)\n\n* chore: Improve public dir proptests to cover metadata\r\n\r\n* fix: generate better file system cases in public dire proptests\r\n\r\n* fix: public directory merge commutativity test case\r\n\r\nThe failing case was having two files with identical content, but\r\ndifferent metadata were merging non-commutativiely.\r\n\r\n* refactor: Extract out `PublicFile::merge` from `PublicDirectory::merge_helper`\r\n\r\n* chore: Avoid deprecated chrono functions\r\n\r\n* fix: `File::set_content` parameter order",
          "timestamp": "2024-04-23T16:38:06+02:00",
          "tree_id": "1dc33439b12696de41128cb7155d316269d85dcc",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/a0bf23c0b5ac3956ec4120e008d920f9f109e4b7"
        },
        "date": 1713883342382,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16209,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2251456,
            "range": "± 34505",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46130,
            "range": "± 710",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54749,
            "range": "± 716",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3177,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63212,
            "range": "± 2719",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38090,
            "range": "± 2581",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 116922,
            "range": "± 6249",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1766016,
            "range": "± 155902",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311950,
            "range": "± 21832",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1879406,
            "range": "± 184498",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 597933,
            "range": "± 18638",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1760707,
            "range": "± 61725",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1002945,
            "range": "± 17053",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 496,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 642,
            "range": "± 34",
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
          "id": "abfefefbb63aec99633811c45fe33343c976ff73",
          "message": "feat: Implement conflict reconciliation for private directories (#431)\n\n* feat: Implement conflict reconciliation for private directories\r\n\r\n* fix: Make `header` public again\r\n\r\n* chore: write basic reconciliation unit tests\r\n\r\n* chore: Write proptest state machine utilities & proptest private reconciliation\r\n\r\n* chore: Write better `Debug` impls\r\n\r\n* feat: Prioritize directories over files during private directory merge\r\n\r\n* chore: Try extracting out a failing testcase\r\n\r\n* fix: Regression test & mock file system merge\r\n\r\n* chore: Found failing test case\r\n\r\n* chore: Try associating replicas with a vector clock\r\n\r\n* refactor: Don't try to state machine proptest private reconciliation\r\n\r\n* refactor: Remove unused dependency\r\n\r\n* feat: Commit `Cargo.lock` and update lockfile\r\n\r\n* chore: docs",
          "timestamp": "2024-04-25T16:00:52+02:00",
          "tree_id": "6bb2b313f3cdabd221b103e591c668b5d0cfa039",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/abfefefbb63aec99633811c45fe33343c976ff73"
        },
        "date": 1714053913381,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 16912,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2267680,
            "range": "± 25285",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 45417,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 53716,
            "range": "± 605",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3156,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 64613,
            "range": "± 3568",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38988,
            "range": "± 2304",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 115774,
            "range": "± 6618",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1806954,
            "range": "± 169105",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 311827,
            "range": "± 24490",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1803252,
            "range": "± 172243",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 600638,
            "range": "± 71462",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1771986,
            "range": "± 56324",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001885,
            "range": "± 25704",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 482,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 646,
            "range": "± 40",
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
          "id": "e265c94a2cb7b7f785570bd18ef21946ad5a7fa9",
          "message": "fix: Public reconciliation of a concurrent write and remove (of independent paths) (#432)\n\n* chore: Write failing test case\r\n\r\nAlso: Add better debug impls for `PublicFile`, `PublicFile`, `PublicLink`, etc.\r\n\r\n* fix: Concurrent write and remove should reconcile correctly",
          "timestamp": "2024-04-25T15:08:14Z",
          "tree_id": "d9421267d5f7dfdcb74339951f81f41d288499c0",
          "url": "https://github.com/wnfs-wg/rs-wnfs/commit/e265c94a2cb7b7f785570bd18ef21946ad5a7fa9"
        },
        "date": 1714057925170,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 15953,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "node set 1000 consecutive",
            "value": 2278449,
            "range": "± 68690",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get",
            "value": 46270,
            "range": "± 611",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove",
            "value": 54007,
            "range": "± 2124",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 3205,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 63880,
            "range": "± 5019",
            "unit": "ns/iter"
          },
          {
            "name": "hamt diff",
            "value": 38292,
            "range": "± 1923",
            "unit": "ns/iter"
          },
          {
            "name": "hamt merge",
            "value": 117369,
            "range": "± 6530",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new_hashed",
            "value": 1792985,
            "range": "± 168361",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new_hashed",
            "value": 313853,
            "range": "± 36602",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumDig>::new(rng)",
            "value": 1829476,
            "range": "± 195650",
            "unit": "ns/iter"
          },
          {
            "name": "NameSegment::<BigNumRug>::new(rng)",
            "value": 595368,
            "range": "± 20737",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig>::add",
            "value": 1771387,
            "range": "± 54863",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug>::add",
            "value": 1001703,
            "range": "± 26367",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumDig> serialization",
            "value": 485,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "NameAccumulator::<BigNumRug> serialization",
            "value": 639,
            "range": "± 30",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}