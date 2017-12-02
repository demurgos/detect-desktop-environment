# 0.5.0-alpha.14 (2017-08-17)

- **[Fix]** Fix published package.

# 0.5.0-alpha.13 (2017-08-17)

- **[Feature]** Add `TryUnion` type
- **[Feature]** Add methods to get the variant used by union types.

# 0.5.0-alpha.12 (2017-08-16)

- **[Fix]** Support lazy options for `TaggedUnion`.

# 0.5.0-alpha.11 (2017-08-15)

- **[Feature]** Add free-form JSON type, contains any valid `JSON` value.
- **[Feature]** Support lazy options definition. Instead of an option object, you can now pass a function returning an
  option object. It will be evaluated only once one of its values is needed. This enables circular dependencies.
- **[Internal]** Update build tools.

# 0.5.0-alpha.10 (2017-08-11)

- **[Breaking]** Replace `"browser"` with ES5 CommonJS modules by `"module""` with ES6 modules in `package.json`
- **[Breaking]** Drop library support of ES5, use a shim or transpiler if needed.
- **[Breaking]** Mark `unorm` and `bson` as optional dependencies.
- **[Breaking]** Drop support for ES modules in favor of deep package imports.
- **[Feature]** Provide experimental deep-require support.
- **[Internal]** Convert gulpfile to Typescript.
- **[Internal]** Add `yarn.lock` to better support Yarn.
- **[Internal]** Refactor library to prepare support for deep package imports
  (`import {IntegerType} from "kryo/integer"`)

# 0.5.0-alpha.9 (2017-07-14)

- **[Fix]** Treat `{foo: undefined}` as `{}` if `foo` is an optional document property.

# 0.5.0-alpha.8 (2017-07-13)

- **[Fix]** Mark `bson` as a normal dependency (instead of a dev dependency).

# 0.5.0-alpha.7 (2017-07-10)

- **[Feature]** Implement `BufferType`
- **[Fix]** `ArrayType#read` now uses `itemType.read` (instead of `.readTrusted`).
- **[Fix]** Do not type value for `readMatcher` and `readMatcher`, used by the general `UnionType`.

# 0.5.0-alpha.6

- **[Feature]** Implement `MapType`
- **[Internal]** Update build-tools to use TSLint 8

# 0.5.0-alpha.5

- **[Patch]** Export `TaggedUnionType`

# 0.5.0-alpha.4

- **[Feature]** Implement `LiteralType`
- **[Feature]** Implement `UnionType`
- **[Feature]** Implement `TaggedUnionType`. This is a subclass of UnionType to simplify
  the definition of unions of documents with a "tag property". The tag property acts as
  a discriminant and allows to retrieve the type of the whole document.
- **[Feature]** Add the `rename` option to `DocumentType`. This allows to rename
  the keys of the document similarly to the values of `SimpleEnumType`.
- **[Patch]** Fix the constraints for `SerializableType` generics.
  `Output` should extend `Input`. This restores the order `T`, `Format`,`Input`, `Output`.
- **[Internal]** Fix documentation generation with typedoc
- **[Internal]** Improve support for the tests checking the output of `.write`
- **[Internal]** Drop `lodash` dependency

# 0.5.0-alpha.3

- **[Feature]** Add support for the `"qs"` format for [the `qs` package][npm-qs] (used by [`express`][npm-express])
- **[Internal]** Create CHANGELOG.md

[npm-express]:https://www.npmjs.com/package/expess
[npm-qs]:https://www.npmjs.com/package/qs
