import {
  Application,
  Context,
  Router,
} from "https://deno.land/x/oak@v11.1.0/mod.ts";

const ficRouter = new Router()
  .get("/B([:+_=-]*):id([0-9A-Z]{9})", (context) => {
    const id = `B${context.params.id.toUpperCase()}`;
    context.response.redirect(`https://amazon.com/dp/${id}`);
  })
  .get("/ROYAL([:+_=-]*):id([0-9]{5})", (context) => {
    const id = parseInt(context.params.id, 10);
    context.response.redirect(`https://royalroad.com/fiction/${id}`);
  })
  .get("/FF([:+_=-]*):id([0-9]{8})", (context) => {
    const id = parseInt(context.params.id, 10);
    context.response.redirect(`https://fanfiction.net/s/${id}`);
  })
  .get("/AO([:+_=-]*):id([0-9]{8})", (context) => {
    const id = parseInt(context.params.id, 10);
    context.response.redirect(`https://archiveofourown.org/works/${id}`);
  })
  .get("/OL([:+_=-]*):id([0-9]{8})", (context) => {
    const id = `OL${parseInt(context.params.id, 10)}W`;
    context.response.redirect(`https://openlibrary.org/works/${id}`);
  })
  .get("/WAT([:+_=-]*):id([0-9]{7})", (context) => {
    const id = parseInt(context.params.id, 10);
    context.response.redirect(`https://wattpad.com/story/${id}`);
  });

const isbnRouter = new Router()
  .get("/(01|gtin|isbn|ian|ean|upc)/:id([0-9]{8,14})", (context) => {
    const id = context.params.id;
    context.response.redirect(`https://openlibrary.org/isbn/${id}`);
  });

const debugRouter = new Router()
  .get("/:status([2345][0-9][0-9])", (context) => {
    context.response.status = parseInt(context.params.status, 10);
  });

const defaultBodies = async (
  context: Context,
  next: () => Promise<unknown>,
) => {
  console.debug(`${context.request.method} ${context.request.url} ...`);
  await next();
  const status = context.response.status;
  const useDefaultBody = !context.response.body &&
    !context.response.type &&
    ["GET", "POST"].includes(context.request.method) && (
      status > 100 && status != 204 && status != 206 && status != 304
    );
  if (useDefaultBody) {
    context.response.status = status;
    const s = (status - 200) / 399;
    const bg = ((Math.min(255, Math.max(0, Math.floor(200 * s - 5))) << 16) +
      (Math.min(255, Math.max(0, Math.floor(40 * s))) << 8) +
      (Math.min(255, Math.max(0, Math.floor(32 * (1 - s)))))).toString(16)
      .padStart(6, "0");
    context.response.type = "text/html;charset=utf-8";
    context.response.body =
      `<!doctype html><style>head,title{display:flex}html{` +
      `background:#${bg};color:#FFF;font:bold clamp(16px,50vh,25vw)sans-serif;padding:clamp(8px,20vh,10vw);` +
      `user-select:none;-webkit-text-fill-color:rgba(255,255,255,0.125);-webkit-text-stroke:max(2px,min(1vw))` +
      `}</style><title>${status}`;
  }
  console.debug(`${context.request.method} ${context.request.url} ${status}`);
};

const app = new Application()
  .use(defaultBodies)
  .use(ficRouter.routes())
  .use(isbnRouter.routes())
  .use(debugRouter.routes());

console.info(`Listening...`);
await app.listen({ port: 8000 });
