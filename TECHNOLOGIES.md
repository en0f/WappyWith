# Complete Technology Detection List

This document lists all 100+ technologies that Wappalyzer-Rust can detect, organized by category.

## Content Management Systems (CMS)

1. **WordPress** - The world's most popular CMS
2. **Drupal** - Open-source CMS platform
3. **Joomla** - Free and open-source CMS
4. **Wix** - Cloud-based website builder
5. **Squarespace** - Website building and hosting
6. **Webflow** - Visual web design platform

## E-commerce Platforms

1. **Shopify** - Complete commerce platform
2. **WooCommerce** - WordPress e-commerce plugin
3. **Magento** - Open-source e-commerce platform
4. **BigCommerce** - SaaS e-commerce platform
5. **PrestaShop** - Open-source e-commerce solution

## JavaScript Frameworks & Libraries

### Frameworks
1. **React** - Facebook's UI library
2. **Vue.js** - Progressive JavaScript framework
3. **Angular** - Google's web application framework
4. **Svelte** - Compiled JavaScript framework
5. **Next.js** - React framework with SSR
6. **Nuxt.js** - Vue.js framework with SSR
7. **Gatsby** - React-based static site generator
8. **Ember.js** - Framework for ambitious web apps

### Libraries
1. **jQuery** - Fast, small JavaScript library
2. **jQuery UI** - User interface library

## CSS Frameworks

1. **Bootstrap** - Popular CSS framework
2. **Tailwind CSS** - Utility-first CSS framework
3. **Foundation** - Responsive front-end framework
4. **Bulma** - Modern CSS framework
5. **Material-UI** - React UI framework

## Analytics & Tracking

1. **Google Analytics** - Web analytics service (detects UA and GA4)
2. **Google Tag Manager** - Tag management system
3. **Adobe Analytics** - Enterprise analytics platform
4. **Mixpanel** - Product analytics platform
5. **Segment** - Customer data platform
6. **Hotjar** - Behavior analytics and user feedback

## Advertising & Marketing

1. **Google Ads** - Google's advertising platform
2. **Facebook Pixel** - Facebook tracking and analytics
3. **HubSpot** - Marketing automation platform
4. **Mailchimp** - Email marketing service

## Content Delivery Networks (CDN)

1. **Cloudflare** - Web infrastructure and security
2. **Fastly** - Edge cloud platform
3. **Amazon CloudFront** - AWS CDN service
4. **Akamai** - Global CDN provider

## Web Servers

1. **Nginx** - High-performance web server
2. **Apache** - Open-source HTTP server
3. **Microsoft IIS** - Windows web server
4. **LiteSpeed** - High-performance web server

## Programming Languages & Runtimes

1. **PHP** - Server-side scripting language
2. **Node.js** - JavaScript runtime
3. **Python** - General-purpose programming language
4. **ASP.NET** - Microsoft web framework

## Web Frameworks

1. **Express** - Node.js web framework
2. **Ruby on Rails** - Ruby web framework
3. **Django** - Python web framework

## Payment Processors

1. **Stripe** - Online payment processing
2. **PayPal** - Digital payment platform
3. **Square** - Payment and business solutions

## Live Chat & Customer Support

1. **Intercom** - Customer messaging platform
2. **Zendesk** - Customer service software
3. **Drift** - Conversational marketing platform
4. **LiveChat** - Customer service software
5. **Tawk.to** - Free live chat software

## Video Players

1. **YouTube** - Video sharing platform
2. **Vimeo** - Video hosting platform
3. **Video.js** - Open-source video player

## Fonts & Icons

1. **Google Fonts** - Free font library
2. **Font Awesome** - Icon library
3. **Adobe Fonts** - Font subscription service

## Security

1. **reCAPTCHA** - Google's anti-bot service
2. **hCaptcha** - Privacy-focused CAPTCHA service

## Hosting & Infrastructure

1. **Vercel** - Frontend cloud platform
2. **Netlify** - Web hosting and automation
3. **GitHub Pages** - Static site hosting

## Social Media Integration

1. **Facebook SDK** - Facebook platform integration
2. **Twitter Widgets** - Twitter content embedding
3. **AddThis** - Social sharing tools
4. **ShareThis** - Social sharing platform

## Tag Managers

1. **Google Tag Manager** - Tag management system

## Detection Methods

For each technology, the analyzer uses multiple detection methods:

### HTML Content Analysis
- Searches for unique strings, class names, and patterns
- Identifies framework-specific attributes (e.g., `data-reactroot`, `ng-app`)
- Detects CMS-specific paths (e.g., `/wp-content/`, `/sites/default/`)

### HTTP Headers
- Server headers (Nginx, Apache, IIS)
- Custom headers (X-Powered-By, X-Framework)
- CDN headers (CF-Ray, X-Amz-Cf-Id)

### JavaScript Files
- Script sources and CDN URLs
- Version numbers in filenames
- Framework-specific file patterns

### Meta Tags
- Generator meta tags
- CMS and framework identifiers
- Version information

### Cookies
- Session cookies (PHPSESSID, ASP.NET_SessionId)
- Analytics cookies (_ga, _gid)
- Platform-specific cookies

## Version Detection

Where possible, the analyzer extracts version numbers for:
- CMS platforms (WordPress, Drupal, Joomla)
- Web servers (Nginx, Apache)
- Frameworks (React, Vue.js, Next.js)
- Languages (PHP)
- Various libraries and tools

Version numbers are extracted using regex patterns that match common version formats.

## Confidence Scoring

Each detection is assigned a confidence score based on:
- Number of patterns matched
- Type of detection (headers score higher than HTML patterns)
- Specificity of the match

Scores range from 0-100%, with higher scores indicating more certain detections.

## Similar to BuiltWith

This implementation provides similar functionality to BuiltWith.com, including:
- ✅ Comprehensive technology detection
- ✅ Multiple categories and subcategories
- ✅ Version identification
- ✅ Confidence scoring
- ✅ Multiple detection methods
- ✅ 100+ technologies across 20+ categories

## Extending the Database

The technology database is easily extensible. To add new technologies, edit `src/technologies.rs` and add a new `Technology` struct with appropriate patterns.

See the README.md for examples of how to add custom technologies.
