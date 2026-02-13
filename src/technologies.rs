use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub name: String,
    pub category: String,
    pub website: Option<String>,
    pub html: Option<Vec<Pattern>>,
    pub scripts: Option<Vec<Pattern>>,
    pub headers: Option<HashMap<String, Pattern>>,
    pub meta: Option<HashMap<String, Pattern>>,
    pub cookies: Option<HashMap<String, Pattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern: String,
    pub version: Option<String>,
}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            version: None,
        }
    }

    pub fn with_version(pattern: &str, version: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            version: Some(version.to_string()),
        }
    }
}

pub fn load_technologies() -> Result<Vec<Technology>> {
    Ok(vec![
        // ============= CONTENT MANAGEMENT SYSTEMS =============
        
        // WordPress
        Technology {
            name: "WordPress".to_string(),
            category: "CMS".to_string(),
            website: Some("https://wordpress.org".to_string()),
            html: Some(vec![
                Pattern::new(r#"/wp-content/"#),
                Pattern::new(r#"/wp-includes/"#),
                Pattern::new(r#"wp-json"#),
                Pattern::with_version(r#"WordPress ([0-9.]+)"#, r"$1"),
                Pattern::new(r#"class="wp-"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"/wp-content/.*\.js"#),
                Pattern::new(r#"/wp-includes/.*\.js"#),
                Pattern::new(r#"wp-emoji"#),
            ]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"WordPress ([0-9.]+)"#, r"$1"),
            )])),
            headers: None,
            cookies: None,
        },

        // Drupal
        Technology {
            name: "Drupal".to_string(),
            category: "CMS".to_string(),
            website: Some("https://www.drupal.org".to_string()),
            html: Some(vec![
                Pattern::new(r#"Drupal"#),
                Pattern::new(r#"/sites/default/files"#),
                Pattern::new(r#"data-drupal-"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"/sites/default/files/js"#),
                Pattern::new(r#"drupal\.js"#),
            ]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"Drupal ([0-9]+)"#, r"$1"),
            )])),
            headers: Some(HashMap::from([
                ("x-drupal-cache".to_string(), Pattern::new(r".")),
                ("x-generator".to_string(), Pattern::new(r"Drupal")),
            ])),
            cookies: None,
        },

        // Joomla
        Technology {
            name: "Joomla".to_string(),
            category: "CMS".to_string(),
            website: Some("https://www.joomla.org".to_string()),
            html: Some(vec![
                Pattern::new(r#"Joomla!"#),
                Pattern::new(r#"/media/system/js/"#),
                Pattern::new(r#"com_content"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"/media/system/js/mootools"#),
                Pattern::new(r#"joomla"#),
            ]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"Joomla! ([0-9.]+)"#, r"$1"),
            )])),
            headers: None,
            cookies: None,
        },

        // Wix
        Technology {
            name: "Wix".to_string(),
            category: "CMS".to_string(),
            website: Some("https://www.wix.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"wix\.com"#),
                Pattern::new(r#"X-Wix-"#),
                Pattern::new(r#"wixstatic\.com"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"static\.parastorage\.com"#)]),
            headers: Some(HashMap::from([(
                "x-wix-request-id".to_string(),
                Pattern::new(r"."),
            )])),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::new(r"Wix.com"),
            )])),
            cookies: None,
        },

        // Squarespace
        Technology {
            name: "Squarespace".to_string(),
            category: "CMS".to_string(),
            website: Some("https://www.squarespace.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"squarespace"#),
                Pattern::new(r#"sqsp"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"static\.squarespace\.com"#)]),
            headers: Some(HashMap::from([(
                "x-servedby".to_string(),
                Pattern::new(r"squarespace"),
            )])),
            meta: None,
            cookies: None,
        },

        // Webflow
        Technology {
            name: "Webflow".to_string(),
            category: "CMS".to_string(),
            website: Some("https://webflow.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"webflow"#),
                Pattern::new(r#"wf-page"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"webflow\.com"#)]),
            meta: None,
            headers: None,
            cookies: None,
        },

        // ============= E-COMMERCE =============

        // Shopify
        Technology {
            name: "Shopify".to_string(),
            category: "Ecommerce".to_string(),
            website: Some("https://www.shopify.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"cdn\.shopify\.com"#),
                Pattern::new(r#"Shopify\.theme"#),
                Pattern::new(r#"shopify-analytics"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"cdn\.shopify\.com"#),
                Pattern::new(r#"shopify_pay"#),
            ]),
            headers: Some(HashMap::from([
                ("x-shopify-stage".to_string(), Pattern::new(r".")),
                ("x-shopid".to_string(), Pattern::new(r".")),
            ])),
            meta: None,
            cookies: Some(HashMap::from([(
                "_shopify_visit".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // WooCommerce
        Technology {
            name: "WooCommerce".to_string(),
            category: "Ecommerce".to_string(),
            website: Some("https://woocommerce.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"woocommerce"#),
                Pattern::new(r#"wc-"#),
                Pattern::new(r#"wp-content/plugins/woocommerce"#),
            ]),
            scripts: Some(vec![Pattern::new(
                r#"wp-content/plugins/woocommerce/.*\.js"#,
            )]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"WooCommerce ([0-9.]+)"#, r"$1"),
            )])),
            headers: None,
            cookies: None,
        },

        // Magento
        Technology {
            name: "Magento".to_string(),
            category: "Ecommerce".to_string(),
            website: Some("https://magento.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"Mage\.Cookies"#),
                Pattern::new(r#"Magento"#),
                Pattern::new(r#"skin/frontend"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"mage/"#),
                Pattern::new(r#"varien/"#),
            ]),
            headers: Some(HashMap::from([(
                "x-magento-tags".to_string(),
                Pattern::new(r"."),
            )])),
            meta: None,
            cookies: Some(HashMap::from([(
                "frontend".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // BigCommerce
        Technology {
            name: "BigCommerce".to_string(),
            category: "Ecommerce".to_string(),
            website: Some("https://www.bigcommerce.com".to_string()),
            html: Some(vec![Pattern::new(r#"bigcommerce"#)]),
            scripts: Some(vec![Pattern::new(r#"cdn.*bigcommerce"#)]),
            headers: Some(HashMap::from([(
                "x-bc-storefront".to_string(),
                Pattern::new(r"."),
            )])),
            meta: None,
            cookies: None,
        },

        // PrestaShop
        Technology {
            name: "PrestaShop".to_string(),
            category: "Ecommerce".to_string(),
            website: Some("https://www.prestashop.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"prestashop"#),
                Pattern::new(r#"powered by PrestaShop"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"prestashop"#)]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"PrestaShop ([0-9.]+)"#, r"$1"),
            )])),
            headers: None,
            cookies: None,
        },

        // ============= JAVASCRIPT FRAMEWORKS =============

        // React
        Technology {
            name: "React".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://react.dev".to_string()),
            html: Some(vec![
                Pattern::new(r#"data-reactroot"#),
                Pattern::new(r#"data-react-helmet"#),
                Pattern::new(r#"data-reactid"#),
                Pattern::new(r#"<div id="root"#),
            ]),
            scripts: Some(vec![
                Pattern::with_version(r#"react@([0-9.]+)"#, r"$1"),
                Pattern::new(r#"/react\..*\.js"#),
                Pattern::new(r#"cdn.*react"#),
                Pattern::new(r#"react-dom"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Vue.js
        Technology {
            name: "Vue.js".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://vuejs.org".to_string()),
            html: Some(vec![
                Pattern::new(r#"data-v-"#),
                Pattern::new(r#"v-cloak"#),
                Pattern::new(r#"v-bind:"#),
                Pattern::new(r#"v-for="#),
                Pattern::new(r#"v-if="#),
            ]),
            scripts: Some(vec![
                Pattern::with_version(r#"vue@([0-9.]+)"#, r"$1"),
                Pattern::new(r#"vue\.js"#),
                Pattern::new(r#"vue\.min\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Angular
        Technology {
            name: "Angular".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://angular.io".to_string()),
            html: Some(vec![
                Pattern::new(r#"ng-"#),
                Pattern::new(r#"\[ng"#),
                Pattern::new(r#"_ngcontent"#),
                Pattern::new(r#"_nghost"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"angular\.js"#),
                Pattern::new(r#"@angular/core"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Svelte
        Technology {
            name: "Svelte".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://svelte.dev".to_string()),
            html: Some(vec![Pattern::new(r#"svelte-"#)]),
            scripts: Some(vec![
                Pattern::new(r#"svelte"#),
                Pattern::new(r#"_app/immutable"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Next.js
        Technology {
            name: "Next.js".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://nextjs.org".to_string()),
            html: Some(vec![
                Pattern::new(r#"__NEXT_DATA__"#),
                Pattern::new(r#"__next"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"/_next/"#),
                Pattern::new(r#"next\.js"#),
            ]),
            headers: Some(HashMap::from([(
                "x-powered-by".to_string(),
                Pattern::with_version(r#"Next\.js ([0-9.]+)"#, r"$1"),
            )])),
            meta: None,
            cookies: None,
        },

        // Nuxt.js
        Technology {
            name: "Nuxt.js".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://nuxt.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"__NUXT__"#),
                Pattern::new(r#"nuxt"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"/_nuxt/"#),
                Pattern::new(r#"nuxt\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Gatsby
        Technology {
            name: "Gatsby".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://www.gatsbyjs.com".to_string()),
            html: Some(vec![Pattern::new(r#"gatsby"#)]),
            scripts: Some(vec![Pattern::new(r#"gatsby-"#)]),
            meta: Some(HashMap::from([(
                "generator".to_string(),
                Pattern::with_version(r#"Gatsby ([0-9.]+)"#, r"$1"),
            )])),
            headers: None,
            cookies: None,
        },

        // Ember.js
        Technology {
            name: "Ember.js".to_string(),
            category: "JavaScript Framework".to_string(),
            website: Some("https://emberjs.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"ember"#),
                Pattern::new(r#"data-ember-"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"ember\.js"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // jQuery
        Technology {
            name: "jQuery".to_string(),
            category: "JavaScript Library".to_string(),
            website: Some("https://jquery.com".to_string()),
            html: None,
            scripts: Some(vec![
                Pattern::with_version(r#"jquery[.-]([0-9.]+)"#, r"$1"),
                Pattern::new(r#"/jquery\.min\.js"#),
                Pattern::new(r#"jquery\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // jQuery UI
        Technology {
            name: "jQuery UI".to_string(),
            category: "JavaScript Library".to_string(),
            website: Some("https://jqueryui.com".to_string()),
            html: Some(vec![Pattern::new(r#"ui-widget"#)]),
            scripts: Some(vec![
                Pattern::with_version(r#"jquery-ui[.-]([0-9.]+)"#, r"$1"),
                Pattern::new(r#"jquery-ui\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= CSS FRAMEWORKS =============

        // Bootstrap
        Technology {
            name: "Bootstrap".to_string(),
            category: "CSS Framework".to_string(),
            website: Some("https://getbootstrap.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"class="[^"]*\bcontainer\b"#),
                Pattern::new(r#"class="[^"]*\brow\b"#),
                Pattern::new(r#"class="[^"]*\bcol-"#),
                Pattern::new(r#"class="[^"]*\bbtn\b"#),
            ]),
            scripts: Some(vec![Pattern::with_version(
                r#"bootstrap[.-]([0-9.]+)"#,
                r"$1",
            )]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Tailwind CSS
        Technology {
            name: "Tailwind CSS".to_string(),
            category: "CSS Framework".to_string(),
            website: Some("https://tailwindcss.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"class="[^"]*\b(flex|grid|hidden|block|inline)\b"#),
                Pattern::new(r#"class="[^"]*\b(bg-|text-|border-|rounded-|shadow-)"#),
                Pattern::new(r#"class="[^"]*\b(px-|py-|pt-|pb-|pl-|pr-|m-|mx-|my-)"#),
            ]),
            scripts: None,
            headers: None,
            meta: None,
            cookies: None,
        },

        // Foundation
        Technology {
            name: "Foundation".to_string(),
            category: "CSS Framework".to_string(),
            website: Some("https://get.foundation".to_string()),
            html: Some(vec![
                Pattern::new(r#"foundation"#),
                Pattern::new(r#"class="[^"]*\bfoundation\b"#),
            ]),
            scripts: Some(vec![Pattern::with_version(
                r#"foundation[.-]([0-9.]+)"#,
                r"$1",
            )]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Bulma
        Technology {
            name: "Bulma".to_string(),
            category: "CSS Framework".to_string(),
            website: Some("https://bulma.io".to_string()),
            html: Some(vec![
                Pattern::new(r#"class="[^"]*\bbulma\b"#),
                Pattern::new(r#"class="[^"]*\bcolumns\b"#),
            ]),
            scripts: None,
            headers: None,
            meta: None,
            cookies: None,
        },

        // Material UI
        Technology {
            name: "Material-UI".to_string(),
            category: "CSS Framework".to_string(),
            website: Some("https://mui.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"class="[^"]*\bMui"#),
                Pattern::new(r#"makeStyles"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"@material-ui"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= ANALYTICS =============

        // Google Analytics
        Technology {
            name: "Google Analytics".to_string(),
            category: "Analytics".to_string(),
            website: Some("https://analytics.google.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"google-analytics\.com/analytics\.js"#),
                Pattern::new(r#"googletagmanager\.com/gtag/js"#),
                Pattern::with_version(r#"GoogleAnalyticsObject.*ga\('create',.*'(UA-[^']+)"#, r"$1"),
                Pattern::new(r#"gtag\("#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"google-analytics\.com"#),
                Pattern::new(r#"gtag/js"#),
                Pattern::new(r#"analytics\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: Some(HashMap::from([
                ("_ga".to_string(), Pattern::new(r".")),
                ("_gid".to_string(), Pattern::new(r".")),
                ("_gat".to_string(), Pattern::new(r".")),
            ])),
        },

        // Google Tag Manager
        Technology {
            name: "Google Tag Manager".to_string(),
            category: "Tag Manager".to_string(),
            website: Some("https://tagmanager.google.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"googletagmanager\.com/gtm\.js"#),
                Pattern::new(r#"GTM-"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"googletagmanager\.com/gtm\.js"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Adobe Analytics
        Technology {
            name: "Adobe Analytics".to_string(),
            category: "Analytics".to_string(),
            website: Some("https://www.adobe.com/analytics".to_string()),
            html: Some(vec![
                Pattern::new(r#"omniture"#),
                Pattern::new(r#"s_code\.js"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"omtrdc\.net"#),
                Pattern::new(r#"s_code\.js"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Mixpanel
        Technology {
            name: "Mixpanel".to_string(),
            category: "Analytics".to_string(),
            website: Some("https://mixpanel.com".to_string()),
            html: Some(vec![Pattern::new(r#"mixpanel"#)]),
            scripts: Some(vec![Pattern::new(r#"mixpanel\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Segment
        Technology {
            name: "Segment".to_string(),
            category: "Analytics".to_string(),
            website: Some("https://segment.com".to_string()),
            html: Some(vec![Pattern::new(r#"analytics\.js"#)]),
            scripts: Some(vec![Pattern::new(r#"cdn\.segment\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Hotjar
        Technology {
            name: "Hotjar".to_string(),
            category: "Analytics".to_string(),
            website: Some("https://www.hotjar.com".to_string()),
            html: Some(vec![Pattern::new(r#"hotjar"#)]),
            scripts: Some(vec![Pattern::new(r#"static\.hotjar\.com"#)]),
            headers: None,
            meta: None,
            cookies: Some(HashMap::from([(
                "_hjid".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // ============= ADVERTISING =============

        // Google Ads
        Technology {
            name: "Google Ads".to_string(),
            category: "Advertising".to_string(),
            website: Some("https://ads.google.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"googleadservices\.com"#),
                Pattern::new(r#"googlesyndication\.com"#),
            ]),
            scripts: Some(vec![
                Pattern::new(r#"googleadservices\.com"#),
                Pattern::new(r#"pagead2\.googlesyndication\.com"#),
            ]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Facebook Pixel
        Technology {
            name: "Facebook Pixel".to_string(),
            category: "Advertising".to_string(),
            website: Some("https://www.facebook.com/business/tools/meta-pixel".to_string()),
            html: Some(vec![
                Pattern::new(r#"fbq\("#),
                Pattern::new(r#"facebook\.com/tr"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"connect\.facebook\.net"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= CDN =============

        // Cloudflare
        Technology {
            name: "Cloudflare".to_string(),
            category: "CDN".to_string(),
            website: Some("https://www.cloudflare.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("server".to_string(), Pattern::new(r"cloudflare")),
                ("cf-ray".to_string(), Pattern::new(r".")),
                ("cf-cache-status".to_string(), Pattern::new(r".")),
            ])),
            meta: None,
            cookies: Some(HashMap::from([(
                "__cfduid".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Fastly
        Technology {
            name: "Fastly".to_string(),
            category: "CDN".to_string(),
            website: Some("https://www.fastly.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("x-served-by".to_string(), Pattern::new(r"cache-")),
                ("fastly-".to_string(), Pattern::new(r".")),
            ])),
            meta: None,
            cookies: None,
        },

        // Amazon CloudFront
        Technology {
            name: "Amazon CloudFront".to_string(),
            category: "CDN".to_string(),
            website: Some("https://aws.amazon.com/cloudfront/".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("x-amz-cf-id".to_string(), Pattern::new(r".")),
                ("via".to_string(), Pattern::new(r"cloudfront")),
            ])),
            meta: None,
            cookies: None,
        },

        // Akamai
        Technology {
            name: "Akamai".to_string(),
            category: "CDN".to_string(),
            website: Some("https://www.akamai.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("x-akamai-".to_string(), Pattern::new(r".")),
                ("akamai-".to_string(), Pattern::new(r".")),
            ])),
            meta: None,
            cookies: None,
        },

        // ============= WEB SERVERS =============

        // Nginx
        Technology {
            name: "Nginx".to_string(),
            category: "Web Server".to_string(),
            website: Some("https://nginx.org".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "server".to_string(),
                Pattern::with_version(r#"nginx/([0-9.]+)"#, r"$1"),
            )])),
            meta: None,
            cookies: None,
        },

        // Apache
        Technology {
            name: "Apache".to_string(),
            category: "Web Server".to_string(),
            website: Some("https://apache.org".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "server".to_string(),
                Pattern::with_version(r#"Apache/([0-9.]+)"#, r"$1"),
            )])),
            meta: None,
            cookies: None,
        },

        // Microsoft IIS
        Technology {
            name: "Microsoft IIS".to_string(),
            category: "Web Server".to_string(),
            website: Some("https://www.iis.net".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                (
                    "server".to_string(),
                    Pattern::with_version(r#"Microsoft-IIS/([0-9.]+)"#, r"$1"),
                ),
                ("x-aspnet-version".to_string(), Pattern::new(r".")),
            ])),
            meta: None,
            cookies: None,
        },

        // LiteSpeed
        Technology {
            name: "LiteSpeed".to_string(),
            category: "Web Server".to_string(),
            website: Some("https://www.litespeedtech.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "server".to_string(),
                Pattern::with_version(r#"LiteSpeed/([0-9.]+)"#, r"$1"),
            )])),
            meta: None,
            cookies: None,
        },

        // ============= PROGRAMMING LANGUAGES =============

        // PHP
        Technology {
            name: "PHP".to_string(),
            category: "Programming Language".to_string(),
            website: Some("https://php.net".to_string()),
            html: Some(vec![Pattern::new(r#"\.php"#)]),
            scripts: None,
            headers: Some(HashMap::from([
                (
                    "x-powered-by".to_string(),
                    Pattern::with_version(r#"PHP/([0-9.]+)"#, r"$1"),
                ),
                ("server".to_string(), Pattern::new(r"PHP")),
            ])),
            meta: None,
            cookies: Some(HashMap::from([(
                "PHPSESSID".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Node.js
        Technology {
            name: "Node.js".to_string(),
            category: "Programming Language".to_string(),
            website: Some("https://nodejs.org".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "x-powered-by".to_string(),
                Pattern::new(r"Express"),
            )])),
            meta: None,
            cookies: None,
        },

        // Python
        Technology {
            name: "Python".to_string(),
            category: "Programming Language".to_string(),
            website: Some("https://python.org".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "server".to_string(),
                Pattern::new(r"Python"),
            )])),
            meta: None,
            cookies: None,
        },

        // ASP.NET
        Technology {
            name: "ASP.NET".to_string(),
            category: "Programming Language".to_string(),
            website: Some("https://dotnet.microsoft.com/apps/aspnet".to_string()),
            html: Some(vec![
                Pattern::new(r#"__VIEWSTATE"#),
                Pattern::new(r#"aspx"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"WebResource\.axd"#)]),
            headers: Some(HashMap::from([
                ("x-aspnet-version".to_string(), Pattern::new(r".")),
                (
                    "x-aspnetmvc-version".to_string(),
                    Pattern::new(r"."),
                ),
            ])),
            meta: None,
            cookies: Some(HashMap::from([(
                "ASP.NET_SessionId".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Ruby on Rails
        Technology {
            name: "Ruby on Rails".to_string(),
            category: "Web Framework".to_string(),
            website: Some("https://rubyonrails.org".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "x-powered-by".to_string(),
                Pattern::new(r"Phusion Passenger"),
            )])),
            meta: None,
            cookies: Some(HashMap::from([(
                "_session_id".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Django
        Technology {
            name: "Django".to_string(),
            category: "Web Framework".to_string(),
            website: Some("https://www.djangoproject.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "x-frame-options".to_string(),
                Pattern::new(r"DENY"),
            )])),
            meta: None,
            cookies: Some(HashMap::from([(
                "csrftoken".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Express
        Technology {
            name: "Express".to_string(),
            category: "Web Framework".to_string(),
            website: Some("https://expressjs.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "x-powered-by".to_string(),
                Pattern::new(r"Express"),
            )])),
            meta: None,
            cookies: None,
        },

        // ============= PAYMENT PROCESSORS =============

        // Stripe
        Technology {
            name: "Stripe".to_string(),
            category: "Payment Processor".to_string(),
            website: Some("https://stripe.com".to_string()),
            html: Some(vec![Pattern::new(r#"stripe"#)]),
            scripts: Some(vec![Pattern::new(r#"js\.stripe\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // PayPal
        Technology {
            name: "PayPal".to_string(),
            category: "Payment Processor".to_string(),
            website: Some("https://www.paypal.com".to_string()),
            html: Some(vec![Pattern::new(r#"paypal"#)]),
            scripts: Some(vec![Pattern::new(r#"paypal\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Square
        Technology {
            name: "Square".to_string(),
            category: "Payment Processor".to_string(),
            website: Some("https://squareup.com".to_string()),
            html: Some(vec![Pattern::new(r#"square"#)]),
            scripts: Some(vec![Pattern::new(r#"squareup\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= CHAT & SUPPORT =============

        // Intercom
        Technology {
            name: "Intercom".to_string(),
            category: "Live Chat".to_string(),
            website: Some("https://www.intercom.com".to_string()),
            html: Some(vec![Pattern::new(r#"intercom"#)]),
            scripts: Some(vec![Pattern::new(r#"widget\.intercom\.io"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Zendesk
        Technology {
            name: "Zendesk".to_string(),
            category: "Customer Support".to_string(),
            website: Some("https://www.zendesk.com".to_string()),
            html: Some(vec![Pattern::new(r#"zendesk"#)]),
            scripts: Some(vec![Pattern::new(r#"zendesk\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Drift
        Technology {
            name: "Drift".to_string(),
            category: "Live Chat".to_string(),
            website: Some("https://www.drift.com".to_string()),
            html: Some(vec![Pattern::new(r#"drift"#)]),
            scripts: Some(vec![Pattern::new(r#"js\.driftt\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // LiveChat
        Technology {
            name: "LiveChat".to_string(),
            category: "Live Chat".to_string(),
            website: Some("https://www.livechat.com".to_string()),
            html: Some(vec![Pattern::new(r#"livechat"#)]),
            scripts: Some(vec![Pattern::new(r#"cdn\.livechatinc\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Tawk.to
        Technology {
            name: "Tawk.to".to_string(),
            category: "Live Chat".to_string(),
            website: Some("https://www.tawk.to".to_string()),
            html: Some(vec![Pattern::new(r#"tawk"#)]),
            scripts: Some(vec![Pattern::new(r#"embed\.tawk\.to"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= VIDEO PLAYERS =============

        // YouTube
        Technology {
            name: "YouTube".to_string(),
            category: "Video Player".to_string(),
            website: Some("https://www.youtube.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"youtube\.com/embed"#),
                Pattern::new(r#"youtube-nocookie\.com"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"youtube\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Vimeo
        Technology {
            name: "Vimeo".to_string(),
            category: "Video Player".to_string(),
            website: Some("https://vimeo.com".to_string()),
            html: Some(vec![Pattern::new(r#"player\.vimeo\.com"#)]),
            scripts: Some(vec![Pattern::new(r#"player\.vimeo\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Video.js
        Technology {
            name: "Video.js".to_string(),
            category: "Video Player".to_string(),
            website: Some("https://videojs.com".to_string()),
            html: Some(vec![Pattern::new(r#"video-js"#)]),
            scripts: Some(vec![Pattern::new(r#"video\.js"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= FONTS =============

        // Google Fonts
        Technology {
            name: "Google Fonts".to_string(),
            category: "Font".to_string(),
            website: Some("https://fonts.google.com".to_string()),
            html: Some(vec![Pattern::new(r#"fonts\.googleapis\.com"#)]),
            scripts: None,
            headers: None,
            meta: None,
            cookies: None,
        },

        // Font Awesome
        Technology {
            name: "Font Awesome".to_string(),
            category: "Font".to_string(),
            website: Some("https://fontawesome.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"font-awesome"#),
                Pattern::new(r#"fa-"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"fontawesome"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Adobe Fonts
        Technology {
            name: "Adobe Fonts".to_string(),
            category: "Font".to_string(),
            website: Some("https://fonts.adobe.com".to_string()),
            html: Some(vec![Pattern::new(r#"use\.typekit\.net"#)]),
            scripts: Some(vec![Pattern::new(r#"typekit\.net"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= SECURITY =============

        // reCAPTCHA
        Technology {
            name: "reCAPTCHA".to_string(),
            category: "Security".to_string(),
            website: Some("https://www.google.com/recaptcha".to_string()),
            html: Some(vec![
                Pattern::new(r#"g-recaptcha"#),
                Pattern::new(r#"grecaptcha"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"google\.com/recaptcha"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // hCaptcha
        Technology {
            name: "hCaptcha".to_string(),
            category: "Security".to_string(),
            website: Some("https://www.hcaptcha.com".to_string()),
            html: Some(vec![Pattern::new(r#"h-captcha"#)]),
            scripts: Some(vec![Pattern::new(r#"hcaptcha\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= MARKETING AUTOMATION =============

        // HubSpot
        Technology {
            name: "HubSpot".to_string(),
            category: "Marketing Automation".to_string(),
            website: Some("https://www.hubspot.com".to_string()),
            html: Some(vec![Pattern::new(r#"hubspot"#)]),
            scripts: Some(vec![Pattern::new(r#"js\.hs-scripts\.com"#)]),
            headers: None,
            meta: None,
            cookies: Some(HashMap::from([(
                "hubspotutk".to_string(),
                Pattern::new(r"."),
            )])),
        },

        // Mailchimp
        Technology {
            name: "Mailchimp".to_string(),
            category: "Email Marketing".to_string(),
            website: Some("https://mailchimp.com".to_string()),
            html: Some(vec![Pattern::new(r#"mailchimp"#)]),
            scripts: Some(vec![Pattern::new(r#"chimpstatic\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ============= HOSTING/PLATFORMS =============

        // Vercel
        Technology {
            name: "Vercel".to_string(),
            category: "Hosting".to_string(),
            website: Some("https://vercel.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("x-vercel-id".to_string(), Pattern::new(r".")),
                ("server".to_string(), Pattern::new(r"Vercel")),
            ])),
            meta: None,
            cookies: None,
        },

        // Netlify
        Technology {
            name: "Netlify".to_string(),
            category: "Hosting".to_string(),
            website: Some("https://www.netlify.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([
                ("x-nf-request-id".to_string(), Pattern::new(r".")),
                ("server".to_string(), Pattern::new(r"Netlify")),
            ])),
            meta: None,
            cookies: None,
        },

        // GitHub Pages
        Technology {
            name: "GitHub Pages".to_string(),
            category: "Hosting".to_string(),
            website: Some("https://pages.github.com".to_string()),
            html: None,
            scripts: None,
            headers: Some(HashMap::from([(
                "server".to_string(),
                Pattern::new(r"GitHub\.com"),
            )])),
            meta: None,
            cookies: None,
        },

        // ============= SOCIAL =============

        // Facebook SDK
        Technology {
            name: "Facebook SDK".to_string(),
            category: "Social".to_string(),
            website: Some("https://developers.facebook.com".to_string()),
            html: Some(vec![
                Pattern::new(r#"fb-"#),
                Pattern::new(r#"facebook\.net"#),
            ]),
            scripts: Some(vec![Pattern::new(r#"connect\.facebook\.net"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // Twitter Widgets
        Technology {
            name: "Twitter Widgets".to_string(),
            category: "Social".to_string(),
            website: Some("https://developer.twitter.com".to_string()),
            html: Some(vec![Pattern::new(r#"twitter-timeline"#)]),
            scripts: Some(vec![Pattern::new(r#"platform\.twitter\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // AddThis
        Technology {
            name: "AddThis".to_string(),
            category: "Social Sharing".to_string(),
            website: Some("https://www.addthis.com".to_string()),
            html: Some(vec![Pattern::new(r#"addthis"#)]),
            scripts: Some(vec![Pattern::new(r#"addthis\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },

        // ShareThis
        Technology {
            name: "ShareThis".to_string(),
            category: "Social Sharing".to_string(),
            website: Some("https://sharethis.com".to_string()),
            html: Some(vec![Pattern::new(r#"sharethis"#)]),
            scripts: Some(vec![Pattern::new(r#"sharethis\.com"#)]),
            headers: None,
            meta: None,
            cookies: None,
        },
    ])
}

