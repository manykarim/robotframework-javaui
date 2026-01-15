package com.testapp.rcp.editors;

import org.eclipse.jface.resource.ImageDescriptor;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IPersistableElement;

/**
 * Simple editor input for file-based editing.
 * Used when opening files programmatically.
 */
public class FileEditorInput implements IEditorInput {

    private String name;
    private String path;
    private String toolTip;

    public FileEditorInput(String name, String path) {
        this.name = name;
        this.path = path;
        this.toolTip = path;
    }

    @Override
    public boolean exists() {
        // In a real implementation, check if file exists
        return true;
    }

    @Override
    public ImageDescriptor getImageDescriptor() {
        return null;
    }

    @Override
    public String getName() {
        return name;
    }

    @Override
    public IPersistableElement getPersistable() {
        return null;
    }

    @Override
    public String getToolTipText() {
        return toolTip;
    }

    @Override
    @SuppressWarnings("unchecked")
    public <T> T getAdapter(Class<T> adapter) {
        return null;
    }

    public String getPath() {
        return path;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null) return false;
        if (getClass() != obj.getClass()) return false;
        FileEditorInput other = (FileEditorInput) obj;
        if (path == null) {
            if (other.path != null) return false;
        } else if (!path.equals(other.path)) {
            return false;
        }
        return true;
    }

    @Override
    public int hashCode() {
        return path != null ? path.hashCode() : 0;
    }
}
