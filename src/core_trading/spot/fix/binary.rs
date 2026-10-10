// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{Fields, Header, Request, Value, codec::wire_value};
use crate::core_trading::spot::sbe::schema::{Node, Schema, fix_schema};
use crate::{Decimal, Error, Symbol};
use zeroize::Zeroizing;

/// Native instrument precision carried from FIX `InstrumentList` increments.
/// Symbol spelling never supplies price/quantity or asset semantics.
#[derive(Clone, Debug)]
pub struct Precision {
    symbol: Symbol,
    price: i8,
    qty: i8,
}
impl Precision {
    /// Retain native symbol and decimal increments from one `InstrumentList` entry.
    ///
    /// The increments are the venue's
    /// [`PRICE_FILTER` `tickSize`](https://github.com/binance/binance-spot-api-docs/blob/master/filters.md#price_filter)
    /// and [`LOT_SIZE` `stepSize`](https://github.com/binance/binance-spot-api-docs/blob/master/filters.md#lot_size):
    /// the intervals a price or quantity moves by. They are not prices. A zero
    /// tick size disables the rule and gives no exponent to encode with, and a
    /// negative value is no interval.
    ///
    /// # Errors
    /// Refuses absent/invalid symbol or nonpositive price/quantity increments.
    pub fn from_instrument(instrument: &Fields) -> Result<Self, Error> {
        let Some(Value::Symbol(symbol)) = instrument.get("Symbol") else {
            return Err(Error::Validation("FIX instrument symbol"));
        };
        let Some(Value::Price(price)) = instrument.get("MinPriceIncrement") else {
            return Err(Error::Validation("FIX instrument price increment"));
        };
        let Some(Value::Quantity(qty)) = instrument.get("MinQtyIncrement") else {
            return Err(Error::Validation("FIX instrument quantity increment"));
        };
        if *price <= Decimal::ZERO || *qty <= Decimal::ZERO {
            return Err(Error::Validation(
                "FIX instrument increments must be positive",
            ));
        }
        Ok(Self {
            symbol: symbol.clone(),
            price: -i8::try_from(price.normalize().scale())
                .map_err(|_| Error::Validation("FIX price precision"))?,
            qty: -i8::try_from(qty.normalize().scale())
                .map_err(|_| Error::Validation("FIX quantity precision"))?,
        })
    }
    /// Native symbol whose metadata supplied this precision.
    #[must_use]
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}
impl Request {
    /// Encode a SOFH-framed native SBE request using supplied instrument metadata.
    /// Every mantissa is checked for exact representability; no input is rounded.
    /// Missing optional fields use the schema's explicit null values.
    ///
    /// # Errors
    /// Refuses missing symbol precision, values requiring rounding, integer/length
    /// overflow or fields not representable in the pinned FIX schema 1:1.
    pub fn encode_sbe(
        &self,
        header: &Header,
        precision: &[Precision],
    ) -> Result<Zeroizing<Vec<u8>>, Error> {
        encode(self.kind.code(), &self.fields, header, precision, None)
    }
}
pub(super) fn encode(
    kind: &str,
    fields: &Fields,
    header: &Header,
    precision: &[Precision],
    recv_window: Option<Decimal>,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let schema = fix_schema()?;
    let node = schema
        .messages
        .values()
        .find(|m| m.attr("semanticType") == Some(kind))
        .ok_or(Error::Validation("FIX SBE message type"))?;
    let block = block_size(schema, &node.children)?;
    let mut output = Zeroizing::new(vec![0; 6]);
    output.extend_from_slice(
        &u16::try_from(block)
            .map_err(|_| Error::Validation("FIX SBE root width"))?
            .to_le_bytes(),
    );
    output.extend_from_slice(
        &node
            .attr("id")
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or(Error::Configuration("FIX SBE template"))?
            .to_le_bytes(),
    );
    output.extend_from_slice(&schema.id.to_le_bytes());
    output.extend_from_slice(&schema.version.to_le_bytes());
    output.extend_from_slice(&header.sequence.to_le_bytes());
    output.extend_from_slice(&header.sending_time.micros().to_le_bytes());
    let mut writer = Writer {
        schema,
        header,
        precision,
        recv_window,
    };
    writer.body(&node.children, fields, &mut output)?;
    let length =
        u32::try_from(output.len()).map_err(|_| Error::Validation("FIX SBE frame length"))?;
    output[..4].copy_from_slice(&length.to_le_bytes());
    output[4..6].copy_from_slice(&0xeb50_u16.to_le_bytes());
    Ok(output)
}
fn block_size(schema: &Schema, nodes: &[Node]) -> Result<usize, Error> {
    nodes
        .iter()
        .filter(|n| n.kind == "field" && n.attr("presence") != Some("constant"))
        .try_fold(0_usize, |size, n| {
            size.checked_add(
                schema.size(
                    n.attr("type")
                        .ok_or(Error::Configuration("SBE field type"))?,
                )?,
            )
            .ok_or(Error::Configuration("SBE block overflow"))
        })
}
struct Writer<'a> {
    schema: &'a Schema,
    header: &'a Header,
    precision: &'a [Precision],
    recv_window: Option<Decimal>,
}
impl Writer<'_> {
    fn value<'a>(node: &Node, fields: &'a Fields) -> Option<&'a Value> {
        let id = node.attr("id").and_then(|s| s.parse::<u32>().ok())?;
        fields.values.iter().find(|v| v.tag == id).map(|v| &v.value)
    }
    fn exponent(&self, node: &Node, fields: &Fields) -> Result<i8, Error> {
        let Some(Value::Symbol(symbol)) = fields.get("Symbol") else {
            return Err(Error::Validation("SBE financial symbol required"));
        };
        let p = self
            .precision
            .iter()
            .find(|p| &p.symbol == symbol)
            .ok_or(Error::Validation("SBE instrument precision required"))?;
        match node.name.as_str() {
            "PriceExponent" => Ok(p.price),
            "QtyExponent" => Ok(p.qty),
            _ => Err(Error::Configuration("FIX SBE exponent name")),
        }
    }
    fn body(
        &mut self,
        nodes: &[Node],
        fields: &Fields,
        out: &mut Zeroizing<Vec<u8>>,
    ) -> Result<(), Error> {
        for node in nodes.iter().filter(|n| n.kind == "field") {
            self.fixed(node, nodes, fields, out)?;
        }
        for node in nodes.iter().filter(|n| n.kind != "field") {
            if node.kind == "group" {
                self.group(node, fields, out)?;
            } else {
                self.data(node, fields, out)?;
            }
        }
        Ok(())
    }
    fn fixed(
        &self,
        node: &Node,
        nodes: &[Node],
        fields: &Fields,
        out: &mut Zeroizing<Vec<u8>>,
    ) -> Result<(), Error> {
        if node.attr("presence") == Some("constant") {
            return Ok(());
        }
        let ty = self.schema.type_node(
            node.attr("type")
                .ok_or(Error::Configuration("SBE field type"))?,
        )?;
        let primitive = ty
            .attr("primitiveType")
            .or_else(|| ty.attr("encodingType"))
            .ok_or(Error::Configuration("SBE field encoding"))?;
        let width = self.schema.size(&ty.name)?;
        let value = Self::value(node, fields);
        let number = if node.attr("type") == Some("exponent8") {
            i128::from(self.exponent(node, fields)?)
        } else if let Some(exponent) = node.attr("exponent") {
            if let Some(value) = value {
                let decimal = match value {
                    Value::Price(v) | Value::Quantity(v) | Value::Amount(v) | Value::Decimal(v) => {
                        *v
                    }
                    _ => return Err(Error::Validation("SBE mantissa requires Decimal")),
                };
                let e = nodes
                    .iter()
                    .find(|n| n.name == exponent)
                    .ok_or(Error::Configuration("SBE mantissa exponent"))?;
                mantissa(decimal, self.exponent(e, fields)?)?
            } else {
                null(node, primitive, width)?
            }
        } else if node.name == "RecvWindow" && self.recv_window.is_some() {
            mantissa(
                self.recv_window
                    .ok_or(Error::Configuration("SBE receive window"))?,
                -3,
            )?
        } else if let Some(v) = value {
            if primitive == "char" {
                let text = wire_value(v)?;
                if text.len() != 1 {
                    return Err(Error::Validation("SBE character width"));
                }
                i128::from(text.as_bytes()[0])
            } else {
                integer(v)?
            }
        } else {
            null(node, primitive, width)?
        };
        write_integer(out, primitive, width, number)?;
        Ok(())
    }
    fn group(
        &mut self,
        node: &Node,
        fields: &Fields,
        out: &mut Zeroizing<Vec<u8>>,
    ) -> Result<(), Error> {
        let entries = match Self::value(node, fields) {
            Some(Value::Group(entries)) => entries.as_slice(),
            None => &[],
            _ => return Err(Error::Validation("SBE group entries")),
        };
        // The same SBE-defined default is used by the decoder.
        let dimensions = self
            .schema
            .type_node(node.attr("dimensionType").unwrap_or("groupSizeEncoding"))?;
        let block = block_size(self.schema, &node.children)?;
        for d in dimensions.children {
            let primitive = d
                .attr("primitiveType")
                .ok_or(Error::Configuration("SBE group dimension"))?;
            let width = self.schema.size(primitive)?;
            let value = match d.name.as_str() {
                "blockLength" => block,
                "numInGroup" => entries.len(),
                _ => return Err(Error::Configuration("SBE dimension name")),
            };
            write_integer(
                out,
                primitive,
                width,
                i128::try_from(value).map_err(|_| Error::Validation("SBE dimension overflow"))?,
            )?;
        }
        for e in entries {
            self.body(&node.children, e, out)?;
        }
        Ok(())
    }
    fn data(
        &self,
        node: &Node,
        fields: &Fields,
        out: &mut Zeroizing<Vec<u8>>,
    ) -> Result<(), Error> {
        let name = node
            .attr("type")
            .ok_or(Error::Configuration("SBE data type"))?;
        let ty = self.schema.type_node(name)?;
        let primitive = ty
            .children
            .first()
            .and_then(|n| n.attr("primitiveType"))
            .ok_or(Error::Configuration("SBE length type"))?;
        let value = match node.name.as_str() {
            "SenderCompId" => Zeroizing::new(
                self.header
                    .sender
                    .as_ref()
                    .ok_or(Error::Validation("FIX sender required"))?
                    .as_str()
                    .to_owned(),
            ),
            "TargetCompId" => Zeroizing::new(
                self.header
                    .target
                    .as_ref()
                    .ok_or(Error::Validation("FIX target required"))?
                    .as_str()
                    .to_owned(),
            ),
            _ => {
                if let Some(v) = Self::value(node, fields) {
                    Zeroizing::new(wire_value(v)?)
                } else if name.starts_with("optional") {
                    Zeroizing::new(String::new())
                } else {
                    return Err(Error::Validation("required SBE data field missing"));
                }
            }
        };
        write_integer(
            out,
            primitive,
            self.schema.size(primitive)?,
            i128::try_from(value.len()).map_err(|_| Error::Validation("SBE data length"))?,
        )?;
        out.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
fn null(node: &Node, primitive: &str, width: usize) -> Result<i128, Error> {
    if node.attr("presence") != Some("optional") {
        return Err(Error::Validation("required SBE field missing"));
    }
    Ok(if primitive == "char" {
        0
    } else if primitive.starts_with("int") {
        -(1_i128 << (width * 8 - 1))
    } else {
        (1_i128 << (width * 8)) - 1
    })
}
fn integer(value: &Value) -> Result<i128, Error> {
    match value {
        Value::Integer(v) => Ok(i128::from(*v)),
        Value::Unsigned(v) | Value::DurationMicros(v) => Ok(i128::from(*v)),
        Value::Boolean(v) => Ok(i128::from(*v)),
        Value::Timestamp(t) => Ok(i128::from(t.micros())),
        Value::Decimal(v) if v.normalize().scale() == 0 => Ok(v.normalize().mantissa()),
        Value::Code(v) => v.parse().map_err(|_| Error::Validation("SBE enum integer")),
        _ => Err(Error::Validation("SBE integer field type")),
    }
}
fn write_integer(
    out: &mut Zeroizing<Vec<u8>>,
    primitive: &str,
    width: usize,
    value: i128,
) -> Result<(), Error> {
    let signed = primitive.starts_with("int");
    let limit = 1_i128
        .checked_shl(
            u32::try_from(width * 8 - usize::from(signed))
                .map_err(|_| Error::Configuration("SBE integer width"))?,
        )
        .ok_or(Error::Configuration("SBE integer width"))?;
    if (signed && !(-limit..limit).contains(&value)) || (!signed && !(0..limit).contains(&value)) {
        return Err(Error::Validation("SBE integer range"));
    }
    out.extend_from_slice(
        value
            .to_le_bytes()
            .get(..width)
            .ok_or(Error::Configuration("SBE integer width"))?,
    );
    Ok(())
}
fn mantissa(value: Decimal, exponent: i8) -> Result<i128, Error> {
    let target = u32::from(exponent.unsigned_abs());
    if exponent > 0 {
        return Err(Error::Validation("unsupported instrument exponent"));
    }
    let mut mantissa = value.mantissa();
    if value.scale() < target {
        mantissa = mantissa
            .checked_mul(
                10_i128
                    .checked_pow(target - value.scale())
                    .ok_or(Error::Validation("SBE scale overflow"))?,
            )
            .ok_or(Error::Validation("SBE mantissa overflow"))?;
    } else if value.scale() > target {
        let factor = 10_i128
            .checked_pow(value.scale() - target)
            .ok_or(Error::Validation("SBE scale overflow"))?;
        if mantissa % factor != 0 {
            return Err(Error::Validation("SBE precision would require rounding"));
        }
        mantissa /= factor;
    }
    Ok(mantissa)
}
