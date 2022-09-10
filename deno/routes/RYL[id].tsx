import { Handlers } from "$fresh/server.ts";

export const config = {
  routeOverride: "/RYL:id([0-9A-Z]{7})",
};

export const handler: Handlers = {
  GET(_request, context) {
    const id = parseInt(context.params.id, 10);
    return Response.redirect(`https://royalroad.com/fiction/${id}`);
  },
};
