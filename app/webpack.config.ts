import reduceWebpack from 'reduce-webpack';
import {Configuration} from 'webpack';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

const config: Configuration = reduceWebpack(
	{
	},
	'v0.1.0',
	__dirname
);

config.plugins.push(
	new WasmPackPlugin({
		crateDirectory: __dirname,
		outDir: 'src/utils/pkg'
	})
);

export default config;
