#!/usr/local/bin/WolframKernel -script
(* ZK-Matrix Join Benchmark: Complexity-Aware Empirical Plotting *)

dataNaive = {{1000, 71437296}, {5000, 450271496}, {10000, 893025827}};
dataOptimized = {{1000, 2145908}, {5000, 10657944}, {10000, 21299845}};

(* Best-fit complexity curves *)
fitNaive = NonlinearModelFit[dataNaive, a * n * Log[n], {a}, n];
fitOptimized = LinearModelFit[dataOptimized, n, n];

plot = Plot[
  {fitNaive[n], fitOptimized[n]},
  {n, 0, 11000},
  PlotRange -> {{0, 11000}, {0, 10^9}},
  Frame -> True,
  FrameLabel -> {"Trace Length (Memory Access Events)", "zkVM Trace Size (AIR Rows)"},
  PlotLabel -> Style["ZK-Matrix Join: Scaling Performance (Empirical Results)", Bold, 16],
  GridLines -> Automatic,
  ImageSize -> 800,
  PlotLegends -> Placed[
    {
      "Global Grand Product Accumulator\n(Baseline SP1 zkVM)",
      "Topological Coprocessor Algorithm\n(Optimized Constraint Model)"
    },
    Below
  ],
  Epilog -> {
    (* Empirical data points *)
    {ColorData[97, 1], PointSize[0.015], Point[dataNaive]},
    {ColorData[97, 2], PointSize[0.015], Point[dataOptimized]},
    Inset[
      Framed[
        Column[{
          Style["Speedup at 10k: 41.9x", Bold],
          Style["Baseline: O(n \!\(\*SubscriptBox[\(log\), \(2\)]\) n)"],
          Style["Optimized: O(n)"]
        }],
        Background -> White, RoundingRadius -> 5
      ],
      {3000, 7*10^8}
    ]
  }
];

(* Define export path relative to the script directory *)
exportPath = ExpandFileName[FileNameJoin[{DirectoryName[ExpandFileName[$InputFileName]], "..", "res", "benchmark_visualization.png"}]];

Export[exportPath, plot];
Print["Visualization exported to ", exportPath]
