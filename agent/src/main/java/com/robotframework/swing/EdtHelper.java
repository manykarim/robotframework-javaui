package com.robotframework.swing;

import javax.swing.SwingUtilities;
import java.awt.EventQueue;
import java.lang.reflect.InvocationTargetException;
import java.util.concurrent.Callable;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Helper utilities for Event Dispatch Thread (EDT) operations.
 * Ensures all Swing operations are performed on the EDT for thread safety.
 */
public class EdtHelper {

    /**
     * Functional interface for actions that don't return a value.
     */
    @FunctionalInterface
    public interface EdtAction {
        void run() throws Exception;
    }

    /**
     * Run an action on the EDT and wait for completion.
     * If already on EDT, runs immediately.
     *
     * @param action Action to run
     * @throws RuntimeException if the action throws an exception
     */
    public static void runOnEdt(EdtAction action) {
        if (SwingUtilities.isEventDispatchThread()) {
            try {
                action.run();
            } catch (Exception e) {
                throw new RuntimeException("EDT action failed", e);
            }
        } else {
            try {
                SwingUtilities.invokeAndWait(() -> {
                    try {
                        action.run();
                    } catch (Exception e) {
                        throw new RuntimeException(e);
                    }
                });
            } catch (InvocationTargetException e) {
                Throwable cause = e.getCause();
                if (cause instanceof RuntimeException) {
                    throw (RuntimeException) cause;
                }
                throw new RuntimeException("EDT action failed", cause);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                throw new RuntimeException("EDT action interrupted", e);
            }
        }
    }

    /**
     * Run a callable on the EDT and return the result.
     * If already on EDT, runs immediately.
     *
     * @param callable Callable to run
     * @param <T> Return type
     * @return The result from the callable
     * @throws RuntimeException if the callable throws an exception
     */
    public static <T> T runOnEdtAndReturn(Callable<T> callable) {
        if (SwingUtilities.isEventDispatchThread()) {
            try {
                return callable.call();
            } catch (Exception e) {
                throw new RuntimeException("EDT callable failed", e);
            }
        } else {
            AtomicReference<T> result = new AtomicReference<>();
            AtomicReference<Exception> exception = new AtomicReference<>();

            try {
                SwingUtilities.invokeAndWait(() -> {
                    try {
                        result.set(callable.call());
                    } catch (Exception e) {
                        exception.set(e);
                    }
                });
            } catch (InvocationTargetException | InterruptedException e) {
                throw new RuntimeException("EDT callable invocation failed", e);
            }

            if (exception.get() != null) {
                throw new RuntimeException("EDT callable failed", exception.get());
            }

            return result.get();
        }
    }

    /**
     * Run an action on the EDT asynchronously (fire and forget).
     *
     * @param action Action to run
     */
    public static void runOnEdtLater(EdtAction action) {
        if (SwingUtilities.isEventDispatchThread()) {
            try {
                action.run();
            } catch (Exception e) {
                System.err.println("EDT action failed: " + e.getMessage());
                e.printStackTrace();
            }
        } else {
            SwingUtilities.invokeLater(() -> {
                try {
                    action.run();
                } catch (Exception e) {
                    System.err.println("EDT action failed: " + e.getMessage());
                    e.printStackTrace();
                }
            });
        }
    }

    /**
     * Wait for the EDT to become idle by posting a no-op event and waiting for it.
     * This ensures all pending events have been processed.
     *
     * @param timeout Maximum time to wait in milliseconds
     * @return true if EDT became idle within timeout, false otherwise
     */
    public static boolean waitForEdt(long timeout) {
        if (SwingUtilities.isEventDispatchThread()) {
            return true;
        }

        final Object lock = new Object();
        final boolean[] completed = {false};

        EventQueue.invokeLater(() -> {
            synchronized (lock) {
                completed[0] = true;
                lock.notifyAll();
            }
        });

        synchronized (lock) {
            long startTime = System.currentTimeMillis();
            while (!completed[0]) {
                long elapsed = System.currentTimeMillis() - startTime;
                if (elapsed >= timeout) {
                    return false;
                }
                try {
                    lock.wait(timeout - elapsed);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    return false;
                }
            }
        }

        return true;
    }

    /**
     * Wait for the EDT to become idle with default timeout of 5 seconds.
     *
     * @return true if EDT became idle within timeout
     */
    public static boolean waitForEdt() {
        return waitForEdt(5000);
    }

    /**
     * Check if current thread is the EDT.
     *
     * @return true if on EDT
     */
    public static boolean isOnEdt() {
        return SwingUtilities.isEventDispatchThread();
    }

    /**
     * Assert that we are on the EDT.
     *
     * @throws IllegalStateException if not on EDT
     */
    public static void assertOnEdt() {
        if (!SwingUtilities.isEventDispatchThread()) {
            throw new IllegalStateException("This method must be called on the Event Dispatch Thread");
        }
    }

    /**
     * Assert that we are NOT on the EDT.
     *
     * @throws IllegalStateException if on EDT
     */
    public static void assertNotOnEdt() {
        if (SwingUtilities.isEventDispatchThread()) {
            throw new IllegalStateException("This method must NOT be called on the Event Dispatch Thread");
        }
    }

    /**
     * Sleep on the current thread without checked exception.
     *
     * @param millis Milliseconds to sleep
     */
    public static void sleep(long millis) {
        try {
            Thread.sleep(millis);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    /**
     * Wait for a condition to become true, checking periodically on the EDT.
     *
     * @param condition Condition to check
     * @param timeout Maximum time to wait in milliseconds
     * @param pollInterval Time between checks in milliseconds
     * @return true if condition became true within timeout
     */
    public static boolean waitForCondition(Callable<Boolean> condition, long timeout, long pollInterval) {
        long startTime = System.currentTimeMillis();

        while (System.currentTimeMillis() - startTime < timeout) {
            try {
                Boolean result = runOnEdtAndReturn(condition);
                if (result != null && result) {
                    return true;
                }
            } catch (Exception e) {
                // Condition check failed, continue waiting
            }
            sleep(pollInterval);
        }

        return false;
    }

    /**
     * Wait for a condition with default poll interval of 100ms.
     *
     * @param condition Condition to check
     * @param timeout Maximum time to wait in milliseconds
     * @return true if condition became true within timeout
     */
    public static boolean waitForCondition(Callable<Boolean> condition, long timeout) {
        return waitForCondition(condition, timeout, 100);
    }
}
