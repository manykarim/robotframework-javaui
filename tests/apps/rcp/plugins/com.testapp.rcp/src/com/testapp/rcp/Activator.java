package com.testapp.rcp;

import org.eclipse.ui.plugin.AbstractUIPlugin;
import org.osgi.framework.BundleContext;

/**
 * The activator class controls the plug-in life cycle.
 * This is the central class for the RCP Test Application plugin.
 */
public class Activator extends AbstractUIPlugin {

    /** The plug-in ID - used throughout the application */
    public static final String PLUGIN_ID = "com.testapp.rcp";

    /** The shared instance */
    private static Activator plugin;

    /**
     * The constructor.
     */
    public Activator() {
    }

    @Override
    public void start(BundleContext context) throws Exception {
        super.start(context);
        plugin = this;
        System.out.println("[RcpTestApp] Plugin started: " + PLUGIN_ID);
    }

    @Override
    public void stop(BundleContext context) throws Exception {
        plugin = null;
        super.stop(context);
        System.out.println("[RcpTestApp] Plugin stopped: " + PLUGIN_ID);
    }

    /**
     * Returns the shared instance.
     *
     * @return the shared instance
     */
    public static Activator getDefault() {
        return plugin;
    }
}
