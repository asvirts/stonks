export default async function Home() {
  const res = await fetch("https://api.polygon.io/v2/aggs/ticker")
  let data = await res.json()
  const nd = JSON.stringify(data)

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1>Geaux</h1>
      <p>{nd}</p>
    </main>
  )
}
