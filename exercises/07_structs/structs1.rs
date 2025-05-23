struct ColorRegularStruct {
    // TODO: 添加字段(fields)，使其能够通过测试 `regular_structs`。
    // 这些字段应具有什么类型？ RGB颜色值的最小值和最大值是多少？
    red: u8,
    green: u8,
    blue: u8,
}   

struct ColorTupleStruct(u8, u8, u8/* TODO: 添加字段(fields)，使其能够通过测试 `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 实例化(Instantiate)一个普通结构体。
        // let green =
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构体。
        // let green =
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个单元结构体。
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
