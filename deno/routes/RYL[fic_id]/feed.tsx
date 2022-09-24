import {
  HandlerContext,
  Handlers,
  PageProps,
  RenderContext,
} from "$fresh/server.ts";

export const config = {
  routeOverride: "/:fic_id(RYL[0-9A-Z]{7})/feed.xml",
};

import render from "preact-render-to-string";
import * as fakeDom from "deno-dom";
import { IS_BROWSER } from "$fresh/runtime.ts";
import { h } from "preact";
import * as z from "zod";
import { RenderableProps } from "https://esm.sh/v95/preact@10.11.0/src/index.d.ts";
import { string } from "https://deno.land/x/zod/mod.ts";

const { DOMParser } = IS_BROWSER
  ? globalThis
  : fakeDom as unknown as typeof globalThis;

const Spine = z.object({
  id10: z.string(),
  title: z.string(),
  length: z.number(),
  chapters: z.array(z.object({
    id10: z.string(),
    timestamp: z.number(),
    title: z.string(),
    length: z.number(),
    starts_with: z.string(),
  })),
});
type Spine = z.infer<typeof Spine>;

const Rss = (props: RenderableProps<{}>) => {
};

declare global {
  namespace preact.createElement.JSX {
    interface IntrinsicElements {
      rss: preact.JSX.HTMLAttributes<HTMLElement> & {
        version: "2.0";
      };
    }
  }
}

export const handler: Handlers = {
  async GET(request, context) {
    const url = new URL(request.url);

    const spine = Spine.parse(
      JSON.parse(
        await Deno.readTextFile(
          `../data/spines/${context.params.fic_id}.json`,
        ),
      ),
    );

    const after = url.searchParams.get("after") ?? "";
    const chapters = spine.chapters.filter((c) => c.id10 > after);

    const pageSizeFloorChars = 128 * 1024;

    const xml = `<?xml version="1.0" encoding="UTF-8"?>${
      render(
        <rss
          version="2.0"
          {...{
            "xmlns:content": "http://purl.org/rss/1.0/modules/content/",
            "xmlns:itunes": "http://www.itunes.com/dtds/podcast-1.0.dtd",
            "xmlns:atom": "http://www.w3.org/2005/Atom",
          }}
        >
          {h("atom:link", {
            children: [
              <b></b>,
            ],
          })}
          "hello"
        </rss>,
        {
          xml: true,
        },
      ).replace(/\sxmlns\-(\w+)="http/g, ` xmlns:$1="http`)
    }`;

    /*
      `\
<?xml
  version="1.0"
  encoding="UTF-8"
?><rss
  version="2.0"
  xmlns:atom="http://www.w3.org/2005/Atom"
  xmlns:content="http://purl.org/rss/1.0/modules/content/"
  xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd"
><channel>
  <title>${spine.title}</title>
  <description>to be determined</description>
  <itunes:author>Test Author</itunes:author>
  <itunes:image href="https://${url.host}/icon.svg" />
  <language>en</language>
  <link>https://${url.host}/${context.params.fic_id}</link>
  <item>
    <title><![CDATA[${spine.chapters[0].title}]]></title>
    <description><![CDATA[${spine.chapters[0].starts_with}]]></description>
    <pubDate><![CDATA[${
        new Date(spine.chapters[0].timestamp * 1000).toISOString()
      }]]></pubDate>
    <enclosure url="https://sfic.s3.amazonaws.com/0.ogg" type="audio/ogg" />
    <guid>https://${url.host}/${context.params.fic_id}/1</guid>
  </item>
</channel></rss>`,*/

    return new Response(
      xml,
      {
        headers: {
          "Content-Type": "application/rss+xml",
        },
      },
    );
  },
};
