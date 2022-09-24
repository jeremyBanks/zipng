import {
  HandlerContext,
  Handlers,
  PageProps,
  RenderContext,
} from "$fresh/server.ts";

export const config = {
  routeOverride: "/:fic_id(RYL[0-9A-Z]{7})/feed.xml",
};
export const handler = (request: Request, context: HandlerContext) =>
  new Response(
    `<?xml version="1.0" encoding="UTF-8"?><rss
  version="2.0"
  xmlns:atom="http://www.w3.org/2005/Atom"
  xmlns:content="http://purl.org/rss/1.0/modules/content/"
  xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd"
>
  <channel>
    <title>Test Feed ${context.params.fic_id}</title>
    <description>to be erased</description>
    <itunes:author>Test Author</itunes:author>
    <itunes:image href="https://fic.is/icon.svg" />
    <language>en</language>
    <link>https://fic.is/${context.params.fic_id}</link>
    <atom:link href="https://fic.is/${context.params.fic_id}/feed.xml" rel="self" type="application/rss+xml" />
    <item>
      <title>Test Item</title>
      <description>or maybe replaced</description>
      <pubDate>Tue, 02 Oct 2020 19:45:01</pubDate>
      <enclosure url="https://sfic.s3.amazonaws.com/0.ogg" type="audio/ogg" />
      <guid>https://fic.is/${context.params.fic_id}/1</guid>
    </item>
  </channel>
</rss>`,
    {
      headers: {
        "Content-Type": "application/rss+xml",
      },
    },
  );
