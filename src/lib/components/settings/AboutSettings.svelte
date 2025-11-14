<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
</script>

<Card>
  <CardHeader>
    <CardTitle>About</CardTitle>
    <CardDescription>Application information</CardDescription>
  </CardHeader>
  <CardContent class="space-y-2">
    <div class="flex items-center justify-between py-2">
      <span class="text-sm text-gray-600">Version:</span>
      <span class="text-sm font-medium text-gray-900">0.1.0</span>
    </div>
    <div class="flex items-center justify-between py-2">
      <span class="text-sm text-gray-600">Build:</span>
      <span class="text-sm font-medium text-gray-900">Development</span>
    </div>
  </CardContent>
</Card>

