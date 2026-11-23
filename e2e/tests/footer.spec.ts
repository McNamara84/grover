import { expect, test, type Locator } from '@playwright/test';

const currentVersion = 'v0.1.0dev';
const footerLinks = [
  ['Accessibility', 'https://www.gfz.de/erklaerung-zur-barrierefreiheit'],
  ['Data protection', 'https://gfz-data-services.gfz.de/about-us/data-protection'],
  ['Imprint', 'https://gfz-data-services.gfz.de/about-us/legal-notice'],
  [`${currentVersion} changelog`, '/changelog'],
  ['GFZ on Instagram', 'https://www.instagram.com/gfz_potsdam/'],
  ['GFZ on YouTube', 'https://www.youtube.com/user/GFZvideos'],
  ['GFZ on Bluesky', 'https://bsky.app/profile/gfz.bsky.social'],
  ['GFZ on LinkedIn', 'https://de.linkedin.com/company/gfz-helmholtz-zentrum-fuer-geoforschung'],
] as const;

async function bounds(locator: Locator) {
  return locator.evaluate(element => {
    const { left, right, top, bottom, width, height } = element.getBoundingClientRect();
    return { left, right, top, bottom, width, height };
  });
}

test.describe('shared footer without JavaScript', () => {
  test.use({ javaScriptEnabled: false });

  test('names all eight destinations and opens the current release notes', async ({ page }) => {
    await page.goto('/');
    const footer = page.getByRole('contentinfo');
    const links = footer.getByRole('link');
    await expect(links).toHaveCount(footerLinks.length);
    for (const [index, [name, href]] of footerLinks.entries()) {
      const link = links.nth(index);
      await expect(link).toHaveAccessibleName(name);
      await expect(link).toHaveAttribute('href', href);
      await expect(link).not.toHaveAttribute('target', '_blank');
    }
    const information = footer.getByRole('navigation', { name: 'Site information' });
    const social = footer.getByRole('navigation', { name: 'GFZ social media' });
    await expect(information.getByRole('link')).toHaveCount(4);
    await expect(social.locator('a > svg[aria-hidden="true"]')).toHaveCount(4);

    const version = information.getByRole('link', { name: `${currentVersion} changelog` });
    await version.click();
    await expect(page).toHaveURL(/\/changelog$/);
    const main = page.getByRole('main');
    await expect(main.getByRole('heading', { level: 1 })).toHaveText('Release notes for GROVER');
    const currentRelease = main.getByRole('listitem').first();
    const heading = currentRelease.getByRole('heading', { level: 2 });
    await expect(heading.getByText(currentVersion, { exact: true })).toBeVisible();
    await expect(heading.getByText('Unreleased')).toBeVisible();
    const change = currentRelease.getByRole('listitem').filter({ hasText: 'Open the release notes from the version link' });
    await expect(change.getByText('Added', { exact: true })).toBeVisible();
    await expect(footer).toHaveCount(1);
  });

  test('exposes a visible keyboard focus indicator in reading order', async ({ page }) => {
    await page.goto('/');
    const footer = page.getByRole('contentinfo');
    for (const [name] of footerLinks) {
      await page.keyboard.press('Tab');
      const link = footer.getByRole('link', { name, exact: true });
      await expect(link).toBeFocused();
      await expect(link).toHaveCSS('outline-style', 'solid');
    }
  });
});

test('the changelog uses the home page text and background colors', async ({ page }) => {
  function readingColors(element: Element) {
    const text = getComputedStyle(element).color;
    for (let surface: Element | null = element; surface; surface = surface.parentElement) {
      const background = getComputedStyle(surface).backgroundColor;
      if (background !== 'rgba(0, 0, 0, 0)') {
        return { text, background };
      }
    }
    throw new Error('No page background found');
  }

  await page.emulateMedia({ colorScheme: 'light' });
  await page.goto('/');
  const home = await page.getByRole('main').getByRole('paragraph').evaluate(readingColors);
  await page.getByRole('contentinfo').getByRole('link', { name: /changelog$/ }).click();
  const heading = page.getByRole('heading', { name: 'Release notes for GROVER' });
  expect(await heading.evaluate(readingColors)).toEqual(home);
});

test('aligns the desktop footer with the reference on a short page', async ({ page }) => {
  await page.setViewportSize({ width: 1496, height: 700 });
  await page.goto('/');
  const footer = page.getByRole('contentinfo');
  const box = await bounds(footer);
  expect(box.left).toBe(0);
  expect(box.width).toBe(1496);
  expect(box.bottom).toBeCloseTo(700, 0);
  expect(box.height).toBeCloseTo(93, 0);

  const information = await bounds(footer.getByRole('navigation', { name: 'Site information' }));
  expect(information.left).toBeCloseTo(16, 0);
  const social = footer.getByRole('navigation', { name: 'GFZ social media' });
  const icons = social.getByRole('link');
  let previousRight = information.right;
  for (const icon of await icons.all()) {
    const target = await bounds(icon);
    expect(target.width).toBeGreaterThanOrEqual(44);
    expect(target.height).toBeGreaterThanOrEqual(44);
    expect(target.left).toBeGreaterThan(previousRight);
    expect(target.top + target.height / 2).toBeCloseTo(box.top + box.height / 2, 0);
    await expect(icon.locator('svg')).toBeVisible();
    previousRight = target.right;
  }
  const lastIcon = await bounds(icons.last().locator('svg'));
  expect(box.right - lastIcon.right).toBeCloseTo(46.5, 0);
});

test('stacks both link groups below the release notes on a narrow screen', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 240 });
  await page.goto('/changelog');
  const footer = page.getByRole('contentinfo');
  const information = await bounds(footer.getByRole('navigation', { name: 'Site information' }));
  const social = await bounds(footer.getByRole('navigation', { name: 'GFZ social media' }));
  expect(social.top).toBeGreaterThanOrEqual(information.bottom);
  const imprint = await bounds(footer.getByRole('link', { name: 'Imprint' }));
  const version = await bounds(footer.getByRole('link', { name: /changelog$/ }));
  expect(version.top).toBeCloseTo(imprint.top, 0);
  expect(version.left).toBeGreaterThan(imprint.right);
  expect(information.left).toBeGreaterThanOrEqual(0);
  expect(social.right).toBeLessThanOrEqual(320);
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(320);
  const lastChange = page.getByRole('main').getByRole('listitem').last();
  expect((await bounds(lastChange)).bottom).toBeLessThanOrEqual((await bounds(footer)).top);
  expect((await bounds(footer)).bottom).toBeGreaterThan(240);
});
