use std::io;
use std::io::Read;

const MAX_HAILSTONES: usize = 300;
type Vec3 = (f64, f64, f64);
type Plane = (Vec3, f64);
type Hailstone = (Vec3, Vec3);

fn read_num(input: &mut &[u8]) -> f64 {
    let neg = input[0] == b'-';
    if neg { *input = &input[1..] }
    let mut x = 0;
    let mut count = 0;
    while let [d @ b'0'..=b'9', ..] = &input[count..] {
        x = 10 * x + (d - b'0') as i64;
        count += 1;
    }
    if count == 0 { panic!("bad u64") }
    *input = &input[count..];
    return (if neg { -x } else { x }) as f64;
}

fn read_input<'a>(hailstones: &'a mut [Hailstone]) -> &'a [Hailstone] {
    let mut buffer = [0; 20 * 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len == 0 || buffer[len - 1] != b'\n' { panic!("bad input") }
    let mut input = &buffer[0..len];
    let mut num_hailstones = 0;
    while input.len() > 0 {
        let x = read_num(&mut input);
        input = input.strip_prefix(b", ").unwrap();
        let y = read_num(&mut input);
        input = input.strip_prefix(b", ").unwrap();
        let z = read_num(&mut input);
        input = input.strip_prefix(b" @ ").unwrap();
        let dx = read_num(&mut input);
        input = input.strip_prefix(b", ").unwrap();
        let dy = read_num(&mut input);
        input = input.strip_prefix(b", ").unwrap();
        let dz = read_num(&mut input);
        input = input.strip_prefix(b"\n").unwrap();
        hailstones[num_hailstones] = ((x, y, z), (dx, dy, dz));
        num_hailstones += 1;
    }
    return &hailstones[0..num_hailstones];
}

fn part1(hailstones: &[Hailstone]) -> u64 {
    let mut total = 0;
    for i in 0..hailstones.len() {
        let ((ax, ay, _), (vax, vay, _)) = hailstones[i];
        for j in i + 1..hailstones.len() {
            let ((bx, by, _), (vbx, vby, _)) = hailstones[j];
            // We are looking for some `ta and `tb` such that `a + va * ta = b + vb * tb`.
            // Rearranging, this gives us the set of simultaneous equations:
            //
            //   |vax -vbx| . |ta| = |bx-ax|
            //   |vay -vby|   |tb|   |by-ay|
            //
            // We can solve for `ta` and `tb` by inverting the velocity matrix:
            //
            //   |ta| = |vax -vbx|^-1 . |bx-ax| = 1/det . |-vby vbx| . |bx-ax|
            //   |tb|   |vay -vby|      |by-ay|           |-vay vax|   |by-ay|
            let det = vay * vbx - vax * vby;
            if det == 0.0 { continue }
            let ta = (-vby * (bx - ax) + vbx * (by - ay)) / det;
            let tb = (-vay * (bx - ax) + vax * (by - ay)) / det;
            // We're only interested in future solutions, i.e. `ta > 0` and `tb > 0`.
            if ta < 0.0 || tb < 0.0 { continue }
            // We're only interested in crossings that fall within the range [2e14, 4e14]^2.
            let x = ax + vax * ta;
            let y = ay + vay * ta;
            if 2e14 <= x && x <= 4e14 && 2e14 <= y && y <= 4e14 { total += 1 }
        }
    }
    return total;
}

fn part2(hailstones: &[Hailstone]) -> u64 {
    // Two objects will collide if their relative position vector is parallel to their relative
    // velocity vector and point in opposite directions. Our goal is to throw a rock which hits
    // every hailstone, so we know that the velocity of the stone relative to each hailstone must be
    // parallel to the rock's offset from that hailstone.
    //
    // Find p, v, ti for all i such that:
    //
    //   pi + vi * ti = p + v * ti for all i
    //
    // Suppose we hit hailstone x0 at t0, and hailstone x1 at t1, where t1 > t0. Then:
    //
    //   * The collision with x0 happens at position p0 + v0 * t0
    //   * The collision with x1 happens at position p1 + v1 * t1
    //   * The delta between these positions is p1 + v1 * t1 - p0 - v0 * t0, which is equal to
    //     (t1 - t0) * v.
    //
    // This gives us:
    //
    //   v = (p1 + v1 * t1 - p0 - v0 * t0) / (t1 - t0)
    //     = a * i + b * j + v1
    //   where a = p1 - p0
    //         i = 1 / (t1 - t0)
    //         b = v1 - v0
    //         j = t0 / (t1 - t0)
    //
    // So the set of suitable velocities forms a plane found by ranging over i and j. This can be
    // re-expressed as:
    //
    //   n . x = d
    //   where n = a x b = (p1 - p0) x (v1 - v0)
    //         d = n . v1
    //
    // By intersecting this plane with two others, we can find the unique velocity which satisfies
    // all three constraints.
    let [a, b, c, ..] = hailstones else { panic!("need at least 3 hailstones") };
    let velocity = intersect(plane(*a, *b), plane(*b, *c), plane(*a, *c));
    // Once we know the stone's velocity, we can find the starting position from two hailstones by
    // finding their collision times:
    //
    // Find t0, t1 such that:
    //
    //   (a + va * ta) + (b * (tb - ta)) = (b + vb * tb)
    //   ^x0 position    ^stone movement   ^x1 position
    //
    // This rearranges to:
    //
    //   b - a = (va - v) * ta + (v - vb) * tb
    //
    // Which gives:
    //
    //   |px qx 0| . |ta| = |dx|
    //   |py qy 0|   |tb|   |dy|
    //   | 0  0 1|   | 1|   | 1|
    //   where d = b - a
    //         p = va - v
    //         q = v - vb
    let d = vsub(b.0, a.0);
    let (px, py, _) = vsub(a.1, velocity);
    let (qx, qy, _) = vsub(velocity, b.1);
    let (ta, _, _) = solve((px, py, 0.0), (qx, qy, 0.0), (0.0, 0.0, 1.0), d);
    // Knowing ta, we can derive the initial position of the stone:
    //
    //   p + ta * v = a + ta * va
    //   p = a + ta * (va - v)
    let position = vadd(a.0, vmul(ta, vsub(a.1, velocity)));
    return (position.0 + position.1 + position.2) as u64;
}

fn vadd((ax, ay, az): Vec3, (bx, by, bz): Vec3) -> Vec3 { (ax + bx, ay + by, az + bz) }
fn vsub((ax, ay, az): Vec3, (bx, by, bz): Vec3) -> Vec3 { (ax - bx, ay - by, az - bz) }
fn vmul(k: f64, (bx, by, bz): Vec3) -> Vec3 { (k * bx, k * by, k * bz) }
fn vdot((ax, ay, az): Vec3, (bx, by, bz): Vec3) -> f64 { ax * bx + ay * by + az * bz }

fn vcross((ax, ay, az): Vec3, (bx, by, bz): Vec3) -> Vec3 {
    return (ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx);
}

fn plane((a, va): Hailstone, (b, vb): Hailstone) -> Plane {
    let n = vcross(vsub(a, b), vsub(va, vb));
    let d = vdot(vb, n);
    return (n, d);
}

fn solve(c0: Vec3, c1: Vec3, c2: Vec3, x: Vec3) -> Vec3 {
    let det = vdot(c0, vcross(c1, c2));
    if det == 0.0 { panic!("unsolvable") }
    return (vdot(x, vcross(c1, c2)) / det,
            vdot(x, vcross(c2, c0)) / det,
            vdot(x, vcross(c0, c1)) / det);
}

fn intersect((r0, x0): Plane, (r1, x1): Plane, (r2, x2): Plane) -> Vec3 {
    let (c0, c1, c2) = ((r0.0, r1.0, r2.0), (r0.1, r1.1, r2.1), (r0.2, r1.2, r2.2));
    let x = (x0, x1, x2);
    return solve(c0, c1, c2, x);
}

fn main() {
    let mut hailstone_buffer = [((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)); MAX_HAILSTONES];
    let hailstones = read_input(&mut hailstone_buffer);

    print!("{}\n{}\n", part1(hailstones), part2(hailstones));
}
