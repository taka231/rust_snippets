use cargo_snippet::snippet;

#[snippet]
fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, v: usize) {
    visited[v] = true;
    for &next_v in &graph[v] {
        if visited[next_v] {
            continue;
        }
        dfs(graph, visited, next_v);
    }
}

/*
    graph : 隣接リスト
    graph[i][j] := (iと隣接しているj個目の頂点, 距離)
*/
#[snippet]
fn dijkstra(graph: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    let n = graph.len();
    let mut pq = std::collections::BinaryHeap::new();
    let mut dist = vec![1u64 << 60; n];
    //距離の小さい順に取りたいので(距離,頂点)のペアでBinaryHeapに突っ込む
    pq.push(std::cmp::Reverse((0, start)));
    dist[start] = 0;
    while let Some(v) = pq.pop() {
        let (cost, now) = v.0;
        //取り出したものが最短距離でないときに無視
        if dist[now] < cost {
            continue;
        }
        for (to, c) in &graph[now] {
            //すでにdistに入っている距離を更新できる場合はBinaryHeapに突っ込んでdistを更新
            if dist[*to] > *c + dist[now] {
                dist[*to] = dist[now] + *c;
                pq.push(std::cmp::Reverse((dist[*to], *to)));
            }
        }
    }
    dist
}
