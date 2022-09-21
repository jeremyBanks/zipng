import { Handlers, PageProps, RenderContext } from "$fresh/server.ts";
import { z } from "zod";
import Page from "../../components/Page.tsx";
import { css, tw } from "twind/css";
import { Head, IS_BROWSER } from "$fresh/runtime.ts";

const { DOMParser } = IS_BROWSER
  ? globalThis
  : await import("deno-dom") as unknown as typeof globalThis;

export const config = {
  routeOverride: "/:fic_id(RYL[0-9A-Z]{7})",
};

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
  async GET(_request, context) {
    const spine = Spine.parse(
      JSON.parse(
        await Deno.readTextFile(
          `../data/spines/${context.params.fic_id}.json`,
        ),
      ),
    );
    return await context.render(spine);
  },
};

const clean = (s: string) =>
  z.string().parse(
    (new DOMParser())
      .parseFromString(
        s,
        "text/html",
      ).body.textContent?.trim() ?? "",
  );

export default ({ data: spine }: PageProps<Spine>) => {
  return (
    <Page>
      <Head>
        <title>{spine.title}</title>
      </Head>
      <main class="p-10 text-lg bg-white lg:w-192">
        <h1 class="text-xl font-bold p-4">{spine.title}</h1>
        <ol class="list-decimal ml-4">
          {spine.chapters.map((chapter) => {
            const chapter_prefix_pattern =
              /^\s*ch(?:ap(?:t(?:er)?)?)?\b\s*(\d+)\b[\s\-\:]*/i;
            const match = chapter_prefix_pattern.exec(
              chapter.title,
            );
            let chapter_number: number | undefined;
            if (match) {
              chapter_number = z.number().parse(parseInt(match[1], 10));
              chapter.title = chapter.title.replace(chapter_prefix_pattern, "");
            }

            return (
              <li class="pb-4" value={chapter_number}>
                <a
                  href={`/${spine.id10}/${chapter.id10}`}
                  class={tw(css({
                    "&": {
                      display: "block",
                      overflow: "clip",
                      contain: "content",
                      maxWidth: "100%",
                      whiteSpace: "nowrap",
                    },
                    "& em": {
                      color: "transparent",
                      textShadow: "0 0 5px rgba(0, 0, 0, 0.5)",
                    },
                    "&:hover em": {
                      color: "inherit",
                      textShadow: "none",
                    },
                    "& em *": { "display": "inline" },
                  }))}
                >
                  <strong>{chapter.title}</strong>{" "}
                  <em
                    dangerouslySetInnerHTML={{
                      __html: clean(chapter.starts_with),
                    }}
                  />
                </a>
              </li>
            );
          })}
        </ol>
      </main>
    </Page>
  );
};
