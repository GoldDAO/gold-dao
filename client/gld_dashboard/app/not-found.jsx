import Link from "next/link";

const NotFound = () => {
  return (
    <main className="flex flex-col items-center gap-2">
      <h2 className="text-4xl font-bold">
        <span className="font-extrabold">404</span> - Not Found
      </h2>
      <p>Could not find requested resource.</p>
      <Link href="/" className="underline">
        Return Home
      </Link>
    </main>
  );
};

export default NotFound;
