use llrt_utils::module::{export_default, ModuleInfo};
use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Result,
};

use self::string_decoder::StringDecoder;

mod string_decoder;

pub struct StringDecoderModule;

impl ModuleDef for StringDecoderModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare(stringify!(StringDecoder))?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<StringDecoder>::define(default)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl From<StringDecoderModule> for ModuleInfo<StringDecoderModule> {
    fn from(val: StringDecoderModule) -> Self {
        ModuleInfo {
            name: "string_decoder",
            module: val,
        }
    }
}

#[cfg(test)]
mod tests {
    use llrt_test::{call_test, test_async_with, ModuleEvaluator};

    use super::*;

    #[tokio::test]
    async fn test_utf_8() {
        test_async_with(|ctx| {
            Box::pin(async move {
                llrt_buffer::init(&ctx).unwrap();
                ModuleEvaluator::eval_rust::<StringDecoderModule>(ctx.clone(), "string_decoder")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { StringDecoder } from 'string_decoder';

                        export async function test() {
                            const decoder = new StringDecoder('utf-8');

                            const cent = Buffer.from([0xC2, 0xA2]);

                            return decoder.write(cent);
                        }
                    "#,
                )
                .await
                .unwrap();
                let result = call_test::<String, _>(&ctx, &module, ()).await;
                assert_eq!(result, "¢");
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_base64() {
        test_async_with(|ctx| {
            Box::pin(async move {
                llrt_buffer::init(&ctx).unwrap();
                ModuleEvaluator::eval_rust::<StringDecoderModule>(ctx.clone(), "string_decoder")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { StringDecoder } from 'string_decoder';

                        export async function test() {
                            const decoder = new StringDecoder('base64');
                            let res = "";
                            res += decoder.write(Buffer.of(0x61));
                            res += decoder.end();
                            res += decoder.write(Buffer.of());
                            res += decoder.end();
                            return res;
                        }
                    "#,
                )
                .await
                .unwrap();
                let result = call_test::<String, _>(&ctx, &module, ()).await;
                assert_eq!(result, "YQ==");
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_utf16le() {
        test_async_with(|ctx| {
            Box::pin(async move {
                llrt_buffer::init(&ctx).unwrap();
                ModuleEvaluator::eval_rust::<StringDecoderModule>(ctx.clone(), "string_decoder")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { StringDecoder } from 'string_decoder';

                        export async function test() {
                            const decoder = new StringDecoder('utf16le');
                            let res = "";
                            res += decoder.write(Buffer.of(0x61, 0x00));
                            res += decoder.end();
                            res += decoder.write(Buffer.of());
                            res += decoder.end();
                            return res;
                        }
                    "#,
                )
                .await
                .unwrap();
                let result = call_test::<String, _>(&ctx, &module, ()).await;
                assert_eq!(result, "a");
            })
        })
        .await;
    }
}
