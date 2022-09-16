import { MiddlewareHandler } from "$fresh/server.ts";

type HostSecurityMode = "none" | "strict" | "preload";
const defaultSecurity = "strict";
const hostsSecurity: Record<string, HostSecurityMode> = {
  localhost: "none",
  "127.0.0.1": "none",
  "0.0.0.0": "none",
  "fic.is": "preload",
};

export const handler: MiddlewareHandler = async (request, context) => {
  const url = new URL(request.url);
  const security: HostSecurityMode = hostsSecurity[url.hostname] ??
    defaultSecurity;
  if (security === "none") {
    return context.next();
  } else if (url.protocol !== "https:") {
    url.protocol = "https:";
    return new Response(null, {
      status: 301,
      headers: {
        Location: url.toString(),
        "Strict-Transport-Security": security === "preload"
          // if we're preloading, use the recommended two year max-age
          ? "max-age=63072000; includeSubDomains; preload"
          // if not, we set it to a short four hours (just this session)
          : "max-age=14400",
      },
    });
  } else if (security === "preload") {
    const response = await context.next();
    response.headers.set(
      "Strict-Transport-Security",
      "max-age=63072000; includeSubDomains; preload",
    );
    return response;
  } else {
    return context.next();
  }
};
