---
name: pick-up-issue
description: Pick up a CityMapMaker GitHub issue end to end, the way ADR-040 and ADR-073 set it out — pre-flight check, read the issue, ask "quick check or full grill?", ten-line design and questions, plan file with a decisions log, then the seven stages up to the hand-back and Jeffrey's joint review, and only then the PR. Use this whenever Jeffrey asks to "pick up", "start", "work on" or "do" an issue by number (for example "pick up #260"). Do NOT use it to create issues or to review someone else's PR.
---

# Pick up an issue

The procedure for one issue in this repository. The rules it follows live in CLAUDE.md
(working agreement, definition of done), DECISIONS.md ADR-040 and ADR-073, and
CODING_STANDARDS.md "Review hygiene". This file says in which order to apply them; where it
and those documents differ, the documents win and this file gets fixed.

Run the steps in order. Skip a step only where this file says so (step 3, trivial fix); a
skipped step is named in the hand-back.

## Step 1: Pre-flight

1. **Issue number.** Take it from Jeffrey's message. Several numbers: ask which one. None:
   ask before anything else.
2. **Working tree.** Run `git status --porcelain` and `git branch --show-current`. A line
   with `M`, `A`, `D`, `R`, `C` or `U` in its first two columns is work in progress; `??` is
   not. Branch names carry no issue number, so find the issue of the current branch through
   its PR: `gh pr view --json closingIssuesReferences`, or a plan file in `.claude/plans/`
   that names the branch. With work in progress, or on a branch of another issue, show the
   branch and the changed files and ask Jeffrey: 1. commit, 2. stash, 3. work in another
   worktree. Never stash without his answer.
3. **Worktree.** Compare `git rev-parse --git-dir` with `git rev-parse --git-common-dir`; when
   they differ, this is a linked worktree. Run `git worktree list`: its first path is the main
   checkout. Note where each branch is checked out.
4. **Existing work.** Look for an open PR of this issue:
   `gh pr list --search "<number>" --state open --json number,headRefName`, and for
   `.claude/plans/issue-<number>.md` in the main checkout; for a sub-issue, also for the
   parent's plan file (`gh issue view <number> --json parent`). Found a PR: tell Jeffrey which
   branch and ask whether to continue on it. Found a plan file: read it; its decisions stand
   unless Jeffrey reopens them.
5. **Labels and milestone.** `gh issue view <number> --json labels,milestone`. The issue
   needs a kind label, a `tier:` label and a milestone. Missing one: ask Jeffrey, with
   options that include "none" for an issue outside the milestones, such as a process
   issue. Jeffrey decides.
6. **Dependencies.** Read the issue body and comments for issues it depends on
   ("depends on", "after", "blocked by", a stack). An open dependency: tell Jeffrey and ask
   whether to wait, stack on it, or go ahead.

## Step 2: Read the issue

Read the title, body, labels, milestone and every comment
(`gh issue view <number> --comments`). Read each linked issue and PR for context, without
grilling on them. Read every document section and ADR the issue names. When the issue
touches lore (names, landmarks, geography), read the vault notes; CLAUDE.md "The vault"
says where they live and that a missing fact becomes a GM? question.

Tell Jeffrey in one or two sentences which issue this is and what it asks.

## Step 3: Quick check or full grill

Ask Jeffrey once: quick check or full grill. Give a recommendation with a one-line reason:

- **Quick check** for a small, well-scoped issue: only the questions that change the plan,
  two to four.
- **Full grill** for a design issue: keep asking until every open branch of the design is
  settled.

The answer holds for steps 4 and 5. Do not ask it again for the same issue.

A trivial fix, as CODING_STANDARDS.md "Review hygiene" defines it, skips steps 3 to 5: say
that it is trivial and why, then go to step 6. A sub-issue whose parent already has a plan
file and Jeffrey's yes skips steps 3 to 5 too: its plan is the parent's (see "Split work and
stacks"). When the issue belongs to a batch (ADR-053), steps 4 and 5 cover the whole batch
and Jeffrey's yes covers every issue in it.

## Step 4: Ten-line design and questions

Read the code and documents first, so that every question is one only Jeffrey can answer.
Then post the design: at most ten lines, saying what changes, where, and in what order.
Under it, ask the open questions under CLAUDE.md "Explicit over implied": one question at a
time, options numbered (grouped questions lettered), the recommended option first with a
one-line reason. Wait for each answer before the next question.

## Step 5: Expanded plan and the yes

With the answers in, write the expanded plan: the files to change and what goes in each,
the tests ARCHITECTURE.md §8 requires, the DECISIONS.md entries, the LEARNING.md entries for
new concepts, and anything still open. Count the files.

**File limit.** At most 10 changed files per PR; `.snap` files do not count. A plan over
10 files lists its slices, each one PR of at most 10 files; "Split work and stacks" below
says how. A plan that cannot be split (a rename across many files) says so; Jeffrey decides
on it with his yes.

Ask Jeffrey for his yes on the expanded plan; this is the yes that ends stage 1 (ADR-073).
Do not write code before it.

## Step 6: Branch and plan file

1. Create the branch from the newest `main`: `git fetch origin`, then
   `git switch -c <branch> origin/main`; in a stack, from the branch below instead. The
   branch name follows CODING_STANDARDS.md "Review hygiene", Flow.
2. Write `.claude/plans/issue-<number>.md` in the main checkout (git ignores
   `.claude/plans/`; never commit it). In a worktree, write it by its full path in the main
   checkout. Template:

   ```markdown
   # #<number>: <title>

   ## Understanding
   ## Plan
   ## Decisions
   ## Open questions
   ```

3. **Decisions log.** Each time a choice between real alternatives is made, during the
   build, the review or the cut, add a line under `## Decisions`: what was chosen, what was
   rejected, why, and who decided. A choice that is a real design choice also gets its
   DECISIONS.md entry in the same commit (CLAUDE.md). The hand-back is written from this log.

## Step 7: Stages 2 to 5 of ADR-040

1. **Stage 2, write.** Commits follow "Review hygiene". When the plan turns out wrong, stop
   and tell Jeffrey; do not diverge silently.
2. **Stage 3, review.** Start one review agent with the diff against the base branch and the
   documents (CLAUDE.md, the documents the diff touches, the issue); never give it Claude's
   conclusions. Meanwhile review the same diff yourself. Merge both lists, check every
   finding against the code, and fix what is real. Then run the checks: for a change with
   Rust, `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and
   `cargo test --workspace`; for every change, `python analysis/check_documents.py main`.
   "Run the tests" below means these checks.
3. **Stage 4, cut.** Go through the diff with one goal: remove every line that is not
   needed to keep the core behaviour. Dead code, duplication, single-caller abstractions,
   checks for impossible cases, options nobody asked for, comments that restate the code.
   Cut in its own commit; run the tests again. Record what was cut, so Jeffrey can decide on
   each cut at stage 6.
4. **Stage 5, review again.** Review the diff after the cut, fix, run the tests.

In split work, steps 7 to 10 run per sub-issue: one sub-issue reaches its PR before the next
one starts stage 2.

## Step 8: Hand-back

Send the three parts together, in this order, at the start of stage 6.

**Status.** The issue, the branch and its base, the file count, what the review found and
fixed, what the cut removed (lines, and each cut Jeffrey decides on), open questions.

**Major changes** (ADR-073, G1). Three to six for a normal issue; a one-line fix gets a
sentence or two. A change is major when it changes behaviour, adds or reshapes a type or a
stage output, touches shared code (the `geometry` facade, `random`, `identifier`), or involved a choice
between real alternatives. Per change, one short paragraph:

- what changed and the `file:line` to start reading;
- why it was done this way: the rule, ADR or requirement behind it;
- the alternative rejected and why, when there was one;
- what it costs or puts at risk;
- the Rust idioms in it, explained for someone learning Rust.

Then: where the build left the approved plan and why, and the choice Claude is least sure
about. Take the reasons from the decisions log, not from rereading the diff.

**Test guide.** For each check Jeffrey does by hand, numbered steps: the setup (branch,
commands to run first), the exact command or file to open (for a visual change, the PNG in
`output/` and the `generate` command that wrote it), and what he should see. Order the
checks by value, the one most likely to find a real problem first, each with one line on
why it is worth doing by hand.

Then list what needs no manual check, each with its reason: the existing test that covers
it (name the file), a test added in this change (name it and what it asserts), or cheap to
change if wrong. Something covered neither by a test nor by a manual check is a **gap** and
is called one. Write only commands and paths that exist in the repository. When Claude ran a
check, say what was run and what was seen; never present a check as run when it was only
reasoned about.

## Step 9: Stage 6, the joint review

Review the change with Jeffrey in the conversation. Jeffrey decides on each cut. Fix what
he asks for, update the decisions log, and run the tests again.

## Step 10: Stage 7, the PR

After Jeffrey's yes:

1. Run the documents check of CODING_STANDARDS.md "Review hygiene".
2. Optional, Claude decides by time and quota: the local CodeRabbit command line
   (ADR-040). No step waits for it.
3. Rebase on `main` (or the branch below) and push; an updated branch is pushed with
   `git push --force-with-lease` (ADR-041).
4. Open the PR with `gh pr create`; in a stack, with `gh stack submit --auto`, which opens a
   PR only for the branch that has none, followed by `gh pr edit <number> --title <title>
   --body-file <file>`. Title and body follow "Review hygiene": a conventional commit title,
   the tier or milestone, `Closes #<number>`, the Rust idioms explained. The body also
   carries a short version of the hand-back.

Never merge, approve or close a PR; Jeffrey reviews and merges (ADR-014, ADR-041).
CodeRabbit reviews on GitHub only when Jeffrey asks for it (ADR-040).

## Split work and stacks (ADR-073, H1 and I1)

**Splitting.** When the expanded plan of step 5 has more than one slice, the issue becomes a
parent issue. After Jeffrey's yes, create one sub-issue per slice with
`gh issue create --parent <parent number>`, with its own labels and milestone, in the order
of the plan. The plan file stays `issue-<parent>.md` and lists the sub-issues. Each
sub-issue gets exactly one PR, which closes it; the PR of the last sub-issue also closes the
parent.

**Stack or loose.** A slice that builds on the slice before it is stacked on that branch; a
slice that does not starts from `main`. The plan says which, per slice. Another issue can
stack on the top branch of a stack; step 1.6 asks Jeffrey.

**Stacking.** `gh stack init <first branch>` creates the first slice's branch on `main`;
`gh stack add <branch>` creates each next slice's branch on top of the current one. Each PR opens right after its own stage 6 (step 10), never earlier; Jeffrey reviews
and merges the PRs one at a time, bottom first, while the next slice is built.

**After Jeffrey merges a PR of the stack.** Run `gh stack sync` before any other work on
the stack. When it stops on a conflict with the merged PR's commits (a squash merge keeps
them in the branch above), run `git rebase --onto origin/main <merged branch> <next branch>`
and push with `git push --force-with-lease`. Tell Jeffrey the next PR is ready only after
this.
