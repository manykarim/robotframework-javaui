import javax.swing.*;
import java.lang.reflect.*;
public class MultiCtx {
    public static void main(String[] a) throws Exception {
        // AppContext #1 (main): a normal frame
        SwingUtilities.invokeLater(() -> mk("Main-Context Frame"));
        // AppContext #2: a separate AppContext on its own thread group (what WebStart/applets do)
        Class<?> stk = Class.forName("sun.awt.SunToolkit");
        Method createCtx = stk.getMethod("createNewAppContext");
        ThreadGroup tg = new ThreadGroup("ctx2");
        Thread t = new Thread(tg, () -> {
            try { createCtx.invoke(null); } catch (Throwable e) { e.printStackTrace(); }
            SwingUtilities.invokeLater(() -> mk("Second-Context Frame"));
            try { Thread.sleep(600000); } catch (InterruptedException ignored) {}
        });
        t.setDaemon(false); t.start();
        Thread.sleep(600000);
    }
    static void mk(String title) {
        JFrame f = new JFrame(title);
        f.setName(title.replace(' ', '_'));
        f.add(new JButton("btn-" + title));
        f.setSize(300, 120); f.setVisible(true);
    }
}
