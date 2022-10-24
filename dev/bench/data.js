window.BENCHMARK_DATA = {
  "lastUpdate": 1666637096195,
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
          "id": "66e74789a86f81adaa6c02e9124b4a4e1a1e05c8",
          "message": "Initial Benchmark Work",
          "timestamp": "2022-10-24T10:56:48Z",
          "url": "https://github.com/wnfs-wg/rs-wnfs/pull/75/commits/66e74789a86f81adaa6c02e9124b4a4e1a1e05c8"
        },
        "date": 1666614539978,
        "tool": "cargo",
        "benches": [
          {
            "name": "Node set",
            "value": 210840,
            "range": "± 802",
            "unit": "ns/iter"
          },
          {
            "name": "With throughput/Node load and get",
            "value": 152115,
            "range": "± 307",
            "unit": "ns/iter"
          },
          {
            "name": "With throughput/Node load and remove",
            "value": 260673,
            "range": "± 1116",
            "unit": "ns/iter"
          },
          {
            "name": "With throughput/HAMT load and decode",
            "value": 34966,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "HAMT set and encode",
            "value": 261470,
            "range": "± 1237",
            "unit": "ns/iter"
          },
          {
            "name": "Namefilter add",
            "value": 12970,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Namefilter contains",
            "value": 12315,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "Namefilter saturate",
            "value": 2469,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Namefilter encode",
            "value": 143,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Namefilter decode",
            "value": 268,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "f2019827d592e3da3d0a3b1392606f4f939a1ffa",
          "message": "Initial Benchmark Work",
          "timestamp": "2022-10-24T10:56:48Z",
          "url": "https://github.com/wnfs-wg/rs-wnfs/pull/75/commits/f2019827d592e3da3d0a3b1392606f4f939a1ffa"
        },
        "date": 1666623773968,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 257680,
            "range": "± 16816",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get/0",
            "value": 188218,
            "range": "± 32678",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove/0",
            "value": 319601,
            "range": "± 20986",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 46206,
            "range": "± 3017",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 310894,
            "range": "± 25529",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 15393,
            "range": "± 1395",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 13591,
            "range": "± 1564",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 3116,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 192,
            "range": "± 19",
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
            "name": "wnfs-wg",
            "username": "wnfs-wg"
          },
          "committer": {
            "name": "wnfs-wg",
            "username": "wnfs-wg"
          },
          "id": "1721644893fd8b23e3661553503743b845653c79",
          "message": "Initial Benchmark Work",
          "timestamp": "2022-10-24T16:32:34Z",
          "url": "https://github.com/wnfs-wg/rs-wnfs/pull/75/commits/1721644893fd8b23e3661553503743b845653c79"
        },
        "date": 1666636560109,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 208929,
            "range": "± 787",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get/0",
            "value": 156076,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove/0",
            "value": 264079,
            "range": "± 1119",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 39073,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 252909,
            "range": "± 653",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 13847,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11533,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 2453,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 133,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter decode/0",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "d4be4d77d0b854ead42a11e73935e7528f983963",
          "message": "Initial Benchmark Work",
          "timestamp": "2022-10-24T18:41:29Z",
          "url": "https://github.com/wnfs-wg/rs-wnfs/pull/75/commits/d4be4d77d0b854ead42a11e73935e7528f983963"
        },
        "date": 1666637095606,
        "tool": "cargo",
        "benches": [
          {
            "name": "node set",
            "value": 211252,
            "range": "± 3980",
            "unit": "ns/iter"
          },
          {
            "name": "node load and get/0",
            "value": 156774,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "node load and remove/0",
            "value": 270803,
            "range": "± 1650",
            "unit": "ns/iter"
          },
          {
            "name": "hamt load and decode/0",
            "value": 38664,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "hamt set and encode",
            "value": 254727,
            "range": "± 643",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter add",
            "value": 15581,
            "range": "± 816",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter contains",
            "value": 11549,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter saturate",
            "value": 2468,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "namefilter encode",
            "value": 133,
            "range": "± 0",
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