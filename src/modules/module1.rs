/*
 * - Creating a module - network
 *  - all the functions inside this module will be in network module scope
 *  - module can be nested with in module
 *  - Accessing module functions
 *      - network::connect()
 */
mod api {
    fn product() {}

    mod user {
        // can have same name as scope of this function is only with client module or client namespace
        fn register() {}
    }
}

// can have multiple modules in same files
mod email {
    fn send() {}
}
