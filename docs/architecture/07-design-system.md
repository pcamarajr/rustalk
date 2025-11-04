# Rustalk Design System

## Overview

This design system provides a comprehensive set of guidelines, components, and patterns for building the Rustalk VoIP application interface. It ensures consistency across all screens and makes white-labeling straightforward.

---

## Foundation

### Colors

#### Primary Palette
```css
--primary-50:  #EFF6FF;
--primary-100: #DBEAFE;
--primary-200: #BFDBFE;
--primary-300: #93C5FD;
--primary-400: #60A5FA;
--primary-500: #3B82F6; /* Primary brand color */
--primary-600: #2563EB;
--primary-700: #1D4ED8;
--primary-800: #1E40AF;
--primary-900: #1E3A8A;
```

#### Neutral Palette
```css
--gray-50:  #F9FAFB;
--gray-100: #F3F4F6;
--gray-200: #E5E7EB;
--gray-300: #D1D5DB;
--gray-400: #9CA3AF;
--gray-500: #6B7280;
--gray-600: #4B5563;
--gray-700: #374151;
--gray-800: #1F2937;
--gray-900: #111827;
```

#### Semantic Colors
```css
/* Success */
--success-50:  #F0FDF4;
--success-500: #10B981;
--success-600: #059669;

/* Error/Danger */
--error-50:  #FEF2F2;
--error-500: #EF4444;
--error-600: #DC2626;

/* Warning */
--warning-50:  #FFFBEB;
--warning-500: #F59E0B;
--warning-600: #D97706;

/* Info */
--info-50:  #EFF6FF;
--info-500: #3B82F6;
--info-600: #2563EB;
```

#### Call State Colors
```css
--calling:   #3B82F6; /* Blue */
--ringing:   #10B981; /* Green */
--active:    #10B981; /* Green */
--holding:   #F59E0B; /* Amber */
--ended:     #6B7280; /* Gray */
--failed:    #EF4444; /* Red */
--missed:    #EF4444; /* Red */
```

### Typography

#### Font Stack
```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 
             'Helvetica Neue', Arial, sans-serif, 'Apple Color Emoji', 
             'Segoe UI Emoji', 'Segoe UI Symbol';
```

#### Font Sizes
```css
--text-xs:   0.75rem;  /* 12px */
--text-sm:   0.875rem; /* 14px */
--text-base: 1rem;     /* 16px */
--text-lg:   1.125rem; /* 18px */
--text-xl:   1.25rem;  /* 20px */
--text-2xl:  1.5rem;   /* 24px */
--text-3xl:  1.875rem; /* 30px */
--text-4xl:  2.25rem;  /* 36px */
```

#### Font Weights
```css
--font-normal:    400;
--font-medium:    500;
--font-semibold:  600;
--font-bold:      700;
```

#### Line Heights
```css
--leading-tight:  1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.625;
```

### Spacing

#### Scale (based on 4px)
```css
--space-0:  0;
--space-1:  0.25rem;  /* 4px */
--space-2:  0.5rem;   /* 8px */
--space-3:  0.75rem;  /* 12px */
--space-4:  1rem;     /* 16px */
--space-5:  1.25rem;  /* 20px */
--space-6:  1.5rem;   /* 24px */
--space-8:  2rem;     /* 32px */
--space-10: 2.5rem;   /* 40px */
--space-12: 3rem;     /* 48px */
--space-16: 4rem;     /* 64px */
```

### Border Radius
```css
--radius-sm:   0.25rem; /* 4px */
--radius-base: 0.5rem;  /* 8px */
--radius-lg:   0.75rem; /* 12px */
--radius-xl:   1rem;    /* 16px */
--radius-full: 9999px;  /* Circular */
```

### Shadows
```css
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
--shadow:    0 1px 3px 0 rgba(0, 0, 0, 0.1), 
             0 1px 2px 0 rgba(0, 0, 0, 0.06);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 
             0 2px 4px -1px rgba(0, 0, 0, 0.06);
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 
             0 4px 6px -2px rgba(0, 0, 0, 0.05);
--shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 
             0 10px 10px -5px rgba(0, 0, 0, 0.04);
```

---

## Components

### Buttons

#### Primary Button
```css
.btn-primary {
  background: var(--primary-500);
  color: white;
  padding: 12px 24px;
  border-radius: var(--radius-base);
  font-weight: var(--font-semibold);
  font-size: var(--text-base);
  border: none;
  cursor: pointer;
  transition: all 200ms ease-in-out;
}

.btn-primary:hover {
  background: var(--primary-600);
}

.btn-primary:active {
  background: var(--primary-700);
}

.btn-primary:disabled {
  background: var(--gray-300);
  cursor: not-allowed;
  opacity: 0.6;
}
```

#### Secondary Button
```css
.btn-secondary {
  background: transparent;
  color: var(--gray-700);
  padding: 12px 24px;
  border-radius: var(--radius-base);
  font-weight: var(--font-medium);
  font-size: var(--text-base);
  border: 1px solid var(--gray-300);
  cursor: pointer;
  transition: all 200ms ease-in-out;
}

.btn-secondary:hover {
  background: var(--gray-50);
  border-color: var(--gray-400);
}
```

#### Icon Button
```css
.btn-icon {
  width: 48px;
  height: 48px;
  background: var(--gray-100);
  color: var(--gray-700);
  border-radius: var(--radius-full);
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 200ms ease-in-out;
}

.btn-icon:hover {
  background: var(--gray-200);
}

.btn-icon:active {
  background: var(--gray-300);
  transform: scale(0.95);
}
```

#### Call Action Buttons
```css
/* Answer/Accept */
.btn-accept {
  background: var(--success-500);
  color: white;
  padding: 20px;
  border-radius: var(--radius-lg);
  font-weight: var(--font-semibold);
}

.btn-accept:hover {
  background: var(--success-600);
}

/* Decline/End */
.btn-decline {
  background: var(--error-500);
  color: white;
  padding: 20px;
  border-radius: var(--radius-lg);
  font-weight: var(--font-semibold);
}

.btn-decline:hover {
  background: var(--error-600);
}
```

### Inputs

#### Text Input
```css
.input-text {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-base);
  font-size: var(--text-base);
  color: var(--gray-900);
  background: white;
  transition: all 200ms ease-in-out;
}

.input-text:focus {
  outline: none;
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px var(--primary-100);
}

.input-text:disabled {
  background: var(--gray-50);
  cursor: not-allowed;
}

.input-text.error {
  border-color: var(--error-500);
}

.input-text.error:focus {
  box-shadow: 0 0 0 3px var(--error-50);
}
```

#### Phone Number Input
```css
.input-phone {
  width: 100%;
  padding: 16px 48px 16px 16px;
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-lg);
  font-size: var(--text-2xl);
  font-weight: var(--font-light);
  text-align: center;
  color: var(--gray-900);
  background: white;
  letter-spacing: 0.05em;
}

.input-phone::placeholder {
  color: var(--gray-400);
  font-weight: var(--font-normal);
}
```

#### Select Dropdown
```css
.select {
  appearance: none;
  padding: 12px 40px 12px 16px;
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-base);
  font-size: var(--text-base);
  color: var(--gray-900);
  background: white url("data:...") no-repeat right 12px center;
  background-size: 16px;
  cursor: pointer;
}

.select:focus {
  outline: none;
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px var(--primary-100);
}
```

### Cards

#### Base Card
```css
.card {
  background: white;
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  box-shadow: var(--shadow-sm);
  transition: all 200ms ease-in-out;
}

.card:hover {
  border-color: var(--gray-300);
  box-shadow: var(--shadow);
}
```

#### Contact Card
```css
.card-contact {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3);
  background: white;
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all 200ms ease-in-out;
}

.card-contact:hover {
  border-color: var(--gray-300);
  background: var(--gray-50);
}
```

#### Call History Item
```css
.card-history {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3);
  background: white;
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-base);
}

.card-history:hover {
  border-color: var(--gray-300);
}
```

### Avatars

#### Base Avatar
```css
.avatar {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--font-semibold);
  color: white;
  overflow: hidden;
}

.avatar-sm { width: 32px; height: 32px; font-size: var(--text-sm); }
.avatar-md { width: 40px; height: 40px; font-size: var(--text-base); }
.avatar-lg { width: 64px; height: 64px; font-size: var(--text-2xl); }
.avatar-xl { width: 96px; height: 96px; font-size: var(--text-4xl); }
```

#### Avatar with Gradient
```css
.avatar-gradient-blue {
  background: linear-gradient(135deg, #60A5FA 0%, #2563EB 100%);
}

.avatar-gradient-green {
  background: linear-gradient(135deg, #34D399 0%, #059669 100%);
}

.avatar-gradient-purple {
  background: linear-gradient(135deg, #A78BFA 0%, #7C3AED 100%);
}
```

### Badges

#### Status Badge
```css
.badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: 4px 12px;
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-medium);
}

.badge-success {
  background: var(--success-50);
  color: var(--success-600);
}

.badge-error {
  background: var(--error-50);
  color: var(--error-600);
}

.badge-warning {
  background: var(--warning-50);
  color: var(--warning-600);
}

.badge-info {
  background: var(--info-50);
  color: var(--info-600);
}
```

### Dial Pad

#### Dial Button
```css
.dial-button {
  width: 100%;
  height: 64px;
  background: white;
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all 150ms ease-in-out;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.dial-button:hover {
  background: var(--gray-50);
}

.dial-button:active {
  background: var(--gray-100);
  transform: scale(0.96);
}

.dial-button-number {
  font-size: var(--text-2xl);
  font-weight: var(--font-medium);
  color: var(--gray-900);
}

.dial-button-letters {
  font-size: var(--text-xs);
  color: var(--gray-500);
  margin-top: 2px;
}
```

### Navigation

#### Sidebar Navigation
```css
.sidebar {
  width: 64px;
  background: white;
  border-right: 1px solid var(--gray-200);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--space-4) 0;
  gap: var(--space-4);
}

.sidebar-item {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-lg);
  color: var(--gray-600);
  cursor: pointer;
  transition: all 200ms ease-in-out;
}

.sidebar-item:hover {
  background: var(--gray-100);
}

.sidebar-item.active {
  background: var(--primary-100);
  color: var(--primary-600);
}
```

### Toast Notifications

```css
.toast {
  min-width: 300px;
  padding: var(--space-4);
  background: white;
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  display: flex;
  align-items: start;
  gap: var(--space-3);
}

.toast-success {
  border-left: 4px solid var(--success-500);
}

.toast-error {
  border-left: 4px solid var(--error-500);
}

.toast-warning {
  border-left: 4px solid var(--warning-500);
}

.toast-info {
  border-left: 4px solid var(--info-500);
}
```

### Modals

```css
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: white;
  border-radius: var(--radius-xl);
  padding: var(--space-6);
  max-width: 500px;
  width: 90%;
  box-shadow: var(--shadow-xl);
}

.modal-header {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--gray-900);
  margin-bottom: var(--space-4);
}

.modal-footer {
  display: flex;
  gap: var(--space-3);
  justify-content: flex-end;
  margin-top: var(--space-6);
}
```

---

## Patterns

### Call States Visual Indicators

```css
/* Ringing Animation */
@keyframes pulse-ring {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.1); opacity: 0.8; }
}

.calling-indicator {
  animation: pulse-ring 2s ease-in-out infinite;
}

/* Active Call Indicator */
.active-call-dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  background: var(--success-500);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

### Audio Level Visualizer

```css
.audio-visualizer {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  height: 32px;
}

.audio-bar {
  flex: 1;
  background: var(--primary-500);
  border-radius: 2px 2px 0 0;
  transition: height 100ms ease-in-out;
}
```

### Loading States

```css
/* Skeleton Loader */
.skeleton {
  background: linear-gradient(
    90deg,
    var(--gray-200) 25%,
    var(--gray-100) 50%,
    var(--gray-200) 75%
  );
  background-size: 200% 100%;
  animation: loading 1.5s ease-in-out infinite;
  border-radius: var(--radius-base);
}

@keyframes loading {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* Spinner */
.spinner {
  width: 24px;
  height: 24px;
  border: 3px solid var(--gray-200);
  border-top-color: var(--primary-500);
  border-radius: var(--radius-full);
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

---

## Layout

### Container Sizes
```css
.container-sm  { max-width: 320px; }
.container-md  { max-width: 480px; }
.container-lg  { max-width: 640px; }
.container-xl  { max-width: 800px; }
```

### App Window
```css
.app-window {
  width: 400px;
  height: 600px;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.app-window-expanded {
  width: 800px;
  height: 768px;
}
```

### Grid Layouts
```css
/* 2-column grid for contacts */
.grid-contacts {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--space-4);
}

/* 3-column grid for dial pad */
.grid-dialpad {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-3);
}

/* 4-column grid for call controls */
.grid-controls {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--space-3);
}
```

---

## Accessibility

### Focus States
```css
*:focus-visible {
  outline: 2px solid var(--primary-500);
  outline-offset: 2px;
}

button:focus-visible {
  outline: 2px solid var(--primary-500);
  outline-offset: 2px;
}
```

### Screen Reader Only
```css
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}
```

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## White-Label Customization

### Configuration Format (TOML)

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

### CSS Variables for Theming

```css
:root {
  /* Customizable via white-label config */
  --brand-primary: var(--primary-500);
  --brand-primary-hover: var(--primary-600);
  --brand-primary-dark: var(--primary-700);
  
  /* Can be overridden */
  --brand-font-family: var(--font-family);
}

/* Override theme */
[data-theme="custom"] {
  --brand-primary: #8B5CF6; /* Purple */
  --brand-primary-hover: #7C3AED;
  --brand-primary-dark: #6D28D9;
}
```

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

@media (min-width: 640px) { /* sm */ }
@media (min-width: 768px) { /* md */ }
@media (min-width: 1024px) { /* lg */ }
@media (min-width: 1280px) { /* xl */ }
```

---

## Usage Examples

### Complete Button Implementation
```jsx
<button className="btn-primary" onClick={handleCall}>
  <PhoneIcon />
  <span>Call</span>
</button>
```

### Contact Card
```jsx
<div className="card-contact">
  <div className="avatar avatar-md avatar-gradient-blue">
    SJ
  </div>
  <div className="flex-1">
    <div className="font-semibold text-gray-900">Sarah Johnson</div>
    <div className="text-sm text-gray-500">+1 (555) 987-6543</div>
  </div>
  <button className="btn-icon">
    <PhoneIcon />
  </button>
</div>
```

### Call State Indicator
```jsx
<div className="flex items-center gap-2">
  <div className="active-call-dot"></div>
  <span className="text-sm font-mono text-gray-700">02:45</span>
</div>
```

---

## Implementation Notes

1. **Use CSS Custom Properties** for theming - makes white-labeling simple
2. **Component-first approach** - Build reusable React components
3. **Tailwind CSS compatible** - Design system aligns with Tailwind utilities
4. **Dark mode ready** - Color variables can be swapped for dark theme
5. **Responsive by default** - All components work on different screen sizes
6. **Accessibility built-in** - ARIA labels, keyboard navigation, focus states
