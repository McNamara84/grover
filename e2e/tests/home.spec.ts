import { expect, test } from '@playwright/test';

const notice = 'This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!';

test.describe('server-rendered home page', () => {
  test.use({ javaScriptEnabled: false });

  test('shows the development notice without JavaScript', async ({ page }) => {
    const response = await page.goto('/');
    expect(response?.status()).toBe(200);
    await expect(page).toHaveTitle('GROVER');
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    const paragraph = page.getByRole('main').getByRole('paragraph');
    await expect(paragraph).toBeVisible();
    await expect(paragraph).toHaveText(notice);
  });
});

test('hydrates the page and handles browser history', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => {
    if (message.type() === 'error' || (message.type() === 'warning' && /hydrat/i.test(message.text()))) {
      errors.push(message.text());
    }
  });

  // The static notice is already visible during SSR. History becomes usable during hydration.
  await page.addInitScript(() => {
    const register = window.addEventListener;
    window.addEventListener = function (...args: Parameters<typeof register>) {
      register.apply(this, args);
      if (args[0] === 'popstate') {
        performance.mark('grover-history-ready');
        window.addEventListener = register;
      }
    };
  });

  const wasm = page.waitForResponse(response => new URL(response.url()).pathname.endsWith('.wasm'));
  const style = page.waitForResponse(response => response.request().resourceType() === 'stylesheet');
  await page.goto('/');
  const bundle = await wasm;
  expect(bundle.status()).toBe(200);
  expect(bundle.headers()['content-type']).toContain('application/wasm');
  expect((await style).ok()).toBe(true);
  await page.waitForFunction(() => performance.getEntriesByName('grover-history-ready').length > 0);

  // Seed two entries in this document so that Back and Forward exercise the client-side router.
  await page.evaluate(() => {
    history.replaceState(null, '', '/browser-test-missing-page');
    history.pushState(null, '', '/');
  });
  await page.goBack();
  await expect(page.getByRole('main')).toHaveText('Page not found.');
  await page.goForward();
  const paragraph = page.getByRole('main').getByRole('paragraph');
  await expect(paragraph).toBeVisible();
  await expect(paragraph).toHaveText(notice);
  expect(errors).toEqual([]);
});
