import { test, expect } from '@playwright/test';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
  });

  test('should display settings page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /settings/i })).toBeVisible();
    await expect(page.getByText(/manage your account and application preferences/i)).toBeVisible();
  });

  test('should display audio settings section', async ({ page }) => {
    // Check for Audio Settings card
    await expect(page.getByText(/audio settings/i)).toBeVisible();
    await expect(page.getByText(/configure your audio input, output, and ringtone preferences/i)).toBeVisible();
  });

  test('should display microphone selection', async ({ page }) => {
    // Look for microphone label
    const microphoneLabel = page.getByText(/microphone/i);
    await expect(microphoneLabel).toBeVisible();

    // Check for microphone select dropdown
    const microphoneSelect = page.locator('button').filter({ hasText: /select microphone|loading devices/i });
    await expect(microphoneSelect.first()).toBeVisible();
  });

  test('should display speaker selection', async ({ page }) => {
    // Look for speaker label
    const speakerLabel = page.getByText(/speaker/i);
    await expect(speakerLabel).toBeVisible();

    // Check for speaker select dropdown
    const speakerSelect = page.locator('button').filter({ hasText: /select speaker|loading devices/i });
    await expect(speakerSelect.first()).toBeVisible();
  });

  test('should allow interaction with audio device selectors', async ({ page }) => {
    // Wait for device loading to complete
    await page.waitForTimeout(1000);

    // Try to click on microphone selector
    const microphoneSelect = page.locator('button').filter({ hasText: /select microphone|loading devices/i }).first();
    
    if (await microphoneSelect.isVisible() && !(await microphoneSelect.textContent())?.includes('Loading')) {
      await microphoneSelect.click();
      // Check if dropdown opens (may be empty if no devices)
      await page.waitForTimeout(500);
    }

    // Try to click on speaker selector
    const speakerSelect = page.locator('button').filter({ hasText: /select speaker|loading devices/i }).first();
    
    if (await speakerSelect.isVisible() && !(await speakerSelect.textContent())?.includes('Loading')) {
      await speakerSelect.click();
      // Check if dropdown opens (may be empty if no devices)
      await page.waitForTimeout(500);
    }
  });

  test('should display account settings section', async ({ page }) => {
    // Scroll to find account settings
    await page.evaluate(() => window.scrollTo(0, 0));
    
    // Look for account-related content
    const accountSection = page.getByText(/account/i).first();
    await expect(accountSection).toBeVisible();
  });

  test('should display SIP account settings', async ({ page }) => {
    // Look for SIP account settings
    const sipSection = page.getByText(/SIP|sip account/i).first();
    await expect(sipSection).toBeVisible();
  });
});

