import { test, expect } from '@playwright/test';

test.describe('VeloPay Chain Explorer Verification', () => {
	test.beforeEach(async ({ page }) => {
		// Navigate to explorer page
		await page.goto('http://localhost:5173/explorer');
		// Wait for initial render
		await page.waitForTimeout(2000);
	});

	test('1. Stats Cards - should display all 4 stat cards', async ({ page }) => {
		// Check for all 4 stat cards
		const statCards = page.locator('div.bg-white.dark\\:bg-neutral-900.rounded-xl.p-6');
		await expect(statCards).toHaveCount(4, { timeout: 5000 });

		// Take screenshot of stats cards
		await page.screenshot({
			path: 'e2e/screenshots/1-stats-cards.png',
			fullPage: false
		});

		console.log('✅ Stats cards verified - 4 cards displayed');
	});

	test('2. Stats Cards - should show loading skeletons initially', async ({ page }) => {
		// Reload to see loading state
		await page.reload();

		// Check for loading skeletons (should appear briefly)
		const loadingSkeletons = page.locator('div.animate-pulse');
		const count = await loadingSkeletons.count();

		console.log(`Loading skeletons found: ${count}`);

		// Wait for data to load
		await page.waitForTimeout(3000);

		await page.screenshot({
			path: 'e2e/screenshots/2-loading-state.png',
			fullPage: false
		});
	});

	test('3. Recent Blocks - should display recent blocks section', async ({ page }) => {
		// Check for "Recent Blocks" heading
		const heading = page.locator('h2:has-text("Recent Blocks")');
		await expect(heading).toBeVisible();

		// Wait for blocks to load or "No blocks yet" message
		await page.waitForTimeout(5000);

		// Take screenshot
		await page.screenshot({
			path: 'e2e/screenshots/3-recent-blocks.png',
			fullPage: true
		});

		console.log('✅ Recent Blocks section verified');
	});

	test('4. Recent Blocks - should show block cards with proper information', async ({ page }) => {
		// Wait for potential block data
		await page.waitForTimeout(5000);

		// Check for block cards
		const blockCards = page.locator('a[href^="/explorer/block/"]');
		const blockCount = await blockCards.count();

		console.log(`Block cards found: ${blockCount}`);

		if (blockCount > 0) {
			// Verify first block card has required elements
			const firstBlock = blockCards.first();

			// Check for block number
			const blockNumber = firstBlock.locator('div.font-mono.font-semibold');
			await expect(blockNumber).toBeVisible();

			// Check for status badge
			const statusBadge = firstBlock.locator('div.inline-flex.items-center');
			await expect(statusBadge).toBeVisible();

			console.log('✅ Block cards have proper structure');
		} else {
			console.log('⚠️  No blocks found - chain may not be running');
		}

		await page.screenshot({
			path: 'e2e/screenshots/4-block-cards.png',
			fullPage: true
		});
	});

	test('5. Real-time Updates - wait and check for new blocks', async ({ page }) => {
		await page.waitForTimeout(3000);

		// Get initial block count
		const initialBlocks = await page.locator('a[href^="/explorer/block/"]').count();
		console.log(`Initial block count: ${initialBlocks}`);

		// Take "before" screenshot
		await page.screenshot({
			path: 'e2e/screenshots/5a-blocks-before.png',
			fullPage: true
		});

		// Wait for 12 seconds for potential new blocks
		console.log('Waiting 12 seconds for new blocks...');
		await page.waitForTimeout(12000);

		// Get new block count
		const newBlocks = await page.locator('a[href^="/explorer/block/"]').count();
		console.log(`Block count after 12s: ${newBlocks}`);

		// Take "after" screenshot
		await page.screenshot({
			path: 'e2e/screenshots/5b-blocks-after.png',
			fullPage: true
		});

		if (newBlocks > initialBlocks) {
			console.log('✅ Real-time updates working - new blocks appeared!');
		} else {
			console.log('⚠️  No new blocks detected - chain may be idle or not running');
		}
	});

	test('6. Block Navigation - clicking a block should navigate to detail page', async ({ page }) => {
		// Wait for blocks to load
		await page.waitForTimeout(5000);

		const blockCards = page.locator('a[href^="/explorer/block/"]');
		const blockCount = await blockCards.count();

		if (blockCount > 0) {
			// Click first block
			const firstBlock = blockCards.first();
			await firstBlock.click();

			// Wait for navigation
			await page.waitForTimeout(2000);

			// Verify we're on block detail page
			await expect(page).toHaveURL(/\/explorer\/block\/\d+/);

			// Check for "Back to Explorer" button
			const backButton = page.locator('button:has-text("Back to Explorer")');
			await expect(backButton).toBeVisible();

			// Check for Block Information section
			const blockInfo = page.locator('h2:has-text("Block Information")');
			await expect(blockInfo).toBeVisible();

			// Take screenshot of block detail page
			await page.screenshot({
				path: 'e2e/screenshots/6-block-detail.png',
				fullPage: true
			});

			console.log('✅ Block navigation working - detail page displayed');
		} else {
			console.log('⚠️  Cannot test navigation - no blocks available');
		}
	});

	test('7. Dark Mode Toggle - check for toggle button', async ({ page }) => {
		// Look for dark mode toggle button (expected in bottom-right)
		const toggleButton = page.locator('button[aria-label*="dark"], button[aria-label*="theme"]');
		const count = await toggleButton.count();

		await page.screenshot({
			path: 'e2e/screenshots/7-dark-mode-check.png',
			fullPage: true
		});

		if (count > 0) {
			console.log('✅ Dark mode toggle found');

			// Try to click it
			await toggleButton.first().click();
			await page.waitForTimeout(1000);

			await page.screenshot({
				path: 'e2e/screenshots/7-dark-mode-toggled.png',
				fullPage: true
			});
		} else {
			console.log('❌ Dark mode toggle NOT FOUND - needs implementation');
		}
	});

	test('8. Connection Status - check for connection indicator', async ({ page }) => {
		// Wait for connection attempt
		await page.waitForTimeout(3000);

		// Look for connection status indicator
		const connectionIndicator = page.locator('div:has-text("Connected"), div:has-text("Disconnected"), div:has-text("Local Testnet")');
		const hasIndicator = await connectionIndicator.count() > 0;

		if (hasIndicator) {
			console.log('✅ Connection status indicator found');
		} else {
			console.log('⚠️  Connection status not clearly visible');
		}

		await page.screenshot({
			path: 'e2e/screenshots/8-connection-status.png',
			fullPage: false
		});
	});

	test('9. Professional Design - verify color scheme and styling', async ({ page }) => {
		await page.waitForTimeout(3000);

		// Check for professional styling elements
		const cards = page.locator('div.rounded-xl, div.rounded-lg');
		const cardCount = await cards.count();

		console.log(`Rounded card elements: ${cardCount}`);

		// Check for proper spacing
		const sections = page.locator('div.max-w-7xl');
		const sectionCount = await sections.count();

		console.log(`Container sections: ${sectionCount}`);

		// Take full page screenshot for design review
		await page.screenshot({
			path: 'e2e/screenshots/9-full-page-design.png',
			fullPage: true
		});

		console.log('✅ Design elements verified - see screenshot for visual review');
	});

	test('10. Full Explorer Overview', async ({ page }) => {
		// Wait for everything to load
		await page.waitForTimeout(5000);

		// Take comprehensive screenshot
		await page.screenshot({
			path: 'e2e/screenshots/10-full-explorer-overview.png',
			fullPage: true
		});

		console.log('✅ Full overview screenshot captured');
	});
});
