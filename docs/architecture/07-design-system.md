# Rustalk Design System

## Overview

This design system provides a comprehensive set of guidelines, components, and patterns for building the Rustalk VoIP application interface. It ensures consistency across all screens and makes white-labeling straightforward.

## Technology Stack

The design system is implemented using:

- **Tailwind CSS** - Utility-first CSS framework for rapid UI development
- **shadcn-svelte** - High-quality component library built on Radix UI primitives
- **Lucide Icons** - Modern icon library with consistent design
- **CSS Custom Properties** - For white-label color theming at build time

This combination provides:

- ✅ Fast development with utility classes
- ✅ Accessible, well-tested components
- ✅ Simple white-labeling via build-time configuration
- ✅ Consistent design language

---

## Foundation

### Design Tokens

We use **Tailwind CSS defaults** for most design tokens (spacing, typography scale, shadows, border radius, etc.). This keeps the design system simple and leverages Tailwind's well-tested defaults.

**Reference Tailwind's default scale:**

- **Spacing**: Use Tailwind's spacing scale (`p-1`, `p-2`, `p-4`, `gap-3`, etc.) - 4px base
- **Typography**: Use Tailwind's text utilities (`text-xs`, `text-sm`, `text-base`, `text-lg`, etc.)
- **Font Weights**: Use Tailwind's font utilities (`font-normal`, `font-medium`, `font-semibold`, `font-bold`)
- **Colors**: Use Tailwind's color palette (`gray-50` through `gray-900`, `green-500`, `red-500`, etc.)
- **Shadows**: Use Tailwind's shadow utilities (`shadow-sm`, `shadow`, `shadow-lg`, etc.)
- **Border Radius**: Use Tailwind's rounded utilities (`rounded-sm`, `rounded`, `rounded-lg`, `rounded-full`, etc.)

### Custom Tokens

We only define custom tokens for:

#### Primary Colors (White-Labelable)

These colors are customizable via the white-label TOML config:

```css
--brand-primary: #3b82f6; /* Customizable */
--brand-primary-hover: #2563eb; /* Customizable */
--brand-primary-dark: #1d4ed8; /* Customizable */
```

**Usage in Tailwind:**

```svelte
<Button class="bg-primary hover:bg-primary-hover">Call</Button>
```

#### Call State Colors (Domain-Specific)

These are specific to VoIP call states and should remain consistent:

```css
--calling: #3b82f6; /* Blue - use bg-blue-500 */
--ringing: #10b981; /* Green - use bg-green-500 */
--active: #10b981; /* Green - use bg-green-500 */
--holding: #f59e0b; /* Amber - use bg-amber-500 */
--ended: #6b7280; /* Gray - use bg-gray-500 */
--failed: #ef4444; /* Red - use bg-red-500 */
--missed: #ef4444; /* Red - use bg-red-500 */
```

**Usage in Tailwind:**

```svelte
<div class="bg-green-500">Active Call</div>
<div class="bg-red-500">Failed Call</div>
```

#### Typography

**Font Stack (White-Labelable)**

The font family can be customized via white-label config:

```css
font-family: var(
  --brand-font-family,
  -apple-system,
  BlinkMacSystemFont,
  "Segoe UI",
  Roboto,
  "Helvetica Neue",
  Arial,
  sans-serif
);
```

**Font sizes, weights, and line heights** use Tailwind defaults:

- Font sizes: `text-xs`, `text-sm`, `text-base`, `text-lg`, `text-xl`, `text-2xl`, etc.
- Font weights: `font-normal`, `font-medium`, `font-semibold`, `font-bold`
- Line heights: `leading-tight`, `leading-normal`, `leading-relaxed`

---

## Components

Components are built using **shadcn-svelte**, which provides accessible, customizable components based on Radix UI primitives. All components use Tailwind CSS utilities and can be customized via CSS custom properties for white-labeling.

### Buttons

Use the `Button` component from shadcn-svelte with Tailwind utility classes.

#### Primary Button

```svelte
<script>
  import { Button } from '$lib/components/ui/button';
  import { Phone } from 'lucide-svelte';
</script>

<Button on:click={handleCall} class="bg-primary hover:bg-primary-hover text-white">
  <Phone class="mr-2 h-4 w-4" />
  Call
</Button>
```

#### Secondary Button

```svelte
<Button variant="outline" on:click={handleCancel}>
  Cancel
</Button>
```

#### Icon Button

```svelte
<Button variant="ghost" size="icon" class="h-12 w-12">
  <Phone class="h-5 w-5" />
</Button>
```

#### Call Action Buttons

```svelte
<script>
  import { Button } from '$lib/components/ui/button';
  import { Phone, PhoneOff } from 'lucide-svelte';
</script>

<!-- Answer/Accept -->
<Button class="bg-green-500 hover:bg-green-600 text-white rounded-lg p-5">
  <Phone class="h-6 w-6" />
</Button>

<!-- Decline/End -->
<Button class="bg-red-500 hover:bg-red-600 text-white rounded-lg p-5">
  <PhoneOff class="h-6 w-6" />
</Button>
```

### Inputs

Use the `Input` and `Select` components from shadcn-svelte.

#### Text Input

```svelte
<script>
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
</script>

<Label for="username">Username</Label>
<Input
  id="username"
  type="text"
  placeholder="Enter username"
  class="w-full"
/>
```

#### Phone Number Input

```svelte
<Input
  type="tel"
  placeholder="Enter phone number"
  class="text-2xl text-center tracking-wide py-4 px-12 rounded-lg"
/>
```

#### Select Dropdown

```svelte
<script>
  import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$lib/components/ui/select';
</script>

<Select>
  <SelectTrigger class="w-full">
    <SelectValue placeholder="Select an option" />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="option1">Option 1</SelectItem>
    <SelectItem value="option2">Option 2</SelectItem>
  </SelectContent>
</Select>
```

### Cards

Use Tailwind utility classes for cards, or the `Card` component from shadcn-svelte.

#### Base Card

```svelte
<div class="bg-white border border-gray-200 rounded-lg p-4 shadow-sm hover:border-gray-300 hover:shadow transition-all">
  <!-- Card content -->
</div>
```

#### Contact Card

```svelte
<script>
  import { Card } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { Phone } from 'lucide-svelte';
</script>

<Card class="flex items-center gap-3 p-3 cursor-pointer hover:bg-gray-50">
  <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-semibold">
    SJ
  </div>
  <div class="flex-1">
    <div class="font-semibold text-gray-900">Sarah Johnson</div>
    <div class="text-sm text-gray-500">+1 (555) 987-6543</div>
  </div>
  <Button variant="ghost" size="icon">
    <Phone class="h-5 w-5" />
  </Button>
</Card>
```

#### Call History Item

```svelte
<Card class="flex items-center justify-between p-3 hover:border-gray-300">
  <div class="flex items-center gap-3">
    <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-sm font-semibold">SJ</div>
    <div>
      <div class="font-medium">Sarah Johnson</div>
      <div class="text-sm text-gray-500">+1 (555) 987-6543</div>
    </div>
  </div>
  <div class="text-sm text-gray-500">02:45</div>
</Card>
```

### Avatars

Use Tailwind utility classes for avatars.

#### Base Avatar

```svelte
<!-- Small -->
<div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-sm font-semibold">
  SJ
</div>

<!-- Medium -->
<div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-semibold">
  SJ
</div>

<!-- Large -->
<div class="w-16 h-16 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-2xl font-semibold">
  SJ
</div>

<!-- Extra Large -->
<div class="w-24 h-24 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-4xl font-semibold">
  SJ
</div>
```

#### Avatar with Gradient

```svelte
<!-- Blue gradient -->
<div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600">SJ</div>

<!-- Green gradient -->
<div class="w-10 h-10 rounded-full bg-gradient-to-br from-green-400 to-green-600">SJ</div>

<!-- Purple gradient -->
<div class="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-purple-600">SJ</div>
```

### Badges

Use the `Badge` component from shadcn-svelte.

#### Status Badge

```svelte
<script>
  import { Badge } from '$lib/components/ui/badge';
</script>

<!-- Success -->
<Badge variant="default" class="bg-green-50 text-green-600 hover:bg-green-50">
  Active
</Badge>

<!-- Error -->
<Badge variant="destructive">
  Failed
</Badge>

<!-- Warning -->
<Badge class="bg-amber-50 text-amber-600 hover:bg-amber-50">
  Warning
</Badge>

<!-- Info -->
<Badge class="bg-blue-50 text-blue-600 hover:bg-blue-50">
  Info
</Badge>
```

### Dial Pad

Use Tailwind utilities for dial pad buttons.

#### Dial Button

```svelte
<button
  class="w-full h-16 bg-white border border-gray-200 rounded-lg cursor-pointer transition-all hover:bg-gray-50 active:bg-gray-100 active:scale-[0.96] flex flex-col items-center justify-center"
  on:click={() => handleDial('1')}
>
  <div class="text-2xl font-medium text-gray-900">1</div>
  <div class="text-xs text-gray-500 mt-0.5"></div>
</button>
```

### Navigation

Use Tailwind utilities for sidebar navigation.

#### Sidebar Navigation

```svelte
<script>
  import { Home } from 'lucide-svelte';

  let active = $state('home');

  function setActive(item) {
    active = item;
  }
</script>

<nav class="w-16 bg-white border-r border-gray-200 flex flex-col items-center py-4 gap-4">
  <button
    class="w-12 h-12 rounded-lg text-gray-600 hover:bg-gray-100 transition-all flex items-center justify-center {active === 'home' ? 'bg-primary-100 text-primary-600' : ''}"
    on:click={() => setActive('home')}
  >
    <Home class="h-5 w-5" />
  </button>
  <!-- More nav items -->
</nav>
```

### Toast Notifications

Use the `Toast` component from shadcn-svelte with the toast store.

```svelte
<script>
  import { toast } from '$lib/components/ui/toast';
  import { useToast } from '$lib/components/ui/toast/use-toast';

  const { toast: showToast } = useToast();

  function showSuccess() {
    showToast({
      title: "Success",
      description: "Call connected successfully",
    });
  }
</script>
```

### Modals / Dialogs

Use the `Dialog` component from shadcn-svelte.

```svelte
<script>
  import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';

  let open = $state(false);
</script>

<Dialog bind:open>
  <DialogContent>
    <DialogHeader>
      <DialogTitle>Confirm Action</DialogTitle>
    </DialogHeader>
    <p>Are you sure you want to proceed?</p>
    <DialogFooter>
      <Button variant="outline" on:click={() => open = false}>Cancel</Button>
      <Button on:click={handleConfirm}>Confirm</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

---

## Patterns

### Call States Visual Indicators

Use Tailwind animation utilities and custom CSS for animations.

```svelte
<!-- Ringing Animation -->
<div class="animate-pulse scale-110">
  <Phone class="h-6 w-6 text-blue-500" />
</div>

<!-- Active Call Indicator -->
<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
```

Custom animations can be added to `tailwind.config.js`:

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      keyframes: {
        "pulse-ring": {
          "0%, 100%": { transform: "scale(1)", opacity: "1" },
          "50%": { transform: "scale(1.1)", opacity: "0.8" },
        },
      },
      animation: {
        "pulse-ring": "pulse-ring 2s ease-in-out infinite",
      },
    },
  },
};
```

### Audio Level Visualizer

```svelte
<div class="flex items-end gap-1 h-8">
  {#each audioLevels as level}
    <div
      class="flex-1 bg-primary rounded-t transition-all"
      style="height: {level}%"
    ></div>
  {/each}
</div>
```

### Loading States

Use the `Skeleton` component from shadcn-svelte or Tailwind utilities.

```svelte
<script>
  import { Skeleton } from '$lib/components/ui/skeleton';
  import { Loader2 } from 'lucide-svelte';
</script>

<!-- Skeleton Loader -->
<Skeleton class="h-4 w-full" />
<Skeleton class="h-4 w-3/4 mt-2" />

<!-- Spinner -->
<Loader2 class="h-6 w-6 animate-spin text-primary" />
```

---

## Layout

### Container Sizes

Use Tailwind's `max-w-*` utilities:

```svelte
<div class="max-w-xs">  <!-- 320px --> </div>
<div class="max-w-sm">  <!-- 384px --> </div>
<div class="max-w-md">  <!-- 448px --> </div>
<div class="max-w-lg">  <!-- 512px --> </div>
<div class="max-w-xl">  <!-- 576px --> </div>
<div class="max-w-2xl"> <!-- 672px --> </div>
```

### App Window

```svelte
<!-- Compact window -->
<div class="w-[400px] h-[600px] bg-white rounded-xl overflow-hidden flex flex-col">
  <!-- App content -->
</div>

<!-- Expanded window -->
<div class="w-[800px] h-[768px] bg-white rounded-xl overflow-hidden flex flex-col">
  <!-- App content -->
</div>
```

### Grid Layouts

Use Tailwind's grid utilities:

```svelte
<!-- 2-column grid for contacts -->
<div class="grid grid-cols-2 gap-4">
  <!-- Contact items -->
</div>

<!-- 3-column grid for dial pad -->
<div class="grid grid-cols-3 gap-3">
  <!-- Dial buttons -->
</div>

<!-- 4-column grid for call controls -->
<div class="grid grid-cols-4 gap-3">
  <!-- Control buttons -->
</div>
```

---

## Accessibility

### Focus States

Use Tailwind's focus utilities with the primary color:

```svelte
<button class="focus-visible:outline-2 focus-visible:outline-primary focus-visible:outline-offset-2">
  Button
</button>
```

### Screen Reader Only

Use Tailwind's `sr-only` utility:

```svelte
<span class="sr-only">Screen reader only text</span>
```

### Reduced Motion

Tailwind automatically respects `prefers-reduced-motion`. For custom animations, use the `motion-safe:` and `motion-reduce:` variants:

```svelte
<div class="motion-safe:animate-pulse motion-reduce:animate-none">
  Animated content
</div>
```

---

## White-Label Customization

White-labeling is handled at **build time** via a TOML configuration file. A Vite plugin processes the config and generates CSS custom properties that Tailwind uses for theming.

### Configuration Format (TOML)

Create a `white-label.toml` file in the project root:

```toml
[branding]
app_name = "Rustalk"
company_name = "Your Company"

[branding.colors]
primary = "#3B82F6"
primary_hover = "#2563EB"
primary_dark = "#1D4ED8"

[branding.logo]
path = "./assets/logo.png"
icon_path = "./assets/icon.ico"
width = 120
height = 40

[branding.typography]
font_family = "Inter, system-ui, sans-serif"
font_url = "https://fonts.googleapis.com/css2?family=Inter"
```

### Build-Time Processing

A Vite plugin reads the TOML config and generates CSS custom properties:

```css
/* Generated at build time: src/lib/styles/theme.css */
:root {
  --brand-primary: #3b82f6;
  --brand-primary-hover: #2563eb;
  --brand-primary-dark: #1d4ed8;
  --brand-font-family: Inter, system-ui, sans-serif;
}
```

### Tailwind Configuration

Tailwind is configured to use these CSS variables:

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "var(--brand-primary)",
          hover: "var(--brand-primary-hover)",
          dark: "var(--brand-primary-dark)",
        },
      },
    },
  },
};
```

### Usage in Components

Components automatically use the white-label colors:

```svelte
<!-- This button uses the brand primary color from TOML config -->
<Button class="bg-primary hover:bg-primary-hover">
  Call
</Button>
```

### Build Process

1. User edits `white-label.toml` with their brand colors
2. Run `npm run build` or `npm run tauri:build`
3. Vite plugin processes TOML → generates CSS variables
4. Tailwind utilities reference the generated variables
5. Final build includes the custom brand colors

---

## Animation Guidelines

### Durations

- **Fast**: 150ms - Micro-interactions (button press, hover)
- **Normal**: 200ms - Standard transitions (color, opacity)
- **Slow**: 300ms - Layout changes, modals

### Easing Functions

```css
--ease-in: cubic-bezier(0.4, 0, 1, 1);
--ease-out: cubic-bezier(0, 0, 0.2, 1);
--ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
```

---

## Responsive Breakpoints

```css
/* Mobile First Approach */
--screen-sm: 640px;
--screen-md: 768px;
--screen-lg: 1024px;
--screen-xl: 1280px;

@media (min-width: 640px) {
  /* sm */
}
@media (min-width: 768px) {
  /* md */
}
@media (min-width: 1024px) {
  /* lg */
}
@media (min-width: 1280px) {
  /* xl */
}
```

---

## Usage Examples

### Complete Button Implementation

```svelte
<script>
  import { Button } from '$lib/components/ui/button';
  import { Phone } from 'lucide-svelte';

  function handleCall() {
    // Call logic
  }
</script>

<Button on:click={handleCall} class="bg-primary hover:bg-primary-hover text-white">
  <Phone class="mr-2 h-4 w-4" />
  <span>Call</span>
</Button>
```

### Contact Card

```svelte
<script>
  import { Card } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { Phone } from 'lucide-svelte';
</script>

<Card class="flex items-center gap-3 p-3 cursor-pointer hover:bg-gray-50">
  <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-semibold">
    SJ
  </div>
  <div class="flex-1">
    <div class="font-semibold text-gray-900">Sarah Johnson</div>
    <div class="text-sm text-gray-500">+1 (555) 987-6543</div>
  </div>
  <Button variant="ghost" size="icon">
    <Phone class="h-5 w-5" />
  </Button>
</Card>
```

### Call State Indicator

```svelte
<div class="flex items-center gap-2">
  <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
  <span class="text-sm font-mono text-gray-700">02:45</span>
</div>
```

---

## Implementation Guide

### Setup Steps

1. **Install Dependencies**

   ```bash
   npm install -D tailwindcss postcss autoprefixer
   npm install -D @tailwindcss/typography
   npx shadcn-svelte@latest init
   npm install lucide-svelte
   ```

2. **Initialize Tailwind**

   ```bash
   npx tailwindcss init -p
   ```

3. **Configure Tailwind** (`tailwind.config.js`)

   ```js
   import { fontFamily } from "tailwindcss/defaultTheme";

   /** @type {import('tailwindcss').Config} */
   export default {
     content: ["./src/**/*.{html,js,svelte,ts}"],
     theme: {
       extend: {
         colors: {
           primary: {
             DEFAULT: "var(--brand-primary)",
             hover: "var(--brand-primary-hover)",
             dark: "var(--brand-primary-dark)",
           },
         },
         fontFamily: {
           sans: ["var(--brand-font-family)", ...fontFamily.sans],
         },
       },
     },
     plugins: [],
   };
   ```

4. **Add Tailwind to Global CSS** (`src/app.css`)

   ```css
   @tailwind base;
   @tailwind components;
   @tailwind utilities;

   @layer base {
     :root {
       --brand-primary: #3b82f6;
       --brand-primary-hover: #2563eb;
       --brand-primary-dark: #1d4ed8;
       --brand-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI",
         Roboto, sans-serif;
     }
   }
   ```

5. **Import CSS in Layout** (`src/routes/+layout.svelte`)

   ```svelte
   <script>
     import '../app.css';
   </script>
   ```

6. **Add shadcn Components**
   ```bash
   npx shadcn-svelte@latest add button
   npx shadcn-svelte@latest add input
   npx shadcn-svelte@latest add card
   # ... add other components as needed
   ```

### Implementation Notes

1. **CSS Custom Properties** - Used for white-label theming at build time
2. **Component Library** - shadcn-svelte provides accessible, customizable components
3. **Tailwind CSS** - Utility-first approach for rapid development
4. **Icons** - Lucide icons for consistent iconography
5. **Responsive by Default** - Tailwind utilities work across screen sizes
6. **Accessibility Built-in** - shadcn components include ARIA labels and keyboard navigation
7. **Build-Time Theming** - White-label colors injected during build process
