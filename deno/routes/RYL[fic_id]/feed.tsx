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
import { h, VNode } from "preact";
import * as z from "zod";
import { RenderableProps } from "https://esm.sh/v95/preact@10.11.0/src/index.d.ts";
import { string } from "https://deno.land/x/zod/mod.ts";
import { renderXml } from "../../xml/xml.ts";
import { Item, Rss } from "../../xml/rss.ts";

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

    const pageSize = 16;
    const pageCount = Math.ceil(spine.chapters.length / pageSize);
    const page = parseInt(url.searchParams.get("page") ?? "1", 10);
    if (!Number.isFinite(page) || page < 1 || page > pageCount) {
      return new Response("invalid page number", { status: 400 });
    }
    const offset = (page - 1) * pageSize;
    const chapters = spine.chapters.slice(offset, offset + pageSize);

    return renderXml(
      <Rss
        title={spine.title}
        link={`https://${url.host}/${context.params.fic_id}`}
        description="to be determined"
        author="Test Author"
      >
        {chapters.map((chapter) => (
          <Item
            pubDate={chapter.timestamp}
            title={chapter.title}
            link={`https://${url.host}/${context.params.fic_id}/${chapter.id10}`}
            enclosure={{
              type: "audio/ogg",
              "url": "https://sfic.s3.amazonaws.com/0.ogg",
            }}
          >
            {chapter.starts_with}
          </Item>
        ))}
      </Rss>,
    );
  },
};
