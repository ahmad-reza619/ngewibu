const playwright = require("playwright");

(async () => {
  const browser = await playwright.chromium.launch({
    headless: false,
    slowMo: 100,
  });
  const context = await browser.newContext();
  const page = await context.newPage();

  // Navigate to the website
  await page.goto(
    "https://otakudesu.cam/episode/jjtsu-ksn-s2-episode-18-sub-indo/",
  );

  await page.waitForTimeout(6000);

  await page
    .locator("body > div.box_item_ads_popup > div > div", { hasText: "Close" })
    .click();
  await page.getByRole("Video").click();

  await browser.close();
})();
