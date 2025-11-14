# Svelte 5 Runes Mode - Development Patterns & Best Practices

**Last Updated**: 2025-11-14  
**Based on**: Phase 2 Implementation Learnings + Contacts Page Refactoring  
**Svelte Version**: 5.x (Runes Mode)

This document captures critical patterns and common pitfalls discovered during Phase 2 implementation to ensure smooth development in future phases.

---

## Table of Contents

1. [Reactive Statements](#reactive-statements)
2. [Side Effects with $effect](#side-effects-with-effect)
3. [Event Handlers](#event-handlers)
4. [Component Props & TypeScript](#component-props--typescript)
5. [Slots vs Children Snippets](#slots-vs-children-snippets)
6. [CSS Scoping with Props](#css-scoping-with-props)
7. [Store Subscriptions](#store-subscriptions)
8. [Dialog and State Management Patterns](#dialog-and-state-management-patterns)
9. [Common Pitfalls](#common-pitfalls)
10. [Advanced Patterns](#advanced-patterns)

---

## Reactive Statements

### ❌ **DO NOT USE** - Legacy Reactive Syntax

```svelte
<script lang="ts">
  let name = 'John';

  // ❌ This will cause runtime error in runes mode
  $: initials = name.split(' ').map(n => n[0]).join('');
  $: classes = ['btn', `btn-${variant}`].join(' ');
</script>
```

### ✅ **USE** - Runes Mode Syntax

```svelte
<script lang="ts">
  let name = $state('John');

  // ✅ Use $derived for computed values
  let initials = $derived(
    name.split(' ').map(n => n[0]).join('')
  );

  let classes = $derived(
    ['btn', `btn-${variant}`].filter(Boolean).join(' ')
  );
</script>
```

### Key Rules

- **Always use `$derived`** instead of `$:` for computed/reactive values
- **Use `$state`** for mutable state variables
- **Use `$effect`** for side effects (replaces `$:` for side effects)
- Legacy `$:` syntax will cause **runtime errors** in Svelte 5 runes mode

---

## Side Effects with $effect

### ❌ **DO NOT USE** - Legacy Reactive Side Effects

```svelte
<script lang="ts">
  let name = $state('John');

  // ❌ This will cause runtime error in runes mode
  $: {
    console.log('Name changed:', name);
    // Side effect code
  }
</script>
```

### ✅ **USE** - $effect for Side Effects

```svelte
<script lang="ts">
  let name = $state('John');

  // ✅ Use $effect for side effects
  $effect(() => {
    console.log('Name changed:', name);
    // Any side effect code (API calls, DOM manipulation, etc.)
  });
</script>
```

### Cleanup Functions

`$effect` can return a cleanup function that runs when the effect is re-run or the component is destroyed:

```svelte
<script lang="ts">
  let isVisible = $state(true);

  $effect(() => {
    if (!isVisible) return;

    // Setup code
    const interval = setInterval(() => {
      console.log('Tick');
    }, 1000);

    // ✅ Return cleanup function
    return () => {
      clearInterval(interval);
    };
  });
</script>
```

### When to Use $effect vs $derived

- **Use `$derived`** for computed values that depend on other reactive values
- **Use `$effect`** for side effects (console.log, API calls, DOM manipulation, subscriptions)

```svelte
<script lang="ts">
  let count = $state(0);
  let name = $state('John');

  // ✅ $derived for computed values
  let doubled = $derived(count * 2);
  let greeting = $derived(`Hello, ${name}!`);

  // ✅ $effect for side effects
  $effect(() => {
    console.log('Count changed:', count);
  });

  $effect(() => {
    document.title = `Count: ${count}`;
  });
</script>
```

### $effect Variants

#### $effect.pre()

Runs synchronously before the DOM updates:

```svelte
<script lang="ts">
  let count = $state(0);

  // ✅ Runs before DOM updates
  $effect.pre(() => {
    if (count > 10) {
      count = 10; // Prevent exceeding limit
    }
  });
</script>
```

#### $effect.untrack()

Runs without tracking dependencies (useful for one-time setup):

```svelte
<script lang="ts">
  let count = $state(0);

  // ✅ Runs once, doesn't track count
  $effect.untrack(() => {
    console.log('Component mounted');
    // Setup code that shouldn't re-run
  });
</script>
```

---

## Event Handlers

### ✅ **USE** - onclick Attribute (Svelte 5)

In Svelte 5, use the `onclick` attribute (not `on:click`) for event handlers. This works for both Svelte components and native HTML elements.

```svelte
<!-- ✅ Use onclick for Svelte 5 event handlers -->
<Button onclick={handleClick}>Click me</Button>
<Card onclick={() => doSomething()}>Content</Card>

<!-- ✅ For native HTML elements, use onclick -->
<button onclick={handleClick}>Native Button</button>
<div onclick={handleClick}>Clickable div</div>
```

### ❌ **DO NOT USE** - Legacy on:click Syntax

```svelte
<!-- ❌ Legacy Svelte 4 syntax - deprecated in Svelte 5 -->
<Button on:click={handleClick}>Click me</Button>
<Card on:click={() => doSomething()}>Content</Card>
```

**Note**: The `on:click` syntax from Svelte 4 is deprecated in Svelte 5. Always use `onclick` instead. See the [Svelte 5 Migration Guide](https://svelte.dev/docs/svelte/v5-migration-guide) for details.

### Event Handler Forwarding

When creating reusable components that need to forward events:

```svelte
<script lang="ts">
  // ✅ Allow event handlers via Record<string, any>
  type ButtonProps = {
    variant?: "primary" | "secondary";
    disabled?: boolean;
  } & Record<string, any>;  // This allows onclick, onfocus, etc.

  let {
    variant = "primary",
    disabled = false,
    ...restProps  // Event handlers will be in restProps
  }: ButtonProps = $props();
</script>

<!-- ✅ Spread restProps to forward events -->
<button type="button" {...restProps}>
  <slot />
</button>
```

---

## Component Props & TypeScript

### Event Handler Type Safety

To properly type components that accept event handlers:

```svelte
<script lang="ts">
  // ✅ Method 1: Use Record<string, any> (simpler, less type-safe)
  type ComponentProps = {
    variant?: string;
    class?: string;
  } & Record<string, any>;

  // ✅ Method 2: Explicitly allow event handlers (more type-safe)
  interface ComponentProps {
    variant?: string;
    class?: string;
    [key: `on:${string}`]: ((event: any) => void) | undefined;
  }

  // ✅ Method 3: Type-safe event handlers (recommended)
  type ButtonEvents = {
    click: MouseEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  };

  type ButtonProps = {
    variant?: "primary" | "secondary";
    disabled?: boolean;
    class?: string;
  } & {
    [K in keyof ButtonEvents as `on:${K}`]?: (event: ButtonEvents[K]) => void;
  };

  let {
    variant = "default",
    disabled = false,
    ...restProps
  }: ButtonProps = $props();
</script>
```

**Recommendation**: Use Method 3 for better type safety, especially in larger codebases where event types matter.

### Bindable Props

`$bindable` creates two-way binding between parent and child components. The parent **must** use `bind:` syntax.

```svelte
<script lang="ts">
  interface InputProps {
    value?: string;
    placeholder?: string;
  }

  let {
    value = $bindable(''),  // ✅ Use $bindable for two-way binding
    placeholder = '',
    ...restProps
  }: InputProps = $props();
</script>

<input bind:value {placeholder} {...restProps} />
```

#### Parent Component Usage

```svelte
<!-- Parent.svelte -->
<script lang="ts">
  import Input from './Input.svelte';

  let inputValue = $state(''); // ✅ Parent state
</script>

<!-- ✅ Must use bind:value, not just value -->
<Input bind:value={inputValue} placeholder="Enter text" />

<!-- ❌ This won't work - no two-way binding -->
<Input value={inputValue} placeholder="Enter text" />
```

#### Type Constraints

`$bindable` works with any type, but the parent and child must use compatible types:

```svelte
<script lang="ts">
  interface CounterProps {
    count?: number;
  }

  let {
    count = $bindable(0), // ✅ Works with numbers
  }: CounterProps = $props();
</script>

<!-- Parent -->
<script lang="ts">
  let count = $state(0);
</script>
<Counter bind:count={count} />
```

#### When to Use $bindable

- Use `$bindable` when the child component needs to update the parent's state
- Use regular props when data flows only from parent to child
- Common use cases: form inputs, toggles, counters, any component that modifies parent state

---

## Slots vs Children Snippets

### ❌ **DO NOT USE** - Legacy Slot Syntax (Deprecated)

```svelte
<script lang="ts">
  // Component definition
</script>

<!-- ❌ Deprecated in Svelte 5 -->
<button>
  <slot />
</button>
```

### ✅ **USE** - Children Snippets (Svelte 5)

```svelte
<script lang="ts">
  interface ButtonProps {
    children?: import("svelte").Snippet;
    // ... other props
  }

  let {
    children,
    ...restProps
  }: ButtonProps = $props();
</script>

<!-- ✅ Use {@render children()} for Svelte 5 -->
<!-- ⚠️ {#if children} check is REQUIRED - {@render} throws if undefined -->
<button {...restProps}>
  {#if children}
    {@render children()}
  {/if}
</button>
```

### Named Snippets

Components can accept multiple named snippets:

```svelte
<script lang="ts">
  interface CardProps {
    children?: import("svelte").Snippet;
    header?: import("svelte").Snippet;
    footer?: import("svelte").Snippet;
  }

  let {
    children,
    header,
    footer,
    ...restProps
  }: CardProps = $props();
</script>

<div {...restProps}>
  {#if header}
    <header>{@render header()}</header>
  {/if}

  {#if children}
    <main>{@render children()}</main>
  {/if}

  {#if footer}
    <footer>{@render footer()}</footer>
  {/if}
</div>
```

### Snippet Props

Snippets can accept props:

```svelte
<script lang="ts">
  interface ButtonProps {
    children?: import("svelte").Snippet;
    icon?: import("svelte").Snippet<[props: { size: number }]>;
  }

  let {
    children,
    icon,
    ...restProps
  }: ButtonProps = $props();
</script>

<button {...restProps}>
  {#if icon}
    {@render icon({ size: 16 })}
  {/if}
  {#if children}
    {@render children()}
  {/if}
</button>
```

### Usage Patterns

```svelte
<!-- ✅ Default children -->
<Button variant="primary">
  Click me
</Button>

<!-- ✅ Named snippets -->
<Card>
  <snippet:header>Card Title</snippet:header>
  <snippet:default>Card content</snippet:default>
  <snippet:footer>Card footer</snippet:footer>
</Card>

<!-- ✅ Snippet with props -->
<Button>
  <snippet:icon let:size>
    <Icon size={size} />
  </snippet:icon>
  <snippet:default>Click me</snippet:default>
</Button>

<!-- ✅ In tests -->
render(Button, { variant: 'primary' }, { default: () => 'Click me' });
```

---

## CSS Scoping with Props

### Problem: CSS Classes Passed as Props

When passing CSS class names as props, Svelte's scoped CSS won't apply to them. However, **static classes defined in the component template** will work with scoped CSS.

### ❌ **DO NOT USE** - Scoped CSS with Prop Classes

```svelte
<script lang="ts">
  let { class: className = '' } = $props();
</script>

<button class="btn {className}">
  <!-- className might be "control-button" but CSS won't apply -->
</button>

<style>
  /* ❌ This won't work if className is passed as prop */
  .control-button {
    width: 100%;
  }
</style>
```

### ✅ **USE** - Global CSS for Prop Classes

```svelte
<script lang="ts">
  let { class: className = '' } = $props();
</script>

<button class="btn {className}">
  Content
</button>

<style>
  .btn {
    /* ✅ Scoped styles work for static classes */
  }

  /* ✅ Use :global() for classes passed as props */
  :global(.control-button) {
    width: 100%;
  }

  /* ✅ Or use :global() with parent selector */
  .controls-row :global(.control-button) {
    width: 100%;
  }
</style>
```

### Dynamic Class Binding with `class:` Directive

The `class:` directive works with scoped CSS when the class name is static:

```svelte
<script lang="ts">
  let { active = false, disabled = false } = $props();
  let isActive = $state(false);
</script>

<!-- ✅ class: directive with static class names -->
<button class:active={isActive} class:disabled={disabled}>
  Click me
</button>

<!-- ✅ class: directive with props (shorthand) -->
<button class:active>
  Content
</button>

<style>
  /* ✅ These work with class: directive */
  .active {
    background-color: blue;
  }

  .disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
```

### Dynamic Class Binding Patterns

```svelte
<script lang="ts">
  let { variant = 'primary' } = $props();
  let isActive = $state(false);

  // ✅ Use $derived for complex class logic
  let classes = $derived(
    [
      'btn',
      `btn-${variant}`,
      isActive && 'btn-active'
    ].filter(Boolean).join(' ')
  );
</script>

<!-- Method 1: $derived computed classes -->
<button class={classes}>Click me</button>

<!-- Method 2: class: directive -->
<button class="btn" class:btn-active={isActive} class:btn-primary={variant === 'primary'}>
  Click me
</button>

<style>
  .btn {
    /* Scoped styles */
  }

  /* ✅ Static variant classes work with scoped CSS */
  .btn-primary {
    background: blue;
  }

  /* ❌ Dynamic classes from props need :global() */
  :global(.btn-custom) {
    background: green;
  }
</style>
```

---

## Store Subscriptions

### ⚠️ **CRITICAL** - Memory Leak Prevention

Manual store subscriptions **must** include cleanup to prevent memory leaks. Always use `$effect` with cleanup for legacy stores.

### ❌ **DO NOT USE** - Manual Subscription Without Cleanup

```svelte
<script lang="ts">
  import { callStore } from '$lib/stores/callStore';

  // ❌ Memory leak - subscription never cleaned up
  let call = $state($callStore.activeCall);

  callStore.subscribe((store) => {
    call = store.activeCall;
  });
</script>
```

### ✅ **USE** - Legacy Stores with $effect Cleanup

For Svelte 4 style stores (writable, readable, derived), use `$effect` with cleanup:

```svelte
<script lang="ts">
  import { callStore } from '$lib/stores/callStore';

  // ✅ Initialize with current store value
  let call = $state($callStore.activeCall);
  let callState = $state($callStore.currentState);

  // ✅ Subscribe with cleanup in $effect
  $effect(() => {
    const unsubscribe = callStore.subscribe((store) => {
      call = store.activeCall;
      callState = store.currentState;
    });

    // ✅ Return cleanup function
    return unsubscribe;
  });
</script>
```

### ✅ **USE** - Svelte 5 Rune-Based Stores

For Svelte 5 stores created with runes, use `$derived` with the `$` prefix for auto-subscription:

```svelte
<script lang="ts">
  import { callStore } from '$lib/stores/callStore';

  // ✅ Auto-subscribe with $derived (if store is rune-based)
  let call = $derived($callStore.activeCall);
  let callState = $derived($callStore.currentState);

  // No manual subscription needed - Svelte handles it automatically
</script>
```

### Derived Stores

```svelte
<script lang="ts">
  import { filteredContacts } from '$lib/stores/contactsStore';

  // ✅ Option 1: Legacy store with cleanup
  let filtered = $state($filteredContacts);

  $effect(() => {
    const unsubscribe = filteredContacts.subscribe((v) => {
      filtered = v;
    });
    return unsubscribe;
  });

  // ✅ Option 2: Rune-based store (auto-subscribe)
  let filtered = $derived($filteredContacts);
</script>
```

### Stores That Return Derived Stores

**⚠️ CRITICAL** - When a store has methods that return Readable stores (like `contactsStore.getFavorites()`), you cannot use the `$` prefix directly on the method call.

#### ❌ **DO NOT USE** - Incorrect Pattern

```svelte
<script lang="ts">
  import { contactsStore } from '$lib/stores/contactsStore';

  // ❌ ERROR: $contactsStore().getFavorites is not a function
  // The $ prefix tries to get the value of contactsStore, but getFavorites is a method
  let favorites = $derived($contactsStore.getFavorites());
</script>
```

#### ✅ **USE** - Correct Pattern with $effect

When a store method returns a Readable store, subscribe to it in `$effect`:

```svelte
<script lang="ts">
  import { contactsStore } from '$lib/stores/contactsStore';
  import type { Contact } from '$lib/stores/contactsStore';

  // ✅ Create local state
  let favorites = $state<Contact[]>([]);

  // ✅ Subscribe to the store returned by getFavorites()
  $effect(() => {
    const store = contactsStore.getFavorites(); // Returns a Readable<Contact[]>
    const unsubscribe = store.subscribe((value) => {
      favorites = value;
    });
    return unsubscribe; // Cleanup subscription
  });
</script>

{#if favorites.length > 0}
  {#each favorites as contact (contact.id)}
    <!-- Render contact -->
  {/each}
{/if}
```

#### ✅ **USE** - Alternative: Direct Store Usage (if method returns store directly)

If the method returns a store that you can reference directly:

```svelte
<script lang="ts">
  import { favoriteContacts } from '$lib/stores/contactsStore';

  // ✅ If favoriteContacts is exported directly as a store
  // You can use $ prefix for auto-subscription
  let favorites = $derived($favoriteContacts);
</script>
```

#### Pattern Summary

- **Store methods that return stores**: Use `$effect` with subscription and cleanup
- **Directly exported stores**: Can use `$derived($store)` for auto-subscription
- **Always cleanup**: Return unsubscribe function from `$effect` to prevent memory leaks

### Store Type Detection

- **Legacy stores**: Use `writable()`, `readable()`, `derived()` from `svelte/store` → Use `$effect` with cleanup
- **Rune-based stores**: Created with `$state` or `$derived` → Use `$derived($store.value)` for auto-subscription
- **Store methods returning stores**: Methods like `getFavorites()`, `getAllContacts()` → Use `$effect` with subscription

---

## Dialog and State Management Patterns

**Last Updated**: 2025-11-14  
**Status**: Active Standard

This section documents the established patterns for managing dialog state and component-level state management in RUSTALK. These patterns ensure components are self-contained, maintainable, and follow the Islands Architecture principles.

### Core Principles

1. **Self-Contained Dialogs**: Dialogs manage their own visibility state internally
2. **Component-Managed State**: Components handle their own state, not parent components
3. **Shared Utilities**: Common functionality (like navigation) is extracted to composables
4. **Minimal Page State**: Pages should be simple shells that compose components

---

### Dialog Pattern: DialogTrigger with Self-Contained State

Dialogs should use the DialogTrigger pattern and manage their own open state internally.

**✅ Correct Pattern:**

```svelte
<!-- AddContactDialog.svelte -->
<script lang="ts">
  import { Dialog, DialogContent, DialogTrigger } from "$lib/components/ui/dialog";
  import type { Snippet } from "svelte";

  interface Props {
    trigger?: Snippet;
  }

  let { trigger }: Props = $props();

  // Manage dialog open state internally
  let open = $state(false);
</script>

<Dialog bind:open>
  {#if trigger}
    <DialogTrigger>
      {@render trigger()}
    </DialogTrigger>
  {:else}
    <DialogTrigger>
      <Button>Default Trigger</Button>
    </DialogTrigger>
  {/if}
  <DialogContent>
    <!-- Dialog content -->
  </DialogContent>
</Dialog>
```

**Usage:**

```svelte
<!-- In page component -->
<AddContactDialog>
  {#snippet trigger()}
    <Button>
      <Plus class="h-4 w-4 mr-2" />
      Add Contact
    </Button>
  {/snippet}
</AddContactDialog>
```

**❌ Avoid:**

```svelte
<!-- ❌ Wrong - parent manages dialog state -->
<script lang="ts">
  let showDialog = $state(false);
</script>

<Button onclick={() => showDialog = true}>Add Contact</Button>
<AddContactDialog bind:open={showDialog} />
```

---

### Dialog Pattern: Self-Contained with open() Method

For dialogs that need to be opened programmatically (e.g., from another component), expose an `open()` method.

**✅ Correct Pattern:**

```svelte
<!-- EditContactDialog.svelte -->
<script lang="ts">
  interface Props {
    contact: Contact;
    onOpenChange?: (open: boolean) => void;
  }

  let { contact, onOpenChange }: Props = $props();

  // Manage dialog open state internally
  let open = $state(false);

  // Expose open method for programmatic opening
  export function openDialog() {
    open = true;
  }

  function handleOpenChange(newOpen: boolean) {
    open = newOpen;
    onOpenChange?.(newOpen);
  }
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogContent
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Dialog content -->
    <DialogFooter>
      <Button
        onclick={(e) => {
          e.stopPropagation();
          handleCancel();
        }}
      >
        Cancel
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

**Usage with bind:this:**

```svelte
<!-- In parent component -->
<script lang="ts">
  let editDialogRef: EditContactDialog | null = null;

  function handleEdit() {
    editDialogRef?.openDialog();
  }
</script>

<Button onclick={handleEdit}>Edit</Button>
<EditContactDialog bind:this={editDialogRef} contact={contact} />
```

**Important Notes:**

- Always use `stopPropagation()` on DialogContent and button handlers when dialogs are nested inside other modals
- Use `onOpenChange` callback to notify parent when dialog state changes (useful for preventing parent modal from closing)
- Pass data objects directly when parent already has them (avoids unnecessary fetching delays)

---

### Component State Management: Component-Managed Search

Search state should be managed by the search component itself, shared via stores when needed.

**✅ Correct Pattern:**

```svelte
<!-- ContactSearchBar.svelte -->
<script lang="ts">
  import { contactsStore } from "$lib/stores/contactsStore";

  // Manage search query state internally
  let searchQuery = $state("");

  // Sync with store for sharing with other components
  $effect(() => {
    const unsubscribe = contactsStore.searchQuery.subscribe((value) => {
      searchQuery = value;
    });
    return unsubscribe;
  });

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    searchQuery = target.value;
    // Update store (debounced in real implementation)
    contactsStore.searchQuery.set(searchQuery);
  }
</script>

<Input value={searchQuery} oninput={handleInput} />
```

**Store Implementation:**

```typescript
// contactsStore.ts
const { subscribe: subscribeSearchQuery, set: setSearchQuery } =
  writable<string>("");

export const searchQueryStore = {
  subscribe: subscribeSearchQuery,
  set: setSearchQuery,
};

export const contactsStore = {
  // ... other methods
  searchQuery: searchQueryStore,
};
```

**❌ Avoid:**

```svelte
<!-- ❌ Wrong - parent manages search state -->
<script lang="ts">
  let searchQuery = $state("");
  function handleSearch(query: string) {
    searchQuery = query;
  }
</script>

<ContactSearchBar onSearch={handleSearch} />
<ContactList {searchQuery} />
```

---

### Shared Functionality: Composable Pattern

Common functionality that's used across multiple components should be extracted to composables in `src/lib/hooks/`.

**✅ Correct Pattern:**

```typescript
// src/lib/hooks/useCallNavigation.ts
import { goto } from "$app/navigation";

/**
 * Composable for handling call navigation
 *
 * @returns A function that navigates to the dialer with the given number
 */
export function useCallNavigation() {
  function initiateCall(number: string) {
    console.log("DEBUG:[CALL/NAVIGATION] Initiating call to:", number);
    goto(`/?number=${encodeURIComponent(number)}`);
  }

  return {
    initiateCall,
  };
}
```

**Usage:**

```svelte
<script lang="ts">
  import { useCallNavigation } from "$lib/hooks/useCallNavigation";

  const { initiateCall } = useCallNavigation();

  function handleCall(number: string) {
    initiateCall(number);
  }
</script>

<Button onclick={() => handleCall(phoneNumber)}>Call</Button>
```

**Benefits:**

- Reusable across components
- Easy to extend for future functionality (e.g., real call handling)
- Keeps components focused on their primary responsibility

---

### Page Component Pattern: Simple Shell

Pages should be minimal shells that compose components. They should not manage dialog states or component-level state.

**✅ Correct Pattern:**

```svelte
<!-- +page.svelte -->
<script lang="ts">
  import ContactSearchBar from "$lib/components/contacts/ContactSearchBar.svelte";
  import ContactList from "$lib/components/contacts/ContactList.svelte";
  import AddContactDialog from "$lib/components/contacts/AddContactDialog.svelte";
</script>

<div class="flex flex-col h-full">
  <div class="p-6">
    <AddContactDialog>
      {#snippet trigger()}
        <Button>Add Contact</Button>
      {/snippet}
    </AddContactDialog>
  </div>

  <div class="flex-1 p-6">
    <ContactSearchBar />
    <ContactList />
  </div>
</div>
```

**❌ Avoid:**

```svelte
<!-- ❌ Wrong - page manages too much state -->
<script lang="ts">
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let selectedContact = $state<Contact | null>(null);
  let searchQuery = $state("");

  function handleAddContact() {
    showAddDialog = true;
  }
  // ... many more handlers
</script>
```

---

### Nested Dialog Pattern: Event Propagation

When dialogs are nested (e.g., EditDialog inside ContactDetails modal), prevent event propagation to avoid conflicts.

**✅ Correct Pattern:**

```svelte
<!-- EditContactDialog inside ContactDetails modal -->
<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogContent
    class="max-w-md"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Content -->
    <DialogFooter>
      <Button
        onclick={(e) => {
          e.stopPropagation();
          handleCancel();
        }}
      >
        Cancel
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

**Parent Modal Protection:**

```svelte
<!-- ContactDetails.svelte -->
<script lang="ts">
  let isEditDialogOpen = $state(false);

  function handleClose() {
    // Don't close if edit dialog is open
    if (isEditDialogOpen) {
      return;
    }
    open = false;
  }

  function handleEditDialogOpenChange(open: boolean) {
    isEditDialogOpen = open;
  }
</script>

{#if open}
  <div onclick={handleClose}>
    <!-- Content -->
    <EditContactDialog
      onOpenChange={handleEditDialogOpenChange}
    />
  </div>
{/if}
```

---

### Data Passing Pattern: Direct Props vs ID

**Use direct props** when parent already has the data (faster, no loading delay):

```svelte
<!-- ✅ Fast - no fetch needed -->
<EditContactDialog contact={contactData} />
```

**Use ID** when component should be fully self-contained and fetch its own data:

```svelte
<!-- ✅ Self-contained - fetches own data -->
<ContactDetails contactId={contact.id} />
```

**Decision Criteria:**

- If parent already has the data → Pass directly (better UX, instant)
- If component should be reusable without parent data → Use ID (more flexible)

---

### Summary Checklist

When creating dialogs or managing component state:

- [ ] Dialog manages its own `open` state internally
- [ ] Use DialogTrigger pattern for user-triggered dialogs
- [ ] Expose `open()` method for programmatically-triggered dialogs
- [ ] Use `stopPropagation()` for nested dialogs
- [ ] Use `onOpenChange` callback to track dialog state in parent
- [ ] Component-managed state (search, filters) uses stores for sharing
- [ ] Shared functionality extracted to composables in `hooks/`
- [ ] Page components are simple shells with minimal state
- [ ] Pass data directly when parent has it (avoid unnecessary fetching)

---

## Common Pitfalls

### 1. Mixing Legacy and Runes Syntax

**Problem**: Using `$:` in runes mode causes runtime errors.

**Solution**: Always use `$derived` for computed values.

```svelte
// ❌ Wrong
$: computed = value * 2;

// ✅ Correct
let computed = $derived(value * 2);
```

### 2. Store Methods That Return Stores

**Problem**: Trying to use `$` prefix on store method calls causes runtime errors.

**Error**: `TypeError: $contactsStore().getFavorites is not a function`

**Solution**: Use `$effect` to subscribe to stores returned by methods.

```svelte
// ❌ Wrong - causes runtime error
let favorites = $derived($contactsStore.getFavorites());

// ✅ Correct - subscribe in $effect
let favorites = $state<Contact[]>([]);
$effect(() => {
  const store = contactsStore.getFavorites();
  const unsubscribe = store.subscribe((value) => {
    favorites = value;
  });
  return unsubscribe;
});
```

See [Stores That Return Derived Stores](#stores-that-return-derived-stores) section for detailed examples.

### 3. Dialog Accessibility

**Problem**: Dialogs/modals without keyboard support trigger lint warnings and accessibility issues.

**Solution**: Always add `tabindex` and keyboard event handlers for interactive dialogs.

```svelte
<!-- ❌ Missing accessibility attributes -->
<div
  role="dialog"
  aria-modal="true"
  onclick={handleClose}
>
  <!-- Content -->
</div>

<!-- ✅ Correct - includes keyboard support -->
<div
  role="dialog"
  aria-modal="true"
  tabindex="-1"
  onclick={handleClose}
  onkeydown={(e) => {
    if (e.key === "Escape") {
      handleClose();
    }
  }}
>
  <!-- Content -->
</div>
```

### 4. Event Handler Type Errors

**Problem**: TypeScript errors when passing `onclick` to components.

**Solution**: Add `Record<string, any>` or explicit event handler types to component props.

```svelte
// ❌ Wrong
interface Props {
  variant?: string;
  // Missing event handler support
}

// ✅ Correct
type Props = {
  variant?: string;
} & Record<string, any>;
```

### 5. CSS Not Applying to Prop Classes

**Problem**: CSS classes passed as props don't get styled.

**Solution**: Use `:global()` for classes that come from props.

```svelte
<style>
  /* ✅ Use :global() */
  :global(.prop-class) {
    /* styles */
  }
</style>
```

### 6. Slot Deprecation Warnings

**Problem**: Using `<slot />` shows deprecation warnings.

**Solution**: Use children snippets with `{@render children()}`.

```svelte
<!-- ❌ Deprecated -->
<slot />

<!-- ✅ Correct -->
{#if children}
  {@render children()}
{/if}
```

### 7. Reactive Statement in Runes Mode

**Problem**: `$:` statements cause "not allowed in runes mode" errors.

**Solution**: Convert all `$:` to `$derived` or `$effect`.

```svelte
// ❌ Wrong - causes runtime error
$: currentPath = $page.url.pathname;

// ✅ Correct
let currentPath = $derived($page.url.pathname);
```

---

## Component Template

Here's a complete template for creating Svelte 5 components with best practices:

```svelte
<script lang="ts">
  // ✅ Define props with type-safe event handlers
  type ButtonEvents = {
    click: MouseEvent;
    focus: FocusEvent;
  };

  type ComponentProps = {
    variant?: "primary" | "secondary";
    disabled?: boolean;
    class?: string;
    children?: import("svelte").Snippet;
  } & {
    [K in keyof ButtonEvents as `on:${K}`]?: (event: ButtonEvents[K]) => void;
  };

  let {
    variant = "primary",
    disabled = false,
    class: className = "",
    children,
    ...restProps
  }: ComponentProps = $props();

  // ✅ Use $derived for computed values
  let classes = $derived(
    ["component", `component-${variant}`, className]
      .filter(Boolean)
      .join(" ")
  );

  // ✅ Error handling example
  let error = $state<string | null>(null);

  function handleAction() {
    try {
      // Component logic
    } catch (e) {
      error = e instanceof Error ? e.message : 'Unknown error';
    }
  }

  // ✅ $effect cleanup example
  $effect(() => {
    if (disabled) return;

    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.key === 'Enter') {
        handleAction();
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => {
      window.removeEventListener('keydown', handleKeyPress);
    };
  });
</script>

<!-- ✅ Use children snippets with accessibility -->
<button
  class={classes}
  disabled={disabled}
  aria-label={variant === 'primary' ? 'Primary action' : 'Secondary action'}
  aria-disabled={disabled}
  {...restProps}
>
  {#if children}
    {@render children()}
  {/if}
</button>

{#if error}
  <div role="alert" class="error">
    {error}
  </div>
{/if}

<style>
  .component {
    /* Scoped styles */
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }

  .component-primary {
    background-color: blue;
    color: white;
  }

  .component-secondary {
    background-color: gray;
    color: black;
  }

  .component:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error {
    color: red;
    margin-top: 0.5rem;
  }

  /* ✅ Use :global() for prop classes */
  :global(.custom-class) {
    /* Global styles for prop classes */
  }
</style>
```

### Key Features in Template

- ✅ Type-safe event handlers
- ✅ Error handling pattern
- ✅ Accessibility attributes (aria-label, aria-disabled)
- ✅ $effect with cleanup for event listeners
- ✅ Proper disabled state handling
- ✅ Scoped CSS with variant classes

---

## Testing Patterns

### Component Testing with Children

```typescript
import { render, screen } from "@testing-library/svelte";
import Button from "./Button.svelte";

// ✅ Use children snippet in tests
it("renders with children", () => {
  render(Button, {}, { default: () => "Click me" });
  const button = screen.getByRole("button", { name: "Click me" });
  expect(button).toBeInTheDocument();
});
```

### Event Handler Testing

```typescript
import userEvent from '@testing-library/user-event';

it('handles click events', async () => {
  const user = userEvent.setup();
  let clicked = false;
  const handleClick = () => { clicked = true; };

  render(Button, { onclick: handleClick }, { default: () => 'Click' });
  const button = screen.getByRole('button');
  await user.click(button);
  expect(clicked).toBe(true);
});
```

### Testing $derived Values

```typescript
import { render } from "@testing-library/svelte";
import { get } from "svelte/store";
import Component from "./Component.svelte";

it("computes derived values correctly", () => {
  const { component } = render(Component, { count: 5 });

  // Access component instance to check $derived values
  expect(component.doubled).toBe(10);
});
```

### Testing $effect Side Effects

```typescript
import { render, waitFor } from "@testing-library/svelte";
import { vi } from "vitest";
import Component from "./Component.svelte";

it("runs effect when state changes", async () => {
  const consoleSpy = vi.spyOn(console, "log");
  const { component } = render(Component, { name: "John" });

  // Update state
  component.name = "Jane";

  await waitFor(() => {
    expect(consoleSpy).toHaveBeenCalledWith("Name changed:", "Jane");
  });

  consoleSpy.mockRestore();
});
```

### Testing $bindable Props

```typescript
import { render, fireEvent } from "@testing-library/svelte";
import Input from "./Input.svelte";

it("updates parent state via bindable prop", async () => {
  let value = "";
  const { component } = render(Input, {
    value: $bindable(value),
  });

  const input = screen.getByRole("textbox");
  await fireEvent.input(input, { target: { value: "test" } });

  expect(value).toBe("test");
});
```

### Mocking Stores in Tests

```typescript
import { render, screen } from "@testing-library/svelte";
import { writable } from "svelte/store";
import Component from "./Component.svelte";
import * as callStoreModule from "$lib/stores/callStore";

it("renders with mocked store", () => {
  const mockStore = writable({ activeCall: { id: "123" } });
  vi.spyOn(callStoreModule, "callStore", "get").mockReturnValue(mockStore);

  render(Component);
  expect(screen.getByText("Call ID: 123")).toBeInTheDocument();
});
```

### Testing Event Forwarding

```typescript
import { render, fireEvent } from "@testing-library/svelte";
import Button from "./Button.svelte";

it('forwards events correctly', async () => {
  const handleClick = vi.fn();
  render(Button, { onclick: handleClick }, { default: () => 'Click' });

  const button = screen.getByRole('button');
  await fireEvent.click(button);

  expect(handleClick).toHaveBeenCalledTimes(1);
});
```

---

## Advanced Patterns

### $props() Destructuring Gotchas

When destructuring `$props()`, be aware of these patterns:

```svelte
<script lang="ts">
  // ✅ Default values work as expected
  let { name = 'Default', age = 0 } = $props();

  // ⚠️ Rest props must come last
  let { name, ...rest } = $props(); // ✅ Correct
  // let { ...rest, name } = $props(); // ❌ Syntax error

  // ✅ Can destructure nested props
  interface Config {
    theme: { primary: string; secondary: string };
  }
  let { theme: { primary, secondary } } = $props<Config>();
</script>
```

### $derived.by() for Complex Computations

For complex derived values that need multiple statements, use `$derived.by()`:

```svelte
<script lang="ts">
  let items = $state([1, 2, 3, 4, 5]);
  let filter = $state('');

  // ✅ Use $derived.by() for complex logic
  let filteredAndSorted = $derived.by(() => {
    let filtered = items.filter(item => item.toString().includes(filter));
    let sorted = filtered.sort((a, b) => a - b);
    return sorted.map(item => item * 2);
  });
</script>
```

**Note**: `$derived.by()` is useful when you need multiple statements or intermediate variables in your computation.

### $effect Variants Summary

- **`$effect()`**: Runs after DOM updates, tracks all dependencies
- **`$effect.pre()`**: Runs synchronously before DOM updates
- **`$effect.untrack()`**: Runs without tracking dependencies (one-time setup)

```svelte
<script lang="ts">
  let count = $state(0);

  // Standard effect - runs after updates
  $effect(() => {
    console.log('Count:', count);
  });

  // Pre-effect - runs before updates (can modify state)
  $effect.pre(() => {
    if (count < 0) count = 0; // Clamp value
  });

  // Untracked - runs once, doesn't track dependencies
  $effect.untrack(() => {
    console.log('Component initialized');
  });
</script>
```

### Server-Side Rendering (SSR) Considerations

When using SvelteKit or other SSR frameworks:

```svelte
<script lang="ts">
  import { browser } from '$app/environment';

  // ✅ Check for browser before using browser APIs
  $effect(() => {
    if (!browser) return; // Skip in SSR

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });

  // ✅ Use $state.raw() for non-reactive values (if needed)
  // Note: $state.raw() may not be available in all Svelte 5 versions
  let nonReactiveValue = { data: 'value' }; // Use regular variable instead
</script>
```

### Performance: $derived vs $state

**Use `$derived`** when:

- Value is computed from other reactive values
- Value shouldn't be directly modified
- You want automatic recomputation

**Use `$state`** when:

- Value is mutable and modified directly
- Value is independent (not computed)
- You need to update it imperatively

```svelte
<script lang="ts">
  let count = $state(0); // ✅ Mutable state
  let doubled = $derived(count * 2); // ✅ Computed value

  // ❌ Don't use $state for computed values
  // let doubled = $state(count * 2); // Wrong - won't update automatically

  // ✅ Use $state for independent mutable values
  let userInput = $state(''); // Independent state
</script>
```

### Migration from Svelte 4 Stores to Runes

If migrating from Svelte 4 stores to runes:

**Before (Svelte 4 Store)**:

```typescript
// store.ts
import { writable } from "svelte/store";
export const count = writable(0);
```

**After (Svelte 5 Runes)**:

```typescript
// store.ts
let count = $state(0);
export { count };
```

**Component Usage**:

```svelte
<!-- Before: Manual subscription -->
<script>
  import { count } from './store';
  let countValue = $state($count);
  count.subscribe(v => countValue = v);
</script>

<!-- After: Auto-subscription -->
<script>
  import { count } from './store';
  let countValue = $derived(count); // ✅ Auto-subscribes
</script>
```

### $inspect for Debugging

Use `$inspect` to debug reactive values (if available in your Svelte version):

```svelte
<script lang="ts">
  import { $inspect } from 'svelte';

  let name = $state('John');
  let age = $state(25);

  // ✅ Debug reactive values
  $inspect(name, age); // Logs whenever name or age changes
</script>
```

**Note**: `$inspect` may not be available in all Svelte 5 versions. Check the official documentation for availability.

### Version Compatibility Notes

This documentation is based on **Svelte 5.x** in runes mode. Key version considerations:

- **Svelte 5.0+**: Runes mode required for new projects
- **Migration**: Svelte 4 code needs conversion to runes
- **Store API**: Legacy stores still work but should migrate to runes
- **TypeScript**: Full TypeScript support for runes

For the latest API changes, refer to:

- [Svelte 5 Runes Documentation](https://svelte.dev/docs/svelte/runes)
- [Svelte 5 Migration Guide](https://svelte.dev/docs/v5-migration-guide)

---

## Quick Reference Checklist

When creating a new Svelte component, ensure:

- [ ] Use `$derived` instead of `$:` for computed values
- [ ] Use `$state` for mutable state
- [ ] Use `$effect` for side effects (with cleanup if needed)
- [ ] Use `onclick` (not `on:click`) for event handlers
- [ ] Add type-safe event handlers to props (Method 3 recommended)
- [ ] Use `{@render children()}` instead of `<slot />` (with `{#if}` check)
- [ ] Use `:global()` for CSS classes passed as props
- [ ] Use `$effect` with cleanup for legacy store subscriptions
- [ ] Use `$derived($store.value)` for rune-based stores
- [ ] Use `$bindable` for two-way binding props (parent must use `bind:`)
- [ ] Include error handling and accessibility attributes
- [ ] Test `$derived`, `$effect`, and `$bindable` patterns

---

## Migration Guide

If you encounter legacy Svelte code:

1. **Reactive Statements**: `$:` → `$derived` (for computed) or `$effect` (for side effects)
2. **Event Handlers**: `on:click` → `onclick`
3. **Slots**: `<slot />` → `{@render children()}` (with `{#if}` check)
4. **Props**: Add type-safe event handler support
5. **CSS**: Use `:global()` for prop classes, `class:` directive for dynamic classes
6. **Stores**: Legacy stores → Use `$effect` with cleanup; Rune stores → Use `$derived($store.value)`
7. **Two-way Binding**: Use `$bindable` with `bind:` in parent

---

## References

- [Svelte 5 Runes Documentation](https://svelte.dev/docs/svelte/runes)
- [Svelte 5 Migration Guide](https://svelte.dev/docs/v5-migration-guide)
- [Svelte Testing Library](https://testing-library.com/docs/svelte-testing-library/intro)

---

**Note**: This document is based on learnings from Phase 2 implementation. Update as new patterns are discovered in future phases.
