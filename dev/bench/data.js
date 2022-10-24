window.BENCHMARK_DATA = {
  "lastUpdate": 1666623775440,
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
      }
    ]
  }
}