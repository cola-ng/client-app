I have identified the cause of the issue: an invisible overlay (`user_btn_overlay`) used for the user menu hover detection is absolutely positioned and incorrectly overlaps the "Manage" (Close App) button, blocking its interaction.

I will fix this by removing the manual overlay and instead attaching the hover logic directly to the visible User Profile container.

**Plan:**
1.  **Modify `mofa-studio-shell/src/app.rs`**:
    *   Update `handle_user_menu_hover` to use `body.dashboard_base.header.user_profile_container` as the target for hover events instead of `user_btn_overlay`.
    *   Remove the `user_btn_overlay` widget definition from the `App` `live_design!` block.
    *   Remove the code in `update_overlay_positions` that updates the position of `user_btn_overlay`.

This change will ensure the hover detection area perfectly matches the visual user avatar button, preventing any interference with the adjacent "Manage" button.
