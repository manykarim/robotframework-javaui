package com.testapp.rcp;

import org.eclipse.swt.graphics.Point;
import org.eclipse.ui.application.ActionBarAdvisor;
import org.eclipse.ui.application.IActionBarConfigurer;
import org.eclipse.ui.application.IWorkbenchWindowConfigurer;
import org.eclipse.ui.application.WorkbenchWindowAdvisor;

/**
 * This class configures the initial workbench window appearance and behavior.
 */
public class ApplicationWorkbenchWindowAdvisor extends WorkbenchWindowAdvisor {

    public ApplicationWorkbenchWindowAdvisor(IWorkbenchWindowConfigurer configurer) {
        super(configurer);
    }

    @Override
    public ActionBarAdvisor createActionBarAdvisor(IActionBarConfigurer configurer) {
        return new ApplicationActionBarAdvisor(configurer);
    }

    @Override
    public void preWindowOpen() {
        IWorkbenchWindowConfigurer configurer = getWindowConfigurer();

        // Set initial window size
        configurer.setInitialSize(new Point(1200, 800));

        // Configure window chrome
        configurer.setShowCoolBar(true);
        configurer.setShowStatusLine(true);
        configurer.setShowMenuBar(true);
        configurer.setShowPerspectiveBar(true);
        configurer.setShowProgressIndicator(true);

        // Set window title
        configurer.setTitle("RCP Test Application");

        System.out.println("[RcpTestApp] Window configuration complete");
    }

    @Override
    public void postWindowOpen() {
        super.postWindowOpen();
        System.out.println("[RcpTestApp] Window opened and ready");
    }

    @Override
    public boolean preWindowShellClose() {
        System.out.println("[RcpTestApp] Window closing...");
        return super.preWindowShellClose();
    }

    @Override
    public void postWindowClose() {
        System.out.println("[RcpTestApp] Window closed");
        super.postWindowClose();
    }
}
