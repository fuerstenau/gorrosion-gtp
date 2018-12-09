use super::Byte;
use super::Input;
use nom::InputIter;
use std::str::FromStr;
use std::string::ToString;
//use super::MessagePart;
struct MessagePart {
	msg: Vec<Byte>,
}

// TODO: I'm unhappy with quite a few of the names.

//use self::simple_entity::SimpleEntity;
use nom::IResult;

pub trait Data: Into<MessagePart> {
	type Type;
	// TODO: Which kind of errors do we need to throw?
	fn parse(i: Input, t: Self::Type) -> IResult<Input, Self>;
	fn typed(&self) -> Self::Type;
}

mod simple_entity {
	use super::Data;
	use std::convert::TryFrom;

	pub enum Type {
		Int,
		Float,
		String,
		Vertex,
		Color,
		Move,
		Boolean,
	}

	pub enum Value {
		Int(super::Int),
		Float(super::Float),
		String(super::String),
		Vertex(super::Vertex),
		Color(super::Color),
		Move(super::Move),
		Boolean(super::Boolean),
	}

	pub trait SimpleEntity: Data + TryFrom<Value> + Into<Value> {}
}

// TODO: Get this to namespace sensibly.
mod data_types {
	macro_rules! singleton_type {
		( $i: ident ) => {
			pub enum $i {
				$i,
			}
		};
	}

	singleton_type!(Int);
	singleton_type!(Float);
	singleton_type!(String);
	singleton_type!(Vertex);
	singleton_type!(Color);
	singleton_type!(Move);
	singleton_type!(Boolean);
}

pub trait GtpType {}
pub trait SingleLine {}
pub trait SimpleEntity {}

impl GtpType for SingleLine {}
impl SingleLine for SimpleEntity {}

pub struct Int {
	data: u32,
}

impl From<Int> for MessagePart {
	fn from(Int { data }: Int) -> MessagePart {
		let msg = Vec::from(data.to_string().as_bytes());
		MessagePart { msg }
	}
}

impl Data for Int {
	type Type = data_types::Int;

	// FIXME: Error handling
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let digits = nom::digit(i);
		match digits {
			Ok((rem, str)) => {
				let str = str
					.iter_elements()
					.map(nom::AsChar::as_char)
					.collect::<std::string::String>();
				let data = FromStr::from_str(&str).unwrap();
				Ok((rem, Int { data }))
			}
			Err(e) => Err(e),
		}
	}

	fn typed(&self) -> Self::Type {
		data_types::Int::Int
	}
}

impl SimpleEntity for Int {}

pub struct Float {
	data: f32,
}

impl From<Float> for MessagePart {
	fn from(Float { data }: Float) -> MessagePart {
		let msg = Vec::from(data.to_string().as_bytes());
		MessagePart { msg }
	}
}

impl Data for Float {
	type Type = data_types::Float;

	/// The GTP “specification” does not specify
	/// in which ways a float may be represented.
	/// We therefore simply accept as a float
	/// whatever nom accepts as a float.
	// FIXME: Error handling
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let result = nom::float(i);
		match result {
			Ok((rem, data)) => Ok((rem, Float { data })),
			Err(e) => Err(e),
		}
	}

	fn typed(&self) -> Self::Type {
		data_types::Float::Float
	}
}

impl SimpleEntity for Float {}

pub struct String {
	data: Vec<Byte>,
}

impl From<String> for MessagePart {
	fn from(String { data }: String) -> MessagePart {
		let msg = data;
		MessagePart { msg }
	}
}

impl Data for String {
	type Type = data_types::String;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let result = take_until_either!(i, b" \n");
		match result {
			Ok((rem, data)) => {
				let data = data.iter_elements().collect();
				Ok((rem, String { data }))
			}
			Err(e) => Err(e),
		}
	}

	fn typed(&self) -> Self::Type {
		data_types::String::String
	}
}

impl SimpleEntity for String {}

pub enum Vertex {
	Pass,
	Coord(char, u8),
}

impl From<Vertex> for MessagePart {
	fn from(vert: Vertex) -> MessagePart {
		let msg: Vec<Byte> = match vert {
			Vertex::Pass => b"pass".to_vec(),
			Vertex::Coord(letter, number) => {
				let mut num = Vec::from(
					number.to_string().as_bytes(),
				);
				let mut msg = Vec::with_capacity(num.len() + 1);
				msg.push(letter as Byte);
				msg.append(&mut num);
				msg
			}
		};
		MessagePart { msg }
	}
}

impl Data for Vertex {
	type Type = data_types::Vertex;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		unimplemented!()
	}

	fn typed(&self) -> Self::Type {
		data_types::Vertex::Vertex
	}
}

impl SimpleEntity for Vertex {}

pub enum Color {
	Black,
	White,
}

impl From<Color> for MessagePart {
	fn from(col: Color) -> MessagePart {
		let msg = match col {
			Color::Black => b"Black",
			Color::White => b"White",
		}.to_vec();
		MessagePart { msg }
	}
}

impl Data for Color {
	type Type = data_types::Color;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		unimplemented!()
	}

	fn typed(&self) -> Self::Type {
		data_types::Color::Color
	}
}

impl SimpleEntity for Color {}

pub struct Move {
	color: Color,
	vertex: Vertex,
}

impl From<Move> for MessagePart {
	fn from(m: Move) -> MessagePart {
		let mut msg = MessagePart::from(m.color).msg;
		msg.extend(b" ");
		msg.append(&mut MessagePart::from(m.vertex).msg);
		MessagePart { msg }
	}
}

impl Data for Move {
	type Type = data_types::Move;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		unimplemented!()
	}

	fn typed(&self) -> Self::Type {
		data_types::Move::Move
	}
}

impl SimpleEntity for Move {}

pub enum Boolean {
	False,
	True,
}

impl From<Boolean> for MessagePart {
	fn from(b: Boolean) -> MessagePart {
		let msg = match b {
			Boolean::False => b"false".to_vec(),
			Boolean::True => b"true".to_vec(),
		};
		MessagePart { msg }
	}
}

impl Data for Boolean {
	type Type = data_types::Boolean;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		unimplemented!()
	}

	fn typed(&self) -> Self::Type {
		data_types::Boolean::Boolean
	}
}

impl SimpleEntity for Boolean {}

enum CollectionItem {
	Int(Int),
	Float(Float),
	String(String),
	Vertex(Vertex),
	Color(Color),
	Move(Move),
	Boolean(Boolean),
}

pub enum Collection {
	None,
	Collection(Box<(CollectionItem, Collection)>),
	IntList(List<Int>),
	FloatList(List<Float>),
	StringList(List<String>),
	VertexList(List<Vertex>),
	ColorList(List<Color>),
	MoveList(List<Move>),
	BooleanList(List<Boolean>),
}

impl SingleLine for Collection {}

pub struct List<T: SimpleEntity> {
	data: Vec<T>,
}

impl<T: SimpleEntity> SingleLine for List<T> {}

// The so-called specification is unclear
// on the matter of alternatives of compound types.
// For now, we'll do the simpler thing.
pub enum Alternatives<T: SimpleEntity, S: SimpleEntity> {
	Left(T),
	Right(S),
}

impl<T: SimpleEntity, S: SimpleEntity> SingleLine for Alternatives<T, S> {}

pub struct MultilineList<T: SingleLine> {
	data: Vec<T>,
}

impl<T: SingleLine> GtpType for MultilineList<T> {}

enum SimpleEntityType {
	Int,
	Float,
	String,
	Vertex,
	Color,
	Move,
	Boolean,
}

enum SimpleEntityValue {
	Int(Int),
	Float(Float),
	String(String),
	Vertex(Vertex),
	Color(Color),
	Move(Move),
	Boolean(Boolean),
}

enum ListType {
	IntList,
	FloatList,
	StringList,
	VertexList,
	ColorList,
	MoveList,
	BooleanList,
}

enum ListValue {
	IntList(List<Int>),
	FloatList(List<Float>),
	StringList(List<String>),
	VertexList(List<Vertex>),
	ColorList(List<Color>),
	MoveList(List<Move>),
	BooleanList(List<Boolean>),
}

enum SingleLineType {
	SimpleEntity(SimpleEntityType),
	Collection,
	List(ListType),
	Alternatives(SimpleEntityType, SimpleEntityType),
}

enum SingleLineValue {
	SimpleEntity(SimpleEntityValue),
	Collection(Collection),
	List(ListValue),
}

enum GtpTypeType {
	SingleLine(SingleLineType),
	MultilineList(SingleLineType),
}
