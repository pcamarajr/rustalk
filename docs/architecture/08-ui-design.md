# Rustalk UI Design Document

## Design Philosophy

**Modern, Clean, Professional**
- Minimalist interface focused on core VoIP functionality
- Consistent design language across all screens
- Accessible and intuitive for non-technical users
- White-label friendly (easily themeable)
- Cross-platform consistency (Windows, macOS, Linux)

## Color System (Customizable via White-Label)

```
Primary Colors:
- Primary: #3B82F6 (Blue-500)
- Primary Hover: #2563EB (Blue-600)
- Primary Light: #DBEAFE (Blue-100)

Neutral Colors:
- Background: #FFFFFF
- Surface: #F9FAFB (Gray-50)
- Border: #E5E7EB (Gray-200)
- Text Primary: #111827 (Gray-900)
- Text Secondary: #6B7280 (Gray-500)

Status Colors:
- Success: #10B981 (Green-500)
- Error: #EF4444 (Red-500)
- Warning: #F59E0B (Amber-500)
- Calling: #3B82F6 (Blue-500)
```

## Typography

```
Font Family: System UI Stack
- -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial

Font Sizes:
- Heading 1: 24px (1.5rem) - Bold
- Heading 2: 20px (1.25rem) - Semibold
- Heading 3: 18px (1.125rem) - Semibold
- Body: 16px (1rem) - Regular
- Small: 14px (0.875rem) - Regular
- Tiny: 12px (0.75rem) - Regular
```

---

## Main Application Layout

### Window Structure
```
┌─────────────────────────────────────────────┐
│  [App Title/Logo]              [- □ ×]      │  ← Title Bar
├─────────────────────────────────────────────┤
│ ┌─────┐                                     │
│ │     │                                     │
│ │  S  │         Main Content Area           │  ← Sidebar + Content
│ │  I  │                                     │
│ │  D  │                                     │
│ │  E  │                                     │
│ │     │                                     │
│ └─────┘                                     │
└─────────────────────────────────────────────┘

Window Size: 
- Default: 380px × 600px (phone-like proportions)
- Expandable: Up to 1024px × 768px for larger views
- Minimum: 320px × 480px
```

---

## 1. Main Dialer Screen (Home)

### Layout
```
┌─────────────────────────────────────────────┐
│  Rustalk                           [- □ ×]  │
├─────┬───────────────────────────────────────┤
│  🏠 │   [Profile: John Doe ▼]        [⚙️]  │
│     │                                       │
│  📞 │   ┌─────────────────────────────┐    │
│     │   │  +1 (555) 123-4567    [×]  │    │
│  📋 │   └─────────────────────────────┘    │
│     │                                       │
│  🕐 │   ┌───┬───┬───┐                      │
│     │   │ 1 │ 2 │ 3 │                      │
│  👤 │   ├───┼───┼───┤                      │
│     │   │ 4 │ 5 │ 6 │                      │
│     │   ├───┼───┼───┤                      │
│     │   │ 7 │ 8 │ 9 │                      │
│     │   ├───┼───┼───┤                      │
│     │   │ * │ 0 │ # │                      │
│     │   └───┴───┴───┘                      │
│     │                                       │
│     │   ┌─────────────────────────────┐    │
│     │   │     📞 Call                 │    │
│     │   └─────────────────────────────┘    │
│     │                                       │
│     │   Recent Calls:                      │
│     │   • Mom (Mobile)        2m ago       │
│     │   • Work - Conference   1h ago       │
│     │   • Sarah Johnson      2h ago       │
└─────┴───────────────────────────────────────┘
```

### Components

**Number Input Field**
- Large, clear display of entered numbers
- Clear button (×) on the right
- Auto-formats as numbers are typed
- Placeholder: "Enter number or name"

**Dial Pad**
- 3×4 grid of large, clickable buttons
- Each button shows number and letters (like phone)
- Button size: 60×60px with 8px spacing
- Hover state: subtle highlight
- Active state: pressed effect
- Haptic feedback on click (if supported)

**Call Button**
- Full width
- Primary color background
- Large, prominent
- Phone icon + "Call" text
- Disabled state if no number entered

**Recent Calls Preview**
- Shows last 3 calls
- Contact name or number
- Time ago
- Call direction icon (incoming/outgoing/missed)

---

## 2. Active Call Screen

### Layout (Compact View)
```
┌─────────────────────────────────────────────┐
│  In Call...                        [- □ ×]  │
├─────────────────────────────────────────────┤
│                                             │
│           [Avatar/Initials]                 │
│                                             │
│         Sarah Johnson                       │
│         +1 (555) 987-6543                   │
│                                             │
│         ⏱ 00:02:45                          │
│                                             │
│     ┌─────────────────────────────┐        │
│     │  Volume: ▁▂▃▄▅▆▇  [🔊]     │        │
│     └─────────────────────────────┘        │
│                                             │
│     ┌───┐  ┌───┐  ┌───┐  ┌───┐            │
│     │🎤│  │⏸ │  │🔢│  │⚙️│            │
│     │Mute│ │Hold│ │Pad│ │...│            │
│     └───┘  └───┘  └───┘  └───┘            │
│                                             │
│     ┌─────────────────────────────┐        │
│     │   📞 End Call              │        │
│     └─────────────────────────────┘        │
│                                             │
└─────────────────────────────────────────────┘
```

### Components

**Call Header**
- Contact name (large, bold)
- Phone number (smaller, gray)
- Call timer (MM:SS or HH:MM:SS)

**Audio Visualizer**
- Real-time volume indicator
- Shows both incoming and outgoing audio levels
- Helps confirm audio is working

**Call Controls Grid**
- 2 rows of controls
- Top row: Mute, Hold, Keypad, More
- Bottom: Large red End Call button

**Control Buttons**
```
Mute Button:
- Shows microphone icon
- Toggles between active/muted
- Muted state: red background with mic-off icon
- Label: "Mute" / "Unmute"

Hold Button:
- Shows pause icon
- Toggles between active/hold
- Hold state: amber background
- Label: "Hold" / "Resume"

Keypad Button:
- Shows dial pad icon
- Opens DTMF keypad overlay
- For navigating phone menus

More Button:
- Shows three dots icon
- Opens menu with:
  - Transfer call
  - Add to conference
  - Record call (if enabled)
  - Audio settings
```

**End Call Button**
- Full width, prominent
- Red background (#EF4444)
- Phone icon + "End Call"

---

## 3. Incoming Call Screen

### Layout
```
┌─────────────────────────────────────────────┐
│  Incoming Call...                  [- □ ×]  │
├─────────────────────────────────────────────┤
│                                             │
│                                             │
│           [Avatar/Initials]                 │
│                                             │
│         Mom (Mobile)                        │
│         +1 (555) 123-4567                   │
│                                             │
│         📞 Incoming call...                 │
│                                             │
│                                             │
│     ┌─────────────┐  ┌─────────────┐       │
│     │   ❌ Decline│  │  ✅ Answer  │       │
│     └─────────────┘  └─────────────┘       │
│                                             │
│                                             │
└─────────────────────────────────────────────┘
```

### Features
- Full screen overlay
- Ringtone playing
- Window flashing/bouncing (system notification)
- Two large buttons: Decline (red) / Answer (green)
- Quick actions: Send to voicemail

---

## 4. Contacts Screen

### Layout
```
┌─────────────────────────────────────────────┐
│  Contacts                          [- □ ×]  │
├─────┬───────────────────────────────────────┤
│  🏠 │   🔍 Search contacts...        [+]   │
│     │                                       │
│  📞 │   Favorites                    ⭐     │
│     │   ┌─────────────────────────────┐    │
│  📋 │   │ 👤 Mom          📞          │    │
│     │   │ 👤 Dad          📞          │    │
│  🕐 │   │ 👤 Sarah        📞          │    │
│     │   └─────────────────────────────┘    │
│  👤 │                                       │
│     │   All Contacts                  A-Z  │
│     │   ┌─────────────────────────────┐    │
│     │   │ A                           │    │
│     │   │ 👤 Alice Smith    📞        │    │
│     │   │ 👤 Alex Jones     📞        │    │
│     │   │                             │    │
│     │   │ B                           │    │
│     │   │ 👤 Bob Williams   📞        │    │
│     │   │ ...                         │    │
│     │   └─────────────────────────────┘    │
└─────┴───────────────────────────────────────┘
```

### Components

**Search Bar**
- Top of screen
- Real-time filtering
- Searches name and number
- Clear button

**Add Contact Button**
- Top right corner
- Opens add contact dialog

**Contact List**
- Alphabetical sections
- Avatar or initials
- Name and primary number
- Quick call button (phone icon)
- Click to view details

**Contact Card (Click to expand)**
```
┌─────────────────────────────────────┐
│  [Avatar]  Alice Smith        [×]   │
├─────────────────────────────────────┤
│                                     │
│  📞 Mobile:  +1 (555) 123-4567     │
│             [Call] [Message]        │
│                                     │
│  📞 Work:    +1 (555) 987-6543     │
│             [Call]                  │
│                                     │
│  📧 Email:   alice@example.com      │
│                                     │
│  [Edit Contact]  [⭐ Favorite]      │
└─────────────────────────────────────┘
```

---

## 5. Call History Screen

### Layout
```
┌─────────────────────────────────────────────┐
│  Call History                      [- □ ×]  │
├─────┬───────────────────────────────────────┤
│  🏠 │   [All ▼] [Today ▼]           [🗑]   │
│     │                                       │
│  📞 │   Today                               │
│     │   ┌─────────────────────────────┐    │
│  📋 │   │ ↗️ Sarah Johnson           │    │
│     │   │    Outgoing · 2:45         │    │
│  🕐 │   │    2:34 PM           [📞]  │    │
│     │   ├─────────────────────────────┤    │
│  👤 │   │ ↙️ Mom (Mobile)            │    │
│     │   │    Incoming · 15:23        │    │
│     │   │    10:15 AM          [📞]  │    │
│     │   ├─────────────────────────────┤    │
│     │   │ ❌ Unknown Number           │    │
│     │   │    Missed · Not answered   │    │
│     │   │    9:42 AM           [📞]  │    │
│     │   └─────────────────────────────┘    │
│     │                                       │
│     │   Yesterday                           │
│     │   ┌─────────────────────────────┐    │
│     │   │ ↗️ Work - Conference       │    │
│     │   │    Outgoing · 45:12        │    │
│     │   │    Yesterday 3:00 PM [📞]  │    │
│     │   └─────────────────────────────┘    │
└─────┴───────────────────────────────────────┘
```

### Components

**Filter Controls**
- Call type filter: All / Missed / Outgoing / Incoming
- Date filter: Today / Yesterday / This Week / Custom Range
- Clear history button (trash icon)

**Call Entry**
- Direction icon (↗️ outgoing, ↙️ incoming, ❌ missed)
- Contact name or number
- Call type and duration
- Date/time
- Quick call back button

**Call Entry Expanded (Click for details)**
```
┌─────────────────────────────────────┐
│  ↗️ Sarah Johnson                   │
│  +1 (555) 987-6543                  │
├─────────────────────────────────────┤
│  📅 November 2, 2025                │
│  🕐 2:34 PM - 2:37 PM               │
│  ⏱ Duration: 2:45                   │
│  📊 Quality: Excellent              │
│                                     │
│  [📞 Call Back]  [💬 Message]       │
│  [👤 Add to Contacts]               │
└─────────────────────────────────────┘
```

---

## 6. Settings Screen

### Layout
```
┌─────────────────────────────────────────────┐
│  Settings                          [- □ ×]  │
├─────┬───────────────────────────────────────┤
│  🏠 │   Account                             │
│     │   ┌─────────────────────────────┐    │
│  📞 │   │ 👤 John Doe                 │    │
│     │   │    john.doe@example.com     │    │
│  📋 │   │    Status: Connected 🟢     │    │
│     │   │    [Edit Profile]           │    │
│  🕐 │   └─────────────────────────────┘    │
│     │                                       │
│  👤 │   Audio Settings                      │
│     │   ┌─────────────────────────────┐    │
│     │   │ 🎤 Microphone                │    │
│     │   │    Built-in Microphone ▼    │    │
│     │   │    [Test] ▁▂▃▄▅▆▇           │    │
│     │   │                              │    │
│     │   │ 🔊 Speaker                   │    │
│     │   │    Built-in Speaker ▼       │    │
│     │   │    [Test] Volume: 75%       │    │
│     │   │                              │    │
│     │   │ 🔔 Ringtone                  │    │
│     │   │    Default Ringtone ▼       │    │
│     │   │    [Play] Volume: 80%       │    │
│     │   └─────────────────────────────┘    │
│     │                                       │
│     │   SIP Account                         │
│     │   ┌─────────────────────────────┐    │
│     │   │ Server: sip.example.com     │    │
│     │   │ Port: 5060                  │    │
│     │   │ Protocol: UDP               │    │
│     │   │ [Edit Connection]           │    │
│     │   └─────────────────────────────┘    │
└─────┴───────────────────────────────────────┘
```

### Settings Sections

**Account**
- Display name
- Email/Username
- Connection status indicator
- Edit profile button

**Audio Settings**
- Microphone selection dropdown
- Microphone test with live audio level
- Speaker/output device selection
- Speaker test button with volume slider
- Ringtone selection and preview
- Echo cancellation toggle
- Noise suppression toggle

**SIP Account**
- Server address
- Port
- Protocol (UDP/TCP/TLS)
- Username
- Password (hidden)
- Register on startup
- Keep-alive interval

**General**
- Launch on system startup
- Minimize to tray
- Notifications enabled
- Call history retention period

**Appearance** (White-Label)
- Theme selection (if multiple themes)
- Language selection
- Font size adjustment

**About**
- App version
- Build information
- License information
- Check for updates button

---

## 7. DTMF Keypad Overlay (During Call)

### Layout
```
┌─────────────────────────────────────┐
│  Enter digits                  [×]  │
├─────────────────────────────────────┤
│                                     │
│   ┌───────────────────────────┐    │
│   │  1 2 3 4 5 *              │    │
│   └───────────────────────────┘    │
│                                     │
│   ┌───┬───┬───┐                    │
│   │ 1 │ 2 │ 3 │                    │
│   │   │ABC│DEF│                    │
│   ├───┼───┼───┤                    │
│   │ 4 │ 5 │ 6 │                    │
│   │GHI│JKL│MNO│                    │
│   ├───┼───┼───┤                    │
│   │ 7 │ 8 │ 9 │                    │
│   │PQRS│TUV│WXYZ│                  │
│   ├───┼───┼───┤                    │
│   │ * │ 0 │ # │                    │
│   │   │   │   │                    │
│   └───┴───┴───┘                    │
│                                     │
└─────────────────────────────────────┘
```

- Overlay appears over active call screen
- Shows digits entered at top
- Each button press sends DTMF tone
- Audible feedback on press
- Close button to return to call screen

---

## Component Library

### Buttons

**Primary Button**
```css
Background: #3B82F6
Text: White
Padding: 12px 24px
Border Radius: 8px
Font: 16px Semibold

Hover: #2563EB
Active: #1D4ED8
Disabled: #9CA3AF with 50% opacity
```

**Secondary Button**
```css
Background: Transparent
Border: 1px solid #E5E7EB
Text: #111827
Padding: 12px 24px
Border Radius: 8px

Hover: Background #F9FAFB
```

**Icon Button**
```css
Size: 48×48px
Border Radius: 50% (circular)
Background: #F9FAFB
Icon: 24×24px

Hover: Background #E5E7EB
Active: Background #D1D5DB
```

### Input Fields

**Text Input**
```css
Height: 48px
Padding: 12px 16px
Border: 1px solid #E5E7EB
Border Radius: 8px
Font: 16px Regular

Focus: Border #3B82F6, Box Shadow
Error: Border #EF4444
```

**Dropdown Select**
```css
Same as text input
Right icon: ▼ chevron
Opens menu below
```

### Cards

**Standard Card**
```css
Background: White
Border: 1px solid #E5E7EB
Border Radius: 12px
Padding: 16px
Box Shadow: 0 1px 3px rgba(0,0,0,0.1)
```

---

## Responsive Behavior

### Window Sizes

**Compact Mode (Default)**
- Width: 380px
- Optimized for quick access
- Minimal chrome
- Essential controls only

**Expanded Mode**
- Width: 800px+
- Two-column layout
- Sidebar shows recent calls
- Main area shows full contact list
- Settings in panels

### Sidebar States

**Collapsed (Compact)**
- Shows only icons
- Width: 60px
- Hover shows tooltip

**Expanded (Default)**
- Shows icons + labels
- Width: 200px

---

## Accessibility

**Keyboard Navigation**
- Tab order follows logical flow
- All controls keyboard accessible
- Escape closes dialogs/overlays
- Enter activates primary action
- Space for checkboxes/toggles

**Screen Reader Support**
- Proper ARIA labels
- Live regions for call status changes
- Descriptive button labels
- Form field associations

**Visual Accessibility**
- Sufficient color contrast (WCAG AA)
- Focus indicators on all interactive elements
- Text scalable without breaking layout
- Support for high contrast mode

---

## White-Label Customization

### Configurable Elements

**Colors**
```toml
[branding.colors]
primary = "#3B82F6"
primary_hover = "#2563EB"
background = "#FFFFFF"
text_primary = "#111827"
text_secondary = "#6B7280"
```

**Logo & Branding**
```toml
[branding]
app_name = "Rustalk"
logo_path = "./assets/logo.png"
icon_path = "./assets/icon.ico"
```

**Typography**
```toml
[branding.typography]
font_family = "Inter, system-ui, sans-serif"
heading_weight = 600
body_weight = 400
```

### Build-time Customization
- Custom installer with company branding
- Unique bundle ID
- Custom update server
- Pre-configured SIP settings

---

## Animations & Transitions

**Standard Transitions**
- Duration: 200ms
- Easing: ease-in-out
- Properties: opacity, transform, background-color

**Call State Transitions**
- Incoming call: Fade in with scale
- Call connecting: Pulse animation
- Call active: Smooth state change
- Call ended: Fade out

**Microanimations**
- Button hover: subtle scale (1.02x)
- Button press: scale down (0.98x)
- List item hover: background fade in
- Tooltip appear: fade + slide up

---

## Icon System

Using Lucide React icons or similar modern icon library:

- Home: home
- Dialer: phone
- Contacts: users
- History: clock
- Settings: settings
- Mute: mic-off
- Unmute: mic
- Hold: pause
- Keypad: grid-3x3
- End Call: phone-off
- Incoming: phone-incoming
- Outgoing: phone-outgoing
- Missed: phone-missed

---

## States & Feedback

**Loading States**
- Spinner for connecting calls
- Skeleton loaders for contact lists
- Progress bars for file operations

**Empty States**
- No contacts: "Add your first contact"
- No history: "No calls yet"
- Search no results: "No matches found"

**Error States**
- Connection failed
- Call failed
- Invalid number
- Microphone access denied

**Success States**
- Call connected
- Contact added
- Settings saved

---

## Performance Considerations

**Optimizations**
- Virtual scrolling for long lists (contacts, history)
- Lazy load contact avatars
- Debounce search input
- Memoize expensive components
- Use React.memo for list items

**Target Metrics**
- First paint: < 300ms
- Time to interactive: < 500ms
- Smooth 60fps animations
- Memory usage: < 100MB