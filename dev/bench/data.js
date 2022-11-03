window.BENCHMARK_DATA = {
  "lastUpdate": 1667487088414,
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
      }
    ]
  }
}