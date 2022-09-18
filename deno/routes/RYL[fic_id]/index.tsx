import { Handlers, PageProps, RenderContext } from "$fresh/server.ts";
import { z } from "https://deno.land/x/zod/mod.ts";

export const config = {
  routeOverride: "/RYL:id([0-9A-Z]+)",
};

const Spine = z.object({
  id: z.number(),
  title: z.string(),
  slug: z.string(),
  chapters: z.array(z.object({
    id: z.number(),
    timestamp: z.number(),
    title: z.string(),
    slug: z.string(),
  })),
});
type Spine = z.infer<typeof Spine>;

export const handler: Handlers = {
  async GET(_request, context) {
    const spine = Spine.parse(
      JSON.parse(
        await Deno.readTextFile(
          `../data/spines/RYL${context.params.id}.json`,
        ),
      ),
    );
    return await context.render(spine);
  },
};

export default ({ data: spine }: PageProps<Spine>) => {
  return (
    <main class="mx-auto p-2">
      <h1 class="text-xl font-bold p-4">{spine.title}</h1>
      <ol>
        {spine.chapters.map((chapter) => (
          <li class="p-2">
            <a
              href={`/RYL${spine.id}/${chapter.id}`}
              class="hover:underline p-2"
            >
              {chapter.title}
            </a>
          </li>
        ))}
      </ol>
    </main>
  );
};
