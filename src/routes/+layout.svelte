<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Header from '$lib/components/Header.svelte';
  import { authStore } from '$lib/stores/authStore';
  import { initializeCallStateListener, cleanupCallStateListener } from '$lib/stores/callStore';

  // Check registration status on mount (non-blocking)
  // Note: Registration is optional for now - users can use the app without registration
  // This is a soft check that doesn't block navigation
  onMount(() => {
    // Initialize call state event listener
    initializeCallStateListener().catch((error) => {
      console.error('DEBUG:[LAYOUT/EVENTS] Error initializing call state listener', error);
      // Continue normally even if listener fails
    });

    // Only check if not already on login page
    if ($page.url.pathname !== '/login') {
      authStore.getRegistrationStatus().catch((error) => {
        console.error('DEBUG:[LAYOUT/REGISTRATION] Error checking registration status', error);
        // Continue normally even if check fails
      });
    }

    // Cleanup on unmount
    return () => {
      cleanupCallStateListener();
    };
  });
</script>

<div class="flex flex-col h-screen bg-gray-50">
  <!-- Header -->
  <Header />

  <!-- Main Layout: Sidebar + Content -->
  <div class="flex flex-1 overflow-hidden">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Content Area -->
    <main class="flex-1 overflow-y-auto">
      <slot />
    </main>
  </div>
</div>

