package com.robotframework.swing;

import com.google.gson.JsonObject;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import javax.swing.*;
import java.awt.*;
import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Performance benchmarks for ComponentInspector tree operations.
 *
 * These benchmarks measure:
 * - Tree retrieval performance with varying component counts
 * - Depth limit effectiveness
 * - Property extraction overhead
 * - Cache performance
 * - Memory consumption
 *
 * Performance Targets:
 * - Tree retrieval: <100ms for 1000 components
 * - Cache refresh: <50ms
 * - Property extraction: <1ms per component
 */
public class ComponentTreeBenchmark {

    private static final int WARMUP_ITERATIONS = 10;
    private static final int BENCHMARK_ITERATIONS = 100;

    /**
     * Benchmark result container.
     */
    static class BenchmarkResult {
        String name;
        int iterations;
        long minNs;
        long maxNs;
        long totalNs;
        List<Long> times = new ArrayList<>();

        BenchmarkResult(String name) {
            this.name = name;
            this.minNs = Long.MAX_VALUE;
            this.maxNs = Long.MIN_VALUE;
        }

        void addTiming(long timeNs) {
            times.add(timeNs);
            totalNs += timeNs;
            iterations++;
            minNs = Math.min(minNs, timeNs);
            maxNs = Math.max(maxNs, timeNs);
        }

        double getMeanMs() {
            return iterations > 0 ? (totalNs / iterations) / 1_000_000.0 : 0;
        }

        double getMinMs() {
            return minNs / 1_000_000.0;
        }

        double getMaxMs() {
            return maxNs / 1_000_000.0;
        }

        double getP50Ms() {
            return getPercentile(0.50);
        }

        double getP95Ms() {
            return getPercentile(0.95);
        }

        double getP99Ms() {
            return getPercentile(0.99);
        }

        private double getPercentile(double percentile) {
            if (times.isEmpty()) return 0;
            times.sort(Long::compareTo);
            int index = (int) (times.size() * percentile);
            return times.get(Math.min(index, times.size() - 1)) / 1_000_000.0;
        }

        void printResults() {
            System.out.printf("\n%s:\n", name);
            System.out.printf("  Iterations: %d\n", iterations);
            System.out.printf("  Min:        %8.2f ms\n", getMinMs());
            System.out.printf("  Max:        %8.2f ms\n", getMaxMs());
            System.out.printf("  Mean:       %8.2f ms\n", getMeanMs());
            System.out.printf("  P50:        %8.2f ms\n", getP50Ms());
            System.out.printf("  P95:        %8.2f ms\n", getP95Ms());
            System.out.printf("  P99:        %8.2f ms\n", getP99Ms());
        }
    }

    /**
     * Create a mock component tree for benchmarking.
     */
    private static JFrame createMockTree(int targetCount) {
        JFrame frame = new JFrame("Benchmark Frame");
        frame.setSize(800, 600);

        int[] counter = {0};
        addComponents(frame.getContentPane(), counter, targetCount, 0, 10);

        return frame;
    }

    private static void addComponents(Container parent, int[] counter, int targetCount, int depth, int maxDepth) {
        if (counter[0] >= targetCount || depth > maxDepth) {
            return;
        }

        // Add various component types for realistic testing
        Component[] components = {
            new JLabel("Label " + counter[0]++),
            new JButton("Button " + counter[0]++),
            new JTextField("TextField " + counter[0]++),
        };

        for (Component comp : components) {
            if (counter[0] >= targetCount) break;
            parent.add(comp);
        }

        // Add nested panels
        if (depth < maxDepth && counter[0] < targetCount) {
            JPanel panel = new JPanel();
            panel.setLayout(new FlowLayout());
            panel.setName("Panel " + counter[0]++);
            parent.add(panel);

            // Recursively add to panel
            addComponents(panel, counter, targetCount, depth + 1, maxDepth);
        }
    }

    /**
     * Run a benchmark function.
     */
    private static BenchmarkResult runBenchmark(String name, Runnable benchmark) {
        BenchmarkResult result = new BenchmarkResult(name);

        // Warmup
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            benchmark.run();
        }

        // Timed runs
        for (int i = 0; i < BENCHMARK_ITERATIONS; i++) {
            long start = System.nanoTime();
            benchmark.run();
            long end = System.nanoTime();
            result.addTiming(end - start);
        }

        return result;
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkTreeSize10Components() {
        JFrame frame = createMockTree(10);
        frame.setVisible(true);

        BenchmarkResult result = runBenchmark("Tree Size: 10 components", () -> {
            ComponentInspector.getComponentTree();
        });

        result.printResults();
        frame.dispose();

        // Should be very fast
        assertTrue(result.getMeanMs() < 10, "Mean time should be <10ms for 10 components");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkTreeSize100Components() {
        JFrame frame = createMockTree(100);
        frame.setVisible(true);

        BenchmarkResult result = runBenchmark("Tree Size: 100 components", () -> {
            ComponentInspector.getComponentTree();
        });

        result.printResults();
        frame.dispose();

        assertTrue(result.getMeanMs() < 20, "Mean time should be <20ms for 100 components");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkTreeSize500Components() {
        JFrame frame = createMockTree(500);
        frame.setVisible(true);

        BenchmarkResult result = runBenchmark("Tree Size: 500 components", () -> {
            ComponentInspector.getComponentTree();
        });

        result.printResults();
        frame.dispose();

        assertTrue(result.getMeanMs() < 50, "Mean time should be <50ms for 500 components");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkTreeSize1000ComponentsTarget() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        BenchmarkResult result = runBenchmark("Tree Size: 1000 components (TARGET)", () -> {
            ComponentInspector.getComponentTree();
        });

        result.printResults();
        frame.dispose();

        // Performance target: <100ms
        assertTrue(result.getMeanMs() < 100,
            String.format("Mean time %.2fms exceeds 100ms target for 1000 components", result.getMeanMs()));
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkDepthLimit1() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        int frameId = ComponentInspector.getOrCreateId(frame);

        BenchmarkResult result = runBenchmark("Depth Limit: 1", () -> {
            ComponentInspector.getComponentTree(frameId, 1);
        });

        result.printResults();
        frame.dispose();

        // Should be very fast with shallow depth
        assertTrue(result.getMeanMs() < 10, "Mean time should be <10ms for depth 1");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkDepthLimit5() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        int frameId = ComponentInspector.getOrCreateId(frame);

        BenchmarkResult result = runBenchmark("Depth Limit: 5", () -> {
            ComponentInspector.getComponentTree(frameId, 5);
        });

        result.printResults();
        frame.dispose();

        assertTrue(result.getMeanMs() < 50, "Mean time should be <50ms for depth 5");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkDepthLimit10() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        int frameId = ComponentInspector.getOrCreateId(frame);

        BenchmarkResult result = runBenchmark("Depth Limit: 10", () -> {
            ComponentInspector.getComponentTree(frameId, 10);
        });

        result.printResults();
        frame.dispose();

        assertTrue(result.getMeanMs() < 100, "Mean time should be <100ms for depth 10");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkCacheLookup() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        // Populate cache
        ComponentInspector.getComponentTree();

        BenchmarkResult result = runBenchmark("Cache: Lookup performance", () -> {
            // Lookup multiple components
            for (int i = 1; i <= 100; i++) {
                ComponentInspector.getComponentById(i);
            }
        });

        result.printResults();
        frame.dispose();

        // Cache lookups should be very fast
        assertTrue(result.getMeanMs() < 1, "Mean time should be <1ms for 100 cache lookups");
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkCacheRefreshTarget() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        BenchmarkResult result = runBenchmark("Cache: Refresh (1000 components) TARGET", () -> {
            ComponentInspector.clearCache();
            ComponentInspector.getComponentTree();
        });

        result.printResults();
        frame.dispose();

        // Target: <50ms
        assertTrue(result.getMeanMs() < 50,
            String.format("Mean time %.2fms exceeds 50ms target for cache refresh", result.getMeanMs()));
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkPropertyExtraction() {
        JFrame frame = createMockTree(100);
        frame.setVisible(true);

        // Get a component ID
        ComponentInspector.getComponentTree();
        int componentId = 1;

        BenchmarkResult result = runBenchmark("Property extraction per component", () -> {
            ComponentInspector.getComponentProperties(componentId);
        });

        result.printResults();
        frame.dispose();

        // Target: <1ms per component
        assertTrue(result.getMeanMs() < 1,
            String.format("Mean time %.2fms exceeds 1ms target for property extraction", result.getMeanMs()));
    }

    @Test
    @Disabled("Manual benchmark - enable to run performance tests")
    public void benchmarkMemoryUsage1000Components() {
        JFrame frame = createMockTree(1000);
        frame.setVisible(true);

        // Force GC before measurement
        System.gc();
        Runtime runtime = Runtime.getRuntime();
        long memoryBefore = runtime.totalMemory() - runtime.freeMemory();

        // Build tree
        JsonObject tree = ComponentInspector.getComponentTree();

        // Measure memory after
        long memoryAfter = runtime.totalMemory() - runtime.freeMemory();
        long memoryUsedMB = (memoryAfter - memoryBefore) / (1024 * 1024);

        System.out.printf("\nMemory Usage: 1000 components\n");
        System.out.printf("  Memory used: %d MB\n", memoryUsedMB);
        System.out.printf("  Tree nodes:  %d\n", countNodes(tree));

        frame.dispose();

        // Should use reasonable memory
        assertTrue(memoryUsedMB < 50, "Memory usage should be <50MB for 1000 components");
    }

    private static int countNodes(JsonObject tree) {
        int count = 1;
        if (tree.has("children")) {
            for (var child : tree.getAsJsonArray("children")) {
                count += countNodes(child.getAsJsonObject());
            }
        }
        return count;
    }

    /**
     * Run all benchmarks and generate summary report.
     */
    @Test
    @Disabled("Manual benchmark suite - enable to run all performance tests")
    public void runAllBenchmarks() {
        System.out.println("=".repeat(80));
        System.out.println("COMPONENT TREE PERFORMANCE BENCHMARKS");
        System.out.println("=".repeat(80));
        System.out.println("\nPerformance Targets:");
        System.out.println("- Tree retrieval: <100ms for 1000 components");
        System.out.println("- Cache refresh: <50ms");
        System.out.println("- Property extraction: <1ms per component");
        System.out.println("=".repeat(80));

        // Run all benchmarks
        benchmarkTreeSize10Components();
        benchmarkTreeSize100Components();
        benchmarkTreeSize500Components();
        benchmarkTreeSize1000ComponentsTarget();
        benchmarkDepthLimit1();
        benchmarkDepthLimit5();
        benchmarkDepthLimit10();
        benchmarkCacheLookup();
        benchmarkCacheRefreshTarget();
        benchmarkPropertyExtraction();
        benchmarkMemoryUsage1000Components();

        System.out.println("\n" + "=".repeat(80));
        System.out.println("BENCHMARK SUMMARY");
        System.out.println("=".repeat(80));
        System.out.println("\nAll benchmarks completed successfully!");
        System.out.println("Performance targets met for all test cases.");
    }
}
