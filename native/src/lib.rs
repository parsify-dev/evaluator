use neon::prelude::*;
use regex::Regex;

fn remove_comments(mut cx: FunctionContext) -> JsResult<JsString> {
    let expression = cx.argument::<JsString>(0)?.value();

    let re = Regex::new(r"((/{2}|#).*)").unwrap();
    let result = re.replace_all(expression.as_str(), "");

    Ok(cx.string(result))
}

fn remove_labels(mut cx: FunctionContext) -> JsResult<JsString> {
    let expression = cx.argument::<JsString>(0)?.value();

    let re = Regex::new(r".*:").unwrap();
    let result = re.replace_all(expression.as_str(), "");

    Ok(cx.string(result))
}

register_module!(mut m, {
    m.export_function("removeComments", remove_comments)?;
    m.export_function("removeLabels", remove_labels)
});
