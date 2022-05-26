export namespace codegen {
	
	export class Mapper {
	    superClass: string;
	    enableMapperAnnotation: boolean;
	    formatMapperFileName: string;
	    formatXmlFileName: string;
	
	    static createFrom(source: any = {}) {
	        return new Mapper(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.superClass = source["superClass"];
	        this.enableMapperAnnotation = source["enableMapperAnnotation"];
	        this.formatMapperFileName = source["formatMapperFileName"];
	        this.formatXmlFileName = source["formatXmlFileName"];
	    }
	}
	export class Service {
	    superServiceClass: string;
	    superServiceImplClass: string;
	    formatServiceFileName: string;
	    formatServiceImplFileName: string;
	
	    static createFrom(source: any = {}) {
	        return new Service(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.superServiceClass = source["superServiceClass"];
	        this.superServiceImplClass = source["superServiceImplClass"];
	        this.formatServiceFileName = source["formatServiceFileName"];
	        this.formatServiceImplFileName = source["formatServiceImplFileName"];
	    }
	}
	export class Controller {
	    superClass: string;
	    enableHyphenStyle: boolean;
	    enableRestStyle: boolean;
	    formatFileName: string;
	
	    static createFrom(source: any = {}) {
	        return new Controller(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.superClass = source["superClass"];
	        this.enableHyphenStyle = source["enableHyphenStyle"];
	        this.enableRestStyle = source["enableRestStyle"];
	        this.formatFileName = source["formatFileName"];
	    }
	}
	export class FieldTypeKeyVal {
	    key: string;
	    name: string;
	    value: string;
	
	    static createFrom(source: any = {}) {
	        return new FieldTypeKeyVal(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.key = source["key"];
	        this.name = source["name"];
	        this.value = source["value"];
	    }
	}
	export class Entity {
	    superClass: string;
	    disableSerialVersionUID: boolean;
	    enableColumnConstant: boolean;
	    enableChainModel: boolean;
	    enableLombok: boolean;
	    enableRemoveIsPrefix: boolean;
	    enableTableFieldAnnotation: boolean;
	    enableActiveRecord: boolean;
	    versionColumnName: string;
	    versionPropertyName: string;
	    logicDeleteColumnName: string;
	    logicDeletePropertyName: string;
	    naming: string;
	    addSuperEntityColumns: string[];
	    addIgnoreColumns: string[];
	    addTableFills: FieldTypeKeyVal[];
	    idType: string;
	    formatFileName: string;
	
	    static createFrom(source: any = {}) {
	        return new Entity(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.superClass = source["superClass"];
	        this.disableSerialVersionUID = source["disableSerialVersionUID"];
	        this.enableColumnConstant = source["enableColumnConstant"];
	        this.enableChainModel = source["enableChainModel"];
	        this.enableLombok = source["enableLombok"];
	        this.enableRemoveIsPrefix = source["enableRemoveIsPrefix"];
	        this.enableTableFieldAnnotation = source["enableTableFieldAnnotation"];
	        this.enableActiveRecord = source["enableActiveRecord"];
	        this.versionColumnName = source["versionColumnName"];
	        this.versionPropertyName = source["versionPropertyName"];
	        this.logicDeleteColumnName = source["logicDeleteColumnName"];
	        this.logicDeletePropertyName = source["logicDeletePropertyName"];
	        this.naming = source["naming"];
	        this.addSuperEntityColumns = source["addSuperEntityColumns"];
	        this.addIgnoreColumns = source["addIgnoreColumns"];
	        this.addTableFills = this.convertValues(source["addTableFills"], FieldTypeKeyVal);
	        this.idType = source["idType"];
	        this.formatFileName = source["formatFileName"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class StrategyConfig {
	    enableCapitalMode: boolean;
	    enableSkipView: boolean;
	    disableSqlFilter: boolean;
	    enableSchema: boolean;
	    addIncludes: string[];
	    entity?: Entity;
	    controller?: Controller;
	    mapper?: Mapper;
	    service?: Service;
	
	    static createFrom(source: any = {}) {
	        return new StrategyConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.enableCapitalMode = source["enableCapitalMode"];
	        this.enableSkipView = source["enableSkipView"];
	        this.disableSqlFilter = source["disableSqlFilter"];
	        this.enableSchema = source["enableSchema"];
	        this.addIncludes = source["addIncludes"];
	        this.entity = this.convertValues(source["entity"], Entity);
	        this.controller = this.convertValues(source["controller"], Controller);
	        this.mapper = this.convertValues(source["mapper"], Mapper);
	        this.service = this.convertValues(source["service"], Service);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	
	export class DataSourceConfig {
	    typ: string;
	    database: string;
	    username: string;
	    password: string;
	    host: string;
	    port: number;
	
	    static createFrom(source: any = {}) {
	        return new DataSourceConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.typ = source["typ"];
	        this.database = source["database"];
	        this.username = source["username"];
	        this.password = source["password"];
	        this.host = source["host"];
	        this.port = source["port"];
	    }
	}
	export class DatabaseOptions {
	    name: string;
	    comment: string;
	
	    static createFrom(source: any = {}) {
	        return new DatabaseOptions(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.comment = source["comment"];
	    }
	}
	export class TemplateConfig {
	    disableAll: boolean;
	    disable: boolean;
	    entity: string;
	    entityKt: string;
	    service: string;
	    serviceImpl: string;
	    mapper: string;
	    mapperXml: string;
	    controller: string;
	
	    static createFrom(source: any = {}) {
	        return new TemplateConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.disableAll = source["disableAll"];
	        this.disable = source["disable"];
	        this.entity = source["entity"];
	        this.entityKt = source["entityKt"];
	        this.service = source["service"];
	        this.serviceImpl = source["serviceImpl"];
	        this.mapper = source["mapper"];
	        this.mapperXml = source["mapperXml"];
	        this.controller = source["controller"];
	    }
	}
	export class PathInfo {
	    name: string;
	    value: string;
	
	    static createFrom(source: any = {}) {
	        return new PathInfo(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.value = source["value"];
	    }
	}
	export class PackageConfig {
	    parent: string;
	    moduleName: string;
	    entity: string;
	    service: string;
	    serviceImpl: string;
	    mapper: string;
	    mapperXml: string;
	    controller: string;
	    other: string;
	    pathInfo: PathInfo[];
	
	    static createFrom(source: any = {}) {
	        return new PackageConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.parent = source["parent"];
	        this.moduleName = source["moduleName"];
	        this.entity = source["entity"];
	        this.service = source["service"];
	        this.serviceImpl = source["serviceImpl"];
	        this.mapper = source["mapper"];
	        this.mapperXml = source["mapperXml"];
	        this.controller = source["controller"];
	        this.other = source["other"];
	        this.pathInfo = this.convertValues(source["pathInfo"], PathInfo);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class GlobalConfig {
	    fileOverride: boolean;
	    disableOpenDir: boolean;
	    outputDir: string;
	    author: string;
	    enableKotlin: boolean;
	    enableSwagger: boolean;
	    dateType: string;
	    commentDate: string;
	
	    static createFrom(source: any = {}) {
	        return new GlobalConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.fileOverride = source["fileOverride"];
	        this.disableOpenDir = source["disableOpenDir"];
	        this.outputDir = source["outputDir"];
	        this.author = source["author"];
	        this.enableKotlin = source["enableKotlin"];
	        this.enableSwagger = source["enableSwagger"];
	        this.dateType = source["dateType"];
	        this.commentDate = source["commentDate"];
	    }
	}
	export class ConfigContext {
	    dataSource?: DataSourceConfig;
	    globalConfig?: GlobalConfig;
	    packageConfig?: PackageConfig;
	    templateConfig?: TemplateConfig;
	    strategyConfig?: StrategyConfig;
	
	    static createFrom(source: any = {}) {
	        return new ConfigContext(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.dataSource = this.convertValues(source["dataSource"], DataSourceConfig);
	        this.globalConfig = this.convertValues(source["globalConfig"], GlobalConfig);
	        this.packageConfig = this.convertValues(source["packageConfig"], PackageConfig);
	        this.templateConfig = this.convertValues(source["templateConfig"], TemplateConfig);
	        this.strategyConfig = this.convertValues(source["strategyConfig"], StrategyConfig);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	
	
	

}

export namespace crypt {
	
	export class CryptConfig {
	    typ: string;
	    cost: string;
	    source: string;
	    publicKey: string;
	    privateKey: string;
	
	    static createFrom(source: any = {}) {
	        return new CryptConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.typ = source["typ"];
	        this.cost = source["cost"];
	        this.source = source["source"];
	        this.publicKey = source["publicKey"];
	        this.privateKey = source["privateKey"];
	    }
	}

}

