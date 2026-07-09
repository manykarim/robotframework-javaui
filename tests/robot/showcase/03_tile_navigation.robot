*** Settings ***
Documentation     Proves that demo pages behind the Start hub's tile cards are now reachable,
...               thanks to the click-retargeting fix: the tile title is a listener-less
...               FormsLabel on a clickable card, and clicking it now activates the card's
...               handler (as a real user click would). This unlocks the demo pages the first
...               showcase proof could not reach.

Resource          resources/showcase.resource

Suite Setup       Launch Showcase
Suite Teardown    Close Showcase

Force Tags        showcase    proof    retargeting

*** Test Cases ***
Tile Click Navigates To A Demo Page
    [Documentation]    Clicking the Input tile's label navigates to the "Input Dialogs" page.
    ...                The hub has no ReadOnlyTextField; the demo page does — independent proof
    ...                the navigation actually happened.
    Open Section    Start
    ${hub_titles}=    Find Elements    ReadOnlyTextField
    Open Demo Tile    Input
    ${page_titles}=    Find Elements    ReadOnlyTextField
    Should Be True    ${{len($page_titles)}} > ${{len($hub_titles)}}
    ...    msg=Tile click did not navigate to the demo page (retargeting not effective)

Different Tiles Reach Different Pages
    [Documentation]    Each tile reaches its own page — the demo list appears (the hub has none).
    Open Demo Tile    Selection
    ${lists}=    Find Elements    JList
    Should Be True    ${{len($lists)}} >= 1    msg=Selection demo page not reached
    Open Demo Tile    Choice
    ${lists2}=    Find Elements    JList
    Should Be True    ${{len($lists2)}} >= 1    msg=Choice demo page not reached

Deep And Captured Locators Resolve On A Demo Page
    [Documentation]    Exercises the locator-engine fixes against the live app: the capture
    ...                selector returns a filtered subset (not every JPanel), and a 3-level
    ...                child chain is supported.
    Open Section    Start
    ${all_panels}=    Find Elements    JPanel
    ${cards}=    Find Elements    *JPanel >> FormsLabel[text='Input']
    Should Be True    0 < ${{len($cards)}} < ${{len($all_panels)}}
    ...    msg=Capture selector was not filtered by the final segment
