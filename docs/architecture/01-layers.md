# Architecture Layers - RUSTALK MVP

**Architecture Style:** Clean Architecture with Hexagonal Pattern
**Layers:** 5 distinct layers with clear boundaries

## Layer Overview

```
User Interaction
      ↓
┌─────────────────────────────────────┐
│  PRESENTATION LAYER                 │ ← SvelteKit UI Components
│  /src                               │
└─────────────────────────────────────┘
      ↓ Tauri IPC (invoke/events)
┌─────────────────────────────────────┐
│  IPC BOUNDARY LAYER                 │ ← Tauri Commands + Validation
│  /src-tauri/src/commands            │
└─────────────────────────────────────┘
      ↓ Service calls
┌─────────────────────────────────────┐
│  APPLICATION LAYER                  │ ← Business Logic Orchestration
│  /src-tauri/src/services            │
└─────────────────────────────────────┘
      ↓ Uses Domain + Infrastructure
┌─────────────────────────────────────┐
│  DOMAIN LAYER                       │ ← Core Entities (Pure Rust)
│  /src-tauri/src/domain              │
└─────────────────────────────────────┘
      ↓ Implemented by Infrastructure
┌─────────────────────────────────────┐
│  INFRASTRUCTURE LAYER               │ ← SIP, RTP, Audio, Storage
│  /src-tauri/src/infrastructure      │
└─────────────────────────────────────┘
      ↓
External Systems (SIP Server, Audio Devices, Keychain)
```

## 1. Presentation Layer

**Location:** `/src` (SvelteKit frontend)
**Technology:** SvelteKit + TypeScript + TailwindCSS

### Responsibilities

- Render UI components
- Handle user input
- Manage UI state via Svelte stores
- Communicate with backend via Tauri IPC
- Display real-time call status
- Provide responsive, accessible interface

### Design System and UI Guidelines

- **Styling Guidelines:** Reference [07-design-system.md](07-design-system.md) for CSS variables, components, and patterns
- **Screen Layouts:** Reference [08-ui-design.md](08-ui-design.md) for detailed UI specifications and screen layouts
- **Component Architecture:** Reference [09-islands-architecture.md](09-islands-architecture.md) for Islands Architecture pattern - **all new screens and components must follow this pattern**

### Key Components

- **Pages/Routes**: `/`, `/settings`, `/login`
- **Components**: `Dialer`, `ActiveCall`, `CallControls`, `AudioDeviceSelector`, `ContactList`
- **Stores**: `callStore`, `authStore`, `audioStore`, `settingsStore`
- **API Layer**: `callApi`, `authApi`, `audioApi` (Tauri invoke wrappers)

### Data Flow

```
User clicks "Call" button
  → callStore.initiate(number)
  → callApi.initiateCall(number)
  → Tauri invoke("initiate_call")
  → IPC Boundary Layer
```

### Testing

- **Unit Tests**: Stores, API layer, utility functions (Vitest)
- **Component Tests**: Svelte Testing Library
- **Coverage Target**: 80%+

---

## 2. IPC Boundary Layer

**Location:** `/src-tauri/src/commands` (Tauri command handlers)
**Technology:** Tauri Commands API + Tauri Events API

### Responsibilities

- Expose Tauri commands to frontend
- Validate and sanitize all inputs
- Translate errors from Rust → TypeScript
- Emit events to frontend for real-time updates
- Enforce security (no credential leakage)

### Command Structure

```rust
// Example: Tauri command
#[tauri::command]
async fn initiate_call(
    number: String,
    state: tauri::State<'_, AppState>,
) -> Result<CallId, CommandError> {
    // Input validation
    validate_phone_number(&number)?;

    // Delegate to application service
    let call_id = state
        .call_service
        .initiate_outbound_call(number)
        .await?;

    Ok(call_id)
}
```

### Key Commands

- **Auth**: `register_account`, `unregister_account`, `get_registration_status`
- **Calls**: `initiate_call`, `answer_call`, `hangup_call`, `mute_call`
- **Audio**: `list_audio_devices`, `set_audio_device`, `get_audio_levels`
- **Settings**: `save_settings`, `load_settings`

### Events (Backend → Frontend)

- `call_state_changed`: Call status updates (ringing, connected, ended)
- `incoming_call`: New inbound call notification
- `registration_state_changed`: SIP registration status
- `audio_device_changed`: Audio device selection changed

### Testing

- **Integration Tests**: Full IPC flow (invoke command → service → response)
- **Mock Backend**: Test commands with mock services
- **Error Handling**: Verify all error cases return proper TypeScript errors

---

## 3. Application Layer

**Location:** `/src-tauri/src/services` (Business logic services)
**Technology:** Rust + Tokio (async)

### Responsibilities

- Orchestrate business operations
- Manage call state machine
- Coordinate SIP sessions
- Route audio streams
- Enforce business rules
- Handle credential lifecycle

### Key Services

```
services/
├── call_service.rs        # Call lifecycle management
├── auth_service.rs        # SIP registration/authentication
├── audio_service.rs       # Audio routing and control
└── settings_service.rs    # User settings management
```

### CallService Example

```rust
pub struct CallService {
    sip_client: Arc<dyn SipClient>,
    rtp_manager: Arc<dyn RtpManager>,
    audio_engine: Arc<dyn AudioEngine>,
    state: Arc<RwLock<CallState>>,
}

impl CallService {
    pub async fn initiate_outbound_call(&self, number: String) -> Result<CallId> {
        // 1. Create call entity
        let call = Call::new_outbound(number);

        // 2. Send SIP INVITE
        self.sip_client.send_invite(&call).await?;

        // 3. Set up RTP session
        let rtp_session = self.rtp_manager.create_session(&call).await?;

        // 4. Update state
        self.state.write().await.add_call(call);

        Ok(call.id)
    }
}
```

### State Management

- Call states: `Idle`, `Ringing`, `Connecting`, `Active`, `OnHold`, `Ended`
- Registration states: `Unregistered`, `Registering`, `Registered`, `Failed`
- Thread-safe state using `Arc<RwLock<T>>`

### Testing

- **Unit Tests**: Business logic with mocked infrastructure
- **Integration Tests**: Full service flows with test doubles
- **Coverage Target**: 85%+

---

## 4. Domain Layer

**Location:** `/src-tauri/src/domain` (Core business entities)
**Technology:** Pure Rust (no external dependencies)

### Responsibilities

- Define core business entities
- Implement domain events
- Define value objects
- Specify domain interfaces (traits)
- **No external dependencies** (framework-agnostic)

### Key Entities

```
domain/
├── entities/
│   ├── call.rs            # Call entity
│   ├── contact.rs         # Contact entity
│   ├── credentials.rs     # SIP credentials value object
│   └── audio_device.rs    # Audio device entity
├── events/
│   ├── call_events.rs     # CallStarted, CallEnded, etc.
│   └── auth_events.rs     # Registered, Unregistered, etc.
└── traits/
    ├── sip_client.rs      # SipClient trait
    ├── audio_engine.rs    # AudioEngine trait
    ├── rtp_manager.rs     # RtpManager trait
    └── credential_store.rs # CredentialStore trait
```

### Example: Call Entity

```rust
#[derive(Debug, Clone)]
pub struct Call {
    pub id: CallId,
    pub direction: CallDirection,
    pub remote_number: String,
    pub state: CallState,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Call {
    pub fn new_outbound(number: String) -> Self {
        Self {
            id: CallId::generate(),
            direction: CallDirection::Outbound,
            remote_number: number,
            state: CallState::Idle,
            start_time: None,
            end_time: None,
        }
    }

    pub fn transition_to(&mut self, new_state: CallState) -> Result<()> {
        // Validate state transition rules
        self.state.can_transition_to(&new_state)?;
        self.state = new_state;
        Ok(())
    }
}
```

### Domain Traits (Dependency Inversion)

```rust
#[async_trait]
pub trait SipClient: Send + Sync {
    async fn send_invite(&self, call: &Call) -> Result<()>;
    async fn send_bye(&self, call_id: &CallId) -> Result<()>;
    async fn register(&self, credentials: &Credentials) -> Result<()>;
}
```

### Testing

- **Unit Tests**: Entity logic, state transitions
- **No Mocks Needed**: Pure business logic

---

## 5. Infrastructure Layer

**Location:** `/src-tauri/src/infrastructure` (External integrations)
**Technology:** Rust + platform-specific libraries

### Responsibilities

- Implement domain traits (SipClient, AudioEngine, etc.)
- Handle SIP protocol (rsip + Tokio)
- Manage RTP sessions (webrtc-rs)
- Platform-specific audio I/O (cpal → CoreAudio/WASAPI)
- Secure credential storage (keyring → Keychain/Credential Manager)
- TLS/SIPS (rustls)

### Module Structure

```
infrastructure/
├── sip/
│   ├── client.rs          # SipClient implementation
│   ├── transport.rs       # UDP/TCP/TLS transport
│   ├── parser.rs          # rsip message parsing
│   └── session.rs         # SIP session management
├── rtp/
│   ├── session.rs         # RTP session (webrtc-rs)
│   ├── codec.rs           # G.711 codec
│   └── jitter_buffer.rs   # Audio jitter buffer
├── audio/
│   ├── engine.rs          # AudioEngine trait impl
│   ├── macos.rs           # CoreAudio backend
│   ├── windows.rs         # WASAPI backend
│   └── device_manager.rs  # cpal device enumeration
└── storage/
    ├── credentials.rs     # CredentialStore trait impl
    ├── keychain.rs        # macOS Keychain
    └── credential_mgr.rs  # Windows Credential Manager
```

### Platform Abstraction Pattern

```rust
// Platform-specific code isolated
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacOSAudioBackend as PlatformAudioBackend;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowsAudioBackend as PlatformAudioBackend;
```

### Testing

- **Unit Tests**: SIP parsing, audio buffer logic
- **Integration Tests**: Full protocol flows with mock servers
- **Platform Tests**: Keychain/Credential Manager with test accounts

---

## Layer Dependencies

### Dependency Rules

1. **Outer layers depend on inner layers** (never the reverse)
2. **Domain layer has ZERO external dependencies**
3. **Infrastructure implements domain traits** (dependency inversion)
4. **Application orchestrates domain + infrastructure**
5. **IPC translates between Rust and TypeScript**

### Dependency Graph

```
Presentation → IPC → Application → Domain ← Infrastructure
                                      ↑
                                      └─ Traits defined here
                                         Implemented by Infrastructure
```

---

## Data Flow Example: Initiating a Call

```
1. USER CLICKS "CALL" BUTTON
   ↓
2. PRESENTATION: callStore.initiate("555-1234")
   ↓
3. PRESENTATION: callApi.initiateCall("555-1234")
   ↓ Tauri invoke
4. IPC: initiate_call(number: String) → validate input
   ↓
5. APPLICATION: call_service.initiate_outbound_call(number)
   ↓
6. APPLICATION: Create Call entity (Domain)
   ↓
7. INFRASTRUCTURE: sip_client.send_invite(&call)
   ↓
8. INFRASTRUCTURE: Send INVITE message to SIP server
   ↓
9. INFRASTRUCTURE: rtp_manager.create_session(&call)
   ↓
10. INFRASTRUCTURE: audio_engine.start_stream()
    ↓
11. APPLICATION: Update call state → Active
    ↓ Emit event
12. IPC: emit("call_state_changed", { call_id, state: "Active" })
    ↓
13. PRESENTATION: callStore receives event → update UI
    ↓
14. USER SEES "CALL ACTIVE" UI
```

---

## Benefits of This Architecture

### 1. Testability

- Each layer can be tested independently
- Domain logic tested without infrastructure
- Mock infrastructure for application tests

### 2. Platform Abstraction

- Platform-specific code isolated to infrastructure layer
- Easy to add Linux support in future

### 3. Framework Independence

- Core logic not tied to Tauri, SvelteKit, or any framework
- Can migrate frontend to React/Vue without touching business logic

### 4. Clear Boundaries

- Each layer has single responsibility
- Easy to understand and navigate codebase

### 5. Scalability

- New features added by extending services
- Infrastructure changes don't affect business logic

---

## Next Steps

1. Implement domain entities and traits (Phase 1)
2. Create infrastructure implementations (Phase 1)
3. Build application services (Phase 2-3)
4. Wire up IPC commands (Phase 2-3)
5. Implement UI components (Phase 2-4)
