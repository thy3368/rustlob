/// Halo2 零知识证明示例
///
/// 本模块展示了如何使用 halo2_proofs 库来构建零知识证明电路
///
/// 示例包括：
/// 1. 简单的平方电路：证明知道 x，使得 x^2 = y
/// 2. 加法电路：证明 a + b = c
/// 3. 乘法电路：证明 a * b = c

use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    dev::MockProver,
    pasta::Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector,
    },
    poly::Rotation,
};
use std::marker::PhantomData;

/// ========================================
/// 示例 1: 平方电路
/// 证明: 我知道 x，使得 x^2 = y
/// ========================================

/// 平方电路的配置
#[derive(Debug, Clone)]
struct SquareConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    selector: Selector,
}

/// 平方芯片（Chip）
struct SquareChip<F: Field> {
    config: SquareConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> Chip<F> for SquareChip<F> {
    type Config = SquareConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> SquareChip<F> {
    fn construct(config: SquareConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: Column<Advice>,
        instance: Column<Instance>,
    ) -> SquareConfig {
        let selector = meta.selector();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        // 约束：x * x = y
        meta.create_gate("square", |meta| {
            let s = meta.query_selector(selector);
            let x = meta.query_advice(advice, Rotation::cur());
            let y = meta.query_advice(advice, Rotation::next());

            vec![s * (x.clone() * x - y)]
        });

        SquareConfig {
            advice,
            instance,
            selector,
        }
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        x: Value<F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "square region",
            |mut region: Region<'_, F>| {
                self.config.selector.enable(&mut region, 0)?;

                // 分配输入 x
                let x_cell = region.assign_advice(
                    || "x",
                    self.config.advice,
                    0,
                    || x,
                )?;

                // 计算并分配 x^2
                let x_squared = x.map(|x| x * x);
                let y_cell = region.assign_advice(
                    || "x^2",
                    self.config.advice,
                    1,
                    || x_squared,
                )?;

                Ok(y_cell)
            },
        )
    }
}

/// 平方电路
#[derive(Default, Clone)]
struct SquareCircuit<F: Field> {
    x: Value<F>,
}

impl<F: Field> Circuit<F> for SquareCircuit<F> {
    type Config = SquareConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        SquareChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = SquareChip::construct(config.clone());

        let y_cell = chip.assign(layouter.namespace(|| "square"), self.x)?;

        // 将结果暴露为公共输入
        layouter.constrain_instance(y_cell.cell(), config.instance, 0)?;

        Ok(())
    }
}

/// ========================================
/// 示例 2: 加法电路
/// 证明: a + b = c
/// ========================================

#[derive(Debug, Clone)]
struct AddConfig {
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    selector: Selector,
}

struct AddChip<F: Field> {
    config: AddConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> Chip<F> for AddChip<F> {
    type Config = AddConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> AddChip<F> {
    fn construct(config: AddConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
    ) -> AddConfig {
        let selector = meta.selector();

        for column in &advice {
            meta.enable_equality(*column);
        }
        meta.enable_equality(instance);

        // 约束：a + b = c
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(advice[0], Rotation::cur());
            let b = meta.query_advice(advice[1], Rotation::cur());
            let c = meta.query_advice(advice[0], Rotation::next());

            vec![s * (a + b - c)]
        });

        AddConfig {
            advice,
            instance,
            selector,
        }
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        a: Value<F>,
        b: Value<F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "add region",
            |mut region: Region<'_, F>| {
                self.config.selector.enable(&mut region, 0)?;

                region.assign_advice(|| "a", self.config.advice[0], 0, || a)?;
                region.assign_advice(|| "b", self.config.advice[1], 0, || b)?;

                let c = a.zip(b).map(|(a, b)| a + b);
                let c_cell = region.assign_advice(
                    || "c",
                    self.config.advice[0],
                    1,
                    || c,
                )?;

                Ok(c_cell)
            },
        )
    }
}

#[derive(Default, Clone)]
struct AddCircuit<F: Field> {
    a: Value<F>,
    b: Value<F>,
}

impl<F: Field> Circuit<F> for AddCircuit<F> {
    type Config = AddConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        AddChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = AddChip::construct(config.clone());
        let c_cell = chip.assign(layouter.namespace(|| "add"), self.a, self.b)?;
        layouter.constrain_instance(c_cell.cell(), config.instance, 0)?;
        Ok(())
    }
}

/// ========================================
/// ZEVM: 零知识证明虚拟机
/// 封装 halo2 的证明生成和验证流程
/// ========================================

pub struct ZEVM;

impl ZEVM {
    /// 示例：运行平方电路的完整流程
    pub fn run_square_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Halo2 平方电路示例 ===\n");

        // 1. 设置参数
        let k = 4; // 电路大小参数 (2^k 行)

        // 2. 私密输入 (witness): x = 5
        let x = Fp::from(5);
        let circuit = SquareCircuit {
            x: Value::known(x),
        };

        // 3. 公开输出: y = 25
        let y = x * x;
        let public_inputs = vec![y];

        println!("私密输入 x: 5");
        println!("公开输出 y: 25\n");

        // 4. 使用 MockProver 验证电路
        println!("验证电路约束...");
        let prover = MockProver::run(k, &circuit, vec![public_inputs])?;

        // 5. 检查约束
        match prover.verify() {
            Ok(_) => {
                println!("✓ 电路约束验证成功！\n");
                println!("解释：");
                println!("证明者成功证明了他知道一个数 x，使得 x^2 = 25");
                println!("但验证者无法从证明中得知 x 的具体值（5）\n");
                println!("在实际应用中，会生成零知识证明并发送给验证者");
                println!("验证者只能验证证明的正确性，无法获知 x 的值");
            }
            Err(e) => {
                println!("✗ 电路约束验证失败:");
                println!("{:?}", e);
            }
        }

        Ok(())
    }

    /// 示例：运行加法电路
    pub fn run_add_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Halo2 加法电路示例 ===\n");

        let k = 4;

        // 私密输入: a = 3, b = 7
        let a = Fp::from(3);
        let b = Fp::from(7);
        let circuit = AddCircuit {
            a: Value::known(a),
            b: Value::known(b),
        };

        // 公开输出: c = 10
        let c = a + b;
        let public_inputs = vec![c];

        println!("私密输入 a: 3, b: 7");
        println!("公开输出 c: 10\n");

        // 验证电路
        println!("验证电路约束...");
        let prover = MockProver::run(k, &circuit, vec![public_inputs])?;

        match prover.verify() {
            Ok(_) => {
                println!("✓ 电路约束验证成功！\n");
                println!("解释：");
                println!("证明者成功证明了他知道两个数 a 和 b，使得 a + b = 10");
                println!("但验证者无法从证明中得知 a 和 b 的具体值\n");
                println!("可能的组合有: (1,9), (2,8), (3,7), (4,6), (5,5) 等等");
                println!("零知识证明保护了这些私密输入");
            }
            Err(e) => {
                println!("✗ 电路约束验证失败:");
                println!("{:?}", e);
            }
        }

        Ok(())
    }

    /// 示例：错误的证明会被拒绝
    pub fn run_invalid_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 错误证明示例 ===\n");

        let k = 4;

        // 私密输入: x = 5
        let x = Fp::from(5);
        let circuit = SquareCircuit {
            x: Value::known(x),
        };

        // 错误的公开输出: y = 24 (实际应该是 25)
        let wrong_y = Fp::from(24);
        let public_inputs = vec![wrong_y];

        println!("私密输入 x: 5 (x^2 = 25)");
        println!("声称的公开输出 y: 24 (错误！)\n");

        // 验证电路
        println!("验证电路约束...");
        let prover = MockProver::run(k, &circuit, vec![public_inputs])?;

        match prover.verify() {
            Ok(_) => {
                println!("✓ 验证通过（不应该发生）");
            }
            Err(e) => {
                println!("✗ 验证失败（预期行为）\n");
                println!("解释：");
                println!("当证明者试图提供错误的公开输出时，");
                println!("电路约束会检测到不一致，验证失败。");
                println!("这展示了零知识证明的'可靠性'属性：");
                println!("虚假的声明无法被证明。");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_circuit() {
        let k = 4;
        let x = Fp::from(5);
        let y = x * x;

        let circuit = SquareCircuit {
            x: Value::known(x),
        };

        let public_inputs = vec![y];

        let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
    }

    #[test]
    fn test_add_circuit() {
        let k = 4;
        let a = Fp::from(3);
        let b = Fp::from(7);
        let c = a + b;

        let circuit = AddCircuit {
            a: Value::known(a),
            b: Value::known(b),
        };

        let public_inputs = vec![c];

        let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
    }

    #[test]
    fn test_square_circuit_wrong_output() {
        let k = 4;
        let x = Fp::from(5);
        let wrong_y = Fp::from(24); // 错误的输出

        let circuit = SquareCircuit {
            x: Value::known(x),
        };

        let public_inputs = vec![wrong_y];

        let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
        assert!(prover.verify().is_err());
    }
}
