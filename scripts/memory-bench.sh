#!/bin/bash
# Memory profiling and benchmarking script for VoIP server
# Usage: ./scripts/memory-bench.sh [component]

set -e

COMPONENT=${1:-all}
REPORT_DIR="reports/memory/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$REPORT_DIR"

echo "🔍 VoIP Server Memory Benchmarking Tool"
echo "======================================="
echo "Component: $COMPONENT"
echo "Report dir: $REPORT_DIR"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run miri tests
run_miri() {
    echo -e "${YELLOW}[MIRI] Running memory safety checks...${NC}"
    if cargo +nightly miri test --workspace 2>&1 | tee "$REPORT_DIR/miri.log"; then
        echo -e "${GREEN}✓ Miri: No undefined behavior detected${NC}"
    else
        echo -e "${RED}✗ Miri: Issues found (see $REPORT_DIR/miri.log)${NC}"
    fi
}

# Function to run valgrind memcheck
run_valgrind() {
    echo -e "${YELLOW}[VALGRIND] Running memory leak detection...${NC}"
    cargo build --release --workspace

    for binary in target/release/deps/*-*; do
        if [[ -x "$binary" && ! "$binary" == *.d ]]; then
            binary_name=$(basename "$binary")
            echo "  Checking $binary_name..."
            valgrind \
                --leak-check=full \
                --show-leak-kinds=all \
                --track-origins=yes \
                --log-file="$REPORT_DIR/valgrind_$binary_name.log" \
                "$binary" 2>/dev/null || true
        fi
    done
    echo -e "${GREEN}✓ Valgrind reports saved to $REPORT_DIR/valgrind_*.log${NC}"
}

# Function to run heaptrack
run_heaptrack() {
    echo -e "${YELLOW}[HEAPTRACK] Profiling heap allocations...${NC}"
    cargo build --release --bin main

    if [[ -f "target/release/main" ]]; then
        heaptrack target/release/main &
        PID=$!
        sleep 5
        kill $PID 2>/dev/null || true

        # Move heaptrack output
        mv heaptrack.main.*.gz "$REPORT_DIR/" 2>/dev/null || true
        echo -e "${GREEN}✓ Heaptrack data saved to $REPORT_DIR/heaptrack.*.gz${NC}"
        echo "  View with: heaptrack_gui $REPORT_DIR/heaptrack.*.gz"
    else
        echo -e "${YELLOW}  No main binary found, skipping heaptrack${NC}"
    fi
}

# Function to measure RSS memory
measure_rss() {
    echo -e "${YELLOW}[RSS] Measuring runtime memory usage...${NC}"
    cargo build --release --workspace

    for binary in target/release/deps/*-*; do
        if [[ -x "$binary" && ! "$binary" == *.d && "$binary" == *test* ]]; then
            binary_name=$(basename "$binary")
            echo "  Measuring $binary_name..."

            # Run with timeout and capture RSS
            timeout 2s /usr/bin/time -v "$binary" 2>&1 | \
                grep -E "Maximum resident set size" | \
                tee -a "$REPORT_DIR/rss_measurements.txt" || true
        fi
    done
    echo -e "${GREEN}✓ RSS measurements saved to $REPORT_DIR/rss_measurements.txt${NC}"
}

# Function to analyze binary size
analyze_binary_size() {
    echo -e "${YELLOW}[BLOAT] Analyzing binary size...${NC}"
    cargo bloat --release --crates -n 20 2>&1 | tee "$REPORT_DIR/bloat_crates.txt"
    cargo bloat --release -n 20 2>&1 | tee "$REPORT_DIR/bloat_functions.txt"
    echo -e "${GREEN}✓ Binary analysis saved to $REPORT_DIR/bloat_*.txt${NC}"
}

# Function to generate memory report
generate_report() {
    echo -e "${YELLOW}[REPORT] Generating summary...${NC}"

    cat > "$REPORT_DIR/summary.md" <<EOF
# Memory Benchmark Report
Generated: $(date)

## Component: $COMPONENT

### Test Results
- **Miri**: See \`miri.log\` for undefined behavior check
- **Valgrind**: See \`valgrind_*.log\` for leak detection
- **Heaptrack**: See \`heaptrack.*.gz\` for allocation profiling
- **RSS**: See \`rss_measurements.txt\` for runtime memory usage
- **Binary Size**: See \`bloat_*.txt\` for size analysis

### Quick Stats
\`\`\`
$(grep "Maximum resident" "$REPORT_DIR/rss_measurements.txt" 2>/dev/null || echo "No RSS data")
\`\`\`

### Recommendations
1. Review any Miri warnings for undefined behavior
2. Check Valgrind logs for memory leaks
3. Use heaptrack_gui to visualize allocation patterns
4. Consider optimizing functions listed in bloat analysis

EOF

    echo -e "${GREEN}✓ Summary report: $REPORT_DIR/summary.md${NC}"
}

# Main execution
echo "Starting memory benchmark suite..."
echo ""

case $COMPONENT in
    miri)
        run_miri
        ;;
    valgrind)
        run_valgrind
        ;;
    heaptrack)
        run_heaptrack
        ;;
    rss)
        measure_rss
        ;;
    bloat)
        analyze_binary_size
        ;;
    all)
        run_miri
        echo ""
        run_valgrind
        echo ""
        # run_heaptrack
        echo ""
        measure_rss
        echo ""
        # analyze_binary_size
        echo ""
        generate_report
        ;;
    *)
        echo -e "${RED}Unknown component: $COMPONENT${NC}"
        echo "Usage: $0 [miri|valgrind|heaptrack|rss|bloat|all]"
        exit 1
        ;;
esac

echo ""
echo "✨ Memory benchmark complete!"
echo "📁 Reports saved to: $REPORT_DIR"