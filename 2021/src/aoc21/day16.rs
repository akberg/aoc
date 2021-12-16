static DAY: i32 = 16;

#[derive(Clone, PartialEq, Debug)]
enum Operator { Sum,Product,Min,Max,Gt,Lt,Eq, }
#[derive(Clone, PartialEq, Debug)]
enum PacketType {
    Literal(u64),
    Operator(Operator, Vec<Box<Packet>>),
}

#[derive(Clone, PartialEq, Debug)]
struct Packet {
    version: u8,
    typeid: u8,
    payload: PacketType,
}

impl Packet {
    /// Create Packet from char stream
    pub fn from_stream(chars: &mut std::str::Chars<'_>) -> Packet {
        Self::sub_packet(chars).1
    }

    /// Return A sub packet and its size
    fn sub_packet(chars: &mut std::str::Chars<'_>) -> (usize, Packet) {
        let version = u8::from_str_radix(&chars.take(3).collect::<String>(), 2).unwrap();
        let typeid  = u8::from_str_radix(&chars.take(3).collect::<String>(), 2).unwrap();
        let mut pkt_size = 6;

        let payload = match typeid {
            4 => {  // Literal value
                /* While msb != 0, shift in 4 lsb of next 5 bits */
                let mut v = 0;
                
                while let Some('1') = chars.next() {
                    let next = chars.take(4).collect::<String>();
                    v = (v<<4) + u64::from_str_radix(&next, 2).unwrap();
                    pkt_size += 5;
                }
                let next = chars.take(4).collect::<String>();
                v = (v<<4) + u64::from_str_radix(&next, 2).unwrap();
                pkt_size += 5;

                PacketType::Literal(v)
            },
            op => { // Operator
                let mut pkts = Vec::new();
                match chars.next().unwrap() {
                    '0' => {    // Next 15 bits contain the total length of subpackets
                        let length = usize::from_str_radix(&chars.take(15).collect::<String>(), 2)
                                            .unwrap();
                        pkt_size += 16;
                        let mut sub_pkt_size = 0;
                        /* Parse packets until their accumulated size is `length` */
                        while sub_pkt_size < length {
                            let (sz, pkt) = Self::sub_packet(chars);
                            pkts.push(Box::new(pkt));
                            pkt_size += sz;
                            sub_pkt_size += sz;
                        }
                    },
                    '1' => {    // Next 11 bits contain the total number of subpackets
                        let num = usize::from_str_radix(&chars.take(11).collect::<String>(), 2)
                                         .unwrap();
                        /* Parse the number of packets and update packet size */
                        pkt_size += 12 + (0..num).fold(0, |acc,_| {
                            let (sz, pkt) = Self::sub_packet(chars);
                            pkts.push(Box::new(pkt));
                            acc + sz
                        });
                    },
                    _ => unreachable!()
                }
                /* Set operator */
                let op = match op {
                    0 => Operator::Sum,
                    1 => Operator::Product,
                    2 => Operator::Min,
                    3 => Operator::Max,
                    5 => Operator::Gt,
                    6 => Operator::Lt,
                    7 => Operator::Eq,
                    _ => unreachable!(),
                };
                PacketType::Operator(op, pkts)
            },
        };
        (pkt_size, Packet { version, typeid, payload })
    }

    /// Compute sum of version numbers
    pub fn sum_version(&self) -> u64 {
        self.version as u64 + match &self.payload {
            PacketType::Literal(_) => 0,
            PacketType::Operator(_, v) => v.iter().map(|pkt| pkt.sum_version()).sum::<u64>(),
        }
    }

    /// Compute value of packet
    pub fn compute(pkt: &Box<Self>) -> u64 {
        match &pkt.payload {
            PacketType::Literal(x) => *x,
            PacketType::Operator(op, v) => match *op {
                Operator::Sum => v.iter().map(Self::compute).sum::<u64>(),
                Operator::Product => v.iter().map(Self::compute).product::<u64>(),
                Operator::Min => v.iter().map(Self::compute).min().unwrap(),
                Operator::Max => v.iter().map(Self::compute).max().unwrap(),
                Operator::Gt => if Self::compute(&v[0]) > Self::compute(&v[1]) { 1 } else { 0 },
                Operator::Lt => if Self::compute(&v[0]) < Self::compute(&v[1]) { 1 } else { 0 },
                Operator::Eq => if Self::compute(&v[0]) == Self::compute(&v[1]) { 1 } else { 0 },
                
            },
        }
    }
}

/// Parse hex line input a bitstream
fn parse_line(line: &str) -> String {
    line.chars()
    .flat_map(
        |c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap())
        .chars()
        .collect::<Vec<_>>()
    )
    .collect::<_>()
}

pub fn input() -> String {
    parse_line(crate::aoc::input_raw(DAY).trim())
}

/// Dijkstra's shortest path to find least risky path
pub fn part1(inputs: &str) -> u64 {
    let pkt = Packet::from_stream(&mut inputs.chars());
    eprintln!("{:?}", pkt);
    pkt.sum_version()
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(inputs: &str) -> u64 {
    Packet::compute(&Box::new(Packet::from_stream(&mut inputs.chars())))
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &'static [&str] = &[
    "D2FE28",
    "38006F45291200",

    "8A004A801A8002F478",
    "620080001611562C8802118E34",
    "C0015000016115A2E0802F182340",
    "A0016C880162017C3686B18A3D4780",

    "C200B40A82",
    "04005AC33890",
    "880086C3E88112",
    "CE00C43D881120",
    "D8005AC2A8F0",
    "F600BC2D8F",
    "9C005AC2F8F0",
    "9C0141080250320F1802104A08",
];

#[test]
fn test_day16_part1() {
    assert_eq!(part1(&parse_line(TEST_INPUT[2])), 16);
    assert_eq!(part1(&parse_line(TEST_INPUT[3])), 12);
    assert_eq!(part1(&parse_line(TEST_INPUT[4])), 23);
    assert_eq!(part1(&parse_line(TEST_INPUT[5])), 31);
}

#[test]
fn test_day16_part2() {
    assert_eq!(part2(&parse_line(TEST_INPUT[6])), 3);
    assert_eq!(part2(&parse_line(TEST_INPUT[7])), 54);
    assert_eq!(part2(&parse_line(TEST_INPUT[8])), 7);
    assert_eq!(part2(&parse_line(TEST_INPUT[9])), 9);
    assert_eq!(part2(&parse_line(TEST_INPUT[10])), 1);
    assert_eq!(part2(&parse_line(TEST_INPUT[11])), 0);
    assert_eq!(part2(&parse_line(TEST_INPUT[12])), 0);
    assert_eq!(part2(&parse_line(TEST_INPUT[13])), 1);
}

#[test]
fn test_day16_parse_line() {
    assert_eq!(parse_line(TEST_INPUT[0]), String::from("110100101111111000101000"));
    assert_eq!(parse_line(TEST_INPUT[1]), String::from("00111000000000000110111101000101001010010001001000000000"));
}

#[test]
fn test_day16_packet() {
    let pkt = Packet::from_stream(&mut parse_line(TEST_INPUT[0]).chars());
    assert_eq!(pkt, Packet { version: 6, typeid: 4, payload: PacketType::Literal(2021)});
    eprintln!("Ok: {:?}", pkt);
    let pkt = Packet::from_stream(&mut parse_line(TEST_INPUT[1]).chars());
    assert_eq!(pkt, Packet { 
        version: 1, 
        typeid: 6, 
        payload: PacketType::Operator(Operator::Lt, vec![
            Box::new(Packet { version: 6, typeid: 4, payload: PacketType::Literal(10)}),
            Box::new(Packet { version: 2, typeid: 4, payload: PacketType::Literal(20)}),
            ])
        }
    );
    eprintln!("Ok: {:?}", pkt);
            
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
