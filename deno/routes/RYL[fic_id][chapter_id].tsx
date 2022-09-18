import { Handlers, PageProps, RenderContext } from "$fresh/server.ts";
import { z } from "https://deno.land/x/zod/mod.ts";

export const config = {
  routeOverride: "/RYL:id([0-9A-Z]+)/:chapter_id([0-9]+)",
};

const Chapter = z.object({
  id: z.number(),
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
          `../target/chapters/RYL${context.params.chapter_id}.json`,
        ),
      ),
    );
    return await context.render(chapter);
  },
};

export default ({ data: chapter }: PageProps<Chapter>) => {
  return (
    <main class="mx-auto p-2">
      <h1 class="text-xl font-bold p-4">{chapter.title}</h1>
      <style>
        {`
        p {
          text-indent: 1em;
          margin: 1em;
        }
        `}
      </style>
      <div class="p-2" dangerouslySetInnerHTML={{ __html: chapter.html }} />
    </main>
  );
};
