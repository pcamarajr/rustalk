import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('should navigate to all main routes', async ({ page }) => {
    // Start at home page
    await page.goto('/');
    
    // Check that we're on the home page (dialer)
    await expect(page).toHaveURL('/');

    // Navigate to contacts
    await page.getByRole('button', { name: /contacts/i }).click();
    await expect(page).toHaveURL('/contacts');
    await expect(page.getByRole('heading', { name: /contacts/i })).toBeVisible();

    // Navigate to history
    await page.getByRole('button', { name: /history/i }).click();
    await expect(page).toHaveURL('/history');
    await expect(page.getByRole('heading', { name: /history/i })).toBeVisible();

    // Navigate to settings
    await page.getByRole('button', { name: /settings/i }).click();
    await expect(page).toHaveURL('/settings');
    await expect(page.getByRole('heading', { name: /settings/i })).toBeVisible();

    // Navigate back to home
    await page.getByRole('button', { name: /home/i }).click();
    await expect(page).toHaveURL('/');
  });

  test('should highlight active navigation item', async ({ page }) => {
    await page.goto('/');

    // Check that Home is active
    const homeButton = page.getByRole('button', { name: /home/i });
    await expect(homeButton).toHaveClass(/bg-blue-50|text-blue-600/);

    // Navigate to settings
    await page.getByRole('button', { name: /settings/i }).click();
    
    // Check that Settings is now active
    const settingsButton = page.getByRole('button', { name: /settings/i });
    await expect(settingsButton).toHaveClass(/bg-blue-50|text-blue-600/);
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page).toHaveURL('/login');
    await expect(page.getByRole('heading', { name: /SIP Account Registration/i })).toBeVisible();
  });

  test('should have sidebar navigation visible on all pages', async ({ page }) => {
    const routes = ['/', '/contacts', '/history', '/settings'];
    
    for (const route of routes) {
      await page.goto(route);
      
      // Check sidebar is visible
      const sidebar = page.locator('aside[role="navigation"]');
      await expect(sidebar).toBeVisible();
      
      // Check navigation buttons are visible
      await expect(page.getByRole('button', { name: /home/i })).toBeVisible();
      await expect(page.getByRole('button', { name: /contacts/i })).toBeVisible();
      await expect(page.getByRole('button', { name: /history/i })).toBeVisible();
      await expect(page.getByRole('button', { name: /settings/i })).toBeVisible();
    }
  });
});

