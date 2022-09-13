import { Options } from "$fresh/plugins/twind.ts";
import { Plugin } from "$fresh/server.ts";
import { asset } from "$fresh/runtime.ts";
import * as colors from "twind/colors";
import { apply } from "twind/css#css_directive";

export default {
  selfURL: import.meta.url,
  theme: {
    extend: {
      colors,
      fontFamily: {
        mono: ["'JetBrains Mono'", "monospace"],
        sans: ["'Atkinson Hyperlegible'", "sans-serif"],
      },
    },
  },
  preflight: (preflight) => ({
    ...preflight,
    html: apply`bg-coolGray-900 text-amber font-sans`,
    code: apply`font-mono`,
    pre: apply`font-mono`,
    "@font-face": [
      {
        fontFamily: "Atkinson Hyperlegible",
        fontStyle: "normal",
        fontWeight: 400,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/sans400.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "Atkinson Hyperlegible",
        fontStyle: "italic",
        fontWeight: 400,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/sans401.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "Atkinson Hyperlegible",
        fontStyle: "normal",
        fontWeight: 700,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/sans700.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "Atkinson Hyperlegible",
        fontStyle: "italic",
        fontWeight: 700,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/sans701.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "JetBrains Mono",
        fontStyle: "normal",
        fontWeight: 400,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/mono400.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "JetBrains Mono",
        fontStyle: "italic",
        fontWeight: 400,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/mono401.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "JetBrains Mono",
        fontStyle: "normal",
        fontWeight: 700,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/mono700.woff2")}) format("woff2")`,
      },
      {
        fontFamily: "JetBrains Mono",
        fontStyle: "italic",
        fontWeight: 700,
        fontDisplay: "fallback",
        src: `url(${asset("/fonts/mono701.woff2")}) format("woff2")`,
      },
    ],
  }),
} as Options;
