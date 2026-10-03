# Cheat Sheet Apps

Every font family ships a per-family `*-cheat-sheet` binary — nine in
total. They are thin wrappers around the shared `fonts-rs-cheat-sheet`
crate and only differ in their `CheatSheetFamily` type parameter:

```rust
fn main() -> miette::Result<()> {
    fonts_rs_cheat_sheet::run::<fonts_rs_lucide::Lucide>()
}
```

All UI behavior — grid, search, history, sidebars, toasts — is generic
over `F: CheatSheetFamily` and lives in `fonts-rs-cheat-sheet`.

## Libadwaita Widget Tree

The application is built on libadwaita (not plain GTK4). The widget
hierarchy inside `build_ui`:

```
adw::Application
└── adw::ApplicationWindow
    └── adw::ToastOverlay                  ← copy-feedback toasts
        └── adw::ToolbarView
            ├── adw::HeaderBar             ← WindowTitle, back/fwd,
            │                                sidebar toggle (left)
            └── adw::NavigationSplitView   ← browse ↔ detail
                ├── sidebar: adw::NavigationPage (browse)
                │   └── adw::OverlaySplitView      ← category list ↔ grid
                │       ├── sidebar: ScrolledWindow
                │       │   └── Box
                │       │       ├── ListBox  "Groups"      (if ordering())
                │       │       └── ListBox  "Categories"  (if any)
                │       └── content: Box
                │           ├── top_bar (counts, group DropDown,
                │           │            search Entry, size Scale,
                │           │            tone row)
                │           └── ScrolledWindow → FlowBox (icon grid)
                └── content: adw::NavigationPage (detail)
                    └── Stack
                        ├── "empty":  adw::StatusPage
                        └── "detail": adw::ToolbarView
                            ├── adw::HeaderBar (close button only —
                            │   window controls suppressed)
                            └── ScrolledWindow → detail content
```

### NavigationSplitView (browse ↔ detail)

The browse grid and the detail page are the two panes of a
`NavigationSplitView`. An `adw::Breakpoint` (`max-width: 800sp`)
collapses the split view so the detail page covers the grid on narrow
windows; its own `HeaderBar` then shows an automatic back button plus a
manual close button. Window controls are suppressed on the detail
`HeaderBar` (`show_start/end_title_buttons(false)`) so they are not
duplicated.

### OverlaySplitView (category sidebar ↔ grid)

Inside the browse page, an `adw::OverlaySplitView`
(`sidebar_position = Start`) carries the navigation sidebar. It is
**revealed at startup** and toggled by a `ToggleButton` in the main
`HeaderBar`, bound bidirectionally to `show-sidebar`. The sidebar is
omitted entirely when a family exposes neither `ordering()` groups nor
categories.

Sidebar content is a sectioned list in one `ScrolledWindow`:

- **Groups** — one row per `group_labels()` entry. Rows route through
  `DropDown::set_selected`, so `active_group` + grid re-render share a
  single code path; re-activating the selected group resets to
  "All groups".
- **Categories** — `"{category} ({count})"` rows, collected once into a
  `BTreeMap` over `all_glyphs()`. Click replaces the search text,
  Shift+click appends (space-separated AND), clicking the active row
  clears back to the full grid. Category pill clicks (`cat-btn` on grid
  cells and the detail page) reveal the sidebar when collapsed.

### ToastOverlay (copy feedback)

Every copy action shows an `adw::Toast` with per-action wording —
"Copied name" / "Copied character" / "Copied SVG" / "Copied codepoint"
in the detail view, plus "Copied name" on secondary-click of a grid
cell. The overlay is shared through `DetailContext::toasts`.

## Search- and History Model

- The search `Entry` (primary `edit-find-symbolic` icon, secondary
  clear icon) filters the grid; every change pushes a
  `HistoryEntry::Search` onto the back/forward stack, so Back/Forward
  restore previous filters.
- Selecting a glyph pushes `HistoryEntry::Icon` and shows the detail
  page; `navigate_to` restores either kind of entry.
- `suppress_history` prevents loop-pushing while navigating history.

## Styling

Custom CSS is intentionally thin — cell sizing, metadata pills
(`cat-btn`/`kw-btn`), and hover/selected states use adwaita palette
variables (`alpha(@accent_bg_color, …)`, `@success_bg_color`,
`@warning_bg_color`). Sidebar theming, dark mode, and focus styles come
from the adwaita stylesheet via `adw::StyleManager`.
