#import "../lib.typ": djvu-pages, djvu-find

#let kailath = read(
  "/references/Kailath T., Sayed A., Hassibi B. - Linear Estimation.djvu",
  encoding: none,
)
#let pages = djvu-pages(kailath)
#let body = pages.slice(25)

#let queries = (
  ("E.4.3", "Unique Stabilizing Solution"),
  ("Theorem E.6.2", "Positive Definite Solution"),
  ("Theorem 14.5.1", "Sufficiency"),
)

#let results = queries.map(q => {
  let hit = djvu-find(body, ..q)
  (query: q, hit: hit)
})

#context [
  #metadata(results)<bench>
]
