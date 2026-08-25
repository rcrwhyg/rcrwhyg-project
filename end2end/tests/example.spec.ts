import { test, expect } from "@playwright/test";

// Scaffold only — update selectors/titles to match the live site before relying on CI.
// See docs/testing.md.

test("homepage brand title is present", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle(/如春日午后阳光/);
  await expect(page.getByRole("link", { name: "如春日午后阳光" })).toBeVisible();
});
