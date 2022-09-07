#[allow(unused_imports)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise, env};
use near_sdk::serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub type AccountId = String;

#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Msg {
    recipient: String,
    msg: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Packages {
    sub_package: String,
    channels: Vec<String>,
    price: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    package_id: String,
    user: String,
}

#[near_bindgen]
pub struct Media {
    packages:HashMap<String, Packages>,
    messages:Vec<Msg>,
    subscriptions:Vec<Subscription>,   
}

#[near_bindgen]
impl Media {
    #[init]
    pub fn default_package() -> Self {
        let mut packages:HashMap<String, Packages> = HashMap::new();
        let messages:Vec<Msg> = Vec::new();
        let subscriptions:Vec<Subscription> = Vec::new();

        packages.insert("Nyota".to_string(), 
        Packages { sub_package: "Nyota".to_string(), 
        channels:vec![
         "https://live-hls-web-aje.getaj.net/AJE/index.m3u8".to_string(), 
         "http://210.210.155.37/dr9445/h/h02/index.m3u8".to_string(), 
         "https://d10rltuy0iweup.cloudfront.net/ATNNAT/myStream/playlist.m3u8".to_string(), 
         "https://d2g7v53450s2i2.cloudfront.net/ATNUS/myStream/playlist.m3u8".to_string(), 
         "https://livecdn.fptplay.net/hda3/babytvhd_vhls.smil/chunklist.m3u8".to_string(),
         "https://59d39900ebfb8.streamlock.net/bahartv/bahartv/playlist.m3u8".to_string(),
         "https://raw.githubusercontent.com/taodicakhia/IPTV_Exception/master/channels/af/tolomusic.m3u8".to_string(),
         "http://51.210.199.56/hls/stream.m3u8".to_string(),
         "https://livecdn.fptplay.net/hda2/bbcearth_vhls.smil/chunklist.m3u8".to_string(),
         "https://cdnuk001.broadcastcdn.net/KUK-BBCNEWSHD/index.m3u8".to_string(),
         "https://livecdn.fptplay.net/sdb/bbcnew_2000.stream/chunklist.m3u8".to_string(),
         "https://liveprodapnortheast.global.ssl.fastly.net/ap1/Channel-APTVqvs-AWS-tokyo-1/Source-APTVqvs-1000-1_live.m3u8".to_string(),
         "https://www.bloomberg.com/media-manifest/streams/asia-event.m3u8".to_string(),
         "http://198.16.106.62:8278/streams/d/Cn/playlist.m3u8".to_string(),
         "http://210.210.155.35/qwr9ew/s/s19/index.m3u8".to_string(),
         "https://news.cgtn.com/resource/live/english/cgtn-news.m3u8".to_string(),
         "http://39.135.138.59:18890/PLTV/88888910/224/3221225645/index.m3u8".to_string(),
         "https://livecdn.fptplay.net/sdb/cnbc_hls.smil/playlist.m3u8".to_string(),
         "https://livecdn.fptplay.net/hda3/dreamworks_vhls.smil/chunklist.m3u8".to_string(),
         "https://dwamdstream102.akamaized.net/hls/live/2015525/dwstream102/index.m3u8".to_string(),
         ], price:1 });

        packages.insert("Mamba".to_string(), 
        Packages { sub_package: "Mamba".to_string(), 
        channels:vec![
            "https://euronews.alteox.app/hls/de_stream.m3u8".to_string(),
            "https://euronews.alteox.app/hls/it_stream.m3u8".to_string(), 
            "https://euronews.alteox.app/hls/pt_stream.m3u8".to_string(), 
            "https://euronews.alteox.app/hls/ru_stream.m3u8".to_string(), 
            "https://euronews.alteox.app/hls/es_stream.m3u8".to_string(), 
            "https://raw.githubusercontent.com/LaneSh4d0w/IPTV_Exception/master/channels/af/fazatv.m3u8".to_string(), 
            "http://free.fullspeed.tv/iptv-query?streaming-ip=https://www.youtube.com/c/FRANCE24English/live".to_string(),
            "http://51.210.199.58/hls/stream.m3u8".to_string(), 
            "https://insighttv-vizio.amagi.tv/playlist.m3u8".to_string(),
            "http://cdn-kanal10.crossnet.net:1935/kanal10/kanal10asia/playlist.m3u8".to_string(),
            "https://playout395.livestreamingcdn.com/live/Stream1/playlist.m3u8".to_string(),
            "https://wms4-kortv.akamaized.net/a_live/63719963/smil:20ch011.smil/playlist.m3u8".to_string(),
            "https://raw.githubusercontent.com/taodicakhia/IPTV_Exception/master/channels/af/lemartv.m3u8".to_string(),
            "https://edge.medcom.id/live-edge/smil:mgnch.smil/playlist.m3u8".to_string(),
            "https://live.mnb.mn/live/mnb_world.stream/playlist.m3u8".to_string(),
            "https://uplynkcontent.sinclairstoryline.com/channel/ddd76fdc1c0a456ba537e4f48e827d3e.m3u8".to_string(),
            "https://nhkwlive-ojp.akamaized.net/hls/live/2003459/nhkwlive-ojp-en/index.m3u8".to_string(),
            "http://live.stream.cdn.pamirtv.com/ptv/d0dbe915091d400bd8ee7f27f0791303.sdp/index.m3u8".to_string(),
            "https://rbmn-live.akamaized.net/hls/live/590964/BoRB-AT/master.m3u8".to_string(),
            "https://rt-rtd.rttv.com/live/rtdoc/playlist.m3u8".to_string(),
            "https://rt-esp.rttv.com/live/rtesp/playlist.m3u8".to_string(),
            "https://rt-glb.rttv.com/live/rtnews/playlist.m3u8".to_string(),
            "https://streaming-live.rtp.pt/liverepeater/smil:rtpi.smil/playlist.m3u8".to_string(),

        ], price:2 });

        packages.insert("Cuber".to_string(), 
        Packages { sub_package: "Cuber".to_string(), 
        channels:vec![
                "http://live.impresa.pt/live/sicint/sicint.m3u8".to_string(),
                "https://d2xeo83q8fcni6.cloudfront.net/v1/master/9d062541f2ff39b5c0f48b743c6411d25f62fc25/SkiTV-SportsTribal/193.m3u8".to_string(), 
                "https://sofy-ger-samsung.amagi.tv/playlist.m3u8".to_string(), 
                "http://51.210.199.42/hls/stream.m3u8".to_string(), 
                "http://210.210.155.35/qwr9ew/s/s34/index.m3u8".to_string(), 
                "https://raw.githubusercontent.com/taodicakhia/IPTV_Exception/master/channels/af/tolonews.m3u8".to_string(), 
                "https://raw.githubusercontent.com/taodicakhia/IPTV_Exception/master/channels/af/tolotv.m3u8".to_string(),
                "https://tv-trtworld.live.trt.com.tr/master.m3u8".to_string(), 
                "http://210.210.155.37/dr9445/s/s24/index.m3u8".to_string(), 
                "http://210.210.155.37/uq2663/h/h158/index2.m3u8".to_string(),
                "http://online.tvm.co.mz:1935/live/smil:Channel2.smil/playlist.m3u8".to_string(),
                "https://cdnapi.kaltura.com/p/2503451/sp/250345100/playManifest/entryId/1_gb6tjmle/protocol/https/format/applehttp/a.m3u8".to_string(), 
                "https://wfcint.mediacdn.ru/cdn/wfcintweb/playlist.m3u8".to_string(),
                "https://d3w4n3hhseniak.cloudfront.net/v1/master/9d062541f2ff39b5c0f48b743c6411d25f62fc25/WPT-DistroTV/150.m3u8".to_string(),

            ], price:3 });

        packages.insert("Zone".to_string(), 
        Packages { sub_package: "Zone".to_string(), 
        channels:vec![
                "https://cdn.jmvstream.com/w/LVW-8155/ngrp:LVW8155_41E1ciuCvO_all/playlist.m3u8".to_string(),
                "http://45.162.230.234:1935/agrobrasiltv/agrobrasiltv/playlist.m3u8".to_string(), 
                "https://evpp.mm.uol.com.br/geob_band/agromais/playlist.m3u8".to_string(), 
                "https://live-hls-web-aje.getaj.net/AJE/index.m3u8".to_string(), 
                "https://5cf4a2c2512a2.streamlock.net/dgrau/dgrau/playlist.m3u8".to_string(), 
                "https://stmv1.srvif.com/animetv/animetv/playlist.m3u8".to_string(),
                "https://livecdn.fptplay.net/hda3/babytvhd_vhls.smil/chunklist.m3u8".to_string(),
                "https://evpp.mm.uol.com.br/geob_band/app/playlist.m3u8".to_string(), 
                "https://evpp.mm.uol.com.br/geob_band/bandnewstv/playlist.m3u8".to_string(),
                "https://cdnuk001.broadcastcdn.net/KUK-BBCNEWSHD/index.m3u8".to_string(),
                "https://stream01.msolutionbrasil.com.br/hls/canal25/live.m3u8".to_string(), 
                "https://arkyvbre1g.zoeweb.tv/fiocruz/fiocruz/playlist.m3u8".to_string(),
                "https://5b33b873179a2.streamlock.net:1443/catve2/catve2/playlist.m3u8".to_string(), 
                "https://5b33b873179a2.streamlock.net:1443/radiocamera/livestream/playlist.m3u8".to_string(), 
                "https://5b33b873179a2.streamlock.net:1443/mastertv/livestream/playlist.m3u8".to_string(), 
                "https://medias.sgr.globo.com/hls/vCBNRJ/vCBNRJ.m3u8".to_string(),
                "https://medias.sgr.globo.com/hls/vCBNSP/vCBNSP.m3u8".to_string(),
                "https://news.cgtn.com/resource/live/english/cgtn-news.m3u8".to_string(),
                "http://39.135.138.59:18890/PLTV/88888910/224/3221225645/index.m3u8".to_string(),
                "http://free.fullspeed.tv/iptv-query?streaming-ip=https://www.youtube.com/channel/UCvdwhh_fDyWccR42-rReZLw/live".to_string(),
                "http://209.91.213.10:8088/play/a014".to_string(),
            ], price:4 });

        packages.insert("Afree".to_string(),
        Packages { sub_package: "Afree".to_string(),
        channels:vec![
            "https://596639ebdd89b.streamlock.net/8032/8032/playlist.m3u8".to_string(),
            "http://str.portalcultura.com.br/funtelpa/tv_funtelpa/live.m3u8".to_string(),
            "http://wowza4.catve.com.br:1935/live/livestream/media.m3u8".to_string(),
            "https://darkmatter-por-samsungbrazil.amagi.tv/playlist.m3u8".to_string(),
            "https://dwamdstream102.akamaized.net/hls/live/2015525/dwstream102/index.m3u8".to_string(),
            "https://euronews.alteox.app/hls/de_stream.m3u8".to_string(),
            "https://euronews.alteox.app/hls/es_stream.m3u8".to_string(),
            "http://flash.softhost.com.br:1935/fonte/fontetv/live.m3u8".to_string(),
            "http://free.fullspeed.tv/iptv-query?streaming-ip=https://www.youtube.com/c/FRANCE24English/live".to_string(),
            "https://tv.unisc.br/hls/test.m3u8".to_string(),
            "https://stmv.video.expressolider.com.br/ghostv/ghostv/playlist.m3u8".to_string(),
            "https://stmv1.srvif.com/gospelf/gospelf/playlist.m3u8".to_string(),
            "https://58a4464faef53.streamlock.net/impd/ngrp:impd_all/playlist.m3u8".to_string(),
            "https://insighttv-vizio.amagi.tv/playlist.m3u8".to_string(),
            "https://cdn.jmvstream.com/w/LVW-9883/LVW9883_lFcfKysrHF/chunklist.m3u8".to_string(),
            "https://d6yfbj4xxtrod.cloudfront.net/out/v1/7836eb391ec24452b149f3dc6df15bbd/index.m3u8".to_string(),
            "https://wms4-kortv.akamaized.net/a_live/63719963/smil:20ch011.smil/playlist.m3u8".to_string(),
            "https://srv1.zcast.com.br/kpoptv/kpoptv/playlist.m3u8".to_string(),
            "https://edge.medcom.id/live-edge/smil:mgnch.smil/playlist.m3u8".to_string(),
            "https://video01.logicahost.com.br/mkkwebtv/mkkwebtv/playlist.m3u8".to_string(),
            "https://live.mnb.mn/live/mnb_world.stream/playlist.m3u8".to_string(),
            "https://appletree-mytime-samsungbrazil.amagi.tv/playlist.m3u8".to_string(),
            "https://uplynkcontent.sinclairstoryline.com/channel/ddd76fdc1c0a456ba537e4f48e827d3e.m3u8".to_string(),
            "https://nhkwlive-ojp.akamaized.net/hls/live/2003459/nhkwlive-ojp-en/index.m3u8".to_string(),
            "http://wz3.dnip.com.br:1935/novaeratv/novaeratv.sdp/live.m3u8".to_string(),
            "https://stream.live.novotempo.com/tv/smil:tvnovotempo.smil/playlist.m3u8".to_string(),
            "https://stmv1.duvoxtv.com.br/novelaplz/novelaplz/playlist.m3u8".to_string(),
            "https://rede-muhler-recordnews-1-br.samsung.wurl.com/manifest/playlist.m3u8".to_string(),
            "https://playpluspa-lh.akamaihd.net/i/pp_pa@377468/index_720_av-p.m3u8".to_string(),
        ], price:5});

        packages.insert("Bouza".to_string(),
        Packages { sub_package: "Bouza".to_string(),
        channels:vec![
            "https://playplusbsa-lh.akamaihd.net/i/pp_bsa@377860/index_720_av-p.m3u8".to_string(),
            "https://playplusgoya-lh.akamaihd.net/i/pp_gna@377833/index_720_av-p.m3u8".to_string(),
            "https://playplussdr-lh.akamaihd.net/i/pp_sdr@377858/index_720_av-b.m3u8".to_string(),
            "https://playplusbh-lh.akamaihd.net/i/pp_bh@377862/index_720_av-p.m3u8".to_string(),
            "https://playplusrjo-lh.akamaihd.net/i/pp_rj@377859/index_720_av-p.m3u8".to_string(),
            "https://playpluspoa-lh.akamaihd.net/i/pp_poa@377864/index_720_av-p.m3u8".to_string(),
            "https://playplusspo-lh.akamaihd.net/i/pp_sp@350176/index_720_av-p.m3u8".to_string(),
            "https://rbmn-live.akamaized.net/hls/live/590964/BoRB-AT/master.m3u8".to_string(),
            "http://rbc.directradios.com:1935/rbc/rbc/live.m3u8".to_string(),
            "https://dd8umsy8yf96u.cloudfront.net/live/cnt-curitiba.m3u8".to_string(),
            "https://dd8umsy8yf96u.cloudfront.net/live/cnt-americana.m3u8".to_string(),
            "https://5a1c76baf08c0.streamlock.net/familia/smil:familia.smil/playlist.m3u8".to_string(),
            "https://cdn.jmvstream.com/w/LVW-8719/LVW8719_AcLVAxWy5J/playlist.m3u8".to_string(),
            "http://streaming03.zas.media:1935/redemaoamiga/redemaoamiga/live.m3u8".to_string(),
            "http://tv02.logicahost.com.br:1935/rederc/rederc/live.m3u8".to_string(),
            "https://cdn.jmvstream.com/w/LVW-10024/ngrp:LVW10024_H3QLdAY6kx_all/playlist.m3u8".to_string(),
            "https://hls.brasilstream.com.br/live/redetves/redetves/playlist.m3u8".to_string(),
            "https://59f1cbe63db89.streamlock.net:1443/tvpampa/tvpampa/playlist.m3u8".to_string(),
            "https://hls.brasilstream.com.br/live/redetv/redetv/playlist.m3u8".to_string(),
            "https://cvd1.cds.ebtcvd.net/live-redevida/smil:redevida.smil/playlist.m3u8".to_string(),
            "https://59f1cbe63db89.streamlock.net:1443/redetvro/redetvro/playlist.m3u8".to_string(),
            "https://stmv1.srvif.com/retrotv/retrotv/playlist.m3u8".to_string(),
        ], price:6});

        packages.insert("Harza".to_string(),
        Packages { sub_package: "Harza".to_string(),
         channels:vec![
            "https://rt-rtd.rttv.com/live/rtdoc/playlist.m3u8".to_string(),
            "https://rt-esp.rttv.com/live/rtesp/playlist.m3u8".to_string(),
            "https://rt-glb.rttv.com/live/rtnews/playlist.m3u8".to_string(),
            "https://streaming-live.rtp.pt/liverepeater/smil:rtpi.smil/playlist.m3u8".to_string(),
            "https://cdn4.hostlagarto.com:8081/static/sanisidrotv/playlist.m3u8".to_string(),
            "http://flash1.crossdigital.com.br/2063/2063/playlist.m3u8".to_string(),
            "https://cdn-cdn-iguacu.ciclano.io:1443/cdn-iguacu/cdn-iguacu/playlist.m3u8".to_string(),
            "https://evpp.mm.uol.com.br/ne10/ne10-tvjornal-caruaru-video-web.sdp/playlist.m3u8".to_string(),
            "https://evpp.mm.uol.com.br/ne10/ne10.smil/playlist.m3u8".to_string(),
            "https://5a1c76baf08c0.streamlock.net/tvsd2/smil:tvsd2_20042020.smil/playlist.m3u8".to_string(),
            "https://cdn-telkomsel-01.akamaized.net/Content/DASH/Live/channel(9ce3f094-4044-467e-84b7-b684a49571d5)/manifest.mpd".to_string(),
            "http://live.impresa.pt/live/sicint/sicint.m3u8".to_string(),
            "https://d2xeo83q8fcni6.cloudfront.net/v1/master/9d062541f2ff39b5c0f48b743c6411d25f62fc25/SkiTV-SportsTribal/193.m3u8".to_string(),
            "https://sofy-ger-samsung.amagi.tv/playlist.m3u8".to_string(),
            "https://tastemade-pt16intl-samsungbrazil.amagi.tv/playlist.m3u8".to_string(),
            "http://painelvj.com.br/tvaguaboa2/tvaguaboa2.sdp/playlist.m3u8".to_string(),
            "http://evpp.mm.uol.com.br:1935/band_live/terraviva/playlist.m3u8".to_string(),
            "https://glxlmn026c.singularcdn.net.br/playout_02/playlist.m3u8".to_string(),
            "https://tv-trtworld.live.trt.com.tr/master.m3u8".to_string(),
            "https://cdn-canalpaulo.ciclano.io:1443/canalpaulo/canalpaulo/playlist.m3u8".to_string(),
            "https://streaming.almg.gov.br/live/tvalmg.m3u8".to_string(),
            "https://cdn.jmvstream.com/w/LVW-9716/LVW9716_HbtQtezcaw/playlist.m3u8".to_string(),
            "http://tv02.logicahost.com.br:1935/tvdigitalbirigui/tvdigitalbirigui/live.m3u8".to_string(),
            "http://tv02.logicahost.com.br:1935/bonner/bonner/live.m3u8".to_string(),
            "https://brics.bonus-tv.ru/cdn/brics/portuguese/playlist.m3u8".to_string(),
        ], price:7});
            

        Media { 
            packages, 
            messages, 
            subscriptions,
        }
    }
    
    pub fn subscribe(&mut self, package_id:String, user:String) {
        match self.packages.get(&package_id) {
            Some(value) => {
                let price = value.price*1_000_000_000_000_000_000_000_000;

                Promise::new(env::current_account_id()).transfer(price);

                self.messages.push(Msg { recipient:user.to_string(), msg: "Subscription successful".to_string()});
                let subscriptions = &mut self.subscriptions;

                let mut counter = 0;
                subscriptions.into_iter().for_each(|subscription| {
                    if subscription.user == user {
                        counter+=1;
                        subscription.package_id = package_id.to_string();
                    }
                });
                if counter <1 {
                    self.subscriptions.push(Subscription {package_id:package_id.to_string(), user:user.to_string()});
                    self.messages.push(Msg { recipient:user.to_string(), msg: "Subscription successful".to_string()});
                }
            }
            None => {
                env::log_str("Invalid package_id");
            }
        }
    }
    pub fn count_subscription(&self) -> usize {
        return self.subscriptions.len();
    }
    
    pub fn count_channels(&self) -> usize {
        return self.subscriptions.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context (current:AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(current);
        builder.account_balance(1_000_000_000_000_000_000_000_000_000);
        return builder;
    }

    #[test]
    fn subscription (){
        let ted = AccountId::new_unchecked("tedadams.testnet".to_string());
        let context = get_context(ted.clone());
        testing_env!(context.build());
        let mut contract = Media::default_package();
        contract.subscribe("Nyota".to_string(), "tedadams.testnet".to_string());
        assert_eq!(contract.count_subscription(),1);
    }

    #[test]
    fn channels() {
        let ted = AccountId::new_unchecked("tedadams.testnet".to_string());
        let context = get_context(ted.clone());
        testing_env!(context.build());
        let mut contract = Media::default_package();
        contract.subscribe("Nyota".to_string(), "tedadams.testnet".to_string());
        assert_eq!(contract.count_channels(),1);
        
    }
}
    