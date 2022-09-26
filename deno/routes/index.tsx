import { PageProps } from "$fresh/server.ts";
import Page from "~/components/Page.tsx";

export default (props: PageProps) => (
  <Page>
    <main className="container mx-auto py-sm px-xs text-amber-100">
      <h1 className="text-2xl font-bold mb-8 mt-16">
        Welcome
      </h1>
      <p className="my-8">
        Welcome <i>to</i> <b>your</b>{" "}
        <i>
          <b>life</b>
        </i>.
      </p>
      <p className="my-8">
        There's no turning back.
      </p>
      <pre className="my-8 p-4 bg-black text-white"><code>{`fn foo() {
const bar = "baz";
}`}</code></pre>
      <p className="my-8">
        We will <code>=&lt; find &gt;=</code> you
      </p>
    </main>
  </Page>
);
