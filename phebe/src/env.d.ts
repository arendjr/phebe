/// <reference path="../.astro/types.d.ts" />
/// <reference types="astro/client" />

declare module "astro:content" {
	interface Frontmatter {
		title: string;
		description: string;
		date: string;
		mastodon?: { toot: string };
		github?: { url: string };
	}
}
