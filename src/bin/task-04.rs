const FILE: &str = include_str!("../../input/4.txt");
const ROW_LENGTH: usize = 140;
fn p1() -> usize {
    let bytes = FILE.as_bytes();
    let horizontal: usize = bytes
        .chunks(ROW_LENGTH + 1)
        .map(|c| c.windows(4).filter(|l| l == b"XMAS" || l == b"SAMX").count())
        .sum();
    // LOOKS LIKE A DIAGONAL OMG
    let vertical = (0..ROW_LENGTH)
        .map(|i| {
            bytes
                .iter()
                .skip(i)
                .step_by(ROW_LENGTH + 1)
                .copied()
                .collect::<Vec<_>>()
                .windows(4)
                .filter(|l| l == b"XMAS" || l == b"SAMX")
                .count()
        })
        .sum::<usize>();
    let diagonal = bytes
        .chunks(ROW_LENGTH + 1)
        .collect::<Vec<_>>()
        .windows(4)
        .map(|c| {
            let mut buf_lr = [0; 4];
            let mut buf_rl = [0; 4];
            (0..ROW_LENGTH - 3)
                .map(|i| {
                    buf_lr[0] = c[0][i];
                    buf_lr[1] = c[1][i + 1];
                    buf_lr[2] = c[2][i + 2];
                    buf_lr[3] = c[3][i + 3];

                    buf_rl[0] = c[0][i + 3];
                    buf_rl[1] = c[1][i + 2];
                    buf_rl[2] = c[2][i + 1];
                    buf_rl[3] = c[3][i];

                    usize::from(&buf_lr == b"XMAS" || &buf_lr == b"SAMX")
                        + usize::from(&buf_rl == b"XMAS" || &buf_rl == b"SAMX")
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    horizontal + vertical + diagonal
}

fn p2() -> usize {
    FILE.as_bytes()
        .chunks(ROW_LENGTH + 1)
        .collect::<Vec<_>>()
        .windows(3)
        .map(|c| {
            let mut buf_lr = [0; 2];
            let mut buf_rl = [0; 2];
            (1..ROW_LENGTH - 1)
                .map(|i| {
                    if c[1][i] != b'A' {
                        0
                    } else {
                        buf_lr[0] = c[0][i - 1];
                        buf_lr[1] = c[2][i + 1];
                        buf_rl[0] = c[0][i + 1];
                        buf_rl[1] = c[2][i - 1];
                        usize::from(
                            (&buf_lr == b"MS" || &buf_lr == b"SM")
                                && (&buf_rl == b"MS" || &buf_rl == b"SM"),
                        )
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("XMAS: {}", p1());
    println!("X-MAS: {}", p2());
}
