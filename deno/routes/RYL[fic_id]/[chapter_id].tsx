import { Handlers, PageProps, RenderContext } from "$fresh/server.ts";
import { z } from "https://deno.land/x/zod/mod.ts";

import { apply, css, tw } from "twind/css";
import Page from "../../components/Page.tsx";

export const config = {
  routeOverride: "/RYL:id([0-9A-Z]{7})/C:chapter_id([0-9A-Z]{9})",
};

const Spine = z.object({
  id: z.number(),
  id10: z.string(),
  title: z.string(),
  slug: z.string(),
  chapters: z.array(z.object({
    id: z.number(),
    id10: z.string(),
    timestamp: z.number(),
    title: z.string(),
    slug: z.string(),
  })),
});
type Spine = z.infer<typeof Spine>;

const Chapter = z.object({
  id: z.number(),
  id10: z.string(),
  timestamp: z.number(),
  title: z.string(),
  slug: z.string(),
  html: z.string(),
});
type Chapter = z.infer<typeof Chapter>;

export const handler: Handlers = {
  async GET(_request, context) {
    const chapter = Chapter.parse(
      JSON.parse(
        await Deno.readTextFile(
          `../target/chapters/C${context.params.chapter_id}.json`,
        ),
      ),
    );
    return await context.render(chapter);
  },
};

export default ({ data: chapter }: PageProps<Chapter>) => (
  <Page>
    <main
      class={tw`block h-full w-full overflow-y-auto ${
        css({
          "&": {
            "scroll-snap-type": "y mandatory",
          },
        })
      }`}
    >
      <h1 class="text-xl font-bold mt-4 border-b-4 border-color-blue-50">
        {chapter.title}
      </h1>
      <div
        class={tw`w-96 text-lg ${
          css({
            "& p": css`text-indent: .5rem; scroll-snap-align: center; ${
              apply("my-2 cursor-pointer hover:bg-blue-50")
            }`,
          })
        }`}
        dangerouslySetInnerHTML={{ __html: chapter.html }}
      />
    </main>
  </Page>
);
