<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import LoginForm from '$lib/components/auth/LoginForm.svelte';
  import { authStore, registrationState } from '$lib/stores/authStore';

  // Check registration status on mount and attempt auto-login
  onMount(async () => {
    try {
      // Check current registration status from backend
      await authStore.getRegistrationStatus();
      
      // Check if already registered
      if ($registrationState === 'registered') {
        console.log('DEBUG:[LOGIN/NAVIGATION] Already registered, redirecting to dialer');
        goto('/');
        return;
      }
      
      // Attempt auto-login if not already registered
      console.log('DEBUG:[LOGIN/AUTO_LOGIN] Attempting auto-login with saved credentials');
      const autoLoginSuccess = await authStore.autoLogin();
      
      if (autoLoginSuccess) {
        console.log('DEBUG:[LOGIN/AUTO_LOGIN] Auto-login initiated, waiting for registration');
        // Registration state polling will handle the redirect when registration completes
      } else {
        console.log('DEBUG:[LOGIN/AUTO_LOGIN] No saved credentials or auto-login failed, showing login form');
      }
    } catch (error) {
      console.error('DEBUG:[LOGIN/NAVIGATION] Error during auto-login check', error);
      // Continue to show login form even if check fails
    }
  });

  // Watch for registration state changes and redirect when registered
  $effect(() => {
    if ($registrationState === 'registered') {
      console.log('DEBUG:[LOGIN/NAVIGATION] Registration successful, redirecting to dialer');
      goto('/');
    }
  });
</script>

<div class="container mx-auto max-w-md px-4 py-6">
  <LoginForm />
</div>

