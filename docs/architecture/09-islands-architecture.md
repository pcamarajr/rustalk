# Islands Architecture - RUSTALK Component Pattern

**Version:** 1.0  
**Date:** 2025-11-14  
**Status:** Active Standard

## Overview

RUSTALK follows the **Islands Architecture** pattern for organizing UI components and screens. This pattern ensures that components are modular, self-contained, and maintainable while keeping pages lightweight and focused.

## What is Islands Architecture?

Islands Architecture is a frontend pattern where:

- **Pages are simple shells** that compose multiple independent "island" components
- **Each island is self-contained** with its own state and logic
- **Islands operate independently** but can share state when needed
- **The majority of the page is static HTML** with interactive islands added only where needed

This pattern was popularized by [Astro](https://docs.astro.build/en/concepts/islands/) and adapted for SvelteKit in RUSTALK.

## Core Principles

### 1. Self-Contained Islands

Each island component manages its own state internally. Islands should be independent and not rely on parent component state unless absolutely necessary.

**✅ Good:**

```svelte
<!-- AccountSettings.svelte - Self-contained -->
<script lang="ts">
  let userName = $state("John Doe");
  let userEmail = $state("john.doe@example.com");
  // Component manages its own state
</script>
```

**❌ Avoid:**

```svelte
<!-- Parent passes all state down -->
<script lang="ts">
  let { userName, userEmail, connectionStatus } = $props();
  // Component depends on parent state
</script>
```

### 2. Simple Shell Pages

Pages should be minimal shells that import and compose island components. Pages should not contain complex logic or extensive state management.

**✅ Good:**

```svelte
<!-- +page.svelte - Simple shell -->
<script lang="ts">
  import AccountSettings from "$lib/components/settings/AccountSettings.svelte";
  import AudioSettings from "$lib/components/settings/AudioSettings.svelte";
</script>

<div class="p-6 space-y-6">
  <AccountSettings />
  <AudioSettings />
</div>
```

**❌ Avoid:**

```svelte
<!-- +page.svelte - Complex page with all logic -->
<script lang="ts">
  // 500+ lines of state and logic
  let userName = $state("...");
  let microphones = $state([...]);
  // ... many more state variables
</script>
```

### 3. Modular Component Organization

Related islands should be organized in folders that reflect their domain or feature area.

**Structure:**

```
src/lib/components/
├── settings/
│   ├── AccountSettings.svelte
│   ├── GeneralSettings.svelte
│   ├── SIPAccountSettings.svelte
│   └── audio/
│       ├── AudioSettings.svelte
│       ├── MicrophoneSettings.svelte
│       ├── SpeakerSettings.svelte
│       └── RingtoneSettings.svelte
```

### 4. Break Down Large Sections

Large sections with multiple sub-features should be broken into smaller islands. For example, Audio Settings contains multiple sub-islands (Microphone, Speaker, Ringtone, Audio Processing).

**✅ Good:**

```svelte
<!-- AudioSettings.svelte - Container for sub-islands -->
<script lang="ts">
  import MicrophoneSettings from "./MicrophoneSettings.svelte";
  import SpeakerSettings from "./SpeakerSettings.svelte";
  import RingtoneSettings from "./RingtoneSettings.svelte";
</script>

<Card>
  <CardHeader>Audio Settings</CardHeader>
  <CardContent>
    <MicrophoneSettings />
    <SpeakerSettings />
    <RingtoneSettings />
  </CardContent>
</Card>
```

## When to Use Islands Architecture

### ✅ Use Islands For:

- **Settings screens** with multiple sections (Account, Audio, SIP, General, etc.)
- **Complex forms** with multiple sub-sections
- **Feature areas** that can be logically separated
- **Reusable components** that might appear in multiple contexts
- **Components with significant interactivity** (forms, controls, selectors)

### ⚠️ Consider Simpler Approach For:

- **Simple pages** with minimal interactivity
- **Single-purpose components** that don't need isolation
- **Layout components** (headers, footers, sidebars)
- **Small utility components** (buttons, badges, icons)

## Implementation Guidelines

### Component Structure

1. **Create folder structure** that reflects the feature domain

   ```
   src/lib/components/{feature}/
   ```

2. **Name components** descriptively: `{Feature}Settings.svelte`, `{Feature}Controls.svelte`

3. **Keep pages simple** - pages should only import and compose islands

4. **Break down large islands** - if an island exceeds ~200 lines, consider splitting into sub-islands

### State Management

1. **Self-contained state** - Each island manages its own state using `$state`
2. **Avoid prop drilling** - Don't pass state through multiple component layers
3. **Use stores when needed** - For shared state across islands, use Svelte stores
4. **Keep state local** - Only lift state up if multiple islands need to share it

### Folder Organization

```
src/lib/components/
├── {feature}/              # Feature-specific islands
│   ├── {Feature}Section.svelte
│   └── {sub-feature}/      # Sub-feature islands
│       ├── {SubFeature}Settings.svelte
│       └── index.ts        # Barrel exports (optional)
```

### Simple Decision Checklist

When creating a new screen or component, ask:

1. **Is this a complex screen with multiple sections?** → Use Islands Architecture
2. **Can this be broken into logical sub-sections?** → Create sub-islands
3. **Does each section have its own state/logic?** → Make it a self-contained island
4. **Is the page becoming too large (>300 lines)?** → Extract islands
5. **Will this component be reused elsewhere?** → Make it an island

## Related Patterns

For detailed patterns on implementing self-contained components, see:

- **Dialog Patterns**: [Dialog and State Management Patterns](../../development/svelte-patterns.md#dialog-and-state-management-patterns) - How to create self-contained dialogs
- **State Management**: [Dialog and State Management Patterns](../../development/svelte-patterns.md#dialog-and-state-management-patterns) - Component-managed state patterns
- **Composables**: [Dialog and State Management Patterns](../../development/svelte-patterns.md#shared-functionality-composable-pattern) - Extracting shared functionality

## Benefits

### 1. Maintainability

- **Clear boundaries** - Each island has a single responsibility
- **Easy to locate** - Components are organized by feature
- **Isolated changes** - Modifying one island doesn't affect others

### 2. Performance

- **Code splitting** - Islands can be lazy-loaded if needed
- **Parallel loading** - Islands load independently
- **Optimized bundles** - Only load JavaScript for interactive islands

### 3. Developer Experience

- **Easier to understand** - Simple shell pages are easy to read
- **Better organization** - Related components grouped together
- **Reusability** - Islands can be reused across pages

### 4. Testing

- **Isolated testing** - Test each island independently
- **Mock-friendly** - Easy to mock dependencies
- **Clear test boundaries** - Each island has clear test scope

## Example: Settings Screen

The Settings screen demonstrates Islands Architecture:

```
src/routes/settings/+page.svelte          # Simple shell (22 lines)
├── AccountSettings.svelte                # Island: Account info
├── AudioSettings.svelte                  # Container island
│   ├── MicrophoneSettings.svelte        # Sub-island
│   ├── SpeakerSettings.svelte           # Sub-island
│   ├── RingtoneSettings.svelte          # Sub-island
│   └── AudioProcessingSettings.svelte   # Sub-island
├── SIPAccountSettings.svelte            # Island: SIP config
├── GeneralSettings.svelte               # Island: App preferences
└── AboutSettings.svelte                  # Island: App info (static)
```

**Before refactoring:** 545-line monolithic component  
**After refactoring:** Simple 22-line shell + 9 focused island components

## Anti-Patterns to Avoid

### ❌ Monolithic Pages

Don't put all logic and state in a single page component:

```svelte
<!-- ❌ Bad: Everything in one file -->
<script lang="ts">
  // 500+ lines of state and logic
</script>
```

### ❌ Prop Drilling

Don't pass state through many component layers:

```svelte
<!-- ❌ Bad: State passed through multiple layers -->
<Page user={user}>
  <Section user={user}>
    <SubSection user={user}>
      <Component user={user} />
    </SubSection>
  </Section>
</Page>
```

### ❌ Shared State Without Stores

Don't use complex parent-child state synchronization:

```svelte
<!-- ❌ Bad: Complex state synchronization -->
<script lang="ts">
  // Parent manages state, children sync via props and callbacks
</script>
```

## Migration Strategy

When refactoring existing screens to use Islands Architecture:

1. **Identify logical sections** - Break screen into feature areas
2. **Extract islands** - Move each section to its own component
3. **Make state self-contained** - Move state into each island
4. **Simplify the page** - Make page a simple shell
5. **Test independently** - Verify each island works in isolation

## References

- [Astro Islands Architecture](https://docs.astro.build/en/concepts/islands/) - Original pattern documentation
- [01-layers.md](01-layers.md) - Presentation Layer architecture
- [08-ui-design.md](08-ui-design.md) - UI design specifications

---

**Status:** This pattern is the standard for all new screens and components in RUSTALK.  
**Last Updated:** 2025-01-XX
