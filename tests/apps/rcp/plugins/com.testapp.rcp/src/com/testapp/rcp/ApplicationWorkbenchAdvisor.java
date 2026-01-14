package com.testapp.rcp;

import org.eclipse.ui.application.IWorkbenchConfigurer;
import org.eclipse.ui.application.IWorkbenchWindowConfigurer;
import org.eclipse.ui.application.WorkbenchAdvisor;
import org.eclipse.ui.application.WorkbenchWindowAdvisor;

/**
 * This workbench advisor creates the window advisor and specifies
 * the perspective ID for the initial window.
 */
public class ApplicationWorkbenchAdvisor extends WorkbenchAdvisor {

    /** Default perspective ID to show on startup */
    public static final String PERSPECTIVE_ID = "com.testapp.rcp.perspective.main";

    @Override
    public WorkbenchWindowAdvisor createWorkbenchWindowAdvisor(
            IWorkbenchWindowConfigurer configurer) {
        return new ApplicationWorkbenchWindowAdvisor(configurer);
    }

    @Override
    public String getInitialWindowPerspectiveId() {
        return PERSPECTIVE_ID;
    }

    @Override
    public void initialize(IWorkbenchConfigurer configurer) {
        super.initialize(configurer);

        // Save and restore workbench state
        configurer.setSaveAndRestore(true);

        System.out.println("[RcpTestApp] Workbench initialized");
    }

    @Override
    public void preStartup() {
        super.preStartup();
        System.out.println("[RcpTestApp] Pre-startup phase");
    }

    @Override
    public void postStartup() {
        super.postStartup();
        System.out.println("[RcpTestApp] Post-startup phase - workbench is ready");
    }

    @Override
    public boolean preShutdown() {
        System.out.println("[RcpTestApp] Pre-shutdown phase");
        return super.preShutdown();
    }

    @Override
    public void postShutdown() {
        System.out.println("[RcpTestApp] Post-shutdown phase");
        super.postShutdown();
    }
}
