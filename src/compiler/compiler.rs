fn execute(query : Query) -> bool {
    match Query.validate() {
        CreateQuery => fs_manager::create(query[name], query[args]),
        DropTable => fs_manager::drop(query[name]),
        Insert => fs_manager::append(query[args]),
        Update => {
            fs_manager::put_content( fs_manager::get_content(query[args]) > query.execute() ) 
        },
        Delete => {
            fs_manager::put_content( fs_manager::get_content(query[args]) > query.execute() )
        },
        Select => {
            project (fs_manager::get_content(query[args]) > query.execute())
        }
    }
}
