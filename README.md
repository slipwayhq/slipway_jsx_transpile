# `slipwayhq.jsx_transpile`

A [Slipway](https://slipway.co/) Component which takes a [JSX definition](https://og-playground.vercel.app/)
as an input and returns a transpiled Javascript version of the JSX.

This Component is used by the [`slipwayhq.jsx`](https://github.com/slipwayhq/slipway_jsx) Component and
is not intended to be used directly.

This Component uses [Speedy Web Compiler](https://swc.rs/) internally.

## Required Inputs

- `jsx`: The JSX string.

## Suggested Permissions

None.

## Example Usage

Test the component by running the following command and pasting in the input when prompted:
```
slipway run-component "slipwayhq.jsx_transpile.0.5.1"
```

Input:
```json
{
  "jsx": "<div style={{height: '100%',width: '100%',display: 'flex',flexDirection: 'column',alignItems: 'center',justifyContent: 'center',backgroundColor: '#fff',fontSize: 32,fontWeight: 600}}><svg width=\"75\" viewBox=\"0 0 75 65\" fill=\"#000\" style={{ margin: '0 75px' }}><path d=\"M37.59.25l36.95 64H.64l36.95-64z\"></path></svg><div style={{ marginTop: 40 }}>{data.text}</div></div>"
}
```

Output:
```json
{
  "js": "React.createElement(\"div\", {\n    style: {\n        height: '100%',\n        width: '100%',\n        display: 'flex',\n        flexDirection: 'column',\n        alignItems: 'center',\n        justifyContent: 'center',\n        backgroundColor: '#fff',\n        fontSize: 32,\n        fontWeight: 600\n    }\n}, React.createElement(\"svg\", {\n    width: \"75\",\n    viewBox: \"0 0 75 65\",\n    fill: \"#000\",\n    style: {\n        margin: '0 75px'\n    }\n}, React.createElement(\"path\", {\n    d: \"M37.59.25l36.95 64H.64l36.95-64z\"\n})), React.createElement(\"div\", {\n    style: {\n        marginTop: 40\n    }\n}, data.text));\n"
}
```