type Edge = { to: number; weight: number };

function dijkstra(graph: Edge[][], start: number): number[] {
  const dist = Array(graph.length).fill(Infinity);
  const used = Array(graph.length).fill(false);
  dist[start] = 0;

  for (let i = 0; i < graph.length; i++) {
    let v = -1;
    for (let j = 0; j < graph.length; j++) {
      if (!used[j] && (v === -1 || dist[j] < dist[v])) v = j;
    }
    if (v === -1 || dist[v] === Infinity) break;
    used[v] = true;

    for (const e of graph[v]) {
      dist[e.to] = Math.min(dist[e.to], dist[v] + e.weight);
    }
  }
  return dist;
}

console.log(dijkstra([[{ to: 1, weight: 4 }], [{ to: 2, weight: 3 }], []], 0));
