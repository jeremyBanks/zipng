import {
  Application,
  Context,
  Router,
  STATUS_TEXT,
} from "https://deno.land/x/oak@v11.1.0/mod.ts";

const ficRouter = new Router()
  .get("/N([:+_=-]*):id([0-9]{9})", ({ params, response }) => {
    // XXX: this is missing the check digit
    const id = parseInt(params.id, 10).toString(10).padStart(9, "0");
    response.redirect(`https://openlibrary.org/isbn/${id}`);
  })
  .get("/B([:+_=-]*):id([0-9A-Z]{9})", ({ params, response }) => {
    const id = `B${params.id.toUpperCase()}`;
    response.redirect(`https://read.amazon.com/?asin=${id}`);
  })
  .get("/RYL([:+_=-]*):id([0-9]{7})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://royalroad.com/fiction/${id}`);
  })
  .get("/FF([:+_=-]*):id([0-9]{8})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://fanfiction.net/s/${id}`);
  })
  .get("/FP([:+_=-]*):id([0-9]{8})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://fictionpress.com/s/${id}`);
  })
  .get("/AO([:+_=-]*):id([0-9]{8})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://archiveofourown.org/works/${id}`);
  })
  .get("/OL([:+_=-]*):id([0-9]{8})", ({ params, response }) => {
    const id = `OL${parseInt(params.id, 10)}W`;
    response.redirect(`https://openlibrary.org/works/${id}`);
  })
  .get("/WAT([:+_=-]*):id([0-9]{7})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://wattpad.com/story/${id}`);
  })
  .get("/WN([:+_=-]*):id([0-9]{9})", ({ params, response }) => {
    const id = parseInt(params.id, 10);
    response.redirect(`https://webnovel.com/book/${id}`);
  })
  .get("/AFF([:+_=-]*):id([0-9]{8})", ({ response }) => {
    // adult-fanfiction.org
    response.status = 403;
  })
  .get("/ERO([:+_=-]*):id([0-9]{7})", ({ response }) => {
    // literotica.com
    response.status = 403;
  });

const examplesRouter = new Router()
  .get("/", ({ response }) => response.redirect("/FF________"))
  .get("/AO(_{0,8})", ({ response }) => response.redirect("/AO05627803"))
  .get("/B(_{0,9})", ({ response }) => response.redirect("/B09T7ZN7NC"))
  .get("/FF(_{0,8})", ({ response }) => response.redirect("/FF10360716"))
  .get("/FP(_{0,8})", ({ response }) => response.redirect("/FP03248665"))
  .get("/N(_{0,9})", ({ response }) => response.redirect("/N076532635"))
  .get("/OL(_{0,8})", ({ response }) => response.redirect("/OL3418158"))
  .get("/ROYAL(_{0,5})", ({ response }) => response.redirect("/ROYAL25137"));

const isbnRouter = new Router()
  .get(
    "/(01|gtin|isbn|ian|ean|upc)/:id([0-9]{8,14})",
    ({ params: { id }, response }) => response.redirect(`/N${id}`),
  );

const debugRouter = new Router()
  .get("/:status([0-9]{3})", ({ params, response }) => {
    const status = parseInt(params.status, 10);
    if (
      status >= 200 && status != 204 && status != 205 && status != 206 &&
      status != 208 && status != 304 && status <= 599
    ) {
      response.status = status;
    }
  });

const defaultBodies = async (
  { request, response }: Context,
  next: () => Promise<unknown>,
) => {
  console.debug(`${request.method} ${request.url} ...`);
  await next();
  const status = response.status;
  const text = STATUS_TEXT[status] ?? "Error";
  const useDefaultBody = !response.body &&
    !response.type &&
    ["GET", "POST"].includes(request.method) && (
      status >= 200 && status != 204 && status != 205 && status != 206 &&
      status != 208 && status != 304 && status <= 599
    );
  if (useDefaultBody) {
    response.status = status;
    const s = (status - 200) / 399;
    const bg = ((Math.min(255, Math.max(0, Math.floor(200 * s - 5))) << 16) +
      (Math.min(255, Math.max(0, Math.floor(40 * s))) << 8) +
      (Math.min(255, Math.max(0, Math.floor(32 * (1 - s)))))).toString(16)
      .padStart(6, "0");
    response.type = "text/html;charset=utf-8";
    response.body = `<!doctype html><style>head,title{display:flex}html{` +
      `background:#${bg};color:#FFF;font:bold clamp(16px,25vh,10vw)sans-serif;` +
      `padding:clamp(8px,10vh,5vw);user-select:none;-webkit-text-fill-color:` +
      `rgba(255,255,255,0.125);-webkit-text-stroke:max(2px,min(0.30vw))` +
      `}</style><title>${status}: ${text}`;
  }
  console.debug(`${request.method} ${request.url} ${status}`);
};

const app = new Application()
  .use(defaultBodies)
  .use(ficRouter.routes())
  .use(isbnRouter.routes())
  .use(examplesRouter.routes())
  .use(debugRouter.routes());

console.info(`Listening...`);
await app.listen({ port: 8000 });
