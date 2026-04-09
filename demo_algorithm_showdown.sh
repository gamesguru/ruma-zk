#!/bin/bash
# ZK-Matrix-Join: Algorithm Showdown Demo
# Compares Optimized (Topological Reducer) vs. Unoptimized (Full Spec)

echo "=========================================================="
echo "   ZK-Matrix-Join: ALGORITHM SHOWDOWN (1,000 Events)      "
echo "=========================================================="
echo "This demo runs the same 1,000 Matrix events through two"
echo "different ZK pipelines to compare efficiency and correctness."
echo ""

FIXTURE="res/real_1k.json"
export MATRIX_FIXTURE_PATH=$FIXTURE

# 1. Run Optimized Pipeline
echo "[1/2] Executing OPTIMIZED Pipeline (Topological Reducer)..."
OPT_OUT=$(MATRIX_FIXTURE_PATH=$FIXTURE cargo run --quiet --bin zk-matrix-join-host 2>/dev/null)
OPT_CYCLES=$(echo "$OPT_OUT" | grep "RISC-V CPU Cycles Used:" | awk '{print $NF}')
OPT_HASH=$(echo "$OPT_OUT" | grep "Matrix Resolved State Hash" | awk '{print $NF}')

# 2. Run Unoptimized Pipeline
echo "[2/2] Executing UNOPTIMIZED Pipeline (Full Spec State Res)..."
UNOPT_OUT=$(EXECUTE_UNOPTIMIZED=1 MATRIX_FIXTURE_PATH=$FIXTURE cargo run --quiet --bin zk-matrix-join-host 2>/dev/null)
UNOPT_CYCLES=$(echo "$UNOPT_OUT" | grep "RISC-V CPU Cycles Used:" | awk '{print $NF}')
UNOPT_HASH=$(echo "$UNOPT_OUT" | grep "Matrix Resolved State Hash" | awk '{print $NF}')

echo ""
echo "=========================================================="
echo "                  BENCHMARK COMPARISON                    "
echo "=========================================================="
printf "% -25s | % -15s | % -15s\n" "Metric" "Optimized" "Unoptimized"
echo "----------------------------------------------------------"
printf "% -25s | % -15s | % -15s\n" "ZK VM Cycles" "$OPT_CYCLES" "$UNOPT_CYCLES"
printf "% -25s | % -15s | % -15s\n" "Algorithm Type" "L2-Sequential" "Full Spec (v2)"
printf "% -25s | % -15s | % -15s\n" "Trust Model" "Math Proven" "Math Proven"
echo "----------------------------------------------------------"
echo "Final State Hash (Matches?)"
echo "Optimized:   $OPT_HASH"
echo "Unoptimized: $UNOPT_HASH"

if [ "$OPT_HASH" == "$UNOPT_HASH" ]; then
    echo ""
    echo "✓ VERIFIED: Both algorithms reached the EXACT same state!"
    echo "Summary: The Optimized pipeline is $(echo "$UNOPT_CYCLES / $OPT_CYCLES" | bc -l | xargs printf "%.1f")x faster."
else
    echo ""
    echo "× ERROR: State Hash Mismatch!"
fi
echo "=========================================================="
