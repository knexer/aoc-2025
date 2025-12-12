pub fn part2(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = parse_input(input);

    // thoughts
    // it's a snek!
    // it alternates horizontal and vertical lines

    // don't know which side is inside or outside
    // could switch from point-first representation to line-segment-first
    // start anywhere on the edge, step in until you reach a corner or a point on a line
    // now we know if it's clockwise or anticlockwise winding; flip the list to canonicalize?

    // another way to test... there should be an odd number of line segments on either side of the point
    // so much simpler!
    // this is because it starts outside, and each segment flips inside <-> outside

    // can order the line segments along the x and y axes, then linear intersection scans are cheap

    // Okay, so the linear scan thing is important, let's build that data structure
    let (horizontal_lines, vertical_lines) = green_lines(red_tiles.clone());

    // Then we still check each pair of corners
    let mut corner_pairs: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            corner_pairs.push((red_tiles[i], red_tiles[j]));
        }
    }

    corner_pairs
        .iter()
        .filter(|&&corner_pair| is_valid_rect(corner_pair, &horizontal_lines, &vertical_lines))
        .map(|corner_pair| area(corner_pair.0, corner_pair.1))
        .max()
        .unwrap()
}

fn is_valid_rect(
    rect: ((i64, i64), (i64, i64)),
    horizontal_lines: &[GreenLine],
    vertical_lines: &[GreenLine],
) -> bool {
    // For each, we check if the cross section crosses any of perpendicular edges in the snek
    let rect_h_span = (rect.0 .0.min(rect.1 .0), rect.0 .0.max(rect.1 .0));
    let rect_v_span = (rect.0 .1.min(rect.1 .1), rect.0 .1.max(rect.1 .1));
    let horizontal_line_intersects = horizontal_lines
        .iter()
        .filter(|line| line.depth > rect_v_span.0 && line.depth < rect_v_span.1)
        .any(|line| line.span.0 < rect_h_span.1 && line.span.1 > rect_h_span.0);
    let vertical_line_intersects = vertical_lines
        .iter()
        .filter(|line| line.depth > rect_h_span.0 && line.depth < rect_h_span.1)
        .any(|line| line.span.0 < rect_v_span.1 && line.span.1 > rect_v_span.1);

    // Also we check if it's on the interior. Could combine those checks maybe? Probably not.
    let middle = ((rect.0 .0 + rect.1 .0) / 2, (rect.0 .1 + rect.1 .1) / 2);
    let is_interior = is_interior(middle, horizontal_lines, vertical_lines);

    is_interior && !horizontal_line_intersects && !vertical_line_intersects
}

fn is_interior(
    point: (i64, i64),
    horizontal_lines: &[GreenLine],
    vertical_lines: &[GreenLine],
) -> bool {
    let horizontally_included = horizontal_lines
        .iter()
        .take_while(|line| line.depth < point.1)
        .filter(|line| line.span.0 <= point.0 && line.span.1 > point.0)
        .count()
        % 2
        == 1;

    let vertically_included = vertical_lines
        .iter()
        .take_while(|line| line.depth < point.0)
        .filter(|line| line.span.0 <= point.1 && line.span.1 > point.1)
        .count()
        % 2
        == 1;

    let on_horizontal_edge = horizontal_lines
        .iter()
        .any(|line| line.depth == point.1 && line.span.0 <= point.0 && line.span.1 >= point.0);
    let on_vertical_edge = vertical_lines
        .iter()
        .any(|line| line.depth == point.0 && line.span.0 <= point.1 && line.span.1 >= point.1);

    (horizontally_included && vertically_included) || on_horizontal_edge || on_vertical_edge
}

fn area((x_1, y_1): (i64, i64), (x_2, y_2): (i64, i64)) -> i64 {
    ((x_2 - x_1).abs() + 1) * ((y_2 - y_1).abs() + 1)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct GreenLine {
    orientation: Orientation,
    depth: i64,
    span: (i64, i64),
}

impl GreenLine {
    fn new(p1: (i64, i64), p2: (i64, i64)) -> GreenLine {
        if p1.0 == p2.0 {
            GreenLine {
                orientation: Orientation::Vertical,
                depth: p1.0,
                span: (p1.1.min(p2.1), p1.1.max(p2.1)),
            }
        } else {
            GreenLine {
                orientation: Orientation::Horizontal,
                depth: p1.1,
                span: (p1.0.min(p2.0), p1.0.max(p2.0)),
            }
        }
    }
}

fn green_lines(red_tiles: Vec<(i64, i64)>) -> (Vec<GreenLine>, Vec<GreenLine>) {
    let mut all_green_lines: Vec<GreenLine> = (0..red_tiles.len())
        .map(|i| {
            let first = red_tiles[i];
            let second = if i + 1 < red_tiles.len() {
                red_tiles[i + 1]
            } else {
                red_tiles[0]
            };
            GreenLine::new(first, second)
        })
        .collect();
    all_green_lines.sort();

    (
        all_green_lines
            .iter()
            .take_while(|line| line.orientation == Orientation::Horizontal)
            .copied()
            .collect(),
        all_green_lines
            .iter()
            .skip_while(|line| line.orientation == Orientation::Horizontal)
            .copied()
            .collect(),
    )
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x_str, y_str)| (x_str.parse().unwrap(), y_str.parse().unwrap()))
        .collect()
}
