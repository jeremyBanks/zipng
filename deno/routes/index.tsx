import { PageProps } from "https://deno.land/x/fresh/server.ts";

export default function IndexPage(props: PageProps) {
  return (
    <main style={{ padding: 24 }}>
      <p>
        Welcome <i>to</i> <b>your</b>{" "}
        <i>
          <b>life</b>
        </i>.
      </p>
      <p>
        There's no turning back.
      </p>
      <pre>Even while you sleep != !== ===</pre>
      <p>
        We will <code>=&lt; find &gt;=</code> you
      </p>
    </main>
  );
}
