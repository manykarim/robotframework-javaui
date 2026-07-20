import com.sun.tools.attach.VirtualMachine;
public class Attacher {
    public static void main(String[] a) throws Exception {
        String pid = a[0], jar = a[1], args = a.length > 2 ? a[2] : "";
        System.out.println("[attacher] attaching to pid=" + pid + " jar=" + jar + " args=" + args);
        VirtualMachine vm = VirtualMachine.attach(pid);
        try { vm.loadAgent(jar, args); System.out.println("[attacher] loadAgent OK"); }
        finally { vm.detach(); }
    }
}
