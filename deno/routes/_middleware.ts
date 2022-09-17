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

  if (security !== "none" && url.protocol !== "https:") {
    url.protocol = "https:";
    return new Response(null, {
      status: 308,
      headers: {
        Location: url.toString(),
        "Strict-Transport-Security": security === "preload"
          ? "max-age=63072000; includeSubDomains; preload"
          : "max-age=14400",
      },
    });
  }

  const response = await context.next();

  if (security === "preload") {
    response.headers.set(
      "Strict-Transport-Security",
      "max-age=63072000; includeSubDomains; preload",
    );
  }

  response.headers.append(
    "Content-Security-Policy",
    `script-src 'self'`,
  );

  return response;
};
