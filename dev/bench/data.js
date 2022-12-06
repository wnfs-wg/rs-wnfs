window.BENCHMARK_DATA = {
  "lastUpdate": 1670319243934,
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
      }
    ]
  }
}