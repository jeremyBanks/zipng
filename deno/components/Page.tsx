import { PageProps } from "$fresh/server.ts";
import { Component, RenderableProps } from "preact";

export default (props: RenderableProps<Record<never, never>>) => (
  <div class="flex flex-grow self-stretch items-center justify-center bg-pink-50 color-white">
    <div class="flex bg-white color-black rounded-md p-4">
      {props.children}
    </div>
  </div>
);
