#include <map>
#include <string>
#include <vector>

#include "client/crashpad_client.h"
#include "client/settings.h"
#include "client/crash_report_database.h"

using crashpad::CrashReportDatabase;
using crashpad::CrashpadClient;

#if defined(__APPLE__) || defined(__linux__)

#define _convert(input)     input
#elif defined(_MSC_VER)

#include <stringapiset.h>
#define _convert(input)     s2ws(input)

std::wstring s2ws(char* str)
{
    int len = strlen(str);
    int size_needed = MultiByteToWideChar(CP_UTF8, 0, str, len, NULL, 0);
    std::wstring wstrTo(size_needed, 0);
    MultiByteToWideChar(CP_UTF8, 0, str, len, &wstrTo[0], size_needed);
    return wstrTo;
}

#endif

extern "C" {
    bool start_crashpad(char* raw_handler,
                        char* raw_datadir,
                        char* raw_url) {
        // Path to the out-of-process handler executable.
        // StartCrashpadInProcessHandler() is only defined on iOS.
        base::FilePath handler(_convert(raw_handler));
        // Cache directory that will store and queue crashpad data.
        base::FilePath database(_convert(raw_datadir));
        // URL used to submit minidumps to.
        std::string url(raw_url);
        // Optional annotations passed via --annotations to the handler.
        std::map<std::string, std::string> annotations;
        // Optional arguments to pass to the handler.
        std::vector<std::string> arguments;

        std::unique_ptr<CrashReportDatabase> db = CrashReportDatabase::Initialize(database);

        if (db == nullptr || db->GetSettings() == nullptr) {
            return false;
        }
        // Enable automated uploads.
        db->GetSettings()->SetUploadsEnabled(true);

        // To disable this limitation:
        // the crashpad handler will limit uploads to one per hour by default.
        arguments.push_back(std::string("--no-rate-limit"));

        CrashpadClient client;
        bool success = client.StartHandler(
            handler,
            database,
            database,
            url,
            annotations,
            arguments,
            true,  // restartable
            false  // asynchronous_start
        );
        if (success == false) {
            return false;
        }

        return true;
    }
}
