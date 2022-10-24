window.BENCHMARK_DATA = {
  "lastUpdate": 1666614540564,
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
      }
    ]
  }
}