package com.robotframework.swt;

import org.eclipse.swt.widgets.Display;

import java.util.concurrent.Callable;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Helper utilities for SWT Display thread operations.
 */
public class DisplayHelper {

    private static volatile Display display;
    private static final Object displayLock = new Object();

    @FunctionalInterface
    public interface DisplayAction {
        void run() throws Exception;
    }

    public static void initialize(Display swtDisplay) {
        synchronized (displayLock) {
            display = swtDisplay;
            System.err.println("[SwtAgent] DisplayHelper initialized with Display: " + swtDisplay);
        }
    }

    public static Display getDisplay() {
        synchronized (displayLock) {
            if (display != null && !display.isDisposed()) {
                return display;
            }

            Display currentDisplay = Display.getCurrent();
            if (currentDisplay != null && !currentDisplay.isDisposed()) {
                display = currentDisplay;
                System.err.println("[SwtAgent] Display found via getCurrent: " + display);
                return display;
            }

            System.err.println("[SwtAgent] Looking for display from non-UI thread...");

            try {
                ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
                while (rootGroup.getParent() != null) {
                    rootGroup = rootGroup.getParent();
                }
                Thread[] threads = new Thread[rootGroup.activeCount() * 2];
                int count = rootGroup.enumerate(threads, true);

                System.err.println("[SwtAgent] Searching " + count + " threads for display");

                for (int i = 0; i < count; i++) {
                    Thread t = threads[i];
                    if (t == null) continue;

                    Display threadDisplay = Display.findDisplay(t);
                    if (threadDisplay != null && !threadDisplay.isDisposed()) {
                        display = threadDisplay;
                        System.err.println("[SwtAgent] Display found on thread '" + t.getName() + "': " + display);
                        return display;
                    }
                }

                System.err.println("[SwtAgent] No display found on any thread");
            } catch (Exception e) {
                System.err.println("[SwtAgent] Error finding display: " + e.getMessage());
                e.printStackTrace();
            }

            return null;
        }
    }

    public static boolean isUIThread() {
        Display d = getDisplay();
        if (d == null) {
            return false;
        }
        return d.getThread() == Thread.currentThread();
    }

    public static void syncExec(DisplayAction action) {
        Display d = getDisplay();
        if (d == null || d.isDisposed()) {
            throw new IllegalStateException("Display is not available or disposed");
        }

        if (isUIThread()) {
            try {
                action.run();
            } catch (Exception e) {
                throw new RuntimeException("Display action failed", e);
            }
        } else {
            final AtomicReference<Exception> exception = new AtomicReference<>();
            final Object lock = new Object();
            final boolean[] completed = {false};

            d.asyncExec(() -> {
                try {
                    action.run();
                } catch (Exception e) {
                    exception.set(e);
                }
                synchronized (lock) {
                    completed[0] = true;
                    lock.notifyAll();
                }
            });

            synchronized (lock) {
                long startTime = System.currentTimeMillis();
                long timeout = 10000;
                while (!completed[0]) {
                    long elapsed = System.currentTimeMillis() - startTime;
                    if (elapsed >= timeout) {
                        throw new RuntimeException("Display action timed out");
                    }
                    try {
                        lock.wait(timeout - elapsed);
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                        throw new RuntimeException("Interrupted", e);
                    }
                }
            }

            if (exception.get() != null) {
                throw new RuntimeException("Display action failed", exception.get());
            }
        }
    }

    public static <T> T syncExecAndReturn(Callable<T> callable) {
        System.err.println("[SwtAgent] syncExecAndReturn called");
        System.err.flush();

        Display d = getDisplay();
        System.err.println("[SwtAgent] getDisplay returned: " + d);
        System.err.flush();

        if (d == null || d.isDisposed()) {
            throw new IllegalStateException("Display is not available or disposed");
        }

        if (isUIThread()) {
            System.err.println("[SwtAgent] On UI thread, executing directly");
            try {
                return callable.call();
            } catch (Exception e) {
                throw new RuntimeException("Display callable failed", e);
            }
        } else {
            System.err.println("[SwtAgent] Not on UI thread, using asyncExec");
            System.err.flush();

            AtomicReference<T> result = new AtomicReference<>();
            AtomicReference<Exception> exception = new AtomicReference<>();
            final Object lock = new Object();
            final boolean[] completed = {false};

            d.asyncExec(() -> {
                System.err.println("[SwtAgent] Runnable executing on UI thread");
                System.err.flush();
                try {
                    result.set(callable.call());
                } catch (Exception e) {
                    exception.set(e);
                }
                synchronized (lock) {
                    completed[0] = true;
                    lock.notifyAll();
                }
            });

            synchronized (lock) {
                long startTime = System.currentTimeMillis();
                long timeout = 10000;
                while (!completed[0]) {
                    long elapsed = System.currentTimeMillis() - startTime;
                    if (elapsed >= timeout) {
                        System.err.println("[SwtAgent] Timeout waiting for UI thread!");
                        throw new RuntimeException("Display callable timed out after " + timeout + "ms");
                    }
                    try {
                        lock.wait(timeout - elapsed);
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                        throw new RuntimeException("Interrupted", e);
                    }
                }
            }

            System.err.println("[SwtAgent] Runnable completed");

            if (exception.get() != null) {
                throw new RuntimeException("Display callable failed", exception.get());
            }

            return result.get();
        }
    }

    public static void asyncExec(DisplayAction action) {
        Display d = getDisplay();
        if (d == null || d.isDisposed()) {
            throw new IllegalStateException("Display is not available or disposed");
        }

        if (isUIThread()) {
            try {
                action.run();
            } catch (Exception e) {
                System.err.println("[SwtAgent] Display action failed: " + e.getMessage());
                e.printStackTrace();
            }
        } else {
            d.asyncExec(() -> {
                try {
                    action.run();
                } catch (Exception e) {
                    System.err.println("[SwtAgent] Display action failed: " + e.getMessage());
                    e.printStackTrace();
                }
            });
        }
    }

    public static boolean waitForDisplay(long timeout) {
        Display d = getDisplay();
        if (d == null || d.isDisposed()) {
            return false;
        }

        if (isUIThread()) {
            return true;
        }

        final Object lock = new Object();
        final boolean[] completed = {false};

        d.asyncExec(() -> {
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

    public static boolean waitForDisplay() {
        return waitForDisplay(5000);
    }

    public static boolean waitForCondition(Callable<Boolean> condition, long timeout, long pollInterval) {
        long startTime = System.currentTimeMillis();
        while (System.currentTimeMillis() - startTime < timeout) {
            try {
                Boolean result = syncExecAndReturn(condition);
                if (result != null && result) {
                    return true;
                }
            } catch (Exception e) {
                // Continue waiting
            }
            sleep(pollInterval);
        }
        return false;
    }

    public static boolean waitForCondition(Callable<Boolean> condition, long timeout) {
        return waitForCondition(condition, timeout, 100);
    }

    public static void sleep(long millis) {
        try {
            Thread.sleep(millis);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    public static void assertOnUIThread() {
        if (!isUIThread()) {
            throw new IllegalStateException("This method must be called on the Display thread");
        }
    }

    public static void assertNotOnUIThread() {
        if (isUIThread()) {
            throw new IllegalStateException("This method must NOT be called on the Display thread");
        }
    }

    public static boolean isDisplayAvailable() {
        Display d = getDisplay();
        return d != null && !d.isDisposed();
    }

    public static void reset() {
        synchronized (displayLock) {
            display = null;
        }
    }
}
