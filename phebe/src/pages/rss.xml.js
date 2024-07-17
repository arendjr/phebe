import rss from "@astrojs/rss";
import { getCollection } from "astro:content";
import { SITE_TITLE, SITE_DESCRIPTION } from "../consts";

export async function GET(context) {
	const today = new Date();
	const posts = await getCollection("blog");
	return rss({
		title: SITE_TITLE,
		description: SITE_DESCRIPTION,
		site: context.site,
		items: posts
			.filter((post) => post.data.pubDate < today)
			.sort((a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf())
			.map((post) => ({
				...post.data,
				link: `/blog/${post.data.pubDate.getUTCFullYear()}/${(post.data.pubDate.getUTCMonth() + 1).toString().padStart(2, "0")}/${post.slug}`,
			})),
	});
}
