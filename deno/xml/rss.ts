import { h } from "preact";
import { RenderableProps } from "https://esm.sh/v95/preact@10.11.0/src/index.d.ts";

declare global {
  namespace preact.createElement.JSX {
    interface IntrinsicElements {
      rss: preact.JSX.HTMLAttributes<HTMLElement>;
      channel: preact.JSX.HTMLAttributes<HTMLElement>;
      description: preact.JSX.HTMLAttributes<HTMLElement>;
    }
  }
}

const rfc2822DateTime = (timestamp: number | Date) => {
  const date = typeof timestamp == "number"
    ? new Date(timestamp * 1000)
    : new Date(timestamp);
  return `${
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][date.getUTCDay()]
  }, ${date.getUTCDate().toString().padStart(2, "0")} ${
    [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ][
      date.getUTCMonth()
    ]
  } ${date.getUTCFullYear()} ${
    date.getUTCHours().toString().padStart(2, "0")
  }:${date.getUTCMinutes().toString().padStart(2, "0")}:${
    date.getUTCSeconds().toString().padStart(2, "0")
  } UT`;
};

export const Rss = (
  { children, title, description, author, image }: RenderableProps<{
    title?: string;
    description?: string;
    language?: string;
    link?: string;
    author?: string;
    image?: string;
  }>,
) =>
  h(
    "rss",
    {
      "version": "2.0",
      "xmlns:content": "http://purl.org/rss/1.0/modules/content/",
      "xmlns:itunes": "http://www.itunes.com/dtds/podcast-1.0.dtd",
      "xmlns:atom": "http://www.w3.org/2005/Atom",
    },
    h(
      "channel",
      {},
      title && h("title", {}, title),
      description && h("description", {}, description),
      author && h("itunes:author", {}, author),
      image && h("itunes:image", { href: image.toString() }),
      children,
    ),
  );

export const Item = (
  { children, title, link, pubDate, enclosure }: RenderableProps<{
    title?: string;
    link?: string;
    pubDate?: number;
    enclosure?: {
      url: string;
      type: string;
      length?: number;
    };
  }>,
) =>
  h(
    "item",
    {},
    title && h("title", {}, title),
    link && h("link", {}, link),
    pubDate && h("pubDate", {}, rfc2822DateTime(pubDate)),
    enclosure && h("enclosure", enclosure),
    children && h("description", {}, children),
  );
