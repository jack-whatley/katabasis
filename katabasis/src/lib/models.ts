// A typescript implementation of the FrontendCollection struct on the
// manager crate.
export interface ICollection {
    name: string;
    target: string;
    plugins: Array<IPlugin>;
    modLoader: string;
}

export interface IPlugin {
    enabled: boolean;
    installTime: Date;
    ident: string;
    fullName: string;
}
