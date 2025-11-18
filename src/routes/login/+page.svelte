<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import LoginForm from '$lib/components/auth/LoginForm.svelte';
  import { authStore, registrationState } from '$lib/stores/authStore';

  // Check registration status on mount and redirect if already registered
  onMount(async () => {
    try {
      // Check current registration status from backend
      await authStore.getRegistrationStatus();
      
      // Check if already registered
      if ($registrationState === 'registered') {
        console.log('DEBUG:[LOGIN/NAVIGATION] Already registered, redirecting to dialer');
        goto('/');
      }
    } catch (error) {
      console.error('DEBUG:[LOGIN/NAVIGATION] Error checking registration status', error);
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

