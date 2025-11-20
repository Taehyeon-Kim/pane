# Tips Authoring Guide

## Introduction

This guide explains how to create and contribute high-quality tips for the Claude Code Tips Viewer bundled skill. The tips database (`data/claude-tips.yaml`) contains curated best practices, workflows, and optimization techniques for Claude Code users.

## Tip Format

Tips are stored in YAML format with a strict schema. Each tip must include all required fields.

### YAML Structure

```yaml
- id: "cc-001"              # Required: Unique identifier
  title: "Tip title here"   # Required: Clear, actionable title
  category: "prompting"     # Required: One of 5 valid categories
  text: |                   # Required: Multi-line tip content
    Concise, actionable tip text here.
    2-5 sentences recommended for readability.
    Terminal-friendly formatting only.
  tags: ["tag1", "tag2"]    # Required: 3-5 searchable tags
```

### Field Descriptions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | Yes | Unique identifier following "cc-XXX" pattern (zero-padded 3-digit number) |
| `title` | String | Yes | Clear, actionable title (3-8 words) |
| `category` | String | Yes | Must be one of: `prompting`, `cost`, `workflow`, `debugging`, `best-practices` |
| `text` | String | Yes | Multi-line tip content (2-5 sentences, ~50-150 words) |
| `tags` | Array | Yes | 3-5 relevant, searchable keywords |

## Writing Guidelines

### 1. Clear, Actionable Titles

**Good Examples:**
- "Use git diff to review changes"
- "Limit context window for focused work"
- "Request explanations for learning"

**Poor Examples:**
- "Claude Code is great!" (not actionable)
- "Some tips about prompting and stuff" (vague)
- "How to use the thing for doing tasks" (unclear)

**Rules:**
- 3-8 words maximum
- Start with a verb when possible ("Use", "Request", "Ask", "Enable", "Check")
- Be specific about the action or benefit

### 2. Concise, Useful Text

**Good Example:**
```yaml
text: |
  Before committing code, verify that it compiles and passes linting checks.
  Run your build command or linter to catch errors early. This prevents broken
  commits and reduces debugging time later.
```

**Poor Example:**
```yaml
text: |
  You should probably think about maybe running some tests or something before
  you commit your code because it might be broken and that would be bad for
  everyone and cause problems down the line so it's important to be careful.
```

**Rules:**
- 2-5 sentences (50-150 words)
- Focus on ONE concept or technique
- Include practical "why" (benefit) not just "what"
- Avoid filler words ("probably", "maybe", "might", "just")
- Be direct and confident

### 3. Terminal-Friendly Formatting

**Good Practices:**
- Plain text only (no Markdown bold, italics, or special formatting)
- Short, punchy sentences
- Natural line breaks for readability
- Test wrapping in 80-column terminal

**Avoid:**
- ASCII art or box drawing characters
- Complex tables or diagrams
- Lines longer than 120 characters
- Special Unicode characters

### 4. Appropriate Categorization

Choose the category that best matches the tip's primary focus:

#### `prompting`
How to write effective prompts and interact with Claude Code.
- Example: Asking for iterative refinement, providing context, being specific

#### `cost`
Token optimization, context management, and cost efficiency.
- Example: Limiting context windows, batching operations, closing unused threads

#### `workflow`
Efficient usage patterns, shortcuts, and integration features.
- Example: Slash commands, git integration, automation

#### `debugging`
Troubleshooting, error handling, and problem resolution.
- Example: Debug logging, testing error paths, checking build status

#### `best-practices`
Code quality, maintainability, and professional development practices.
- Example: Code review, consistent style, documentation

### 5. Relevant, Searchable Tags

**Good Tag Examples:**
- Specific technologies: "git", "testing", "logging"
- Actions: "workflow", "prompting", "debugging"
- Concepts: "context", "efficiency", "quality"

**Poor Tag Examples:**
- Too generic: "code", "programming", "good"
- Too specific: "function-declaration-in-typescript"
- Redundant with category: tip in "workflow" category shouldn't only have "workflow" tag

**Rules:**
- 3-5 tags per tip
- Use lowercase
- Be consistent with existing tags
- Think about what users would search for

## ID Naming Convention

**Pattern:** `cc-XXX` where XXX is a zero-padded 3-digit number

**Examples:**
- `cc-001`, `cc-002`, ..., `cc-009` (first 9 tips)
- `cc-010`, `cc-011`, ..., `cc-099` (tips 10-99)
- `cc-100`, `cc-101`, ..., `cc-999` (tips 100+)

**Rules:**
- Always use "cc-" prefix (stands for "Claude Code")
- Use exactly 3 digits with leading zeros
- IDs must be unique across all tips
- Assign the next sequential number when adding new tips

## Validation Rules

All tips must pass these validation checks:

### Required Fields
- ✅ All 5 fields must be present: id, title, category, text, tags
- ✅ No fields can be empty strings or empty arrays

### Unique IDs
- ✅ Each tip ID must be globally unique
- ✅ Duplicate IDs will cause parser errors

### Category Constraints
- ✅ Category must be one of the 5 valid categories (case-sensitive)
- ✅ Invalid categories will cause validation errors

### ID Format
- ✅ ID must start with "cc-"
- ✅ ID must have numeric suffix (zero-padded 3 digits)

## Examples: Good vs Poor Tips

### Good Tip Example

```yaml
- id: "cc-018"
  title: "Break down complex tasks iteratively"
  category: "prompting"
  text: |
    Instead of asking Claude Code to implement an entire feature at once, break it
    into smaller steps. Request one component or function at a time, review the output,
    and refine before moving to the next step. This produces higher quality results.
  tags: ["prompting", "iterative", "workflow", "quality"]
```

**Why it's good:**
- ✅ Clear, action-oriented title
- ✅ Concise text (3 sentences, ~50 words)
- ✅ Explains both "what" and "why"
- ✅ Appropriate category
- ✅ Relevant, searchable tags

### Poor Tip Example

```yaml
- id: "bad-tip"
  title: "Claude Code stuff"
  category: "general"
  text: |
    Claude Code is really useful and can help you with many things. You should
    try using it more often because it will make your work easier and faster.
    It's great for all kinds of tasks and very powerful!
  tags: ["claude"]
```

**Why it's poor:**
- ❌ ID doesn't follow "cc-XXX" format
- ❌ Vague, non-actionable title
- ❌ Invalid category ("general" doesn't exist)
- ❌ Not specific or actionable (no concrete advice)
- ❌ No clear benefit or technique explained
- ❌ Insufficient tags (only 1)

## Checklist for Tip Authors

Use this checklist when adding new tips:

### Content Quality
- [ ] Title is clear and actionable (3-8 words)
- [ ] Text is concise (2-5 sentences, 50-150 words)
- [ ] Tip provides specific, practical advice
- [ ] Benefit or "why" is explained
- [ ] Text is terminal-friendly (no Markdown, short lines)

### Format Compliance
- [ ] ID follows "cc-XXX" pattern with zero-padding
- [ ] ID is unique (not used by any other tip)
- [ ] Category is one of the 5 valid categories
- [ ] 3-5 relevant tags provided
- [ ] All required fields present

### Validation
- [ ] YAML syntax is valid (no parser errors)
- [ ] Tip loads successfully with parser
- [ ] Unit tests pass: `cargo test --package claude-tips`
- [ ] Integration tests pass: `cargo test --package pane --test claude_tips_e2e`

### Testing
- [ ] Tip displays correctly in TUI
- [ ] Tip is searchable by tags and title
- [ ] Tip wraps correctly in 80-column terminal
- [ ] No text truncation or display issues

## Adding New Tips

### Step-by-Step Process

1. **Determine next ID:** Check the last tip in `data/claude-tips.yaml` and increment

2. **Draft tip content:**
   - Choose appropriate category
   - Write clear title and concise text
   - Select 3-5 relevant tags

3. **Add to YAML file:**
   ```bash
   # Edit the tips file
   vim skills/claude-tips/data/claude-tips.yaml
   ```

4. **Validate syntax:**
   ```bash
   # Run tip parser tests
   cargo test --package claude-tips
   ```

5. **Run integration tests:**
   ```bash
   # Run E2E tests
   cargo test --package pane --test claude_tips_e2e
   ```

6. **Manual testing:**
   ```bash
   # Build and run the skill
   cargo run --package claude-tips --release
   ```

7. **Verify in TUI:**
   - Browse to your new tip
   - Search for it using tags
   - Check text wrapping in different terminal sizes

8. **Commit changes:**
   ```bash
   git add skills/claude-tips/data/claude-tips.yaml
   git commit -m "Add new tip: <title>"
   ```

## Tips Content Guidelines

### Audience
- **Target users:** Developers using Claude Code for the first time through experienced power users
- **Assume:** Basic programming knowledge, familiarity with terminals and git
- **Don't assume:** Deep knowledge of AI prompting or Claude Code features

### Content Sources
- Official Claude Code documentation
- Common user questions and pain points
- Best practices from experienced users
- Token optimization techniques
- Workflow efficiency patterns

### Quality Standards
- **Actionable:** Every tip provides a specific action or technique
- **Accurate:** Verify against official Claude Code documentation
- **Useful:** Address real user needs or common questions
- **Concise:** Get to the point quickly
- **Searchable:** Use terms users would actually search for

## Maintenance

### Updating Existing Tips
- Preserve the tip ID when updating content
- Update text or tags to reflect changes in Claude Code features
- Run validation tests after any modifications

### Removing Deprecated Tips
- Mark deprecated tips with category change or removal
- Update integration tests if category distribution changes
- Document removal reasons in commit message

## Getting Help

If you have questions about tip authoring:
- Review existing tips in `data/claude-tips.yaml` for examples
- Check the validation errors from test output
- Reference this guide's examples and checklists
