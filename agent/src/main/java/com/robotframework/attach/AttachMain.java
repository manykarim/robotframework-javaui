package com.robotframework.attach;

import com.sun.tools.attach.VirtualMachine;

/**
 * Standalone entry point that loads this same agent JAR into an already-running JVM via the
 * JDK Attach API ({@code agentmain}). Lets the library attach the agent to a target that was
 * started WITHOUT {@code -javaagent} (found-in-the-wild apps, Java Web Start, etc.).
 *
 * <p>Because it lives in the shipped agent jar, no separate attacher artifact is needed:
 * <pre>
 *   java --add-modules jdk.attach -cp javagui-agent.jar \
 *        com.robotframework.attach.AttachMain &lt;pid&gt; &lt;agent-jar&gt; "port=5678,toolkit=auto"
 * </pre>
 *
 * <p>Requires a JDK (the {@code jdk.attach} module) on the host running THIS process — the
 * TARGET only needs to be an attachable HotSpot JVM owned by the same user. For JRE-only hosts
 * the library falls back to the {@code jattach} binary instead of this class.
 *
 * <p>Exit codes: 0 = agent loaded; 2 = usage; 3 = attach/target error; 4 = agent failed to
 * initialize inside the target (e.g. a restrictive SecurityManager denied it — common for
 * sandboxed IcedTea-Web JNLP apps).
 */
public final class AttachMain {

    public static void main(String[] args) {
        if (args.length < 2) {
            System.err.println("usage: AttachMain <pid> <agent-jar-path> [agent-args]");
            System.exit(2);
            return;
        }
        String pid = args[0];
        String agentJar = args[1];
        String agentArgs = args.length > 2 ? args[2] : "";

        VirtualMachine vm;
        try {
            vm = VirtualMachine.attach(pid);
        } catch (Throwable t) {
            System.err.println("ATTACH_FAILED: cannot attach to pid " + pid + ": " + t);
            System.exit(3);
            return;
        }
        try {
            vm.loadAgent(agentJar, agentArgs);
            System.out.println("AGENT_LOADED pid=" + pid + " args=" + agentArgs);
        } catch (com.sun.tools.attach.AgentInitializationException e) {
            // The jar loaded but agentmain failed inside the target. The usual cause is a
            // SecurityManager (e.g. IcedTea-Web JNLPSecurityManager) denying the agent's init.
            System.err.println("AGENT_INIT_FAILED: agent loaded but failed to initialize in the "
                    + "target JVM (return=" + e.returnValue() + "). A restrictive SecurityManager "
                    + "in the target (e.g. a sandboxed JNLP app) is the usual cause.");
            System.exit(4);
        } catch (Throwable t) {
            System.err.println("AGENT_LOAD_FAILED: " + t);
            System.exit(3);
        } finally {
            try { vm.detach(); } catch (Throwable ignore) { }
        }
    }

    private AttachMain() { }
}
