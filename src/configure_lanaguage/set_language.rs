#![allow(unused)]
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

use crate::run_commands::run_command;

const LANGUAGES: [&str; 489] = [
    "aa_DJ.UTF-8 UTF-8",
    "aa_DJ ISO-8859-1",
    "aa_ER UTF-8",
    "aa_ET UTF-8",
    "af_ZA.UTF-8 UTF-8",
    "af_ZA ISO-8859-1",
    "agr_PE UTF-8",
    "ak_GH UTF-8",
    "am_ET UTF-8",
    "an_ES.UTF-8 UTF-8",
    "an_ES ISO-8859-15",
    "anp_IN UTF-8",
    "ar_AE.UTF-8 UTF-8",
    "ar_AE ISO-8859-6",
    "ar_BH.UTF-8 UTF-8",
    "ar_BH ISO-8859-6",
    "ar_DZ.UTF-8 UTF-8",
    "ar_DZ ISO-8859-6",
    "ar_EG.UTF-8 UTF-8",
    "ar_EG ISO-8859-6",
    "ar_IN UTF-8",
    "ar_IQ.UTF-8 UTF-8",
    "ar_IQ ISO-8859-6",
    "ar_JO.UTF-8 UTF-8",
    "ar_JO ISO-8859-6",
    "ar_KW.UTF-8 UTF-8",
    "ar_KW ISO-8859-6",
    "ar_LB.UTF-8 UTF-8",
    "ar_LB ISO-8859-6",
    "ar_LY.UTF-8 UTF-8",
    "ar_LY ISO-8859-6",
    "ar_MA.UTF-8 UTF-8",
    "ar_MA ISO-8859-6",
    "ar_OM.UTF-8 UTF-8",
    "ar_OM ISO-8859-6",
    "ar_QA.UTF-8 UTF-8",
    "ar_QA ISO-8859-6",
    "ar_SA.UTF-8 UTF-8",
    "ar_SA ISO-8859-6",
    "ar_SD.UTF-8 UTF-8",
    "ar_SD ISO-8859-6",
    "ar_SS UTF-8",
    "ar_SY.UTF-8 UTF-8",
    "ar_SY ISO-8859-6",
    "ar_TN.UTF-8 UTF-8",
    "ar_TN ISO-8859-6",
    "ar_YE.UTF-8 UTF-8",
    "ar_YE ISO-8859-6",
    "ayc_PE UTF-8",
    "az_AZ UTF-8",
    "az_IR UTF-8",
    "as_IN UTF-8",
    "ast_ES.UTF-8 UTF-8",
    "ast_ES ISO-8859-15",
    "be_BY.UTF-8 UTF-8",
    "be_BY CP1251",
    "be_BY@latin UTF-8",
    "bem_ZM UTF-8",
    "ber_DZ UTF-8",
    "ber_MA UTF-8",
    "bg_BG.UTF-8 UTF-8",
    "bg_BG CP1251",
    "bhb_IN.UTF-8 UTF-8",
    "bho_IN UTF-8",
    "bho_NP UTF-8",
    "bi_VU UTF-8",
    "bn_BD UTF-8",
    "bn_IN UTF-8",
    "bo_CN UTF-8",
    "bo_IN UTF-8",
    "br_FR.UTF-8 UTF-8",
    "br_FR ISO-8859-1",
    "br_FR@euro ISO-8859-15",
    "brx_IN UTF-8",
    "bs_BA.UTF-8 UTF-8",
    "bs_BA ISO-8859-2",
    "byn_ER UTF-8",
    "ca_AD.UTF-8 UTF-8",
    "ca_AD ISO-8859-15",
    "ca_ES.UTF-8 UTF-8",
    "ca_ES ISO-8859-1",
    "ca_ES@euro ISO-8859-15",
    "ca_ES@valencia UTF-8",
    "ca_FR.UTF-8 UTF-8",
    "ca_FR ISO-8859-15",
    "ca_IT.UTF-8 UTF-8",
    "ca_IT ISO-8859-15",
    "ce_RU UTF-8",
    "chr_US UTF-8",
    "ckb_IQ UTF-8",
    "cmn_TW UTF-8",
    "crh_RU UTF-8",
    "crh_UA UTF-8",
    "cs_CZ.UTF-8 UTF-8",
    "cs_CZ ISO-8859-2",
    "csb_PL UTF-8",
    "cv_RU UTF-8",
    "cy_GB.UTF-8 UTF-8",
    "cy_GB ISO-8859-14",
    "da_DK.UTF-8 UTF-8",
    "da_DK ISO-8859-1",
    "de_AT.UTF-8 UTF-8",
    "de_AT ISO-8859-1",
    "de_AT@euro ISO-8859-15",
    "de_BE.UTF-8 UTF-8",
    "de_BE ISO-8859-1",
    "de_BE@euro ISO-8859-15",
    "de_CH.UTF-8 UTF-8",
    "de_CH ISO-8859-1",
    "de_DE.UTF-8 UTF-8",
    "de_DE ISO-8859-1",
    "de_DE@euro ISO-8859-15",
    "de_IT.UTF-8 UTF-8",
    "de_IT ISO-8859-1",
    "de_LI.UTF-8 UTF-8",
    "de_LU.UTF-8 UTF-8",
    "de_LU ISO-8859-1",
    "de_LU@euro ISO-8859-15",
    "doi_IN UTF-8",
    "dsb_DE UTF-8",
    "dv_MV UTF-8",
    "dz_BT UTF-8",
    "el_GR.UTF-8 UTF-8",
    "el_GR ISO-8859-7",
    "el_GR@euro ISO-8859-7",
    "el_CY.UTF-8 UTF-8",
    "el_CY ISO-8859-7",
    "en_AG UTF-8",
    "en_AU.UTF-8 UTF-8",
    "en_AU ISO-8859-1",
    "en_BW.UTF-8 UTF-8",
    "en_BW ISO-8859-1",
    "en_CA.UTF-8 UTF-8",
    "en_CA ISO-8859-1",
    "en_DK.UTF-8 UTF-8",
    "en_DK ISO-8859-1",
    "en_GB.UTF-8 UTF-8",
    "en_GB ISO-8859-1",
    "en_HK.UTF-8 UTF-8",
    "en_HK ISO-8859-1",
    "en_IE.UTF-8 UTF-8",
    "en_IE ISO-8859-1",
    "en_IE@euro ISO-8859-15",
    "en_IL UTF-8",
    "en_IN UTF-8",
    "en_NG UTF-8",
    "en_NZ.UTF-8 UTF-8",
    "en_NZ ISO-8859-1",
    "en_PH.UTF-8 UTF-8",
    "en_PH ISO-8859-1",
    "en_SC.UTF-8 UTF-8",
    "en_SG.UTF-8 UTF-8",
    "en_SG ISO-8859-1",
    "en_US.UTF-8 UTF-8",
    "en_US ISO-8859-1",
    "en_ZA.UTF-8 UTF-8",
    "en_ZA ISO-8859-1",
    "en_ZM UTF-8",
    "en_ZW.UTF-8 UTF-8",
    "en_ZW ISO-8859-1",
    "eo UTF-8",
    "es_AR.UTF-8 UTF-8",
    "es_AR ISO-8859-1",
    "es_BO.UTF-8 UTF-8",
    "es_BO ISO-8859-1",
    "es_CL.UTF-8 UTF-8",
    "es_CL ISO-8859-1",
    "es_CO.UTF-8 UTF-8",
    "es_CO ISO-8859-1",
    "es_CR.UTF-8 UTF-8",
    "es_CR ISO-8859-1",
    "es_CU UTF-8",
    "es_DO.UTF-8 UTF-8",
    "es_DO ISO-8859-1",
    "es_EC.UTF-8 UTF-8",
    "es_EC ISO-8859-1",
    "es_ES.UTF-8 UTF-8",
    "es_ES ISO-8859-1",
    "es_ES@euro ISO-8859-15",
    "es_GT.UTF-8 UTF-8",
    "es_GT ISO-8859-1",
    "es_HN.UTF-8 UTF-8",
    "es_HN ISO-8859-1",
    "es_MX.UTF-8 UTF-8",
    "es_MX ISO-8859-1",
    "es_NI.UTF-8 UTF-8",
    "es_NI ISO-8859-1",
    "es_PA.UTF-8 UTF-8",
    "es_PA ISO-8859-1",
    "es_PE.UTF-8 UTF-8",
    "es_PE ISO-8859-1",
    "es_PR.UTF-8 UTF-8",
    "es_PR ISO-8859-1",
    "es_PY.UTF-8 UTF-8",
    "es_PY ISO-8859-1",
    "es_SV.UTF-8 UTF-8",
    "es_SV ISO-8859-1",
    "es_US.UTF-8 UTF-8",
    "es_US ISO-8859-1",
    "es_UY.UTF-8 UTF-8",
    "es_UY ISO-8859-1",
    "es_VE.UTF-8 UTF-8",
    "es_VE ISO-8859-1",
    "et_EE.UTF-8 UTF-8",
    "et_EE ISO-8859-15",
    "eu_ES.UTF-8 UTF-8",
    "eu_ES ISO-8859-1",
    "eu_ES@euro ISO-8859-15",
    "eu_FR.UTF-8 UTF-8",
    "eu_FR ISO-8859-1",
    "fa_IR UTF-8",
    "ff_SN UTF-8",
    "fi_FI.UTF-8 UTF-8",
    "fi_FI ISO-8859-1",
    "fi_FI@euro ISO-8859-15",
    "fil_PH UTF-8",
    "fo_FO.UTF-8 UTF-8",
    "fo_FO ISO-8859-1",
    "fr_BE.UTF-8 UTF-8",
    "fr_BE ISO-8859-1",
    "fr_BE@euro ISO-8859-15",
    "fr_CA.UTF-8 UTF-8",
    "fr_CA ISO-8859-1",
    "fr_CH.UTF-8 UTF-8",
    "fr_CH ISO-8859-1",
    "fr_FR.UTF-8 UTF-8",
    "fr_FR ISO-8859-1",
    "fr_FR@euro ISO-8859-15",
    "fr_LU.UTF-8 UTF-8",
    "fr_LU ISO-8859-1",
    "fr_LU@euro ISO-8859-15",
    "fur_IT UTF-8",
    "fy_DE UTF-8",
    "fy_NL.UTF-8 UTF-8",
    "fy_NL ISO-8859-1",
    "ga_IE.UTF-8 UTF-8",
    "ga_IE ISO-8859-1",
    "ga_IE@euro ISO-8859-15",
    "gd_GB.UTF-8 UTF-8",
    "gd_GB ISO-8859-15",
    "gez_ER UTF-8",
    "gez_ER@abegede UTF-8",
    "gez_ET UTF-8",
    "gez_ET@abegede UTF-8",
    "gl_ES.UTF-8 UTF-8",
    "gl_ES ISO-8859-1",
    "gl_ES@euro ISO-8859-15",
    "gu_IN UTF-8",
    "gv_GB.UTF-8 UTF-8",
    "gv_GB ISO-8859-1",
    "gv_GB ISO-8859-14",
    "ha_NG UTF-8",
    "hak_TW UTF-8",
    "he_IL.UTF-8 UTF-8",
    "he_IL ISO-8859-8",
    "hi_IN UTF-8",
    "hne_IN UTF-8",
    "hr_HR.UTF-8 UTF-8",
    "hr_HR ISO-8859-2",
    "hsb_DE UTF-8",
    "ht_HT UTF-8",
    "hu_HU.UTF-8 UTF-8",
    "hu_HU ISO-8859-2",
    "hy_AM.UTF-8 UTF-8",
    "hy_AM ARMSCII-8",
    "ia_FR UTF-8",
    "id_ID.UTF-8 UTF-8",
    "id_ID ISO-8859-1",
    "ig_NG UTF-8",
    "ik_CA UTF-8",
    "is_IS.UTF-8 UTF-8",
    "is_IS ISO-8859-1",
    "it_CH.UTF-8 UTF-8",
    "it_CH ISO-8859-1",
    "it_IT.UTF-8 UTF-8",
    "it_IT ISO-8859-1",
    "it_IT@euro ISO-8859-15",
    "iu_CA UTF-8",
    "iw_IL UTF-8",
    "ja_JP.UTF-8 UTF-8",
    "ka_GE.UTF-8 UTF-8",
    "ka_GE GEORGIAN-PS",
    "kab_DZ UTF-8",
    "kk_KZ.UTF-8 UTF-8",
    "kk_KZ PT154",
    "kl_GL.UTF-8 UTF-8",
    "kl_GL ISO-8859-1",
    "km_KH UTF-8",
    "kn_IN UTF-8",
    "kok_IN UTF-8",
    "ko_KR.UTF-8 UTF-8",
    "ko_KR.EUC-KR EUC-KR",
    "ks_IN UTF-8",
    "ks_IN@devanagari UTF-8",
    "ku_TR.UTF-8 UTF-8",
    "ku_TR ISO-8859-9",
    "kw_GB.UTF-8 UTF-8",
    "kw_GB ISO-8859-1",
    "ky_KG UTF-8",
    "lb_LU.UTF-8 UTF-8",
    "lg_UG.UTF-8 UTF-8",
    "lg_UG ISO-8859-10",
    "li_BE UTF-8",
    "li_NL UTF-8",
    "lij_IT UTF-8",
    "ln_CD UTF-8",
    "lo_LA UTF-8",
    "lt_LT.UTF-8 UTF-8",
    "lt_LT ISO-8859-13",
    "lt_LT ISO-8859-4",
    "lv_LV.UTF-8 UTF-8",
    "lv_LV ISO-8859-13",
    "lzh_TW UTF-8",
    "mag_IN UTF-8",
    "mai_IN UTF-8",
    "mai_NP UTF-8",
    "mfe_MU UTF-8",
    "mg_MG.UTF-8 UTF-8",
    "mg_MG ISO-8859-15",
    "mhr_RU UTF-8",
    "mi_NZ.UTF-8 UTF-8",
    "mi_NZ ISO-8859-1",
    "mk_MK.UTF-8 UTF-8",
    "mk_MK ISO-8859-5",
    "ml_IN UTF-8",
    "mn_MN UTF-8",
    "mni_IN UTF-8",
    "mr_IN UTF-8",
    "ms_MY.UTF-8 UTF-8",
    "ms_MY ISO-8859-1",
    "mt_MT.UTF-8 UTF-8",
    "mt_MT ISO-8859-3",
    "my_MM UTF-8",
    "nan_TW UTF-8",
    "nan_TW@latin UTF-8",
    "nb_NO.UTF-8 UTF-8",
    "nb_NO ISO-8859-1",
    "nds_DE UTF-8",
    "nds_NL UTF-8",
    "ne_NP UTF-8",
    "nhn_MX UTF-8",
    "niu_NU UTF-8",
    "niu_NZ UTF-8",
    "nl_AW UTF-8",
    "nl_BE.UTF-8 UTF-8",
    "nl_BE ISO-8859-1",
    "nl_BE@euro ISO-8859-15",
    "nl_NL.UTF-8 UTF-8",
    "nl_NL ISO-8859-1",
    "nl_NL@euro ISO-8859-15",
    "nn_NO.UTF-8 UTF-8",
    "nn_NO ISO-8859-1",
    "nr_ZA UTF-8",
    "nso_ZA UTF-8",
    "oc_FR.UTF-8 UTF-8",
    "oc_FR ISO-8859-1",
    "oc_FR@euro ISO-8859-15",
    "om_ET UTF-8",
    "om_KE.UTF-8 UTF-8",
    "om_KE ISO-8859-1",
    "or_IN UTF-8",
    "os_RU UTF-8",
    "pa_IN UTF-8",
    "pap_AN UTF-8",
    "pap_AW UTF-8",
    "pap_CW UTF-8",
    "pl_PL.UTF-8 UTF-8",
    "pl_PL ISO-8859-2",
    "ps_AF UTF-8",
    "pt_BR.UTF-8 UTF-8",
    "pt_BR ISO-8859-1",
    "pt_PT.UTF-8 UTF-8",
    "pt_PT ISO-8859-1",
    "pt_PT@euro ISO-8859-15",
    "quz_PE UTF-8",
    "raj_IN UTF-8",
    "ro_RO.UTF-8 UTF-8",
    "ro_RO ISO-8859-2",
    "ru_RU.UTF-8 UTF-8",
    "ru_RU ISO-8859-5",
    "ru_UA.UTF-8 UTF-8",
    "ru_UA KOI8-U",
    "rw_RW UTF-8",
    "sa_IN UTF-8",
    "sat_IN UTF-8",
    "sc_IT UTF-8",
    "sd_IN UTF-8",
    "sd_IN@devanagari UTF-8",
    "se_NO UTF-8",
    "sgs_LT UTF-8",
    "shn_MM UTF-8",
    "shs_CA UTF-8",
    "sid_ET UTF-8",
    "si_LK UTF-8",
    "sk_SK.UTF-8 UTF-8",
    "sk_SK ISO-8859-2",
    "sl_SI.UTF-8 UTF-8",
    "sl_SI ISO-8859-2",
    "sm_WS UTF-8",
    "so_DJ.UTF-8 UTF-8",
    "so_DJ ISO-8859-1",
    "so_ET UTF-8",
    "so_KE.UTF-8 UTF-8",
    "so_KE ISO-8859-1",
    "so_SO.UTF-8 UTF-8",
    "so_SO ISO-8859-1",
    "sq_AL.UTF-8 UTF-8",
    "sq_AL ISO-8859-1",
    "sq_MK UTF-8",
    "sr_ME UTF-8",
    "sr_RS.UTF-8 UTF-8",
    "sr_RS ISO-8859-2",
    "sr_RS CP1251",
    "sr_RS@latin UTF-8",
    "sr_RS@latin ISO-8859-2",
    "ss_ZA UTF-8",
    "st_ZA.UTF-8 UTF-8",
    "st_ZA ISO-8859-1",
    "sv_FI.UTF-8 UTF-8",
    "sv_FI ISO-8859-1",
    "sv_FI@euro ISO-8859-15",
    "sv_SE.UTF-8 UTF-8",
    "sv_SE ISO-8859-1",
    "sw_KE.UTF-8 UTF-8",
    "sw_KE ISO-8859-1",
    "sw_TZ.UTF-8 UTF-8",
    "sw_TZ ISO-8859-1",
    "szl_PL UTF-8",
    "ta_IN UTF-8",
    "ta_LK UTF-8",
    "tcy_IN.UTF-8 UTF-8",
    "te_IN UTF-8",
    "tg_TJ.UTF-8 UTF-8",
    "tg_TJ KOI8-T",
    "th_TH.UTF-8 UTF-8",
    "th_TH TIS-620",
    "the_NP UTF-8",
    "ti_ER UTF-8",
    "ti_ET UTF-8",
    "tig_ER UTF-8",
    "tk_TM UTF-8",
    "tl_PH.UTF-8 UTF-8",
    "tl_PH ISO-8859-1",
    "tn_ZA UTF-8",
    "to_TO UTF-8",
    "tpi_PG UTF-8",
    "tr_CY.UTF-8 UTF-8",
    "tr_CY ISO-8859-9",
    "tr_TR.UTF-8 UTF-8",
    "tr_TR ISO-8859-9",
    "ts_ZA UTF-8",
    "tt_RU UTF-8",
    "tt_RU TATAR-CYR",
    "ug_CN UTF-8",
    "uk_UA.UTF-8 UTF-8",
    "uk_UA KOI8-U",
    "unm_US UTF-8",
    "ur_IN UTF-8",
    "ur_PK.UTF-8 UTF-8",
    "ur_PK CP1256",
    "uz_UZ UTF-8",
    "uz_UZ ISO-8859-1",
    "uz_UZ@cyrillic UTF-8",
    "ve_ZA UTF-8",
    "vi_VN UTF-8",
    "wa_BE.UTF-8 UTF-8",
    "wa_BE ISO-8859-1",
    "wa_BE@euro ISO-8859-15",
    "wae_CH UTF-8",
    "wal_ET UTF-8",
    "wo_SN UTF-8",
    "xh_ZA.UTF-8 UTF-8",
    "xh_ZA ISO-8859-1",
    "yi_US.UTF-8 UTF-8",
    "yi_US CP1255",
    "yo_NG UTF-8",
    "yue_HK UTF-8",
    "zh_CN.UTF-8 UTF-8",
    "zh_CN.GB18030 GB18030",
    "zh_CN.GBK GBK",
    "zh_HK.UTF-8 UTF-8",
    "zh_HK BIG5-HKSCS",
    "zh_SG.UTF-8 UTF-8",
    "zh_SG.GB18030 GB18030",
    "zh_SG.GBK GBK",
    "zh_TW.UTF-8 UTF-8",
    "zh_TW BIG5",
    "zu_ZA.UTF-8 UTF-8",
    "zu_ZA ISO-8859-1",
];
pub fn set_language() -> Result<(), String> {
    println!("Configurando linguagem do sistema...");
    println!("Selcione com espaço");

    let language_selected = get_user_selections();
    println!("Você escolheu: ");

    if language_selected.len() < 1 {
        return Err("set_language()".to_string());
    }

    for selection in &language_selected {
        println!("{}", selection)
    }

    edit_locale_gen(language_selected.clone())?;

    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language_selected.clone())?;

    println!("Linguagem do sistema configurada com sucesso.");
    Ok(())
}

fn get_user_selections() -> Vec<String> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecione uma linguagem")
        .items(&LANGUAGES)
        .interact()
        .unwrap();

    let filtered_selections: Vec<String> = selections
        .into_iter()
        .filter(|&i| !LANGUAGES[i].contains("ISO"))
        .map(|i| LANGUAGES[i].to_string())
        .collect();

    filtered_selections
}

fn edit_locale_gen(language: Vec<String>) -> Result<(), String> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {}: {}", locale_gen_path, e))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| format!("Falha ao ler linha: {}", e))?;
        if line.trim() == format!("#{}", language[0].trim()) {
            line = language[0].to_string();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_gen_path, e))?;

    for line in lines {
        writeln!(file, "{}", line);
    }

    Ok(())
}

fn configure_locale_conf(language: Vec<String>) -> Result<(), String> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!("LANG={}\n", language[0]);

    let file = File::create_new(locale_conf_path).map_err(|e| format!("sla: {}", e));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_conf_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_conf_path, e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Falha ao escrever no {}: {}", locale_conf_path, e))?;
    Ok(())
}
