<script lang="ts">
  import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Select, SelectContent, SelectItem, SelectTrigger } from '$lib/components/ui/select';
  import { authStore, isRegistering } from '$lib/stores/authStore';
  import { ChevronDown, ChevronUp, Loader2 } from 'lucide-svelte';

  // Form state
  let server = $state('');
  let port = $state('5060');
  let protocol = $state<'UDP' | 'TCP' | 'TLS'>('UDP');
  let username = $state('');
  let password = $state('');
  let contactUri = $state('');
  let expires = $state('3600');

  // UI state
  let showAdvanced = $state(false);
  let isSubmitting = $state(false);
  let errors = $state<Record<string, string>>({});
  let submitError = $state('');

  // Validation functions
  function validateServer(value: string): string | null {
    if (!value.trim()) {
      return 'Server is required';
    }
    // Basic hostname/IP validation
    const hostnameRegex = /^([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$|^(\d{1,3}\.){3}\d{1,3}$|^localhost$/;
    if (!hostnameRegex.test(value.trim())) {
      return 'Invalid hostname or IP address';
    }
    return null;
  }

  function validatePort(value: string): string | null {
    if (!value.trim()) {
      return 'Port is required';
    }
    const portNum = parseInt(value, 10);
    if (isNaN(portNum) || portNum < 1 || portNum > 65535) {
      return 'Port must be between 1 and 65535';
    }
    return null;
  }

  function validateUsername(value: string): string | null {
    if (!value.trim()) {
      return 'Username is required';
    }
    return null;
  }

  function validatePassword(value: string): string | null {
    if (!value.trim()) {
      return 'Password is required';
    }
    return null;
  }

  function validateContactUri(value: string): string | null {
    if (!value.trim()) {
      return null; // Optional field
    }
    // Basic SIP URI validation
    const sipUriRegex = /^sip:[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+(:\d+)?$/;
    if (!sipUriRegex.test(value.trim())) {
      return 'Invalid SIP URI format (e.g., sip:user@host:port)';
    }
    return null;
  }

  function validateExpires(value: string): string | null {
    if (!value.trim()) {
      return null; // Optional field
    }
    const expiresNum = parseInt(value, 10);
    if (isNaN(expiresNum) || expiresNum <= 0) {
      return 'Expires must be a positive number';
    }
    return null;
  }

  // Validate all fields
  function validateForm(): boolean {
    errors = {};
    let isValid = true;

    const serverError = validateServer(server);
    if (serverError) {
      errors.server = serverError;
      isValid = false;
    }

    const portError = validatePort(port);
    if (portError) {
      errors.port = portError;
      isValid = false;
    }

    const usernameError = validateUsername(username);
    if (usernameError) {
      errors.username = usernameError;
      isValid = false;
    }

    const passwordError = validatePassword(password);
    if (passwordError) {
      errors.password = passwordError;
      isValid = false;
    }

    if (showAdvanced) {
      const contactUriError = validateContactUri(contactUri);
      if (contactUriError) {
        errors.contactUri = contactUriError;
        isValid = false;
      }

      const expiresError = validateExpires(expires);
      if (expiresError) {
        errors.expires = expiresError;
        isValid = false;
      }
    }

    return isValid;
  }

  // Handle form submission
  async function handleSubmit() {
    submitError = '';
    
    if (!validateForm()) {
      return;
    }

    isSubmitting = true;
    console.log('DEBUG:[LOGIN_FORM/SUBMIT] Starting registration', {
      server,
      port: parseInt(port, 10),
      protocol: protocol.toLowerCase(),
      username,
      contactUri: contactUri.trim() || undefined,
      expires: expires.trim() ? parseInt(expires, 10) : undefined,
    });

    try {
      await authStore.register(
        server.trim(),
        parseInt(port, 10),
        protocol.toLowerCase() as 'udp' | 'tcp' | 'tls',
        username.trim(),
        password,
        contactUri.trim() || undefined,
        expires.trim() ? parseInt(expires, 10) : undefined
      );
      console.log('DEBUG:[LOGIN_FORM/SUBMIT] Registration initiated successfully');
      // Navigation will be handled by the page component watching registration state
    } catch (error) {
      console.error('DEBUG:[LOGIN_FORM/SUBMIT] Registration failed', error);
      submitError = error instanceof Error ? error.message : 'Registration failed. Please check your credentials and try again.';
    } finally {
      isSubmitting = false;
    }
  }

</script>

<Card class="w-full max-w-md mx-auto">
  <CardHeader>
    <CardTitle>SIP Account Registration</CardTitle>
    <CardDescription>Enter your SIP account credentials to connect</CardDescription>
  </CardHeader>

  <CardContent>
    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
      <!-- Server -->
      <div class="space-y-2">
        <Label for="server">Server *</Label>
        <Input
          id="server"
          type="text"
          bind:value={server}
          placeholder="sip.example.com"
          required
          aria-invalid={errors.server ? 'true' : undefined}
          aria-describedby={errors.server ? 'server-error' : undefined}
        />
        {#if errors.server}
          <p id="server-error" class="text-sm text-destructive" role="alert">{errors.server}</p>
        {/if}
      </div>

      <!-- Port -->
      <div class="space-y-2">
        <Label for="port">Port *</Label>
        <Input
          id="port"
          type="number"
          bind:value={port}
          placeholder="5060"
          min="1"
          max="65535"
          required
          aria-invalid={errors.port ? 'true' : undefined}
          aria-describedby={errors.port ? 'port-error' : undefined}
        />
        {#if errors.port}
          <p id="port-error" class="text-sm text-destructive" role="alert">{errors.port}</p>
        {/if}
      </div>

      <!-- Protocol -->
      <div class="space-y-2">
        <Label for="protocol">Protocol *</Label>
        <Select
          type="single"
          value={protocol}
          onValueChange={(value) => {
            if (value) {
              protocol = value as 'UDP' | 'TCP' | 'TLS';
            }
          }}
        >
          <SelectTrigger id="protocol" class="w-full">
            {protocol}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="UDP" label="UDP" />
            <SelectItem value="TCP" label="TCP" />
            <SelectItem value="TLS" label="TLS" />
          </SelectContent>
        </Select>
      </div>

      <!-- Username -->
      <div class="space-y-2">
        <Label for="username">Username *</Label>
        <Input
          id="username"
          type="text"
          bind:value={username}
          placeholder="username"
          required
          aria-invalid={errors.username ? 'true' : undefined}
          aria-describedby={errors.username ? 'username-error' : undefined}
        />
        {#if errors.username}
          <p id="username-error" class="text-sm text-destructive" role="alert">{errors.username}</p>
        {/if}
      </div>

      <!-- Password -->
      <div class="space-y-2">
        <Label for="password">Password *</Label>
        <Input
          id="password"
          type="password"
          bind:value={password}
          placeholder="password"
          required
          aria-invalid={errors.password ? 'true' : undefined}
          aria-describedby={errors.password ? 'password-error' : undefined}
        />
        {#if errors.password}
          <p id="password-error" class="text-sm text-destructive" role="alert">{errors.password}</p>
        {/if}
      </div>

      <!-- Advanced Options Toggle -->
      <div class="pt-2">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          onclick={() => {
            showAdvanced = !showAdvanced;
          }}
          class="w-full justify-between"
          aria-expanded={showAdvanced}
          aria-controls="advanced-options"
        >
          <span>Advanced Options</span>
          {#if showAdvanced}
            <ChevronUp class="h-4 w-4" />
          {:else}
            <ChevronDown class="h-4 w-4" />
          {/if}
        </Button>
      </div>

      <!-- Advanced Options -->
      {#if showAdvanced}
        <div id="advanced-options" class="space-y-4 pt-2 border-t">
          <!-- Contact URI -->
          <div class="space-y-2">
            <Label for="contact-uri">Contact URI</Label>
            <Input
              id="contact-uri"
              type="text"
              bind:value={contactUri}
              placeholder="sip:user@192.168.1.100:5060"
              aria-invalid={errors.contactUri ? 'true' : undefined}
              aria-describedby={errors.contactUri ? 'contact-uri-error' : undefined}
            />
            {#if errors.contactUri}
              <p id="contact-uri-error" class="text-sm text-destructive" role="alert">{errors.contactUri}</p>
            {/if}
            <p class="text-xs text-muted-foreground">Optional: Custom contact URI (defaults to server address)</p>
          </div>

          <!-- Expires -->
          <div class="space-y-2">
            <Label for="expires">Expires (seconds)</Label>
            <Input
              id="expires"
              type="number"
              bind:value={expires}
              placeholder="3600"
              min="1"
              aria-invalid={errors.expires ? 'true' : undefined}
              aria-describedby={errors.expires ? 'expires-error' : undefined}
            />
            {#if errors.expires}
              <p id="expires-error" class="text-sm text-destructive" role="alert">{errors.expires}</p>
            {/if}
            <p class="text-xs text-muted-foreground">Optional: Registration expiration time in seconds (default: 3600)</p>
          </div>
        </div>
      {/if}

      <!-- Submit Error -->
      {#if submitError}
        <div class="rounded-md bg-destructive/10 border border-destructive/20 p-3" role="alert">
          <p class="text-sm text-destructive">{submitError}</p>
        </div>
      {/if}

      <!-- Loading State -->
      {#if $isRegistering}
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <Loader2 class="h-4 w-4 animate-spin" />
          <span>Registering...</span>
        </div>
      {/if}
    </form>
  </CardContent>

  <CardFooter class="flex-col gap-2">
    <Button
      type="submit"
      onclick={handleSubmit}
      disabled={isSubmitting || $isRegistering}
      class="w-full"
    >
      {#if isSubmitting || $isRegistering}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        Registering...
      {:else}
        Register
      {/if}
    </Button>
  </CardFooter>
</Card>

