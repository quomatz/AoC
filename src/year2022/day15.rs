use nom::bytes::complete::tag;

use crate::day0::*;

pub struct Day15<const P : i32>;



impl<const P : i32> Day<2022,15,Vec<Sensor>, u32>  for Day15<P>
{
    fn solve(input: Vec<Sensor>) -> u32 {
		
		todo!()
		
    }


    fn parse(input: &str) -> Vec<Sensor> {
		nom::multi::separated_list0(tag("\n"),parsing::sensor)(input).unwrap().1
    }
}

struct Point
{
	x : i32,
	y : i32,
}

pub struct Sensor
{
	pos : Point,
	/// Closest beacon.
	beacon : Point
}


mod intervals
{
    use itertools::Itertools;

	

	struct IntervalArray
	{
		switches : Vec<u32>,
	}

	impl IntervalArray
	{
		fn new() -> IntervalArray
		{
			IntervalArray { switches: Vec::new() }
		}
		
		fn add(&mut self, l : u32, r : u32)
		{
			let i = 0;
			let ir = self.switches.partition_point(|&x| x > r);
			let il = self.switches.partition_point(|&x| x < l);
			self.switches.splice(il..ir,[l,r]);
		}

		fn count(&self) -> u32
		{
			self.switches.iter().tuples().fold(0,|acc,(l,r)| acc + r - l + 1)
		}
	}

	#[cfg(test)]
	mod tests
	{
    use super::IntervalArray;

		#[test]
		fn add_1()
		{
			let mut ia = IntervalArray::new();

			ia.add(5,8);
			ia.add(9,13);

			assert_eq!(ia.count(),9);
		}
	}
}
mod parsing
{
    use nom::{IResult, sequence::{preceded, delimited}, bytes::complete::tag, character::complete::i32};

    use super::*;

	pub(crate) fn sensor(input : &str) -> IResult<&str, Sensor>
	{	
		let (input, pos) = delimited(
			tag("Sensor at "),
			point,
			tag(": "),
		)(input)?;
		let (input, beacon) = preceded(
			tag("closest beacon is at "),
			point
		)(input)?;

		Ok((input,Sensor{pos,beacon}))
	}

	fn point(input : &str) -> IResult<&str,Point>
	{
	
		let (input,x) = delimited(
			tag("x="),
			i32,
			tag(", "),
		)(input)?;
		let (input,y) = preceded(
			tag("y="),
			i32
		)(input)?;

		Ok((input,Point{x,y}))
		
	}

	#[cfg(test)]
	mod tests
	{
    use nom::Finish;

    use super::sensor;

		#[test]
		fn parse_sensor()
		{
			let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
			let sensor = sensor(input).finish().unwrap().1;
			
			assert_eq!(sensor.pos.x,2);
			assert_eq!(sensor.pos.y,18);
			assert_eq!(sensor.beacon.x,-2);
			assert_eq!(sensor.beacon.y,15);
			
		}
	}
	
}