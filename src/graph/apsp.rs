mod warshal_froid {
    // dist could be negative as well as bellman ford
    fn warshal_froid(d: &mut [Vec<i64>]) {
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
    fn test() {
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

        let INF = 1000000;
        let mut dist = vec![vec![INF; 6]; 6];
        for i in 0 .. 6 {
            dist[i][i] = 0;
        }
        for l in &dat {
            dist[l.0][l.1] = l.2;
            dist[l.1][l.0] = l.2;
        }

        warshal_froid(&mut dist);

        assert_eq!(dist[0][5], 5);
        assert_eq!(dist[2][3], 3);
    }
}
