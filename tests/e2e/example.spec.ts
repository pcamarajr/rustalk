import { test, expect } from '@playwright/test';

test('homepage loads', async ({ page }) => {
  await page.goto('/');

  // Check that RUSTALK branding is visible
  await expect(page.locator('h1')).toContainText('RUSTALK');

  // Check that the description is present
  await expect(page.locator('.description')).toContainText('VoIP Desktop Application');
});
