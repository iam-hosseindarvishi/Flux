# Flux UI: PRD فنی

# Flux UI: PRD فنی
نسخه 1.0 وضعیت: Draft مالک: حسین درویشی

مرجع طراحی: Private ([https://app.clickup.com/90151376297/docs/2kyq1nd9-555](https://app.clickup.com/90151376297/docs/2kyq1nd9-555)) · پرامپت‌ها: Private ([https://app.clickup.com/90151376297/docs/2kyq1nd9-535](https://app.clickup.com/90151376297/docs/2kyq1nd9-535)) · تسک: Private ([https://app.clickup.com/t/124drtge9eg](https://app.clickup.com/t/124drtge9eg))

* * *
## ۱. خلاصه
این PRD لایه UI برنامه Flux و **موتور تم (Theme Engine)** رو تعریف می‌کنه: پیاده‌سازی ۵ تم (Dark، Light، Glass، Neumorphism، AMOLED Cyber) روی یه سیستم توکن مشترک، کتابخونه کامپوننت، و اتصال زنده به هسته دانلود.

**داخل محدوده:** توکن‌ها، موتور تم، کامپوننت‌ها، صفحه‌های اصلی، قرارداد داده UI با هسته.

**خارج از محدوده:** منطق دانلود (Rust core)، افزونه مرورگر (فقط popup اون داخل این PRDه)، الگوریتم تشخیص سریال.

* * *
## ۲. اهداف و معیارهای موفقیت

| هدف | معیار | مقدار هدف |
| ---| ---| --- |
| سوییچ تم بدون مکث | زمان اعمال تم بدون reload | < 100ms |
| روان بودن زیر بار | FPS با ۵۰ دانلود فعال و ۱۶ سگمنت | ≥ 55fps (Glass ≥ 45fps) |
| سبک بودن | حجم bundle فرانت (gzip) | < 350KB |
| مصرف حافظه | RAM پنجره در حالت idle | < 120MB |
| دسترسی‌پذیری | کنتراست متن اصلی و ثانویه در هر ۵ تم | WCAG AA (≥ 4.5:1) |
| یکپارچگی | رنگ hex مستقیم در کد کامپوننت‌ها | ۰ (با lint چک میشه) |

* * *
## ۳. استک فنی

| لایه | انتخاب | دلیل |
| ---| ---| --- |
| Shell | Tauri 2 | باینری سبک، اتصال مستقیم به هسته Rust |
| Framework | React 19 + TypeScript + Vite | اکوسیستم بزرگ، جذب مشارکت‌کننده اوپن‌سورس |
| استایل | CSS Variables + Tailwind v4 (مپ شده به توکن‌ها) | سوییچ تم فقط با تغییر یه attribute |
| توکن‌ها | Style Dictionary (فرمت W3C DTCG) | یه منبع واحد ← CSS + تایپ TS |
| State | Zustand | ساده، سریع، selector-based re-render |
| لیست طولانی | TanStack Virtual | هزاران ردیف بدون افت FPS |
| نمودار | uPlot | ~40KB، Canvas، مناسب داده زنده |
| آیکون | Lucide | tree-shakable، هماهنگ با طرح |
| i18n | i18next + پشتیبانی RTL | فارسی و انگلیسی از روز اول |
| افکت نیتیو پنجره | window-vibrancy (Tauri plugin) | Mica/Acrylic در ویندوز، Vibrancy در مک برای تم Glass |

### ساختار پوشه‌ها

```plain
ui/
  tokens/
    base.json            # فونت، فاصله، radius، motion
    themes/
      dark.json  light.json  glass.json  neumorphism.json  amoled.json
  src/
    theme/               # ThemeProvider، useTheme، capability flags
    components/          # Button, Input, DownloadRow, SegmentBar, StreamMatrix, ...
    screens/             # Main, AddDownload, BatchAdd, SeriesLibrary, Settings
    bridge/              # اتصال به Tauri commands و events
    store/               # Zustand stores
  .storybook/
```

* * *
## ۴. معماری توکن‌ها
سه لایه: **Primitive** ← **Semantic** ← **Component**. کامپوننت‌ها فقط از Semantic و Component استفاده می‌کنن.

```json
{
  "color": {
    "surface": { "$type": "color", "$value": "#16181D" },
    "accent":  { "$type": "color", "$value": "#7C5CFF" }
  },
  "capability": {
    "blur": { "$value": false },
    "glow": { "$value": true },
    "neuShadow": { "$value": false }
  }
}
```

**Build pipeline:** `tokens/*.json` ← Style Dictionary ← خروجی `tokens.css` (با selectorهای `[data-theme="..."]`) + `tokens.d.ts` (تایپ اسم توکن‌ها). اجرا در `prebuild` و در CI.

مقادیر کامل هر تم در بخش ۲ و ۶ Private ([https://app.clickup.com/90151376297/docs/2kyq1nd9-555](https://app.clickup.com/90151376297/docs/2kyq1nd9-555)) هست.

* * *
## ۵. نیازمندی‌های عملکردی: موتور تم

| ID | نیازمندی | معیار پذیرش | اولویت |
| ---| ---| ---| --- |
| TH-1 | پشتیبانی از ۵ تم با مقدار `data-theme` روی `<html>` | همه صفحه‌ها در هر تم بدون المان بی‌استایل رندر میشن | P0 |
| TH-2 | حالت System: دنبال کردن `prefers-color-scheme` (Dark/Light) | با تغییر تم سیستم‌عامل، برنامه بدون ری‌استارت عوض میشه | P0 |
| TH-3 | ذخیره تم در تنظیمات (سمت Rust) | بعد از ری‌استارت همون تم میاد، بدون flash تم اشتباه (تم قبل از اولین paint اعمال میشه) | P0 |
| TH-4 | انیمیشن نرم هنگام سوییچ (View Transitions API) | cross-fade ۲۰۰ms. با reduced-motion حذف میشه | P1 |
| TH-5 | Capability flags برای هر تم (blur، glow، neuShadow) | کامپوننت‌ها از طریق `useTheme().caps` رفتار رو عوض می‌کنن، نه با چک اسم تم | P0 |
| TH-6 | Fallback برای Glass وقتی `backdrop-filter` پشتیبانی نمیشه یا FPS کمه (مخصوصاً WebKitGTK روی لینوکس) | سطح‌ها به `rgba(20,20,35,0.85)` تبدیل میشن. اگه FPS برای ۳ ثانیه زیر ۳۰ بمونه، blur خودکار خاموش میشه و به کاربر اطلاع داده میشه | P0 |
| TH-7 | افکت نیتیو پنجره برای Glass (Mica / Acrylic / Vibrancy) | روی ویندوز ۱۱ و مک، دسکتاپ از پشت پنجره دیده میشه. در بقیه سیستم‌ها wallpaper گرادیانی داخلی | P1 |
| TH-8 | احترام به `prefers-reduced-motion` | glow متحرک، shimmer و transition تم غیرفعال میشن | P0 |
| TH-9 | پشتیبانی RTL با `dir="rtl"` و فونت Vazirmatn | فقط از logical properties استفاده میشه (`margin-inline-start`). اعداد، URLها و نوار پیشرفت همیشه LTR | P1 |
| TH-10 | حالت High Contrast | با `forced-colors: active` از رنگ‌های سیستم استفاده میشه | P2 |

* * *
## ۶. نیازمندی‌های کامپوننت‌ها
همه کامپوننت‌ها باید: در هر ۵ تم story داشته باشن، با کیبورد قابل استفاده باشن، focus ring قابل دیدن داشته باشن، و هیچ رنگ مستقیمی نداشته باشن.

| کامپوننت | Props / وضعیت‌ها | معیار پذیرش | اولویت |
| ---| ---| ---| --- |
| Button | variant: primary | secondary | ghost | danger · size: sm | md · loading · disabled | ارتفاع md = 40px. در Neumorphism حالت press = inset | P0 |
| IconButton | icon · label (اجباری، برای tooltip و aria-label) | 36×36. بدون label کامپایل نمیشه (تایپ TS) | P0 |
| UrlInput | value · onPaste · onSubmit · detecting | paste با ⌘V/Ctrl+V یا دکمه. Enter = شروع دانلود. وقتی چند لینک paste شد، BatchAdd باز میشه | P0 |
| SidebarItem | icon · label · count · active | آیتم فعال هم‌اندازه بقیه‌ست. count با فونت mono | P0 |
| SegmentBar | segments: Segment\[\] · height (پیش‌فرض 6) | رندر با Canvas یا یه SVG واحد (نه ۱۶ تا div). فاصله 2px. رنگ‌ها: done / active / slow / empty | P0 |
| DownloadRow | item: DownloadItem · selected · onAction | memo شده، فقط با تغییر همون آیتم re-render میشه. ارتفاع ثابت برای virtualization. Queued با opacity 0.7 | P0 |
| SmartDetectionChip | detection: SmartDetection · onEditPath | زیر ردیف‌های ویدیویی با confidence ≥ 0.8 نمایش داده میشه. کلیک = ویرایش مسیر. زیر 0.8 با آیکون «؟» و درخواست تأیید نمایش داده میشه | P0 |
| StreamMatrix | segments: Segment\[\] · columns (پیش‌فرض 4) | تا ۳۲ سلول. hover = tooltip با بایت‌های دانلودشده و سرعت همون کانکشن | P1 |
| SpeedChart | samples: number\[\] (۶۰ ثانیه آخر) | uPlot، رنگ‌ها از CSS variable خونده میشن و با سوییچ تم آپدیت میشن | P1 |
| ThroughputWidget | current · peak · history | تنها جای نمایش سرعت کلی در کل برنامه | P0 |
| StatusChip | status: DownloadStatus | رنگ و آیکون فقط از جدول وضعیت Design System. وضعیت فقط با رنگ نشون داده نمیشه (متن یا آیکون هم داره) | P0 |
| Toggle / Checkbox | checked · disabled | role و aria درست. Neumorphism: کانال inset و دستگیره raised | P0 |
| ThemePicker | value · onChange | کارت پیش‌نمایش زنده برای هر تم + گزینه System | P1 |

/

* * *
## ۷. صفحه‌ها

| صفحه | نیازمندی کلیدی | اولویت |
| ---| ---| --- |
| Main Window | Sidebar (240) + Top bar (64) + لیست virtualized + Inspector (360، قابل بستن) + Status bar (36). حداقل اندازه پنجره 1100×700. زیر 1280px، Inspector به drawer تبدیل میشه | P0 |
| Add Download | تشخیص خودکار اطلاعات فایل، پیشنهاد مسیر با SmartDetectionChip، اسلایدر کانکشن (۱ تا ۳۲) | P0 |
| Batch Add | parse زنده لینک‌ها تا ۵۰۰ خط بدون هنگ کردن (در Web Worker)، جدول با checkbox | P0 |
| Settings | تب‌ها: General، Connections، Speed Limits، Categories & Paths، Series Detection، Browser Integration، Proxy، Scheduler، AI Assistant، Appearance | P1 |
| Series Library | گرید پوستر، تب فصل‌ها، قسمت‌های گمشده، بج زیرنویس | P2 (فاز ۲) |
| Extension Popup | 360×500، همون توکن‌ها (پکیج مشترک `@flux/tokens`) | P1 |

* * *
## ۸. قرارداد داده UI با هسته

```plain
type DownloadStatus =
  | "downloading" | "queued" | "paused" | "throttled"
  | "completed" | "verified" | "error" | "link_expired";

interface Segment {
  index: number;
  start: number;          // byte offset
  end: number;
  downloaded: number;
  speed: number;          // bytes/s
  state: "idle" | "active" | "slow" | "done" | "error";
}

interface SmartDetection {
  kind: "series" | "movie" | "music" | "other";
  title?: string;
  season?: number;
  episode?: number;
  year?: number;
  confidence: number;     // 0..1
  targetPath: string;
  source: "pattern" | "ai" | "user";
}

interface DownloadItem {
  id: string;
  fileName: string;
  url: string;
  totalBytes: number | null;
  downloadedBytes: number;
  speed: number;
  eta: number | null;     // seconds
  status: DownloadStatus;
  category: string;
  resumable: boolean;
  segments: Segment[];
  detection?: SmartDetection;
  queueId?: string;
  queuePosition?: number;
}
```

**کانال ارتباطی:**
*   Commands (UI ← Core): `add_download`، `pause`، `resume`، `cancel`، `set_speed_limit`، `set_theme`، `get_settings`
*   Events (Core ← UI): `download:progress` به صورت **batch** هر ۲۵۰ms (نه برای هر بایت)، `download:status`، `download:detected`
*   داده progress فقط فیلدهای تغییرکرده (diff) رو می‌فرسته

* * *
## ۹. نیازمندی‌های غیرعملکردی
### عملکرد
*   حداکثر یه re-render در هر ۲۵۰ms برای هر ردیف
*   انیمیشن‌ها فقط با `transform` و `opacity`
*   در تم Glass، blur فقط روی لایه‌های اصلی (Sidebar، Top bar، Inspector) اعمال میشه، نه روی هر ردیف
*   در تم AMOLED حداکثر ۳ المان glow همزمان
### دسترسی‌پذیری
*   WCAG AA در همه تم‌ها. تم Neumorphism: text-2 هیچ‌وقت روشن‌تر از #5A6478 نمیشه
*   همه کارها با کیبورد انجام‌پذیرن

| میانبر | عمل |
| ---| --- |
| ⌘/Ctrl + V | paste لینک و تشخیص خودکار |
| ⌘/Ctrl + K | جستجو در دانلودها |
| Space | توقف یا ادامه آیتم انتخاب‌شده |
| Delete | لغو (با تأیید) |
| ⌘/Ctrl + I | باز و بسته کردن Inspector |
| ⌘/Ctrl + , | تنظیمات |

### پلتفرم
*   Windows 10/11 (WebView2)، macOS 12+ (WKWebView)، Linux (WebKitGTK)
*   تایتل‌بار سفارشی. دکمه‌های پنجره در مک سمت چپ و در ویندوز سمت راست

* * *
## ۱۰. تست و کنترل کیفیت
*   **Storybook** با سوییچر تم: هر کامپوننت × ۵ تم × همه وضعیت‌ها
*   **Visual regression** با Playwright: اسکرین‌شات هر صفحه در هر تم، با هر PR مقایسه میشه
*   **تست دسترسی‌پذیری** با axe-core در CI. شکست کنتراست = شکست build
*   **Lint:** قانون stylelint که رنگ hex و rgb مستقیم رو بیرون از پوشه tokens ممنوع می‌کنه
*   **تست بار:** حالت mock با ۵۰ دانلود فعال و ۱۶ سگمنت، اندازه‌گیری FPS در هر تم

* * *
## ۱۱. مایلستون‌ها

| مایلستون | خروجی | تخمین |
| ---| ---| --- |
| M1: توکن‌ها و موتور تم | Style Dictionary، ۵ تم، ThemeProvider، Storybook | ۱ هفته |
| M2: کامپوننت‌های P0 | Button، Input، SidebarItem، SegmentBar، DownloadRow، StatusChip، SmartDetectionChip | ۲ هفته |
| M3: پنجره اصلی + bridge | Main Window متصل به هسته واقعی، virtualization | ۲ هفته |
| M4: دیالوگ‌ها و تنظیمات | Add Download، Batch Add، Settings، ThemePicker | ۱.۵ هفته |
| M5: پولیش و QA | Glass نیتیو، RTL، تست بار، visual regression کامل | ۱ هفته |

* * *
## ۱۲. ریسک‌ها و سؤال‌های باز

| ریسک | اثر | راه‌حل |
| ---| ---| --- |
| backdrop-filter روی WebKitGTK کنده | تم Glass روی لینوکس لاگ میزنه | TH-6: fallback خودکار |
| سایه‌های دوگانه Neumorphism روی لیست بلند | افت FPS موقع اسکرول | سایه ردیف‌ها با `contain: paint` و تست بار |
| باران eventهای progress | هنگ کردن UI | batch هر ۲۵۰ms و diff |
| تشخیص اشتباه سریال | فایل توی پوشه غلط میره | آستانه confidence 0.8 و تأیید کاربر |

**سؤال‌های باز:**
- [ ] تم پیش‌فرض بعد از نصب: System یا Dark؟
- [ ] اسم نهایی برنامه همین Flux میمونه؟ (بررسی تداخل با برندهای موجود)
- [ ] کاربر بتونه تم سفارشی بسازه؟ (پیشنهاد: فاز ۲، با import فایل JSON توکن)