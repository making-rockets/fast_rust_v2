use fast_common::common::orm_config::RB;
use fast_common::models::menu::{Menu, MenuVo};

use chrono::NaiveDateTime;

use fast_common::models::role::RoleVo;
use fast_common::models::user::User;

pub struct MenuService {}

impl MenuService {
    pub async fn add(mut menu: Menu) -> Result<> {
        let id = 1 as u64;
        menu.id = Some(id);
        menu.create_time = Some(NaiveDateTime::now());
        let result = RB.save(&menu, &[]).await?;
        return Ok(result);
    }

    pub async fn update(mut menu: Menu) -> Result<u64> {
        let mut wrapper = RB.new_wrapper();
        let result = RB.update_by_wrapper(&mut menu, wrapper, &[]).await;
        return result;
    }

    pub async fn delete(menu: Menu) -> Result<u64> {
        let mut wrapper = RB.new_wrapper();
        let x = RB.remove_by_wrapper::<Menu>(wrapper).await;
        //let result  = RB.remove_by_id::<Menu>("", &menu.id.unwrap()).await;
        return x;
    }

    pub async fn list(arg: MenuVo) -> Result<Page<Menu>> {
        let mut wrapper = RB.new_wrapper();
        if arg.id.is_some() {
            wrapper = wrapper.eq("id", &arg.id);
        }
        /* if arg.user_name.is_some() {
            wrapper = wrapper.like("%user_name%", &arg.user_name);
        }
        if arg.age.is_some() {
            wrapper = wrapper.eq("age", &arg.user_name);
        }*/
        if arg.create_time.is_some() {
            wrapper = wrapper.gt("create_time", arg.create_time);
        }

        let page_request = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap());
        let page = RB.fetch_page_by_wrapper(wrapper, &page_request).await;
        return page;
    }

    pub async fn find_menus_by_role(role_id: u64) -> Result<Vec<Menu>> {
        let mut wrapper = RB.new_wrapper();
        wrapper = wrapper.do_if(false, |wrapper| wrapper.eq("role_id", role_id));
        let x = RB.fetch_list_by_wrapper::<Menu>(wrapper).await;
        return x;
    }
}
