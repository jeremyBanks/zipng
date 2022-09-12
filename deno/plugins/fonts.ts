import { Plugin } from "$fresh/server.ts";
import { asset } from "$fresh/runtime.ts";
import {
  PluginRenderContext,
  PluginRenderResult,
} from "https://deno.land/x/fresh@1.1.0/src/server/types.ts";

export const name = "fonts";

export const render = (context: PluginRenderContext): PluginRenderResult => {
  context.render();
  return {
    styles: [{
      cssText: `
body {
    font-family: "Atkinson Hyperlegible", sans-serif;
}
@font-face {
    font-family: "Atkinson Hyperlegible";
    font-style: normal;
    font-weight: 400;
    font-display: optional;
    src: url(${asset("/fonts/sans400.woff2")}) format("woff2");
}
@font-face {
    font-family: "Atkinson Hyperlegible";
    font-style: italic;
    font-weight: 400;
    font-display: optional;
    src: url(${asset("/fonts/sans401.woff2")}) format("woff2");
}
@font-face {
    font-family: "Atkinson Hyperlegible";
    font-style: normal;
    font-weight: 700;
    font-display: optional;
    src: url(${asset("/fonts/sans700.woff2")}) format("woff2");
}
@font-face {
    font-family: "Atkinson Hyperlegible";
    font-style: italic;
    font-weight: 700;
    font-display: optional;
    src: url(${asset("/fonts/sans701.woff2")}) format("woff2");
}
code, pre {
    font-family: "JetBrains Mono", monospace;
    font-variant-ligatures: discretionary-ligatures;
}
@font-face {
    font-family: "JetBrains Mono";
    font-style: normal;
    font-weight: 400;
    font-display: optional;
    src: url(${asset("/fonts/mono400.woff2")}) format("woff2");
}
@font-face {
    font-family: "JetBrains Mono";
    font-style: italic;
    font-weight: 400;
    font-display: optional;
    src: url(${asset("/fonts/mono401.woff2")}) format("woff2");
}
@font-face {
    font-family: "JetBrains Mono";
    font-style: normal;
    font-weight: 700;
    font-display: optional;
    src: url(${asset("/fonts/mono700.woff2")}) format("woff2");
}
@font-face {
    font-family: "JetBrains Mono";
    font-style: italic;
    font-weight: 700;
    font-display: optional;
    src: url(${asset("/fonts/mono701.woff2")}) format("woff2");
}
        `,
    }],
  };
};

export default {
  name,
  render,
} as Plugin;
