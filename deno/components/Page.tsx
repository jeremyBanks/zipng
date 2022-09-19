import { PageProps } from "$fresh/server.ts";
import { Component, RenderableProps } from "preact";

export default (props: RenderableProps<Record<never, never>>) => (
  <div class="flex flex-grow self-stretch items-center justify-center bg-violet-100 color-white">
    {props.children}
  </div>
);
