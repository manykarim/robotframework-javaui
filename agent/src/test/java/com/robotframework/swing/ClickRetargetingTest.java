package com.robotframework.swing;

import org.junit.jupiter.api.Test;

import javax.swing.JLabel;
import javax.swing.JPanel;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

/**
 * Verifies that a synthetic click on a listener-less child is retargeted to the nearest
 * ancestor that has mouse listeners — matching AWT's LightweightDispatcher behavior, so
 * "if a user can steer the app by clicking, the library can too". No showcase jar required.
 */
class ClickRetargetingTest {

    @Test
    void resolveClickTargetWalksUpToTheListenerBearingCard() {
        JPanel card = new JPanel();
        card.addMouseListener(new MouseAdapter() { });
        JLabel label = new JLabel("Input");   // listener-less child, like a tile's FormsLabel
        card.add(label);
        card.setSize(228, 112);
        label.setBounds(10, 10, 60, 20);

        assertSame(card, ActionExecutor.resolveClickTarget(label),
                "click on a listener-less child should retarget to the card that has the listener");
    }

    @Test
    void resolveClickTargetKeepsAComponentThatHasItsOwnListener() {
        JPanel card = new JPanel();
        card.addMouseListener(new MouseAdapter() { });
        JLabel label = new JLabel("Input");
        label.addMouseListener(new MouseAdapter() { });   // the label handles clicks itself
        card.add(label);

        assertSame(label, ActionExecutor.resolveClickTarget(label),
                "a component with its own listener must not be retargeted");
    }

    @Test
    void resolveClickTargetKeepsTheTargetWhenNoAncestorHasListeners() {
        JPanel container = new JPanel();       // no listeners anywhere
        JLabel label = new JLabel("Input");
        container.add(label);

        assertSame(label, ActionExecutor.resolveClickTarget(label),
                "with no listener-bearing ancestor, the original target is used");
    }

    @Test
    void performMouseClickReachesTheAncestorHandler() {
        AtomicInteger released = new AtomicInteger();
        AtomicInteger clicked = new AtomicInteger();
        JPanel card = new JPanel();
        card.setSize(228, 112);
        card.addMouseListener(new MouseAdapter() {
            @Override public void mouseReleased(MouseEvent e) { released.incrementAndGet(); }
            @Override public void mouseClicked(MouseEvent e) { clicked.incrementAndGet(); }
        });
        JLabel label = new JLabel("Input");
        label.setBounds(10, 10, 60, 20);
        card.add(label);

        // Clicking the listener-less label must fire the card's handler (as a real click would).
        ActionExecutor.performMouseClick(label, 1);

        assertEquals(1, released.get(), "card mouseReleased should fire once");
        assertEquals(1, clicked.get(), "card mouseClicked should fire once");
    }
}
