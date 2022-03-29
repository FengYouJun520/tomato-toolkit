import * as models from './models';

export interface go {
  "codegen": {
    "Manager": {
		CodeGenerate(arg1:models.ConfigContext):Promise<string|Error>
    },
  }

  "main": {
    "App": {
		GetOs():Promise<string>
		PingDb(arg1:models.DataSourceConfig):Promise<Array<models.DatabaseOptions>|Error>
    },
  }

}

declare global {
	interface Window {
		go: go;
	}
}
