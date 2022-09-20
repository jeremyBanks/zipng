import { PageProps } from "$fresh/server.ts";
import { Component, RenderableProps } from "preact";
import { asset, Head } from "$fresh/runtime.ts";

export type Props = {
  _?: unknown;
};

export default (props: RenderableProps<Props>) => (
  <>
    <Head>
      <link rel="icon" href={asset("/icon.svg")} />
    </Head>

    <main class="flex flex-grow self-stretch items-center justify-center bg-yellow-50 color-white">
      {props.children}
    </main>
  </>
);
