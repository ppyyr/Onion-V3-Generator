#!/bin/bash

# Onion Generator Benchmark Script

set -e

echo "🚀 Onion Generator Benchmark"
echo "=============================="

# Build the project
echo "📦 Building release version..."
cargo build --release

# Test different configurations
echo ""
echo "🧪 Running benchmarks..."

# Single-threaded test
echo ""
echo "1️⃣ Single-threaded performance (10 seconds):"
timeout 10s ./target/release/onion-generator --single-threaded a 2>/dev/null | grep "Generated" | tail -1 || echo "No results in 10 seconds"

# Multi-threaded with 2 workers
echo ""
echo "2️⃣ Multi-threaded with 2 workers (10 seconds):"
timeout 10s ./target/release/onion-generator --workers 2 a 2>/dev/null | grep "Generated" | tail -1 || echo "No results in 10 seconds"

# Multi-threaded with 4 workers
echo ""
echo "3️⃣ Multi-threaded with 4 workers (10 seconds):"
timeout 10s ./target/release/onion-generator --workers 4 a 2>/dev/null | grep "Generated" | tail -1 || echo "No results in 10 seconds"

# Multi-threaded with 8 workers
echo ""
echo "4️⃣ Multi-threaded with 8 workers (10 seconds):"
timeout 10s ./target/release/onion-generator --workers 8 a 2>/dev/null | grep "Generated" | tail -1 || echo "No results in 10 seconds"

# Auto-detect workers (default)
echo ""
echo "5️⃣ Auto-detect workers (10 seconds):"
timeout 10s ./target/release/onion-generator a 2>/dev/null | grep "Generated" | tail -1 || echo "No results in 10 seconds"

echo ""
echo "✅ Benchmark complete!"
echo ""
echo "💡 Tips:"
echo "   - Use more workers for better performance on multi-core systems"
echo "   - Longer prefixes take exponentially more time to find"
echo "   - Single character prefixes are found very quickly"
echo "   - Use Docker for consistent cross-platform performance"
