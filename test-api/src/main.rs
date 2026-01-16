#[tokio::main]
async fn main() {

    println!("Setting;Old value;New value;meanFPS;meanPower;Timestamp");

    let slo_id = match std::env::var("SLO_ID"){
        Ok(val) => val,
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        "ServiceSLO".to_owned()},
    };

    let conf_param = match std::env::var("CONF_PARAM"){
        Ok(val) => val,
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        "ServiceParam".to_owned()},
    };

    let sleep_interval = match std::env::var("SLEEP_INTERVAL"){
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        10},
    };

    let sleep_duration = std::time::Duration::new(sleep_interval, 0);

    let slo_min = match std::env::var("SLO_MIN"){
        Ok(val) => val.parse::<f64>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        24.0},
    };

    let slo_max = match std::env::var("SLO_MAX"){
        Ok(val) => val.parse::<f64>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        30.0},
    };

    let param_min = match std::env::var("PARAM_MIN"){
        Ok(val) => val.parse::<i32>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        0},
    };

    let param_max = match std::env::var("PARAM_MAX"){
        Ok(val) => val.parse::<i32>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        16},
    };

    let lambda = match std::env::var("PARAM_CORRELATION"){
        Ok(val) => val.parse::<i32>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        1},
    };

    let delta = match std::env::var("PARAM_CHANGE"){
        Ok(val) => val.parse::<i32>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        1},
    };

    let api_host = match std::env::var("API_HOST"){
        Ok(val) => val,
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        "http://localhost:8080/v0".to_owned()},
    };

    let max_iter = match std::env::var("NUM_STEPS"){
        Ok(val) => val.parse::<i32>().unwrap(),
        Err(e) => {
        eprintln!("Error loading environment variable SLO_ID: {}", e);
        0},
    };

    let mut infinite_looping = false;

    if max_iter <= 0 {
        infinite_looping = true;
        eprintln!("Looping infinitely");
    }
    else{
        eprintln!("Taking {} steps", max_iter);
    }

    let mut api_config = ServiceAPIClient::apis::configuration::Configuration::new();
    api_config.base_path = api_host.to_owned();

    let mut it = 0;

    while it < max_iter && !infinite_looping {
        it += 1;
        eprintln!("Iteration {}", it);
        match get_slo(&api_config, &slo_id).await {
            Some(val) => {
                let slo_value = val;
                eprintln!("Got SLO value {}", slo_value);
                let mut var_amount = 0i32;
                if slo_value > slo_max {
                    var_amount -= lambda*delta;
                    eprintln!("SLO too high, decreasing");
                }
                else {
                    if slo_value < slo_min{
                        var_amount += lambda*delta;
                        eprintln!("SLO too low, increasing");
                    }
                }
                match get_param(&api_config, &conf_param).await {
                    Some(pval) => {
                        let config_value = pval;
                        eprintln!("Got original config value {}", config_value);
                        let mut new_config_value = config_value;
                        if var_amount != 0 {
                            new_config_value += var_amount;
                            if new_config_value > param_max {
                                new_config_value = param_max;
                            }
                            else{
                                if new_config_value < param_min {
                                    new_config_value = param_min;
                                }
                            }
                            match change_param(&api_config, &conf_param, new_config_value).await{
                                Ok(_v) => {
                                    eprintln!("Successfully changed value to {}", new_config_value);
                                },
                                Err(e) => {
                                    eprintln!("Error changing parameter: {}", e);
                                    it -= 1;
                                }
                            }
                        }
                        else{
                            eprintln!("All good, no changes were made");
                        }
                        let ts = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
                        let power_slo = match get_slo(&api_config, "meanPower").await {
                            Some(v) => v,
                            None => -1f64,
                        }; // Only for reporting purposes
                        println!("{};{};{};{};{};{}", slo_id, config_value, new_config_value, slo_value, power_slo, ts);
                        std::thread::sleep(sleep_duration);
                    },
                    None => {
                        it -= 1; // This iteration "doesn't count" 
                    }
                }
            },
            None => {
                it -= 1; // This iteration "doesn't count" 
            },
        }
    }
}

async fn get_slo(api_config: &ServiceAPIClient::apis::configuration::Configuration, slo_id: &str) -> Option<f64>{
    use ServiceAPIClient::models::slo_value::SloValue::{Boolean, String, Number};
    let slo_got = match ServiceAPIClient::apis::slo_api::slo_get(api_config, slo_id).await{
        Ok(val) => Some(val),
        Err(e) => {
            eprintln!("Error getting SLO: {}", e);
            None
        },
    };
    let slo_value: Option<f64> = match slo_got {
        None => None,
        Some(slo_inner) => {
            match *slo_inner.value.unwrap() {
                Boolean(b) => if b {Some(1f64)} else {Some(0f64)},
                String(_s) => Some(0f64),
                Number(n) => Some(n),
                }
        }
    };
    return slo_value;
}

async fn get_param(api_config: &ServiceAPIClient::apis::configuration::Configuration, param_id: &str) -> Option<i32> {
    use ServiceAPIClient::models::conf_param_value::ConfParamValue::{Boolean, String, Number};
    let param_got = match ServiceAPIClient::apis::config_api::config_get(api_config, param_id).await{
        Ok(val) => Some(val),
        Err(_e) => None,
    };
    let param_value: Option<i32> = match param_got{
        None => None,
        Some(param_inner) => {
            match *param_inner.value {
                    Boolean(b) => if b {Some(1i32)} else {Some(0i32)},
                    String(_s) => Some(0i32),
                    Number(n) => Some(n as i32),
                }
        }
    };
    return param_value;
}

async fn change_param(api_config: &ServiceAPIClient::apis::configuration::Configuration, param_id: &str, new_value: i32) -> Result<(), ServiceAPIClient::apis::Error<ServiceAPIClient::apis::config_api::ConfigChangeError>>{
    let change_target = ServiceAPIClient::models::slo_description_target::SloDescriptionTarget::NumberInt(new_value);
    let conf_change = Some(ServiceAPIClient::models::conf_param_change::ConfParamChange::new(change_target));
    return ServiceAPIClient::apis::config_api::config_change(api_config, param_id, conf_change).await;
}