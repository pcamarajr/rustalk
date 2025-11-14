<script lang="ts">
  import { Plus, X } from "lucide-svelte";
  import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
  } from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import { contactsStore } from "$lib/stores/contactsStore";
  import type { PhoneNumber } from "$lib/stores/contactsStore";
  import type { Snippet } from "svelte";

  interface Props {
    trigger?: Snippet;
  }

  let { trigger }: Props = $props();

  // Manage dialog open state internally
  let open = $state(false);

  let name = $state("");
  let email = $state("");
  let phoneNumbers = $state<
    Array<{ type: PhoneNumber["type"]; number: string }>
  >([{ type: "mobile", number: "" }]);

  function addPhoneNumber() {
    phoneNumbers = [...phoneNumbers, { type: "mobile", number: "" }];
  }

  function removePhoneNumber(index: number) {
    phoneNumbers = phoneNumbers.filter((_, i) => i !== index);
  }

  function updatePhoneNumber(
    index: number,
    field: "type" | "number",
    value: string
  ) {
    phoneNumbers = phoneNumbers.map((pn, i) =>
      i === index ? { ...pn, [field]: value } : pn
    );
  }

  function getPhoneTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      mobile: "Mobile",
      work: "Work",
      home: "Home",
      other: "Other",
    };
    return labels[type] || "Type";
  }

  function handleSubmit() {
    // Validation
    if (!name.trim()) {
      console.log("DEBUG:[CONTACTS/ADD] Name is required");
      return;
    }

    const validNumbers = phoneNumbers
      .map((pn) => ({
        type: pn.type,
        number: pn.number.replace(/\D/g, ""), // Strip non-digits
      }))
      .filter((pn) => pn.number.length > 0);

    if (validNumbers.length === 0) {
      console.log("DEBUG:[CONTACTS/ADD] At least one phone number is required");
      return;
    }

    // Add contact
    contactsStore.addContact({
      name: name.trim(),
      email: email.trim() || undefined,
      numbers: validNumbers,
      favorite: false,
    });

    // Reset form
    name = "";
    email = "";
    phoneNumbers = [{ type: "mobile", number: "" }];

    // Close dialog
    open = false;
  }

  function handleCancel() {
    // Reset form
    name = "";
    email = "";
    phoneNumbers = [{ type: "mobile", number: "" }];
    open = false;
  }

  // Reset form when dialog opens
  $effect(() => {
    if (open) {
      name = "";
      email = "";
      phoneNumbers = [{ type: "mobile", number: "" }];
    }
  });
</script>

<Dialog bind:open>
  {#if trigger}
    <DialogTrigger>
      {@render trigger()}
    </DialogTrigger>
  {:else}
    <DialogTrigger>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        Add Contact
      </Button>
    </DialogTrigger>
  {/if}
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>Add Contact</DialogTitle>
      <DialogDescription
        >Add a new contact to your address book.</DialogDescription
      >
    </DialogHeader>

    <div class="space-y-4 py-4">
      <!-- Name -->
      <div class="space-y-2">
        <Label for="name">Name *</Label>
        <Input
          id="name"
          type="text"
          bind:value={name}
          placeholder="John Doe"
          required
        />
      </div>

      <!-- Phone Numbers -->
      <div class="space-y-2">
        <Label>Phone Numbers *</Label>
        {#each phoneNumbers as phoneNumber, index (index)}
          <div class="flex gap-2">
            <Select
              type="single"
              value={phoneNumber.type}
              onValueChange={(value) => updatePhoneNumber(index, "type", value)}
            >
              <SelectTrigger class="w-32">
                {getPhoneTypeLabel(phoneNumber.type)}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="mobile">Mobile</SelectItem>
                <SelectItem value="work">Work</SelectItem>
                <SelectItem value="home">Home</SelectItem>
                <SelectItem value="other">Other</SelectItem>
              </SelectContent>
            </Select>
            <Input
              type="tel"
              bind:value={phoneNumbers[index].number}
              placeholder="+1 (555) 123-4567"
              class="flex-1"
            />
            {#if phoneNumbers.length > 1}
              <Button
                variant="ghost"
                size="icon"
                onclick={() => removePhoneNumber(index)}
                aria-label="Remove phone number"
              >
                <X class="h-4 w-4" />
              </Button>
            {/if}
          </div>
        {/each}
        <Button
          variant="outline"
          size="sm"
          onclick={addPhoneNumber}
          class="w-full"
        >
          <Plus class="h-4 w-4 mr-2" />
          Add Phone Number
        </Button>
      </div>

      <!-- Email -->
      <div class="space-y-2">
        <Label for="email">Email</Label>
        <Input
          id="email"
          type="email"
          bind:value={email}
          placeholder="john@example.com"
        />
      </div>
    </div>

    <DialogFooter>
      <Button variant="outline" onclick={handleCancel}>Cancel</Button>
      <Button onclick={handleSubmit}>Add Contact</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
