import { PageProps } from "https://deno.land/x/fresh/server.ts";

export default function IndexPage(props: PageProps) {
  return (
    <main className="container mx-auto p-8 text-amber-100">
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
  );
}
