# Complete Phase Item

## Overview
Check the phase plan, find the next task to be completed, and complete it following the project workflow. Works on section-specific branches (e.g., `phase-1-section-verification-wrap-up`) to enable automated PR creation when sections are complete. If on `develop` branch, creates a section-specific branch automatically.

## Steps
1. **Reference documents first**
   - Read `@docs/phase-plan.md` to understand phase structure
   - Read `@docs/architecture.md` for technology stack and patterns
   - Read `@docs/requirements.md` for feature requirements
   - For UI work: Reference `/design/*.html` files (see `@design-rules.mdc`)

2. **Find next incomplete section and determine branch**
   - Open the specific phase file (`@docs/phases/phase-X.md`)
   - Extract phase number from filename (e.g., `phase-1.md` → phase number is `1`)
   - Find the next incomplete section:
     - Look for sections marked with `### Section Name`
     - Check if the section has `- [x] Section Complete` checked immediately after the section header
     - The first section without "Section Complete" checked is the active section
   - Create section slug from section name:
     - Convert to lowercase
     - Replace spaces and special characters with dashes
     - Remove consecutive dashes
     - Example: "Verification & Wrap-Up" → "verification-wrap-up"
   - Section branch name: `phase-${PHASE_NUM}-section-${SECTION_SLUG}`

3. **Check current branch and create section branch if needed**
   - Check current branch: `CURRENT_BRANCH=$(git branch --show-current)`
   - If on `develop` branch:
     - **Fetch and pull latest develop:** `git fetch origin develop && git pull origin develop`
     - Create section branch: `SECTION_BRANCH="phase-${PHASE_NUM}-section-${SECTION_SLUG}"`
     - Check if section branch exists: `git show-ref --verify --quiet refs/heads/${SECTION_BRANCH}`
     - If branch doesn't exist:
       - Create and checkout from develop: `git checkout -b ${SECTION_BRANCH} develop`
       - Push to remote: `git push -u origin ${SECTION_BRANCH}`
     - If branch exists:
       - Switch to it: `git checkout ${SECTION_BRANCH}`
       - **Pull latest changes:** `git pull origin ${SECTION_BRANCH}`
   - If already on a section branch (pattern `phase-*-section-*`):
     - **Fetch and pull latest changes:** `git fetch origin ${CURRENT_BRANCH} && git pull origin ${CURRENT_BRANCH}`
     - Verify we're on the correct section branch for the active section
   - If on a phase branch (pattern `phase-*` but not `phase-*-section-*`):
     - Switch to or create the appropriate section branch for the active section
   - If on other branches (`setup`, `master`), work directly on that branch

4. **Find next task in active section**
   - Within the active section, find the next task to be completed (first unchecked task)
   - Tasks are expressed as checklist items like `- [ ] T00X Short description`
   - Verify dependencies are complete
   - **CRITICAL: Do NOT mark the phase complete checkbox (`- [x] Phase X Complete`)**
   - **CRITICAL: Do NOT update the phase status to "✅ Complete"**
   - **CRITICAL: Only complete ONE task at a time**

5. **Complete phase task**
   - Work on one task at a time
   - Follow architecture patterns and design specifications
   - Update the checklist entry for the selected task by switching `- [ ] T00X ...` to `- [x] T00X ...` and leave the task identifier (`T00X`) and description intact
   - Do not add per-task status lines; keep only the checklist item
   - If manual setup is needed, add instructions to `@docs/setup.md`
   - **DO NOT check the "Phase X Complete" checkbox**
   - **DO NOT change the phase-level status**
   - **After completing the last task in a section:** Check the "Section Complete" checkbox (`- [ ] Section Complete` → `- [x] Section Complete`) for that section

6. **Verify completion**
   - Confirm code meets requirements
   - Ensure no build errors
   - Confirm only the targeted task checkbox changed from `- [ ]` to `- [x]`
   - If this was the last task in the section, verify "Section Complete" is checked
   - Verify phase completion checkbox remains unchecked
   - Verify phase status does NOT show "✅ Complete"

7. **Commit changes**
   - Commit changes with descriptive message: `feat: complete T00X - [Task Name]`
   - Example: `feat: complete T001 - Confirm implementation scope`
   - **IMPORTANT:** Only commit changes at this step - DO NOT push yet
   - **IMPORTANT:** Pushing is only done when a section is complete (see step 8)

8. **Push changes (only when section is complete)**
   - **If this was the last task in the section (Section Complete is checked):**
     - Push changes to current branch: `git push origin $(git branch --show-current)`
     - **This push will trigger GitHub Actions to automatically create a PR for the completed section**
   - **If this was NOT the last task in the section:**
     - **DO NOT push** - keep changes local until the section is complete
     - You can continue working on the next task in the section
   - **IMPORTANT:** Only push when "Section Complete" is checked to avoid triggering false PRs
   - **IMPORTANT:** Only use `@complete-phase.md` command when ALL tasks in ALL sections are done to mark phase complete