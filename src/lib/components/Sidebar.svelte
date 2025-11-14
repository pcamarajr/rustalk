<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { Home, Phone, Users, Clock, Settings } from "lucide-svelte";

  // Navigation items configuration
  const navItems = [
    { path: "/", label: "Home", icon: Home },
    { path: "/contacts", label: "Contacts", icon: Users },
    { path: "/history", label: "History", icon: Clock },
    { path: "/settings", label: "Settings", icon: Settings },
  ];

  // Get current pathname from page store
  let currentPath = $derived($page.url.pathname);

  // Check if a nav item is active
  function isActive(path: string): boolean {
    // Home and Dialer both use "/" so we need special handling
    if (path === "/") {
      return currentPath === "/";
    }
    return currentPath.startsWith(path);
  }

  function handleNavigation(path: string) {
    console.log("DEBUG:[SIDEBAR/NAVIGATION] Navigating to:", path);
    goto(path);
  }
</script>

<aside
  class="w-[200px] bg-white border-r border-gray-200 flex flex-col py-4"
  role="navigation"
  aria-label="Main navigation"
>
  <nav class="flex flex-col gap-1 px-2">
    {#each navItems as item}
      {@const Icon = item.icon}
      {@const active = isActive(item.path)}
      <button
        type="button"
        onclick={() => handleNavigation(item.path)}
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 {active
          ? 'bg-blue-50 text-blue-600'
          : 'text-gray-700 hover:bg-gray-50'}"
        aria-label={item.label}
        aria-current={active ? "page" : undefined}
      >
        <Icon class="h-5 w-5 shrink-0" />
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>
</aside>

<style>
  /* Additional styles if needed */
</style>
