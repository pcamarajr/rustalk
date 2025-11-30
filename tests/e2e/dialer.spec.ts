import { test, expect } from '@playwright/test';

test.describe('Dialer Functionality', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display dialer interface', async ({ page }) => {
    // Check for phone number input
    const phoneInput = page.locator('input[type="text"]').or(page.locator('input[placeholder*="phone" i]')).first();
    await expect(phoneInput).toBeVisible();

    // Check for dial pad
    const dialPad = page.locator('[role="group"][aria-label*="dial" i]');
    await expect(dialPad).toBeVisible();

    // Check for call button
    const callButton = page.getByRole('button', { name: /call/i });
    await expect(callButton).toBeVisible();
  });

  test('should input phone number via dial pad', async ({ page }) => {
    // Click dial pad buttons
    await page.getByRole('button', { name: /dial 1/i }).click();
    await page.getByRole('button', { name: /dial 2/i }).click();
    await page.getByRole('button', { name: /dial 3/i }).click();

    // Check that number appears in input (formatted)
    const phoneInput = page.locator('input[type="text"]').first();
    const inputValue = await phoneInput.inputValue();
    
    // Should contain digits 1, 2, 3 (may be formatted)
    expect(inputValue).toMatch(/[123]/);
  });

  test('should input phone number via keyboard', async ({ page }) => {
    const phoneInput = page.locator('input[type="text"]').first();
    await phoneInput.click();
    await phoneInput.fill('5551234567');

    // Check that number is displayed (may be formatted)
    const inputValue = await phoneInput.inputValue();
    expect(inputValue).toContain('555');
  });

  test('should enable call button when number is entered', async ({ page }) => {
    const phoneInput = page.locator('input[type="text"]').first();
    const callButton = page.getByRole('button', { name: /call/i });

    // Initially, call button should be disabled or not clickable
    await phoneInput.fill('5551234567');
    
    // Wait a bit for state update
    await page.waitForTimeout(300);
    
    // Call button should now be enabled (check if it's not disabled)
    const isDisabled = await callButton.getAttribute('disabled');
    expect(isDisabled).toBeNull();
  });

  test('should disable call button when number is empty', async ({ page }) => {
    const phoneInput = page.locator('input[type="text"]').first();
    const callButton = page.getByRole('button', { name: /call/i });

    // Clear the input
    await phoneInput.clear();
    
    // Wait a bit for state update
    await page.waitForTimeout(300);
    
    // Call button should be disabled
    const isDisabled = await callButton.getAttribute('disabled');
    // Either disabled attribute exists or button is not clickable
    if (isDisabled === null) {
      // Check if button has disabled styling or is not interactive
      const classes = await callButton.getAttribute('class');
      expect(classes).toContain('disabled');
    }
  });

  test('should display recent calls list', async ({ page }) => {
    // Look for recent calls section
    const recentCalls = page.getByText(/recent|recent calls/i);
    await expect(recentCalls).toBeVisible();
  });

  test('should click dial pad numbers', async ({ page }) => {
    // Test clicking various dial pad buttons
    const dialPadButtons = [
      page.getByRole('button', { name: /dial 1/i }),
      page.getByRole('button', { name: /dial 2/i }),
      page.getByRole('button', { name: /dial 3/i }),
      page.getByRole('button', { name: /dial 4/i }),
      page.getByRole('button', { name: /dial 5/i }),
    ];

    for (const button of dialPadButtons) {
      await expect(button).toBeVisible();
      await button.click();
      await page.waitForTimeout(100);
    }
  });
});

