import { Handler, PageProps } from "$fresh/server.ts";

export const config = {
  routeOverride: "/(01|gtin|isbn|ean|ian|upc)/:gtin([0-9]{7,13}[0-9xX]{1})",
};

export const handler: Handler = (_request, context) => {
  return new Response(`hello ${context.params.gtin}`);
};
