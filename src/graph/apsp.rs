mod warshal_floyd {
    use cargo_snippet::snippet;

    /// ワーシャルフロイド法
    /// 
    /// 帰納法による証明:
    /// あるi,jの最短距離について考える。
    /// Gの部分集合をGとして、G_kを{0,...,k}とする。
    /// この時、G_k U {i,j}の中でmind_k(i,j)が求まったとすると、
    /// G_k+1 U {i,j}のmind_k+1(i,j)は、
    /// mind_k(i,j) or i->k+1->j のどちらかとなる。
    /// 
    /// 従って、初期値は
    /// d[i][i] = 0
    /// else = inf
    /// 
    /// 計算量 O(V^3)

    #[snippet("warshal_floyd")]
    fn warshal_floyd(d: &mut [Vec<i64>]) {
        let n = d.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                     d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }
    }
    #[test]
    fn test_warshal_floyd() {
        let dat = [
            (0,1,1),
            (0,3,2),
            (0,4,3),
            (1,2,4),
            (1,4,1),
            (2,4,1),
            (2,5,3),
            (3,4,2),
            (4,5,3),
        ];

        let inf = std::i64::MAX/2;
        let mut dist = vec![vec![inf; 6]; 6];
        for i in 0 .. 6 {
            dist[i][i] = 0;
        }
        for l in &dat {
            dist[l.0][l.1] = l.2;
            dist[l.1][l.0] = l.2;
        }

        warshal_floyd(&mut dist);

        assert_eq!(dist[0][5], 5);
        assert_eq!(dist[2][3], 3);
    }
}

