fn main() {
    {
        let mut recipes = vec![3, 7];
        let mut p1 = 0;
        let mut p2 = 1;
        let input = 147061;
        while recipes.len() < input + 10 {
            let mut score = recipes[p1] + recipes[p2];
            if score >= 10 {
                recipes.push(score / 10);
                score %= 10;
            }
            recipes.push(score);
            p1 = (p1 + 1 + recipes[p1]) % recipes.len();
            p2 = (p2 + 1 + recipes[p2]) % recipes.len();
        }
        println!("{:?}", &recipes[input..input+10]);
    }

    {
        let mut recipes = vec![3, 7];
        let mut p1 = 0;
        let mut p2 = 1;
        let input = &[1,4,7,0,6,1];
        let ans;
        loop {
            let mut score = recipes[p1] + recipes[p2];
            let mut double = false;
            if score >= 10 {
                double = true;
                recipes.push(score / 10);
                score %= 10;
            }
            recipes.push(score);
            p1 = (p1 + 1 + recipes[p1]) % recipes.len();
            p2 = (p2 + 1 + recipes[p2]) % recipes.len();
            let rl = recipes.len();
            let il = input.len();
            if rl > il {
                if &recipes[rl-il..rl] == input {
                    ans = rl-il;
                    break;
                }
            }
            if double && rl > il+1 {
                if &recipes[rl-1-il..rl-1] == input {
                    ans = rl-1-il;
                    break;
                }
            }
        }
        println!("{:?}", ans);
    }
}
