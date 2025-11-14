<script lang="ts">
  import { Phone, Mail, Star, Edit, Trash2, X } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { contactsStore } from "$lib/stores/contactsStore";
  import EditContactDialog from "./EditContactDialog.svelte";
  import { useCallNavigation } from "$lib/hooks/useCallNavigation";
  import { formatPhoneNumber } from "$lib/utils";
  import type { Contact } from "$lib/stores/contactsStore";

  interface Props {
    contactId: string;
  }

  let { contactId }: Props = $props();

  // Manage visibility state internally
  let open = $state(false);

  // Fetch contact from store
  let contactData = $state<Contact | null>(null);

  // Subscribe to contact data
  $effect(() => {
    const contactStore = contactsStore.getContactById(contactId);

    const unsubscribe = contactStore.subscribe((value) => {
      contactData = value || null;
    });

    return unsubscribe;
  });

  // Use call navigation composable
  const { initiateCall } = useCallNavigation();

  let showDeleteConfirm = $state(false);
  let editDialogRef: EditContactDialog | null = null;
  let isEditDialogOpen = $state(false);

  // Expose open method
  export function openDialog() {
    open = true;
  }

  function getInitials(name: string): string {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  function getNumberTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      mobile: "Mobile",
      work: "Work",
      home: "Home",
      other: "Other",
    };
    return labels[type] || type;
  }

  function handleToggleFavorite() {
    if (contactData) {
      contactsStore.toggleFavorite(contactData.id);
    }
  }

  function handleEdit() {
    editDialogRef?.openDialog();
  }

  function handleEditDialogOpenChange(open: boolean) {
    isEditDialogOpen = open;
  }

  function handleDelete() {
    if (showDeleteConfirm && contactData) {
      contactsStore.deleteContact(contactData.id);
      open = false;
    } else {
      showDeleteConfirm = true;
    }
  }

  function handleCall(number: string) {
    initiateCall(number);
  }

  function handleClose() {
    // Don't close if edit dialog is open
    if (isEditDialogOpen) {
      return;
    }
    open = false;
    showDeleteConfirm = false;
  }
</script>

{#if open}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) {
        handleClose();
      }
    }}
    onkeydown={(e) => {
      if (e.key === "Escape") {
        handleClose();
      }
    }}
  >
    <div class="max-w-md w-full">
      {#if contactData}
        <Card>
          <CardHeader>
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div
                  class="w-12 h-12 rounded-full bg-linear-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-lg font-semibold"
                >
                  {getInitials(contactData.name)}
                </div>
                <CardTitle>{contactData.name}</CardTitle>
              </div>
              <Button
                variant="ghost"
                size="icon-sm"
                onclick={handleClose}
                aria-label="Close"
              >
                <X class="h-4 w-4" />
              </Button>
            </div>
          </CardHeader>
          <CardContent class="space-y-4">
            <!-- Phone Numbers -->
            {#each contactData.numbers as phoneNumber (phoneNumber.number)}
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <Phone class="h-5 w-5 text-gray-400" />
                  <div>
                    <div class="text-sm font-medium text-gray-900">
                      {getNumberTypeLabel(phoneNumber.type)}
                    </div>
                    <div class="text-sm text-gray-500">
                      {formatPhoneNumber(phoneNumber.number)}
                    </div>
                  </div>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={() => handleCall(phoneNumber.number)}
                >
                  <Phone class="h-4 w-4 mr-2" />
                  Call
                </Button>
              </div>
            {/each}

            <!-- Email -->
            {#if contactData.email}
              <div class="flex items-center gap-3">
                <Mail class="h-5 w-5 text-gray-400" />
                <div>
                  <div class="text-sm font-medium text-gray-900">Email</div>
                  <div class="text-sm text-gray-500">{contactData.email}</div>
                </div>
              </div>
            {/if}

            <!-- Actions -->
            <div class="flex items-center gap-2 pt-4 border-t border-gray-200">
              <Button
                variant="outline"
                onclick={handleToggleFavorite}
                class="flex-1"
              >
                <Star
                  class="h-4 w-4 mr-2 {contactData.favorite
                    ? 'text-yellow-500 fill-yellow-500'
                    : ''}"
                />
                {contactData.favorite
                  ? "Remove from Favorites"
                  : "Add to Favorites"}
              </Button>
              <Button variant="outline" onclick={handleEdit} class="flex-1">
                <Edit class="h-4 w-4 mr-2" />
                Edit
              </Button>
              <Button
                variant={showDeleteConfirm ? "destructive" : "outline"}
                onclick={handleDelete}
                class="flex-1"
              >
                <Trash2 class="h-4 w-4 mr-2" />
                {showDeleteConfirm ? "Confirm Delete" : "Delete"}
              </Button>
            </div>
          </CardContent>
        </Card>

        <!-- Edit Contact Dialog -->
        <EditContactDialog
          bind:this={editDialogRef}
          contact={contactData}
          onOpenChange={handleEditDialogOpenChange}
        />
      {:else}
        <Card>
          <CardContent class="py-12 text-center">
            <p class="text-gray-500">Loading contact...</p>
          </CardContent>
        </Card>
      {/if}
    </div>
  </div>
{/if}
