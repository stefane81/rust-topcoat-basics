use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn ui() -> Result {
    view! {
        <div style="position: absolute; left: 20px; bottom: 20px; z-index: 20; background-color: rgba(15, 23, 42, 0.95); border: 1px solid #1e293b; padding: 12px 18px; border-radius: 8px; font-family: sans-serif; color: #94a3b8; font-size: 11px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.3); pointer-events: none;">
          <div style="color: white; font-weight: bold; margin-bottom: 6px; font-size: 12px;">Keyboard Shortcuts</div>
          <div style="display: grid; grid-template-columns: auto auto; gap: 6px 16px;">
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Arrows</kbd> Navigate</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Enter</kbd> Edit Node Text</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Shift + Enter</kbd> Add Sibling</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Tab</kbd> Add Child</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Backspace</kbd> Delete Branch</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">Z / X</kbd> Zoom In / Out</div>
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">D</kbd> Toggle Done</div>
            <!-- New HUD Entry -->
            <div><kbd style="background: #334155; padding: 2px 4px; border-radius: 4px; color: white;">S</kbd> Auto-Tidy Layout</div>
          </div>
        </div>
    }
}
